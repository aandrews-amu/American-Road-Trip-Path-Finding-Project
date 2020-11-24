use super::{csp::CSP, Constraint, Domain, PartialValuation, ValueType, Var};

/// A toy finite-domain constraint over variables.  In a real
/// implementation with constraint propagation we would want to allow for
/// using constants directly in constraints and offer more
/// constraints (e.g., that values are increasing or that the product of a
/// vec of variables is the same as some variable or constant).
pub enum FD {
    /// All variables have equal values
    Eq(Vec<Var>),
    /// All variables have distinct values
    AllDiff(Vec<Var>),
    /// All variables in the vec sum up to the given var
    Sum(Var, Vec<Var>),
    /// All variables in the vec are in strictly increasing order
    Inc(Vec<Var>),
}

impl<FDVal> Constraint<FDVal> for FD
where
    FDVal:
        ValueType + std::ops::Add<Output = FDVal> + std::ops::Sub<Output = FDVal> + std::iter::Sum,
{
    fn vars(&self) -> Vec<Var> {
        match self {
            FD::Eq(v) => v.to_vec(),
            FD::AllDiff(v) => v.to_vec(),
            FD::Sum(v1, vars) => {
                let mut vs = vars.clone();
                vs.push(*v1);
                vs.to_vec()
            } // need to combine var and vec of vars and return as one vec
            FD::Inc(v) => v.to_vec(),
        }
    }

    fn is_satisfied(&self, vals: &PartialValuation<FDVal>) -> bool {
        match self {
            FD::Eq(v) => {
                let vals: Vec<_> = v.iter().map(|var| vals.get_value(*var)).collect();
                for v in vals.windows(2) {
                    if v[0] != v[1] {
                        return false;
                    }
                }
                true
            }
            FD::AllDiff(v) => {
                let mut k: Vec<_> = v.iter().map(|var| vals.get_value(*var)).collect();
                k.sort();
                k.dedup();
                v.len() == k.len() // compares original vector to unique vector, if they are the same then all elements are different
            }
            FD::Sum(v, vars) => {
                vals.all_fixed(vars)
                    && vars
                        .iter()
                        .map(|var| vals.get_value(*var).unwrap())
                        .sum::<FDVal>()
                        == vals.get_value(*v).unwrap()
            }
            FD::Inc(v) => {
                //todo!() // clone the original vector, sort it in increasing order and then check for equality ,
                //let mut v_copy = v.clone();
                //v_copy.sort(); // sorts in increasing order
                //assert_eq!(v, &v_copy)
                let vals: Vec<_> = v.iter().map(|var| vals.get_value(*var)).collect();
                for v in vals.windows(2) {
                    if v[0] >= v[1] {
                        return false;
                    }
                }
                true
            }
        }

        // Note you may want to verify that every variable you care about is fixed!
    }

    fn propagate(&self, var: Var, vals: &mut PartialValuation<FDVal>) -> bool {
        // This will include your implementation from hw3, but also you'll need to propagate a new type of constraint: Where the given variables are in strictly ascending order, i.e. where each element must be bigger than the one before.  When a variable changes you can propagate just to the variable immediately before in the chain and the one immediately after, or you can update the whole vec in one go.  Think about which values you can remove from the variables before and after the changed variable.
        match self {
            FD::Eq(vs) => {
                let d = vals.get_domain(var).clone();
                for v in vs.iter() {
                    if !vals.narrow(*v, &d) {
                        return false;
                    }
                }
                // var just got a domain
                // let d = get the domain of var
                // narrow every variable in vs by d
                true
            }

            FD::AllDiff(vs) => {
                if vals.is_fixed(var) {
                    let value = vals.get_value(var).unwrap();
                    for v in vs.iter() {
                        if *v == var {
                            continue;
                        }
                        if !vals.remove(*v, value) {
                            return false;
                        }
                    }
                }
                true
            }

            FD::Sum(v, vs) => {
                // solve for each vbl in v ++ vs except for the one == var
                if *v != var {
                    let low: FDVal = vs
                        .iter()
                        .map(|&v| *vals.get_domain(v).iter().min().unwrap())
                        .sum();
                    let high: FDVal = vs
                        .iter()
                        .map(|&v| *vals.get_domain(v).iter().max().unwrap())
                        .sum();
                    for &val in vals.get_domain(*v).clone().iter() {
                        if (val < low || high < val) && !vals.remove(*v, val) {
                            return false;
                        }
                    }
                }
                // vi = v - sum(vjs)
                for &vi in vs.iter() {
                    if vi == var {
                        continue;
                    }
                    let low: FDVal = *vals.get_domain(*v).iter().min().unwrap()
                        - vs.iter()
                            .filter_map(|&vj| {
                                if vj == vi {
                                    None
                                } else {
                                    Some(*vals.get_domain(vj).iter().max().unwrap())
                                }
                            })
                            .sum();
                    let high: FDVal = *vals.get_domain(*v).iter().max().unwrap()
                        - vs.iter()
                            .filter_map(|&vj| {
                                if vj == vi {
                                    None
                                } else {
                                    Some(*vals.get_domain(vj).iter().min().unwrap())
                                }
                            })
                            .sum();
                    for &val in vals.get_domain(vi).clone().iter() {
                        if (val < low || high < val) && !vals.remove(vi, val) {
                            return false;
                        }
                    }
                }
                true
            }
            FD::Inc(v) => {
                // find the position of the changed vector
                let i = v.iter().position(|&x| x == var).unwrap();

                let changed_domain = vals.get_domain(v[i]).clone();
                let min_changed_domain = changed_domain.min();
                let max_changed_domain = changed_domain.max();

                if i > 0 {
                    let left_domain = vals.get_domain(v[i - 1]).clone();
                    // change the variable to the left of v (v - 1)
                    // remove values that are larger than the min of v
                    let narrow_left = Domain::new(
                        left_domain
                            .iter()
                            .filter(|val| val < &max_changed_domain)
                            .cloned()
                            .collect(),
                    );
                    if !vals.narrow(v[i - 1], &narrow_left) {
                        return false;
                    }
                }

                if i < v.len() - 1 {
                    let right_domain = vals.get_domain(v[i + 1]).clone();
                    // change the variable to the right of v (v + 1)
                    // remove values that are smaller than the max of v
                    let narrow_right = Domain::new(
                        right_domain
                            .iter()
                            .filter(|val| val > &min_changed_domain)
                            .cloned()
                            .collect(),
                    );
                    if !vals.narrow(v[i + 1], &narrow_right) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/// Reified logical constraints over FD constraints
pub enum Reify {
    /// Plain FD constraint
    Holds(FD),
    /// Var = ((FD))
    Reify(Var, FD),
    /// Var = v1 & v2 & ...
    And(Var, Vec<Var>),
    /// Var = v1 | v2 | ...
    Or(Var, Vec<Var>),
    /// Var = !Var
    Not(Var, Var),
    /// Var => Var
    Implies(Var, Var),
}
impl ValueType for i16 {}

impl Constraint<i16> for Reify {
    fn vars(&self) -> Vec<Var> {
        match self {
            Reify::Holds(fd) => {
                let fd: &dyn Constraint<i16> = fd;
                fd.vars()
            }
            Reify::Reify(v, fd) => {
                let fd: &dyn Constraint<i16> = fd;
                [*v].iter().chain(fd.vars().iter()).cloned().collect()
            }
            Reify::And(v, vs) => [*v].iter().chain(vs.iter()).cloned().collect(),
            Reify::Or(v, vs) => [*v].iter().chain(vs.iter()).cloned().collect(),
            Reify::Not(v1, v2) => vec![*v1, *v2],
            Reify::Implies(v1, v2) => vec![*v1, *v2],
        }
    }

    fn is_satisfied(&self, vals: &PartialValuation<i16>) -> bool {
        match self {
            Reify::Holds(fd) => fd.is_satisfied(vals),
            Reify::Reify(v, fd) => {
                let fd: &dyn Constraint<i16> = fd;
                if !vals.is_fixed(*v) {
                    return false;
                }
                if !vals.all_fixed(&fd.vars()) {
                    return false;
                }
                (vals.get_value(*v).unwrap() != 0) == fd.is_satisfied(vals)
            }
            Reify::And(v, vs) => {
                if !vals.is_fixed(*v) {
                    return false;
                }
                if !vals.all_fixed(vs) {
                    return false;
                }
                (vals.get_value(*v).unwrap() != 0)
                    == vs.iter().all(|vi| vals.get_value(*vi).unwrap() != 0)
            }
            Reify::Or(v, vs) => {
                if !vals.is_fixed(*v) {
                    return false;
                }
                if !vals.all_fixed(vs) {
                    return false;
                }
                (vals.get_value(*v).unwrap() != 0)
                    == vs.iter().any(|vi| vals.get_value(*vi).unwrap() != 0)
            }
            Reify::Not(v1, v2) => {
                if !vals.all_fixed(&[*v1, *v2]) {
                    return false;
                }
                vals.get_value(*v1).unwrap()
                    == 1 - if vals.get_value(*v2).unwrap() == 0 {
                        0
                    } else {
                        1
                    }
            }
            Reify::Implies(v1, v2) => {
                if !vals.all_fixed(&[*v1, *v2]) {
                    return false;
                }
                vals.get_value(*v1).unwrap() == 0 || vals.get_value(*v2).unwrap() == 1
            }
        }
    }

    fn propagate(&self, var: Var, vals: &mut PartialValuation<i16>) -> bool {
        match self {
            Reify::Holds(fd) => fd.propagate(var, vals),
            Reify::Reify(v, fd) => {
                // We can propagate only if v is true, or if fd is satisfied, or if fd is not satisfied and fd is fully fixed
                if var != *v && vals.is_fixed(*v) && vals.get_value(*v).unwrap() != 0 {
                    if !fd.propagate(var, vals) {
                        return false;
                    }
                } else if var != *v {
                    let fd: &dyn Constraint<i16> = fd;
                    if vals.all_fixed(&fd.vars()) && fd.is_satisfied(vals) {
                        if !vals.narrow(*v, &Domain::new(vec![1])) {
                            return false;
                        }
                    } else if vals.all_fixed(&fd.vars()) && !vals.narrow(*v, &Domain::new(vec![0]))
                    {
                        return false;
                    }
                }
                true
            }
            Reify::And(v, vs) => {
                // if v is true, vs must all be true
                if vals.is_fixed(*v) && vals.get_value(*v).unwrap() != 0 {
                    for v in vs.iter() {
                        if !vals.narrow(*v, &Domain::new(vec![1])) {
                            return false;
                        }
                    }
                }
                // if any vs is false, v must be false
                for v in vs.iter() {
                    if vals.is_fixed(*v) && vals.get_value(*v).unwrap() == 0 {
                        if !vals.narrow(*v, &Domain::new(vec![0])) {
                            return false;
                        }
                        break;
                    }
                }
                true
            }
            Reify::Or(v, vs) => {
                // if v is false, vs must all be false
                // if any vs is true, v must be true
                if vals.is_fixed(*v) && vals.get_value(*v).unwrap() == 0 {
                    for v in vs.iter() {
                        if !vals.narrow(*v, &Domain::new(vec![0])) {
                            return false;
                        }
                    }
                }
                for v in vs.iter() {
                    if vals.is_fixed(*v) && vals.get_value(*v).unwrap() != 0 {
                        if !vals.narrow(*v, &Domain::new(vec![1])) {
                            return false;
                        }
                        break;
                    }
                }
                true
            }
            Reify::Not(v1, v2) => {
                // if v1 is fixed, v2 is 1-v1
                // if v2 is fixed, v1 is 1-v2
                if vals.is_fixed(*v1)
                    && !vals.narrow(
                        *v2,
                        &Domain::new(vec![
                            1 - if vals.get_value(*v1).unwrap() == 0 {
                                0
                            } else {
                                1
                            },
                        ]),
                    )
                {
                    return false;
                }
                if vals.is_fixed(*v2)
                    && !vals.narrow(
                        *v1,
                        &Domain::new(vec![
                            1 - if vals.get_value(*v2).unwrap() == 0 {
                                0
                            } else {
                                1
                            },
                        ]),
                    )
                {
                    return false;
                }
                true
            }
            Reify::Implies(v1, v2) => {
                // if v1 is fixed true, v2 is fixed true
                // if v2 is false, v1 must be fixed false
                if vals.is_fixed(*v1)
                    && vals.get_value(*v1).unwrap() != 0
                    && !vals.narrow(*v2, &Domain::new(vec![1]))
                {
                    return false;
                }
                if vals.is_fixed(*v2)
                    && vals.get_value(*v2).unwrap() == 0
                    && !vals.narrow(*v1, &Domain::new(vec![0]))
                {
                    return false;
                }
                true
            }
        }
    }
}

//#[cfg(test)]
//mod pub_tests;
