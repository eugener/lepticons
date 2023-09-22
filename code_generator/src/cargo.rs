use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CargoToml {
    pub package: Table,
    pub lib: Table,
    pub dependencies: Table,
    pub features: Table,
}

impl CargoToml {

    pub fn load(path: String) -> Self {

        let cargo_str = std::fs::read_to_string(&PathBuf::from(path)).expect("read config.toml");
        // println!("{}", cargo_str);
        let cfg: CargoToml = toml::from_str(&cargo_str).expect("parse config.toml");
        // println!("{}", cfg.features.get("default").unwrap());
        cfg
    }

    pub fn store(&self,path: String)  {
        let cargo_str = toml::to_string(&self).unwrap();
        std::fs::write(&PathBuf::from(path), cargo_str).expect("write Cargo.toml");
    }
}

