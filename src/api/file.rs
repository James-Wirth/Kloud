use warp::{Filter, Rejection, Reply};
use std::sync::Arc;
use tokio::sync::Mutex;
use bytes::Bytes;
use crate::encryption::aes;
use crate::storage::FileStorage;
use crate::errors::AppError;

pub fn routes(
    storage: Arc<Mutex<FileStorage>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let upload = warp::post()
        .and(warp::path("upload"))
        .and(warp::body::bytes())
        .and(warp::path::param::<String>())
        .and(with_storage(storage.clone()))
        .and_then(upload_handler);

    let download = warp::get()
        .and(warp::path("download"))
        .and(warp::path::param::<String>())
        .and(with_storage(storage))
        .and_then(download_handler);

    upload.or(download)
}

async fn upload_handler(
    body: Bytes,
    file_name: String,
    storage: Arc<Mutex<FileStorage>>,
) -> Result<impl Reply, Rejection> {
    let encrypted_data = aes::encrypt(&body).map_err(|e| warp::reject::custom(AppError::from(e)))?;
    storage.lock().await.add_file(&file_name, &encrypted_data);
    Ok(warp::reply::json(&format!("File '{}' uploaded successfully.", file_name)))
}

async fn download_handler(
    file_name: String,
    storage: Arc<Mutex<FileStorage>>,
) -> Result<impl Reply, Rejection> {
    let storage = storage.lock().await;
    if let Some(data) = storage.get_file(&file_name) {
        let decrypted_data = aes::decrypt(data).map_err(|e| warp::reject::custom(AppError::from(e)))?;
        // Convert decrypted data to a string
        let decrypted_string = String::from_utf8(decrypted_data)
            .map_err(|_| warp::reject::custom(AppError::from("Invalid UTF-8 data".to_string())))?;
        Ok(warp::reply::json(&format!(
            "File '{}' retrieved: {}",
            file_name, decrypted_string
        )))
    } else {
        Err(warp::reject::not_found())
    }
}

fn with_storage(
    storage: Arc<Mutex<FileStorage>>,
) -> impl Filter<Extract = (Arc<Mutex<FileStorage>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
