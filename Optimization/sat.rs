use super::{Constraint, PartialValuation, ValueType, Var};

/// A SAT literal: a `Var` appears either positively or negatively.
pub enum Lit {
    Pos(Var),
    Neg(Var),
}
impl ValueType for bool {}
/// A SAT clause constraint, comprising a vec of literals; the clause
/// is satisfied if any literal is satisfied.
pub struct SAT(pub Vec<Lit>);
impl Constraint<bool> for SAT {
    fn vars(&self) -> Vec<Var> {
        self.0
            .iter()
            .map(|l| match l {
                Lit::Pos(v) => *v,
                Lit::Neg(v) => *v,
            })
            .collect()
    }
    fn is_satisfied(&self, vals: &PartialValuation<bool>) -> bool {
        self.0.iter().any(|l| match *l {
            Lit::Pos(v) => vals.get(v),
            Lit::Neg(v) => !vals.get(v),
        })
    }
}
#[cfg(test)]
mod pub_tests {
    use super::*;
    use crate::{csp::CSP, Domain, Valuation};
    #[test]
    fn test_sat() {
        let mut csp = CSP::new();
        let v1 = csp.add_variable(Domain::new(vec![false, true]));
        let v2 = csp.add_variable(Domain::new(vec![false, true]));
        let v3 = csp.add_variable(Domain::new(vec![false, true]));
        csp.add_constraint(SAT(vec![Lit::Pos(v1), Lit::Pos(v2)]));
        csp.add_constraint(SAT(vec![Lit::Neg(v2), Lit::Neg(v3)]));
        csp.add_constraint(SAT(vec![Lit::Pos(v3)]));
        let soln = csp.solve();
        assert!(soln.is_some());
        assert_eq!(soln, Some(Valuation::new(vec![true, false, true])));
    }
}
