A set of (hacky) commands which allow using Sway easier! 

This project names Sway workspaces in the format of num:name which each numbers is unique to the entire WM (in other words, no duplicates between monitors). This is due to how Sway works while good practice of having a clear status state for workspaces.

| Command | Description |
| --- | --- |
| `spc switch-to <num>` | Goes to Sway workspace of the number, while presenting a wellness screen (with a 60 second cooldown)|
| `spc rename` | Uses `qtai`'s "echo" collection to rename workspaces fast |
| `spc swap-with <num>` | Swap numbers of the current Sway workspace and the Sway workspace of the number without renaming the whole thing |


![Dmenu with renaming options](./assets/qtai.png)
This project requires [qtai](https://github.com/pbun206/qtai) with an "echo" collection for renaming. qtai is super powerful, I recommend it a lot (no bias)! 

Here is a preview of the wellness screen.

![A black screen with sigh instructions and quote](./assets/preview.png)
For switch-to wellness screen, write a config file at `~/.config/spc/splashes.ron` in the following format:

```ron
(
    content: [
        "\"However, one girl genius had made the impossible possible.\" - Matsuri Isora, 2021",
        "\"All models are wrong, but some are useful.\" - George Box, 1979",
        "\"Meow meow meow\" - Char, 2026",
    ]
)
```


## Installing

```bash
# git clone
git clone https://github.com/pbun206/sway_productivity_cli.git

# get into the directory
cd sway_productivity_cli

# make sure you have cargo installed
cargo install --path . 

# building might take a while :p
```
