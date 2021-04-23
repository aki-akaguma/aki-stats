//
use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

use crate::util::OptLocaleLoc;
use num_format::Locale;
use crate::util::OptUcXParam;
use std::str::FromStr;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
output the statistics of text, like a wc of linux command.
"#;
/*
const ARGUMENTS_TEXT: &str = r#"Argument:
  <url>                     url to getting, protocol is http or ftp
"#;
*/
const EXAMPLES_TEXT: &str = r#"Examples:
  Outputs the line count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -l
  Outputs the byte count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -b
  Outputs the word count:
    echo -e "acbde fghi\njkln opqr" | aki-stats -w
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, EXAMPLES_TEXT].join("\n")
}

#[rustfmt::skip]
fn opt_uc_x_help_message(_program: &str) -> String {
    let z_opts = concat!(
        "Options:\n",
        "  -X rust-version-info     display package version info and exit\n",
        "  -X base_dir=<path>       set <path> is base directory\n",
    );
    z_opts.to_string()
}

#[rustfmt::skip]
fn opt_uc_x_package_version_info(_program: &str) -> String {
    use std::io::Read;
    let mut string = String::new();
    let fnm = format!("/usr/share/doc/{}/rust-version-info.txt", env!("CARGO_PKG_NAME"));
    let file = std::fs::File::open(&fnm);
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut string).unwrap();
            string
        },
        Err(err) => {
            format!("ERROR: {}: '{}'", err, fnm)
        },
    }
}

#[rustfmt::skip]
fn query_locale(_program: &str) -> String {
    format!( "locales: C {}", Locale::available_names().join(" "))
}

#[rustfmt::skip]
fn query_error(_program: &str, s: &str) -> String {
    format!( "unknown query: {}\navailable query: locale", s)
}

//----------------------------------------------------------------------
#[allow(clippy::unnecessary_wraps)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf {
        prog_name: a_prog_name.to_string(),
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(&conf.prog_name)));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    if !conf.opt_uc_x.is_empty() {
        if conf.is_opt_uc_x_help() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::help_message(&opt_uc_x_help_message(
                &conf.prog_name,
            )));
            return Err(errs);
        }
        if conf.is_opt_uc_x_package_version_info() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::help_message(&opt_uc_x_package_version_info(
                &conf.prog_name,
            )));
            return Err(errs);
        }
    }
    if conf.opt_query.is_some() {
        let mut errs = OptParseErrors::new();
        let s = conf.opt_query.unwrap();
        match s.as_str() {
            "locale" => {
                errs.push(OptParseError::version_message(&query_locale(
                    &conf.prog_name,
                )));
            }
            _ => {
                errs.push(OptParseError::version_message(&query_error(
                    &conf.prog_name,
                    s.as_str(),
                )));
            }
        }
        return Err(errs);
    }
    //
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        if conf.flg_all {
            conf.flg_bytes = true;
            conf.flg_chars = true;
            conf.flg_lines = true;
            conf.flg_words = true;
            conf.flg_max_line_bytes = true;
        } else if !conf.flg_bytes && !conf.flg_chars && !conf.flg_lines && !conf.flg_words {
            errs.push(OptParseError::missing_option("b, c, l, w or a"));
        }
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                errs.push(OptParseError::unexpected_argument(&free[0]));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
