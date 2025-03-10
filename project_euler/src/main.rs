use std::time::Instant;

pub mod options;
pub mod utils;

pub mod fibonacci;
pub mod primes;

pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem78;
pub mod problem86;
pub mod problem88;
pub mod problem101;
pub mod problem172;
pub mod problem197;
pub mod problem611;
pub mod problem684;
pub mod problem686;
pub mod problem700;
pub mod problem710;
pub mod problem718;
pub mod problem751;
pub mod problem757;
pub mod problem800;
pub mod problem808;
pub mod problem816;
pub mod problem820;
pub mod problem822;
pub mod problem853;
pub mod problem862;
pub mod problem872;
pub mod problem885;
pub mod problem918;

fn main() {
    options::set_opts(argh::from_env());

    let problem = options::get_opts().problem;
    log_verbose!("Solving problem {problem}");

    let timer = Instant::now();
    match problem {
        1 => println!("Solution is {}", crate::problem1::solve(1000)),
        2 => println!("Solution is {}", crate::problem2::solve(4000000)),
        3 => println!("Solution is {}", crate::problem3::solve(600851475143)),
        4 => println!("Solution is {}", crate::problem4::solve(3).unwrap()),
        78 => println!("Solution is {}", crate::problem78::solve(1000000)),
        86 => println!("Solution is {}", crate::problem86::solve(1000000)),
        88 => println!("Solution is {}", crate::problem88::solve(2, 12000)),
        101 => println!("Solution is {}", crate::problem101::solve()),
        172 => println!("Solution is {}", crate::problem172::solve(18, 3)),
        197 => println!("Solution is {}", crate::problem197::solve()),
        611 => println!("Solution is {}", crate::problem611::solve(10_usize.pow(9))),
        684 => println!("Solution is {}", crate::problem684::solve(90, 1_000_000_007)),
        686 => println!("Solution is {}", crate::problem686::solve(123, 678910)),
        700 => println!("Solution is {}", crate::problem700::solve(None)),
        710 => println!("Solution is {}", crate::problem710::solve(1000000)),
        718 => println!("Solution is {}", crate::problem718::solve(10_u64.pow(14))),
        751 => println!("Solution is {}", crate::problem751::solve()),
        757 => println!("Solution is {}", crate::problem757::solve(10_i64.pow(14))),
        800 => println!("Solution is {}", crate::problem800::solve(800800)),
        808 => println!("Solution is {}", crate::problem808::solve(50)),
        816 => println!("Solution is {}", crate::problem816::solve(2000000)),
        820 => println!("Solution is {}", crate::problem820::solve(10_000_000)),
        822 => println!("Solution is {}", crate::problem822::solve(10_usize.pow(4), 10_usize.pow(16), 1234567891)),
        853 => println!("Solution is {}", crate::problem853::solve(120, 10_u128.pow(9))),
        862 => println!("Solution is {}", crate::problem862::solve(12)),
        872 => println!("Solution is {}", crate::problem872::solve(10_u64.pow(17), 9_u64.pow(17))),
        885 => println!("Solution is {}", crate::problem885::solve(18, 1123455689)),
        918 => println!("Solution is {}", crate::problem918::solve(10_usize.pow(12))),
        _ => println!("Unsolved problem {problem}"),
    }
    println!("Time: {:.2?}", timer.elapsed());
}
