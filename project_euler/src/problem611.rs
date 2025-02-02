use crate::log_verbose;
use itertools::iproduct;
use std::collections::HashSet;
use std::time::Instant;

pub fn solve(n: usize) -> usize {
    let squares = (1..=((n as f64).sqrt() as usize))
        .map(|x| x*x)
        .collect::<Vec<_>>();
    let mut opened = HashSet::new();
    let chunk_size = 1024;
    let mut result = 0;
    let mut timer = Instant::now();
    let nb_chunks = (squares.len()-1) / chunk_size;
    let mut chunks = iproduct!(0..=nb_chunks, 0..=nb_chunks).filter(|(i,j)| i <= j).collect::<Vec<_>>();
    chunks.sort_by_key(|(i, j)| squares[i*chunk_size] + squares[j*chunk_size]);
    for (k, (i, j)) in chunks.iter().enumerate() {
        if k % 1024 == 0 {
            log_verbose!("Chunk {k:5}/{} - {} - {:.2?}", chunks.len(), opened.len(), timer.elapsed());
            timer = Instant::now();
        }
        let end_a = std::cmp::min((i+1)*chunk_size, squares.len());
        let end_b = std::cmp::min((j+1)*chunk_size, squares.len());
        for a in squares[i*chunk_size..end_a].iter() {
            for b in squares[j*chunk_size..end_b].iter() {
                if b <= a {
                    continue;
                }
                if a + b > n {
                    break;
                }
                let door = a+b;
                if opened.contains(&door) {
                    //log_verbose!("Closing {door}");
                    opened.remove(&door);
                    result -= 1;
                } else {
                    //log_verbose!("Opening {door}");
                    opened.insert(door);
                    result += 1;
                }
            }
        }
        let retain = squares[i*chunk_size] + squares[j*chunk_size];
        opened.retain(|&x| x >= retain);
    }
    //log_verbose!("Opened: {opened:?}");
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(1, solve(5));
        assert_eq!(27, solve(100));
        assert_eq!(233, solve(1000));
        //assert_eq!(112168, solve(10_usize.pow(6)));
    }
}
