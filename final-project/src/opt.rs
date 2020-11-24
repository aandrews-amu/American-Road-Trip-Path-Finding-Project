use super::{Domain, ValueType, Var};
use crate::counting_sat::{Lit, CSAT};
use crate::csp::CSP;
use crate::fd::{Reify, FD};
use std::collections::HashMap;
use std::ops::Index;
use std::time::{Duration, Instant};

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

// fn main() {
//     let start = Instant::now();

//     let vcount: usize = 10;
//     let edges: &[(usize, usize, u32)] = &[
//         (0, 1, 1968322),
//         (0, 2, 1628566),
//         (0, 3, 5112437),
//         (0, 4, 1505777),
//         (0, 5, 2018763),
//         (0, 6, 418564),
//         (0, 7, 3178396),
//         (0, 8, 3193609),
//         (0, 9, 1143520),
//         (1, 0, 1968322),
//         (1, 2, 3142137),
//         (1, 3, 5419755),
//         (1, 4, 1309183),
//         (1, 5, 2039012),
//         (1, 6, 1656253),
//         (1, 7, 3480732),
//         (1, 8, 3870772),
//         (1, 9, 2705997),
//         (2, 0, 1628566),
//         (2, 1, 3142137),
//         (2, 3, 4137151),
//         (2, 4, 2194450),
//         (2, 5, 1784503),
//         (2, 6, 1839604),
//         (2, 7, 2307545),
//         (2, 8, 2071110),
//         (2, 9, 582719),
//         (3, 0, 5112437),
//         (3, 1, 5419755),
//         (3, 2, 4137151),
//         (3, 4, 4166428),
//         (3, 5, 3361923),
//         (3, 6, 5022145),
//         (3, 7, 1943926),
//         (3, 8, 2145457),
//         (3, 9, 4180868),
//         (4, 0, 1505777),
//         (4, 1, 1309183),
//         (4, 2, 2194450),
//         (4, 3, 4166428),
//         (4, 5, 784687),
//         (4, 6, 1304514),
//         (4, 7, 2226406),
//         (4, 8, 2520499),
//         (4, 9, 1776705),
//         (5, 0, 2018763),
//         (5, 1, 2039012),
//         (5, 2, 1784503),
//         (5, 3, 3361923),
//         (5, 4, 784687),
//         (5, 6, 1933093),
//         (5, 7, 1425501),
//         (5, 8, 1677934),
//         (5, 9, 1479752),
//         (6, 0, 418564),
//         (6, 1, 1656253),
//         (6, 2, 1839604),
//         (6, 3, 5022145),
//         (6, 4, 1304514),
//         (6, 5, 1933093),
//         (6, 7, 3086599),
//         (6, 8, 3225667),
//         (6, 9, 1354266),
//         (7, 0, 3178396),
//         (7, 1, 3480732),
//         (7, 2, 2307545),
//         (7, 3, 1943926),
//         (7, 4, 2226406),
//         (7, 5, 1425501),
//         (7, 6, 3086599),
//         (7, 8, 593395),
//         (7, 9, 2373768),
//         (8, 0, 3193609),
//         (8, 1, 3870772),
//         (8, 2, 2071110),
//         (8, 3, 2145457),
//         (8, 4, 2520499),
//         (8, 5, 1677934),
//         (8, 6, 3225667),
//         (8, 7, 593395),
//         (8, 9, 2287913),
//         (9, 0, 1143520),
//         (9, 1, 2705997),
//         (9, 2, 582719),
//         (9, 3, 4180868),
//         (9, 4, 1776705),
//         (9, 5, 1479752),
//         (9, 6, 1354266),
//         (9, 7, 2373768),
//         (9, 8, 2287913),
//     ];

//     roadtrip(vcount, edges);
//     let duration = start.elapsed();

//     println!("Time elapsed in expensive_function() is: {:?}", duration);
// }

#[cfg(test)]
mod pub_tests;
