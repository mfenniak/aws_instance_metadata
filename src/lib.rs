//! aws_instance_metadata is a module for retrieving instance metadata when running on AWS EC2 instances.
//!
//! Instance metadata (or, meta-data) is a service that Amazon provides that uses a fixed IP address and a simple
//! HTTP API to retrieve information about the currently running EC2 instance.  This metadata is typically used
//! for reducing the amount of configuration required in software that uses the AWS APIs.  For example, the
//! current AWS region can be retrieved so that an SDK can be configured to make API calls within that region,
//! rather than having to configure software with the correct region explicitly.
//!
//! # Examples
//!
//! ```rust
//! extern crate aws_instance_metadata;
//!
//! fn main() {
//!     let metadata = aws_instance_metadata::get().unwrap();
//!     println!("instance_id: {:?}", metadata.instance_id);
//!     println!("region: {:?}", metadata.region());
//!     println!("ip: {:?}", metadata.private_ip());
//! }
//! ```

#![deny(missing_docs)]

extern crate rusoto;
extern crate hyper;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use hyper::Client;
use std::io::Read;

/// Parsing for AWS metadata service
pub mod metadata;
/// Error type definition
pub mod myerr;

use metadata::InstanceMetadata;
use myerr::MetadataRetrievalError;

/// Retrieves the AWS instance metadata.
///
/// Accesses `http://169.254.169.254/latest/dynamic/instance-identity/document` to read metadata about the current
/// running EC2 instance.  For more information, see AWS's documentation at
/// http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html
///
/// Not all metadata available via the web service is currently exposed.
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
