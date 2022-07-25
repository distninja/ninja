mod arg;
mod config;

use arg::arg::Argument;
use config::config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = Argument::new();
    a.parse()?;

    let mut c = Config::new(a.config_file, a.version_info);
    c.build()?;

    Ok(())
}
