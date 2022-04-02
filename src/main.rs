mod arg;
mod config;
mod ui;

use arg::arg::Argument;
use config::config::Config;
use std::error::Error;

use ui::ui::Ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = Argument {
        ..Default::default()
    };

    a.parse()?;

    let mut c = Config {
        config_file: a.config_file,
        show_ui: a.show_ui,
        version_info: a.version_info,
        ..Default::default()
    };

    c.build()?;

    if c.show_ui {
        let mut u = Ui {
            ..Default::default()
        };

        u.run()?;
    }

    Ok(())
}
