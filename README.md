A set of commands which allow using sway easier! 

This project names sway workspaces in the format of num:name. This is due to how Sway works while good practice of having a clear status state for workspaces.

| Command | Description |
| --- | --- |
| `spc switch-to <num>` | Goes to sway workspace of the number, while presenting a wellness screen (with a 60 second cooldown)|
| `spc rename` | Uses `qtai`'s "echo" collection to rename workspaces fast |
| `spc swap-with` | Swap numbers of two Sway workspaces without renaming the whole thing |


This project requires `qtai` with an "echo" collection for renaming.

For switch-to wellness screen, write a config file at `~/.config/spc/splashes.ron` in the following format:

```ron
(
    content: [
        "\"However, one girl genius had made the impossible possible.\" - Matsuri Isora, 2021",
        "\"Throughout every age, there existed one all-powerful phrase a person could use to get out of any situation: I have to go to the bathroom.\" - Matsuri Isora, 2021",
        "\"Meow meow meow\" - Char, 2026",
    ]
)
```

