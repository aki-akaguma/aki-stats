const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

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
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: b, c, l, w, a or --map-ascii\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
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
}

mod test_0_x_options {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("Options:"));
        assert!(oup.stdout.contains("-X rust-version-info"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
}

mod test_1 {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
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
    //
    #[test]
    fn test_invalid_utf8() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open(fixture_invalid_utf8!()).unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-l"], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_empty_input() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], b"");
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"0\", bytes:\"0\", chars:\"0\", words:\"0\", max:\"0\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_only_newlines() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], b"\n\n\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"3\", bytes:\"0\", chars:\"0\", words:\"0\", max:\"0\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_combined_flags() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-l", "-w"], b"hello world\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "lines:\"1\", words:\"2\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_crlf_line_endings() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], b"line1\r\nline2\r\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"2\", bytes:\"10\", chars:\"10\", words:\"2\", max:\"5\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_word_separators() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-w"], b"word1  word2	word3\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "words:\"3\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mixed_line_endings() {
        let input = "line1\nline2\r\nline3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"3\", bytes:\"15\", chars:\"15\", words:\"3\", max:\"5\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_leading_trailing_whitespace() {
        let input = "  word1  \n\tword2\t\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"2\", bytes:\"16\", chars:\"16\", words:\"2\", max:\"9\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_word_counting_with_punctuation() {
        let input = "hello, world! one-two three.four\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-w"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "words:\"4\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_line() {
        let input = "a".repeat(10000);
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"1\", bytes:\"10000\", chars:\"10000\", words:\"1\", max:\"10000\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_newline_at_end() {
        let input = "hello world";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"1\", bytes:\"11\", chars:\"11\", words:\"2\", max:\"11\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mixed_separators() {
        let input = "word1 	 word2  word3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-w"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "words:\"3\"\n");
        assert!(oup.status.success());
    }
}

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

mod test_1_more {
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

mod test_4_query_locale {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_query_locale() {
        let oup = exec_target(TARGET_EXE_PATH, ["--query", "locale"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("en"));
        assert!(oup.stdout.contains("fr"));
        assert!(oup.stdout.contains("de"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_query_invalid() {
        let oup = exec_target(TARGET_EXE_PATH, ["--query", "invalid"]);
        assert_eq!(oup.stderr, "");
        // unknown query: invalid
        // available query: locale
        //assert!(oup.stderr.contains("unknown query: invalid"));
        //assert_eq!(oup.stdout, "");
        //assert!(!oup.status.success());
        assert_eq!(
            oup.stdout,
            "unknown query: invalid\navailable query: locale\n"
        );
        assert!(oup.status.success());
    }
}

mod test_4_with_fixtures {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_sample_text_all() {
        let content = std::fs::read(fixture_sample_text!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], &content);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"10\", bytes:\"120\", chars:\"120\", words:\"10\", max:\"12\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_sherlock_text_all() {
        let content = std::fs::read(fixture_sherlock!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], &content);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "lines:\"26\", bytes:\"1207\", chars:\"1207\", words:\"226\", max:\"83\"\n"
        );
        assert!(oup.status.success());
    }
}

mod test_4_with_locale {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_unicode_chars() {
        let input = "こんにちは 世界\n"; // Hello world in Japanese
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-a"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        // "こんにちは" is 5 chars (15 bytes), " " is 1 char (1 byte), "世界" is 2 chars (6 bytes), "\n" is 1 char (1 byte)
        // total chars: 5 + 1 + 2 + 1 = 9
        // total bytes: 15 + 1 + 6 + 1 = 23
        // words: 2
        // lines: 1
        // max line length: 23
        assert_eq!(
            oup.stdout,
            "lines:\"1\", bytes:\"22\", chars:\"8\", words:\"2\", max:\"22\"\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_bytes_with_locale() {
        let input = "a".repeat(1234);
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-b", "--locale", "en"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bytes:\"1,234\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_chars_with_locale() {
        let input = "a".repeat(5678);
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-c", "--locale", "fr"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "chars:\"5\u{202f}678\"\n" // French locale uses a narrow non-breaking space
        );
        assert!(oup.status.success());
    }
}
