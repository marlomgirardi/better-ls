# Better LS

⚠️ As of now, this is no replacement for anything, this is only a personal project I decided to create to learn rust.

We use icons from [NerdFonts](https://www.nerdfonts.com/cheat-sheet)

This is compatible only with terminals that support `truecolor`.

Type `echo $COLORTERM` in your terminal, if the result is `truecolor` you are good to go!

## How-to

### Testing within the project repo.

There is a feature called `find_project_root` that will run in place to find the closest `Cargo.toml` while in dev.

### Add icons

Under config there is two files, `folders.yml` and `files.yml`. Both are used to map names to icons with the same structure:

- `icons` contains a map from name to icon, so if there is a new icon, this is probably the place to add.
- `aliases` contains a map to icons, so if there is an icon already that you want to map to a name, this is the place.

### Update colors

Under config you will find `colors.yml` containing `dark` and `light` both will have the same mapping for its variant.

We use [colored](https://docs.rs/colored/2.0.0/colored/) for the colors with the `true-color` standard, so the mapping will be to RGB.

## TODOs

There are two boards:

- [Flags](https://github.com/users/marlomgirardi/projects/3/views/1) - Flags to be implemented (probably will stick with the ones I use the most for now.)
- [Improvements](https://github.com/users/marlomgirardi/projects/4/views/1) - Things that can be done better.

### Inspired by

- [colorls](https://github.com/athityakumar/colorls), as I've been using it for quite a while.
- [ls](https://man7.org/linux/man-pages/man1/ls.1.html) as expected to support the same commands as the unix LS.
- my desire to learn [Rust](https://doc.rust-lang.org/book/).
