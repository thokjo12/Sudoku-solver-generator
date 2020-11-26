use std::collections::{HashSet, HashMap};
use std::borrow::{BorrowMut};
use std::cell::{RefCell};
use std::iter::{FromIterator};
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use rand::Rng;

pub struct Csp {
    pub variables: Vec<(i32, i32)>,
    // a position in the board x,y
    pub domains: HashMap<(i32, i32), RefCell<Vec<i32>>>,
    //legal values for a (x,y) position
    pub constraint_filter: fn(&i32, &i32) -> bool,
    //The constraint filter to determine legal moves

    pub constraining_arcs: Vec<((i32, i32), (i32, i32))>,
}

impl Csp {
    pub fn new(filter: fn(&i32, &i32) -> bool) -> Self {
        let vec = Vec::from_iter(0..9);
        let variables = Csp::get_all_possible_pairs(&vec, &vec).map(|(x, y)| { (*x, *y) }).collect::<Vec<_>>();
        let mut problem = Csp {
            variables: variables.clone(),
            domains: {
                let mut map: HashMap<(i32, i32), RefCell<Vec<i32>>> = HashMap::new();
                variables.iter().for_each(|item| {
                    map.insert(
                        item.clone(),
                        RefCell::new(Vec::from_iter(1..10)),
                    );
                });
                map
            },
            constraint_filter: filter,
            constraining_arcs: Default::default(),
        };
        problem.constraining_arcs = problem.generate_all_constraining_arcs();
        return problem;
    }

    pub fn create_sudoku(&self, threshold: f64) -> HashMap<(i32, i32), RefCell<Vec<i32>>> {
        let mut solution = self.backtrack(self.domains.clone()).unwrap();

        let mut rng = rand::thread_rng();
        for row in 0..9 {
            for col in 0..9 {
                if rng.gen::<f64>() <= threshold {
                    solution.entry((row, col)).and_modify(|item| {
                        item.borrow_mut().replace(vec![0]);
                    });
                }
            }
        }
        return solution;
    }

    pub fn backtrack(&self, domain: HashMap<(i32, i32), RefCell<Vec<i32>>>) -> Option<HashMap<(i32, i32), RefCell<Vec<i32>>>> {
        if domain.iter().all(|item| item.1.borrow().len() == 1) {
            return Some(domain);
        }

        let priority = self.minimum_remaining_values(&domain);

        let start = domain.get(&priority).unwrap().borrow();
        for value in start.iter() {
            let mut domain_copy = domain.clone();
            domain_copy.entry(priority).and_modify(|cell| { cell.replace(vec![value.clone()]); });
            if self.inference(&mut domain_copy, self.constraining_arcs.clone()) {
                let result = self.backtrack(domain_copy);

                if result != None {
                    return result;
                }
            }
        }
        return None;
    }

    pub fn inference(&self, domain: &mut HashMap<(i32, i32), RefCell<Vec<i32>>>, mut arcs: Vec<((i32, i32), (i32, i32))>) -> bool {
        while arcs.len() != 0 {
            let (arc_start, arc_end) = arcs.remove(0);

            if self.revise(domain, arc_start, arc_end) {
                if domain.get(&arc_start).unwrap().borrow().len() == 0 { // no legal values to assign, CSP failed, revise.
                    return false;
                }
                for neighbor in self.get_neighboring_arcs(&arc_start) { // get the whole neighbor set, without including variable2
                    arcs.push((neighbor, arc_start))
                }
            }
        }
        return true;
    }

    pub fn revise(&self, domain: &mut HashMap<(i32, i32), RefCell<Vec<i32>>>, arc_start: (i32, i32), arc_end: (i32, i32)) -> bool {
        let mut revised = false;

        let mut possible_start = domain.get(&arc_start)
            .unwrap()
            .borrow_mut();

        let possible_end = domain.get(&arc_end)
            .unwrap()
            .borrow();

        possible_start.retain(|variable| {
            let possible_constraints: HashSet<(i32, i32), RandomState> = HashSet::from_iter(possible_end.iter()
                .filter_map(|end| {
                    if *variable != *end {
                        Some((*variable, *end))
                    } else {
                        None
                    }
                }));
            let all_constraints: HashSet<(i32, i32), RandomState> = HashSet::from_iter(
                self.get_all_constraints_one_way(arc_start, arc_end).iter().cloned()
            );
            if possible_constraints.intersection(&all_constraints).collect::<Vec<_>>().len() == 0 {
                revised = true;
                false
            } else {
                true
            }
        });

        return revised;
    }

    pub fn generate_all_constraining_arcs(&self) -> Vec<((i32, i32), (i32, i32))> {
        let vec = Vec::from_iter(0..9);
        Csp::get_all_possible_pairs(&vec, &vec)
            .flat_map(|(x, y)| {
                self.get_neighboring_arcs(&(*x, *y)).iter()
                    .map(|(a, b)| { ((*x, *y), (*a, *b)) }).collect::<Vec<_>>()
            }).collect::<Vec<((i32, i32), (i32, i32))>>()
    }

    pub fn get_neighboring_arcs(&self, variable: &(i32, i32)) -> Vec<(i32, i32)> {
        let x = variable.0;
        let y = variable.1;
        let box_centers = vec![
            (1, 1), (4, 1), (7, 1),
            (1, 4), (4, 4), (7, 4),
            (1, 7), (4, 7), (7, 7)
        ];

        let center = *box_centers.iter().find(|(box_x, box_y)| {
            ((*box_x as f64 - x as f64).powi(2) + (*box_y as f64 - y as f64).powi(2)).sqrt() <= 2f64.sqrt()
        }).unwrap();

        let mut constraining_arcs = vec![
            (center.0 - 1, center.1 - 1), (center.0, center.1 - 1), (center.0 + 1, center.1 - 1),
            (center.0 - 1, center.1), center, (center.0 + 1, center.1),
            (center.0 - 1, center.1 + 1), (center.0, center.1 + 1), (center.0 + 1, center.1 + 1),
        ];

        let mut horizontal = (0..9).map(|x_pos| { (x_pos, y) }).collect::<Vec<(i32, i32)>>();
        let mut vertical = (0..9).map(|y_pos| { (x, y_pos) }).collect::<Vec<(i32, i32)>>();

        constraining_arcs.append(&mut horizontal);
        constraining_arcs.append(&mut vertical);
        constraining_arcs.into_iter()
            .unique()
            .filter(|tuple| { tuple != variable })
            .collect::<Vec<(i32, i32)>>()
    }

    pub fn get_all_possible_pairs<'a, 'b>(x: &'a Vec<i32>, y: &'b Vec<i32>) -> itertools::Product<std::slice::Iter<'a, i32>, std::slice::Iter<'b, i32>> {
        x.iter().cartesian_product(y)
    }


    pub fn get_all_constraints_one_way(&self, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
        let x = self.domains.get(&start).unwrap().borrow();
        let y = self.domains.get(&end).unwrap().borrow();

        let all_pairs_iter = Csp::get_all_possible_pairs(&x, &y);
        all_pairs_iter.filter(|(x, y)| { (self.constraint_filter)(*x, *y) })
            .map(|(x, y)| { (*x, *y) })
            .collect::<Vec<_>>()
    }

    pub fn minimum_remaining_values(&self, domain: &HashMap<(i32, i32), RefCell<Vec<i32>>>) -> (i32, i32) {
        let mut least_values = &(0, 0);

        for (key, value) in domain.into_iter() {
            if domain.get(&least_values).unwrap().borrow().len() > value.borrow().len() && value.borrow().len() != 1 {
                least_values = key;
            } else if domain.get(&least_values).unwrap().borrow().len() == 1 && value.borrow().len() != 1 {
                least_values = key
            }
        }

        return least_values.clone();
    }
}




