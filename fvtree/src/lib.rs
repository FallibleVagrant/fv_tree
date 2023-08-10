use std::fmt;
use common::Point;
use text_canvas;

pub struct PrintableFvtree {
    canvas: text_canvas::Canvas,
}

use std::convert::TryFrom;

impl PrintableFvtree {
    //Put down the root of all fvtrees, which looks like this:
    //
    //    O
    //   /|\
    //
    //Puts it below the x-axis so building proper starts at (0, 0).
    fn put_down_root(canvas: &mut text_canvas::Canvas) {
        canvas.put(Point {x: 0, y: -1}, 'O');
        canvas.put(Point {x: -1, y: -2}, '/');
        canvas.put(Point {x: 0, y: -2}, '|');
        canvas.put(Point {x: 1, y: -2}, '\\');
    }

    pub fn build(tree: &Fvtree) -> Result<PrintableFvtree, &'static str> {
        let mut cursor = Point {
            x: 0,
            y: 0,
        };
        let mut canvas = text_canvas::Canvas::new();

        //Convert to Sticks.
        let mut sticks: Vec<Stick> = Vec::new();
        for c in tree.tree_string.chars() {
            let stick = Stick::try_from(c)?;
            sticks.push(stick);
        }

        PrintableFvtree::put_down_root(&mut canvas);

        let mut branch_points: Vec<Point> = Vec::new();

        for stick in sticks {
//println!("{}", canvas);
            if stick.is_control_char() {
                match stick {
                    Stick::BranchIndicator => {
//println!("Putting branch down at {:?}.", cursor);
canvas.put(cursor, 'Y');
                        branch_points.push(cursor);
                    },
                    Stick::BranchReturn => {
                        cursor = branch_points.pop().expect("Found a BranchReset but not a corresponding BranchIndicator.");
                    },
                    _ => return Err("Encountered undefined control character?"),
                }
            }
            else {
                let cursor_move = stick.cursor_move()?;

                if !canvas.is_char_point(cursor, 'Y') {
//println!("Overwriting at {:?} with {}", cursor, stick.to_char());
                    canvas.put(cursor, stick.to_char());
                }
                cursor += cursor_move;
            }
        }

        return Ok(PrintableFvtree{canvas});
    }
}

impl fmt::Display for PrintableFvtree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canvas)
    }
}

pub struct Fvtree {
    tree_string: String,
}

mod sticks;
mod confstats;

use crate::sticks::Stick;
use crate::confstats::{Stats, Config};

mod tree_gen_recursive;

impl Fvtree {
    pub fn new() -> Fvtree {
        Fvtree::new_recursive()
    }

    pub fn new_recursive() -> Fvtree {
        let mut rng = rand::thread_rng();
        let conf = Config::new();
        let mut stats = Stats::new();

        if conf.t.max_sticks == 0 {
            return Fvtree{tree_string: "".to_string()};
        }

        let tree_string = tree_gen_recursive::gen(&mut rng, &mut stats, &conf);

        Fvtree{tree_string: tree_string.to_string()}
    }

    pub fn build(tree_string: &str) -> Fvtree {
        Fvtree{tree_string: tree_string.to_string()}
    }

    //pub fn build_procedural(config: &TreeConfig) -> Fvtree {
    //}

    pub fn tree_string(&self) -> String {
        self.tree_string.to_string()
    }
}

impl fmt::Display for Fvtree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree_string)
    }
}

#[cfg(test)]
mod fvtree_tests {
    use super::*;

    #[test]
    fn dont_panic() {
        assert!(true);
    }
}
