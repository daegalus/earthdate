# Earthdate

A tool that takes inspiration from Star Trek Stardates, and implements something useful for today.

This is just a fun tool. I also personally use it for version tagging my releases for non-serious projects or ones that release based on date and not semver.

```sh
> earthdate
24N14.609

> earthdate --month DayOfYear
```

## Options

Creates a date that is based on recent years with variations.

### Year variants, --year (default: C20)

- `C20`: 2000 as the base year (2024 -> 24)
- `C19`: 1900 as the base year (2024 -> 124)
- `UnixEpoch`: 1970 (unix epoch year) as the base year (2024 -> 54)

### Month variants, --month (default: Alpha)

- `Numeric`: Standard 01-12 for months, followed by day of the month (ex: July 24th -> 0724)
- `Alpha`: A single letter variant, followed by day of the month. (ex: July 24th -> L24)
  - J, F, M, A Y, U, L, G, S, O, N, D
- `DayOfYear`: Instead of Month + Day, just the day of the year. (ex: July 24th -> 205 (or 206 leapyear))
- `None`: Empty string, no month in string.

### Time variants, --timev (default: Metric)

- `Standard`: `%H%M` which results in 24 hour time (ex 15:03 -> 1503)
- `Metric`: Number of metric seconds since midnight (1000 metric seconds = 1 day). (ex: 15:13 -> 592)
  - Also known as `Swatch Internet Time` or `Beats`
  - Modified to be UTC, instead of UTC+1
  - Timezone support coming later.
- `FullStandard`: Same as `Standard` but includes seconds.
- `FullMetric`: Same as `Metric` but includes seconds/higher precision. (ex: 15:14:09 -> 593.16)
  - In this case, for formatting, it will be printed as `59316`

## Beats

You can optionally use this tool to just beats/metric seconds returned.

`earthdate beats` -> `593.16`