use crate::core::{Package, PackageAssetBuilder};
use flate2::read::GzDecoder;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};
use tar::Archive;
use uuid::Uuid;

pub fn read_from_file(file: File) -> Package {
    let mut package = Package::new();
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let entries = archive
        .entries()
        .expect("Package should not be empty and contain valid entries");

    let regex = Regex::new(r"\.\/([0-9a-f]{32})\/(.*)").unwrap();

    let mut found_assets: HashMap<Uuid, PackageAssetBuilder> = HashMap::new();
    for entry in entries {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();
        let path_str = path.to_str().unwrap();

        if let Some(captures) = regex.captures(path_str) {
            let asset_guid = Uuid::parse_str(&captures[1]).unwrap();
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

    package
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use uuid::uuid;

    fn get_package_file(path: &str) -> File {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push(format!("tests/fixtures/{}.unitypackage", path));
        File::open(root).unwrap()
    }

    #[test]
    fn asset_simple_material() {
        let package = read_from_file(get_package_file("simple-material"));

        assert_eq!(package.assets.len(), 1);
        assert!(package
            .assets
            .contains_key(&uuid!("c24c6def6556015fb913fec2280e3315")));
    }

    #[test]
    fn asset_simple_cube() {
        let package = read_from_file(get_package_file("simple-cube"));

        assert_eq!(package.assets.len(), 2);
        assert!(package
            .assets
            .contains_key(&uuid!("c24c6def6556015fb913fec2280e3315")));
        assert!(package
            .assets
            .contains_key(&uuid!("8109a09196ba303c59774d4f4048f48c")));
    }
}
