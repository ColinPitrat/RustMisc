use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_numbers(numbers: &Vec<(i64, usize)>) {
    println!("{:?}", numbers.iter().map(|(n, p)| n).collect::<Vec<_>>());
}

fn reorder_numbers(numbers: &Vec<(i64, usize)>) -> Vec<(i64, usize)> {
    let mut numbers = numbers.clone();

    //print_numbers(&numbers);

    for to_move in 0..numbers.len() {
        for (i, (num, idx)) in numbers.clone().iter().enumerate() {
            if *idx != to_move {
                continue
            }
            //println!("Moving {}", num);
            if *num == 0 {
                break;
            }
            let mut new_pos = i as i64 + num;
            new_pos = new_pos.rem_euclid(numbers.len() as i64 - 1);
            if new_pos == 0 {
                new_pos = numbers.len() as i64 - 1;
            }
            numbers.remove(i);
            numbers.insert(new_pos as usize, (*num, *idx));
            break;
        }
        //print_numbers(&numbers);
    }

    //print_numbers(&numbers);

    numbers
}

fn grove_coordinates(numbers: &Vec<(i64, usize)>) -> i64 {
    let mut start_pos = 0;
    for (i, (v, _)) in numbers.iter().enumerate() {
        if *v == 0 {
            start_pos = i;
            println!("At start pos {}: {}", start_pos, numbers[start_pos].0);
            break
        }
    }
    let mut result = 0;
    for i in [1000, 2000, 3000] {
        let p = (start_pos + i) % numbers.len();
        result += numbers[p].0;
        println!("At {}: {}", i, numbers[p].0);
    }
    result
}

fn part1(numbers: &Vec<(i64, usize)>) -> i64 {
    let numbers = reorder_numbers(numbers);
    grove_coordinates(&numbers)
}

fn part2(numbers: &Vec<(i64, usize)>) -> i64 {
    let key = 811589153;
    let mut numbers = numbers.iter().map(|(x, b)| (x*key, *b)).collect::<Vec<_>>();

    for i in 0..10 {
        println!("Reorder {}", i);
        //print_numbers(&numbers);
        numbers = reorder_numbers(&numbers);
    }

    grove_coordinates(&numbers)
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let numbers = lines.enumerate().map(|(i, x)| (x.unwrap().parse::<i64>().unwrap(), i)).collect::<Vec<_>>();

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

    Ok(())
}
