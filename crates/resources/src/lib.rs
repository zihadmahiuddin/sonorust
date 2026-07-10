pub mod browser;
pub mod error;
pub mod extension;
#[cfg(feature = "http")]
pub mod http;
pub mod provider;
pub mod types;
#[cfg(feature = "zip")]
pub mod zip;

#[cfg(not(any(feature = "http", feature = "zip")))]
compile_error!(
    "At least one of the following features must be enabled for this crate: \"http\", \"zip\"."
);
