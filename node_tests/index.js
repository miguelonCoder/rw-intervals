import * as RwIntervals from '../index.mjs'

var requests = [
  {
    "id": "I",
    "start_date": 13,
    "end_date": 14,
    "weight": 22
    },
    {
    "id": "H",
    "start_date": 7,
    "end_date": 14,
    "weight": 20
    },
    {
    "id": "G",
    "start_date": 10,
    "end_date": 12,
    "weight": 18
    },
    {
    "id": "F",
    "start_date": 8,
    "end_date": 12,
    "weight": 15
    },
    {
    "id": "D",
    "start_date": 12,
    "end_date": 14,
    "weight": 12
    },
    {
    "id": "C",
    "start_date": 10,
    "end_date": 13,
    "weight": 11
    },
    {
    "id": "A",
    "start_date": 9,
    "end_date": 12,
    "weight": 10
    },
    {
    "id": "B",
    "start_date": 8,
    "end_date": 10,
    "weight": 9
    },
    {
    "id": "E",
    "start_date": 7,
    "end_date": 8,
    "weight": 8
    }
]

//console.log(RwIntervals)

var schedule = RwIntervals.buildSchedule(JSON.stringify(requests));

console.log(schedule)