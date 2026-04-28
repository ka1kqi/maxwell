# Terminal Cat Companion — Design

**Date:** 2026-04-27
**Status:** Approved (brainstorm), pending implementation plan
**Binary name:** `pet`

## Goal

A cute terminal command that displays a calm, cycling cat companion. Inspired structurally by `parrot.live` (frame-cycling animation), but slowed down and themed around three artist-drawn sketched-line cats. Runs continuously until the user presses Ctrl+C.

Non-goals: walking/wandering motion, interactive input, multiple creatures on screen, sprite editing/customization, daemonization.

## User-facing behavior

`pet` runs in the foreground, takes over the terminal in alternate-screen mode, and renders one cat at a time. The cat slowly cycles through three poses with a gentle breathing motion within each pose.

Flags:

| Flag         | Behavior                                                   |
| ------------ | ---------------------------------------------------------- |
| (none)       | Cycle through all three poses                              |
| `--sit`      | Pin the sitting cat (no pose cycling, breathing only)      |
| `--grass`    | Pin the cat-in-grass pose                                  |
| `--curled`   | Pin the curled-up sleeping cat                             |
| `-h`/`--help`| Print help and exit                                        |
| `-V`         | Print version and exit                                     |

Exit: Ctrl+C restores the terminal cleanly (cellophane handles raw mode and alternate screen via `Drop`).

## Animation model

Two layered animations:

1. **Pose cycle (slow):** sit → grass → curled → sit → ... Each pose holds for 8 seconds, then cuts to the next. ~24 second full loop.
2. **Breathing bob (fast):** within any held pose, the entire sprite is drawn at vertical offset `Y` or `Y+1`, alternating every 2.5 seconds. Subtle suggestion of breath.

Render rate: **2 fps**. Most ticks return a frame identical to the previous one; cellophane's frame diffing makes this essentially free.

Color: a single soft pastel applied uniformly to the sprite glyphs — lavender `#b19cd9` (`Color::Rgb { r: 177, g: 156, b: 217 }`). One color per session, no cycling. Background untouched (uses terminal default).

If `--<pose>` flag is passed, the pose cycle is disabled — only the breathing bob runs.

## Architecture

A single Rust binary crate with three small modules:

```
pet/
├── Cargo.toml
├── src/
│   ├── main.rs          # entry point, wires Animator + CatAnimation
│   ├── cli.rs           # clap derive struct, --sit/--grass/--curled
│   ├── pose.rs          # Pose enum, sprite data, sprite loading
│   ├── animation.rs     # CatAnimation: impl cellophane::Animation
│   └── sprites/
│       ├── sit.txt      # [bug] sitting cat
│       ├── grass.txt    # fL cat in grass
│       └── curled.txt   # dp curled sleeping cat
└── docs/superpowers/specs/2026-04-27-terminal-cat-companion-design.md
```

### Module: `cli`

`clap` derive struct with mutually-exclusive flags `--sit` / `--grass` / `--curled`. Resolves to:

```rust
pub enum Mode {
    Cycle,            // default — rotate through all three poses
    Pinned(Pose),     // hold one pose forever
}
```

### Module: `pose`

```rust
pub enum Pose { Sit, Grass, Curled }

pub struct Sprite {
    lines: Vec<&'static str>,
    height: usize,
    width: usize,
}

impl Pose {
    pub fn sprite(self) -> &'static Sprite { ... }
    pub fn next(self) -> Pose { ... }
}
```

Sprite text is embedded with `include_str!` so the binary is self-contained. Width is computed at first access (lazy `OnceLock`) as the max line length.

### Module: `animation`

```rust
pub struct CatAnimation {
    mode: Mode,                  // Cycle or Pinned(Pose)
    current_pose: Pose,
    elapsed: Duration,           // accumulated dt
    cols: usize, rows: usize,
}

impl cellophane::Animation for CatAnimation {
    fn init(&mut self, initial: Frame) { /* capture dims */ }
    fn update(&mut self, dt: Duration) -> Frame { /* see below */ }
    fn is_done(&self) -> bool { false }
    fn resize(&mut self, w: usize, h: usize) { ... }
}
```

`update(dt)`:

1. Add `dt` to `elapsed`.
2. If `mode == Cycle` and `elapsed % 8s` rolled over a boundary, advance `current_pose`.
3. Compute breathing offset: `(elapsed.as_secs_f32() / 2.5) as usize % 2` → 0 or 1.
4. Build a fresh `Frame` sized to terminal dims.
5. Compute placement: center the sprite horizontally; vertical position = `(rows - sprite.height) / 2 + breathing_offset`.
6. Write each sprite line as `Cell`s with the chosen pastel `fg` color. Skip cells outside the frame (graceful when terminal is small).
7. Return the frame.

### `main`

```rust
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

Frame rate cap is set on `Animator` (cellophane API) to 2 fps.

## Sprites

All three are pre-existing artist works in matching sketched-line style. Artist initials are preserved in the sprite text and credited in the README.

### `sit.txt` — credited to `[bug]`

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

### `grass.txt` — credited to `fL`

```
               __..--''``\--....___   _..,_
     ///// _.-'    .-/";  `        ``<._  ``-+'~=. ////
    ///_.-' _..--.'-    \                    `(^) ) //
   // ((..-' // (< -     ;_..__               ; `' //
  ////////////// `-._,_)'//////``--...____..-' /////
 //////////////////////////////////////////////////
///fL/////////////////////////////////////////////
```

### `curled.txt` — credited to `dp`

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

## Error handling

- IO errors from `Animator` propagate to `main`, which returns them to the OS.
- `ErrorKind::Interrupted` (Ctrl+C path that bypasses cellophane's normal shutdown) is treated as clean exit.
- Cellophane's `Drop` restores raw mode and alternate screen unconditionally — terminals stay usable even on panic.
- Terminal too small: if `cols < sprite.width` or `rows < sprite.height`, we still render — sprite cells outside frame bounds are silently skipped. (No splash error; the cat just gets clipped.)

## Testing

Unit tests:

- `pose::tests` — `Pose::next()` cycles `Sit → Grass → Curled → Sit`. Sprite parsing produces non-empty `lines`, `height` matches line count, `width` matches max line length.
- `animation::tests` — given a fixed `dt` sequence, `CatAnimation` advances through poses at the expected boundaries; breathing offset alternates 0/1 every 2.5s; pinned mode never advances pose.

Visual verification (manual): run `cargo run` and `cargo run -- --sit` etc., watch the cat. No automated visual tests — cellophane's render correctness is its responsibility.

## Dependencies

- `cellophane` — terminal animation framework (https://github.com/km-clay/cellophane)
- `clap` with the `derive` feature — argument parsing
- (no `rand` needed — pose cycle is deterministic by elapsed time, no randomness in this design)

## Future / out of scope

- Day-night color mode
- Random pose order or random hold durations
- Adding more sprites (would require artwork in the same sketched style)
- Mouse/keyboard interaction
- Configuration file
