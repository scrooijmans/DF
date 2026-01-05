Title: Building Agentic RAG with Rust, OpenAI & Qdrant | by Joshua Mo | Medium

Description: Hey there! In this article, we’re gonna talk about building an agentic RAG workflow with Rust! We’ll be building an agent that can take a CSV file, parse it and embed it into Qdrant, as well as…

Sitemap

Open in app

Sign up

Sign in

Write

Sign up

Sign in

Building Agentic RAG with Rust, OpenAI & Qdrant
===============================================

Joshua Mo

7 min read

·

May 24, 2024

\--

Listen

Share

Hey there! In this article, we’re gonna talk about building an agentic RAG workflow with Rust! We’ll be building an agent that can take a CSV file, parse it and embed it into Qdrant, as well as retrieving the relevant embeddings from Qdrant to answer questions from users about the contents of the CSV file.

Interested in deploying or just want to see what the final code looks like? You can find the repository here.

What is Agentic RAG?
====================

Agentic RAG, or Agentic Retrieval Augmented Generation, is the concept of mixing AI agents with RAG to be able to produce a workflow that is even better at being tailored to a specific use case than an agent workflow normally would be.

Essentially, the difference between this workflow and a regular agent workflow would be that each agent can individually access embeddings from a vector database to be able to retrieve contextually relevant data — resulting in more accurate answers across the board in an AI agent workflow!

Getting Started
===============

To get started, use `cargo shuttle init` to create a new project.

Next, we’ll add the dependencies we need using a shell snippet:

cargo add anyhow cargo add async-openai cargo add qdrant-client cargo add serde -F derive cargo add serde-json cargo add shuttle-qdrant cargo add uuid -F v4

We’ll also need to make sure to have a Qdrant URL and an API key, as well as an OpenAI API key. Shuttle uses environment variables via a `SecretStore` macro in the main function, and can be stored in the `Secrets.toml` file:

Next, we’ll update our main function to have our Qdrant macro and our secrets macro. We’ll iterate through each secret and set it as an environment variable — this allows us to use our secrets globally, without having to reference the `SecretStore` variable at all:

Building an agentic RAG workflow
================================

Setting up our agent
====================

The agent itself is quite simple: it holds an OpenAI client, as well as a Qdrant client to be able to search for relevant document embeddings. Other fields can also be added here, depending on what capabilities your agent requires.

Next we’ll want to create a helper method for creating the agent, as well as a system message which we’ll feed into the model prompt later.

static SYSTEM\_MESSAGE: &str = "  
You are a world-class data analyst, specialising in analysing comma-delimited CSV files.  

Your job is to analyse some CSV snippets and determine what the results are for the question that the user is asking.  

You should aim to be concise. If you don't know something, don't make it up but say 'I don't know.'.  
"  

impl MyAgent {  
pub fn new(qdrant\_client: QdrantClient) -> Self {  
let api\_key = std::env::var("OPENAI\_API\_KEY").unwrap();  
let config = OpenAIConfig::new().with\_api\_key(api\_key);  

let openai\_client = OpenAIClient::with\_config(config);  

Self {  
openai\_client,  
qdrant\_client,  
}  
}  
}

File parsing and embedding into Qdrant
======================================

Next, we will implement a `File` struct for CSV file parsing - it should be able to hold the file path, contents as well as the rows as a `Vec<String>` (string array, or more accurately a vector of strings). There's a few reasons why we store the rows as a `Vec<String>`:

*   Smaller chunks improve the retrieval accuracy, one of the biggest challenges that RAG has to deal with. Retrieving a wrong or otherwise inaccurate document can hamper accuracy significantly.
*   Improved retrieval accuracy leads to enhanced contextual relevance — which is quite important for complex queries that require specific question.
*   Processing and indexing smaller chunks

pub struct File {  
pub path: String,  
pub contents: String,  
pub rows: Vec<String>,  
}  

impl File {  
pub fn new(path: PathBuf) -> Result<Self> {  
let contents = std::fs::read\_to\_string(&path)?;  

let path\_as\_str = format!("{}", path.display());  

let rows = contents  
.lines()  
.map(|x| x.to\_owned())  
.collect::<Vec<String>>();  

Ok(Self {  
path: path\_as\_str,  
contents,  
rows  
})  
}  
}

While the above parsing method _is_ serviceable (collecting all the lines into a `Vec<String>`), note that it is a naive implementation. Based on how your CSV files are delimited and/or if there is dirty data to clean up, you may want to either prepare your data so that it is already well-prepared, or include some form of data cleaning or validation. Some examples of this might be:

Next, we’ll go back to our agent and implement a method for embedding documents into Qdrant that will take the `File` struct we defined.

To do this, we need to do the following:

*   Take the rows we created earlier and add them as the input for our embed request.
*   Create the embeddings (with openAI) and create a payload for storing alongside the embeddings in Qdrant. Note that although we use a `uuid::Uuid` for unique storage, you could just as easily use numbers by adding a number counter to your struct and incrementing it by 1 after you've inserted an embedding.
*   Assuming there are no errors, return `Ok(())`

use async\_openai::types::{ CreateEmbeddingRequest, EmbeddingInput };  
use async\_openai::Embeddings;  
use qdrant\_client::prelude::{Payload, PointStruct};  

static COLLECTION: &str = "my-collection";  

// text-embedding-ada-002 is the model name from OpenAI that deals with embeddings  
static EMBED\_MODEL: &str = "text-embedding-ada-002";  

impl MyAgent {  
pub async fn embed\_document(&self, file: File) -> Result<()> {  
if file.rows.is\_empty() {  
return Err(anyhow::anyhow!("There's no rows to embed!"));  
}  

let request = CreateEmbeddingRequest {  
model: EMBED\_MODEL.to\_string(),  
input: EmbeddingInput::StringArray(file.rows.clone()),  
user: None,  
dimensions: Some(1536),  
..Default::default()  
};  

let embeddings\_result = Embeddings::new(&self.openai\_client).create(request).await?;  

for embedding in embeddings\_result.data {  
let payload: Payload = serde\_json::json!({  
"id": file.path.clone(),  
"content": file.contents,  
"rows": file.rows  
})  
.try\_into()  
.unwrap();  

println!("Embedded: {}", file.path);  

let vec = embedding.embedding;  

let points = vec!\[PointStruct::new(  
uuid::Uuid::new\_v4().to\_string(),  
vec,  
payload,  
)\];  
self.qdrant\_client  
.upsert\_points(COLLECTION, None, points, None)  
.await?;  
}  
Ok(())  
}  
}

Document searching
==================

Now that we’ve embedded our document, we’ll want a way to check whether our embeddings are contextually relevant to whatever prompt the user gives us. For this, we’ll create a `search_document` function that does the following:

*   Embed the prompt using `CreateEmbeddingRequest` and get the embedding from the results. We'll be using this embedding in our document search. Because we've only added one sentence to embed here (the prompt), it will only return one sentence - so we can create an iterator from the vector and attempt to find the first result.
*   Create a parameter list for our document search through the `SearchPoints` struct (see below). Here we need to set the collection name, the vector that we want to search against (ie the input), how many results we want to be returned if there are any matches, as well as the payload selector.
*   Search the database for results — if there are no results, return an an error; if there is a result, then return the result back.

use qdrant\_client::qdrant::{  
with\_payload\_selector::SelectorOptions, SearchPoints, WithPayloadSelector,  
};  

impl MyAgent {  
async fn search\_document(&self, prompt: String) -> Result<String> {  
let request = CreateEmbeddingRequest {  
model: EMBED\_MODEL.to\_string(),  
input: EmbeddingInput::String(prompt),  
user: None,  
dimensions: Some(1536),  
..Default::default()  
};  

let embeddings\_result = Embeddings::new(&self.openai\_client).create(request).await?;  

let embedding = &embeddings\_result.data.first().unwrap().embedding;  

let payload\_selector = WithPayloadSelector {  
selector\_options: Some(SelectorOptions::Enable(true)),  
};  

// set parameters for search  
let search\_points = SearchPoints {  
collection\_name: COLLECTION.to\_string(),  
vector: embedding.to\_owned(),  
limit: 1,  
with\_payload: Some(payload\_selector),  
..Default::default()  
};  

// if the search is successful  
// attempt to iterate through the results vector and find a result  
let search\_result = self.qdrant\_client.search\_points(&search\_points).await?;  
let result = search\_result.result.into\_iter().next();  

match result {  
Some(res) => Ok(res.payload.get("contents").unwrap().to\_string()),  
None => Err(anyhow::anyhow!("There were no results that matched :(")),  
}  
}  
}

Now that everything we need to use our agent effectively is set up, we can set up a prompt function!

use async\_openai::types::{  
ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,  
ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,  
};  

static PROMPT\_MODEL: &str = "gpt-4o";  

impl MyAgent {  
pub async fn prompt(&self, prompt: &str) -> anyhow::Result<String> {  
let context = self.search\_document(prompt.to\_owned()).await?;  
let input = format!(  
"{prompt}  

Provided context:  
{}  
",  
context // this is the payload from Qdrant  
);  

let res = self  
.openai\_client  
.chat()  
.create(  
CreateChatCompletionRequestArgs::default()  
.model(PROMPT\_MODEL)  
.messages(vec!\[  
//First we add the system message to define what the Agent does  
ChatCompletionRequestMessage::System(  
ChatCompletionRequestSystemMessageArgs::default()  
.content(SYSTEM\_MESSAGE)  
.build()?,  
),  
//Then we add our prompt  
ChatCompletionRequestMessage::User(  
ChatCompletionRequestUserMessageArgs::default()  
.content(input)  
.build()?,  
),  
\])  
.build()?,  
)  
.await  
.map(|res| {  
//We extract the first one  
res.choices\[0\].message.content.clone().unwrap()  
})?;  

println!("Retrieved result from prompt: {res}");  

Ok(res)  
}  
}

Hooking the agent up to our web service
=======================================

Because we separated the agent logic from our web service logic, we just need to connect the bits together and we should be done!

Firstly, we’ll create a couple of structs — the `Prompt` struct that will take a JSON prompt, and the `AppState` function that will act as shared application state in our Axum web server.

#\[derive(Deserialize)\]  
pub struct Prompt {  
prompt: String,  
}  

#\[derive(Clone)\]  
pub struct AppState {  
agent: MyAgent,  
}

We’ll also introduce our prompt handler endpoint here:

async fn prompt(  
State(state): State<AppState>,  
Json(json): Json<Prompt>,  
) -> Result<impl IntoResponse> {  
let prompt\_response = state.agent.prompt(&json.prompt).await?;  

Ok((StatusCode::OK, prompt\_response))  
}

Then we need to parse our CSV file in the main function, create our `AppState` and embed the CSV, as well as setting up our router:

#\[shuttle\_runtime::main\]  
async fn main(  
#\[shuttle\_qdrant::Qdrant\] qdrant\_client: QdrantClient,  
#\[shuttle\_runtime::Secrets\] secrets: SecretStore,  
) -> shuttle\_axum::ShuttleAxum {  
secrets.into\_iter().for\_each(|x| {  
set\_var(x.0, x.1);  
});  

// note that this already assumes you have a file called "test.csv"  
// in your project root  
let file = File::new("test.csv".into())?;  

let state = AppState {  
agent: MyAgent::new(qdrant\_client),  
};  

state.agent.embed\_document(file).await?;  

let router = Router::new()  
.route("/", get(hello\_world))  
.route("/prompt", post(prompt))  
.with\_state(state);  

Ok(router.into())  
}

Deploying
=========

To deploy, all you need to do is use `cargo shuttle deploy` (with the `--ad` flag if on a Git branch with uncommitted changes), sit back and watch the magic happen!

Finishing Up
============

Thanks for reading! With the power of combining AI agents and RAG, we can create powerful workflows to be able to satisfy many different use cases. With Rust, we can leverage performance benefits to be able to run our workflows safely and with a low memory footprint.

_Originally published at_ _https://dev.to_ _on May 24, 2024._

AI

Rust

Rust Programming Language

Tutorial

Web Development

Written by Joshua Mo
--------------------

11 followers

·1 following

Hey there! I'm a Junior DevRel Engineer working at Shuttle.

No responses yet
----------------

Help

Status

About

Careers

Press

Blog

Privacy

Rules

Terms

Text to speech