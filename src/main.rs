mod arg;
mod config;
mod ui;

use arg::arg::Argument;
use config::config::Config;
use std::error::Error;

use ui::ui::Ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = Argument::new();
    a.parse()?;

    let mut c = Config::new(a.config_file, a.show_ui, a.version_info);
    c.build()?;

    if c.show_ui {
        let mut u = Ui::new();
        u.run()?;
    }

    Ok(())
}
