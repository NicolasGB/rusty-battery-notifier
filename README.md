# rusty-battery-notifier
A simple cli battery percent notifier in Rust. Useful for wm such as i3.

## Why Rust?
- "Rust btw."

## Installation

Clone the respository:

```bash
git clone git@github.com:NicolasGB/rusty-battery-notifier.git
```
Then build the binary with cargo:
```bash
cd rusty-battery-notifier
cargo build --release
```
Finally copy the generated binary to somewhere you want. For example in your i3 config directory.
```bash
cp target/release/rusty-battery-notifier ~/.config/i3/battery-notify
```

## Usage

The binary accepts 3 different flags, you can display them with the `--help` flag.

| Flag | Description | Default |
|------|-------------|---------|
| -f   | The first threshold to notify at. (%)| 15 |
| -s   | The second threshold to notify at. (%)| 5 |
| -i   | The interval to check the battery at. (minutes) | 5 |

----------------
#### i3

Add the following for your config with your desired values.
```
exec --no-startup-id ~/.config/i3/battery-notify -f 20 -s 10 -i 3
```

#### Recommendation

Use a Nerd Font to view the icons in the alert.

## License
This project is licensed under the [MIT license](https://choosealicense.com/licenses/mit/)
