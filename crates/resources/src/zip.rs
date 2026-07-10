use std::{
    io::{Read, Seek},
    sync::Mutex,
};

use thiserror::Error;
use tracing::info;
use zip::{ZipArchive, result::ZipError};

use crate::provider::SonorustResourceProvider;

pub struct SonorustZipResourceProvider<R>
where
    R: Read + Seek,
{
    zip_archive: Mutex<ZipArchive<R>>,
}

impl<'a, R> SonorustZipResourceProvider<R>
where
    R: Read + Seek,
{
    pub fn new(zip_reader: R) -> Result<Self, SonorustZipResourceError> {
        let zip_archive = Mutex::new(ZipArchive::new(zip_reader)?);
        for file_name in zip_archive.lock().unwrap().file_names() {
            info!("Zip has file {}", file_name);
        }
        Ok(Self { zip_archive })
    }
}

impl<R> SonorustResourceProvider for SonorustZipResourceProvider<R>
where
    R: Read + Seek,
{
    type Error = SonorustZipResourceError;

    async fn fetch_bytes(&self, path: impl AsRef<str>) -> Result<Vec<u8>, Self::Error> {
        let mut zip_archive = self.zip_archive.lock().unwrap();
        let path = path.as_ref();
        let path = path.strip_prefix('/').unwrap_or_else(|| path);
        info!("Attempting to look up {}", path);
        let mut file = zip_archive.by_name(path)?;
        let mut bytes = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut bytes)?;
        Ok(bytes.into())
    }
}

#[derive(Debug, Error)]
pub enum SonorustZipResourceError {
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("Zip error occurred: {0}")]
    Zip(#[from] ZipError),
}
