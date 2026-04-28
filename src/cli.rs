#![allow(dead_code)]

use crate::pose::Pose;
use clap::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Cycle,
    Pinned(Pose),
}

#[derive(Parser, Debug)]
#[command(version, about = "A calm cat companion for your terminal", long_about = None)]
pub struct Cli {
    /// Pin the sitting cat (no pose cycling)
    #[arg(long, group = "pose")]
    pub sit: bool,

    /// Pin the cat-in-grass pose
    #[arg(long, group = "pose")]
    pub grass: bool,

    /// Pin the curled-up sleeping cat
    #[arg(long, group = "pose")]
    pub curled: bool,
}

impl Cli {
    pub fn into_mode(self) -> Mode {
        if self.sit {
            Mode::Pinned(Pose::Sit)
        } else if self.grass {
            Mode::Pinned(Pose::Grass)
        } else if self.curled {
            Mode::Pinned(Pose::Curled)
        } else {
            Mode::Cycle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Cli {
        // First arg is binary name; clap requires it.
        let mut full = vec!["pet"];
        full.extend_from_slice(args);
        Cli::try_parse_from(full).expect("parse")
    }

    #[test]
    fn no_flags_yields_cycle_mode() {
        assert_eq!(parse(&[]).into_mode(), Mode::Cycle);
    }

    #[test]
    fn sit_flag_yields_pinned_sit() {
        assert_eq!(parse(&["--sit"]).into_mode(), Mode::Pinned(Pose::Sit));
    }

    #[test]
    fn grass_flag_yields_pinned_grass() {
        assert_eq!(parse(&["--grass"]).into_mode(), Mode::Pinned(Pose::Grass));
    }

    #[test]
    fn curled_flag_yields_pinned_curled() {
        assert_eq!(parse(&["--curled"]).into_mode(), Mode::Pinned(Pose::Curled));
    }

    #[test]
    fn two_flags_is_a_parse_error() {
        let result = Cli::try_parse_from(["pet", "--sit", "--grass"]);
        assert!(result.is_err(), "clap should reject mutually-exclusive flags");
    }
}
