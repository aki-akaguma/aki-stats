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

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

mod test_s0 {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
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

mod test_s1 {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t0() {
        let (r, sioe) = do_execute!(&["-a"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-l"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "lines:\"26\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["-b"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bytes:\"1207\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(&["-c"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "chars:\"1207\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(&["-w"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "words:\"226\"\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let (r, sioe) = do_execute!(&["-l", "-m"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "lines:\"26\", max:\"83\"\n");
        assert!(r.is_ok());
    }
}

mod test_s2 {
    use libaki_stats::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t0_en() {
        let (r, sioe) = do_execute!(&["-a", "--locale", "en"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-a", "--locale", "fr"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-a", "--locale", "de"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "lines:\"26\", bytes:\"1.207\", chars:\"1.207\", words:\"226\", max:\"83\"\n"
        );
        assert!(r.is_ok());
    }
}
