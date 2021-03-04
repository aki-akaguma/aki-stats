use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::fmt::Write as FmtWrite;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let r = run_0(sioe, conf);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

struct Stats {
    byte_count: u64,
    char_count: u64,
    line_count: u64,
    word_count: u64,
    max_line_bytes: u64,
}
impl std::default::Default for Stats {
    fn default() -> Self {
        Self {
            byte_count: 0,
            char_count: 0,
            line_count: 0,
            word_count: 0,
            max_line_bytes: 0,
        }
    }
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut stats = Stats::default();
    // input
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        stats.line_count += 1;
        //
        let line_bytes = line_len as u64;
        if conf.flg_bytes {
            stats.byte_count += line_bytes;
        }
        if conf.flg_max_line_bytes {
            stats.max_line_bytes = stats.max_line_bytes.max(line_bytes);
        }
        //
        if conf.flg_chars || conf.flg_words {
            let mut prev_c: char = ' ';
            for c in line_ss.chars() {
                stats.char_count += 1;
                if prev_c.is_ascii_whitespace() && !c.is_ascii_whitespace() {
                    stats.word_count += 1;
                }
                prev_c = c;
            }
        }
    }
    // output
    {
        let mut vec: Vec<String> = Vec::new();
        if conf.flg_lines {
            vec.push(my_formatted(conf, "lines", stats.line_count)?);
        }
        if conf.flg_bytes {
            vec.push(my_formatted(conf, "bytes", stats.byte_count)?);
        }
        if conf.flg_chars {
            vec.push(my_formatted(conf, "chars", stats.char_count)?);
        }
        if conf.flg_words {
            vec.push(my_formatted(conf, "words", stats.word_count)?);
        }
        if conf.flg_max_line_bytes {
            vec.push(my_formatted(conf, "max", stats.max_line_bytes)?);
        }
        //
        let mut o = sioe.pout().lock();
        o.write_fmt(format_args!("{}\n", vec.join(", ")))?;
        o.flush()?;
    }
    //
    Ok(())
}

fn my_formatted(conf: &CmdOptConf, label: &str, num: u64) -> anyhow::Result<String> {
    let mut s = String::new();
    s.write_fmt(format_args!(
        "{}:\"{}\"",
        label,
        conf.opt_locale.formatted_string(num)
    ))?;
    Ok(s)
}
