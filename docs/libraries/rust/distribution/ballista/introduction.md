# Overview — Apache DataFusion Ballista  documentation
Ballista is a distributed compute platform primarily implemented in Rust, and powered by Apache DataFusion.

Ballista has a scheduler and an executor process that are standard Rust executables and can be executed directly, but Dockerfiles are provided to build images for use in containerized environments, such as Docker, Docker Compose, and Kubernetes. See the [deployment guide](#deployment.md) for more information

SQL and DataFrame queries can be submitted from Python and Rust, and SQL queries can be submitted via the Arrow Flight SQL JDBC driver, supporting your favorite JDBC compliant tools such as [DataGrip](#datagrip) or [tableau](#tableau). For setup instructions, please see the [FlightSQL guide](#flightsql.md).

How does this compare to Apache Spark?
--------------------------------------------------------------------------------------------------------

Although Ballista is largely inspired by Apache Spark, there are some key differences.

*   The choice of Rust as the main execution language means that memory usage is deterministic and avoids the overhead of GC pauses.
    
*   Ballista is designed from the ground up to use columnar data, enabling a number of efficiencies such as vectorized processing (SIMD and GPU) and efficient compression. Although Spark does have some columnar support, it is still largely row-based today.
    
*   The combination of Rust and Arrow provides excellent memory efficiency and memory usage can be 5x - 10x lower than Apache Spark in some cases, which means that more processing can fit on a single node, reducing the overhead of distributed compute.
    
*   The use of Apache Arrow as the memory model and network protocol means that data can be exchanged between executors in any programming language with minimal serialization overhead.
    

[deployment](#./deployment) [datagrip](https://www.jetbrains.com/datagrip/) [tableau](https://help.tableau.com/current/pro/desktop/en-us/examples_otherdatabases_jdbc.htm)