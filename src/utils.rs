pub mod utils {
    use reqwest;
    use serde::Deserialize;
    use serde_json::Value;
    use std::fs;
    
    use tui::widgets::Row;

    const CONFIG_PATH: &str = "config.json";

    #[derive(Debug, Deserialize)]
    pub struct ConfigStructure {
        pub source: String,
        pub station_id: i32,
        pub duration: i32,
        pub refresh_rate: i32,
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

    pub async fn process_data(data: &Value) -> Vec<Row<'_>> {

        let mut table_rows: Vec<Row<'_>> = vec![];

        let departures = data.get("departures").unwrap();

        if let Some(departures_array) = departures.as_array() {
            for element in departures_array {
                let destination = element.get("direction").unwrap();
                let line = element.get("line")
                    .and_then(|val| val.get("name")).unwrap();
                let departure_time = element.get("when").unwrap();
                table_rows.push(Row::new(vec![line.to_string(), destination.to_string(), departure_time.to_string()]));
            }
        }


        
        table_rows
    }
}