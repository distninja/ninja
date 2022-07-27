use clap::{Arg, Command};
use std::error::Error;

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
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
            .arg(
                Arg::new("working_dir")
                    .short('C')
                    .value_name("DIR")
                    .help("change to DIR before doing anything else")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("enable_debug")
                    .short('d')
                    .value_name("MODE")
                    .help("enable debugging (use '-d list' to list modes)")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("input_file")
                    .short('f')
                    .value_name("FILE")
                    .help("specify input build file [default=build.ninja]")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("parallelism")
                    .short('j')
                    .value_name("N")
                    .help("run N jobs in parallel (0 means infinity) [default=18 on this system]")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("failures_allowed")
                    .short('k')
                    .value_name("N")
                    .help("keep going until N jobs fail (0 means infinity) [default=1]")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("max_load_average")
                    .short('l')
                    .value_name("N")
                    .help("do not start new jobs if the load average is greater than N")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("dry_run")
                    .short('n')
                    .help("dry run (don't run commands but act like they succeeded)")
                    .takes_value(false)
                    .required(false),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .help("don't show progress status, just command output")
                    .takes_value(false)
                    .required(false),
            )
            .arg(
                Arg::new("tool")
                    .short('t')
                    .value_name("TOOL")
                    .help("run a subtool (use '-t list' to list subtools)")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("show all command lines while building")
                    .takes_value(false)
                    .required(false),
            )
            .arg(
                Arg::new("enable_warn")
                    .short('w')
                    .value_name("FLAG")
                    .help("adjust warnings (use '-w list' to list warnings)")
                    .takes_value(true)
                    .required(false),
            )
            .version(&*self.version_info)
            .get_matches();

        match matches.value_of("config_file") {
            Some(name) => self.config_file = name.to_string(),
            None => self.config_file = "".to_string(),
        }

        Ok(())
    }
}
