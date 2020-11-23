use super::{Constraint, Domain, PartialValuation, Valuation, ValueType, Var};
use std::collections::VecDeque;

/// A constraint satisfaction problem, parameterized on a value type and constraint theory.
pub struct CSP<Val: ValueType, Con: Constraint<Val>> {
    variables: Vec<Var>,           // One per variable
    strong_propagation: Vec<bool>, // One per variable
    domains: Vec<Domain<Val>>,     // One per variable
    constraints: Vec<Con>,
}
impl<Val: ValueType, Con: Constraint<Val>> CSP<Val, Con> {
    /// Creates a new empty CSP.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            strong_propagation: Vec::new(),
            domains: Vec::new(),
            constraints: Vec::new(),
        }
    }
    /// Adds a new variable with the given domain and returns its
    /// identifier.
    pub fn add_variable(&mut self, d: Domain<Val>) -> Var {
        self.add_variable_prop(d, true)
    }
    pub fn add_weak_variable(&mut self, d: Domain<Val>) -> Var {
        self.add_variable_prop(d, false)
    }
    pub fn add_variable_prop(&mut self, d: Domain<Val>, strong_prop: bool) -> Var {
        // This should update variables *and* domains.  Recall the
        // distinction between weak and strong propagation!
        self.domains.push(d);
        let var = Var(self.variables.len());
        self.variables.push(var);
        var
    }
    /// Adds a new constraint from the CSP's constraint theory.
    pub fn add_constraint(&mut self, c: Con) {
        // It's probably a good idea to build an index from variables
        // (which get changed) to which constraints should get checked
        // when they do change.  You can achieve this by putting two
        // vecs into `CSP`: one that goes from constraint number to a
        // vec of variables, and one that goes from a variable number
        // to a vec of constraints.
        // This will probably be the same as HW3.
        self.constraints.push(c);
    }
    /// Selects the next variable from `vals` to be assigned.  `vals`
    /// will have `None` at an index if that variable is unassigned,
    /// and `Some(value)` otherwise. This might be based on the size
    /// of the domain, the number of constraints the variable is
    /// involved in, the order of variables in the problem, or some
    /// other measure.  Note that the returned `Domain` has the same
    /// "lifetime" as the input `PartialValuation`, which just means
    /// that it should be obtained by calling `get_domain` or
    /// `domain_iter` on `vals`.
    fn pick_variable<'pv>(
        &self,
        vals: &'pv PartialValuation<Val>,
    ) -> Option<(Var, &'pv Domain<Val>)> {
        // This will be similar to HW3, but if you were ordering variables by domain size before you'll want to consider strongly vs weakly propagating variables.
        vals.domain_iter()
            .find(|(var, domain)| !vals.is_fixed(*var))
    }
    /// Returns whether the given partial valuation `vals` represents
    /// a solution, i.e. that every domain is unit and it satisfies
    /// all constraints.
    fn is_solution(&self, vals: &PartialValuation<Val>) -> bool {
        vals.all_fixed(&self.variables) && self.constraints.iter().all(|c| c.is_satisfied(vals))
    }
    /// Propagates a variable update through the csp's constraints,
    /// narrowing domains and gathering up all the changes for later undoing.
    fn propagate(&self, var: Var, vals: &mut PartialValuation<Val>) -> bool {
        // Note that you'll want to call `vals.propagate` and use its
        // return value to figure out what variables were modified by
        // the propagation of this particular constraint (and then
        // call propagate again on the constraints of those
        // variables).  What does it mean if `vals.propagate` returns
        // `None` versus `Some(Vec<Var>)`?
        let mut vstack = vec![var]; // initializing stack with first var

        while let Some(v) = vstack.pop() {
            for c in self
                .constraints
                .iter()
                .filter(|con| con.vars().contains(&v))
            {
                match vals.propagate(v, c) {
                    Some(vs) => vstack.extend(vs.into_iter()), // getting the list of changed variables, so enqueue, may need to filter out anything that's on the stack already
                    None => return false,                      // propagation failed, got a conflict
                }
            }
        }

        true
    }

    pub fn bnb<Cost: Ord>(
        &self,
        cost: impl Fn(&PartialValuation<Val>) -> Cost,
        limit: Cost,
    ) -> Option<Valuation<Val>> {
        // You can use the given initial limit and cost function
        // rather than using a fixed bound or a fixed `score()`
        // function.  Note that this works with any Ordered thing as
        // the type of cost!  So you can do scalars or lexicographic
        // orderings (the template from the slides won't exactly
        // work for Pareto optimality).
        //todo!()
        let mut queue = VecDeque::new();
        queue.push_back(PartialValuation::new(self.domains.clone()));
        let mut best = None;
        let mut limit = limit;

        while let Some(vals) = queue.pop_front() {
            let score = cost(&vals);
            // bounding step
            if score >= limit {
                continue;
            }
            if self.is_solution(&vals) {
                // update limit and best-seen solution
                limit = score;
                best = Some(vals);
                continue;
            }
            // split on the domain of the next variable
            if let Some((var, dom)) = self.pick_variable(&vals) {
                // branching step
                for choice in dom.clone().into_iter() {
                    let mut par_val = vals.clone();
                    par_val.assign(var, choice); // what should this be doing?
                                                 // propagate
                                                 //if propagate(val_i, var) == UNSAT: continue;

                    if self.propagate(var, &mut par_val) {
                        queue.push_back(par_val);
                    }
                }
            }
        }
        best.and_then(|v| v.finalize())
    }
}
