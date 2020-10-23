mod dice;
mod parser;

use crate::dice::*;
use crate::parser::*;
use std::collections::HashMap;
use std::env;

// parse a command and print the resut
fn main() {
    let command = env::args().collect::<Vec<_>>()[1..].join(" ");
    let (attack_dice, defense_dice, is_def) = parse(&command);

    let attack = roll(attack_dice);
    let defense = roll(defense_dice);

    if is_def {
        println!(
            "attack:  {:?}\ndefense: {:?}\nresult:  {:?}",
            frequency(&attack),
            frequency(&defense),
            frequency(&apply_defense(attack, defense))
        )
    } else {
        println!("{:?}", frequency(&attack))
    }
}

// apply defence shields on the attack and remove unrelevant faces from the attack
fn apply_defense(attack: Roll, defense: Roll) -> Roll {
    let input = (attack, count(Face::Shield, &defense));
    let (ret, _) = cancel(Face::Push, cancel(Face::Disrupt, cancel(Face::Kill, input)));

    ret.into_iter()
        .filter(|f| *f != Face::Shield)
        .filter(|f| *f != Face::Blank)
        .collect::<Vec<_>>()
}

// cancel roll faces by an amount of shield count
fn cancel(face: Face, (roll, face_count): (Roll, usize)) -> (Roll, usize) {
    let mut n = face_count;
    let mut ret = Vec::new();

    for f in roll.iter() {
        if n <= 0 || *f != face {
            ret.push(*f)
        } else {
            n -= 1;
        }
    }
    (ret, n)
}

// count the number of a given face in a roll
fn count(face: Face, roll: &Roll) -> usize {
    roll.iter().filter(|f| **f == face).count()
}

// frequency of each face in the roll
fn frequency(roll: &Roll) -> HashMap<Face, usize> {
    let mut ret = HashMap::new();
    for r in roll {
        let n = ret.entry(*r).or_insert(0);
        *n += 1;
    }
    ret
}
