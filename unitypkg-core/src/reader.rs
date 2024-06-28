use std::{collections::HashMap, io::Read};

use flate2::read::GzDecoder;
use regex::Regex;
use tar::Archive;
use thiserror::Error;
use uuid::Uuid;

use crate::{Package, PackageAssetBuilder};

#[derive(Error, Debug)]
pub enum PackageReadError {
    #[error("Failed to read package: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to decode gzip stream: {0}")]
    GzipError(#[from] flate2::DecompressError),
    #[error("Failed to parse UUID: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("Invalid asset path: {0}")]
    InvalidAssetPath(String),
}

pub fn read_package<R: Read>(r: R) -> Result<Package, PackageReadError> {
    let mut package = Package::new();
    let decoder = GzDecoder::new(r);
    let mut archive = Archive::new(decoder);

    let entries = archive.entries()?;

    let regex = Regex::new(r"\.\/([0-9a-f]{32})\/(.*)").unwrap();

    let mut found_assets: HashMap<Uuid, PackageAssetBuilder> = HashMap::new();
    for entry in entries {
        let mut entry = entry?;
        let path = entry.path()?;
        let path_str = path
            .to_str()
            .ok_or_else(|| PackageReadError::InvalidAssetPath(format!("{:?}", path)))?;

        if let Some(captures) = regex.captures(path_str) {
            let asset_guid = Uuid::parse_str(&captures[1])?;
            let asset_path = &captures[2];

            let builder = found_assets
                .entry(asset_guid)
                .or_insert_with(PackageAssetBuilder::default);

            match asset_path {
                "pathname" => {
                    let mut pathname = String::new();
                    entry
                        .read_to_string(&mut pathname)
                        .expect("Cannot read pathname of asset");
                    builder.pathname = Some(pathname);
                }
                "preview.png" => {
                    let mut preview = Vec::new();
                    entry
                        .read_to_end(&mut preview)
                        .expect("Cannot read preview of asset");
                    builder.preview = Some(preview);
                }
                "asset.meta" => {
                    let mut meta = Vec::new();
                    entry
                        .read_to_end(&mut meta)
                        .expect("Cannot read meta of asset");
                    builder.meta = Some(meta);
                }
                "asset" => {
                    let mut data = Vec::new();
                    entry
                        .read_to_end(&mut data)
                        .expect("Cannot read data of asset");
                    builder.data = Some(data);
                }
                _ => {}
            }
        }
    }

    for (uuid, builder) in found_assets {
        package.assets.insert(uuid, builder.build());
    }

    Ok(package)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use uuid::uuid;

    use super::*;

    fn get_package_file(path: &str) -> File {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push(format!("tests/{}.unitypackage", path));
        File::open(root).unwrap()
    }

    #[test]
    fn asset_simple_material() {
        let package = read_package(get_package_file("simple-material")).unwrap();

        assert_eq!(package.assets.len(), 1);
        assert!(package
            .assets
            .contains_key(&uuid!("c24c6def6556015fb913fec2280e3315")));
    }

    #[test]
    fn asset_simple_cube() {
        let package = read_package(get_package_file("simple-cube")).unwrap();

        assert_eq!(package.assets.len(), 2);
        assert!(package
            .assets
            .contains_key(&uuid!("c24c6def6556015fb913fec2280e3315")));
        assert!(package
            .assets
            .contains_key(&uuid!("8109a09196ba303c59774d4f4048f48c")));
    }

    #[test]
    fn asset_invalid_uuid() {
        let package = read_package(get_package_file("invalid-uuid")).unwrap();

        assert_eq!(package.assets.len(), 0);
    }

    #[test]
    fn empty_archive() {
        let package = read_package(get_package_file("empty")).unwrap();

        assert_eq!(package.assets.len(), 0);
    }
}
