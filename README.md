# Interval Selector

## Overview
`rw-intervals` is a lightweight JavaScript library written in Rust that provides functions to select overlapping intervals based on their associated weights. It efficiently resolves conflicts between overlapping intervals, ensuring optimal selection.

## Features
- Identify overlapping intervals.
- Select the most relevant intervals based on weight.
- Efficient computation for large datasets.

## Installation

You can install `rw-intervals` via npm:

```sh
npm install rw-intervals
```

or using yarn:

```sh
yarn add rw-intervals
```

## Usage

### Importing the Library


```javascript
import * as RwIntervals from "rw-intervals";
```

### Creating intervals

rw-intervals exposes two interfaces to define schedule and request (interval) objects:

```javascript

export interface Request {
    id: String,
    start_date: Number,
    end_date: Number,
    weight: Number,
  }

  export interface Schedule {
    name: String,
    overlaps: String[],
    requests: Request[],
    reservations: String[],
  }

```
You can create an interval list as follow:

```javascript
const intervals: Request[] = [
    {
      "id": "A",
      "start_date": 28986660,
      "end_date": 28986780,
      "weight": 80
    },
    {
      "id": "B",
      "start_date": 28986751,
      "end_date": 28986870,
      "weight": 40
    },
    {
      "id": "C",
      "start_date": 28947600,
      "end_date": 28948620,
      "weight": 30
    }
];
```

To get the schedule with all selected weighted intervals, you must use the RwIntervals.buildSchedule method sending a request array as String:


```javascript
const intervals: Request[] = [
    {
      "id": "A",
      "start_date": 28986660,
      "end_date": 28986780,
      "weight": 80
    },
    {
      "id": "B",
      "start_date": 28986751,
      "end_date": 28986870,
      "weight": 40
    },
    {
      "id": "C",
      "start_date": 28947600,
      "end_date": 28948620,
      "weight": 30
    }
];

let schedule = RwIntervals.buildSchedule(JSON.stringify(intervals));
```

### Output

```json
{
  "name": "a",
  "requests": [
    {
      "id": "A",
      "start_date": 28986660,
      "end_date": 28986780,
      "weight": 80
    },
    {
      "id": "B",
      "start_date": 28986751,
      "end_date": 28986870,
      "weight": 40
    },
    {
      "id": "C",
      "start_date": 28947600,
      "end_date": 28948620,
      "weight": 30
    }
  ],
  "overlaps": [
    "A:B"
  ],
  "reservations": [
    "A",
    "C"
  ]
}
```
Note that in this output we have an overlaps property that contains a pair of ids separated by a ":". This also contains the reservations array, which represent the winner requests. 

If you want to recalculate the schedule adding a new request, you can use the RwIntervals.recalculateSchedule method:


```javascript
const newInterval: Request = {
      "id": "9fc829f1-2cac-4e00-a318-1ca864667daa",
      "start_date": 28986751,
      "end_date": 28986870,
      "weight": 40
};

let newSchedule = RwIntervals.recalculateSchedule(JSON.stringify(schedule), JSON.stringify(newInterval));
```

## API

### `buildSchedule(intervals: String): Schedule`
- **Parameters:**
  - `intervals` *(Array<Request>)::String*: An array of Request objects.
- **Returns:**
  - Schedule object.

  ### `recalculateSchedule(schedule: String, interval: String): Schedule`
- **Parameters:**
  - `schedule` *(Schedule)::String*: schedule to update.
  - `interval` *(Request)::String*: New request to add into de shcedule.
- **Returns:**
  - Schedule object.

## License

MIT License

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on [GitHub](https://github.com/miguelonCoder/rw-intervals.git).

## Author
Developed by [Miguelon Coder](https://github.com/miguelonCoder).

