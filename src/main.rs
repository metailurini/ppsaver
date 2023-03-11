mod ppsaver;

use ppsaver::Manner;
use rocket::fs::NamedFile;
use rocket::serde::json;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/index.html")).await.ok()
}

#[get("/")]
fn processes() -> json::Value {
    let result = match Manner::get_top_processes() {
        Ok(processes) => processes,
        Err(err) => {
            return json::json!(format!("get_top_processes: {}", err));
        }
    };
    json::json!(result)
}

#[get("/<uid>")]
fn kill_processes(uid: &str) -> json::Value {
    _ = Manner::kill_process(vec![uid.to_string()]);
    json::json!("ok")
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let port = 3000;
    let addresses = match Manner::get_addresses(port) {
        Ok(addresses) => addresses,
        Err(_) => vec![],
    };
    _ = Manner::send_email(addresses).await;

    rocket::build()
        .mount("/", routes![index])
        .mount("/processes", routes![processes])
        .mount("/kill-processes", routes![kill_processes])
}
