use std::env;
use std::net::IpAddr;

pub fn get_address() -> IpAddr {
    return match env::var("IP") {
        Ok(address) => address,
        Err(_) => "0.0.0.0".to_string(),
    }
    .parse()
    .unwrap();
}

pub fn get_port() -> u16 {
    let raw_port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8000".to_string(),
    };
    let port = raw_port.parse::<u16>().unwrap();
    return port;
}

pub fn get_telegram_url() -> String {
    match env::var("TELEGRAM_URL") {
        Ok(telegram_url) => telegram_url,
        Err(_) => "".to_string(),
    }
}

pub fn get_telegram_chat_id() -> f64 {
    let telegram_chat_id = match env::var("TELEGRAM_CHAT_ID") {
        Ok(telegram_chat_id) => telegram_chat_id,
        Err(_) => "0".to_string(),
    };
    let chat_id = telegram_chat_id.parse::<f64>().unwrap();
    return chat_id;
}
