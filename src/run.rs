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

struct StatsAscii {
    ascii: Vec<u64>,
}
impl StatsAscii {
    fn new(sz: usize) -> StatsAscii {
        let mut v = Vec::with_capacity(sz);
        v.resize(sz, 0);
        Self { ascii: v }
    }
    fn count_up(&mut self, b: u8) {
        if b < 128 {
            self.ascii[b as usize] += 1;
        }
    }
    fn get_count(&self, idx: usize) -> u64 {
        if idx < 128 {
            self.ascii[idx]
        } else {
            0
        }
    }
    fn max(&self) -> u64 {
        *self.ascii.iter().max().unwrap()
    }
}
impl std::default::Default for StatsAscii {
    fn default() -> Self {
        Self {
            ascii: Vec::with_capacity(0),
        }
    }
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut stats = Stats::default();
    let mut map_ascii = if conf.flg_map_ascii {
        StatsAscii::new(128)
    } else {
        StatsAscii::default()
    };
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
        //
        if conf.flg_map_ascii {
            for b in line_ss.as_bytes() {
                map_ascii.count_up(*b);
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
    if conf.flg_map_ascii {
        if conf.is_opt_uc_x_map_ascii_rust_src() {
            let mut vec: Vec<String> = Vec::new();
            let max_val = map_ascii.max();
            for i in 0x00..0x80 {
                let val = map_ascii.get_count(i);
                let val = val * 255 / max_val;
                vec.push(format!("{}", val as u8));
            }
            //
            let mut o = sioe.pout().lock();
            o.write_fmt(format_args!(
                "const ASCII_STOCHAS: [u8;128] = [{}];\n",
                vec.join(", ")
            ))?;
            o.flush()?;
        } else {
            let mut vec: Vec<String> = Vec::new();
            //
            let mut ascii_ctrl: u64 = 0;
            let mut ascii_ctrl_ht: u64 = 0;
            let mut ascii_ctrl_vt: u64 = 0;
            //let mut ascii_ctrl_lf: u64 = 0;
            //let mut ascii_ctrl_cr: u64 = 0;
            for i in 0..0x1F {
                let val = map_ascii.get_count(i);
                match i {
                    0x09 => ascii_ctrl_ht = val,
                    0x0B => ascii_ctrl_vt = val,
                    //0x0A => ascii_ctrl_lf = val,
                    //0x0D => ascii_ctrl_cr = val,
                    _ => ascii_ctrl += val,
                }
            }
            ascii_ctrl += map_ascii.get_count(0x7F);
            vec.push(format!("ctrl: --: {}", ascii_ctrl));
            //vec.push(format!("ctrl: lf: {}", ascii_ctrl_lf));
            //vec.push(format!("ctrl: cr: {}", ascii_ctrl_cr));
            vec.push(format!("ctrl: ht: {}", ascii_ctrl_ht));
            vec.push(format!("ctrl: vt: {}", ascii_ctrl_vt));

            //
            vec.push(format!("0x20: SP: {}", map_ascii.get_count(0x20)));
            for i in 0x21..0x7F {
                vec.push(format!(
                    "0x{:02x}:  {}: {}",
                    i,
                    i as u8 as char,
                    map_ascii.get_count(i)
                ));
            }
            //
            let mut o = sioe.pout().lock();
            o.write_fmt(format_args!("{}\n", vec.join("\n")))?;
            o.flush()?;
        }
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
