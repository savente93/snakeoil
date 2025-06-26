use std::{fs::File, io::Write, path::PathBuf};

use reqwest::blocking::Response;

use crate::indexing::cache::init_cache;
use color_eyre::Result;

pub fn cache_remote_objects_inv(
    url: &str,
    project_name: String,
    maybe_cache_path: Option<PathBuf>,
) -> Result<()> {
    let response = fetch_objects_inv_blocking(url)?;
    let data = response.bytes()?;

    create_object_inv_cache(data.to_vec(), project_name, maybe_cache_path)?;
    Ok(())
}

pub fn fetch_objects_inv_blocking(url: &str) -> Result<Response> {
    Ok(reqwest::blocking::get(url)?)
}

fn create_object_inv_cache(
    data: Vec<u8>,
    project_name: String,
    maybe_cache_path: Option<PathBuf>,
) -> Result<()> {
    let cache_path = init_cache(maybe_cache_path)?
        .join("sphinx")
        .join(PathBuf::from(project_name.to_lowercase()))
        .with_extension("inv");

    let mut file = File::create(cache_path)?;

    file.write_all(&data)?;

    Ok(())
}

#[cfg(test)]
mod test {

    use std::fs::exists;

    use assert_fs::TempDir;
    use color_eyre::Result;

    use crate::indexing::fetch::cache_remote_objects_inv;

    #[test]
    fn cache_clean_numpy_obj_inv() -> Result<()> {
        let url = "https://numpy.org/doc/stable/objects.inv";

        let tmp_dir = TempDir::new()?;
        cache_remote_objects_inv(url, "numpy".to_string(), Some(tmp_dir.path().to_path_buf()))?;

        assert!(exists(
            tmp_dir
                .path()
                .join("sphinx")
                .join("numpy")
                .with_extension("inv")
        )?);

        Ok(())
    }
}
