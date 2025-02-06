# timetable_cli

This app shows the user departures from a given train/bus station along with their delays utilizing the transport.rest API. It is entirely CLI based and will have a UI which can be navigated via the keyboard. It can display all departures from the stations specified for a given time period. It can now also filter for specific lines in case you are only interested in certain ones. Currently in early alpha.

![image](https://github.com/user-attachments/assets/9d36b947-4d0c-4b7d-a79f-8d9860c748eb)

### How to use

Specify all of the stations in the config.json file that you want to view the departures from. To start the program, simply execute it via the terminal and use the keyboard shortcuts to navigate:
1. **n** cycles through the stations
2. **q** quits the program

### config.json

The config.json file contains all of the user adjustable parameters. It should be set as follows to achieve the desired behavior:
- **source:** This is the data source from the transport.rest API. By default it uses Berlin's public transportation service (BVG). An alternative source should be inserted following the default format (without https://...). At this time, the focus is on supporting only BVG and DB (German national rail operator).
- **station_id:** This is an *array* of the station IDs to be shown within the program, with the first one being the default. To find the station ID for your station, please refer to the [transport.rest playground](https://petstore.swagger.io/?url=https%3A%2F%2Fv6.bvg.transport.rest%2F.well-known%2Fservice-desc%0A#/default/get_locations) for now (a search function will be implemented later).
- **duration:** This is the time limit to the departures loaded in minutes. E.g.: If set to 60, the departures for the next 60 minutes will be loaded. A higher number means a bigger amount of data will be fetched
- **refresh_rate:** Interval for data refresh in seconds. When set to 15, the most recent data will be downloaded every 15 seconds.
- **lines:** An array of the lines to filter for. Unless it is empty, *only* the lines specified in this array will be shown in the time table. For example, if you are only interested in bus lines 26 and 61 at your station, you would fill it is ["26", "61"].
- **show_cancelled:** Boolean that determines whether cancelled departures will show up in the time table or simply be filtered out.
