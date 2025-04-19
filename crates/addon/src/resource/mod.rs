use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sha2::{Digest, Sha256};
use zip::ZipArchive;
use crate::error::AddonError;
use crate::error::AddonError::{FormatError, IOError, JsonError};
use crate::language::Languages;
use crate::manifest::AddonManifest;
use crate::Addon;

#[derive(Debug)]
pub struct ResourcePack {
    pub manifest: AddonManifest,
    pub languages: Option<Languages>,
    pub sha256: Vec<u8>,
    file: File,
    buf: Vec<u8>
}

impl ResourcePack {
    pub fn get_pack_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn get_pack_chunk(&mut self, off: u64, len: usize) -> Result<Vec<u8>, AddonError> {
        if len < 1 {
            return Err(AddonError::FormatError {
                message: "1".to_string(),
                path: Default::default(),
                line: None,
                column: None,
            });
        }

        let mut file = self.file.try_clone().unwrap();

        // Seek to the start offset
        file.seek(SeekFrom::Start(off)).expect("file seek");

        // Create a buffer with at most `len` size
        let mut buffer = Vec::with_capacity(len);
        let mut limited_reader = file.take(len as u64);

        // Read as much as we can up to `len`
        let bytes_read = limited_reader.read_to_end(&mut buffer).expect("file read");
        buffer.truncate(bytes_read); // probably not necessary, but safe

        Ok(buffer)
    }
}

impl Addon for ResourcePack {
    fn manifest(&self) -> &AddonManifest {
        &self.manifest
    }

    fn import(path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let mut file = File::open(path).map_err(|e| IOError(Arc::new(e), path.to_path_buf()))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| IOError(Arc::new(e), path.to_path_buf()))?;
        let sha256 = Sha256::digest(&buf).to_vec();
        let reader = Cursor::new(buf.clone());
        let mut zip = ZipArchive::new(reader).map_err(|e| AddonError::ZipError(Arc::new(e), path.to_path_buf()))?;
        let mut manifest_json = String::new();


        let mut manifest_file = zip.by_name("manifest.json")
            .map_err(|e| AddonError::FormatError{
                message: "".to_string(),
                path: Default::default(),
                line: None,
                column: None,
            })?;
        manifest_file.read_to_string(&mut manifest_json)
            .map_err(|e| AddonError::FormatError{
                message: "".to_string(),
                path: Default::default(),
                line: None,
                column: None,
            })?;


        let manifest: AddonManifest =
            serde_json::from_str(&manifest_json).map_err(|e| JsonError(Arc::new(e), PathBuf::from("manifest.json")))?;
        // Manifest
        // let manifest_path = path.join("manifest.json");
        // let manifest = fs::read_to_string(&manifest_path)
        //     .map_err(|e| IOError(Arc::new(e), manifest_path.clone()))?;
        // let manifest: AddonManifest =
        //     serde_json::from_str(&manifest).map_err(|e| JsonError(Arc::new(e), manifest_path))?;
        //
        // // Languages
        // let languages = Languages::import(path.join("texts"))?;
        Ok(Self {
            manifest,
            languages: None,
            sha256,
            file,
            buf
        })
    }

    fn export(_path: impl AsRef<Path>) -> Result<Self, AddonError>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn merge(_addons: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
