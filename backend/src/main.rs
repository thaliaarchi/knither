use std::env;

use knither::{codegen::Builder, knitout::Carrier};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Usage: knither WIDTH COLOR");
    }
    let width = args[1].parse::<i32>().unwrap();
    let mut builder = Builder::new(vec![
        (Carrier::C1, "Nylon draw thread".into()),
        (Carrier::C4, args[2].clone()),
    ]);
    builder.cast_on(Carrier::C4, width);
    builder.finish(Carrier::C4, width, 4);
    print!("{builder}");
}
