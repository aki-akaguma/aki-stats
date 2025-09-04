#[allow(unused_macros)]
macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-stats [options]

            output the statistics of text, like a wc of linux command.

            Options:
              -a, --all                 output the all statistics of text, exclude ascii map
              -b, --bytes               output the byte counts
              -c, --chars               output the unicode character counts
              -l, --lines               output the line counts
                  --map-ascii           output the ascii map statistics
              -m, --max-line-bytes      output the maximum byte counts of line
              -w, --words               output the word counts
                  --locale <loc>        locale of number format: en, fr, ... posix
              -?, --query <q>           display available names of locale and exit

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Examples:
              Outputs the line count:
                echo -e "acbde fghi\njkln opqr" | aki-stats -l
              Outputs the byte count:
                echo -e "acbde fghi\njkln opqr" | aki-stats -b
              Outputs the word count:
                echo -e "acbde fghi\njkln opqr" | aki-stats -w
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-stats"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_sample_text {
    () => {
        "fixtures/sample-text.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_sherlock {
    () => {
        "fixtures/sherlock.txt"
    };
}
