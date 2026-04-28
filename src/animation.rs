#![allow(dead_code)]

use std::time::{Duration, Instant};
use crate::cli::Mode;
use crate::pose::Pose;

const POSE_HOLD: Duration = Duration::from_secs(8);
const BREATH_PERIOD: Duration = Duration::from_millis(2500);

pub struct AnimState {
    mode: Mode,
    current_pose: Pose,
    elapsed: Duration,
}

impl AnimState {
    pub fn new(mode: Mode) -> Self {
        let current_pose = match &mode {
            Mode::Pinned(p) => *p,
            Mode::Cycle => Pose::Sit,
        };
        Self { mode, current_pose, elapsed: Duration::ZERO }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.elapsed += dt;
        if matches!(self.mode, Mode::Cycle) {
            // Pose index based on total elapsed; deterministic, no drift.
            let secs = self.elapsed.as_secs();
            let idx = (secs / POSE_HOLD.as_secs()) % 3;
            self.current_pose = match idx {
                0 => Pose::Sit,
                1 => Pose::Grass,
                2 => Pose::Curled,
                _ => unreachable!(),
            };
        }
    }

    pub fn current_pose(&self) -> Pose {
        self.current_pose
    }

    /// 0 or 1: vertical bob offset for breathing.
    pub fn breathing_offset(&self) -> usize {
        let half_periods = (self.elapsed.as_millis() / BREATH_PERIOD.as_millis()) as usize;
        half_periods % 2
    }
}

use cellophane::{Animation, Cell, Frame};
use cellophane::crossterm::style::Color;

const LAVENDER: Color = Color::Rgb { r: 177, g: 156, b: 217 };

pub struct CatAnimation {
    state: AnimState,
    rows: usize,
    cols: usize,
    last_update: Option<Instant>,
}

impl CatAnimation {
    pub fn new(mode: Mode) -> Self {
        Self { state: AnimState::new(mode), rows: 0, cols: 0, last_update: None }
    }
}

impl Animation for CatAnimation {
    fn init_with(&mut self, initial: Frame) {
        let (rows, cols) = initial.dims().unwrap_or((0, 0));
        self.rows = rows;
        self.cols = cols;
    }

    fn update(&mut self) -> Frame {
        let now = Instant::now();
        let dt = self.last_update.map_or(Duration::ZERO, |t| now.saturating_duration_since(t));
        self.last_update = Some(now);
        self.state.tick(dt);

        let mut frame = Frame::with_capacity(self.cols, self.rows);
        let sprite = self.state.current_pose().sprite();

        // Center horizontally; center vertically with breathing offset.
        let start_col = self.cols.saturating_sub(sprite.width) / 2;
        let mid_row = self.rows.saturating_sub(sprite.height) / 2;
        let start_row = mid_row + self.state.breathing_offset();

        for (line_idx, line) in sprite.lines.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    continue; // preserve transparency over background
                }
                let row = start_row + line_idx;
                let col = start_col + col_idx;
                if let Some(cell) = frame.get_cell_mut(row, col) {
                    *cell = Cell::default().with_char(ch).with_fg(LAVENDER);
                }
            }
        }

        frame
    }

    fn is_done(&self) -> bool { false }

    fn resize(&mut self, w: usize, h: usize) {
        self.cols = w;
        self.rows = h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_mode_starts_at_sit() {
        let s = AnimState::new(Mode::Cycle);
        assert_eq!(s.current_pose(), Pose::Sit);
    }

    #[test]
    fn pinned_mode_starts_at_pinned_pose() {
        let s = AnimState::new(Mode::Pinned(Pose::Curled));
        assert_eq!(s.current_pose(), Pose::Curled);
    }

    #[test]
    fn cycle_advances_at_8_second_boundaries() {
        let mut s = AnimState::new(Mode::Cycle);
        s.tick(Duration::from_secs(7));
        assert_eq!(s.current_pose(), Pose::Sit, "still Sit at 7s");
        s.tick(Duration::from_secs(2)); // total 9s
        assert_eq!(s.current_pose(), Pose::Grass, "Grass after first 8s boundary");
        s.tick(Duration::from_secs(8)); // total 17s
        assert_eq!(s.current_pose(), Pose::Curled, "Curled after second boundary");
        s.tick(Duration::from_secs(8)); // total 25s
        assert_eq!(s.current_pose(), Pose::Sit, "wraps back to Sit");
    }

    #[test]
    fn pinned_mode_never_advances() {
        let mut s = AnimState::new(Mode::Pinned(Pose::Grass));
        for _ in 0..10 {
            s.tick(Duration::from_secs(8));
        }
        assert_eq!(s.current_pose(), Pose::Grass);
    }

    #[test]
    fn breathing_offset_alternates_every_2_5_seconds() {
        let mut s = AnimState::new(Mode::Cycle);
        assert_eq!(s.breathing_offset(), 0, "0s -> 0");
        s.tick(Duration::from_millis(2400));
        assert_eq!(s.breathing_offset(), 0, "2.4s -> 0");
        s.tick(Duration::from_millis(200)); // 2.6s
        assert_eq!(s.breathing_offset(), 1, "2.6s -> 1");
        s.tick(Duration::from_millis(2500)); // 5.1s
        assert_eq!(s.breathing_offset(), 0, "5.1s -> 0");
    }

    #[test]
    fn breathing_offset_independent_of_mode() {
        let mut a = AnimState::new(Mode::Cycle);
        let mut b = AnimState::new(Mode::Pinned(Pose::Sit));
        let dt = Duration::from_millis(2700);
        a.tick(dt);
        b.tick(dt);
        assert_eq!(a.breathing_offset(), b.breathing_offset());
    }
}
