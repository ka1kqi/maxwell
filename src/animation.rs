use crate::pose::frames;
use cellophane::crossterm::style::Color;
use cellophane::{Animation, Cell, Frame};
use std::time::{Duration, Instant};

const FRAME_INTERVAL: Duration = Duration::from_millis(100); // 10 fps animation rate

pub struct CatAnimation {
    rows: usize,
    cols: usize,
    started: Option<Instant>,
    color: Option<Color>,
}

impl CatAnimation {
    pub fn new(color: Option<Color>) -> Self {
        Self {
            rows: 0,
            cols: 0,
            started: None,
            color,
        }
    }

    fn current_frame_index(&self, total: usize) -> usize {
        match self.started {
            None => 0,
            Some(t) => {
                let elapsed = Instant::now().saturating_duration_since(t);
                (elapsed.as_millis() / FRAME_INTERVAL.as_millis()) as usize % total
            }
        }
    }
}

impl Default for CatAnimation {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Animation for CatAnimation {
    fn init_with(&mut self, initial: Frame) {
        let (rows, cols) = initial.dims().unwrap_or((0, 0));
        self.rows = rows;
        self.cols = cols;
        self.started = Some(Instant::now());
    }

    fn update(&mut self) -> Frame {
        let mut frame = Frame::with_capacity(self.cols, self.rows);
        let frames = frames();
        let idx = self.current_frame_index(frames.len());
        let sprite = &frames[idx];

        let start_col = self.cols.saturating_sub(sprite.width) / 2;
        let start_row = self.rows.saturating_sub(sprite.height) / 2;

        for (line_idx, line) in sprite.lines.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                // Skip blanks and pixel-art background dots so the cat appears
                // on the terminal's normal background instead of a dot rectangle.
                if ch == ' ' || ch == '.' {
                    continue;
                }
                let row = start_row + line_idx;
                let col = start_col + col_idx;
                if let Some(cell) = frame.get_cell_mut(row, col) {
                    let mut new_cell = Cell::default().with_char(ch);
                    if let Some(c) = self.color {
                        new_cell = new_cell.with_fg(c);
                    }
                    *cell = new_cell;
                }
            }
        }

        frame
    }

    fn is_done(&self) -> bool {
        false
    }

    fn resize(&mut self, w: usize, h: usize) {
        self.cols = w;
        self.rows = h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_index_starts_at_zero() {
        let a = CatAnimation::new(None);
        assert_eq!(a.current_frame_index(57), 0);
    }

    #[test]
    fn frame_index_wraps_modulo_total() {
        let mut a = CatAnimation::new(None);
        // Pretend the animation started 6 seconds ago: 60 frame intervals at 10 fps,
        // 60 % 57 = 3.
        a.started = Some(Instant::now() - Duration::from_secs(6));
        assert_eq!(a.current_frame_index(57), 3);
    }

    #[test]
    fn frame_index_advances_with_time() {
        let mut a = CatAnimation::new(None);
        a.started = Some(Instant::now() - Duration::from_millis(350));
        assert_eq!(a.current_frame_index(57), 3);
    }
}
