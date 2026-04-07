/// Round-trip safe Cargo.toml manipulation.
/// Only mutates the [features] table, preserving all other sections.
pub struct CargoToml {
    doc: toml::Value,
}

impl CargoToml {
    pub fn load(path: String) -> Self {
        let content = std::fs::read_to_string(&path).expect("read Cargo.toml");
        let doc: toml::Value = toml::from_str(&content).expect("parse Cargo.toml");
        Self { doc }
    }

    pub fn features(&mut self) -> &mut toml::value::Table {
        self.doc
            .get_mut("features")
            .and_then(|v| v.as_table_mut())
            .expect("[features] table in Cargo.toml")
    }

    pub fn store(&self, path: String) {
        let content = toml::to_string_pretty(&self.doc).unwrap();
        std::fs::write(&path, content).expect("write Cargo.toml");
    }
}
