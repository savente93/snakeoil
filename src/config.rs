use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::render::SSG;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub minify: bool,
    pub output_dir: PathBuf,
    pub pkg_path: PathBuf,
    pub skip_undoc: bool,
    pub skip_private: bool,
    pub exclude: Vec<PathBuf>,
    pub ssg: SSG,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConfigBuilder {
    minify: Option<bool>,
    output_dir: Option<PathBuf>,
    pkg_path: Option<PathBuf>,
    skip_undoc: Option<bool>,
    skip_private: Option<bool>,
    exclude: Option<Vec<PathBuf>>,
    ssg: Option<SSG>,
}

impl ConfigBuilder {
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.minify = Some(minify);
        self
    }
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = Some(output_dir);
        self
    }
    pub fn with_pkg_path(mut self, pkg_path: PathBuf) -> Self {
        self.pkg_path = Some(pkg_path);
        self
    }
    pub fn with_skip_undoc(mut self, skip_undoc: bool) -> Self {
        self.skip_undoc = Some(skip_undoc);
        self
    }
    pub fn with_skip_private(mut self, skip_private: bool) -> Self {
        self.skip_private = Some(skip_private);
        self
    }
    pub fn exclude_paths(mut self, excluded: Vec<PathBuf>) -> Self {
        match &mut self.exclude {
            Some(v) => v.extend(excluded),
            None => self.exclude = Some(excluded),
        }
        self
    }
    pub fn exclude_path(mut self, excluded: PathBuf) -> Self {
        match &mut self.exclude {
            Some(v) => v.push(excluded),
            None => self.exclude = Some(vec![excluded]),
        }
        self
    }
    pub fn with_exclude(mut self, exclude: Vec<PathBuf>) -> Self {
        self.exclude = Some(exclude);
        self
    }
    pub fn with_ssg(mut self, ssg: SSG) -> Self {
        self.ssg = Some(ssg);
        self
    }
    pub fn build(self) -> Result<Config> {
        Ok(Config {
            minify: self.minify.unwrap_or(false),
            output_dir: self.output_dir.unwrap_or(PathBuf::from("_build")),
            pkg_path: self.pkg_path.unwrap_or(PathBuf::from(".")),
            skip_undoc: self.skip_undoc.unwrap_or(true),
            skip_private: self.skip_private.unwrap_or(false),
            exclude: self.exclude.unwrap_or_default(),
            ssg: self.ssg.unwrap_or(SSG::Markdown),
        })
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        let serialized = toml::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn from_path(path: &Path) -> Result<ConfigBuilder> {
        let mut file_contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut file_contents)?;
        let config: ConfigBuilder = toml::from_str(&file_contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::ConfigBuilder;
    use assert_fs::TempDir;
    use color_eyre::Result;

    #[test]
    fn empty_builder_creates_valid_config() -> Result<()> {
        let config = ConfigBuilder::default().build();
        assert!(config.is_ok());
        Ok(())
    }

    #[test]
    fn config_round_trip() -> Result<()> {
        let builder = ConfigBuilder::default()
            .with_minify(true)
            .with_skip_undoc(false)
            .with_skip_private(true);

        let tmp_dir = TempDir::new()?;
        let path = tmp_dir.join("build_config.toml");
        builder.to_file(&path)?;
        let deserialized = ConfigBuilder::from_path(&path)?;
        assert_eq!(builder, deserialized);
        Ok(())
    }
}
