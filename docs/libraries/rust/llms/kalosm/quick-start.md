Title: Kalosm

Description: Floneum is a graph editor for local LLM workflows. 

Introduction
============

Kalosm is a library with dead simple interfaces for local, language, audio, and image models

Quick Start
-----------

*   Add the Kalosm and Tokio libraries

# Enable the metal or cuda feature if you have a supported accelerator
cargo add kalosm \--features language
cargo add tokio \--features full

*   Initialize a Kalosm model

let model \= Llama::new\_chat().await?;

*   Start a chat session with a pirate

use kalosm::language::\*;
#\[tokio::main\]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let model \= Llama::new\_chat().await?;
// New code
let mut chat \= model        .chat()
.with\_system\_prompt("The assistant will act like a pirate");
loop {
chat(&prompt\_input("\\n\> ")?).to\_std\_out().await?;
}
}

*   Add build configuration to your `.cargo/config.toml` for improved performance

\[build\]
rustflags \= \["-C", "target-cpu=native"\]
\[target.x86\_64\-apple\-darwin\]
rustflags \= \["-C", "target-feature=-avx,-avx2"\]

*   Run the program

cargo run \--release