use crate::solver::Csp;
use std::collections::HashMap;
use std::cell::RefCell;
use rand::Rng;

mod solver;
mod tests;

fn main() {
    println!("Hello, world!");
    let problem = Csp::new(|x, y| { x != y });
    let result = problem.create_sudoku(0.5);
    printboard(result.clone());
}


fn printboard(sol: HashMap<(i32, i32), RefCell<Vec<i32>>>) {
    let mut rng = rand::thread_rng();

    for row in 0..9 {
        for col in 0..9 {
            print!("{} ", sol.get(&(row, col)).unwrap().borrow()[0]);
            if col == 2 || col == 5 {
                print!("| ")
            }
        }
        println!();
        if row == 2 || row == 5 {
            println!("------+-------+------");
        }
    }
}



