# aki-stats

*aki-stats* is output the statistics of text, like a wc of linux command.

## Features

*aki-stats*  is output the statistics of text, like a wc of linux command.

* command help

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

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-stats
```

2. you can build debian package:

```text
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
