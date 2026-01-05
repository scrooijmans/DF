A Guide to User-Defined Functions in Apache Arrow DataFusion
Dade Aderemi
Dade Aderemi
Software Developer


April 16, 2023
Efficiently processing massive volumes of data is a critical aspect of modern software development, and Apache Arrow has emerged as a powerful tool for achieving this. Apache Arrow is a cross-language development platform for in-memory data that enables high-performance communication between different systems, languages, and processes. With Arrow, developers can easily move data between different applications and perform operations on that data in a fast and efficient manner.

Apache Arrow and Columnar Data Layout
At the heart of Apache Arrow's power is its columnar nature, which enables it to handle large volumes of data with great speed and efficiency. In contrast to traditional row-oriented data storage, which stores data in rows and requires expensive operations to retrieve specific columns, columnar storage stores data in columns, making it easy to access and process specific subsets of data. 

The illustration below shows how records from database tables are typically stored in disk blocks in a row-oriented data storage system

No alt text provided for this image
Fig 1. Row oriented data storage (source AWS)
As can be seen, each row of data is saved in contiguous memory blocks, placed one after the other. This is how data is stored in row-based storage systems.

The next illustration, on the other hand, shows how the same records will be stored in columnar data storage systems.

No alt text provided for this image
Fig 2. Columnar oriented data storage (source AWS)
In the case of columnar data storage, values from each columns are saved in contiguous memory blocks, placed one after the other.

Since Apache Arrow is concerned with how data is laid out in memory, and not how data is stored on disk, its use of columnar orientation dictates how data is saved in memory for processing.

This has several advantages, including better compression, faster query performance, and improved analytics capabilities. By storing data in a columnar format, compression algorithms can be applied more efficiently, resulting in higher compression ratios and reduced storage costs.

Columnar layouts are also more efficient for analytics workloads, as they allow for vectorised processing of data. This means that operations can be applied to entire columns at once, rather than row-by-row, resulting in significant performance improvements. Moreover, modern CPUs are designed to take advantage of SIMD (Single Instruction, Multiple Data) instructions, which are well-suited to vectorised processing of data in columnar formats.

Finally, columnar layouts are more flexible and extensible than row-based layouts, as each column can be stored independently. This makes it easier to add or remove columns from a table, or to modify the data types of existing columns. Additionally, columnar layouts are well-suited to parallel and distributed computing, allowing them to scale to handle large and complex datasets.

DataFusion a Fast, Extensible Query Engine
Apache DataFusion is an open-source library written in Rust for executing queries on Apache Arrow-based data sources. It is a query engine that can execute queries across multiple data sources, including CSV files, Parquet files, and more. Queries can be written using either SQL or a DataFrame API.

No alt text provided for this image
Fig 3. Engine Available by CC0 licence on unsplash. https://unsplash.com/photos/xe-e69j6-Ds
While DataFusion provides a powerful query engine, there may be situations where a specific operation is not directly supported by the built-in SQL functions. In such cases, user-defined functions (UDFs) can be a powerful tool for data engineers and data scientists to extend the functionality of DataFusion and perform customs operations on their data. By defining their UDFs, users can tailor DataFusion's query engine to their specific needs and achieve greater flexibility and control over their data processing pipeline.

In this post, we'll explore how to define and use UDFs in DataFusion to enable customised data processing.

Arrays in Apache Arrow
Before diving into how to create and use UDFs in DataFusion, it's important to have some understanding of the Arrow's Array's since the data that DataFusion operates will be handed over to our UDF's as Apache Arrow's Arrays.

No alt text provided for this image
Fig 4. Arrow Available by CC0 licence on unsplash. https://unsplash.com/photos/I0iaRFzFJZM
Arrow defines a set of primitive and composite data types that are used to represent data in-memory across different systems and programming languages. These category of data types include Scalar, Arrays, ChunkedArrays, RecordBatch, Table, and Dataset. This post will only look at Arrays since that is what we will be operating on, but to learn more about the other data types consult Arrow Columnar Format Specification

Arrays in Apache Arrow are a collection of values of the same data type, represented as a contiguous block of memory. For example, a BooleanArray might represent a sequence of true/false values, while an Int64Array might represent a sequence of 64-bit integers. You can think of Apache's Arrays as the fundamental data structure used to represent columnar data. The Array data types and functionality are implemented in arrow-array crate. DataFusion makes use of this crate and values passed into our UDF will be of Array types.

With the preambles out of the way, we talk next about the functionality we want to implement as UDFs and show the implementation on how to go about it

User Defined Functions for Network, Broadcast and Address Family
To demonstrate how user-defined functions can be implemented, we will use a sample dataset of IP address prefixes that have been announced by different autonomous systems. This dataset contains both IPv4 and IPv6 addresses and is similar to data that can be retrieved from RIPE Stats tool. Our goal will be to write user-defined functions that can extract the network, broadcast, and family addresses from these IP addresses, and use them to perform further analysis and calculations.

The sample CSV we will be using contains the following entries

prefix,announced_by,rir

217.165.64.0/18,AS5384,RIPE

172.110.244.0/24,AS11597,ARIN

142.64.0.0/16,AS5769,ARIN

2001:408::/32,AS14793,ARIN

2001:410:2000::/48,AS10965,ARIN

200.0.85.0/24,AS264102,LACNIC
Set up: Adding Dependencies

To implement our user-defined functions, we will be using the DataFusion and Arrow libraries for data processing, along with the Tokio asynchronous runtime for handling concurrent tasks. Additionally, we will be using the IPNet library to perform IP address calculations. Therefore, in our Rust project, we will need to include the following dependencies in Cargo.toml:

datafusion = "22.0.0"

arrow = "37.0.0"

tokio = {version = "1.25.0", features = ["full"] }

ipnet = "2.7.2"
Implementing the UDFs

We will be implementing 3 UDFs. UFs, for getting the network and broadcast address, the a third one for getting the address family; that is, if an address is IPv4 or IPv6. 

The general structure for implementing UDFs in DataFusion involves:

Defining a rust function that contains the logic of the DFS. The function takes and return `ArrayRef` data types
DataFusion has its own data types (built on top of Arrow). These are values of type of ColumnarValue. For DataFusion to be able to use our function as a UDF, we wrap it in another function that takes care of converting DataFusions values to arrays before calling the function and vice-versa after evaluation.
We create the UDF using our wrapped function
We register the created UDF with DataFusion
Now we can use the UDF in DataFusion

To illustrate the above steps, we start with the defining, and registering the network function.

 In a rust file, preferably for simplicity in main.rs, have the following function defined:

use std::str::FromStr
use std::sync::Arc;


use datafusion::arrow::array::{ArrayRef, StringArray, UInt8Array};
use datafusion::common::DataFusionError;
use datafusion::error::Result;
use ipnet::IpNet;


#[tokio::main]
async fn main() ->  anyhow::Result<()> {

    Ok(())
}

pub fn network(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
      return Err(DataFusionError::Internal(format!(
          "network() was called with {} arguments. It requires 1.",
           args.len()
       )));
    }


   let mut result: Vec<String> = vec![];
   let ip_string = datafusion::common::cast::
                     as_string_array(&args[0])?;
   ip_string.iter().flatten().try_for_each(|ip_string| {
        let network_address = IpNet::from_str(ip_string)
            .map_err(|e| DataFusionError::Internal(
            format!("Parsing {ip_string} failed with error {e}")))?
            .network();
        result.push(network_address.to_string());
        Ok::<(), DataFusionError>(())
    })?;


   Ok(Arc::new(StringArray::from(result)) as ArrayRef)
};
Let us go through the core part of the function. First off, we define the function to take &[ArrayRef] as input and returns Result<ArrayRef>. ArrayRef is a type alias to Arc<dyn Array> that is defined within the arrow-array crate. It is a reference-counted reference to a generic arrow Array.

The first thing we do is to check that our function is being called with one argument. This is optional but good practice.

Then we move to the core of the implementation, which is basically parsing the input value to their IP number representation and returning the network address. 

One important thing we had to do, is to downcast the ArrayRef; a generic Array to a StringArray, which is no longer generic but represents an array where each element is a variable-sized sequence of bytes representing a string. This is needed to be able to parse the input as strings.

The next thing we need to do to complete the implementation is to wrap the function, turn it into a UDF and then register it with DataFusion. To do this, we define another function that helps with this. The code below is extended to do this.

use std::str::FromStr
use std::sync::Arc;


use datafusion::arrow::array::{ArrayRef, StringArray, UInt8Array};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::DataFusionError;
use datafusion::error::Result;
use datafusion::logical_expr::{create_udf, Volatility};
use datafusion::physical_expr::functions::make_scalar_function;
use datafusion::prelude::SessionContext;
use ipnet::IpNet;


#[tokio::main]
async fn main() ->  anyhow::Result<()> {

    Ok(())
}


pub fn network(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
        return Err(DataFusionError::Internal(format!(
         "network() was called with {} arguments. It requires 1.",
            args.len()
        )));
    }

    let mut result: Vec<String> = vec![];
    let ip_string = datafusion::common::cast::
                     as_string_array(&args[0])?;
    ip_string.iter().flatten().try_for_each(|ip_string| {
        let network_address = IpNet::from_str(ip_string)
            .map_err(|e| DataFusionError::Internal(
           format!("Parsing {ip_string} failed with error {e}")))?
            .network();
        result.push(network_address.to_string());
        Ok::<(), DataFusionError>(())
    })?;


    Ok(Arc::new(StringArray::from(result)) as ArrayRef)
}


pub fn register_udfs(ctx: &SessionContext) {
    let network = make_scalar_function(network);
    let network_udf = create_udf(
        "network",
        vec![DataType::Utf8],
        Arc::new(DataType::Utf8),
        Volatility::Immutable,
        network,
    );


    ctx.register_udf(network_udf);
};
Let us go over the newly added function register_udfs.

The function takes a reference to SessionContext which can be considered as the main interface for executing queries with `DataFusion`. The first thing the function does is to decorate the network function by wrapping it with the functionality that converts inputs from DataFusion's data type to Arrow Arrays that it expects. This is done via the make_scalar_function function.

The next is to create the actual UDF using the helper function create_udf. First argument. `network` is the name we use in registering the UDF and how it will be called. The second and third arguments are the input and output data types of the UDFs. The fourth argument is a bit low-level and refers to the function's eligibility for certain optimisations by DataFusion. We set it to Volatility::Immutable meaning our UDF will always return the same output given the same input. And the fourth argument is the actual function we are turning into a UDF. 

Next, we look at how to use the UDF we have defined. To do this, we write a test.

Using the network UDF

We have defined a network function that contains the core implementation of our UDF. We also define a register_udfs that takes care of turning our function into an actual UDF and registering it to a DataFusion SessionContext.

Next, we put all of these together and use our UDF in a test as shown below:

#[cfg(test)
mod tests {
    use datafusion::error::Result;
    use datafusion::prelude::{CsvReadOptions, SessionContext};

    use super::*;

    #[tokio::test]
    pub async fn run_udfs() -> Result<()> {
        let ctx = SessionContext::new();
        register_udfs(&ctx);

        ctx.register_csv("data", 
              "./data/ip_data.csv", 
                CsvReadOptions::default()
         ).await?;

        let df = ctx.sql(r#"
            select prefix, network(prefix) as network from data
                        "#).await?;
        df.show().await?;

        Ok(())
    }
}]
In the test, we create a DataFusion SessionContext, and we then pass it to the register_udfs where the network UDF is registered. Then we move on to using the UDF. In other to do that, we register the sample csv located at ./data/ip_data.csv with the SessionContext. Then we execute an SQL query where we use the network UDF. We then call the show() method on the DataFrame returned which prints the result to the console.

Running the test will print the following to the console:

+--------------------+-----------------
| prefix             | network         |
+--------------------+-----------------+
| 217.165.64.0/18    | 217.165.64.0    |
| 172.110.244.0/24   | 172.110.244.0   |
| 142.64.0.0/16      | 142.64.0.0      |
| 2001:408::/32      | 2001:408::      |
| 2001:410:2000::/48 | 2001:410:2000:: |
| 200.0.85.0/24      | 200.0.85.0      |
+--------------------+-----------------++
The above shows the general procedure for creating, registering, and using UDF in DataFusion. We now extend the code with implementations for the UDFs to get the broadcast address and address family. The full code including the test is reproduced below:

use std::str::FromStr
use std::sync::Arc;


use datafusion::arrow::array::{ArrayRef, StringArray, UInt8Array};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::DataFusionError;
use datafusion::error::Result;
use datafusion::logical_expr::{create_udf, Volatility};
use datafusion::physical_expr::functions::make_scalar_function;
use datafusion::prelude::SessionContext;
use ipnet::IpNet;


#[tokio::main]
async fn main() ->  anyhow::Result<()> {
    // Empty. Implementation is used within tests below
    Ok(())
}


// define the logic for the network UDF
pub fn network(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
    return Err(DataFusionError::Internal(format!(
        "network() was called with {} arguments. It requires 1.",
        args.len()
     )));
    }


    let mut result: Vec<String> = vec![];
    let ip_string = datafusion::common::cast::as_string_array(
                         &args[0]
                     )?;
    ip_string.iter().flatten().try_for_each(|ip_string| {
        let network_address = IpNet::from_str(ip_string)
          .map_err(|e| DataFusionError::Internal(
            format!("Parsing {ip_string} failed with error {e}"))
            )?
          .network();
        result.push(network_address.to_string());
        Ok::<(), DataFusionError>(())
    })?;


    Ok(Arc::new(StringArray::from(result)) as ArrayRef)
}


// define the logic for the broadcast UDF
pub fn broadcast(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
      return Err(DataFusionError::Internal(format!(
          "network() was called with {} arguments. It requires 1.",
          args.len()
      )));
    }


    let mut result: Vec<String> = vec![];
    let ip_string = datafusion::common::cast::as_string_array(
                              &args[0]
                     )?;
   ip_string.iter().flatten().try_for_each(|ip_string| {
       let broadcast_address = IpNet::from_str(ip_string)
          .map_err(|e| DataFusionError::Internal(
             format!("Parsing {ip_string} failed with error {e}"))
           )?
           .broadcast();
        result.push(broadcast_address.to_string());
        Ok::<(), DataFusionError>(())
    })?;


    Ok(Arc::new(StringArray::from(result)) as ArrayRef)
}


// define the logic for the family UDF
pub fn family(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
        return Err(DataFusionError::Internal(format!(
           "network() was called with {} arguments. It requires 1.",
            args.len()
        )));
    }


    let mut result: Vec<u8> = vec![];
    let ip_string = datafusion::common::cast::as_string_array(
                        &args[0]
                     )?;
    ip_string.iter().flatten().try_for_each(|ip_string| {
    let family = if IpNet::from_str(ip_string)
        .map_err(|e| DataFusionError::Internal(
          format!(
            "family fn parsing {ip_string} failed with error {e}"
          ))
        )?
        .network()
        .is_ipv4() {
            4
        } else {
            6
        };
        result.push(family);
        Ok::<(), DataFusionError>(())
    })?;

    Ok(Arc::new(
     result.into_iter().collect::<UInt8Array>()
     ) as ArrayRef)
}


// Function to register the UDF with SessionContext
pub fn register_udfs(ctx: &SessionContext) {
    let network = make_scalar_function(network);
    let broadcast = make_scalar_function(broadcast);
    let family = make_scalar_function(family);


    let network_udf = create_udf(
        "network",
        vec![DataType::Utf8],
        Arc::new(DataType::Utf8),
        Volatility::Immutable,
        network,
    );


    let broadcast_udf = create_udf(
        "broadcast",
        vec![DataType::Utf8],
        Arc::new(DataType::Utf8),
        Volatility::Immutable,
        broadcast,
    );


    let family_udf = create_udf(
        "family",
        vec![DataType::Utf8],
        Arc::new(DataType::UInt8),
        Volatility::Immutable,
        family,
    );


    ctx.register_udf(network_udf);
    ctx.register_udf(broadcast_udf);
    ctx.register_udf(family_udf);
}




#[cfg(test)]
mod tests {
    use datafusion::error::Result;
    use datafusion::prelude::{CsvReadOptions, SessionContext};


    use super::*;


    #[tokio::test]
    pub async fn run_udfs() -> Result<()> {
        let ctx = SessionContext::new();
        register_udfs(&ctx);


      ctx.register_csv("data", 
                      "./data/ip.csv", 
                       CsvReadOptions::default()
                       ).await?;

      let df = ctx.sql(r#"
                    select prefix,
                           network(prefix) as network,
                           broadcast(prefix) as broadcast,
                           family(prefix) as host_count
                    from data"#).await?;
        df.show().await?;


        Ok(())
    }
};
And running the test should print the following output below, indicating that the UDFs works

+--------------------+-----------------+----------------------------------------+------------+
| prefix             | network         | broadcast                              | host_count |
+--------------------+-----------------+----------------------------------------+------------+
| 217.165.64.0/18    | 217.165.64.0    | 217.165.127.255                        | 4          |
| 172.110.244.0/24   | 172.110.244.0   | 172.110.244.255                        | 4          |
| 142.64.0.0/16      | 142.64.0.0      | 142.64.255.255                         | 4          |
| 2001:408::/32      | 2001:408::      | 2001:408:ffff:ffff:ffff:ffff:ffff:ffff | 6          |
| 2001:410:2000::/48 | 2001:410:2000:: | 2001:410:2000:ffff:ffff:ffff:ffff:ffff | 6          |
| 200.0.85.0/24      | 200.0.85.0      | 200.0.85.255                           | 4          |
+--------------------+-----------------+----------------------------------------+------------++
Conclusion
In this blog post, we explored the power of user-defined functions in DataFusion. We explained the columnar data layout and its advantages over row-based layouts, such as better cache utilisation and improved vectorisation through SIMD. We also provided a brief overview of the Array data type in Arrow, which serve as the foundational data type for DataFusion.

We then saw how User-defined functions (UDFs) in DataFusion are a mechanism for extending the functionality of SQL queries beyond what is provided by default. To demonstrate this we implemented UDFs for getting the network, broadcast address from IP notation, and whether the IP number is IPv4 or IPv6.

