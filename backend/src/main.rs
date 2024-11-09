use std::{env, fs, net::SocketAddr, path::Path};

use axum::{extract::Multipart, http::header::CONTENT_TYPE, response::IntoResponse, routing::{get, post}, Json, Router};

use pyo3::prelude::*;
use tokio::task;
use uuid::Uuid;

fn set_python_executable() {
    let python_path = Path::new("..")
        .join("mlenv")
        .join("Scripts")
        .join("python.exe")
        .canonicalize()
        .unwrap(); // Unwrap to panic on error

    // Manually add site-packages to sys.path
    let site_packages = Path::new("..")
        .join("mlenv")
        .join("Lib")
        .join("site-packages")
        .canonicalize()
        .unwrap();

    Python::with_gil(|py| {
        let sys = py.import_bound("sys").unwrap();
        let sys_path = sys.getattr("path").unwrap();
        sys_path.call_method1("append", (site_packages.to_str().unwrap(),)).unwrap();
        println!("Python sys.path after appending site-packages: {:?}", sys_path);
    });

    // Set the environment variable for Python executable
    env::set_var("PYTHON_SYS_EXECUTABLE", python_path);
}

fn classify_image(image_path: &str) -> Result<String, String> {
    Python::with_gil(|py| {
        // Add the `ml` directory to the Python path
        let sys = py.import_bound("sys").map_err(|e| format!("Failed to import sys: {:?}", e))?;
        let sys_path = sys.getattr("path").map_err(|e| format!("Failed to get sys.path: {:?}", e))?;
        let ml_path = Path::new("..").canonicalize().map_err(|e| format!("Failed to resolve ml path: {:?}", e))?;
        sys_path.call_method1("append", (ml_path.to_str().unwrap(),)).map_err(|e| format!("Failed to append ml directory to Python path: {:?}", e))?;

        let paths: Vec<String> = sys_path.extract().unwrap();
        println!("Python sys.path: {:?}", paths);
        // Import the `ml` module
        let ml = py.import_bound("ml").map_err(|e| format!("Failed to import ml module: {:?}", e))?;

        // Call the `test_model` function in the `ml` module with the image path
        let result = ml
            .call_method1("test_model", (image_path,))
            .map_err(|e| format!("Failed to call test_model function: {:?}", e))?;

        // Extract the classification result from the Python object as a String
        result.extract::<String>().map_err(|e| format!("Failed to extract result: {:?}", e))
    })
}

#[tokio::main]
async fn main() {
    println!("Hello?");

    pyo3::prepare_freethreaded_python();
    set_python_executable();

    let app = Router::new()
    .route("/", get(handler))
    .route("/upload", post(upload_image))
    .layer(
      tower_http::cors::CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE])
        .allow_methods([axum::http::Method::GET]),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server started, listening on {addr}");

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}


#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

async fn upload_image(mut multipart : Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let data = field.bytes().await.unwrap();
            let img = image::load_from_memory(&data).expect("Failed to load image");

            let img = img.to_rgba8();
            //img.make_ascii_uppercase();

            let temp_filename = format!("./temp_images/{}.png", Uuid::new_v4());
            img.save(&temp_filename).expect("Failed to save temporary image");

            let temp_filename_clone = temp_filename.clone();

            let classification_result = task::spawn_blocking(move || classify_image(&temp_filename_clone))
            .await
            .expect("Failed to execute classification task");
            
            fs::remove_file(&temp_filename).expect("Failed to delete temporary file");
            
            match classification_result {
                Ok(classification_result)  => {
                    println!("Results: {}", classification_result);
                    
                    return Json(Message {
                        message: format!("Image classified as: {}", classification_result),
                    });
                }
                Err(err) => {
                    return Json(Message {
                        message: format!("Error during classification: {}", err),
                    })
                }
            }
        }
    }
    return Json(Message {
        message: format!("File not found"),
    })
}

