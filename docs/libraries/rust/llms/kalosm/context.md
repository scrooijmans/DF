Title: Kalosm

Description: Floneum is a graph editor for local LLM workflows. 

Text Embeddings
===============

Embeddings are a numerical representation of the meaning of some data. Text embeddings represent something about the meaning of some text. Embeddings can be used either directly or as input to machine learning models. In this chapter, we will learn how to use Kalosm to create text embeddings and integrate them into a database. We will also learn how to search the embedding database for documents that are similar to a given query.

Creating an Embedding model
---------------------------

First, we need to create an embedding model. An embedding model is a machine learning model that can be used to create embeddings. Kalosm provides a `Bert` struct that can be used to create an embedding model.

use kalosm::language::\*;
let bert \= Bert::new().await?;

> Bonus: Download progress
> 
> If you need to update progress while you are downloading the model, you can use the bert builder with the `build_with_loading_handler` method.
> 
> let bert \= Bert::builder()
>     .build\_with\_loading\_handler(|progress| match &progress {
>         ModelLoadingProgress::Downloading {            source,            progress: file\_loading\_progress,        } \=> {
>             let elapsed \= file\_loading\_progress.start\_time.elapsed().as\_secs\_f32();
>             let progress \= (progress.progress() \* 100.0) as u32;
>             println!("Downloading file {source} {progress}% ({elapsed}s)");
>         }        ModelLoadingProgress::Loading { progress } \=> {
>             let progress \= (progress \* 100.0) as u32;
>             println!("Loading model {progress}%");
>         }    })    .await?;

Creating Embeddings
-------------------

Once we have created an embedding model, we can use it to create embeddings. Kalosm provides a `Bert` struct that can be used to create embeddings.

let text \= "Hello, world!";
let embeddings \= bert.embed(text).await?;
println!("{:?}", embeddings);

Try different values for the text we are embedding. How does the embedding change?

Creating an Embedding Database
------------------------------

Now that we know how to create embeddings, we can use them to create an embedding database. An embedding database is a data structure that stores embeddings and allows you to search for documents that are similar to a given query. Kalosm provides a `DocumentTable` struct that can be used to create an embedding database linked to a table in a Surrealdb database. You can choose a chunk strategy to use when creating the embedding database. A chunk strategy determines how documents are split into chunks before being embedded. In this example, we will use the `Sentence` chunk strategy, which splits documents into sentences before embedding them. The bert embedding model tends to work best with single sentence chunks.

use kalosm::language::\*;
// Create database connection
let db \=
surrealdb::Surreal::new::<surrealdb::engine::local::SurrealKv>("./db/temp.db").await?;
// Select a specific namespace / database
db.use\_ns("test").use\_db("test").await?;
// Create a document table
let document\_table \= db    .document\_table\_builder("documents")
// Store the embedding database in the ./db/embeddings.db file
.at("./db/embeddings.db")
.build()
.await?;

### Adding Documents to the Embedding Database

Once you have created an embedding database, you can add documents to it with the `extend` method. The `extend` method takes something that can be turned into documents and adds them to the embedding database. In this example, we will add documents from a RSS feed to the embedding database.

let nyt \= RssFeed::new(Url::parse(
"https://rss.nytimes.com/services/xml/rss/nyt/US.xml",
)?);
// Fetch the documents from the feed
let documents \= nyt.into\_documents().await?;
// And insert them into the database
for document in documents {    document\_table.insert(document).await?;
}

> This example uses rss context, but you can also use audio, filesystem, or search engine context You can also use a fuzzy search engine with the same api if you prefer traditional search

### Searching the Embedding Database

Next, you can use search through the documents you embedded with the `search` method. The `search` method takes a query and returns a list of documents that are similar to the query. The `search` method also takes a `limit` parameter that determines how many documents to return.

loop {
let user\_question \= prompt\_input("Query: ")?;
let user\_question\_embedding \= document\_table        .embedding\_model()
.embed(&user\_question)
.await?;
println!(        "vector: {:?}",
document\_table            .search(user\_question\_embedding)
.with\_results(5)
.await?
);
}

Conclusion
----------

In this chapter, we learned how to use Kalosm to create embeddings and integrate them into a database. We also learned how to search the embedding database for similar documents.