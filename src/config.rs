// config module
// configuration files are stored under $XDG_CONFIG_HOME/skirr
// main config file is in skirr.json
// lua scripts are linked there, but should be stored in scripts/

use std::env;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub path: String,
}

impl Script {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub scripts: Vec<Script>,
}

impl Config {
    pub fn get_default_script(&self) -> Option<Script> {
        self.scripts.first().cloned()
    }

    fn get_filepath() -> PathBuf {
        PathBuf::from(match env::var("XDG_CONFIG_HOME") {
            Ok(val) => format!("{}/skirr/skirr.json", val),
            Err(_) => format!("{}/.config/skirr/skirr.json", env::var("HOME").unwrap())
        })
    }
}

impl Default for Config {
    fn default() -> Self {

        let filepath = Self::get_filepath();

        if filepath.exists() {
            let mut config: Config = serde_json::from_reader(std::fs::File::open(filepath).unwrap()).unwrap();
            config.scripts.iter_mut().for_each(|script| {
                script.path = String::from(Self::get_filepath().parent().unwrap().join(&script.path).to_str().unwrap());
            });
            config
        } else {
            let config = Config {
                scripts: Vec::new(),
            };
            std::fs::create_dir_all(filepath.parent().unwrap()).unwrap();
            serde_json::to_writer(std::fs::File::create(filepath).unwrap(), &config).unwrap();
            config
        }
        
    }
}
