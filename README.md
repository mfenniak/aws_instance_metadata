# aws_instance_metadata

This is a small Rust library to pull AWS instance metadata for the currently running AWS instance from the AWS EC2 metadata web service (http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html).

It currently only supports basic identity metadata; region, availability zone, instance type, account id, instance id, AMI id (image_id), and private IP.

Documentation: <https://docs.rs/aws_instance_metadata>

Distributed under the terms of the MIT license.

## Example

Cargo.toml:
```toml
[dependencies]
aws_instance_metadata = "0.1"
```

main.rs:
```rust
extern crate aws_instance_metadata;

fn main() {
    let metadata = aws_instance_metadata::get().unwrap();
    println!("instance_id: {:?}", metadata.instance_id);
    println!("region: {:?}", metadata.region());
    println!("ip: {:?}", metadata.private_ip());
}
```
