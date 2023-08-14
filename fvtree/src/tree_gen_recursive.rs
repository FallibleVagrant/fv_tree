use rand::Rng;

use crate::sticks::Stick;
use crate::confstats::{Stats, Config, TreeStats, BranchStats};

//For returning BranchReturn char 'r' or BranchIndicator char 'y'.
use crate::sticks::{BranchIndicator, BranchReturn};
use crate::sticks::StickCanonical;

use crate::choose_amongst::Lottery;

//Returns a "stick" for the trunk, or None.
fn gen_trunk_stick_or_stop(rng: &mut rand::rngs::ThreadRng, stats: &mut Stats, conf: &Config) -> Option<Stick> {
    let mut possible_outputs: Vec<Option<Stick>> = Vec::new();

    if stats.b.height < conf.t.height_when_trunk_forced_to_branch || stats.t.has_branched {
        possible_outputs.push(Some(Stick::UpBranch));
        possible_outputs.push(Some(Stick::UpLeftBranch));
        possible_outputs.push(Some(Stick::UpRightBranch));
    }

    //conf.t.height_when_trunk_forced_to_branch is always >=
    //conf.t.min_height_before_trunk_can_branch,
    //so BranchIndicator is always a possible output when the other sticks aren't.
    if stats.b.height >= conf.t.min_height_before_trunk_can_branch {
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

    possible_outputs.push(Some(Stick::UpBranch));
    possible_outputs.push(Some(Stick::UpLeftBranch));
    possible_outputs.push(Some(Stick::UpRightBranch));

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
            None => panic!(),
        }

        branch_str.push(stick.to_char());
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
