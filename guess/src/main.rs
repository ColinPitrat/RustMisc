extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn pick_number() -> u32 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0, 101);
}

fn ask_for_guess() -> u32 {
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Couldn't read from stdio");
    match guess.trim().parse::<u32>() {
      Ok(i) => return i,
      Err(_) => continue,
    };
  }
}

fn one_play() -> u32 {
  let to_guess = pick_number();
  let mut tries = 0;
  loop {
    println!("What's your guess ?");

    tries += 1;
    match ask_for_guess().cmp(&to_guess) {
      Ordering::Less => println!("Too small"),
      Ordering::Equal => {
      	println!("Well done !");
	break;
      }
      Ordering::Greater => println!("Too big"),
    };
  }
  tries
}

fn play_again() -> bool {
  loop {
    println!("Play again (y/n) ?");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Couldn't read from stdio");
    match answer.trim() {
      "y" | "yes" | "Y" | "YES" => return true,
      "n" | "no" | "N" | "NO" => return false,
      _ => continue,
    };
  }
}

fn main() {
  println!("Guess the number !");

  loop {
    one_play();
    match play_again() {
      true => continue,
      false => break,
    }
  }

  println!("Game over ! Get back to work.");
}
