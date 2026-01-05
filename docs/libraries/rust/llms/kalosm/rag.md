Title: Kalosm

Description: Floneum is a graph editor for local LLM workflows. 

Retrieval-augmented generation in Kalosm
========================================

Introduction
------------

Retrieval-augmented generation in Kalosm is a powerful approach that combines natural language generation with real-time information from audio data. This guide will walk you through the process of implementing retrieval-augmented generation using the Kalosm library.

> Before you begin, make sure you have the Kalosm library installed and set up in your Rust project. You can refer to the Introduction for instructions on how to install Kalosm.

Creating a Transcription Model
------------------------------

First we need to create a transcription model. Kalosm provides a `Whisper` struct that serves as a transcription model. You can initialize it as follows:

use kalosm::language::\*;
use kalosm::sound::\*;
use kalosm::\*;
use std::sync::Arc;
use tokio::time::{Duration, Instant};
let model \= WhisperBuilder::default()
.with\_source(WhisperSource::MediumEn)
.build()
.await?;

This sets up a transcription model using the Whisper source for English ( `WhisperSource::MediumEn`). Adjust the source according to your language preferences.

Creating a Context Database
---------------------------

The next step is to create a context database. Kalosm provides `DocumentTable` struct that indexes a Surrealdb database table with a vector database which can serve as our context database. We need to wrap the database in a `Arc` so that it can be shared across threads:

// Create database connection
let db \= surrealdb::Surreal::new::<surrealdb::engine::local::SurrealKv>("./db/temp.db").await?;
// Select a specific namespace / database
db.use\_ns("test").use\_db("test").await?;
// Create a new document database table
let document\_table \= Arc::new(
db.document\_table\_builder("documents")
// Store the embedding database at ./db/embeddings.db
.at("./db/embeddings.db")
.build()
.await?,
);

Recording Audio
---------------

Next, we need to record audio data. Kalosm provides a `MicInput` struct that can be used to record audio data.

{
std::thread::spawn(move || {
tokio::runtime::Runtime::new()            .unwrap()
.block\_on(async move {
let recording\_time \= Duration::from\_secs(30);
loop {
let \_input \= MicInput::default()
.record\_until(Instant::now() + recording\_time)                        .await;                }            })    });
}

This code records audio until a certain point in time, providing a continuous stream of audio data.

Transcribing Audio
------------------

Once you have recorded audio data, you can transcribe it into text using the transcription model. The `transcribe` method returns a stream of text snippets along with the confidence of the transcription. We can add the text snippets to the context database to create a real-time context.

{
let document\_table \= document\_table.clone();
std::thread::spawn(move || {
tokio::runtime::Runtime::new()            .unwrap()
.block\_on(async move {
let recording\_time \= Duration::from\_secs(30);
loop {
let input \= MicInput::default()
.record\_until(Instant::now() + recording\_time)                        .await;
let mut stream \= model.transcribe(input);
while let Some(transcribed) \= stream.next().await {
if transcribed.probability\_of\_no\_speech() < 0.90 {
let document \= transcribed.text().into\_document().await.unwrap();
document\_table.insert(document).await.unwrap();
}                    }                }            })    });
}

Create Chat Model
-----------------

Next, we need to create a chat model. We can use the default chat model provided by Kalosm along with the `Chat` interface.

let model \= Llama::new\_chat().await?;
let mut chat \= model.chat().with\_system\_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.");

Implementing Retrieval-Augmented Generation
-------------------------------------------

Finally, we can implement the main chat loop that asks the user for input, searches the context database for relevant information, and generates a response using the chat model.

loop {
// Ask the user for a question
let user\_question \= prompt\_input("\\n\> ")?;
// Search for relevant context in the document engine
let context \= document\_table        .search(&user\_question)
.with\_results(5)
.await?
.into\_iter()
.map(|document| {
format!(                "Title: {}\\nBody: {}\\n",
document.record.title(),
document.record.body()
)        })        .collect::<Vec<\_\>>()
.join("\\n");
// Format a prompt with the question and context
let prompt \= format!(
"Here is the relevant context:\\n{context}\\nGiven that context, answer the following question:\\n{user\_question}"
);
// Display the prompt to the user for debugging purposes
println!("{}", prompt);
// And finally, respond to the user
let mut output\_stream \= chat.add\_message(prompt);
print!("Bot: ");
output\_stream.to\_std\_out().await?;
}

Conclusion
----------

Retrieval-augmented generation in Kalosm makes it possible for language models to generate responses based on up-to-date information from the real world. This guide has shown you how to implement retrieval-augmented generation using the Kalosm library. For more information, the reference documentation documents more details about: Whisper, DocumentTable, and LLMS.