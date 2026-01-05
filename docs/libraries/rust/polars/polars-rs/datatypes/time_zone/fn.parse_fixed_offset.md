# Function parse_fixed_offset Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/temporal/time_zone.rs.html#175" class="src">Source</a>

``` rust
pub fn parse_fixed_offset(tz: &str) -> Result<PlSmallStr, PolarsError>
```

Expand description

Convert fixed offset to Etc/GMT one from time zone database

E.g. +01:00 -\> Etc/GMT-1

Note: the sign appears reversed, but is correct, see <https://en.wikipedia.org/wiki/Tz_database#Area>:

> In order to conform with the POSIX style, those zone names beginning with “Etc/GMT” have their sign reversed from the standard ISO 8601 convention. In the “Etc” area, zones west of GMT have a positive sign and those east have a negative sign in their name (e.g “Etc/GMT-14” is 14 hours ahead of GMT).
