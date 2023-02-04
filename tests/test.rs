const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

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

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-stats"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

//mod helper;

mod test_0 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, [""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: b, c, l, w, a or --map-ascii\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
} // mod test_0

const IN_DAT_1: &str = "\
You could not possibly have come at a better time, my dear Watson,
he said cordially.
I was afraid that you were engaged.
So I am. Very much so.
Then I can wait in the next room.
Not at all.
This gentleman, Mr. Wilson,
has been my partner and helper in many of my most successful cases,
and I have no doubt that he will be of the utmost use to me in yours also.
Try the settee, said Holmes,
relapsing into his armchair and putting his finger-tips together,
as was his custom when in judicial moods.
I know, my dear Watson,
that you share my love of all that is bizarre and outside the conventions
and humdrum routine of everyday life.
You have shown your relish for it by the enthusiasm which has prompted
you to chronicle, and, if you will excuse my saying so,
somewhat to embellish so many of my own little adventures.

\"Your cases have indeed been of the greatest interest to me,\" I observed.

You will remember that I remarked the other day,
just before we went into the very simple problem presented by Miss Mary Sutherland,
that for strange effects and extraordinary combinations we must go to life itself,
which is always far more daring than any effort of the imagination.
A proposition which I took the liberty of doubting.
";

mod test_1 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t0() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"26\", bytes:\"1207\", chars:\"1207\", words:\"226\", max:\"83\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-l"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "lines:\"26\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-b"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bytes:\"1207\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-c"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "chars:\"1207\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-w"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "words:\"226\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-l", "-m"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "lines:\"26\", max:\"83\"\n");
        assert!(oup.status.success());
    }
}

mod test_2 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t0_en() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-a", "--locale", "en"],
            super::IN_DAT_1.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"26\", bytes:\"1,207\", chars:\"1,207\", words:\"226\", max:\"83\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t0_fr() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-a", "--locale", "fr"],
            super::IN_DAT_1.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"26\", bytes:\"1\u{202f}207\", chars:\"1\u{202f}207\", words:\"226\", max:\"83\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t0_de() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-a", "--locale", "de"],
            super::IN_DAT_1.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"26\", bytes:\"1.207\", chars:\"1.207\", words:\"226\", max:\"83\"\n"
        );
        assert!(oup.status.success());
    }
}
