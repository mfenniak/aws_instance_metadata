extern crate serde_json;
extern crate hyper;
extern crate std;

#[derive(Debug)]
pub enum MetadataRetrievalError {
    Unknown,
    JsonParseError(serde_json::Error),
    HttpRequestError(hyper::Error),
    IoError(std::io::Error),
}
