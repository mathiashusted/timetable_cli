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
        pub station_id: Vec<i32>,
        pub duration: i32,
        pub refresh_rate: u64,
        pub lines: Vec<String>,
        pub show_cancelled: bool
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

    pub fn trim_string(to_trim: &str) -> String {
        to_trim[1..to_trim.len()-1].to_string()
    }

    // Our function which actually makes the request, and then returns the parsed JSON keys along with their values
    pub async fn make_request(url: String) -> Value {
        match reqwest::get(url).await {
            Ok(response) => match response.text().await {
                Ok(body) => {
                    match serde_json::from_str(body.as_str()) {
                        Ok(json) => json,
                        Err(err) => {
                            eprintln!("Error loading the contents of the HTTP request: {}", err);
                            Value::Null
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Error Processing the response from the HTTP request: {}", err);
                    return Value::Null;
                },
            },
            Err(err) => {
                eprintln!("Error making the HTTP request: {}", err);
                return Value::Null;
            }
        }
    }

    pub async fn process_tables<'a>(data: &Value, lines: &Vec<String>, show_cancelled: bool) -> Vec<Row<'a>> {

        let mut table_rows: Vec<Row<'_>> = vec![];

        if let Some(departures_array) = data.get("departures").and_then(|val| val.as_array()) {
            for element in departures_array {
                let line_untrimmed = element.get("line")
                    .and_then(|val| val.get("name")).unwrap().to_string();
                let line = trim_string(&line_untrimmed);
                // Start by checking if we have to filter by lines
                if !lines.is_empty() && !lines.contains(&line) {
                    continue;
                }

                let destination_untrimmed = element.get("direction").unwrap().to_string();
                let destination = trim_string(&destination_untrimmed);
                let departure_time = element.get("when").unwrap();
                let planned_departure_time = element.get("plannedWhen").unwrap();
                if departure_time.is_null() || planned_departure_time.is_null() {
                    if show_cancelled {
                        table_rows.push(Row::new(vec![line, destination, "CANCELLED".to_string(), "".to_string()]));
                    }
                    continue;
                }
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
                    Cell::from(destination),
                    Cell::from(wait_and_delay.0.to_string() + "\""),
                    wait_and_delay_cell
                ]));
            }
        } else {
            table_rows.push(Row::new(vec!["".to_string(), "Can't parse the data, attempting again...".to_string()]));
        }
        table_rows
    }

    pub fn loading_screen() -> Vec<Row<'static>> {
        let mut table_rows: Vec<Row<'_>> = vec![];
        table_rows.push(Row::new(vec!["".to_string(), "Loading the next station...".to_string()]));

        table_rows
    }

    pub async fn get_station_name(id: i32, source: &str) -> String {
        let request_string: String = format!(
            "https://{}/stops/{}?linesOfStops=false&language=en",
            source,
            id
        );
        let data = make_request(request_string).await;
        if let Some(body) = data.get("name") {
            return trim_string(&body.to_string());
        }

        String::from("STATION NAME NOT FOUND") // Default behavior in case name couldn't be found
    }

    pub fn process_delay(actual_departure: &Value, planned_departure: &Value) -> (i32, i32) {
        let date_format = "%Y-%m-%dT%H:%M:%S%z";
        let parsed_departure_time = NaiveDateTime::parse_from_str(actual_departure.as_str().unwrap(), date_format).unwrap();
        let parsed_time_processed = Local.from_local_datetime(&parsed_departure_time).unwrap();
        let parsed_planned = NaiveDateTime::parse_from_str(planned_departure.as_str().unwrap(), date_format).unwrap();
        let difference = if (parsed_time_processed - Local::now()).num_minutes() > 0 {
            (parsed_time_processed - Local::now()).num_minutes()
        } else {
            0
        };
        let delay = (parsed_departure_time - parsed_planned).num_minutes();
        (difference as i32, delay as i32)
    }

    pub fn create_url(source: &str, station_id: i32, duration: i32) -> String {
        return format!(
            "https://{}/stops/{}/departures?duration={}&linesOfStops=false&remarks=true&language=en",
            source,
            station_id,
            duration
        );
    }

}