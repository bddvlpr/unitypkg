use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use tar::{Builder, Header};

use crate::Package;

pub fn write_package<W: Write>(package: Package, w: W, compression: Compression) {
    let encoder = GzEncoder::new(w, compression);
    let mut builder = Builder::new(encoder);

    for (uuid, asset) in package.assets {
        let guid_str = uuid.to_string().replace("-", "");

        {
            let path = format!("./{}/pathname", guid_str);
            let mut header = Header::new_gnu();
            header.set_path(&path).unwrap();
            header.set_size(asset.pathname.len() as u64);
            header.set_cksum();
            builder.append(&header, asset.pathname.as_bytes()).unwrap();
        }

        if let Some(preview) = asset.preview {
            let path = format!("./{}/preview.png", guid_str);
            let mut header = Header::new_gnu();
            header.set_path(&path).unwrap();
            header.set_size(preview.len() as u64);
            header.set_cksum();
            builder.append(&header, preview.as_slice()).unwrap();
        }

        if let Some(meta) = asset.meta {
            let path = format!("./{}/asset.meta", guid_str);
            let mut header = Header::new_gnu();
            header.set_path(&path).unwrap();
            header.set_size(meta.len() as u64);
            header.set_cksum();
            builder.append(&header, meta.as_slice()).unwrap();
        }

        if let Some(data) = asset.data {
            let path = format!("./{}/asset", guid_str);
            let mut header = Header::new_gnu();
            header.set_path(&path).unwrap();
            header.set_size(data.len() as u64);
            header.set_cksum();
            builder.append(&header, data.as_slice()).unwrap();
        }
    }

    builder.finish().unwrap();
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        io::{Cursor, Read},
    };

    use flate2::read::GzDecoder;
    use tar::Archive;
    use uuid::Uuid;

    use super::*;
    use crate::PackageAssetBuilder;

    #[test]
    fn write() {
        let mut package = Package::new();
        let uuid = Uuid::new_v4();
        let guid_str = uuid.to_string().replace("-", "");
        let mut builder = PackageAssetBuilder::default();
        builder.pathname = Some("test_pathname".to_string());
        builder.preview = Some(vec![1, 2, 3, 4]);
        builder.meta = Some(vec![5, 6, 7, 8]);
        builder.data = Some(vec![9, 10, 11, 12]);

        package.assets.insert(uuid, builder.build());

        let mut buffer = Cursor::new(Vec::new());
        write_package(package, &mut buffer, Compression::default());

        let output = buffer.into_inner();
        assert!(!output.is_empty());

        let decoder = GzDecoder::new(Cursor::new(output));
        let mut archive = Archive::new(decoder);

        let mut expected_files = HashMap::new();
        expected_files.insert(
            format!("{}/pathname", guid_str),
            "test_pathname".as_bytes().to_vec(),
        );
        expected_files.insert(format!("{}/preview.png", guid_str), vec![1, 2, 3, 4]);
        expected_files.insert(format!("{}/asset.meta", guid_str), vec![5, 6, 7, 8]);
        expected_files.insert(format!("{}/asset", guid_str), vec![9, 10, 11, 12]);

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap().to_str().unwrap().to_string();
            let mut content = Vec::new();
            entry.read_to_end(&mut content).unwrap();

            if let Some(expected_content) = expected_files.get(&path) {
                assert_eq!(content, *expected_content);
                expected_files.remove(&path);
            } else {
                panic!("Unexpected file in archive: {}", path);
            }
        }

        assert!(expected_files.is_empty());
    }
}
