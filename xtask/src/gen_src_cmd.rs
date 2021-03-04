use flood_tide_gen::gen_src_match;
use flood_tide_gen::parse_input_file;
use flood_tide_gen::update_file;
use flood_tide_gen::{gen_src_help, SrcHelpFlags};
use flood_tide_gen::{MetaType, OptStr};

pub fn do_gen_src() -> anyhow::Result<()> {
    let (mut vec_optstr, vec_line) = parse_input_file("xtask/src/cmd.txt")?;
    //
    fix_type(&mut vec_optstr);
    //
    let sss = gen_src_help(&vec_optstr, &vec_line, SrcHelpFlags::default())?;
    update_file(&sss, "src/conf/cmd.help.rs.txt")?;
    //
    let sss = gen_src_match(&vec_optstr)?;
    update_file(&sss, "src/conf/cmd.match.rs.txt")?;
    //
    Ok(())
}

fn fix_type(vec_optstr: &mut [OptStr]) {
    for v in vec_optstr {
        let v_meta_type = match v.lon.as_str() {
            "head" => MetaType::Usize,
            "tail" => MetaType::Usize,
            "locale" => MetaType::Other("opt_locale_loc".to_string()),
            "query" => MetaType::String,
            _ => v.meta_type.clone(),
        };
        //
        v.meta_type = v_meta_type;
        //
        /*
        let v_is_vec = match v.lon.as_str() {
            "expression" => true,
            "format" => true,
            _ => false,
        };
        v.is_vec = v_is_vec;
        */
        let v_is_opt = match v.lon.as_str() {
            "query" => true,
            _ => false,
        };
        v.is_opt = v_is_opt;
    }
}