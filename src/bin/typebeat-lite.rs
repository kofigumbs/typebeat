use anyhow::Error;

fn main() -> Result<(), Error> {
    let _controller = typebeat::start()?;
    Ok(())
}
