# Better LS

We use icons from [NerdFonts](https://www.nerdfonts.com/cheat-sheet)

## How-to

### Add icons

Under config there is two files, `folders.yml` and `files.yml`. Both are used to map names to icons with the same structure:

- `icons` contains a map from name to icon, so if there is a new icon, this is probably the place to add.
- `aliases` contains a map to icons, so if there is an icon already that you want to map to a name, this is the place.

### Update colors

Under config you will find `colors.yml` containing `dark` and `light` both will have the same mapping for its variant.

We use [colored](https://docs.rs/colored/2.0.0/colored/) for the colors with the `true-color` standard, so the mapping will be to RGB.

If you want to check if your terminal is compatible, type `echo $COLORTERM`. If the return is `truecolor` you are all set.

## TODOs

- [ ] Map `ls` flags.
- [ ] Check the implications of reading from file (runtime) instead of `include_str!` (compile-time).
- [ ] Figure it out a way to have custom colors and add colors that looks good in all places.
- [ ] Learn how tests on rust works
- [ ] Refactor (because for sure I don't know what I'm doing and there is probably best practices to follow 😅)
- [ ] Test on linux (and windows?)
- [ ] Break line based on terminal size

### Inspired by

- [colorls](https://github.com/athityakumar/colorls), as I've been using it for quite a while.
- [ls](https://man7.org/linux/man-pages/man1/ls.1.html) as expected to support the same commands as the unix LS.
- my desire to learn [Rust](https://doc.rust-lang.org/book/).
