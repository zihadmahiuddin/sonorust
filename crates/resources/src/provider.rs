use std::error::Error;

pub trait SonorustResourceProvider {
    type Error: Error;

    fn fetch_bytes(
        &self,
        path: impl AsRef<str>,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>>;
}
