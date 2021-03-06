# Sudoku-solver-generator
A relatively simple implementation of a Sudoku solver using the AC-3 algorithm and backtracking.

The sudokus are formulated as [Constraint satisfaction problems](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem):   
* variables:  
represents any position on the sudoku board.  
* domains:  
the legal values for any variable (1 to 9).    
* constraining_filter:  
the filter that is used to determine legal values.  
for a Sudoku this should be set to x!=y 
as we do not want to allow duplicate assignments,  
but it could be interesting to try different constraint filters.
* constraining_arcs:  
The constraining positions for any set position, this is autogenerated.


# Usage example
```Rust
use crate::solver::Csp;
mod solver;

fn main() {
    let problem = Csp::new(|x, y| { x != y });
    
    let result = problem.create_sudoku(0.5); 
    // 0.5 here is a threshold which 
    // says how much of the board we replace 
    // with zeros that represents empty cells
}
```

# Current "status"

- [x] Ability to create sudokus form scratch
- [ ] Add ability to parse sudokus and solve those from some format 
- [ ] Add ability to export partially solved sudokus 
