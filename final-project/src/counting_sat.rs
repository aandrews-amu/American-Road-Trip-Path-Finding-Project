use super::{Constraint, Domain, PartialValuation, ValueType, Var};

/// A propositional literal: a `Var` appears either positively or negatively.
pub enum Lit {
    Pos(Var),
    Neg(Var),
}
impl ValueType for bool {}
/// Unlike regular SAT, Cardinality-SAT puts a minimum and maximum
/// number of satisfied literals onto each clause
pub struct CSAT {
    pub lits: Vec<Lit>,
    pub at_least: usize,
    pub at_most: usize,
}
impl Constraint<bool> for CSAT {
    fn vars(&self) -> Vec<Var> {
        self.lits
            .iter()
            .map(|l| match l {
                Lit::Pos(v) => *v,
                Lit::Neg(v) => *v,
            })
            .collect()
    }
    /// Is the number of currently forced-true literals between min and max?
    fn is_satisfied(&self, vals: &PartialValuation<bool>) -> bool {
        let forced = self.forced_count(vals);
        self.at_least <= forced && forced <= self.at_most
    }
    fn propagate(&self, var: Var, vals: &mut PartialValuation<bool>) -> bool {
        let forced = self.forced_count(vals);
        let prevented = self.prevented_count(vals);
        // If the number of forced-true literals is above max, fail; if it's exactly max, propagate that the other literals are all false.
        if forced == self.at_most {
            //force all possible to not
            for l in self.lits.iter() {
                match *l {
                    Lit::Pos(v) => {
                        if v != var && !vals.is_fixed(v) {
                            vals.narrow(v, &Domain::new(vec![false]));
                        }
                    }
                    Lit::Neg(v) => {
                        if v != var && !vals.is_fixed(v) {
                            vals.narrow(v, &Domain::new(vec![true]));
                        }
                    }
                }
            }
        } else if forced > self.at_most {
            //conflict; just narrow the input var to empty
            vals.narrow(var, &Domain::new(vec![]));
            return false;
        }
        // If the number of literals minus the number of forced-false literals is above min, fail; if it's exactly min, propagate that the other literals are all true.
        let possible = self.lits.len() - prevented;
        if possible < self.at_least {
            //conflict; just narrow the input var to empty
            vals.narrow(var, &Domain::new(vec![]));
            return false;
        } else if possible == self.at_least {
            //force all possible to true
            for l in self.lits.iter() {
                match *l {
                    Lit::Pos(v) => {
                        if v != var && !vals.is_fixed(v) {
                            vals.narrow(v, &Domain::new(vec![true]));
                        }
                    }
                    Lit::Neg(v) => {
                        if v != var && !vals.is_fixed(v) {
                            vals.narrow(v, &Domain::new(vec![false]));
                        }
                    }
                }
            }
        }
        true
    }
}

impl CSAT {
    // How many literals are presently forced to true?
    fn forced_count(&self, vals: &PartialValuation<bool>) -> usize {
        self.lits
            .iter()
            .filter(|l| match **l {
                Lit::Pos(v) => vals.get_value(v) == Some(true),
                Lit::Neg(v) => vals.get_value(v) == Some(false),
            })
            .count()
    }
    // How many literals are presently forced to false?
    fn prevented_count(&self, vals: &PartialValuation<bool>) -> usize {
        self.lits
            .iter()
            .filter(|l| match **l {
                Lit::Pos(v) => vals.get_value(v) == Some(false),
                Lit::Neg(v) => vals.get_value(v) == Some(true),
            })
            .count()
    }
}

//#[cfg(test)]
//mod pub_tests;
