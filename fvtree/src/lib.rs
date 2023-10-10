use std::fmt;
use common::Point;
use text_canvas;

pub struct Fvtree {
    canvas: text_canvas::Canvas,
}

use std::convert::TryFrom;

impl Fvtree {
    //Put down the root of all fvtrees, which looks like this:
    //
    //    O
    //   /|\
    //
    //Puts it at (0, 0) so proper building starts on top of it.
    fn put_down_root(canvas: &mut text_canvas::Canvas) {
        canvas.put(Point {x: 0, y: 0}, 'O');
        canvas.put(Point {x: -1, y: -1}, '/');
        canvas.put(Point {x: 0, y: -1}, '|');
        canvas.put(Point {x: 1, y: -1}, '\\');
    }

    pub fn build(tree: &FvtreeString) -> Result<Fvtree, &'static str> {
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

        Fvtree::put_down_root(&mut canvas);

        let mut branch_points: Vec<Point> = Vec::new();
        let mut leaf_spawn_point: Option<Point> = None;

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
                        //TODO: rewrite to return err.
                        cursor = branch_points.pop().expect("Found a BranchReset but not a corresponding BranchIndicator.");
                    },

                    Stick::LeafSpawn => {
                        leaf_spawn_point = Some(cursor);
                        canvas.put(cursor, 'O');
                    },
                    Stick::LeafReturn => {
                        if leaf_spawn_point.is_none() {
                            return Err("Found a LeafReturn but not a corresponding LeafSpawn.");
                        }
                        cursor = leaf_spawn_point.unwrap();
                    },

                    _ => return Err("Encountered undefined control character?"),
                }
            }
            else {
                let cursor_move = stick.cursor_move()?;

                //Don't overwrite branch points or seed spawns displayed on the canvas,
                //unless you are placing a leaf.
                if (canvas.is_char_point(cursor + cursor_move, 'Y') || canvas.is_char_point(cursor + cursor_move, 'O'))
                    && !stick.is_leaf() {
                    cursor += cursor_move;
                    continue;
                }

                //No longer in leaf-placing state.
                if !stick.is_leaf() {
                    leaf_spawn_point = None;
                }
                else {
                    //Always put a quote where a leaf would be -- may change later to alias leaves
                    //into looking like other characters.
                    cursor += cursor_move;
                    canvas.put(cursor, '\"');
                    continue;
                }

//println!("Overwriting at {:?} with {}", cursor, stick.to_char());
                cursor += cursor_move;
                canvas.put(cursor, stick.to_char());
            }
        }

        return Ok(Fvtree{canvas});
    }
}

impl fmt::Display for Fvtree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canvas)
    }
}

pub struct FvtreeString {
    tree_string: String,
}

mod sticks;
mod confstats;

use crate::sticks::Stick;
use crate::confstats::{Stats, Config};

mod tree_gen_recursive;
mod choose_amongst;

impl FvtreeString {
    pub fn new() -> FvtreeString {
        FvtreeString::new_recursive()
    }

    pub fn new_recursive() -> FvtreeString {
        let mut rng = rand::thread_rng();
        let conf = Config::new();
        let mut stats = Stats::new();

        if conf.t.max_sticks == 0 {
            return FvtreeString{tree_string: "".to_string()};
        }

        let tree_string = tree_gen_recursive::gen(&mut rng, &mut stats, &conf);

        FvtreeString{tree_string: tree_string.to_string()}
    }

    //TODO: proper recursive descent parser, but for now this will do.
    pub fn build(tree_string: &str) -> Result<FvtreeString, &'static String> {
        Ok(FvtreeString{tree_string: tree_string.to_string()})
    }

    //pub fn build_procedural(config: &TreeConfig) -> Fvtree {
    //}

    pub fn tree_string(&self) -> String {
        self.tree_string.to_string()
    }
}

impl fmt::Display for FvtreeString {
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
