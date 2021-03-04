use anyhow::Result;

use kasm::{cpu::{CPU, ExecResult}, lexer::Document, RAM};

fn main() -> Result<()> {
    let file = std::fs::read_to_string(r"C:\Users\info\Code\Rust\kasm\examples\loop.kasm")?;
    let doc = Document::from_str(&file)?;

    let mut ram = doc.as_ram();
    let mut cpu = CPU::new(&mut ram, std::io::stdout());

    cpu.step_to_end()?;

    Ok(())
}
