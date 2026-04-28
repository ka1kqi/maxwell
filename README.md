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

5. Press `Ctrl` + `C` to make him leave.

You can now type `maxwell` in any Terminal window to see him again.

## Install from source (any platform with Rust)

```sh
cargo install --git https://github.com/ka1kqi/maxwell
```

## Notes

- The cat is rendered in lavender on your terminal's normal background.
- Needs roughly an 80×30 terminal — Maxwell is a big cat. Make the window bigger if he looks cut off.
- Works on both Apple Silicon and Intel Macs (universal binary).

## Credits

The 57 Maxwell frames are from [hugomd/ascii-live](https://github.com/hugomd/ascii-live) (MIT), which extends [parrot.live](https://github.com/hugomd/parrot.live). Maxwell is the cat from the well-known spinning-cat meme.

Built with the [cellophane](https://github.com/km-clay/cellophane) terminal animation framework.
