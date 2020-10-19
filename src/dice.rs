use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Face {
  Kill,
  Disrupt,
  Push,
  Shield,
  Blank,
  Trample,
  Death,
  Rally,
  DelayedRally,
}

pub type Dice = [Face; 6];

pub const BLACK: Dice = [
  Face::Kill,
  Face::Disrupt,
  Face::Disrupt,
  Face::Shield,
  Face::Shield,
  Face::Shield,
];

pub const RED: Dice = [
  Face::Kill,
  Face::Kill,
  Face::Disrupt,
  Face::Disrupt,
  Face::Push,
  Face::Shield,
];

pub const YELLOW: Dice = [
  Face::Disrupt,
  Face::Push,
  Face::Push,
  Face::Shield,
  Face::Blank,
  Face::Blank,
];

pub const WHITE: Dice = [
  Face::Disrupt,
  Face::Disrupt,
  Face::Push,
  Face::Shield,
  Face::Shield,
  Face::Blank,
];

pub const GIGANTIC: Dice = [
  Face::Kill,
  Face::Disrupt,
  Face::Disrupt,
  Face::Push,
  Face::Trample,
  Face::Trample,
];

pub const DOOM: Dice = [
  Face::Disrupt,
  Face::Death,
  Face::Death,
  Face::Rally,
  Face::Rally,
  Face::DelayedRally,
];

// roll 1 dice
fn roll1(dice: Dice) -> Face {
  let mut rng = rand::thread_rng();
  return dice[rng.gen_range(0, dice.len())];
}

// roll n dices
fn rolln(n: usize, dice: Dice) -> Vec<Face> {
  let mut res = vec![];
  for _ in 0..n {
    res.push(roll1(dice));
  }
  return res;
}

// roll a list of dices
pub fn roll(dices: Vec<(usize, Dice)>) -> Vec<Face> {
  let mut res = vec![];
  for (n, d) in dices.iter() {
    res.extend(rolln(*n, *d))
  }
  return res;
}
