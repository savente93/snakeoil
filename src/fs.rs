use std::{
    ffi::OsStr,
    fs::{File, create_dir_all, exists},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use color_eyre::eyre::{OptionExt, Result, eyre};

/// determines whether given path is a Python module
/// i.e. a file with a .py extension
/// see <https://docs.python.org/3/tutorial/modules.html#modules>
/// # Errors
/// returns an error if there is any `fs` error
pub fn is_python_module(path: &Path) -> Result<bool> {
    Ok(path.extension().is_some_and(|x| x == OsStr::new("py")))
}

/// determines whether given path is a Python package
/// i.e. a directory with a `__init__.py` file
/// see <https://docs.python.org/3/tutorial/modules.html#packages>
/// # Errors
/// returns an error if there is any `fs` error
pub fn is_python_package(path: &Path) -> Result<bool> {
    Ok(path.is_dir() && exists(path.join("__init__.py"))?)
}

pub fn is_private_module(path: &Path) -> bool {
    if let Some(stem) = path.file_stem() {
        stem.to_str()
            .map(|s| s.starts_with("_") && s != "__init__")
            .unwrap_or(false)
    } else {
        false
    }
}

pub fn get_python_prefix(rel_path: &Path) -> Result<Option<String>> {
    if let Some(file_name) = rel_path.file_name() {
        if file_name == "__init__.py" {
            let parent = {
                // necessary because the parent of a relative path with only one component
                // is Some("") and we don't want that
                let temp = rel_path.parent().and_then(|p| p.parent());
                if temp == Some(&PathBuf::new()) {
                    None
                } else {
                    temp
                }
            };

            if let Some(par) = parent {
                Ok(Some(
                    par.components()
                        .filter_map(|comp| comp.as_os_str().to_str())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("."),
                ))
            } else {
                Ok(None)
            }
        } else {
            let parent = rel_path.parent();

            if let Some(par) = parent {
                Ok(Some(
                    par.components()
                        .filter_map(|comp| comp.as_os_str().to_str())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("."),
                ))
            } else {
                Ok(None)
            }
        }
    } else {
        Err(eyre!("Could not determine file name."))
    }
}

/// determines the name of a python module which is the file stem of the .py file
/// ```
/// # use snakeoil::fs::get_module_name;
/// # use color_eyre::Result;
/// # use assert_fs::prelude::*;
/// # fn foo() -> Result<()> {
///   let temp_dir = assert_fs::TempDir::new()?;
///   let path = temp_dir.child("foo.py");
///   path.touch()?;
///   assert_eq!(get_module_name(&path)?, String::from("test"));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// returns an error if there is any `fs` error or
/// if the provided directory is not a python module
pub fn get_module_name(path: &Path) -> Result<String> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_eyre("Could not determine file name of python module due to fs error");

    if let Ok(stem) = file_stem {
        if stem == "__init__" {
            path.parent()
                .and_then(|p| p.file_stem())
                .and_then(|p| p.to_str())
                .map(|s| s.to_string())
                .ok_or_eyre("could not determine name of parent dir.")
        } else {
            Ok(stem.to_string())
        }
    } else {
        Err(eyre!(format!(
            "{} is not a python module, thus it's name could not be determined",
            path.display()
        )))
    }
}

/// Convenience function to create empty python package on disc
/// # Errors
/// can error if there are any `fs` errors
pub fn create_empty_python_package_on_disk(root: &Path) -> Result<()> {
    if !root.exists() {
        create_dir_all(root)?;
    }

    let init_file_path = root.join("__init__.py");

    let _ = File::create(init_file_path)?;
    Ok(())
}

/// determines all the submodules of a package (not recursively)
/// ```
/// # use snakeoil::fs::{get_package_modules, create_empty_python_package_on_disk};
/// # use color_eyre::Result;
/// # use assert_fs::prelude::*;
/// # use std::fs::File;
/// # fn foo() -> Result<()> {
///   let temp_dir = assert_fs::TempDir::new()?;
///   let root_pkg_path = temp_dir.join("test");
///   let sub_pkg_a_path = root_pkg_path.join("a");
///   let sub_pkg_b_path = sub_pkg_a_path .join("b");
///   create_empty_python_package_on_disk(&root_pkg_path);
///   create_empty_python_package_on_disk(&sub_pkg_a_path);
///   create_empty_python_package_on_disk(&sub_pkg_b_path);
///
///   let _ = File::create(sub_pkg_a_path.join("foo.py"))?;
///   let _ = File::create(sub_pkg_a_path.join("bar.py"))?;
//
///   let _ = File::create(sub_pkg_b_path.join("baz.py"))?;
///
///   assert_eq!(get_package_modules(&sub_pkg_a_path)?, vec![root_pkg_path.join("foo.py"),root_pkg_path.join("bar.py")]);
///   assert_eq!(get_package_modules(&sub_pkg_b_path)?, vec![root_pkg_path.join("baz.py")]);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// returns an error if there is any `fs` error or
/// if the provided directory is not a python package
/// see  also `is_python_package`
///
pub fn get_package_modules(pkg_path: &Path) -> Result<Vec<PathBuf>> {
    if !is_python_package(pkg_path)? {
        return Err(eyre!("{} is not a package", pkg_path.display()));
    }
    let pkg_modules = std::fs::read_dir(pkg_path)?
        .filter_map(std::result::Result::ok)
        .map(|p| p.path())
        .filter(|p| is_python_module(p).is_ok_and(|b| b))
        .collect();
    Ok(pkg_modules)
}

/// determines all the subpackages of a package (not recursively)
/// ```
/// # use snakeoil::fs::{get_subpackages, create_empty_python_package_on_disk};
/// # use color_eyre::Result;
/// # use assert_fs::prelude::*;
/// # fn foo() -> Result<()> {
///   let temp_dir = assert_fs::TempDir::new()?;
///   let root_pkg_path = temp_dir.join("test");
///   let sub_pkg_a_path = root_pkg_path.join("a");
///   let sub_pkg_b_path = root_pkg_path.join("b");
///   let sub_pkg_c_path = sub_pkg_b_path .join("c");
///   create_empty_python_package_on_disk(&root_pkg_path);
///   create_empty_python_package_on_disk(&sub_pkg_a_path);
///   create_empty_python_package_on_disk(&sub_pkg_b_path);
///   create_empty_python_package_on_disk(&sub_pkg_c_path);
///   assert_eq!(get_subpackages(&root_pkg_path )?, vec![root_pkg_path.join("a"),root_pkg_path.join("b")]);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// returns an error if there is any `fs` error or
/// if the provided directory is not a python package
/// see  also `is_python_package`
///
pub fn get_subpackages(pkg_path: &Path) -> Result<Vec<PathBuf>> {
    if !is_python_package(pkg_path)? {
        return Err(eyre!("{} is not a package", pkg_path.display()));
    }

    let pkg_modules = std::fs::read_dir(pkg_path)?
        .filter_map(std::result::Result::ok)
        .map(|p| p.path())
        .filter(|p| is_python_package(p).is_ok_and(|b| b))
        .collect();
    Ok(pkg_modules)
}

#[derive(Debug)]
pub struct PackageIndex {
    pub module_paths: Vec<PathBuf>,
    pub package_paths: Vec<PathBuf>,
}

/// will walk the provided path and index all the subpackages and modules
/// # Errors
/// Can error on fs errors
pub fn walk_package(pkg_path: &Path, skip_private: bool) -> Result<PackageIndex> {
    let mut sub_modules = vec![];
    let mut sub_packages = vec![];

    for entry in WalkDir::new(pkg_path).into_iter().filter_entry(|e| {
        (is_python_package(e.path()).unwrap_or(false)
            || is_python_module(e.path()).unwrap_or(false))
            && (!is_private_module(e.path()) || !skip_private)
    }) {
        let module_or_package = entry?;
        let module_or_package_path = module_or_package.path();
        if is_private_module(module_or_package_path) && skip_private {
            continue;
        }
        if is_python_module(module_or_package_path)? {
            sub_modules.push(module_or_package_path.to_path_buf());
        } else {
            sub_packages.push(module_or_package_path.to_path_buf());
        }
    }

    Ok(PackageIndex {
        module_paths: sub_modules,
        package_paths: sub_packages,
    })
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;

    use super::*;

    use color_eyre::eyre::Result;

    #[test]
    fn created_empty_package_is_recognised() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        create_empty_python_package_on_disk(&module_dir)?;

        assert!(is_python_package(&module_dir)?);

        Ok(())
    }

    #[test]
    fn correctly_determines_dummy_python_package_name() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        create_empty_python_package_on_disk(&module_dir)?;

        assert_eq!(get_module_name(&module_dir)?, String::from("test"));

        Ok(())
    }

    #[test]
    fn correctly_determines_dummy_python_module_name() -> Result<()> {
        let temp_root = assert_fs::TempDir::new()?;
        let input_file = temp_root.child("foo.py");
        input_file.touch()?;

        assert_eq!(get_module_name(&input_file)?, String::from("foo"));

        Ok(())
    }

    #[test]
    fn test_get_module_name() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let path = temp_dir.child("foo.py");
        path.touch()?;
        assert_eq!(get_module_name(&path)?, String::from("foo"));
        Ok(())
    }

    #[test]
    fn test_get_package_name() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let pkg_path = temp_dir.join("test");
        create_empty_python_package_on_disk(&pkg_path)?;
        assert_eq!(get_module_name(&pkg_path)?, String::from("test"));
        Ok(())
    }

    #[test]
    fn test_package_modules() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let root_pkg_path = temp_dir.join("test");
        let sub_pkg_a_path = root_pkg_path.join("a");
        let sub_pkg_b_path = sub_pkg_a_path.join("b");
        create_empty_python_package_on_disk(&root_pkg_path)?;
        create_empty_python_package_on_disk(&sub_pkg_a_path)?;
        create_empty_python_package_on_disk(&sub_pkg_b_path)?;

        let _ = File::create(sub_pkg_a_path.join("foo.py"))?;
        let _ = File::create(sub_pkg_a_path.join("bar.py"))?;
        let _ = File::create(sub_pkg_b_path.join("baz.py"))?;

        assert_eq!(
            get_package_modules(&root_pkg_path)?,
            vec![root_pkg_path.join("__init__.py")]
        );
        let mut b_sub_packages = get_package_modules(&sub_pkg_b_path)?;
        b_sub_packages.sort();
        assert_eq!(
            b_sub_packages,
            vec![
                sub_pkg_b_path.join("__init__.py"),
                sub_pkg_b_path.join("baz.py"),
            ]
        );
        let mut a_sub_packages = get_package_modules(&sub_pkg_a_path)?;
        a_sub_packages.sort();
        assert_eq!(
            a_sub_packages,
            vec![
                sub_pkg_a_path.join("__init__.py"),
                sub_pkg_a_path.join("bar.py"),
                sub_pkg_a_path.join("foo.py"),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_get_subpackages() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let root_pkg_path = temp_dir.join("test");
        let sub_pkg_a_path = root_pkg_path.join("a");
        let sub_pkg_b_path = root_pkg_path.join("b");
        let sub_pkg_c_path = sub_pkg_b_path.join("c");
        create_empty_python_package_on_disk(&root_pkg_path)?;
        create_empty_python_package_on_disk(&sub_pkg_a_path)?;
        create_empty_python_package_on_disk(&sub_pkg_b_path)?;
        create_empty_python_package_on_disk(&sub_pkg_c_path)?;
        let mut sub_packages = get_subpackages(&root_pkg_path)?;
        sub_packages.sort();
        assert_eq!(
            sub_packages,
            vec![root_pkg_path.join("a"), root_pkg_path.join("b")]
        );
        Ok(())
    }

    #[test]
    fn errors_non_package_modules() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        assert!(get_package_modules(temp_dir.path()).is_err());
        Ok(())
    }
    #[test]
    fn errors_non_package_sub_packages() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        assert!(get_subpackages(temp_dir.path()).is_err());
        Ok(())
    }

    #[test]
    fn walk_package_finds_packages_and_modules() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let root_pkg_path = temp_dir.join("test");
        let sub_pkg_a_path = root_pkg_path.join("a");
        let sub_pkg_b_path = root_pkg_path.join("b");
        let sub_pkg_c_path = sub_pkg_b_path.join("c");
        create_empty_python_package_on_disk(&root_pkg_path)?;
        create_empty_python_package_on_disk(&sub_pkg_a_path)?;
        create_empty_python_package_on_disk(&sub_pkg_b_path)?;
        create_empty_python_package_on_disk(&sub_pkg_c_path)?;
        let _ = File::create(sub_pkg_a_path.join("foo.py"))?;
        let _ = File::create(sub_pkg_a_path.join("bar.py"))?;
        let _ = File::create(sub_pkg_b_path.join("baz.py"))?;

        let mut index = walk_package(&root_pkg_path, false)?;

        index.module_paths.sort();
        index.package_paths.sort();

        let expected_sub_pkgs: Vec<PathBuf> = vec!["", "a", "b", "b/c"]
            .into_iter()
            .map(|s| root_pkg_path.join(s))
            .collect();

        assert_eq!(index.package_paths, expected_sub_pkgs);

        let expected_sub_modules: Vec<PathBuf> = vec![
            "__init__.py",
            "a/__init__.py",
            "a/bar.py",
            "a/foo.py",
            "b/__init__.py",
            "b/baz.py",
            "b/c/__init__.py",
        ]
        .into_iter()
        .map(|s| root_pkg_path.join(s))
        .collect();

        assert_eq!(index.module_paths, expected_sub_modules);
        Ok(())
    }

    #[test]
    fn test_get_python_prefix_package() -> Result<()> {
        let input = PathBuf::from("foo/bar/baz/__init__.py");
        let expected = String::from("foo.bar");
        assert_eq!(get_python_prefix(&input)?, Some(expected));
        Ok(())
    }
    #[test]
    fn test_get_python_prefix_module() -> Result<()> {
        let input = PathBuf::from("foo/bar/baz/mew.py");
        let expected = String::from("foo.bar.baz");
        assert_eq!(get_python_prefix(&input)?, Some(expected));
        Ok(())
    }
    #[test]
    fn test_shallow_prefix() -> Result<()> {
        let input = PathBuf::from("foo/__init__.py");
        assert_eq!(get_python_prefix(&input)?, None);
        Ok(())
    }
}
