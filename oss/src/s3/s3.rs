use std::error::Error;
use std::io::Read;
use aws_s3::S3Client;

#[derive(Clone,Copy)]
pub struct S3 {
    region: Option<String>,
    endpoints: Option<String>,
}

impl S3 {
    pub fn new(&self,endpoints:Option<String>) -> Self {
    }
}

// impl Oss for S3 {
//     fn pub_object(&self, key: &str, content: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
//         self.client.put_object()
//     }
//
//     fn pub_object_with_reader(&self, key: &str, content: &mut dyn Read) -> Result<(), Box<dyn Error + Send + Sync>> {
//         todo!()
//     }
// }