mod arg;
mod util;

use arg::arg::Argument;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = Argument::new();
    a.parse()?;

    Ok(())
}
