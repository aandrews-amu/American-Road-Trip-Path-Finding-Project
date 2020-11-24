use super::{Domain, ValueType, Var};
use crate::counting_sat::{Lit, CSAT};
use crate::csp::CSP;
use crate::fd::{Reify, FD};
use std::collections::HashMap;
use std::ops::Index;

impl ValueType for usize {}

pub fn roadtrip(vcount: usize, edges: &[(usize, usize, u32)]) -> (Vec<usize>, u32) {
    let mut csp = CSP::new();

    // collect all the edges and costs into a hash map
    let edges_map: HashMap<(usize, usize), u32> = edges
        .iter()
        .map(|(from, to, cost)| ((*from, *to), *cost))
        .collect();

    // create variables for each time choice for vertices
    let choices: Vec<_> = (0..vcount)
        .map(|i| {
            if i == 0 {
                csp.add_variable(Domain::new(vec![0]))
            } else {
                csp.add_variable(Domain::new((0..vcount).collect()))
            }
        })
        .collect();

    // make the first and last variable be the same so that we create a cycle
    let mut cycle = choices.clone();
    cycle.push(choices[0]);

    // we need to visit all vertices, so all variables must be different
    csp.add_constraint(FD::AllDiff(choices));

    // generate solution using branch and bound
    let soln = csp
        .bnb(
            |v| {
                cycle
                    .windows(2)
                    .map(|cs| {
                        let from = cs[0];
                        let to = cs[1];
                        match (v.get_value(from), v.get_value(to)) {
                            (Some(from), Some(to)) => edges_map[&(from, to)],
                            _ => (0 as u32),
                        }
                    })
                    .sum()
            },
            u32::MAX,
        )
        .unwrap();

    // compute the total cost for the trip
    let net_cost = cycle
        .windows(2)
        .map(|cs| {
            let from = cs[0];
            let to = cs[1];

            edges_map[&(soln[from], soln[to])]
        })
        .sum();

    // iterate through the variables in our cycle and get there value
    let order: Vec<_> = cycle.iter().map(|var| *soln.index(var.0)).collect();

    (order, net_cost)
}

#[cfg(test)]
mod pub_tests;
