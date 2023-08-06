pub struct TreeConfig {
    min_leaves: usize,
    min_trunk_height: usize,
    min_sticks: usize,
    min_sticks_in_branch: usize,
    max_sticks_in_branch: usize,
}

impl TreeConfig {
    pub fn new() -> TreeConfig {
        TreeConfig {
            min_leaves: 0,
            min_trunk_height: 0,
            min_sticks: 20,
            min_sticks_in_branch: 2,
            max_sticks_in_branch: 4,
        }
    }
}

struct TreeStats {
    num_leaves_in_current_branch: usize,
    height: usize,
    trunk_height: usize,
    num_sticks_in_current_branch: usize,
    branch_depth: usize,
}

impl TreeStats {
    pub fn new() -> TreeStats {
        TreeStats {
            num_leaves_in_current_branch: 0,
            height: 0,
            trunk_height: 0,
            num_sticks_in_current_branch: 0,
            branch_depth: 0,
        }
    }
}

use std::fmt;
use rand::Rng;
use crate::common::Point;
use crate::text_canvas;

pub struct PrintableFvtree {
    canvas: text_canvas::Canvas,
}

use std::convert::TryFrom;

impl PrintableFvtree {
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

        let mut branch_points: Vec<Point> = Vec::new();

        for stick in sticks {
//println!("{}", canvas);
            if stick.is_control_char() {
                match stick {
                    Stick::BranchIndicator => {
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

use crate::sticks::Stick;

impl Fvtree {
    //Returns a "stick", or None.
    fn gen_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut TreeStats, config: &TreeConfig) -> Option<Stick> {
        if stats.num_sticks_in_current_branch >= config.max_sticks_in_branch {
            let num = rng.gen_range(1..=100);
            
            if num < 40 {
                if stats.branch_depth > 0 {
                    stats.num_sticks_in_current_branch = 0;
                    stats.branch_depth -= 1;
                    return Some(Stick::BranchReturn);
                }
                else {
                    return None;
                }
            }
            if true {
                stats.num_sticks_in_current_branch = 0;
                stats.branch_depth += 1;
                return Some(Stick::BranchIndicator);
            }
        }

        match rng.gen_range(1..=5) {
            1 => {
                stats.num_sticks_in_current_branch += 1;
                Some(Stick::VerticalBranch)
            },
            2 => {
                stats.num_sticks_in_current_branch += 1;
                Some(Stick::LeftBranch)
            },
            3 => {
                stats.num_sticks_in_current_branch += 1;
                Some(Stick::RightBranch)
            },
            4 => {
                if stats.num_sticks_in_current_branch <= config.min_sticks_in_branch {
                    return Fvtree::gen_stick_or_stop(rng, stats, config);
                }
                if stats.branch_depth > 0 {
                    match rng.gen_range(1..=3) {
                        1 => {
                            stats.branch_depth += 1;
                            Some(Stick::BranchIndicator)
                        },
                        2 | 3 => {
                            stats.branch_depth -= 1;
                            Some(Stick::BranchReturn)
                        },
                        _ => {
println!("Didn't branch instead ended.");
                            return None;
                        },
                    }
                }
                else {
                    stats.branch_depth += 1;
                    Some(Stick::BranchIndicator)
                }
            },
            _ => {
                return None;
            },
        }
    }

    pub fn new() -> Fvtree {
        let mut rng = rand::thread_rng();
        let mut tree_string = String::new();
        let config = TreeConfig::new();
        let mut stats = TreeStats::new();

        loop {
            let stick = Fvtree::gen_stick_or_stop(&mut rng, &mut stats, &config);

            match stick {
                Some(stick) => tree_string.push(stick.to_char()),
                None => if tree_string.len() > config.min_sticks {
                    break;
                },
            }
        }

        Fvtree{tree_string: tree_string.to_string()}
    }

    pub fn build(tree_string: &str) -> Fvtree {
        Fvtree{tree_string: tree_string.to_string()}
    }

    pub fn build_with_config(tree_string: &str, config: &TreeConfig) -> Fvtree {
        Fvtree{tree_string: tree_string.to_string()}
    }

    pub fn tree_string(&self) -> String {
        self.tree_string.to_string()
    }
}

impl fmt::Display for Fvtree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree_string)
    }
}
