use dspbp::cmdline;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();
    cmdline()
}
