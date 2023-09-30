use std::env;

use fvtree;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    let fvtree = fvtree::Fvtree::new();
    let fvtree_printable = fvtree::PrintableFvtree::build(&fvtree).unwrap();

    let print_tree_string: bool = args.iter().any(|s| s == "-t");
    let print_fvtree: bool = args.iter().any(|s| s == "-p");

    if print_tree_string {
        println!("{}\n", fvtree);
    }
    if print_fvtree {
        print!("{}", fvtree_printable.to_string());
    }

    if !print_tree_string && !print_fvtree {
        println!("{}\n", fvtree);
    }
}
