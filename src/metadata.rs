extern crate rusoto;
extern crate hyper;
extern crate serde_json;
extern crate serde;

use rusoto::{Region, ParseRegionError};
use myerr::MetadataRetrievalError;
use std::str::FromStr;
use std::net::{IpAddr, AddrParseError};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceMetadata {
    private_ip: String,
    pub availability_zone: String,
    pub instance_id: String,
    pub instance_type: String,
    pub account_id: String,
    architecture: String, // FIXME: best data type?
    pub image_id: String,
    region: String, 
    // FIXME: devpayProductCodes
    // FIXME: billingProducts
    // FIXME: pendingTime
    // FIXME: kernelId
    // FIXME: ramdiskId
}

impl InstanceMetadata {
    pub fn private_ip(&self) -> Result<IpAddr, AddrParseError> {
        IpAddr::from_str(self.private_ip.as_str())
    }
    pub fn region(&self) -> Result<Region, ParseRegionError> {
        Region::from_str(self.region.as_str())
    }
}

pub fn parse_metadata(text: &str) -> Result<InstanceMetadata, MetadataRetrievalError> {
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(MetadataRetrievalError::JsonParseError(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    const EXAMPLE_DOCUMENT: &'static str = r#"
        {
            "privateIp" : "172.30.0.60",
            "availabilityZone" : "us-west-2a",
            "devpayProductCodes" : null,
            "version" : "2010-08-31",
            "instanceId" : "i-0edd3671c0bb87981",
            "billingProducts" : null,
            "instanceType" : "t2.nano",
            "pendingTime" : "2017-03-19T16:32:58Z",
            "accountId" : "1234321",
            "architecture" : "x86_64",
            "kernelId" : null,
            "ramdiskId" : null,
            "imageId" : "ami-f173dc91",
            "region" : "us-west-2"
        }
    "#;

    #[test]
    fn parses_metadata_json() {
        let metadata = parse_metadata(EXAMPLE_DOCUMENT).unwrap();
        assert!(metadata.private_ip == "172.30.0.60");
        assert!(metadata.availability_zone == "us-west-2a");
        assert!(metadata.instance_id == "i-0edd3671c0bb87981");
        assert!(metadata.instance_type == "t2.nano");
        assert!(metadata.account_id == "1234321");
        assert!(metadata.architecture == "x86_64");
        assert!(metadata.image_id == "ami-f173dc91");
        assert!(metadata.region == "us-west-2");
    }

    #[test]
    fn provides_parsed_ipv4() {
        let metadata = parse_metadata(EXAMPLE_DOCUMENT).unwrap();
        assert!(metadata.private_ip().unwrap() == Ipv4Addr::new(172, 30, 0, 60));
    }

    #[test]
    fn provides_region_enum() {
        let metadata = parse_metadata(EXAMPLE_DOCUMENT).unwrap();
        assert!(metadata.region().unwrap() == Region::UsWest2);
    }

    #[test]
    fn parse_error() {
        parse_metadata("{]").unwrap_err();
    }
}
