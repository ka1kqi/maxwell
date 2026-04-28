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
