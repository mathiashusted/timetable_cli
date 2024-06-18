use std::{io, io::stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::prelude::CrosstermBackend;
use utils::utils::ConfigStructure;


mod utils;
mod render;

const TICK_RATE: u64 = 1000; // Refresh screen every x milliseconds


//async fn master_loop(data_refresh_rate: u64, station_id: Vec<i32>, lines: Vec<String>, show_cancelled: bool)
async fn master_loop(config: ConfigStructure) {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    if config.station_id.is_empty() {
        eprintln!("station_id has to contain at least one station!");
    }
    let mut current_station = 0;
    let max_station = config.station_id.len() - 1;
    let mut url: String = utils::utils::create_url(&config.source.to_string(), config.station_id[current_station], config.duration);

    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut data_refresh_tick: u64 = 0; // Iterator for when to update data
    // Our initial request to fetch the data
    let mut data = utils::utils::make_request(url.to_string()).await;
    let mut timetable;
    let mut metadata = utils::utils::process_metadata(&data);

    let mut should_quit = false;

    while !should_quit {
        let events = handle_events().unwrap(); // Input handling will time out for TICK_RATE
        if events == 1 {
            should_quit = true;
        }
        else if events == 2 {
            if current_station >= max_station {
                current_station = 0;
            }
            else {
                current_station += 1;
            }
            url = utils::utils::create_url(&config.source.to_string(), config.station_id[current_station], config.duration);
            data_refresh_tick = config.refresh_rate;
            metadata = utils::utils::process_metadata(&data);
        }

        if data_refresh_tick == config.refresh_rate {
            data = utils::utils::make_request(url.to_string()).await;
            data_refresh_tick = 0;
        }
        timetable = utils::utils::process_tables(&data, &config.lines, config.show_cancelled).await;

        data_refresh_tick += 1;
        render::render::draw(&mut terminal, &timetable, &metadata);
    }

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn handle_events() -> io::Result<i8> {
    if event::poll(std::time::Duration::from_millis(TICK_RATE)).unwrap() {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(1)
            }
            else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('n') {
                return Ok(2)
            }
        }
    }
    Ok(0)
}


#[tokio::main]
async fn main() -> io::Result<()> {
    if let Ok(config) = utils::utils::read_config().await {
        tokio::spawn(master_loop(config)).await.unwrap();
    } else {
        eprintln!("Error reading config file!");
    }
    Ok(())

}
