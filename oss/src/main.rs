use aws_sdk_s3::types;
#[::tokio::main]
async fn main() {
    let config = aws_config::from_env().region("`").load().await;
    let client = aws_sdk_s3::Client::new(&config);
    // client.list_objects().send().await.unwrap();
    let err = client
        .put_object()
        .bucket("1")
        .key("1")
        .set_body(None)
        .acl(types::ObjectCannedAcl::PublicRead)
        .send()
        .await
        .expect("11");
    println!("{:?}", client);
}
