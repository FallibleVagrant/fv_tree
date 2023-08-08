use std::env;

use fvtree;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut args = env::args();

    let fvtree = fvtree::Fvtree::new();
    //println!("{}\n", fvtree);
    let fvtree = fvtree::PrintableFvtree::build(&fvtree).unwrap();

    if args.any(|s| s == "-p") {
        print!("{}", fvtree);
    }
    else {
        println!("else{}", fvtree.to_string());
    }
}
