Introduction to Apache Arrow with Rust
Andrew Leverette
Andrew Leverette

Follow
8 min read
·
Mar 15, 2021
202

Press enter or click to view image in full size

For the past few months, I’ve been working on an application that is responsible for processing in-memory data. The project is exciting to me for two reasons. One is that the project is written entirely in Rust, and the other is the opportunity to learn about new topics and libraries. This article explores what I have learned while working with Apache Arrow.

When I first started working on the project, I was not aware of Apache Arrow. I just needed a way to aggregate data as efficiently as possible. I even built a proof-of-concept that provided most of the functionality that I needed, including joins. It was a rough draft, but I could already tell that there were some performance issues. One fundamental issue was the performance of aggregation functions on the data structure that represented a data set in a row-based format. This design meant that column-based operations like filtering and mathematical operations were costly. There were a few other concerns, so I decided to do some more research which finally led me to Apache Arrow.

What is Apache Arrow
Apache Arrow is a language-agnostic software development platform used to build applications that process and transport large data sets. Not only did this product provide a column-oriented data format, but it also provided a few other helpful libraries and a developer ecosystem maintained by the Apache Software Foundation.

Memory Format
The core feature of Apache Arrow is its in-memory columnar data format which is a specification for structuring tabular data sets in memory and provides a well-defined type system. This makes the format an ideal building block for projects like database systems or data frame libraries. One major benefit of this memory format is that it excels at processing large chunks of data and enables vectorization using SIMD operations.

Libraries
Other libraries are provided as companions to Apache Arrow. They provide common functionality that you wouldn’t want to implement yourself. Two Rust-specific libraries I found helpful were DataFusion and Arrow Flight. DataFusion is a query engine built on Apache Arrow that provides data frame and SQL query APIs. Arrow Flight is a serialization library that is used for transporting data across a network. I’ll go into more detail about these libraries in later articles.

Developer Ecosystem
One important factor in deciding on Apache Arrow was the developer ecosystem. Apache Arrow is maintained by the Apache Software Foundation which provides a governing body and decision-making process. The foundation also works to maintain a community of open-source developers that is open to everyone.

Rust Arrow Implementation
There are multiple implementations of Apache Arrow, but I will focus on the Rust version. This section will be broken into four parts: low-level arrays, high-level constructs, data readers, and compute kernels. Not every aspect of the library will be covered but just enough to get a good idea of how it can be used.

Low-Level Arrays
The critical component of the Rust implementation is the Array trait. It represents a generic, fix-sized, immutable, thread-safe array of nullable elements. There are two ways to construct an Array: a converter method and a builder method.

The converter method is used to construct an Array instance from a vector of native Rust types. In my opinion, this method is the most common since iterating over a collection of data and collecting that data into a vector is very common. Using the converter method might look something like this:

Press enter or click to view image in full size

A primitive array constructed with a converter
In this snippet, Int32Array is an Array that represents an array of optional 32-bit signed integers. The from method takes a vector of Option<i32>values to construct the array. There are several implementations of the Array trait, so be sure to check the documentation for the correct type of Array.

The builder method is a way to construct an array by appending values to it. This is useful for constructing arrays from values dynamically. Here is an example of using the builder method:

Press enter or click to view image in full size

A primitive array constructed from a builder
In the above example, Int32Builder is used to initialize a builder instance that has a capacity of 100 elements. For the first 50 elements, the generated value is appended to the array if it is even, otherwise, a null is appended — this isn’t an actual null value. For the last 50 elements, the generated values are collected into a vector which is then appended to the array as a slice. The finish method converts the builder instance into an array instance.

High-Level Constructs
Arrays are an incredibly powerful abstraction for working with data. However, they do not provide an easy way to work with tabular data sets. That is where these high-level constructs come into the picture. Three data structures work together with low-level arrays to represent a data set:

Field — Defines the metadata of a single column of data
Schema — Defines the metadata of a data set, contains a vector of Field
RecordBatch — Defines a two-dimensional data set that contains low-level arrays of the same length and a Schema that matches the array types
The RecordBatch type offers a convenient interface for working with a data set and is the most common structure for tasks like computation and serialization. Below is an example of using these types to represent a data set:

Press enter or click to view image in full size

A data set represented by RecordBatch, Schema, Field types
In the above example, the RecordBatch type becomes the top-level component that wraps the schema and the primitive arrays. One thing to note is that RecordBatch consists of atomically reference-counted pointers to the schema and individual primitive arrays. Using this type of reference is an important reason why Apache Arrow can provide zero-copy reads.

Data Readers
Having the ability to construct a data set from the core components of Apache Arrow has limited use cases. There is a definite need to read data into the Apache Arrow memory format without a tremendous amount of effort. In the Rust implementation of Apache Arrow, one of the data readers that are provided is a CSV reader.

The CSV reader reads the records from a CSV file then converts the columns into the appropriate primitive array type based on the schema. The CSV reader can either be used directly or generated from a builder object with custom settings. Using the CSV reader directly could look like this:

Press enter or click to view image in full size

Reading data with a CSV reader
In this example, a schema is defined that represents the data in the CSV file. The CSV reader is created using the opened file, a reference to the schema, and some configuration details like if the file has headers and the batch size. The batch size is an interesting parameter since that determines how many batches of records will be generated. For example, if you have a CSV file that contains 10,000 records, and the batch size is 1000 then the reader will eventually return a collection of 10 RecordBatch instances. Another interesting feature is that the CSV reader is an iterator. This means that the reader can be used in a for loop or any other iterative methods in Rust.

The CSV reader can also be generated from a builder object, which provides a bit more flexibility. Using the builder method could look like this:

Press enter or click to view image in full size

Constructing reader with a reader builder
This example demonstrates using ReaderBuilder to create a builder object that is configured to expect headers in the file and attempt to infer the schema from the first 100 records of the file. The build method will attempt to consume the builder and return a reader with a schema that matches what is in the file.

Compute Kernels
The compute kernels are the workhorses of Apache Arrow. Many kernel functions are provided, but this section will only cover the filter and sort functions in detail.

The filter function takes a reference to an array of data and an array of boolean values that indicate whether elements in that position should be filtered. To filter the arrays in a RecordBatch object, the filter function has to be applied to all the arrays using the same boolean array. For example, filtering the CSV data by the groupcolumn could look something like this:

Press enter or click to view image in full size

An example of using the filter method
In this example, a filter array is created by comparing each element of the groupcolumn to the value of the group parameter. Then the filter array is used to filter every column in the batch. A new RecordBatch instance is created and returned using the original schema and the filtered arrays.

Apache Arrow provides a few functions related to sorting. The relevant one to this example is the sort_to_indices function that takes a reference to an array of data and an optional SortOptions object and returns an array of unsigned integers that represent the new positions of the elements in the array. Here’s an example:

Press enter or click to view image in full size

An example using the sort method
This example builds an array of indices by calling the sort_to_indices function with a reference to the group column and no sort options. Then thetake function is mapped to all the columns to reorder the arrays based on the sorted indices. A new RecordBatch object is created and returned using the same schema and the sorted arrays.

The Apache Arrow library is focused on low-level operations. This means some common data aggregation operations aren’t part of the API. For example, there isn’t a group by function. However, group by functionality can be implemented with filters, sorts, and choice of aggregate functions. Here’s an example:

Press enter or click to view image in full size

An example of implementing a group by
This example first finds the unique group column values by sorting the group column, collecting the values into a vector, and removing duplicate values. A vector of builder arrays is initialized to hold the average score for each score column. For each unique group, the filter_by_group function is used to find all the corresponding score values. The average is calculated for each column in that group and appended to the corresponding builder. The results of the builders are then collected, a new schema is created that represents the new structure, and a RecordBatch instance is returned.

Final Thoughts
There are many other topics to explore, but the main goal was to provide an overview of Apache Arrow and the Rust implementation. My experience with working with Apache Arrow was overwhelmingly positive. The library does not provide the high-level aggregation methods that you might need, but that is not the purpose of this library. It provides the building blocks, but you have to put them together.

Thanks
Thanks for reading! If you would like to connect or want to provide feedback, feel free to contact me on LinkedIn.
