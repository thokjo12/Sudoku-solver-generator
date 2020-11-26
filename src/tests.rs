#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::cell::{RefCell, Ref};
    use std::borrow::BorrowMut;
    use itertools::Product;
    use std::slice::Iter;
    use crate::solver::Csp;

    #[test]
    fn revise_should_not_revise_on_valid_assignment() {
        let problem = Csp::new(|x,y| x!=y);
        let mut domain = problem.domains.clone();
        let a = problem.revise(&mut domain, (0, 0), (1, 1));
        assert_eq!(a, false)
    }

    #[test]
    fn get_all_possible_pairs_for_two_variables() {
        let a = (0..9).collect::<Vec<i32>>();
        let b = vec![3];
        let pairs = Csp::get_all_possible_pairs(&a, &b);
        let result = pairs.collect::<Vec<_>>();
        assert_eq!(result.len(), 9)
    }

    #[test]
    fn test_get_all_constraints_one_way_for_variable_with_reduced_space() {
        let a = (1..10).collect::<Vec<i32>>();
        let b = vec![3];
        let csp = Csp {
            variables: vec![],
            domains: {
                let mut map: HashMap<(i32, i32), RefCell<Vec<i32>>> = HashMap::new();
                map.insert((0, 0), RefCell::new(a.clone()));
                map.insert((0, 4), RefCell::new(b.clone()));
                map
            },
            constraint_filter: |x, y| { x != y },
            constraining_arcs: Vec::new(),
        };
        let result = csp.get_all_constraints_one_way((0, 0), (0, 4));
        assert_eq!(result.len(), 8)
    }

    #[test]
    fn test_generating_archs_for_neighbor_includes_hor_vert_and_box() {
        let mut csp = Csp::new(|x,y| x!=y);
        let mut result = csp.get_neighboring_arcs(&(0, 4));
        let mut target = vec![
            (0, 0), (0, 1), (0, 2),
            (0, 3), (1, 3), (2, 3),
            (1, 4), (2, 4), (3, 4),
            (4, 4), (5, 4), (6, 4),
            (7, 4), (8, 4), (0, 5),
            (1, 5), (2, 5), (0, 6),
            (0, 7), (0, 8),
        ];

        result.sort();
        target.sort();
        assert_eq!(result, target)
    }

    #[test]
    fn test_obtain_all_constraining_arcs(){
        let mut csp = Csp::new(|x,y| x!=y);
        let result = csp.generate_all_constraining_arcs();
        assert_eq!(result.len(),1620) // 81 variables, each one has 20 neighboring constraints, 81*20 = 1620
    }

    #[test]
    fn get_all_constraints_for_start_end_with_full_problem_space() {
        let mut csp = Csp::new(|x,y| x!=y);
        let start = (0, 4);
        let end = (0, 1);

        let constraints = csp.get_all_constraints_one_way(start, end);
        assert_eq!(constraints.len(), 72)
    }
}