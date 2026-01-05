# Constant UNIX_EPOCH_DAY Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#51" class="src">Source</a>

``` rust
pub const UNIX_EPOCH_DAY: i64 = 719_163; // 719_163i64
```

Expand description

Constant from chrono crate

Number of days between Januari 1, 1970 and December 31, 1 BCE which we define to be day 0. 4 full leap year cycles until December 31, 1600 4 \* 146097 = 584388 1 day until January 1, 1601 1 369 years until Januari 1, 1970 369 \* 365 = 134685 of which floor(369 / 4) are leap years floor(369 / 4) = 92 except for 1700, 1800 and 1900 -3 + –––– 719163
