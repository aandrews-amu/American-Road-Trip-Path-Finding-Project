use super::{Domain, ValueType, Var};
use crate::counting_sat::{Lit, CSAT};
use std::collections::{HashMap};
use crate::csp::CSP;
use crate::fd::{Reify, FD};

impl ValueType for usize {}

pub fn roadtrip(
    vcount: usize,
    edges: &[(usize, usize, u32)],
) -> (Vec<(usize, usize, u32)>, u32) {
    let mut csp = CSP::new();
    
    let edges_map: HashMap<((usize, usize), u32)> = edges.iter().map(|(from, to, cost)| ((from, to), cost)).collect();
    // create variables for each time choice for vertices
    let choices = (0..vcount.len()).map(|i| {
        if i == 0 {
            csp.add_variable(Domain::new(vec![0]))
        } else {
            csp.add_variable(Domain::new((0..vcount.len()).collect()))
        }
    }).collect();

    let mut cycle = choices.clone();
    cycle.push(choices[0]);

    // add vertices must be different
    csp.add_constraint(FD::AllDiff(choices));
    

    // generate solution
    let soln = csp.bnb(
        |v| {
            cycle.windows(2).map(|cs| 
                {
                    let from = cs[0];
                    let to = cs[1];
                    match (v.get_value(from), v.get_value(to)) {
                        (Some(from), Some(to)) => {
                            edges_map[(from, to)]
                        },
                        _ => &(0 as u32)
                    }
                }    
            ).sum() 
        },
        u32::MAX,
    );

    // FIX
    let net_cost: u32 = choices.windows(2).next().map(|[from, to]| 
        {
            if edges_map.contains_key((v.get_value(from), v.get_value(to))) {
                edges_map.get((v.get_value(from), v.get_value(to)))
            }
            else {
                &(0 as u32)
            }
        }    


 
        // FIX: cycle.iter().map( get values and put into vec)
    // let ordered_edges: Vec<_> = edges
    //     .iter()
    //     .zip(all_edges.iter())
    //     .filter(|(_e, edge_var)| soln.as_ref().unwrap()[**edge_var])
    //     .map(|(e, _edge_var)| *e)
    //     .collect();

    // return vertices that are true and net cost
    (choices, net_cost)
}
