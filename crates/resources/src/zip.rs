use std::{
    io::{Read, Seek},
    sync::Mutex,
};

use thiserror::Error;
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
        let mut file = zip_archive.by_name(path.as_ref())?;
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
