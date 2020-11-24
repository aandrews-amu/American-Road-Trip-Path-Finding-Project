#![allow(
    clippy::new_without_default,
    clippy::len_without_is_empty,
    unused_imports,
    clippy::let_and_return,
    unused_variables,
    dead_code,
    clippy::comparison_chain
)]

use std::collections::BTreeSet;

/// A constraint for a CSP; the type provides the constraint theory.
pub trait Constraint<Val: ValueType> {
    /// Returns a vec of variables involved in this particular
    /// constraint.
    fn vars(&self) -> Vec<Var>;
    /// Propagates assuming that `var` has just had its domain narrowed.
    /// Returns false if this produced an evident conflict.
    fn propagate(&self, var: Var, values: &mut PartialValuation<Val>) -> bool;
    /// Determines whether the constraint is presently satisfied by the given partial valuation.
    fn is_satisfied(&self, values: &PartialValuation<Val>) -> bool;
}

/// A CSP variable.  Mostly interchangeable with its index.  These
/// should only be created via `CSP::add_variable`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Var(pub usize); // 2^64 variables ought to be enough for anybody

/// Anything used as a value in a CSP should implement `Copy`, `Eq`, `Ord`, and `Debug`.
pub trait ValueType: Copy + Eq + Ord + std::fmt::Debug {}

/// A CSP variable domain.  Ensures `values` remain sorted.
///
/// We leave some performance and expressiveness on the floor by forcing an explicit set representation: we can't handle infinite domains (e.g. ranges of reals) and have a lot of pointer chasing going on.  Using SmallVec instead of Vec could help, but really CSP should be made generic over `Domain` in a real implementation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Domain<T: ValueType> {
    values: Vec<T>,
}
impl<Val: ValueType> Domain<Val> {
    /// Creates a new domain from the given `Vec` of `Val` values.
    pub fn new(mut values: Vec<Val>) -> Self {
        values.sort_unstable();
        Self { values }
    }
    /// Returns whether the domain contains the given value
    pub fn contains(&self, val: &Val) -> bool {
        self.values.binary_search(val).is_ok()
    }
    /// True if the domain is a unit domain, i.e. has `len` equal to 1
    pub fn is_unit(&self) -> bool {
        self.len() == 1
    }
    /// Collapse the domain to a single unit value, if possible.
    pub fn get_value(&self) -> Option<Val> {
        if self.is_unit() {
            Some(self.values[0])
        } else {
            None
        }
    }
    /// Creates an iterator over the domain's values.
    pub fn iter(&self) -> std::slice::Iter<Val> {
        self.values.iter()
    }
    /// Returns the number of values in the domain.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Narrows a domain to its intersection with the argument.
    /// Returns the removed elements.
    pub fn narrow(&mut self, dom: &Domain<Val>) -> Vec<Val> {
        let mut result = Vec::new();
        self.values.retain(|v| {
            if dom.contains(&v) {
                true
            } else {
                result.push(*v);
                false
            }
        });
        result
    }
    /// Removes the given value from the domain, returning whether it was present in the first place.
    pub fn remove(&mut self, val: Val) -> bool {
        if let Ok(idx) = self.values.binary_search(&val) {
            self.values.remove(idx);
            true
        } else {
            false
        }
    }
    /// Puts the given value into the domain
    pub fn add(&mut self, val: Val) {
        let idx = self.values.binary_search(&val).unwrap_or_else(|x| x);
        self.values.insert(idx, val);
    }
    /// Returns true if the receiver is empty, i.e. there is a
    /// conflict.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn min(&self) -> &Val {
        &self.values[0]
    }
    pub fn max(&self) -> &Val {
        self.values.last().unwrap()
    }
}
impl<Val: ValueType> IntoIterator for Domain<Val> {
    type Item = Val;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    /// Creates an iterator over the available values, consuming the
    /// receiver.
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
/// A valuation, or solution, to a CSP.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valuation<Val: ValueType> {
    values: Vec<Val>, // One per variable
}
impl<Val: ValueType> Valuation<Val> {
    /// Creates a new solution given a vector of values.
    pub fn new(values: Vec<Val>) -> Self {
        Self { values }
    }
    /// Creates an iterator over the values of every variable.  Note
    /// that when not using `Var` indexing, the caller relies on
    /// knowledge of the order in which variables were added to the
    /// constraint problem.
    pub fn iter(&self) -> std::slice::Iter<Val> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
impl<Val: ValueType> std::ops::Index<Var> for Valuation<Val> {
    type Output = Val;
    fn index(&self, var: Var) -> &Self::Output {
        &self.values[var.0]
    }
}

impl<Val: ValueType> std::ops::Index<usize> for Valuation<Val> {
    type Output = Val;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.values[idx]
    }
}

impl<Val: ValueType> IntoIterator for Valuation<Val> {
    type Item = Val;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    /// Creates an iterator over the values of every variable,
    /// consuming the receiver.  Note that when not using `Var`
    /// indexing, the caller relies on knowledge of the order in which
    /// variables were added to the constraint problem.
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

/// A partial valuation, or solution-in-progress, to a CSP.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PartialValuation<Val: ValueType> {
    domains: Vec<Domain<Val>>, // One per variable
    values: Vec<Option<Val>>,  // One per variable
    changed: BTreeSet<Var>,    // add a field for tracking variables that have been changed
}

impl<Val: ValueType> std::cmp::PartialOrd for PartialValuation<Val> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}
impl<Val: ValueType> std::cmp::Ord for PartialValuation<Val> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl<Val: ValueType> PartialValuation<Val> {
    /// Creates a new solution given a vector of values.
    pub fn new(domains: Vec<Domain<Val>>) -> Self {
        let vcount = domains.len();
        Self {
            domains,
            values: vec![None; vcount],
            changed: BTreeSet::new(), // empty BTreeSet
        }
    }
    /// Returns whether the given var is fixed
    pub fn is_fixed(&self, var: Var) -> bool {
        self.get_value(var).is_some()
    }
    /// Returns whether the partial valuation is currently in conflict
    pub fn is_conflict(&self) -> bool {
        self.domains.iter().any(|d| d.is_empty())
    }
    /// Returns whether all the given vars are fixed
    pub fn all_fixed(&self, vars: &[Var]) -> bool {
        vars.iter().all(|v| self.get_value(*v).is_some())
    }
    /// Tries to convert this partial valuation to a total valuation
    pub fn finalize(self) -> Option<Valuation<Val>> {
        if self.values.iter().all(Option::is_some) {
            Some(Valuation::new(
                self.values.into_iter().map(Option::unwrap).collect(),
            ))
        } else {
            None
        }
    }
    /// Removes a single value from v's domain
    pub fn remove(&mut self, v: Var, val: Val) -> bool {
        let dom = &mut self.domains[v.0];
        if dom.remove(val) {
            //Maybe track that this value was actually removed from the domain
            self.changed.insert(v);
        }
        if self.values[v.0].is_none() && dom.is_unit() {
            self.values[v.0] = dom.get_value();
        }
        !dom.is_empty()
    }
    /// Narrows the given var to its intersection with the given domain
    pub fn narrow(&mut self, var: Var, indom: &Domain<Val>) -> bool {
        let dom = &mut self.domains[var.0];
        for removed in dom.narrow(indom) {
            //Maybe track that this value was actually removed from the domain
            self.changed.insert(var);
        }
        if self.values[var.0].is_none() && dom.is_unit() {
            self.values[var.0] = dom.get_value();
        }
        !dom.is_empty()
    }
    /// Assigns the given var to the given value. Panics if the assignment is invalid (`val` not in `var`'s domain, `var` already assigned).
    pub fn assign(&mut self, var: Var, val: Val) {
        assert!(self.values[var.0].is_none());
        assert!(self.domains[var.0].contains(&val));
        // If you're doing checkpointing you might want to make your copy of self.values and self.domains here, while if you're doing backtracking you'll need to somehow mark this decision point so you can rewind to it later.
        self.values[var.0] = Some(val);
        assert!(self.narrow(var, &Domain::new(vec![val])))
    }
    /// Undoes the assignment to the given variable.
    pub fn unassign(&mut self, var: Var) {
        self.values[var.0] = None;
        // If you're doing checkpointing within PartialValuation you'll want to step back by one checkpoint, while if you're doing backtracking you'll need to undo any narrowings or value removals up to the last assignment.
    }
    /// Calls propagate on the given constraint, tracking and returning which variable domains have narrowed.
    /// May return the same var multiple times.
    pub fn propagate(&mut self, var: Var, con: &impl Constraint<Val>) -> Option<Vec<Var>> {
        self.changed.clear(); // resetting the changed variables

        // return the changed variables
        if con.propagate(var, self) {
            Some(self.changed.iter().cloned().collect())
        } else {
            None
        }

        // Here you'll need to call `con.propagate(var,self)`, but the trick is that you need a way of figuring out and returning the variables which have changed as a result of the narrowings and removals performed in `con.propagate`. If you're implementing backtracking this could be obtained from your trace of removals, while if you're doing checkpointing you might track which of the valuation's variables have changed since the last time propahate was called.
    }
    /// Creates an iterator over the domains of every variable.
    pub fn domain_iter(&self) -> Box<dyn Iterator<Item = (Var, &Domain<Val>)> + '_> {
        Box::new(
            self.domains
                .iter()
                .enumerate()
                .map(|(vi, dom)| (Var(vi), dom)),
        )
    }
    /// Creates an iterator over the values of every assigned variable.
    pub fn value_iter(&self) -> Box<dyn Iterator<Item = (Var, Val)> + '_> {
        Box::new(self.values.iter().enumerate().filter_map(|(vi, val)| {
            if val.is_some() {
                Some((Var(vi), val.unwrap()))
            } else {
                None
            }
        }))
    }
    /// Gets the assigned value for the given variable.
    pub fn get_value(&self, v: Var) -> Option<Val> {
        self.values[v.0]
    }
    /// Gets the domain for the given variable
    pub fn get_domain(&self, v: Var) -> &Domain<Val> {
        &self.domains[v.0]
    }
}

impl<Val: ValueType> std::ops::Index<Var> for PartialValuation<Val> {
    type Output = Domain<Val>;
    fn index(&self, var: Var) -> &Self::Output {
        &self.domains[var.0]
    }
}

impl<Val: ValueType> std::ops::Index<usize> for PartialValuation<Val> {
    type Output = Domain<Val>;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.domains[idx]
    }
}

pub mod counting_sat;
pub mod csp;
pub mod fd;
pub mod opt;
