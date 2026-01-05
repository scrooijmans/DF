# IntervalUnit in arrow_schema - Rust

```
pub enum IntervalUnit {
    YearMonth,
    DayTime,
    MonthDayNano,
}
```

Expand description

YEAR_MONTH, DAY_TIME, MONTH_DAY_NANO interval in SQL style.

[§](#variant.YearMonth)

Indicates the number of elapsed whole months, stored as 4-byte integers.

[§](#variant.DayTime)

Indicates the number of elapsed days and milliseconds, stored as 2 contiguous 32-bit integers (days, milliseconds) (8-bytes in total).

[§](#variant.MonthDayNano)

A triple of the number of elapsed months, days, and nanoseconds. The values are stored contiguously in 16 byte blocks. Months and days are encoded as 32 bit integers and nanoseconds is encoded as a 64 bit integer. All integers are signed. Each field is independent (e.g. there is no constraint that nanoseconds have the same sign as days or that the quantity of nanoseconds represents less than a day’s worth of time).

[§](#impl-Freeze-for-IntervalUnit)

[§](#impl-RefUnwindSafe-for-IntervalUnit)

[§](#impl-Send-for-IntervalUnit)

[§](#impl-Sync-for-IntervalUnit)

[§](#impl-Unpin-for-IntervalUnit)

[§](#impl-UnwindSafe-for-IntervalUnit)
