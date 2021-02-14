use ansi_term::Colour as CLR;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::string::String;
use chrono::prelude::*;

fn main() {
  let game_name = "GUESSING GAME";

  welcome(game_name);

  say("Enter a number to guess the secret number 🤔.");
  say("Enter \"quit\" or \"Q\" to exit the game.\n");

  let secret = rand::thread_rng().gen_range(1..101);

  // say(&format!("secret number is {}", secret));

  loop {
    let error_msg = format!(
      "Failed to read line because of io exception. {} exited!",
      game_name
    );

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect(&CLR::Red.paint(error_msg));

    guess = guess.trim().to_string();

    if guess.to_lowercase() == "quit" || guess.to_uppercase() == "Q" {
      println!("{}", CLR::Green.paint(quote("Good bye.")));
      break;
    }

    let number: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("{}", CLR::Red.paint(quote("Not a number.")));
        continue;
      }
    };

    match number.cmp(&secret) {
      Ordering::Equal => {
        println!("{}", CLR::Green.paint(quote(&format!(
          "You win! the secret number is {} 🎉.",
          secret
        ))));

        break;
      },

      Ordering::Greater => say(&format!("You guessed {}. Too large.\n", CLR::Yellow.paint(number.to_string()))),
      Ordering::Less => say(&format!("You guessed {}. Too small.\n", CLR::Green.paint(number.to_string()))),
    };
  }
}

fn quote(str: &str) -> String {
  return format!("🤖 {}", str);
}

fn say(str: &str) {
  println!("{}", quote(str));
}

fn welcome(name: &str) {
  let phase = get_time_phase();
  print!("\n");

  if phase.trim().len() > 0 {
    print!("             ");
    println!("Good {}! Welcome to", phase[0..1].to_uppercase() + &phase[1..].to_lowercase());
  }

  println!("                {} 🚀\n", CLR::Green.paint(name));
}

fn get_time_phase() -> &'static str {
  let now = Local::now();
  let hour = now.hour();

  // println!("hour is {}", hour);

  match hour {
    6..=12 => "morning",
    13..=17 => "afternoon",
    18..=23 => "night",
    0..=5 => "night",
    _ => ""
  }
}
