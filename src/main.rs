mod emoji;
mod git;

use emoji::*;
use git::*;
use std::env;

fn main() {
    let mut emojis = Emojis::new();
    let types = Type::all();
    let gx = GitX::new();

    let arg1 = env::args().nth(1).unwrap_or_default();
    match arg1.as_str() {
        "ji-test" => ji_test(emojis, types),

        "commit" => {
            let args = env::args().skip(2).collect();
            gx.exec_commit(args)
        }
        _ => {
            let args = env::args().skip(1).collect();
            gx.exec_git(args)
        }
    }
}

fn ji_test(mut emojis: Emojis, types: Vec<Type>) {
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
