use std::sync::Mutex;

use crate::manner::Manner;
use crate::storage::Storage;
use crate::{envs, storage};

use colored::Colorize;
use cronjob::CronJob;
use once_cell::sync::Lazy;
use rocket::tokio;

const NOTIFY_ADDRESSES: &str = "notify_addresses";

fn notify(addresses: Vec<String>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        info!("send notification: {}", addresses.join(",").green());
        _ = Manner::new()
            .with_telegram_url(envs::get_telegram_url())
            .with_telegram_chat_id(envs::get_telegram_chat_id())
            .send_notification(addresses)
            .await;
    });
}

fn notify_addresses<T>(db: &'static Lazy<Mutex<T>>)
where
    T: Storage + 'static,
{
    let port = envs::get_port();
    let ip_addresses = match Manner::get_addresses(port) {
        Ok(addresses) => addresses,
        Err(_) => vec![],
    };

    let addresses = match storage::get(db, NOTIFY_ADDRESSES.to_string()) {
        Some(value) => value,
        None => "".to_string(),
    };

    if addresses == ip_addresses.join(",") {
        // if this is the same as the last time, do not send a notification
        return;
    }

    storage::set(db, NOTIFY_ADDRESSES.to_string(), ip_addresses.join(",")).unwrap();
    notify(ip_addresses);
}

pub fn init_cron_job<T>(db: &'static Lazy<Mutex<T>>)
where
    T: Storage + 'static + Send + Sync,
{
    let mut cron_notify_addresses = CronJob::new(NOTIFY_ADDRESSES, move |_: &str| {
        info!("[*] start cron: cron::notify_addresses");
        notify_addresses(db);
    });
    // notification_cron set for every minute
    cron_notify_addresses.seconds("1");
    cron_notify_addresses.minutes("*");
    cron_notify_addresses.hours("*");
    cron_notify_addresses.day_of_month("*");
    cron_notify_addresses.year("*");

    CronJob::start_job_threaded(cron_notify_addresses);
}
