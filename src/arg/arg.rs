use clap::{Arg, Command};
use std::error::Error;

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
    pub show_ui: bool,
    pub version_info: String,
}

impl Argument {
    pub fn new() -> Self {
        Argument {
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        self.version_info =
            concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")).to_string();

        let matches = Command::new("ninja")
            .version(&*self.version_info)
            .arg(
                Arg::new("config_file")
                    .short('c')
                    .long("config-file")
                    .value_name("NAME")
                    .help("Config file (.yml)")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::new("show_ui")
                    .short('u')
                    .long("show-ui")
                    .help("Show UI")
                    .takes_value(false)
                    .required(false),
            )
            .get_matches();

        match matches.value_of("config_file") {
            Some(name) => self.config_file = name.to_string(),
            None => self.config_file = "".to_string(),
        }

        if matches.is_present("show_ui") {
            self.show_ui = true;
        } else {
            self.show_ui = false;
        }

        Ok(())
    }
}
