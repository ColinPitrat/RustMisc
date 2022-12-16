use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Elf {
    id: i32,
    calories: i64,
}

fn main() -> Result<(), Box<dyn Error>>  {
    let mut elves = vec!();
    let mut elf_id = 1;

    let filename = "sample.txt";
    //let filename = "my_input.txt";
    let content = fs::read_to_string(filename)?;

    for elf_content in content.split("\n\n") {
        println!("Elf content: {}", elf_content);
        let mut elf_calories = 0;
        for line in elf_content.to_string().split('\n') {
            if line.is_empty() {
                continue
            }
            elf_calories += line.parse::<i64>()?;
        }
        elves.push(Elf{id: elf_id, calories: elf_calories});
        elf_id += 1;
    }

    elves.sort_by_key(|e| -e.calories);

    let mut total = 0;
    println!("Top 3 elves are:\n");
    for i in 0..3 {
        println!(" - {:?}", elves[i]);
        total += elves[i].calories;
    }
    println!("For a total of {} calories", total);

    Ok(())
}
