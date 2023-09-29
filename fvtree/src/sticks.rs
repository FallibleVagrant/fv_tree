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

//God this is dumb, there's definitely a better way to do this. (So why don't I take it?)
//Hacky macro to emulate enum_dispatch, i.e. each variant calls the same func.
//Uglier than enum_dispatch, but better than without.
macro_rules! match_func_on_each_variant {
    ($self: ident, $func: ident) => {
        match $self {
            Stick::UpBranch => UpBranch::$func(),
            Stick::UpLeftBranch => UpLeftBranch::$func(),
            Stick::UpRightBranch => UpRightBranch::$func(),

            Stick::UpLeaf => UpLeaf::$func(),
            Stick::LeftLeaf => LeftLeaf::$func(),
            Stick::RightLeaf => RightLeaf::$func(),
            Stick::DownLeaf => DownLeaf::$func(),

            Stick::UpLeftLeaf => UpLeftLeaf::$func(),
            Stick::UpRightLeaf => UpRightLeaf::$func(),
            Stick::DownLeftLeaf => DownLeftLeaf::$func(),
            Stick::DownRightLeaf => DownRightLeaf::$func(),

            Stick::BranchIndicator => BranchIndicator::$func(),
            Stick::BranchReturn => BranchReturn::$func(),

            Stick::LeafSpawn => LeafSpawn::$func(),
            Stick::LeafReturn => LeafReturn::$func(),
        }
    }
}

macro_rules! match_func_on_each_variant_except_ctrl_chars {
    ($self: ident, $func_name: ident, $error_msg: literal) => {
        match $self {
            Stick::UpBranch => Ok(UpBranch::$func_name()),
            Stick::UpLeftBranch => Ok(UpLeftBranch::$func_name()),
            Stick::UpRightBranch => Ok(UpRightBranch::$func_name()),

            Stick::UpLeaf => Ok(UpLeaf::$func_name()),
            Stick::LeftLeaf => Ok(LeftLeaf::$func_name()),
            Stick::RightLeaf => Ok(RightLeaf::$func_name()),
            Stick::DownLeaf => Ok(DownLeaf::$func_name()),

            Stick::UpLeftLeaf => Ok(UpLeftLeaf::$func_name()),
            Stick::UpRightLeaf => Ok(UpRightLeaf::$func_name()),
            Stick::DownLeftLeaf => Ok(DownLeftLeaf::$func_name()),
            Stick::DownRightLeaf => Ok(DownRightLeaf::$func_name()),

            _ => Err($error_msg),
        }
    }
}

impl Stick {
    pub fn is_control_char(&self) -> bool {
        match_func_on_each_variant!(self, is_control_char)
    }

    pub fn to_char(&self) -> char {
        match_func_on_each_variant!(self, to_char)
    }

    pub fn cursor_move(&self) -> Result<Point, &'static str> {
        match_func_on_each_variant_except_ctrl_chars!(self, cursor_move, "Expected a canonical Stick that is not a control character.")
    }

    pub fn is_leaf(&self) -> bool {
        match_func_on_each_variant!(self, is_leaf)
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

//Sticks that change the flow of the tree, and may or may not render.
//i.e. control characters.
//
//What is changing the flow of the tree?
//State.
//These characters change a bit of state while the tree is being constructed.
//BranchIndicator puts the current cursor position on a stack,
//LeafSpawn puts the tree into "leaf mode", so LeafReturn always returns to
//the point LeafSpawn was placed at.
pub struct BranchIndicator;
pub struct BranchReturn;

stickctrl!(BranchIndicator, BRANCH_INDICATOR_C);
stickctrl!(BranchReturn, BRANCH_RETURN_C);

pub struct LeafSpawn;
pub struct LeafReturn;

stickctrl!(LeafSpawn, LEAF_SPAWN_C);
stickctrl!(LeafReturn, LEAF_RETURN_C);
