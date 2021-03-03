use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
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
    //
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
    //
    {
        let mut o = sioe.pout().lock();
        //
        if conf.flg_lines {
            o.write_fmt(format_args!("lines: {}, ", stats.line_count))?;
        }
        if conf.flg_bytes {
            o.write_fmt(format_args!("bytes: {}, ", stats.byte_count))?;
        }
        if conf.flg_chars {
            o.write_fmt(format_args!("chars: {}, ", stats.char_count))?;
        }
        if conf.flg_words {
            o.write_fmt(format_args!("words: {}, ", stats.word_count))?;
        }
        if conf.flg_max_line_bytes {
            o.write_fmt(format_args!("max: {}, ", stats.max_line_bytes))?;
        }
        //
        o.write_fmt(format_args!("\n"))?;
        o.flush()?;
    }
    /*
    #[rustfmt::skip]
    sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
    sioe.pout().lock().flush()?;
    */
    //
    Ok(())
}
