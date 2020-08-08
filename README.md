# nice_prompt

> Minimal prompt for my shell.

[![asciicast](https://asciinema.org/a/349727.svg)](https://asciinema.org/a/349727)

## Usage

```
USAGE:
    nice_prompt [FLAGS]

    FLAGS:
        -h, --help       Prints help information
        -l, --logging    Stores logging outupt in a cache directory.
        -V, --version    Prints version information
```

To log use the `-l` flag. Logs are present at following paths:

| Platform | Example                                               |
| -------  | ----------------------------------------------------- |
| Linux    | /home/alice/.config/nice_prompt/                      |
| macOS    | /Users/Alice/Library/Application Support/nice_prompt/ |
| Windows  | C:\Users\Alice\AppData\Roaming\nice_prompt            |

## Install

- Install [rust](https://rustup.rs/)
- Install `nice_prompt`

    - using `cargo`

        ```shell
        cargo install nice_prompt
        ```

    - using git

        ```shell
        cargo install --force --git https://github.com/samyakahuja/nice_prompt.git
        ```

- Put this in your `.bashrc`

    ```shell
    PS1='$(nice_prompt)'
    ```
