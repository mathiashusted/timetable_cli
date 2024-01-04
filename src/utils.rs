pub mod utils {
    use ratatui::style::Stylize;
    use reqwest;
    use serde::Deserialize;
    use serde_json::Value;
    use std::fs;
    use chrono::prelude::*;
    
    use ratatui::widgets::Row;
    use ratatui::widgets::Cell;
    use ratatui::prelude::Style;
    use ratatui::prelude::Color;

    const CONFIG_PATH: &str = "config.json";

    #[derive(Debug, Deserialize)]
    pub struct ConfigStructure {
        pub source: String,
        pub station_id: i32,
        pub duration: i32,
        pub refresh_rate: u64,
    }


    // This functions reads the contents of the config file and returns it according to the struct
    pub async fn read_config() -> std::io::Result<ConfigStructure> {
        let contents = match fs::read_to_string(CONFIG_PATH) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error finding config.json file: {}", err);
                return Err(err);
            }
        };

        let output: ConfigStructure = match serde_json::from_str(&contents) {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Error reading the data of config.json: {}", err);
                return Err(err.into());
            }
        };

        Ok(output)
    }

    // Our function which actually makes the request, and then returns the parsed JSON keys along with their values
    pub async fn make_request(url: String) -> Value {
        match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(body) => serde_json::from_str(body.as_str()).unwrap(),
                Err(err) => {
                    eprintln!("Error making the request: {}", err);
                    return Value::Null;
                },
            },
            Err(err) => {
                eprintln!("Error making the request: {}", err);
                return Value::Null;
            }
        }
    }

    pub async fn process_tables(data: &Value) -> Vec<Row<'_>> {

        let mut table_rows: Vec<Row<'_>> = vec![];

        let departures = data.get("departures").unwrap();

        if let Some(departures_array) = departures.as_array() {
            for element in departures_array {
                let destination = element.get("direction").unwrap();
                let line = element.get("line")
                    .and_then(|val| val.get("name")).unwrap();
                let departure_time = element.get("when").unwrap();
                let planned_departure_time = element.get("plannedWhen").unwrap();
                if departure_time.is_null() || planned_departure_time.is_null() {
                    table_rows.push(Row::new(vec![line.to_string(), destination.to_string(), "CANCELLED".to_string(), "".to_string()]));
                }
                else {
                    let wait_and_delay = process_delay(departure_time, planned_departure_time);
                    let wait_and_delay_cell = if wait_and_delay.1 == 0 {
                        Cell::from("(=)").style(Style::new().fg(Color::LightGreen))
                    } else if wait_and_delay.1 > 0 {
                        Cell::from(wait_and_delay.1.to_string() + "\"").style(Style::new().fg(Color::Red))
                    } else {
                        Cell::from(wait_and_delay.1.to_string() + "\"").style(Style::new().fg(Color::Yellow))
                    };
                    table_rows.push(Row::new(vec![
                        Cell::from(line.to_string()).style(Style::new().bold()),
                        Cell::from(destination.to_string()),
                        Cell::from(wait_and_delay.0.to_string() + "\""),
                        wait_and_delay_cell
                    ]));
                }
            }
        }
        table_rows
    }

    pub fn process_metadata(data: &Value) -> String {
        let departures = data.get("departures").unwrap();

        if let Some(departures_array) = departures.as_array() {
            let stop_name = departures_array[0].get("stop")
                .and_then(|val| val.get("name")).unwrap();
            return stop_name.to_string(); // Position 0 in array: Name of the stop
        }

        String::from("STATION NAME NOT FOUND") // In case name couldn't be found
    }

    pub fn process_delay(actual_departure: &Value, planned_departure: &Value) -> (i32, i32) {
        let date_format = "%Y-%m-%dT%H:%M:%S%z";
        let parsed_departure_time = NaiveDateTime::parse_from_str(actual_departure.as_str().unwrap(), date_format).unwrap();
        let parsed_time_processed = Local.from_local_datetime(&parsed_departure_time).unwrap();
        let parsed_planned = NaiveDateTime::parse_from_str(planned_departure.as_str().unwrap(), date_format).unwrap();
        let difference = parsed_time_processed - Local::now();
        let delay = parsed_departure_time - parsed_planned;
        (difference.num_minutes() as i32, delay.num_minutes() as i32)
    }

}