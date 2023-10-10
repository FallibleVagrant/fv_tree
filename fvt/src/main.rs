use std::env;

use fvtree;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    let fvtree_string;
    let index_option = args.iter().position(|s| s == "-i");

    //If there is a -i, validate the tree_string following it at idx + 1.
    if index_option.is_some() {
        let idx = index_option.unwrap();

        if idx + 1 > args.len() - 1 {
            eprintln!("-i was enabled, but no tree_string followed it.");
            std::process::exit(-1);
        }
        else {
            let input_tree_string: String = args[idx + 1].clone();

            match fvtree::FvtreeString::build(&input_tree_string) {
                Ok(ts) => fvtree_string = ts,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(-1);
                },
            }
        }
    }
    //There is no -i, generate randomly.
    else {
        fvtree_string = fvtree::FvtreeString::new();
    }

    let print_tree_string: bool = args.iter().any(|s| s == "-t");
    let print_fvtree: bool = args.iter().any(|s| s == "-p");

    if print_tree_string {
        println!("{}", fvtree_string);
    }
    if print_fvtree {
        let fvtree = fvtree::Fvtree::build(&fvtree_string).unwrap();
        print!("{}", fvtree.to_string());
    }

    if !print_tree_string && !print_fvtree {
        println!("{}", fvtree_string);
    }
}
