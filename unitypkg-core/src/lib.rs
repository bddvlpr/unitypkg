pub mod reader;
pub mod unpack;
pub mod writer;

use std::collections::HashMap;

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub assets: HashMap<Uuid, PackageAsset>,
}

impl Package {
    pub fn new() -> Self {
        Package {
            assets: HashMap::new(),
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackageAsset {
    pub pathname: String,
    pub preview: Option<Vec<u8>>,
    pub meta: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

impl PackageAsset {
    pub fn new(pathname: String) -> Self {
        PackageAsset {
            pathname,
            preview: None,
            meta: None,
            data: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct PackageAssetBuilder {
    pub pathname: Option<String>,
    pub preview: Option<Vec<u8>>,
    pub meta: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

impl PackageAssetBuilder {
    pub fn with_pathname(mut self, pathname: String) -> PackageAssetBuilder {
        self.pathname = Some(pathname);
        self
    }

    pub fn with_preview(mut self, preview: Vec<u8>) -> PackageAssetBuilder {
        self.preview = Some(preview);
        self
    }

    pub fn with_meta(mut self, meta: Vec<u8>) -> PackageAssetBuilder {
        self.meta = Some(meta);
        self
    }

    pub fn with_data(mut self, data: Vec<u8>) -> PackageAssetBuilder {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> PackageAsset {
        PackageAsset {
            pathname: self.pathname.expect("Asset has a required path"),
            preview: self.preview,
            meta: self.meta,
            data: self.data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_package() {
        let package_from_internal = Package {
            assets: HashMap::new(),
        };
        let package = Package::new();

        assert_eq!(package_from_internal, package);
    }

    #[test]
    fn create_package_asset() {
        let packageasset_from_internal = PackageAsset {
            pathname: String::from("Assets/TestMaterial.mat"),
            preview: None,
            meta: None,
            data: None,
        };
        let packageasset = PackageAsset::new(String::from("Assets/TestMaterial.mat"));

        assert_eq!(packageasset_from_internal, packageasset);
    }
}
