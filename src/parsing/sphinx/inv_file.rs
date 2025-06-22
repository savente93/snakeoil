use color_eyre::{Result, eyre::eyre};
use flate2::bufread::ZlibDecoder;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq)]
pub enum SphinxInvVersion {
    V1,
    V2,
}

fn parse_inv_version<R: Read>(reader: &mut BufReader<R>) -> Result<SphinxInvVersion> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // this is how sphinx itself does it https://github.com/sphinx-doc/sphinx/blob/ac3f74a3e0fbb326f73989a16dfa369e072064ca/sphinx/util/inventory.py#L126
    // this part should only have ascii so we should be fine using just chars
    // even though they might be unicode
    let v: String = line.chars().skip(27).collect::<String>().trim().to_string();
    let version: usize = v.parse()?;

    match version {
        1 => Ok(SphinxInvVersion::V1),
        2 => Ok(SphinxInvVersion::V2),
        _ => Err(eyre!("unknown version: {version}")),
    }
}

fn parse_inv_project_name<R: Read>(reader: &mut BufReader<R>) -> Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // this is how sphinx itself does it https://github.com/sphinx-doc/sphinx/blob/ac3f74a3e0fbb326f73989a16dfa369e072064ca/sphinx/util/inventory.py#L126
    let (_, proj_name) = line.split_at(11);

    Ok(proj_name.trim().to_string())
}

fn parse_inv_project_version<R: Read>(reader: &mut BufReader<R>) -> Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // this is how sphinx itself does it https://github.com/sphinx-doc/sphinx/blob/ac3f74a3e0fbb326f73989a16dfa369e072064ca/sphinx/util/inventory.py#L126
    let (_, proj_version) = line.split_at(11);

    Ok(proj_version.trim().to_string())
}

fn parse_sphinx_inv_header<R: Read>(
    reader: &mut BufReader<R>,
) -> Result<(SphinxInvVersion, String, String)> {
    let inv_version = parse_inv_version(reader)?;
    if inv_version != SphinxInvVersion::V2 {
        return Err(eyre!("Inventory file format not supported"));
    }
    let inv_project_name = parse_inv_project_name(reader)?;
    let inv_project_version = parse_inv_project_version(reader)?;
    let mut warning_header = String::new();
    reader.read_line(&mut warning_header)?;

    if !warning_header.contains("zlib") {
        return Err(eyre!("Sphinx inventory file was not compressed with zlib"));
    }

    Ok((inv_version, inv_project_name, inv_project_version))
}

fn decompress_remaining_zlib_data<R: Read>(reader: &mut BufReader<R>) -> Result<String> {
    // Read the rest of the file and decompress it using zlib
    let mut compressed_data = Vec::new();
    reader.read_to_end(&mut compressed_data)?;

    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    let decompressed_text = String::from_utf8(decompressed_data)?;
    Ok(decompressed_text)
}

#[cfg(test)]
mod test {
    use assert_fs::TempDir;
    use color_eyre::Result;
    use std::fs::File;
    use std::io::{BufReader, Write};
    use std::path::PathBuf;

    use crate::parsing::sphinx::inv_file::{
        SphinxInvVersion, decompress_remaining_zlib_data, parse_inv_version,
        parse_sphinx_inv_header,
    };

    fn write_test_header(header: &str) -> Result<(TempDir, PathBuf)> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.join("header_file.inv");
        let mut file = File::create(&path)?;
        file.write_all(header.as_bytes())?;

        Ok((temp_dir, path))
    }

    #[test]
    fn test_numpy_header() -> Result<()> {
        let header = "# Sphinx inventory version 2
# Project: NumPy
# Version: 2.3
# The remainder of this file is compressed using zlib.";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let (inv_ver, proj_name, proj_ver) = parse_sphinx_inv_header(&mut reader)?;
        assert_eq!(inv_ver, SphinxInvVersion::V2);
        assert_eq!(proj_name, "NumPy".to_string());
        assert_eq!(proj_ver, "2.3".to_string());
        Ok(())
    }

    #[test]
    fn test_garbange_header() -> Result<()> {
        let header = "# Sphinx inventory version 3.14...
# Project: asdfasdf
# Version: ll.3
# The remainder of this file is compressed using my butt.";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let result = parse_sphinx_inv_header(&mut reader);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_short_header() -> Result<()> {
        let header = "# Sphinx inventory version 2
# Project: NumPy
# Version: 2.3";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let result = parse_sphinx_inv_header(&mut reader);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_no_zlib_header() -> Result<()> {
        let header = "# Sphinx inventory version 2
# Project: NumPy
# Version: 2.3
# The remainder of this file is compressed using gzip.";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let result = parse_sphinx_inv_header(&mut reader);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_old_inv_version() -> Result<()> {
        let header = "# Sphinx inventory version 1";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let inv_ver = parse_inv_version(&mut reader)?;
        assert_eq!(inv_ver, SphinxInvVersion::V1);
        Ok(())
    }

    #[test]
    fn test_unknown_inv_version() -> Result<()> {
        let header = "# Sphinx inventory version 3.14";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let inv_ver = parse_inv_version(&mut reader);
        assert!(inv_ver.is_err());
        Ok(())
    }
    #[test]
    fn test_garbage_inv_version() -> Result<()> {
        let header = "# asdfasdfsadf";

        let (_temp_dir, header_path) = write_test_header(header)?;
        let file = File::open(header_path)?;
        let mut reader = BufReader::new(&file);
        let inv_ver = parse_inv_version(&mut reader);
        assert!(inv_ver.is_err());
        Ok(())
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn load_numpy_file() -> Result<()> {
        let filename = "tests/sphinx_objects/numpy.inv";
        let file = File::open(filename).expect("could not load file");
        let mut reader = BufReader::new(&file);

        let (inv_ver, proj_name, proj_ver) = parse_sphinx_inv_header(&mut reader)?;
        assert_eq!(inv_ver, SphinxInvVersion::V2);
        assert_eq!(proj_name, "NumPy".to_string());
        assert_eq!(proj_ver, "2.3".to_string());

        let _decompressed = decompress_remaining_zlib_data(&mut reader)?;

        Ok(())
    }
}
