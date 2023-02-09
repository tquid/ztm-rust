// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let tiles: Vec<Tile> = vec![
        Tile::Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 200,
        }),
        Tile::Wood,
        Tile::Water(Pressure(4)),
        Tile::Grass,
        Tile::Brick(BrickStyle::Gray),
        Tile::Brick(BrickStyle::Dungeon),
    ];
    let _: Vec<()> = tiles.into_iter().map(|tile| match tile {
        Tile::Brick(color @ (BrickStyle::Gray | BrickStyle::Red)) => {
            println!("The brick is color {:?}", color)
        }
        Tile::Brick(s @ _) => println!("{:?} brick", s),
        Tile::Grass | Tile::Sand | Tile::Dirt => println!("Ground tile"),
        Tile::Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount,
        }) if amount >= 100 => println!("Lots of gold!"),
        Tile::Water(Pressure(pressure)) if pressure >= 10 => println!("High water pressure!"),
        Tile::Water(Pressure(pressure)) => println!("Water pressure level {}", pressure),
        Tile::Treasure(_) | Tile::Wood => (),
    }).collect();
}
