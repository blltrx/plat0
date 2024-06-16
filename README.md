# Platform Allocation RealTimeTrains Scraper

this is a rust based web scraper for real time trains, focused on actual historic platform allocation. tts original use was to track platform zero calls at Stockport station due to its irregular pattern. 

this is part of my [platform zero](https://roseis.gay/projects/plat0) project.

it can be installed with:
```
cargo install --git https://github.com/blltrx/plat0
```

# usage

```
Usage: plat0-scraper [OPTIONS] <STATION> [DATE] [RANGE]

Arguments:
  <STATION>  NR 3 char station code
  [DATE]     ISO formatted date between year 2001 and 2100
             <year>-<month>-<day> [default: 2024-06-16]
  [RANGE]    Range of days to search starting on DATE [default: 1]

Options:
  -o, --only-platform-zero               Only show calls at platform zero
  -m, --missing-string <MISSING_STRING>  Character for empty entries [default: ~~~~]
  -h, --help                             Print help```
```
