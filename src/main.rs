mod animation;
mod cli;
mod pose;

use std::io::{ErrorKind, stdout};

use cellophane::Animator;
use cellophane::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use clap::Parser;

use crate::animation::CatAnimation;
use crate::cli::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let anim = Box::new(CatAnimation::new(
        args.color.to_color(),
        args.bg.to_color(),
        true, // flinch from mouse cursor
    ));
    let mut animator = Animator::new(anim).target_fps(10);
    animator.enter()?;
    // Best-effort: enable mouse tracking so on_event receives Mouse(Moved).
    // Terminals that don't support mouse capture will simply produce no events.
    let _ = execute!(stdout(), EnableMouseCapture);

    let result = run(&mut animator);

    let _ = execute!(stdout(), DisableMouseCapture);
    result
}

fn run(animator: &mut Animator) -> std::io::Result<()> {
    loop {
        match animator.tick() {
            Ok(true) => continue,
            Ok(false) => break Ok(()),
            Err(e) if e.kind() == ErrorKind::Interrupted => break Ok(()),
            Err(e) => break Err(e),
        }
    }
}
