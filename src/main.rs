#[macro_use]
mod logg;
mod manner;
mod storage;

use crate::storage::Storage;
use colored::Colorize;
use cronjob::CronJob;
use manner::Manner;
use once_cell::sync::Lazy;
use rocket::fs::NamedFile;
use rocket::serde::json;
use rocket::{tokio, Config};
use std::env;
use std::error::Error;
use std::net::IpAddr;
use std::path::Path;
use std::sync::Mutex;
use storage::RMS;

static MEM_DB: Lazy<Mutex<RMS>> = Lazy::new(|| Mutex::new(RMS::init()));

const MAILING_ADDRESSES: &str = "mailing_addresses";

#[macro_use]
extern crate rocket;
extern crate cronjob;

fn get<T>(db: &'static Lazy<Mutex<T>>, key: String) -> Option<String>
where
    T: Storage + 'static,
{
    let d = match db.lock() {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };
    d.get(key)
}

fn set<T>(x: &'static Lazy<Mutex<T>>, key: String, value: String) -> Result<(), Box<dyn Error>>
where
    T: Storage + 'static,
{
    x.lock()?.set(key, value)
}

fn init_cron_job() {
    let mut mailing_cron = CronJob::new(MAILING_ADDRESSES, mailing_addresses);

    // mailing_addresses_cron set for every minute
    mailing_cron.seconds("1");
    mailing_cron.minutes("*");
    mailing_cron.hours("*");
    mailing_cron.day_of_month("*");
    mailing_cron.year("*");

    info!("[*] start cron: mailing_cron");
    CronJob::start_job_threaded(mailing_cron);
}

fn mailing_addresses(_: &str) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let port = get_port();
    let server_addresses = match Manner::get_addresses(port) {
        Ok(addresses) => addresses,
        Err(_) => vec![],
    };

    let value = match get(&MEM_DB, MAILING_ADDRESSES.to_string()) {
        Some(value) => value,
        None => {
            warning!("mailing_addresses: no value");
            "".to_string()
        }
    };
    if value == server_addresses.join(",") {
        return;
    }

    set(
        &MEM_DB,
        MAILING_ADDRESSES.to_string(),
        server_addresses.join(","),
    )
    .unwrap();
    rt.block_on(async {
        info!("send email: {}", server_addresses.join(",").green());
        _ = Manner::send_email(server_addresses).await;
    });
}

fn get_address() -> IpAddr {
    return match env::var("IP") {
        Ok(address) => address,
        Err(_) => "0.0.0.0".to_string(),
    }
    .parse()
    .unwrap();
}

fn get_port() -> u16 {
    let raw_port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8000".to_string(),
    };
    let port = raw_port.parse::<u16>().unwrap();
    return port;
}

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
    init_cron_job();

    let mut config = Config::default();
    config.port = get_port();
    config.address = get_address();

    rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .mount("/processes", routes![processes])
        .mount("/kill-processes", routes![kill_processes])
}
