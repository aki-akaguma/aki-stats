/*!
output the statistics of text, like a wc of linux command.

# Features

- output the statistics of text, like a wc of linux command.
- minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)

# Command help

```text
aki-stats --help
```

```text
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

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-stats
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Example 1: the all statistics

Output the all statistics of input text.

command line:
```text
cat fixtures/sherlock.txt | aki-stats -a
```

result output:
```text
lines:"26", bytes:"1207", chars:"1207", words:"226", max:"83"
```

## Example 2: the line count

Output the only line count.

command line:
```text
cat fixtures/sherlock.txt | aki-stats -l
```

result output:
```text
lines:"26"
```

## Example 3: the byte count with locale en

Output the only byte count.

command line:
```text
cat fixtures/sherlock.txt | aki-stats -b --locale en
```

result output:
```text
bytes:"1,207"
```

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
*/
#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute stats
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "stats"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: the all statistics
///
/// Output the all statistics of input text.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_stats::execute(&RunnelIoeBuilder::new().build(),
///     "stats", &["-a"]);
/// ```
///
/// ## Example 2: the line count
///
/// Output the only line count.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_stats::execute(&RunnelIoeBuilder::new().build(),
///     "stats", &["-l"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
