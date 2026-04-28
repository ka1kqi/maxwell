use crate::pose::frames;
use cellophane::crossterm::event::{Event, MouseEvent, MouseEventKind};
use cellophane::crossterm::style::Color;
use cellophane::{Animation, Cell, Frame};
use std::time::{Duration, Instant};

const FRAME_INTERVAL: Duration = Duration::from_millis(100); // 10 fps animation rate
const FLINCH_RADIUS: i32 = 25; // cells: how close the cursor has to get
const FLINCH_MAX: i32 = 4; // cells: maximum displacement of the sprite

pub struct CatAnimation {
    rows: usize,
    cols: usize,
    started: Option<Instant>,
    color: Option<Color>,
    bg: Option<Color>,
    mouse_pos: Option<(usize, usize)>,
    flinch_enabled: bool,
}

impl CatAnimation {
    pub fn new(color: Option<Color>, bg: Option<Color>, flinch_enabled: bool) -> Self {
        Self {
            rows: 0,
            cols: 0,
            started: None,
            color,
            bg,
            mouse_pos: None,
            flinch_enabled,
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

    /// Compute how far to push the sprite away from the cursor.
    /// Returns (row_offset, col_offset). Both 0 if the mouse is far away,
    /// disabled, or never moved.
    fn flinch_offset(&self, sprite_w: usize, sprite_h: usize) -> (i32, i32) {
        if !self.flinch_enabled {
            return (0, 0);
        }
        let Some((mr, mc)) = self.mouse_pos else {
            return (0, 0);
        };
        // Cat's centered position before flinch.
        let base_col = self.cols.saturating_sub(sprite_w) / 2;
        let base_row = self.rows.saturating_sub(sprite_h) / 2;
        let cr = (base_row + sprite_h / 2) as i32;
        let cc = (base_col + sprite_w / 2) as i32;

        let dr = cr - mr as i32;
        let dc = cc - mc as i32;
        let dist_sq = dr * dr + dc * dc;
        if dist_sq >= FLINCH_RADIUS * FLINCH_RADIUS {
            return (0, 0);
        }

        let dist = (dist_sq as f32).sqrt().max(1.0);
        // Strength tapers from FLINCH_MAX at zero distance to 0 at FLINCH_RADIUS.
        let strength = (FLINCH_RADIUS as f32 - dist) / FLINCH_RADIUS as f32 * FLINCH_MAX as f32;
        // Unit vector from mouse to cat, scaled by strength.
        let off_r = (dr as f32 / dist * strength).round() as i32;
        let off_c = (dc as f32 / dist * strength).round() as i32;
        (off_r, off_c)
    }
}

impl Default for CatAnimation {
    fn default() -> Self {
        Self::new(None, None, false)
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

        // Background tint fills first; cat cells overwrite where they apply.
        if let Some(bg) = self.bg {
            for r in 0..self.rows {
                for c in 0..self.cols {
                    if let Some(cell) = frame.get_cell_mut(r, c) {
                        *cell = Cell::default().with_bg(bg);
                    }
                }
            }
        }

        let all_frames = frames();
        let idx = self.current_frame_index(all_frames.len());
        let sprite = &all_frames[idx];

        let base_col = self.cols.saturating_sub(sprite.width) / 2;
        let base_row = self.rows.saturating_sub(sprite.height) / 2;
        let (off_r, off_c) = self.flinch_offset(sprite.width, sprite.height);
        // Apply flinch with bounds clamping so the sprite stays in the visible region.
        let max_row = self.rows.saturating_sub(sprite.height) as i32;
        let max_col = self.cols.saturating_sub(sprite.width) as i32;
        let start_row = (base_row as i32 + off_r).clamp(0, max_row.max(0)) as usize;
        let start_col = (base_col as i32 + off_c).clamp(0, max_col.max(0)) as usize;

        for (line_idx, line) in sprite.lines.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
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
                    if let Some(bg) = self.bg {
                        new_cell = new_cell.with_bg(bg);
                    }
                    *cell = new_cell;
                }
            }
        }

        frame
    }

    fn on_event(&mut self, event: Event) {
        if let Event::Mouse(MouseEvent {
            column,
            row,
            kind,
            ..
        }) = event
        {
            // Track on Move and Drag; ignore button events / scrolls.
            if matches!(kind, MouseEventKind::Moved | MouseEventKind::Drag(_)) {
                self.mouse_pos = Some((row as usize, column as usize));
            }
        }
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
        let a = CatAnimation::new(None, None, false);
        assert_eq!(a.current_frame_index(57), 0);
    }

    #[test]
    fn frame_index_wraps_modulo_total() {
        let mut a = CatAnimation::new(None, None, false);
        a.started = Some(Instant::now() - Duration::from_secs(6));
        assert_eq!(a.current_frame_index(57), 3);
    }

    #[test]
    fn frame_index_advances_with_time() {
        let mut a = CatAnimation::new(None, None, false);
        a.started = Some(Instant::now() - Duration::from_millis(350));
        assert_eq!(a.current_frame_index(57), 3);
    }

    #[test]
    fn flinch_zero_when_disabled() {
        let mut a = CatAnimation::new(None, None, false);
        a.rows = 50;
        a.cols = 200;
        a.mouse_pos = Some((25, 100)); // right on top
        assert_eq!(a.flinch_offset(100, 30), (0, 0));
    }

    #[test]
    fn flinch_zero_when_no_mouse() {
        let mut a = CatAnimation::new(None, None, true);
        a.rows = 50;
        a.cols = 200;
        assert_eq!(a.flinch_offset(100, 30), (0, 0));
    }

    #[test]
    fn flinch_pushes_cat_away_from_nearby_mouse() {
        let mut a = CatAnimation::new(None, None, true);
        a.rows = 50;
        a.cols = 200;
        // Cat center is at (25, 100). Mouse just to the left.
        a.mouse_pos = Some((25, 95));
        let (dr, dc) = a.flinch_offset(100, 30);
        assert_eq!(dr, 0, "no vertical push when mouse is on same row");
        assert!(dc > 0, "cat moves to the right (positive col delta)");
    }

    #[test]
    fn flinch_zero_when_mouse_far() {
        let mut a = CatAnimation::new(None, None, true);
        a.rows = 50;
        a.cols = 200;
        a.mouse_pos = Some((0, 0)); // far corner
        // Cat center at (25, 100), distance >> FLINCH_RADIUS.
        assert_eq!(a.flinch_offset(100, 30), (0, 0));
    }
}
