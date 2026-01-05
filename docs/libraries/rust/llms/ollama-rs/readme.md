Title: GitHub - pepperoni21/ollama-rs: A simple and easy-to-use library for interacting with the Ollama API.

Description: A simple and easy-to-use library for interacting with the Ollama API. - pepperoni21/ollama-rs                                           

Skip to content  

You signed in with another tab or window. Reload to refresh your session. You signed out in another tab or window. Reload to refresh your session. You switched accounts on another tab or window. Reload to refresh your session. Dismiss alert

pepperoni21 / **ollama-rs** Public

*   Notifications You must be signed in to change notification settings
*   Fork 127
*   Star 846

A simple and easy-to-use library for interacting with the Ollama API.

### License

MIT license

846 stars 127 forks Branches Tags Activity

Star

Notifications You must be signed in to change notification settings

pepperoni21/ollama-rs
=====================

 master

BranchesTags

Go to file

Code

Open more actions menu

Folders and files
-----------------

| Name | Name | 
Last commit message

| 

Last commit date

|
| --- | --- | --- | --- |
| 

Latest commit
-------------

History
-------

486 Commits

|
| 

.github

| 

.github

| 

| 

|
| 

ollama-rs-macros

| 

ollama-rs-macros

| 

| 

|
| 

ollama-rs

| 

ollama-rs

| 

| 

|
| 

.gitignore

| 

.gitignore

| 

| 

|
| 

Cargo.lock

| 

Cargo.lock

| 

| 

|
| 

Cargo.toml

| 

Cargo.toml

| 

| 

|
| 

LICENSE.md

| 

LICENSE.md

| 

| 

|
| 

Makefile

| 

Makefile

| 

| 

|
| 

README.md

| 

README.md

| 

| 

|
| 

View all files

|

Repository files navigation
---------------------------

Ollama-rs
=========

### A simple and easy-to-use library for interacting with the Ollama API.

This library was created following the Ollama API documentation.

Table of Contents
-----------------

*   Installation
*   Initialization
*   Usage
*   Completion Generation
*   Completion Generation (Streaming)
*   Completion Generation (With Options)
*   Chat Mode
*   List Local Models
*   Show Model Information
*   Create a Model
*   Create a Model (Streaming)
*   Copy a Model
*   Delete a Model
*   Generate Embeddings
*   Generate Embeddings (Batch)
*   Make a Function Call
*   Create a custom tool
*   Completion Generation (With Thinking)

Installation
------------

### Add ollama-rs to your Cargo.toml

\[dependencies\]
ollama-rs = "0.3.2"

If you absolutely want the latest version, you can use the `master` branch by adding the following to your `Cargo.toml` file:

\[dependencies\]
ollama-rs = { git = "https://github.com/pepperoni21/ollama-rs.git", branch = "master" }

_Note that the `master` branch may not be stable and may contain breaking changes._

Initialization
--------------

### Initialize Ollama

use ollama\_rs::Ollama;

// By default, it will connect to localhost:11434
let ollama = Ollama::default();

// For custom values:
let ollama = Ollama::new("http://localhost".to\_string(), 11434);

Usage
-----

Feel free to check the Chatbot example that shows how to use the library to create a simple chatbot in less than 50 lines of code. You can also check some other examples.

_These examples use poor error handling for simplicity, but you should handle errors properly in your code._

### Completion Generation

use ollama\_rs::generation::completion::GenerationRequest;

let model = "llama2:latest".to\_string();
let prompt = "Why is the sky blue?".to\_string();

let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

if let Ok(res) = res {
println!("{}", res.response);
}

**OUTPUTS:** _The sky appears blue because of a phenomenon called Rayleigh scattering..._

### Completion Generation (Streaming)

_Requires the `stream` feature._

use ollama\_rs::generation::completion::GenerationRequest;
use tokio::io::{self, AsyncWriteExt};
use tokio\_stream::StreamExt;

let model = "llama2:latest".to\_string();
let prompt = "Why is the sky blue?".to\_string();

let mut stream = ollama.generate\_stream(GenerationRequest::new(model, prompt)).await.unwrap();

let mut stdout = io::stdout();
while let Some(res) = stream.next().await {
let responses = res.unwrap();
for resp in responses {
stdout.write\_all(resp.response.as\_bytes()).await.unwrap();
stdout.flush().await.unwrap();
}
}

Same output as above but streamed.

### Completion Generation (With Options)

use ollama\_rs::generation::completion::GenerationRequest;
use ollama\_rs::models::ModelOptions;

let model = "llama2:latest".to\_string();
let prompt = "Why is the sky blue?".to\_string();

let options = ModelOptions::default()
.temperature(0.2)
.repeat\_penalty(1.5)
.top\_k(25)
.top\_p(0.25);

let res = ollama.generate(GenerationRequest::new(model, prompt).options(options)).await;

if let Ok(res) = res {
println!("{}", res.response);
}

**OUTPUTS:** _1\. Sun emits white sunlight: The sun consists primarily ..._

### Chat Mode

_Every message sent and received will be stored in the library's history._

Example with history:

use ollama\_rs::generation::chat::{ChatMessage, ChatMessageRequest};
use ollama\_rs::history::ChatHistory;

let model = "llama2:latest".to\_string();
let prompt = "Why is the sky blue?".to\_string();
// \`Vec<ChatMessage>\` implements \`ChatHistory\`,
// but you could also implement it yourself on a custom type
let mut history = vec!\[\];

let res = ollama
.send\_chat\_messages\_with\_history(
&mut history, // <- messages will be saved here
ChatMessageRequest::new(
model,
vec!\[ChatMessage::user(prompt)\], // <- You should provide only one message
),
)
.await;

if let Ok(res) = res {
println!("{}", res.message.content);
}

_Check chat with history examples for default and stream_

### List Local Models

let res = ollama.list\_local\_models().await.unwrap();

_Returns a vector of `LocalModel` structs._

### Show Model Information

let res = ollama.show\_model\_info("llama2:latest".to\_string()).await.unwrap();

_Returns a `ModelInfo` struct._

### Create a Model

use ollama\_rs::models::create::CreateModelRequest;

let res = ollama.create\_model(CreateModelRequest::path("model".into(), "/tmp/Modelfile.example".into())).await.unwrap();

_Returns a `CreateModelStatus` struct representing the final status of the model creation._

### Create a Model (Streaming)

_Requires the `stream` feature._

use ollama\_rs::models::create::CreateModelRequest;
use tokio\_stream::StreamExt;

let mut res = ollama.create\_model\_stream(CreateModelRequest::path("model".into(), "/tmp/Modelfile.example".into())).await.unwrap();

while let Some(res) = res.next().await {
let res = res.unwrap();
// Handle the status
}

_Returns a `CreateModelStatusStream` that will stream every status update of the model creation._

### Copy a Model

let \_ = ollama.copy\_model("mario".into(), "mario\_copy".into()).await.unwrap();

### Delete a Model

let \_ = ollama.delete\_model("mario\_copy".into()).await.unwrap();

### Generate Embeddings

use ollama\_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

let request = GenerateEmbeddingsRequest::new("llama2:latest".to\_string(), "Why is the sky blue?".into());
let res = ollama.generate\_embeddings(request).await.unwrap();

### Generate Embeddings (Batch)

use ollama\_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

let request = GenerateEmbeddingsRequest::new("llama2:latest".to\_string(), vec!\["Why is the sky blue?", "Why is the sky red?"\].into());
let res = ollama.generate\_embeddings(request).await.unwrap();

_Returns a `GenerateEmbeddingsResponse` struct containing the embeddings (a vector of floats)._

### Make a Function Call

use ollama\_rs::coordinator::Coordinator;
use ollama\_rs::generation::chat::{ChatMessage, ChatMessageRequest};
use ollama\_rs::generation::tools::implementations::{DDGSearcher, Scraper, Calculator};
use ollama\_rs::models::ModelOptions;

let mut history = vec!\[\];

let mut coordinator = Coordinator::new(ollama, "qwen2.5:32b".to\_string(), history)
.options(ModelOptions::default().num\_ctx(16384))
.add\_tool(DDGSearcher::new())
.add\_tool(Scraper {})
.add\_tool(Calculator {});

let resp = coordinator
.chat(vec!\[ChatMessage::user("What is the current oil price?")\])
.await.unwrap();

println!("{}", resp.message.content);

_Uses the given tools (such as searching the web) to find an answer, feeds that answer back into the LLM, and returns a `ChatMessageResponse` with the answer to the question._

### Create a custom tool

The `function` macro simplifies the creation of custom tools. Below is an example of a tool that retrieves the current weather for a specified city:

/// Retrieve the weather for a specified city.
///
/// \* city - The city for which to get the weather.
#\[ollama\_rs::function\]
async fn get\_weather(city: String) -> Result<String, Box<dyn std::error::Error + Sync + Send\>\> {
let url = format!("https://wttr.in/{city}?format=%C+%t");
let response = reqwest::get(&url).await?.text().await?;
Ok(response)
}

To create a custom tool, define a function that returns a `Result<String, Box<dyn std::error::Error + Sync + Send>>` and annotate it with the `function` macro. This function will be automatically converted into a tool that can be used with the `Coordinator`, just like any other tool.

Ensure that the doc comment above the function clearly describes the tool's purpose and its parameters. This information will be provided to the LLM to help it understand how to use the tool.

For a more detailed example, see the function call example.

### Completion Generation (With Thinking)

let model = "qwen3:latest".to\_string();
let prompt = "Why is the sky blue?".to\_string();

let res = ollama.generate(GenerationRequest::new(model, prompt).think(true)).await;

if let Ok(res) = res {
println!("{}", res.response);
}

About
-----

A simple and easy-to-use library for interacting with the Ollama API.

### Resources

Readme

### License

MIT license

### Uh oh!

There was an error while loading. Please reload this page.

Activity

### Stars

**846** stars

### Watchers

**11** watching

### Forks

**127** forks

Report repository

Releases 20
-----------

v0.3.2 Latest

Jun 24, 2025

\+ 19 releases

Packages 0
----------

No packages published  

### Uh oh!

There was an error while loading. Please reload this page.

Contributors 46
---------------

\+ 32 contributors

Languages
---------

*   Rust 100.0%

You can’t perform that action at this time.