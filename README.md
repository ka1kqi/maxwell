# pet

Maxwell the cat, animated in your terminal. Cycles through 57 frames at 10 fps until you press Ctrl+C.

## Install

**macOS (no Rust required):**

```sh
curl -fsSL https://raw.githubusercontent.com/ka1kqi/maxwell/main/install.sh | sh
```

You'll be asked for your Mac password (the installer needs it to put `maxwell` in `/usr/local/bin`). After that, type `maxwell` in any Terminal window.

**From source (any platform with Rust):**

```sh
cargo install --git https://github.com/ka1kqi/maxwell
```

## Use

```sh
pet
```

Press Ctrl+C to exit.

The cat is rendered in lavender on your terminal's normal background. Needs roughly an 80×30 terminal — Maxwell is a big cat.

## Credits

The 57 Maxwell frames are taken from [hugomd/ascii-live](https://github.com/hugomd/ascii-live) (MIT-licensed), which itself extends [parrot.live](https://github.com/hugomd/parrot.live). Maxwell is the cat from the well-known spinning-cat meme.

Built on the [cellophane](https://github.com/km-clay/cellophane) terminal animation framework.
