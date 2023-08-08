use std::convert::TryFrom;
use std::fmt;

use common::Point;

pub enum Stick {
    VerticalBranch,
    LeftBranch,
    RightBranch,
    BranchIndicator,
    BranchReturn,
}

impl Stick {
    pub fn is_control_char(&self) -> bool {
        match self {
            Stick::VerticalBranch => VerticalBranch::is_control_char(),
            Stick::LeftBranch => LeftBranch::is_control_char(),
            Stick::RightBranch => RightBranch::is_control_char(),
            Stick::BranchIndicator => BranchIndicator::is_control_char(),
            Stick::BranchReturn => BranchReturn::is_control_char(),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Stick::VerticalBranch => VerticalBranch::to_char(),
            Stick::LeftBranch => LeftBranch::to_char(),
            Stick::RightBranch => RightBranch::to_char(),
            Stick::BranchIndicator => BranchIndicator::to_char(),
            Stick::BranchReturn => BranchReturn::to_char(),
        }
    }

    pub fn cursor_move(&self) -> Result<Point, &'static str> {
        match self {
            Stick::VerticalBranch => Ok(VerticalBranch::cursor_move()),
            Stick::LeftBranch => Ok(LeftBranch::cursor_move()),
            Stick::RightBranch => Ok(RightBranch::cursor_move()),
            _ => Err("Expected a canonical Stick that is not a control character."),
        }
    }
}


impl TryFrom<char> for Stick {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Stick::VerticalBranch),
            '\\' => Ok(Stick::LeftBranch),
            '/' => Ok(Stick::RightBranch),
            'y' => Ok(Stick::BranchIndicator),
            'r' => Ok(Stick::BranchReturn),
            _ => Err("Expected one of the canonical Sticks."),
        }
    }
}

trait StickPrint: fmt::Display {
    fn cursor_move() -> Point;
}

trait StickCanonical {
    fn is_same_char(c: char) -> bool;
    fn to_char() -> char;
    fn is_control_char() -> bool;
}

macro_rules! sticknonctrl {
    ($name: ty, $c: expr, $pointx: expr, $pointy: expr) => {
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
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", $c)
            }
        }
    }
}

pub struct VerticalBranch;
pub struct LeftBranch;
pub struct RightBranch;

sticknonctrl!(VerticalBranch, '|', 0, 1);
sticknonctrl!(LeftBranch, '\\', -1, 1);
sticknonctrl!(RightBranch, '/', 1, 1);

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

stickctrl!(BranchIndicator, 'y');
stickctrl!(BranchReturn, 'r');
