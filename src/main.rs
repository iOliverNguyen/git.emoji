mod emoji;
mod git;

use emoji::*;
use git::*;

fn main() {
    let mut emojis = Emojis::new();
    let types = Type::all();

    for emoji in emojis.all().iter() {
        print!("{}\t", emoji);
    }
    for typ in types.iter() {
        print!("{:>20}  ", &typ.name);
        for emoji in typ.emoji.iter() {
            print!("{}  ", emoji);
        }
        println!()
    }

    let gx = GitX::new();
    println!("gitemoji: {:?}", gx.gitemoji);
    println!("original: {:?}", gx.original);
}
