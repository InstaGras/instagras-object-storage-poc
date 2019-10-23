extern crate dotenv;
extern crate rusoto_connect;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rusoto_sqs;

use std::env;
use dotenv::dotenv;
use rusoto_connect::Credentials;
use rusoto_s3::S3Client;

fn main() {

    dotenv().expect("Failed to read .env file.");
    println!("{}", env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY not found."));

//    let client = new S3Client;

    let credentials = Credentials {
        access_token: env::var("S3_ACCESS_KEY"),
    };

    // S3Client.get_object(input: GetObjectRequest)
    
    // S3Client.put_object(input: PutObjectRequest)
}
