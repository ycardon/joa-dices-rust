mod dice;
mod parser;

fn main() {
    let faces = dice::roll(vec![
        (1, dice::RED),
        (1, dice::YELLOW),
        (1, dice::BLACK),
        (1, dice::DOOM),
        (1, dice::GIGANTIC),
        (1, dice::WHITE),
    ]);
    println!("{0:?}", faces);

    let (attack, defense, is_def) = parser::parse("12R 4D - 3B".to_string());
    println!(
        "attack: {0:?}\ndefense: {1:?}\nis_def: {2:?}",
        attack, defense, is_def
    )
}
