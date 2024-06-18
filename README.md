# timetable_cli

This app will display departures from a given train/bus station via the transport.rest API. It is entirely CLI based and will
have a UI which can be navigated via the keyboard. For now, it can only fetch and display all departures from the station specified in
the config.json file for a given time period and be quit using the "Q" key. It can now also filter for specific lines in case you wish to specify a specific train/bus line you want to monitor. In that case, add those to the config file. When it's empty it will default to show all lines.