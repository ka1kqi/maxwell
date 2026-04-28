# Terminal Cat Companion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `pet`, a Rust binary that renders a calm cycling cat companion in the terminal — three artist-drawn sketched-line cat poses, parrot.live-style frame cycling at 2 fps, with a subtle breathing bob. Loops until Ctrl+C.

**Architecture:** Single Rust binary built on the [`cellophane`](https://github.com/km-clay/cellophane) terminal animation framework. `clap` handles CLI flags. Three modules: `cli` (flags → Mode), `pose` (sprites + Pose enum), `animation` (implements `cellophane::Animation`). Sprites are baked into the binary via `include_str!`. Pose cycle and breathing are driven by elapsed time (no randomness, no allocations per frame beyond the Frame itself).

**Tech Stack:** Rust 2021 edition, `cellophane`, `clap` (with `derive` feature). No async runtime needed.

**Spec:** See `docs/superpowers/specs/2026-04-27-terminal-cat-companion-design.md`.

---

## File Structure

```
pet/
├── Cargo.toml                       # crate manifest, deps
├── .gitignore                       # target/, etc.
├── README.md                        # usage + artist credits
├── src/
│   ├── main.rs                      # entry point, Animator wiring
│   ├── cli.rs                       # clap CLI → Mode
│   ├── pose.rs                      # Pose enum + Sprite + sprite loading
│   ├── animation.rs                 # CatAnimation: impl cellophane::Animation
│   └── sprites/
│       ├── sit.txt                  # sitting cat ([bug])
│       ├── grass.txt                # cat in grass (fL)
│       └── curled.txt               # curled sleeping cat (dp)
└── docs/                            # spec + plan (already in repo)
```

Each module has a single responsibility. `cli` doesn't touch animation logic. `pose` is data-only (no state). `animation` ties pose + time → frames.

---

## Task 1: Cargo project scaffold

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/Cargo.toml`
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/main.rs`
- Create: `/Users/kaikaidu/Documents/GitHub/pet/.gitignore`

- [ ] **Step 1: Initialize cargo crate in-place**

The repo already exists with the `docs/` folder. We need to add a `Cargo.toml` and `src/main.rs` without disturbing existing files.

Run from `/Users/kaikaidu/Documents/GitHub/pet`:

```bash
cargo init --name pet --bin .
```

This creates `Cargo.toml`, `src/main.rs`, and `.gitignore` next to the existing `docs/`.

Expected: prints `Created binary (application) package` and adds those files.

- [ ] **Step 2: Add dependencies**

Run:

```bash
cargo add cellophane
cargo add clap --features derive
```

Expected: each prints "Adding ... to dependencies" and updates `Cargo.toml`. Both crates fetch and resolve.

- [ ] **Step 3: Verify it builds (empty hello world)**

Run:

```bash
cargo build
```

Expected: builds successfully. The default `main.rs` from `cargo init` prints "Hello, world!" — leave it for now, we replace in Task 7.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs .gitignore
git commit -m "Scaffold cargo project with cellophane + clap"
```

---

## Task 2: Embed sprite text files

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/sprites/sit.txt`
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/sprites/grass.txt`
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/sprites/curled.txt`

These are pasted directly from the spec. Preserve trailing whitespace (sketched ASCII relies on column alignment) and artist credits.

- [ ] **Step 1: Create `sit.txt`**

Path: `src/sprites/sit.txt`. Exact content (no leading/trailing blank lines beyond what's shown):

```
       _                        
       \`*-.                    
        )  _`-.                 
       .  : `. .                
       : _   '  \               
       ; *` _.   `*-._          
       `-.-'          `-.       
         ;       `       `.     
         :.       .        \    
         . \  .   :   .-'   .   
         '  `+.;  ;  '      :   
         :  '  |    ;       ;-. 
         ; '   : :`-:     _.`* ;
[bug] .*' /  .*' ; .*`- +'  `*' 
      `*-*   `*-*  `*-*'
```

- [ ] **Step 2: Create `grass.txt`**

Path: `src/sprites/grass.txt`. Exact content:

```
               __..--''``\--....___   _..,_
     ///// _.-'    .-/";  `        ``<._  ``-+'~=. ////
    ///_.-' _..--.'-    \                    `(^) ) //
   // ((..-' // (< -     ;_..__               ; `' //
  ////////////// `-._,_)'//////``--...____..-' /////
 //////////////////////////////////////////////////
///fL/////////////////////////////////////////////
```

- [ ] **Step 3: Create `curled.txt`**

Path: `src/sprites/curled.txt`. Exact content:

```
                                               .--.
                                               `.  \
                                                 \  \
                                                  .  \
                                                  :   .
                                                  |    .
                                                  |    :
                                                  |    |
  ..._  ___                                       |    |
 `."".`''''""--..___                              |    |
 ,-\  \             ""-...__         _____________/    |
 / ` " '                    `""""""""                  .
 \                                                      L
 (>                                                      \
/                                                         \
\_    ___..---.                                            L
  `--'         '.                                           \
                 .                                           \_
                _/`.                                           `.._
             .'     -.                                             `.
            /     __.-Y     /''''''-...___,...--------.._            |
           /   _."    |    /                ' .      \   '---..._    |
          /   /      /    /                _,. '    ,/           |   |
          \_,'     _.'   /              /''     _,-'            _|   |
                  '     /               `-----''               /     |
                  `...-'     dp                                `...-'
```

- [ ] **Step 4: Commit**

```bash
git add src/sprites/
git commit -m "Add three sketched-line cat sprite sources"
```

---

## Task 3: `pose` module — Pose enum, Sprite, loading (TDD)

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/pose.rs`
- Modify: `/Users/kaikaidu/Documents/GitHub/pet/src/main.rs` (add `mod pose;` declaration)

The `pose` module exposes a `Pose` enum, a `Sprite` struct, and a `Pose::sprite(self) -> &'static Sprite` accessor. Width is the max line length in chars; height is the line count.

- [ ] **Step 1: Declare the module from `main.rs`**

Edit `src/main.rs`. Replace the `cargo init` boilerplate with:

```rust
mod pose;

fn main() {
    println!("Hello, world!");
}
```

(`main` body is replaced in Task 7.)

- [ ] **Step 2: Write failing tests**

Create `src/pose.rs`:

```rust
#![allow(dead_code)]

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

impl Pose {
    pub fn sprite(self) -> &'static Sprite {
        unimplemented!()
    }
    pub fn next(self) -> Pose {
        unimplemented!()
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
        assert_eq!(s.width, s.lines.iter().map(|l| l.chars().count()).max().unwrap_or(0));
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
```

- [ ] **Step 3: Run tests; verify they fail**

```bash
cargo test --lib pose
```

Expected: all four tests fail (panic in `unimplemented!()` or compilation error since `unimplemented!()` returns `!`). They should at least fail at runtime, not compile-time. If compilation fails, that's also "failing" for our purposes.

- [ ] **Step 4: Implement `Pose::next` and `Pose::sprite`**

Replace the body of `src/pose.rs` with:

```rust
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
    Sprite { lines, height, width }
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
        assert_eq!(s.width, s.lines.iter().map(|l| l.chars().count()).max().unwrap_or(0));
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
```

- [ ] **Step 5: Run tests; verify they pass**

```bash
cargo test --lib pose
```

Expected: 4 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/pose.rs src/main.rs
git commit -m "Add pose module with Sprite loading and pose cycling"
```

---

## Task 4: `cli` module — flags → Mode (TDD)

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/cli.rs`
- Modify: `/Users/kaikaidu/Documents/GitHub/pet/src/main.rs` (add `mod cli;`)

The CLI parses `pet`, `pet --sit`, `pet --grass`, `pet --curled`. The flags are mutually exclusive. Output is a `Mode`.

- [ ] **Step 1: Add `mod cli;` to main.rs**

Edit `src/main.rs`:

```rust
mod cli;
mod pose;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 2: Write failing tests**

Create `src/cli.rs`:

```rust
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
        unimplemented!()
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
```

- [ ] **Step 3: Run tests; verify they fail**

```bash
cargo test --lib cli
```

Expected: the four "into_mode" tests panic in `unimplemented!()`. The "two flags" test should already pass (clap's `group` attribute enforces mutual exclusion at parse time).

- [ ] **Step 4: Implement `into_mode`**

Replace the `into_mode` body in `src/cli.rs`:

```rust
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
```

- [ ] **Step 5: Run tests; verify they pass**

```bash
cargo test --lib cli
```

Expected: 5 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/cli.rs src/main.rs
git commit -m "Add CLI parsing with mutually-exclusive pose flags"
```

---

## Task 5: `animation` module — pose cycling + breathing logic (TDD)

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/src/animation.rs`
- Modify: `/Users/kaikaidu/Documents/GitHub/pet/src/main.rs` (add `mod animation;`)

This task tests **only the pure-logic** parts of the animation (pose advancement, breathing offset, mode behavior). Rendering into a `Frame` is added in Task 6 because it depends on the cellophane `Frame` API and is hard to unit test cleanly.

We split the logic into a small `AnimState` struct that's pure (no cellophane types). The `cellophane::Animation` trait impl on a wrapper struct is added in Task 6 and delegates to `AnimState`.

- [ ] **Step 1: Add `mod animation;` to main.rs**

Edit `src/main.rs`:

```rust
mod animation;
mod cli;
mod pose;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 2: Write failing tests**

Create `src/animation.rs`:

```rust
#![allow(dead_code)]

use std::time::Duration;
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

    pub fn tick(&mut self, _dt: Duration) {
        unimplemented!()
    }

    pub fn current_pose(&self) -> Pose {
        self.current_pose
    }

    /// 0 or 1: vertical bob offset for breathing.
    pub fn breathing_offset(&self) -> usize {
        unimplemented!()
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
```

- [ ] **Step 3: Run tests; verify they fail**

```bash
cargo test --lib animation
```

Expected: tests panic in `unimplemented!()`.

- [ ] **Step 4: Implement `tick` and `breathing_offset`**

Replace the `tick` and `breathing_offset` bodies in `src/animation.rs`:

```rust
impl AnimState {
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

    pub fn breathing_offset(&self) -> usize {
        let half_periods = (self.elapsed.as_millis() / BREATH_PERIOD.as_millis()) as usize;
        half_periods % 2
    }
}
```

- [ ] **Step 5: Run tests; verify they pass**

```bash
cargo test --lib animation
```

Expected: 6 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/animation.rs src/main.rs
git commit -m "Add animation state: pose cycle and breathing offset"
```

---

## Task 6: `animation` module — cellophane `Animation` impl

**Files:**
- Modify: `/Users/kaikaidu/Documents/GitHub/pet/src/animation.rs`

Wraps `AnimState` in a `CatAnimation` struct that implements `cellophane::Animation`. The `update` method builds a `Frame`, places the current sprite centered with the breathing offset, and applies the lavender color.

> ⚠️ **Cellophane Cell API:** the example in the cellophane README only demonstrates `Cell::default().with_bg(color)`. The methods to set the **glyph** and **foreground color** are most likely `with_ch`/`with_grapheme` and `with_fg`, but **before writing the implementation, verify the actual method names** by running `cargo doc --open -p cellophane` or reading `target/doc/cellophane/struct.Cell.html`. Adjust the call sites in Step 2 below to match.

- [ ] **Step 1: Verify the `Cell` and `Frame` API surface**

Run:

```bash
cargo doc -p cellophane --no-deps
open target/doc/cellophane/struct.Cell.html
open target/doc/cellophane/struct.Frame.html
```

Note down:
- The method to set the glyph (likely `with_ch(char)` or `with_grapheme(...)`).
- The method to set the foreground color (likely `with_fg(Color)`).
- The constructor for `Frame` (the README shows `Frame::with_capacity(cols, rows)`).
- The accessor for cell mutation (the README shows `frame.get_cell_mut(row, col) -> Option<&mut Cell>`).

If any name differs from what's used below, **substitute the correct name everywhere it appears in the implementation**.

- [ ] **Step 2: Add `CatAnimation` to `src/animation.rs`**

Append to `src/animation.rs` (above the `#[cfg(test)] mod tests` block):

```rust
use cellophane::{Animation, Cell, Frame};
use cellophane::crossterm::style::Color;

const LAVENDER: Color = Color::Rgb { r: 177, g: 156, b: 217 };

pub struct CatAnimation {
    state: AnimState,
    rows: usize,
    cols: usize,
}

impl CatAnimation {
    pub fn new(mode: Mode) -> Self {
        Self { state: AnimState::new(mode), rows: 0, cols: 0 }
    }
}

impl Animation for CatAnimation {
    fn init(&mut self, initial: Frame) {
        let (rows, cols) = initial.dims().unwrap_or((0, 0));
        self.rows = rows;
        self.cols = cols;
    }

    fn update(&mut self, dt: Duration) -> Frame {
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
                    *cell = Cell::default().with_ch(ch).with_fg(LAVENDER);
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
```

- [ ] **Step 3: Build and fix any API mismatches**

```bash
cargo build
```

If `with_ch`, `with_fg`, or any other method name doesn't exist, substitute the correct name from the docs you opened in Step 1. Re-run until build is clean.

Expected: clean build (with at most warnings about unused imports — those should be addressed before commit).

- [ ] **Step 4: Re-run all tests to confirm nothing broke**

```bash
cargo test
```

Expected: all tests from Tasks 3, 4, 5 still pass.

- [ ] **Step 5: Commit**

```bash
git add src/animation.rs
git commit -m "Add CatAnimation implementing cellophane::Animation"
```

---

## Task 7: Wire up `main.rs`

**Files:**
- Modify: `/Users/kaikaidu/Documents/GitHub/pet/src/main.rs`

Replace the placeholder `main` with the real entry point: parse args, build `CatAnimation`, drive it via `Animator`.

- [ ] **Step 1: Replace `main.rs` body**

Full content of `src/main.rs`:

```rust
mod animation;
mod cli;
mod pose;

use std::io::ErrorKind;

use cellophane::Animator;
use clap::Parser;

use crate::animation::CatAnimation;
use crate::cli::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let anim = Box::new(CatAnimation::new(args.into_mode()));
    let mut animator = Animator::enter_with(anim)?;
    loop {
        match animator.tick() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) if e.kind() == ErrorKind::Interrupted => break,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
```

- [ ] **Step 2: Build**

```bash
cargo build
```

Expected: clean build. If `Animator::enter_with` has a different name (e.g., `Animator::new`), check the cellophane docs and substitute.

- [ ] **Step 3: Set the frame rate cap (if needed)**

The spec calls for 2 fps. Cellophane's `Animator` has some way to limit frame rate — likely a builder method or a setter. Check the docs:

```bash
open target/doc/cellophane/struct.Animator.html
```

Look for methods like `with_fps`, `set_fps`, or `with_frame_rate`. If found, apply it before the loop:

```rust
let mut animator = Animator::enter_with(anim)?;
animator.set_fps(2); // or whatever the actual method is
```

If no such method exists, accept whatever default cellophane uses (the README implies it's frame-rate-limited internally). Document the resolution in the commit message.

- [ ] **Step 4: Smoke-test the binary**

```bash
cargo run -- --sit
```

Expected: terminal switches to alternate screen, lavender sitting cat appears centered, gently bobbing every ~2.5s. Press Ctrl+C — terminal returns to normal.

Repeat with `--grass` and `--curled`. Then `cargo run` (no flag) and watch for ~30 seconds: the cat should change pose every 8s, cycling sit → grass → curled → sit.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "Wire main entry point: parse args, drive Animator"
```

---

## Task 8: README with usage and artist credits

**Files:**
- Create: `/Users/kaikaidu/Documents/GitHub/pet/README.md`

- [ ] **Step 1: Write README.md**

Full content:

```markdown
# pet

A calm cat companion for your terminal. Three sketched-line cat poses cycle slowly, breathing softly, until you press Ctrl+C.

## Install

```sh
cargo install --path .
```

## Use

```sh
pet              # cycle through all three poses (default)
pet --sit        # pin the sitting cat
pet --grass      # pin the cat-in-grass
pet --curled     # pin the curled sleeping cat
```

Press Ctrl+C to exit.

## Credits

Cat sprites are existing ASCII works by their respective artists; initials preserved in-sprite:

- **Sitting cat** — `[bug]`
- **Cat in grass** — `fL`
- **Curled sleeping cat** — `dp`

Built on the [cellophane](https://github.com/km-clay/cellophane) terminal animation framework.
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "Add README with usage and artist credits"
```

---

## Task 9: Final verification

**Files:** none.

- [ ] **Step 1: Full test suite**

```bash
cargo test
```

Expected: all tests pass (4 in `pose`, 5 in `cli`, 6 in `animation` — total 15).

- [ ] **Step 2: Lint**

```bash
cargo clippy --all-targets -- -D warnings
```

Expected: no warnings. If clippy flags issues, fix them.

- [ ] **Step 3: Format**

```bash
cargo fmt
```

Expected: no diff. If formatting changes were applied, review and commit.

- [ ] **Step 4: Manual visual run**

```bash
cargo run --release
```

Watch for ~30 seconds:
- ✅ Lavender cat appears centered
- ✅ Cat bobs vertically every ~2.5 seconds (subtle)
- ✅ Pose changes every ~8 seconds: sit → grass → curled → sit
- ✅ Ctrl+C cleanly exits to your shell with terminal in normal mode (no leftover artifacts, cursor visible)

Repeat with each `--flag` to confirm pinned modes hold one pose forever (still breathing).

- [ ] **Step 5: Commit any verification fixes (if any)**

If clippy/fmt produced changes:

```bash
git add -A
git commit -m "Apply clippy fixes and rustfmt"
```

If everything was already clean, no commit needed.

---

## Done.

The repo at `/Users/kaikaidu/Documents/GitHub/pet/` should now contain:

- A working `pet` binary (run with `cargo run` or `cargo install --path .`)
- 15 passing unit tests
- A README crediting the artists
- The original spec and this plan in `docs/superpowers/`
