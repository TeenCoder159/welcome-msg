# Welcome-msg
A simple program that prints a welcome message on the terminal.

## Installation
To install the program, you can use the following command:

```bash
cargo install welcome-msg
```

## Usage
Setup a config file in your home directory.

```bash
touch ~/.config/welcome-msg/config.json
```

Edit the config file with your preferred text editor.

Example config file:
```json
{
  "font": "~/.config/welcome-msg/ansi-shadow.flf",
  "lines": [
    "Welcome",
    "Iron Man"
  ],
  "color": [135, 175, 215]
}
```

### Fonts (optional)
You can use a custom font for the program.
but they have to be .flf files.

I found a list [here](https://github.com/xero/figlet-fonts/blob/master/Examples.md)

### Lines (minimum 1 is required)
You can add as many lines as you want.

### Color (optional)
You can change the color of the text.
It uses the RGB color format, and will give an error if you do not use the right format
> See example config above for the correct format

To use the program, you can run the following command:

```bash
welcome-msg
```
Or just add it to your `~/.zshrc` or `~/.bashrc` file.

This will print a welcome message on the terminal.
