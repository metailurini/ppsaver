use reqwest::Url;
use rocket::serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::{Command, Output};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Process {
    id: String,
    user: String,
    mem: String,
    fname: String,
}

pub(crate) struct Manner {}

impl Manner {
    pub fn get_top_processes() -> Result<Vec<Process>, Box<dyn Error>> {
        let output = match Self::cmd(
            "ps -eo pid,user,%mem,cmd --sort=-%mem \
            | head -n30 \
            | sed 1d"
                .to_string(),
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

            let data = line.split(" ").filter(|i| *i != "").collect::<Vec<&str>>();
            let process = Process {
                id: data.get(0).unwrap_or(&"").to_string(),
                user: data.get(1).unwrap_or(&"").to_string(),
                mem: data.get(2).unwrap_or(&"").to_string(),
                fname: data.get(3).unwrap_or(&"").to_string(),
            };

            processes.push(process);
        }

        Ok(processes)
    }

    pub fn kill_process(ids: Vec<String>) -> Result<(), Box<dyn Error>> {
        Self::kill_pids(ids)
    }

    pub fn get_addresses(port: u16) -> Result<Vec<String>, Box<dyn Error>> {
        let content = match Self::cmd(
            "ifconfig \
        | grep 'inet .* netmask' \
        | awk '{print $2}' \
        | grep '[0-9][0-9][0-9]'"
                .to_string(),
        ) {
            Ok(content) => String::from_utf8_lossy(&content.stdout).into_owned(),
            Err(err) => return Err(err.into()),
        };

        Ok(content
            .split("\n")
            .filter(|line| line.len() != 0)
            .map(|line| format!("{}:{}", line, port))
            .collect())
    }

    pub async fn send_email(addresses: Vec<String>) -> Result<(), Box<dyn Error>> {
        if addresses.len() == 0 {
            return Ok(());
        }

        let url = Url::parse(&*format!(
            "https://api.val.town/eval/@shanenoi.ip_address?ips={}",
            addresses.join(", ")
        ))?;
        _ = reqwest::get(url).await;
        Ok(())
    }

    fn kill_pids(pids: Vec<String>) -> Result<(), Box<dyn Error>> {
        if pids.len() > 0 {
            let joined_pids = pids.join(" ");

            let mut kill_cmd = "kill -9 ".to_owned();
            kill_cmd.push_str(&joined_pids);
            _ = Self::cmd(kill_cmd);
        }
        Ok(())
    }

    fn cmd(command: String) -> std::io::Result<Output> {
        Command::new("sh").arg("-c").arg(command).output()
    }
}
