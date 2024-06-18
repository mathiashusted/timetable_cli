# timetable_cli

This app will display departures from a given train/bus station via the transport.rest API. It is entirely CLI based and will
have a UI which can be navigated via the keyboard. For now, it can fetch and display all departures from the stations specified in
the config.json file for a given time period. It can now also filter for specific lines in case you wish to specify a specific train/bus line you want to monitor. In that case, add those to the config file. When it's empty it will default to show all lines.

### How to use

Specify all of the stations in the config.json file that you want to view the departures from. To find the station IDs, please refer to the [transport.rest playground](https://petstore.swagger.io/?url=https%3A%2F%2Fv6.bvg.transport.rest%2F.well-known%2Fservice-desc%0A#/default/get_locations) for now (search function will be implemented later). To start the program, simply execute it via the terminal and use the keyboard shortcuts to navigate:
1. **n** cycles through the stations
2. **q** quits the program
