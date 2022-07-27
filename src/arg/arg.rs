use crate::util::util::Util;
use clap::{Arg, Command};
use phf::phf_map;
use std::env;
use std::error::Error;

#[derive(Clone, Default)]
pub struct Argument {
    pub dry_run: bool,
    pub enable_debug: String,
    pub enable_warn: String,
    pub failures_allowed: u64,
    pub input_file: String,
    pub max_load_average: u64,
    pub parallelism: u64,
    pub quiet: bool,
    pub tool: String,
    pub verbose: bool,
    pub version_info: String,
    pub working_dir: String,
}

static DEBUG_MODES_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "stats" => "        print operation counts/timing info",
    "explain" => "      explain what caused a command to execute",
    "keepdepfile" => "  don't delete depfiles after they're read by ninja",
    "keeprsp" => "      don't delete @response files on success",
};

static TOOL_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "browse" => "       browse dependency graph in a web browser",
    "clean" => "        clean built files",
    "commands" => "     list all commands required to rebuild given targets",
    "inputs" => "       list all inputs required to rebuild given targets",
    "deps" => "         show dependencies stored in the deps log",
    "missingdeps" => "  check deps log dependencies on generated files",
    "graph" => "        output graphviz dot file for targets",
    "query" => "        show inputs/outputs for a path",
    "targets" => "      list targets by their rule or depth in the DAG",
    "compdb" => "       dump JSON compilation database to stdout",
    "recompact" => "    recompacts ninja-internal data structures",
    "restat" => "       restats all outputs in the build log",
    "rules" => "        list all rules",
    "cleandead" => "    clean built files that are no longer produced by the manifest",
};

static WARN_MODES_LIST: &'static [&str] = &["err", "warn"];

impl Argument {
    pub fn new() -> Self {
        Argument {
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        self.version_info =
            concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")).to_string();

        let mut parallelism_help: String =
            "run N jobs in parallel (0 means infinity) [default=".to_string();
        parallelism_help.push_str(&Util::guess_parallelism().to_string());
        parallelism_help.push_str(" on this system]");

        let mut enable_debug_help: String = "debugging modes:\n".to_string();
        for (key, val) in DEBUG_MODES_MAP.into_iter() {
            enable_debug_help.push_str("  ");
            enable_debug_help.push_str(key);
            enable_debug_help.push_str(val);
            enable_debug_help.push_str("\n");
        }
        enable_debug_help.push_str("multiple modes can be enabled via -d FOO -d BAR");

        let mut tool_help: String = "ninja subtools:\n".to_string();
        for (key, val) in TOOL_MAP.into_iter() {
            tool_help.push_str("  ");
            tool_help.push_str(key);
            tool_help.push_str(val);
            tool_help.push_str("\n");
        }

        let mut enable_warn_help: String = "warning flags:\n".to_string();
        enable_warn_help.push_str("  phonycycle={");
        for item in WARN_MODES_LIST {
            enable_warn_help.push_str(item);
            enable_warn_help.push_str(",");
        }
        enable_warn_help.push_str("}  phony build statement references itself");

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
                    .help(parallelism_help.as_str())
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
            .after_help("if targets are unspecified, builds the 'default' target (see manual).")
            .get_matches();

        match matches.value_of("working_dir") {
            Some(val) => self.working_dir = val.to_string(),
            None => self.working_dir = env::current_dir().unwrap().display().to_string(),
        }

        match matches.value_of("enable_debug") {
            Some(val) => self.enable_debug = val.to_string(),
            None => self.enable_debug = "".to_string(),
        }

        if self.enable_debug == "list" {
            println!("{}", enable_debug_help);
        }

        match matches.value_of("input_file") {
            Some(val) => self.input_file = val.to_string(),
            None => self.input_file = "build.ninja".to_string(),
        }

        match matches.value_of("parallelism") {
            Some(val) => self.parallelism = val.parse().unwrap(),
            None => self.parallelism = Util::guess_parallelism(),
        }

        match matches.value_of("failures_allowed") {
            Some(val) => self.failures_allowed = val.parse().unwrap(),
            None => self.failures_allowed = 1,
        }

        match matches.value_of("max_load_average") {
            Some(val) => self.max_load_average = val.parse().unwrap(),
            None => self.max_load_average = 0,
        }

        self.dry_run = matches.is_present("dry_run");
        self.quiet = matches.is_present("quiet");

        match matches.value_of("tool") {
            Some(val) => self.tool = val.parse().unwrap(),
            None => self.tool = "".to_string(),
        }

        if self.tool == "list" {
            println!("{}", tool_help);
        }

        self.verbose = matches.is_present("verbose");

        match matches.value_of("enable_warn") {
            Some(val) => self.enable_warn = val.parse().unwrap(),
            None => self.enable_warn = "".to_string(),
        }

        if self.enable_warn == "list" {
            println!("{}", enable_warn_help);
        }

        Ok(())
    }
}
