use kloud::api::file::routes;
use kloud::storage::FileStorage;

#[tokio::test]
async fn test_upload_and_download() {
    use warp::test::request;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let storage = Arc::new(Mutex::new(FileStorage::new()));
    let api = routes(storage.clone());

    // Test upload
    let file_name = "test.txt";
    let file_content = "Hello, Warp!".as_bytes();

    let upload_response = request()
        .method("POST")
        .path(&format!("/upload/{}", file_name))
        .body(file_content)
        .reply(&api)
        .await;

    assert_eq!(upload_response.status(), 200);

    // Test download
    let download_response = request()
        .method("GET")
        .path(&format!("/download/{}", file_name))
        .reply(&api)
        .await;

    assert_eq!(download_response.status(), 200);
}
