pub mod options;
pub mod utils;
pub mod problem1;
pub mod problem2;
pub mod problem3;

fn main() {
    options::set_opts(argh::from_env());

    let problem = options::get_opts().problem;
    log_verbose!("Solving problem {problem}");

    match problem {
        1 => println!("Solution is {}", crate::problem1::solve(1000)),
        2 => println!("Solution is {}", crate::problem2::solve(4000000)),
        3 => println!("Solution is {}", crate::problem3::solve(600851475143)),
        _ => println!("Unsolved problem {problem}"),
    }
}
