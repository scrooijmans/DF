# Function string_to_datetime Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#176" class="src">Source</a>

``` rust
pub fn string_to_datetime<T>(
    timezone: &T,
    s: &str,
) -> Result<DateTime<T>, ArrowError>where
    T: TimeZone,
```

Expand description

Accepts a string and parses it relative to the provided `timezone`

In addition to RFC3339 / ISO8601 standard timestamps, it also accepts strings that use a space ` ` to separate the date and time as well as strings that have no explicit timezone offset.

Examples of accepted inputs:

- `1997-01-31T09:26:56.123Z` \# RCF3339
- `1997-01-31T09:26:56.123-05:00` \# RCF3339
- `1997-01-31 09:26:56.123-05:00` \# close to RCF3339 but with a space rather than T
- `2023-01-01 04:05:06.789 -08` \# close to RCF3339, no fractional seconds or time separator
- `1997-01-31T09:26:56.123` \# close to RCF3339 but no timezone offset specified
- `1997-01-31 09:26:56.123` \# close to RCF3339 but uses a space and no timezone offset
- `1997-01-31 09:26:56` \# close to RCF3339, no fractional seconds
- `1997-01-31 092656` \# close to RCF3339, no fractional seconds
- `1997-01-31 092656+04:00` \# close to RCF3339, no fractional seconds or time separator
- `1997-01-31` \# close to RCF3339, only date no time

[IANA timezones](https://www.iana.org/time-zones) are only supported if the `arrow-array/chrono-tz` feature is enabled

- `2023-01-01 040506 America/Los_Angeles`

If a timestamp is ambiguous, for example as a result of daylight-savings time, an error will be returned

Some formats supported by PostgresSql <https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-DATETIME-TIME-TABLE> are not supported, like

- “2023-01-01 04:05:06.789 +07:30:00”,
- “2023-01-01 040506 +07:30:00”,
- “2023-01-01 04:05:06.789 PST”,
