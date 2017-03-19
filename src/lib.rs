extern crate rusoto;
extern crate hyper;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use hyper::Client;
use std::io::Read;

mod metadata;
mod myerr;

use metadata::InstanceMetadata;
use myerr::MetadataRetrievalError;

pub fn get() -> Result<InstanceMetadata, MetadataRetrievalError> {
    let client = Client::new();
    let url = "http://169.254.169.254/latest/dynamic/instance-identity/document";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(e) => {
            return Err(MetadataRetrievalError::HttpRequestError(e));
        }
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => {
            return Err(MetadataRetrievalError::IoError(e));
        }
    };
    metadata::parse_metadata(buf.as_str())
}

#[cfg(test)]
mod tests {}
