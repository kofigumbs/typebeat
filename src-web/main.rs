use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _controller = typebeat::start()?;
    Ok(())
}
