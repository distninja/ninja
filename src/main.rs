mod arg;
mod config;
mod ui;

use arg::arg::Argument;
use config::config::Config;
use std::process;
use ui::ui::Ui;

fn main() {
    let mut a = Argument {
        ..Default::default()
    };

    if let Err(err) = a.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut c = Config {
        config_file: a.config_file,
        show_ui: a.show_ui,
        version_info: a.version_info,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }

    if c.show_ui {
        let mut u = Ui {
            ..Default::default()
        };

        if let Err(err) = u.run() {
            println!("failed to run ui: {}", err);
            process::exit(-3);
        }
    }
}
