#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize, Deserialize};
use std::fs;
use tempfile::NamedTempFile;

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
    language: String,
    input: String,
}

#[derive(Serialize)]
struct CompileResponse {
    stdout: String,
    stderr: String,
}

#[post("/compile", format = "json", data = "<request>")]
async fn compile(request: Json<CompileRequest>) -> Json<CompileResponse> {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    fs::write(file_path, &request.code).unwrap();

    // Here you would normally compile or process the code
    // For demo purposes, just returning a dummy response
    Json(CompileResponse {
        stdout: "Compiled successfully".to_string(),
        stderr: "".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![compile])
}
