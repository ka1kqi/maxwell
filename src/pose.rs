#![allow(dead_code)]

use std::sync::OnceLock;

const FRAME_DELIM: &str = "~~~FRAME~~~";
const MAXWELL_TEXT: &str = include_str!("sprites/maxwell.txt");

pub struct Sprite {
    pub lines: Vec<&'static str>,
    pub height: usize,
    pub width: usize,
}

fn build_sprite(text: &'static str) -> Sprite {
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

static FRAMES: OnceLock<Vec<Sprite>> = OnceLock::new();

pub fn frames() -> &'static [Sprite] {
    FRAMES
        .get_or_init(|| {
            MAXWELL_TEXT
                .split(&format!("\n{FRAME_DELIM}\n"))
                .map(build_sprite)
                .collect()
        })
        .as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_are_loaded() {
        let f = frames();
        assert!(f.len() > 1, "should have multiple frames");
    }

    #[test]
    fn maxwell_has_expected_frame_count() {
        // The extracted Maxwell animation has 57 frames.
        assert_eq!(frames().len(), 57);
    }

    #[test]
    fn all_frames_have_consistent_dimensions() {
        let fs = frames();
        let h = fs[0].height;
        let w = fs[0].width;
        for (i, frame) in fs.iter().enumerate() {
            assert_eq!(frame.height, h, "frame {i} height differs");
            assert_eq!(frame.width, w, "frame {i} width differs");
        }
    }

    #[test]
    fn first_frame_is_nontrivial() {
        let f = &frames()[0];
        assert!(f.height >= 20, "should be tall enough to show a cat");
        assert!(f.width >= 80, "should be wide enough");
    }
}
