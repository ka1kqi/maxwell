#![allow(dead_code)]

use std::sync::OnceLock;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pose {
    Sit,
    Grass,
    Curled,
}

pub struct Sprite {
    pub lines: Vec<&'static str>,
    pub height: usize,
    pub width: usize,
}

const SIT_TEXT: &str = include_str!("sprites/sit.txt");
const GRASS_TEXT: &str = include_str!("sprites/grass.txt");
const CURLED_TEXT: &str = include_str!("sprites/curled.txt");

fn build_sprite(text: &'static str) -> Sprite {
    // Split on '\n'. Drop a trailing empty line if the file ends with a newline.
    let mut lines: Vec<&'static str> = text.split('\n').collect();
    if matches!(lines.last(), Some(&"")) {
        lines.pop();
    }
    let height = lines.len();
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    Sprite {
        lines,
        height,
        width,
    }
}

static SIT: OnceLock<Sprite> = OnceLock::new();
static GRASS: OnceLock<Sprite> = OnceLock::new();
static CURLED: OnceLock<Sprite> = OnceLock::new();

impl Pose {
    pub fn sprite(self) -> &'static Sprite {
        match self {
            Pose::Sit => SIT.get_or_init(|| build_sprite(SIT_TEXT)),
            Pose::Grass => GRASS.get_or_init(|| build_sprite(GRASS_TEXT)),
            Pose::Curled => CURLED.get_or_init(|| build_sprite(CURLED_TEXT)),
        }
    }

    pub fn next(self) -> Pose {
        match self {
            Pose::Sit => Pose::Grass,
            Pose::Grass => Pose::Curled,
            Pose::Curled => Pose::Sit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pose_next_cycles_sit_grass_curled() {
        assert_eq!(Pose::Sit.next(), Pose::Grass);
        assert_eq!(Pose::Grass.next(), Pose::Curled);
        assert_eq!(Pose::Curled.next(), Pose::Sit);
    }

    #[test]
    fn sit_sprite_loads_with_correct_dims() {
        let s = Pose::Sit.sprite();
        assert!(!s.lines.is_empty(), "sit sprite has lines");
        assert_eq!(s.height, s.lines.len());
        assert_eq!(
            s.width,
            s.lines.iter().map(|l| l.chars().count()).max().unwrap_or(0)
        );
    }

    #[test]
    fn all_three_sprites_load() {
        for pose in [Pose::Sit, Pose::Grass, Pose::Curled] {
            let s = pose.sprite();
            assert!(s.height > 0, "{:?} has rows", pose);
            assert!(s.width > 0, "{:?} has cols", pose);
        }
    }

    #[test]
    fn sprites_preserve_artist_credits() {
        let sit_text = Pose::Sit.sprite().lines.join("\n");
        let grass_text = Pose::Grass.sprite().lines.join("\n");
        let curled_text = Pose::Curled.sprite().lines.join("\n");
        assert!(sit_text.contains("[bug]"), "sit credits [bug]");
        assert!(grass_text.contains("fL"), "grass credits fL");
        assert!(curled_text.contains("dp"), "curled credits dp");
    }
}
