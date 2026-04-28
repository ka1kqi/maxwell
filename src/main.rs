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
    let mut animator = Animator::new(anim).target_fps(2);
    animator.enter()?;
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
