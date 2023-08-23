use std::convert::TryFrom;
use std::fmt;

use common::Point;

#[derive(Copy, Clone, Debug)]
pub enum Stick {
    UpBranch,
    UpLeftBranch,
    UpRightBranch,

    UpLeaf,
    LeftLeaf,
    RightLeaf,
    DownLeaf,

    UpLeftLeaf,
    UpRightLeaf,
    DownLeftLeaf,
    DownRightLeaf,

    BranchIndicator,
    BranchReturn,

    LeafSpawn,
    LeafReturn,
}

impl Stick {
    pub fn is_control_char(&self) -> bool {
        match self {
            Stick::UpBranch => UpBranch::is_control_char(),
            Stick::UpLeftBranch => UpLeftBranch::is_control_char(),
            Stick::UpRightBranch => UpRightBranch::is_control_char(),

            Stick::UpLeaf => UpLeaf::is_control_char(),
            Stick::LeftLeaf => LeftLeaf::is_control_char(),
            Stick::RightLeaf => RightLeaf::is_control_char(),
            Stick::DownLeaf => DownLeaf::is_control_char(),

            Stick::UpLeftLeaf => UpLeftLeaf::is_control_char(),
            Stick::UpRightLeaf => UpRightLeaf::is_control_char(),
            Stick::DownLeftLeaf => DownLeftLeaf::is_control_char(),
            Stick::DownRightLeaf => DownRightLeaf::is_control_char(),

            Stick::BranchIndicator => BranchIndicator::is_control_char(),
            Stick::BranchReturn => BranchReturn::is_control_char(),

            Stick::LeafSpawn => LeafSpawn::is_control_char(),
            Stick::LeafReturn => LeafReturn::is_control_char(),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Stick::UpBranch => UpBranch::to_char(),
            Stick::UpLeftBranch => UpLeftBranch::to_char(),
            Stick::UpRightBranch => UpRightBranch::to_char(),

            Stick::UpLeaf => UpLeaf::to_char(),
            Stick::LeftLeaf => LeftLeaf::to_char(),
            Stick::RightLeaf => RightLeaf::to_char(),
            Stick::DownLeaf => DownLeaf::to_char(),

            Stick::UpLeftLeaf => UpLeftLeaf::to_char(),
            Stick::UpRightLeaf => UpRightLeaf::to_char(),
            Stick::DownLeftLeaf => DownLeftLeaf::to_char(),
            Stick::DownRightLeaf => DownRightLeaf::to_char(),

            Stick::BranchIndicator => BranchIndicator::to_char(),
            Stick::BranchReturn => BranchReturn::to_char(),

            Stick::LeafSpawn => LeafSpawn::to_char(),
            Stick::LeafReturn => LeafReturn::to_char(),
        }
    }

    pub fn cursor_move(&self) -> Result<Point, &'static str> {
        match self {
            Stick::UpBranch => Ok(UpBranch::cursor_move()),
            Stick::UpLeftBranch => Ok(UpLeftBranch::cursor_move()),
            Stick::UpRightBranch => Ok(UpRightBranch::cursor_move()),

            Stick::UpLeaf => Ok(UpLeaf::cursor_move()),
            Stick::LeftLeaf => Ok(LeftLeaf::cursor_move()),
            Stick::RightLeaf => Ok(RightLeaf::cursor_move()),
            Stick::DownLeaf => Ok(DownLeaf::cursor_move()),

            Stick::UpLeftLeaf => Ok(UpLeftLeaf::cursor_move()),
            Stick::UpRightLeaf => Ok(UpRightLeaf::cursor_move()),
            Stick::DownLeftLeaf => Ok(DownLeftLeaf::cursor_move()),
            Stick::DownRightLeaf => Ok(DownRightLeaf::cursor_move()),

            _ => Err("Expected a canonical Stick that is not a control character."),
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Stick::UpBranch => UpBranch::is_leaf(),
            Stick::UpLeftBranch => UpLeftBranch::is_leaf(),
            Stick::UpRightBranch => UpRightBranch::is_leaf(),

            Stick::UpLeaf => UpLeaf::is_leaf(),
            Stick::LeftLeaf => LeftLeaf::is_leaf(),
            Stick::RightLeaf => RightLeaf::is_leaf(),
            Stick::DownLeaf => DownLeaf::is_leaf(),

            Stick::UpLeftLeaf => UpLeftLeaf::is_leaf(),
            Stick::UpRightLeaf => UpRightLeaf::is_leaf(),
            Stick::DownLeftLeaf => DownLeftLeaf::is_leaf(),
            Stick::DownRightLeaf => DownRightLeaf::is_leaf(),

            Stick::BranchIndicator => BranchIndicator::is_leaf(),
            Stick::BranchReturn => BranchReturn::is_leaf(),

            Stick::LeafSpawn => LeafSpawn::is_leaf(),
            Stick::LeafReturn => LeafReturn::is_leaf(),
        }
    }
}

//Perhaps this isn't necessary, but whatever.
const UP_BRANCH_C: char = '|';
const UPLEFT_BRANCH_C: char = '\\';
const UPRIGHT_BRANCH_C: char = '/';

const UP_LEAF_C: char = '^';
const LEFT_LEAF_C: char = '<';
const RIGHT_LEAF_C: char = '>';
const DOWN_LEAF_C: char = 'v';

const UPLEFT_LEAF_C: char = 't';
const UPRIGHT_LEAF_C: char = 'u';
const DOWNLEFT_LEAF_C: char = 'b';
const DOWNRIGHT_LEAF_C: char = 'm';

const BRANCH_INDICATOR_C: char = 'y';
const BRANCH_RETURN_C: char = 'r';

const LEAF_SPAWN_C: char = 'o';
const LEAF_RETURN_C: char = 'l';

impl TryFrom<char> for Stick {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            UP_BRANCH_C => Ok(Stick::UpBranch),
            UPLEFT_BRANCH_C => Ok(Stick::UpLeftBranch),
            UPRIGHT_BRANCH_C => Ok(Stick::UpRightBranch),

            UP_LEAF_C => Ok(Stick::UpLeaf),
            LEFT_LEAF_C => Ok(Stick::LeftLeaf),
            RIGHT_LEAF_C => Ok(Stick::RightLeaf),
            DOWN_LEAF_C => Ok(Stick::DownLeaf),

            UPLEFT_LEAF_C => Ok(Stick::UpLeftLeaf),
            UPRIGHT_LEAF_C => Ok(Stick::UpRightLeaf),
            DOWNLEFT_LEAF_C => Ok(Stick::DownLeftLeaf),
            DOWNRIGHT_LEAF_C => Ok(Stick::DownRightLeaf),

            BRANCH_INDICATOR_C => Ok(Stick::BranchIndicator),
            BRANCH_RETURN_C => Ok(Stick::BranchReturn),

            LEAF_SPAWN_C => Ok(Stick::LeafSpawn),
            LEAF_RETURN_C => Ok(Stick::LeafReturn),

            _ => Err("Expected one of the canonical Sticks."),
        }
    }
}

trait StickPrint: fmt::Display {
    fn cursor_move() -> Point;
}

pub trait StickCanonical {
    fn is_same_char(c: char) -> bool;
    fn to_char() -> char;
    fn is_control_char() -> bool;
    fn is_leaf() -> bool;
}

macro_rules! sticknonctrl {
    ($name: ty, $c: expr, $pointx: expr, $pointy: expr, $is_leaf: expr) => {
        impl StickPrint for $name {
            fn cursor_move() -> Point {
                Point {x: $pointx, y: $pointy}
            }
        }

        impl StickCanonical for $name {
            fn is_same_char(c: char) -> bool {
                $c == c
            }

            fn to_char() -> char {
                $c
            }

            fn is_control_char() -> bool {
                false
            }

            fn is_leaf() -> bool {
                $is_leaf
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", $c)
            }
        }
    }
}

//Branches.

pub struct UpBranch;
pub struct UpLeftBranch;
pub struct UpRightBranch;

sticknonctrl!(UpBranch, UP_BRANCH_C, 0, 1, false);
sticknonctrl!(UpLeftBranch, UPLEFT_BRANCH_C, -1, 1, false);
sticknonctrl!(UpRightBranch, UPRIGHT_BRANCH_C, 1, 1, false);

//Leaves.

pub struct UpLeaf;
pub struct LeftLeaf;
pub struct RightLeaf;
pub struct DownLeaf;

pub struct UpLeftLeaf;
pub struct UpRightLeaf;
pub struct DownLeftLeaf;
pub struct DownRightLeaf;

sticknonctrl!(UpLeaf, UP_LEAF_C, 0, 1, true);
sticknonctrl!(LeftLeaf, LEFT_LEAF_C, -1, 0, true);
sticknonctrl!(RightLeaf, RIGHT_LEAF_C, 1, 0, true);
sticknonctrl!(DownLeaf, DOWN_LEAF_C, 0, -1, true);

sticknonctrl!(UpLeftLeaf, UPLEFT_LEAF_C, -1, 1, true);
sticknonctrl!(UpRightLeaf, UPRIGHT_LEAF_C, 1, 1, true);
sticknonctrl!(DownLeftLeaf, DOWNLEFT_LEAF_C, -1, -1, true);
sticknonctrl!(DownRightLeaf, DOWNRIGHT_LEAF_C, 1, -1, true);

macro_rules! stickctrl {
    ($name: ty, $c: expr) => {
        impl StickCanonical for $name {
            fn is_same_char(c: char) -> bool {
                $c == c
            }

            fn to_char() -> char {
                $c
            }

            fn is_control_char() -> bool {
                true
            }

            fn is_leaf() -> bool {
                false
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", $c)
            }
        }
    }
}

//Sticks that don't render in the printed tree but exist in tree_string.
//i.e. control characters.
pub struct BranchIndicator;
pub struct BranchReturn;

stickctrl!(BranchIndicator, BRANCH_INDICATOR_C);
stickctrl!(BranchReturn, BRANCH_RETURN_C);

pub struct LeafSpawn;
pub struct LeafReturn;

stickctrl!(LeafSpawn, LEAF_SPAWN_C);
stickctrl!(LeafReturn, LEAF_RETURN_C);
