mod emoji;
use emoji::*;

fn main() {
    let mut emojis = Emojis::new();

    for emoji in emojis.all().iter() {
        print!("{}\t", emoji);
    }
}
