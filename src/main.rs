use serde_json::Value;
use tui::{backend::TermionBackend, Terminal};
use std::{io::{stdin, Read, stdout, Write}, time::Duration};
use termion::{async_stdin, raw::{IntoRawMode, RawTerminal}};


mod utils;
mod render;

const TICK_RATE: u64 = 50; // Refresh every x milliseconds




async fn master_loop(refresh_rate: i32, url: String) {

    let stdout = std::io::stdout().into_raw_mode().unwrap();

    let backend = TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend).unwrap();

    terminal.clear().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut data_refresh_tick = 0; // Iterator for when to update data

    // Our initial request to fetch the data
    let mut data = utils::utils::make_request(url.to_string()).await;
    let mut timetable = utils::utils::process_data(&data).await;

    loop {
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }

        // Check if it's time to update our data
        if data_refresh_tick == refresh_rate {
            data = utils::utils::make_request(url.to_string()).await;
            timetable = utils::utils::process_data(&data).await;
            data_refresh_tick = 0;
        }
        // If we aren't given any different instructions, draw our frame
        render::render::draw(&mut terminal, &timetable).await;

        data_refresh_tick += 1;
        // Wait until tick rate has passed
        std::thread::sleep(Duration::from_millis(TICK_RATE));
    }
}

#[tokio::main]
async fn main() {
    println!("Starting...");

    if let Ok(config) = utils::utils::read_config().await {
        let url = format!(
            "https://{}/stops/{}/departures?duration={}&linesOfStops=false&remarks=true&language=en",
            config.source,
            config.station_id,
            config.duration
        );

        tokio::spawn(master_loop(config.refresh_rate, url)).await.unwrap();
    } else {
        eprintln!("Error reading config file!");
    }


}
