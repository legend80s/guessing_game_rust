use ansi_term::Colour as CLR;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::string::String;

mod utils;

fn main() {
  let game_name = "GUESSING GAME";

  utils::welcome(game_name);

  utils::say("Enter a number to guess the secret number 🤔.");
  utils::say("Enter \"quit\" or \"Q\" to exit the game.\n");

  let secret = rand::thread_rng().gen_range(1..101);

  // utils::say(&format!("secret number is {}", secret));

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
      println!("{}", CLR::Green.paint(utils::quote("Good bye.")));
      break;
    }

    let number: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("{}", CLR::Red.paint(utils::quote("Not a number.")));
        continue;
      }
    };

    match number.cmp(&secret) {
      Ordering::Equal => {
        println!("{}", CLR::Green.paint(utils::quote(&format!(
          "You win! the secret number is {} 🎉.",
          secret
        ))));

        break;
      },

      Ordering::Greater => utils::say(&format!("You guessed {}. Too large.\n", CLR::Yellow.paint(number.to_string()))),
      Ordering::Less => utils::say(&format!("You guessed {}. Too small.\n", CLR::Green.paint(number.to_string()))),
    };
  }
}
