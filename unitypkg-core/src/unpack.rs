use std::{
    fs::{create_dir_all, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::Package;

pub fn unpack_package(package: Package, directory: &PathBuf) -> Result<(), io::Error> {
    create_dir_all(&directory)?;

    for asset in package.assets.values() {
        let asset_file = directory.join(&asset.pathname);
        let meta_file = asset_file.with_extension("meta");

        if let Some(parent) = asset_file.parent() {
            create_dir_all(parent)?;
        }

        if let Some(data) = &asset.data {
            let mut file = File::create(&asset_file).unwrap();
            file.write_all(data)?;
        }

        if let Some(meta) = &asset.meta {
            let mut file = File::create(&meta_file).unwrap();
            file.write_all(meta)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        io::Read,
    };

    use uuid::Uuid;

    use super::*;
    use crate::PackageAssetBuilder;

    #[test]
    fn unpack() {
        let temp_dir = temp_dir().join("test_unpack");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir).unwrap();
        }
        create_dir(&temp_dir).unwrap();

        let mut package = Package::new();
        let uuid = Uuid::new_v4();
        let mut builder = PackageAssetBuilder::default();
        builder.pathname = Some("Assets/SomeMaterial".to_string());
        builder.data = Some(vec![9, 10, 11, 12]);
        builder.meta = Some(vec![5, 6, 7, 8]);
        package.assets.insert(uuid, builder.build());

        unpack_package(package, &temp_dir).unwrap();

        let asset_file = temp_dir.join("Assets/SomeMaterial");
        let meta_file = asset_file.with_extension("meta");

        assert!(asset_file.exists());
        assert!(meta_file.exists());

        let mut data = Vec::new();
        File::open(&asset_file)
            .unwrap()
            .read_to_end(&mut data)
            .unwrap();
        assert_eq!(data, vec![9, 10, 11, 12]);

        let mut meta = Vec::new();
        File::open(meta_file)
            .unwrap()
            .read_to_end(&mut meta)
            .unwrap();
        assert_eq!(meta, vec![5, 6, 7, 8]);

        remove_dir_all(temp_dir).unwrap();
    }
}
