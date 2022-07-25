use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Config {
    pub config_data: ConfigData,
    pub config_file: String,
    pub version_info: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Default)]
pub struct ConfigData {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: MetaData,
    pub spec: Spec,
}

#[derive(Clone, Default)]
pub struct MetaData {
    pub name: String,
}

#[derive(Clone, Default)]
pub struct Spec {
    pub foo: String,
}

impl Config {
    pub fn new(cfg: String, ver: String) -> Self {
        Config {
            config_file: cfg,
            version_info: ver,
            ..Default::default()
        }
    }

    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        self.config()?;
        Ok(())
    }

    pub fn config(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config_file.len() == 0 {
            return Err("name invalid".into());
        }

        if !self.config_file.ends_with(".yml") && !self.config_file.ends_with(".yaml") {
            return Err("suffix invalid".into());
        }

        if !Path::new(&self.config_file).exists() {
            return Err(format!("{} not found", self.config_file).into());
        }

        let _ = File::open(&self.config_file)?;

        Ok(())
    }
}
