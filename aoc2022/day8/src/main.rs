use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn score(i: usize, j: usize, map: &Vec<Vec<i32>>) -> i32 {
    let height = map.len();
    let width = map.last().unwrap().len();

    let mut left = 0;
    for a in 0..i {
        if i-a == 0 {
            break;
        }
        if map[i-a-1][j] < map[i][j] {
            left += 1;
        } else {
            left += 1;
            break;
        }
    }

    let mut right = 0;
    for a in i..width {
        if a == i {
            continue;
        }
        if map[a][j] < map[i][j] {
            right += 1;
        } else {
            right += 1;
            break;
        }
    }

    let mut up = 0;
    for a in 0..j {
        if j-a == 0 {
            break;
        }
        if map[i][j-a-1] < map[i][j] {
            up += 1;
        } else {
            up += 1;
            break;
        }
    }

    let mut down = 0;
    for a in j..height {
        if a == j {
            continue;
        }
        if map[i][a] < map[i][j] {
            down += 1;
        } else {
            down += 1;
            break;
        }
    }

    left * right * up * down
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut map = vec!();
    let mut visible_map = vec!();
    let mut scores_map = vec!();
    for l in lines {
        let l = l?;
        map.push(vec!());
        visible_map.push(vec!());
        scores_map.push(vec!());
        for c in l.chars() {
            let height = c.to_string().parse::<i32>()?;
            map.last_mut().unwrap().push(height);
            visible_map.last_mut().unwrap().push(false);
            scores_map.last_mut().unwrap().push(-1);
        }
    }

    // We assume all lines are the same length
    let height = map.len();
    let width = map.last().unwrap().len();

    for i in 0..height {
        let mut max_height_left = -1;
        let mut max_height_right = -1;
        for j in 0..width {
            if map[i][j] > max_height_left {
                visible_map[i][j] = true;
                max_height_left = map[i][j];
            }
            if map[i][width-j-1] > max_height_right {
                visible_map[i][width-j-1] = true;
                max_height_right = map[i][width-j-1];
            }
        }
    }

    for j in 0..width {
        let mut max_height_up = -1;
        let mut max_height_down = -1;
        for i in 0..height {
            if map[i][j] > max_height_up {
                visible_map[i][j] = true;
                max_height_up = map[i][j];
            }
            if map[height-i-1][j] > max_height_down {
                visible_map[height-i-1][j] = true;
                max_height_down = map[height-i-1][j];
            }
        }
    }

    //println!("Map: {:?}", map);
    //println!("Visible map: {:?}", visible_map);

    let visible_trees = visible_map.iter().flatten().filter(|x| **x).count();
    println!("Visible trees: {}", visible_trees);

    // Part 2
    for i in 0..height {
        for j in 0..width {
            scores_map[i][j] = score(i, j, &map);
        }
    }
    //println!("Score trees: {:?}", scores_map);

    let highest_scenic_score = scores_map.iter().flatten().max().unwrap();
    println!("Highest scenic score: {}", highest_scenic_score);

    Ok(())
}
