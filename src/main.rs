use std::borrow::Cow;
use std::sync::Mutex;

use crate::storage::{Storage, RMS};
use manner::Manner;

use once_cell::sync::Lazy;
use rocket::response::content::RawHtml;
use rocket::serde::json;
use rocket::Config;
use rust_embed::RustEmbed;

#[macro_use]
mod logg;
mod cron;
mod envs;
mod manner;
mod storage;

#[macro_use]
extern crate rocket;

static MEM_DB: Lazy<Mutex<RMS>> = Lazy::new(|| Mutex::new(RMS::init()));

#[derive(RustEmbed)]
#[folder = "static"]
pub(crate) struct Asset;

#[get("/")]
async fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
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
    cron::init_cron_job(&MEM_DB);

    let mut config = Config::default();
    config.port = envs::get_port();
    config.address = envs::get_address();

    rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .mount("/processes", routes![processes])
        .mount("/kill-processes", routes![kill_processes])
}
