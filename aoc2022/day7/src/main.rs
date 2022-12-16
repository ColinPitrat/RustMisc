use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut cwd = "/".to_string();
    let mut dir_sizes: HashMap<String, i64> = HashMap::new();
    for l in lines {
        let l = l?;
        let line = l.chars().collect::<Vec<_>>();
        if line[0] == '$' {
            println!("Command: {}", line.iter().collect::<String>());
            // Command
            if line[2] == 'c' && line[3] == 'd' {
                let arg = line[5..].into_iter().collect::<String>();
                if line[4] == '/' {
                    cwd = arg;
                } else {
                    if arg == ".." {
                        let items = cwd.split('/').collect::<Vec<_>>();
                        cwd = items[0..items.len()-1].join("/")
                    } else {
                        cwd.push_str("/");
                        cwd.push_str(arg.as_str());
                        cwd = cwd.replace("//", "/");
                        cwd = cwd.replace("//", "/");
                    }
                    if cwd == "" {
                        cwd = "/".to_string();
                    }
                }
                println!("Changed directory to {}", cwd);
            } else if line[2] == 'l' && line[3] == 's' {
                println!("Listing {}", cwd);
            }
        } else {
            /*
            // This only look at the direct directory content
            println!("Parsing: {}", l);
            let mut parts = l.split(' ');
            let token = parts.next().unwrap();
            if token != "dir" {
                let size = token.parse::<i64>()?;
                let s = dir_sizes.entry(cwd.clone()).or_insert(0);
                *s += size;
                println!("Dir sizes: {:?}", dir_sizes);
            }
            */
            // This sums too much, counting all files indirectly
            println!("Parsing: {}", l);
            let mut parts = l.split(' ');
            let token = parts.next().unwrap();
            if token != "dir" {
                let mut dir_elements = cwd.split('/').collect::<Vec<_>>();
                // If directory is "/", the split contains 2 empty elements, we want only 1.
                if cwd == "/" {
                    dir_elements.pop();
                }
                println!("Dir elements: {:?}", dir_elements);
                let mut acwd = "".to_string();
                for d in dir_elements {
                    acwd.push_str("/");
                    acwd.push_str(d);
                    acwd = acwd.replace("//", "/");
                    println!("Adding to dir: {}", acwd);
                    let size = token.parse::<i64>()?;
                    let s = dir_sizes.entry(acwd.clone()).or_insert(0);
                    *s += size;
                    println!("Dir sizes: {:?}", dir_sizes);
                }
            }
        }

				let mut total_size_small_dirs = 0;
				for (_, sz) in &dir_sizes {
						if *sz <= 100000 {
								total_size_small_dirs += *sz;
						}
				}
				println!("Total size: {}", total_size_small_dirs);

				let disk_size: i64 = 70000000;
				let needed_space: i64 = 30000000;
				let used_space = *dir_sizes.entry("/".to_string()).or_insert(0);
				let to_free = needed_space - (disk_size - used_space);
				println!("Disk size: {} - Needed space: {} - Used space: {}", disk_size, needed_space, used_space);
				println!("Looking for directory of size > {} to delete", to_free);
				let mut dir_size_to_free = disk_size;
				for (_, sz) in &dir_sizes {
						if *sz > to_free && *sz < dir_size_to_free {
								dir_size_to_free = *sz;
						}
				}
				println!("Size of directory to delete: {}", dir_size_to_free);
    }

    Ok(())
}
