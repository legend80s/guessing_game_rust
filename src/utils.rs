
use ansi_term::Colour;
use chrono::prelude::*;

pub fn quote(str: &str) -> String {
  return format!("🤖 {}", str);
}

pub fn say(str: &str) {
  println!("{}", quote(str));
}

pub fn welcome(name: &str) {
  let phase = get_time_phase();
  print!("\n");

  if phase.trim().len() > 0 {
    print!("             ");
    println!(
      "Good {}! Welcome to",
      phase[0..1].to_uppercase() + &phase[1..].to_lowercase()
    );
  }

  println!("                {} 🚀\n", Colour::Green.paint(name));
}

pub fn get_time_phase() -> &'static str {
  let now = Local::now();
  let hour = now.hour();

  // println!("hour is {}", hour);

  match hour {
    6..=12 => "morning",
    13..=17 => "afternoon",
    18..=23 => "night",
    0..=5 => "night",
    _ => "",
  }
}
