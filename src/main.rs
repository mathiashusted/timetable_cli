// use serde_json::Value;
use std::{io, io::stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::prelude::CrosstermBackend;


mod utils;
mod render;

const TICK_RATE: u64 = 30; // Refresh every x milliseconds



async fn master_loop(data_refresh_rate: i32, url: String) {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();

    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut data_refresh_tick = 0; // Iterator for when to update data

    // Our initial request to fetch the data
    let mut data = utils::utils::make_request(url.to_string()).await;
    let mut timetable = utils::utils::process_data(&data).await;

    let mut should_quit = false;

    while !should_quit {
        if event::poll(std::time::Duration::from_millis(TICK_RATE)).unwrap() {
            should_quit = handle_events().unwrap();
            if data_refresh_tick == data_refresh_rate {
                data = utils::utils::make_request(url.to_string()).await;
                timetable = utils::utils::process_data(&data).await;
                data_refresh_tick = 0;
            }
        }
        render::render::draw(&mut terminal, &timetable);
    }

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn handle_events() -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
            return Ok(true)
        }
    }
    Ok(false)
}


#[tokio::main]
async fn main() -> io::Result<()> {
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
    Ok(())

}
