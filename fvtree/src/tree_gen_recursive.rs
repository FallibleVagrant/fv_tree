use rand::Rng;

use crate::sticks::Stick;
use crate::confstats::{Stats, Config, TreeStats, BranchStats};

//For returning BranchReturn char 'r' or BranchIndicator char 'y'.
use crate::sticks::{BranchIndicator, BranchReturn};
use crate::sticks::StickCanonical;

//For returning LeafSpawn char 'o' or LeafReturn char 'l'.
use crate::sticks::{LeafSpawn, LeafReturn};

use crate::choose_amongst::Lottery;

//Returns a "stick" for the trunk, or None.
fn gen_trunk_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
    let mut lottery: Lottery<Option<Stick>> = Lottery::build(rng);

    if stats.b.height < conf.t.height_when_trunk_forced_to_branch || stats.t.has_branched {
        lottery.add(Some(Stick::UpBranch));
        lottery.add(Some(Stick::UpLeftBranch));
        lottery.add(Some(Stick::UpRightBranch));
    }

    //conf.t.height_when_trunk_forced_to_branch is always >=
    //conf.t.min_height_before_trunk_can_branch,
    //so BranchIndicator is always a possible output when the other sticks aren't.
    if stats.b.height >= conf.t.min_height_before_trunk_can_branch {
        //The trunk may also branch.
        lottery.add(Some(Stick::BranchIndicator));
    }

    if stats.b.height >= conf.t.min_trunk_height && stats.t.num_sticks >= conf.t.min_sticks {
        //We've reached the min_trunk_height and may stop now.
        lottery.add(None);
    }

    let output = lottery.choose();

    if let Some(stick) = output {
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
    return output;
}

fn gen_branch_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
    let mut lottery: Lottery<Option<Stick>> = Lottery::build(rng);

    lottery.add(Some(Stick::UpBranch));
    lottery.add(Some(Stick::UpLeftBranch));
    lottery.add(Some(Stick::UpRightBranch));

    if stats.b.num_sticks >= conf.b.min_sticks_before_branch {
        lottery.add(Some(Stick::BranchIndicator));
    }

    if stats.b.num_sticks >= conf.b.min_sticks {
        //We've attained min_sticks and may stop now.
        lottery.add(None);
    }

    if stats.b.num_sticks >= conf.b.min_sticks && conf.b.allow_dead_branches {
        //We've attained min_sticks and may stop with a dead branch.
        lottery.add(Some(Stick::BranchReturn));
    }

    let output = lottery.choose();

    if let Some(stick) = output {
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
    return output;
}

fn gen_leaf_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
    if !stats.b.is_leaf_state {
        stats.b.is_leaf_state = true;
        return Some(Stick::LeafSpawn);
    }

    let mut lottery: Lottery<Option<Stick>> = Lottery::build(rng);

    if stats.b.num_leaves_in_leaflet < conf.b.max_leaves_in_leaflet {
        lottery.add(Some(Stick::UpLeaf));
        lottery.add(Some(Stick::LeftLeaf));
        lottery.add(Some(Stick::RightLeaf));
        lottery.add(Some(Stick::DownLeaf));

        lottery.add(Some(Stick::UpLeftLeaf));
        lottery.add(Some(Stick::UpRightLeaf));
        lottery.add(Some(Stick::DownLeftLeaf));
        lottery.add(Some(Stick::DownRightLeaf));
    }

    if stats.b.num_leaves_in_leaflet >= conf.b.min_leaves_in_leaflet
    && stats.b.num_leaflets < conf.b.max_leaflets {
        lottery.add(Some(Stick::LeafReturn));
    }

    if (stats.b.num_leaflets > 0 || conf.b.allow_dead_branches)
    && stats.b.num_leaves_in_leaflet >= conf.b.min_leaves_in_leaflet
    && stats.b.num_leaflets >= conf.b.min_leaflets {
        //We've attained min_leaves and may stop now,
        //but not if we need more leaves in current leaflet,
        //and not if we need more leaflets.
        lottery.add(None);
    }

    let output = lottery.choose();

    if let Some(stick) = output {
        if !stick.is_control_char() {
            stats.add_one_leaf();
        }
        else {
            match stick {
                Stick::LeafReturn => {
                    stats.b.num_leaves_in_leaflet = 0;
                    stats.b.num_leaflets += 1;
                },
                _ => panic!("Undefined control character!"),
            }
        }
    }

    return output;
}

fn gen_leaves(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
    let mut leaves = String::new();

    //There may be more than one LeafSpawn per leaves in future versions,
    //so can't just place one initially in leaves here.

    while true {
        match gen_leaf_stick_or_stop(rng, stats, conf) {
            Some(Stick::LeafSpawn) => {
                leaves.push(LeafSpawn::to_char());
            },
            Some(Stick::LeafReturn) => {
                leaves.push(LeafReturn::to_char());
            },
            Some(s) => leaves.push(s.to_char()),
            None => {
                stats.b.is_leaf_state = false;
                stats.b.num_leaves_in_leaflet = 0;
                stats.b.num_leaflets = 0;
                return leaves;
            },
        }
    }

    //Never reaches here.
    return leaves;
}

fn gen_branches(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
    let mut branch_str = String::new();

    while stats.b.num_sticks < conf.b.max_sticks {
        let stick;

        match gen_branch_stick_or_stop(rng, stats, conf) {
            Some(Stick::BranchIndicator) => {
                branch(&mut branch_str, rng, stats, conf);
                continue;
            },
            Some(Stick::BranchReturn) => {
                branch_str.push(BranchReturn::to_char());
                return branch_str;
            },
            Some(s) => stick = s,
            //TODO: figure out what None does with branches later.
            None => {
                if !conf.t.dead_tree {
                    branch_str.push_str(&gen_leaves(rng, stats, conf));
                }
                branch_str.push(BranchReturn::to_char());
                stats.sub_one_branch();
                return branch_str;
            },
        }

        branch_str.push(stick.to_char());
    }

    if !conf.b.allow_dead_branches && !conf.t.dead_tree {
        branch_str.push_str(&gen_leaves(rng, stats, conf));
    }
    branch_str.push(BranchReturn::to_char());
    stats.sub_one_branch();
    return branch_str;
}

fn branch(current_branch: &mut String, rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) {
    current_branch.push(BranchIndicator::to_char());
    let current_branch_stats = stats.b;
    stats.b = BranchStats::new();

    let branch = gen_branches(rng, stats, conf);

    stats.b = current_branch_stats;

    current_branch.push_str(&branch);
}

fn gen_trunk(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
    let mut trunk: String = String::new();

    //The trunk is technically a branch, though it will still generate if
    //conf.t.min_trunk_height > conf.b.max_sticks.
    while stats.b.height < conf.t.max_trunk_height {
        let stick;

        match gen_trunk_stick_or_stop(rng, stats, conf) {
            Some(Stick::BranchIndicator) => {
                branch(&mut trunk, rng, stats, conf);
                continue;
            },
            Some(s) => stick = s,
            None => return trunk,
        }

        trunk.push(stick.to_char());
    }

    return trunk;
}

pub fn gen(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> String {
    return gen_trunk(rng, stats, conf);
}
