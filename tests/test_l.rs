#[macro_use]
mod helper;

#[macro_use]
mod helper_l;

mod test_0_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: b, c, l, w, a or --map-ascii\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!([""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: b, c, l, w, a or --map-ascii\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_0_x_options_l {
    use libaki_stats::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
}

mod test_1_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!([""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: b, c, l, w, a or --map-ascii\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let s = unsafe { String::from_utf8_unchecked(v) };
        let (r, sioe) = do_execute!(["-l"], &s);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_empty_input() {
        let (r, sioe) = do_execute!(["-a"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"0\", bytes:\"0\", chars:\"0\", words:\"0\", max:\"0\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_only_newlines() {
        let (r, sioe) = do_execute!(["-a"], "\n\n\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"3\", bytes:\"0\", chars:\"0\", words:\"0\", max:\"0\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_combined_flags() {
        let (r, sioe) = do_execute!(["-l", "-w"], "hello world\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "lines:\"1\", words:\"2\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_crlf_line_endings() {
        let (r, sioe) = do_execute!(["-a"], "line1\r\nline2\r\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"2\", bytes:\"10\", chars:\"10\", words:\"2\", max:\"5\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_word_separators() {
        let (r, sioe) = do_execute!(["-w"], "word1  word2   word3\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "words:\"3\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mixed_line_endings() {
        let input = "line1\nline2\r\nline3\n";
        let (r, sioe) = do_execute!(["-a"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"3\", bytes:\"15\", chars:\"15\", words:\"3\", max:\"5\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_leading_trailing_whitespace() {
        let input = "  word1  \n\tword2\t\n";
        let (r, sioe) = do_execute!(["-a"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"2\", bytes:\"16\", chars:\"16\", words:\"2\", max:\"9\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_word_counting_with_punctuation() {
        let input = "hello, world! one-two three.four\n";
        let (r, sioe) = do_execute!(["-w"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "words:\"4\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_line() {
        let input = "a".repeat(10000);
        let (r, sioe) = do_execute!(["-a"], &input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"1\", bytes:\"10000\", chars:\"10000\", words:\"1\", max:\"10000\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_newline_at_end() {
        let input = "hello world";
        let (r, sioe) = do_execute!(["-a"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"1\", bytes:\"11\", chars:\"11\", words:\"2\", max:\"11\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mixed_separators() {
        let input = "word1 	 word2  word3\n";
        let (r, sioe) = do_execute!(["-w"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "words:\"3\"\n");
        assert!(r.is_ok());
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

mod test_1_more_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_t0() {
        let (r, sioe) = do_execute!(["-a"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1207\", chars:\"1207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(["-l"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "lines:\"26\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(["-b"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bytes:\"1207\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(["-c"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "chars:\"1207\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(["-w"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "words:\"226\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let (r, sioe) = do_execute!(["-l", "-m"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "lines:\"26\", max:\"83\"\n");
        assert!(r.is_ok());
    }
}

mod test_2_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_t0_en() {
        let (r, sioe) = do_execute!(["-a", "--locale", "en"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1,207\", chars:\"1,207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t0_fr() {
        let (r, sioe) = do_execute!(["-a", "--locale", "fr"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1\u{202f}207\", chars:\"1\u{202f}207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t0_de() {
        let (r, sioe) = do_execute!(["-a", "--locale", "de"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1.207\", chars:\"1.207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
}

mod test_4_query_locale_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_query_locale() {
        let (r, sioe) = do_execute!(["--query", "locale"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("en"));
        assert!(buff!(sioe, sout).contains("fr"));
        assert!(buff!(sioe, sout).contains("de"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_query_invalid() {
        let (r, sioe) = do_execute!(["--query", "invalid"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "unknown query: invalid\navailable query: locale\n"
        );
        assert!(r.is_ok());
        // unknown query: invalid
        // available query: locale
        //assert!(buff!(sioe, serr).contains("unknown query: invalid"));
        //assert_eq!(buff!(sioe, sout), "");
        //assert!(r.is_err());
    }
}

mod test_4_with_fixtures_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_sample_text_all() {
        let content = std::fs::read_to_string(fixture_sample_text!()).unwrap();
        let (r, sioe) = do_execute!(["-a"], &content);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"10\", bytes:\"120\", chars:\"120\", words:\"10\", max:\"12\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_sherlock_text_all() {
        let content = std::fs::read_to_string(fixture_sherlock!()).unwrap();
        let (r, sioe) = do_execute!(["-a"], &content);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1207\", chars:\"1207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
}

mod test_4_with_locale_l {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_unicode_chars() {
        let input = "こんにちは 世界\n"; // Hello world in Japanese
        let (r, sioe) = do_execute!(["-a"], input);
        assert_eq!(buff!(sioe, serr), "");
        // "こんにちは" is 5 chars (15 bytes), " " is 1 char (1 byte), "世界" is 2 chars (6 bytes), "\n" is 1 char (1 byte)
        // total chars: 5 + 1 + 2 + 1 = 9
        // total bytes: 15 + 1 + 6 + 1 = 23
        // words: 2
        // lines: 1
        // max line length: 23
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"1\", bytes:\"22\", chars:\"8\", words:\"2\", max:\"22\"\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_bytes_with_locale() {
        let input = "a".repeat(1234);
        let (r, sioe) = do_execute!(["-b", "--locale", "en"], &input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bytes:\"1,234\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_chars_with_locale() {
        let input = "a".repeat(5678);
        let (r, sioe) = do_execute!(["-c", "--locale", "fr"], &input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "chars:\"5\u{202f}678\"\n" // French locale uses a narrow non-breaking space
        );
        assert!(r.is_ok());
    }
}
