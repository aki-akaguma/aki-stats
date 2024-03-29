# aki-stats

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

output the statistics of text, like a wc of linux command.

## Features

- output the statistics of text, like a wc of linux command.
- minimum support rustc 1.58.1 (db9d1b20b 2022-01-20)

## Command help

```
aki-stats --help
```

```
Usage:
  aki-stats [options]

output the statistics of text, like a wc of linux command.

Options:
  -a, --all                 output the all statistics of text
  -b, --bytes               output the byte counts
  -c, --chars               output the unicode character counts
  -l, --lines               output the line counts
  -m, --max-line-bytes      output the maximum byte counts of line
  -w, --words               output the word counts
      --locale <loc>        locale of number format: en, fr, ... posix
  -?, --query <q>           display available names of locale and exit

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Examples:
  Outputs the line count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -l
  Outputs the byte count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -b
  Outputs the word count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -w
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-stats
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Example 1: the all statistics

Output the all statistics of input text.

command line:
```
cat fixtures/sherlock.txt | aki-stats -a
```

result output:
```
lines:"26", bytes:"1207", chars:"1207", words:"226", max:"83"
```

### Example 2: the line count

Output the only line count.

command line:
```
cat fixtures/sherlock.txt | aki-stats -l
```

result output:
```
lines:"26"
```

### Example 3: the byte count with locale en

Output the only byte count.

command line:
```
cat fixtures/sherlock.txt | aki-stats -b --locale en
```

result output:
```
bytes:"1,207"
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-stats/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-stats.svg
[crate-link]: https://crates.io/crates/aki-stats
[docs-image]: https://docs.rs/aki-stats/badge.svg
[docs-link]: https://docs.rs/aki-stats/
[rustc-image]: https://img.shields.io/badge/rustc-1.58+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/aki-stats/actions/workflows/test-windows.yml
