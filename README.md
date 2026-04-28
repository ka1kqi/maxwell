# maxwell

Maxwell the cat, animated in your terminal. Cycles through 57 frames at 10 fps until you press Ctrl+C.

## Install (macOS, no setup required)

1. Open the **Terminal** app. (Press `Cmd` + `Space`, type "Terminal", press `Enter`.)

2. Copy this line, paste it into Terminal, and press `Enter`:

   ```sh
   curl -fsSL https://raw.githubusercontent.com/ka1kqi/maxwell/main/install.sh | sh
   ```

3. When asked, type your **Mac login password** and press `Enter`. (Nothing will appear as you type — that's normal.)

4. Type `maxwell` and press `Enter`. Maxwell appears.

5. Press `q` (or `Ctrl` + `C`) to make him leave.

You can now type `maxwell` in any Terminal window to see him again.

## Install from source (any platform with Rust)

```sh
cargo install --git https://github.com/ka1kqi/maxwell
```

## Colors

Maxwell is lavender by default. Both the cat color and the screen background tint are configurable.

```sh
maxwell                                 # lavender cat, your terminal's normal background
maxwell --color pink                    # pink cat
maxwell --color none                    # cat in your terminal's default text color (always readable)

maxwell --bg cream                      # cream background tint while maxwell runs
maxwell --bg sky --color cream          # cream cat on sky-blue background

maxwell --help                          # full list of options and presets
```

Color presets (work for both `--color` and `--bg`): `lavender`, `pink`, `mint`, `peach`, `sky`, `cream`, `white`, `yellow`, `none`.

## Notes

- Move your mouse cursor near Maxwell — he flinches away from it.
- Needs roughly an 80×30 terminal — Maxwell is a big cat. Make the window bigger if he looks cut off.
- Works on both Apple Silicon and Intel Macs (universal binary).

## Credits

The 57 Maxwell frames are from [hugomd/ascii-live](https://github.com/hugomd/ascii-live) (MIT), which extends [parrot.live](https://github.com/hugomd/parrot.live). Maxwell is the cat from the well-known spinning-cat meme.

Built with the [cellophane](https://github.com/km-clay/cellophane) terminal animation framework.
