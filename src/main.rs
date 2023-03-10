use reqwest::Url;
use rocket::fs::NamedFile;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::process::Command;

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Process {
    id: String,
    user: String,
    mem: String,
    fname: String,
    command: String,
}

struct Manner {}

impl Manner {
    pub fn new() -> Self {
        Manner {}
    }

    pub fn get_top_processes(&self) -> Result<Vec<Process>, Box<dyn Error>> {
        let output = match cmd(
            "ps -eo pid,user,%mem,fname,command --sort=-%mem | head -n30 | sed 1d".to_string(),
        ) {
            Ok(output) => output.stdout,
            Err(err) => return Err(err.into()),
        };

        let lines = match std::str::from_utf8(&output) {
            Ok(lines) => lines.split("\n").collect::<Vec<&str>>(),
            Err(err) => return Err(err.into()),
        };

        let mut processes: Vec<Process> = vec![];
        for raw_line in lines {
            let line = raw_line.trim();
            if line == "" {
                continue;
            }

            let mut data = line.split(" ").filter(|i| *i != "").collect::<Vec<&str>>();
            let process = Process {
                id: data.get(0).unwrap_or(&"").to_string(),
                user: data.get(1).unwrap_or(&"").to_string(),
                mem: data.get(2).unwrap_or(&"").to_string(),
                fname: data.get(3).unwrap_or(&"").to_string(),
                command: data.drain(4..data.len()).collect::<Vec<&str>>().join(" "),
            };

            processes.push(process);
        }

        Ok(processes)
    }

    pub fn kill_process(&self, ids: Vec<String>) -> Result<(), Box<dyn Error>> {
        kill_pids(ids)
    }
}

async fn send_email_ips(ips: Vec<String>) {
    let url = Url::parse(&*format!(
        "https://api.val.town/eval/@shanenoi.ip_address?ips={}",
        ips.join(", ")
    ))
    .unwrap();
    _ = reqwest::get(url).await;
}
fn cmd(command: String) -> std::io::Result<std::process::Output> {
    Command::new("sh").arg("-c").arg(command).output()
}

fn getips() -> Vec<String> {
    let content = match cmd("ifconfig \
        | grep 'inet .* netmask' \
        | awk '{print $2}' \
        | grep '[0-9][0-9][0-9]'"
        .to_string())
    {
        Ok(content) => String::from_utf8_lossy(&content.stdout).into_owned(),
        Err(_) => return vec![],
    };
    let port = 3000;
    content
        .split("\n")
        .filter(|i| i.len() != 0)
        .map(|i| format!("{}:{}", i, port))
        .collect()
}

fn kill_pids(pids: Vec<String>) -> Result<(), Box<dyn Error>> {
    if pids.len() > 0 {
        let joined_pids = pids.join(" ");

        let mut kill_cmd = "kill -9 ".to_owned();
        kill_cmd.push_str(&joined_pids);

        _ = cmd(kill_cmd);
    }

    Ok(())
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/index.html")).await.ok()
}

#[get("/")]
fn processes() -> rocket::serde::json::Value {
    let manner = Manner::new();
    let processes = match manner.get_top_processes() {
        Ok(processes) => processes,
        Err(_) => {
            println!("err");
            return rocket::serde::json::Value::String("".to_string());
        }
    };
    json!(processes)
}

#[get("/<uid>")]
fn kill_processes<'a>(uid: &'a str) -> rocket::serde::json::Value {
    let manner = Manner::new();
    _ = manner.kill_process(vec![uid.to_string()]);
    json!("ok")
}

#[launch]
async fn rocket() -> _ {
    send_email_ips(getips()).await;
    rocket::build()
        .mount("/", routes![index])
        .mount("/processes", routes![processes])
        .mount("/kill-processes", routes![kill_processes])
}
