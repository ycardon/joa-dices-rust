use crate::dice::*;

// parse the command line
pub fn parse(command: &String) -> (Vec<(usize, Dice)>, Vec<(usize, Dice)>, bool) {
  let mut attack = Vec::new();
  let mut defense = Vec::new();
  let mut is_defense = false;

  for s in command.split_whitespace() {
    if s == "-" {
      is_defense = true
    } else {
      let mut n = String::from(s);
      let d = n.pop().unwrap();
      match parse_dice(d) {
        None => (),
        Some(dice) => {
          if is_defense {
            defense.push((parse_int(n), dice))
          } else {
            attack.push((parse_int(n), dice))
          }
        }
      }
    }
  }
  (attack, defense, is_defense)
}

// returns a number or 1
fn parse_int(n: String) -> usize {
  match n.parse::<usize>() {
    Ok(i) => i,
    Err(_) => 1,
  }
}

// return some dice or none
fn parse_dice(d: char) -> Option<Dice> {
  match d {
    'B' => Some(BLACK),
    'R' => Some(RED),
    'Y' => Some(YELLOW),
    'W' => Some(WHITE),
    'G' => Some(GIGANTIC),
    'D' => Some(DOOM),
    _ => None,
  }
}
