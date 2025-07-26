use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use url::Url;

use crate::render::{
    SSG,
    formats::{Renderer, md::MdRenderer, zola::ZolaRenderer},
};

pub struct Config {
    pub output_dir: PathBuf,
    pub pkg_path: PathBuf,
    pub skip_undoc: bool,
    pub skip_private: bool,
    pub renderer: Box<dyn Renderer>,
    pub exclude: Vec<PathBuf>,
    pub externals: HashMap<String, Url>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConfigBuilder {
    output_dir: Option<PathBuf>,
    pkg_path: Option<PathBuf>,
    skip_undoc: Option<bool>,
    skip_private: Option<bool>,
    ssg: Option<SSG>,
    exclude: Option<Vec<PathBuf>>,
    externals: Option<HashMap<String, String>>,
}

impl ConfigBuilder {
    pub fn init_with_defaults(mut self) -> Self {
        self = self
            .with_output_dir(Some(PathBuf::from("_build")))
            .with_skip_undoc(Some(true))
            .with_skip_private(Some(false))
            .with_pkg_path(Some(PathBuf::from(".")))
            .with_exclude(Some(Vec::new()))
            .with_ssg(Some(SSG::Markdown))
            .with_externals(Some(HashMap::new()));

        self
    }
    pub fn with_output_dir(mut self, output_dir: Option<PathBuf>) -> Self {
        if output_dir.is_some() {
            self.output_dir = output_dir;
        }
        self
    }
    pub fn with_pkg_path(mut self, pkg_path: Option<PathBuf>) -> Self {
        if pkg_path.is_some() {
            self.pkg_path = pkg_path;
        }
        self
    }
    pub fn with_skip_undoc(mut self, skip_undoc: Option<bool>) -> Self {
        if skip_undoc.is_some() {
            self.skip_undoc = skip_undoc;
        }
        self
    }
    pub fn with_skip_private(mut self, skip_private: Option<bool>) -> Self {
        if skip_private.is_some() {
            self.skip_private = skip_private;
        }
        self
    }
    pub fn exclude_paths(&mut self, excluded: Vec<PathBuf>) {
        match &mut self.exclude {
            Some(v) => v.extend(excluded),
            None => self.exclude = Some(excluded),
        }
    }
    pub fn exclude_path(&mut self, excluded: PathBuf) {
        match &mut self.exclude {
            Some(v) => v.push(excluded),
            None => self.exclude = Some(vec![excluded]),
        }
    }
    pub fn with_exclude(mut self, exclude: Option<Vec<PathBuf>>) -> Self {
        if exclude.is_some() {
            self.exclude = exclude;
        }
        self
    }
    pub fn add_external(&mut self, name: String, link: String) -> Result<()> {
        if self.externals.is_none() {
            self.externals = Some(HashMap::new());
        }

        let externals = self.externals.get_or_insert_default();

        externals.insert(name, link);

        Ok(())
    }
    pub fn with_externals(mut self, externals: Option<HashMap<String, String>>) -> Self {
        if externals.is_some() {
            self.externals = externals
        }
        self
    }
    pub fn with_ssg(mut self, ssg: Option<SSG>) -> Self {
        if ssg.is_some() {
            self.ssg = ssg;
        }
        self
    }
    pub fn build(self) -> Result<Config> {
        let renderer: Box<dyn Renderer> = match self.ssg {
            Some(SSG::Markdown) | None => Box::new(MdRenderer::new()),
            Some(SSG::Zola) => Box::new(ZolaRenderer::new()),
        };

        let mut external_linkings = HashMap::new();

        if let Some(external_links) = self.externals {
            for (name, link) in external_links {
                let url = Url::parse(&link)?;
                external_linkings.insert(name, url);
            }
        }

        Ok(Config {
            output_dir: self.output_dir.unwrap_or(PathBuf::from("_build")),
            pkg_path: self.pkg_path.unwrap_or(PathBuf::from(".")),
            skip_undoc: self.skip_undoc.unwrap_or(true),
            skip_private: self.skip_private.unwrap_or(false),
            exclude: self.exclude.unwrap_or_default(),
            renderer,
            externals: external_linkings,
        })
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        let serialized = toml::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn merge(mut self, other: ConfigBuilder) -> Self {
        if other.output_dir.is_some() {
            self.output_dir = other.output_dir;
        }

        if other.pkg_path.is_some() {
            self.pkg_path = other.pkg_path;
        }

        if other.skip_undoc.is_some() {
            self.skip_undoc = other.skip_undoc
        }

        if other.skip_private.is_some() {
            self.skip_private = other.skip_private
        }

        if other.ssg.is_some() {
            self.ssg = other.ssg
        }

        if let Some(v) = other.exclude {
            self.exclude_paths(v)
        }

        self
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
    use std::path::PathBuf;

    use crate::render::SSG;

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
        let mut builder = ConfigBuilder::default()
            .with_skip_undoc(Some(false))
            .with_skip_private(Some(true));

        builder.exclude_paths(vec![PathBuf::from("asdf")]);

        let tmp_dir = TempDir::new()?;
        let path = tmp_dir.join("build_config.toml");
        builder.to_file(&path)?;
        let deserialized = ConfigBuilder::from_path(&path)?;
        assert_eq!(builder, deserialized);
        Ok(())
    }

    #[test]
    fn config_merge_other_takes_precident() -> Result<()> {
        let mut first = ConfigBuilder::default()
            .with_pkg_path(Some(PathBuf::from(".")))
            .with_ssg(Some(SSG::Markdown));

        first.exclude_path(PathBuf::from("asdf"));

        let second = ConfigBuilder::default()
            .with_pkg_path(Some(PathBuf::from("content")))
            .with_skip_undoc(Some(true))
            .with_exclude(Some(vec![PathBuf::from("zxcv")]));

        let third = ConfigBuilder::default()
            .with_output_dir(Some(PathBuf::from("_output")))
            .with_skip_private(Some(false))
            .with_pkg_path(Some(PathBuf::from("pkg")))
            .with_exclude(Some(vec![PathBuf::from("qwert")]))
            .with_ssg(Some(SSG::Zola));

        let expected = ConfigBuilder::default()
            .with_pkg_path(Some(PathBuf::from("pkg")))
            .with_output_dir(Some(PathBuf::from("_output")))
            .with_skip_undoc(Some(true))
            .with_skip_undoc(Some(true))
            .with_skip_private(Some(false))
            .with_exclude(Some(vec![
                PathBuf::from("asdf"),
                PathBuf::from("zxcv"),
                PathBuf::from("qwert"),
            ]))
            .with_skip_undoc(Some(true))
            .with_ssg(Some(SSG::Zola));

        let computed = first.merge(second).merge(third);
        assert_eq!(expected, computed);

        Ok(())
    }
}
