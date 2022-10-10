# flipnote-id

A tool to modify Flipnote Studio ID (FSID) implemented in Rust

```
Usage: flipnote-id [OPTIONS] <ACTION>

Arguments:
  <ACTION>  [possible values: set, extract, check]

Options:
  -f, --file <FILE>  Flipnote Studio option file [default: option.bin]
  -i, --id <FSID>    Flipnote Studio ID (use with `set` action)
  -d, --no-backup    Don't backup the original file when setting FSID (used by `set` action)
  -h, --help         Print help information (use `--help` for more detail)
  -V, --version      Print version information
```

# Example

```
$ ./flipnote-id set --id 5000000000000000 --file option.bin
```

# Caution

I'm not responsible for any kind of data loss or service terminations (including bans from custom Flipnote servers)

# Credits

- Thanks [Flipnote Collective](https://github.com/Flipnote-Collective) for providing [FSID format](https://github.com/Flipnote-Collective/flipnote-studio-docs/wiki/FSIDs-and-Filenames#flipnote-studio-ids) info
- Thanks nocash for providing [option.bin file strucure](https://problemkaputt.de/gbatek-dsi-sd-mmc-flipnote-files.htm)
- [genact](https://github.com/svenstaro/genact) project as an example for Rust GitHub Actions release file and Cargo.toml options

# License

[MIT License](LICENSE)