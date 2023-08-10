use std::fmt;
use rand::Rng;
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
use crate::confstats::{Stats, Config, TreeStats, BranchStats};

//For returning BranchReturn char 'r' or BranchIndicator char 'y'.
use crate::sticks::{BranchIndicator, BranchReturn};
use crate::sticks::StickCanonical;

impl Fvtree {
    pub fn new() -> Fvtree {
        Fvtree::new_recursive()
    }

    //Returns a "stick" for the trunk, or None.
    fn gen_trunk_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
        let mut possible_outputs: Vec<Option<Stick>> = Vec::new();

        possible_outputs.push(Some(Stick::VerticalBranch));
        possible_outputs.push(Some(Stick::LeftBranch));
        possible_outputs.push(Some(Stick::RightBranch));

        if stats.b.height >= conf.t.min_height_before_trunk_branches {
            //The trunk may also branch.
            possible_outputs.push(Some(Stick::BranchIndicator));
        }

        if stats.b.height >= conf.t.min_trunk_height && stats.t.num_sticks >= conf.t.min_sticks {
            //We've reached the min_trunk_height and may stop now.
            possible_outputs.push(None);
        }

        let num = rng.gen_range(0..possible_outputs.len());

        if let Some(stick) = possible_outputs[num] {
            //Not BranchIndicator.
            if !stick.is_control_char() {
                //TODO: integrate height into add_one_stick().
                stats.b.height += 1;
                stats.add_one_stick();
            }
            //Is BranchIndicator.
            else {
                stats.add_one_branch();
            }
        }

//println!("Putting down {:?}", possible_outputs[num]);
        return possible_outputs[num];
    }

    fn gen_branch_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
        let mut possible_outputs: Vec<Option<Stick>> = Vec::new();

        possible_outputs.push(Some(Stick::VerticalBranch));
        possible_outputs.push(Some(Stick::LeftBranch));
        possible_outputs.push(Some(Stick::RightBranch));

        if stats.b.num_sticks >= conf.b.min_sticks_before_branch {
            possible_outputs.push(Some(Stick::BranchIndicator));
        }

        if stats.b.num_sticks >= conf.b.min_sticks {
            //We've attained min_sticks and may stop now.
            possible_outputs.push(Some(Stick::BranchReturn));
        }

        let num = rng.gen_range(0..possible_outputs.len());

        if let Some(stick) = possible_outputs[num] {
            if !stick.is_control_char() {
                stats.add_one_stick();
            }
            else {
                match stick {
                    Stick::BranchIndicator => {
                        stats.add_one_branch();
                    },
                    Stick::BranchReturn => {
                        stats.sub_one_branch();
                    },
                    _ => panic!("Undefined control character!"),
                }
            }
        }

//println!("Putting down {:?}", possible_outputs[num]);
        return possible_outputs[num];
    }

    fn gen_branches(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
        let mut branch: String = String::new();

        while stats.b.num_sticks < conf.b.max_sticks {
            let stick;

            match Fvtree::gen_branch_stick_or_stop(rng, stats, conf) {
                Some(Stick::BranchIndicator) => {
                    Fvtree::branch(&mut branch, rng, stats, conf);
                    continue;
                },
                Some(Stick::BranchReturn) => {
                    branch.push(BranchReturn::to_char());
                    return branch;
                },
                Some(s) => stick = s,
                //TODO: figure out what None does with branches later.
                None => panic!(),
            }

            branch.push(stick.to_char());
        }

        branch.push(BranchReturn::to_char());
        stats.sub_one_branch();
        return branch;
    }

    fn branch(current_branch: &mut String, rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) {
        current_branch.push(BranchIndicator::to_char());
        let current_branch_stats = stats.b;
        stats.b = BranchStats::new();

        let branch = Fvtree::gen_branches(rng, stats, conf);

        stats.b = current_branch_stats;

        current_branch.push_str(&branch);
    }

    fn gen_trunk(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
        let mut trunk: String = String::new();

        //The trunk is technically a branch, though it will still generate if
        //conf.t.min_trunk_height > conf.b.max_sticks.
        while stats.b.height < conf.t.max_trunk_height {
            let stick;

            match Fvtree::gen_trunk_stick_or_stop(rng, stats, conf) {
                Some(Stick::BranchIndicator) => {
                    Fvtree::branch(&mut trunk, rng, stats, conf);
                    continue;
                },
                Some(s) => stick = s,
                None => return trunk,
            }

            trunk.push(stick.to_char());
        }

        return trunk;
    }

    pub fn new_recursive() -> Fvtree {
        let mut rng = rand::thread_rng();
        let mut tree_string = String::new();
        let conf = Config::new();
        let mut stats = Stats::new();

        if conf.t.max_sticks == 0 {
            return Fvtree{tree_string: tree_string.to_string()};
        }

        let trunk = Fvtree::gen_trunk(&mut rng, &mut stats, &conf);
        tree_string.push_str(&trunk);

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
