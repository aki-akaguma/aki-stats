use libaki_stats::execute;
use runnel::RunnelIoeBuilder;

fn main() {
    let program = env!("CARGO_PKG_NAME");
    let args = std::env::args().skip(1);
    //
    let sioe = RunnelIoeBuilder::new().build();
    //
    if let Err(err) = execute(&sioe, program, args) {
        let _ = sioe.pg_err().write_line(format!("{program}: {err:#}"));
        std::process::exit(1);
    };
}
