#![deny(missing_docs)]

extern crate serde_json;
extern crate hyper;
extern crate std;

/// Possible errors returned from metadata retrieval.
#[derive(Debug)]
pub enum MetadataRetrievalError {
    /// Metadata was retrieved from AWS, but could not be parsed.
    JsonParseError(serde_json::Error),
    /// Error opening HTTP connection and sending request.
    HttpRequestError(hyper::Error),
    /// Error reading metadata in HTTP response.
    IoError(std::io::Error),
}
