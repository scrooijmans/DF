# ADBC: Arrow Database Connectivity — Apache Arrow v21.0.0
Full Documentation on ADBC can be found at [https://arrow.apache.org/adbc/](https://arrow.apache.org/adbc/).

ADBC is:

*   A set of abstract APIs in different languages (C/C++, Go, and Java, with more on the way) for working with databases and Arrow data.
    
    For example, result sets of queries in ADBC are all returned as streams of Arrow data, not row-by-row.
    
*   A set of implementations of that API in different languages (C/C++, C#/.NET, Go, Java, Python, and Ruby) that target different databases (e.g. PostgreSQL, SQLite, any database supporting Flight SQL).
    

See the [ADBC Specification](https://arrow.apache.org/adbc/current/format/specification.html "(in ADBC v19)") for details.

The ADBC specification is currently at version 1.1.0.

Updating this specification[#](#updating-this-specification "Permalink to this heading")
----------------------------------------------------------------------------------------

ADBC is versioned separately from the core Arrow project. The API standard and components (driver manager, drivers) are also versioned separately, but both follow semantic versioning.

For example: components may make backwards-compatible releases as 1.0.0, 1.0.1, 1.1.0, 1.2.0, etc. They may release backwards-incompatible versions such as 2.0.0, but which still implement the API standard version 1.0.0.

Similarly, this documentation describes the ADBC API standard version 1.1.0. If/when an ABI-compatible revision is made (e.g. new standard options are defined), the next version would be 1.2.0. If incompatible changes are made (e.g. new API functions), the next version would be 2.0.0.