mod dice;

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
}
