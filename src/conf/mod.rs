pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::RustVersionInfo = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_map_ascii_rust_src(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::MapAsciiRustSrc = o {
                return true;
            }
        }
        false
    }
}
