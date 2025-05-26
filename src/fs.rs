#![allow(dead_code)]

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
    Ok(exists(path)? && path.is_file() && path.extension().is_some_and(|x| x == OsStr::new("py")))
}

/// determines whether given path is a Python package
/// i.e. a directory with a `__init__.py` file
/// see <https://docs.python.org/3/tutorial/modules.html#packages>
/// # Errors
/// returns an error if there is any `fs` error
pub fn is_python_package(path: &Path) -> Result<bool> {
    Ok(exists(path)? && path.is_dir() && exists(path.join("__init__.py"))?)
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
    if is_python_module(path)? {
        let name: Result<String> = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(ToString::to_string)
            .ok_or_eyre("Could not determine file name of python module due to fs error");
        name
    } else {
        Err(eyre!(format!(
            "{} is not a python module, thus it's name could not be determined",
            path.display()
        )))
    }
}

/// determines the name of a python package on disk
/// which is the name of the directory containing the `__init__.py` file.
/// ```
/// # use snakeoil::fs::{get_package_name, create_empty_python_package_on_disk};
/// # use color_eyre::Result;
/// # use assert_fs::prelude::*;
/// # fn foo() -> Result<()> {
///   let temp_dir = assert_fs::TempDir::new()?;
///   let pkg_path = temp_dir.join("test");
///   create_empty_python_package_on_disk(&temp_dir.join("test"));
///   assert_eq!(get_package_name(&temp_dir.join("test"))?, String::from("test"));
/// # Ok(())
/// # }
/// ```
/// # Errors
/// returns an error if there is any `fs` error or
/// if the provided directory is not a python package
pub fn get_package_name(path: &Path) -> Result<String> {
    if is_python_package(path)? {
        let name: Result<String> = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(ToString::to_string)
            .ok_or_eyre("Could not determine file name of python package due to fs error");
        name
    } else {
        Err(eyre!(format!(
            "{} is not a python package, thus it's name could not be determined",
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

/// will walk the provided path and index all the subpackages and modules
/// # Errors
/// Can error on fs errors
pub fn walk_package(pkg_path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut sub_modules = vec![];
    let mut sub_packages = vec![];

    for entry in WalkDir::new(pkg_path).into_iter().filter_entry(|e| {
        dbg!(e);
        is_python_package(e.path()).unwrap_or(false) || is_python_module(e.path()).unwrap_or(false)
    }) {
        let module_or_package = entry?;
        let module_or_package_path = module_or_package.path();
        dbg!(&module_or_package_path);
        if is_python_module(module_or_package_path)? {
            sub_modules.push(module_or_package_path.to_path_buf());
        } else {
            sub_packages.push(module_or_package_path.to_path_buf());
        }
    }

    Ok((sub_packages, sub_modules))
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;

    use std::fs::create_dir;

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
    fn non_existing_dir_is_not_python_module() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        assert!(get_module_name(&module_dir).is_err());

        Ok(())
    }

    #[test]
    fn correctly_determines_dummy_python_package_name() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        create_empty_python_package_on_disk(&module_dir)?;

        assert_eq!(get_package_name(&module_dir)?, String::from("test"));

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
    fn determine_module_names_fails_on_non_module() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        create_dir(&module_dir)?;

        assert!(get_module_name(&module_dir).is_err());

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
    fn get_module_name_err() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let module_dir = temp_dir.join("test");
        create_dir(&module_dir)?;

        assert!(get_module_name(&module_dir).is_err());
        Ok(())
    }
    #[test]
    fn test_get_package_name() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let pkg_path = temp_dir.join("test");
        create_empty_python_package_on_disk(&pkg_path)?;
        assert_eq!(get_package_name(&pkg_path)?, String::from("test"));
        Ok(())
    }
    #[test]
    fn test_get_package_name_err() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let pkg_path = temp_dir.join("foo.py");
        let _ = File::create(&pkg_path)?;
        assert!(get_package_name(&pkg_path).is_err());
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

        let (mut sub_pkgs, mut sub_modules) = walk_package(&root_pkg_path)?;

        sub_pkgs.sort();
        sub_modules.sort();

        let expected_sub_pkgs: Vec<PathBuf> = vec!["", "a", "b", "b/c"]
            .into_iter()
            .map(|s| root_pkg_path.join(s))
            .collect();

        assert_eq!(sub_pkgs, expected_sub_pkgs);

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

        assert_eq!(sub_modules, expected_sub_modules);
        Ok(())
    }
}
