use std::error::Error;
use std::process::{Command, Output};

use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Process {
    id: String,
    user: String,
    mem: String,
    fname: String,
}

pub struct Manner {
    telegram_url: String,
    telegram_chat_id: f64,
}

impl Manner {
    pub fn new() -> Self {
        Self {
            telegram_url: "".to_string(),
            telegram_chat_id: 0 as f64,
        }
    }

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

    pub async fn send_notification(&self, addresses: Vec<String>) -> Result<(), Box<dyn Error>> {
        if addresses.len() == 0 {
            return Ok(());
        }

        self.telegrapher(format!(
            "we found IP address in your network:\n+{}",
            addresses.join("\n+")
        ))
        .await
    }

    pub fn with_telegram_url(mut self, telegram_url: String) -> Self {
        self.telegram_url = telegram_url;
        self
    }

    pub fn with_telegram_chat_id(mut self, telegram_chat_id: f64) -> Self {
        self.telegram_chat_id = telegram_chat_id;
        self
    }

    async fn telegrapher(&self, message: String) -> Result<(), Box<dyn Error>> {
        if self.telegram_url.as_str().len() == 0 || self.telegram_chat_id == 0 as f64 {
            return Err("telegram_url or telegram_chat_id is empty".into());
        }

        let client = reqwest::Client::new();
        _ = client
            .post(self.telegram_url.as_str())
            .body(format!(
                "{{
                \"chat_id\": {},
                \"text\": \"{}\",
            }}",
                self.telegram_chat_id, message
            ))
            .header("Content-Type", "application/json")
            .send()
            .await?;

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
