Title: Kalosm

Description: Floneum is a graph editor for local LLM workflows. 

Audio Transcription
===================

Kalosm provides helpers that allow you to easily transcribe audio data from either your microphone or a file. In this chapter, we will learn how to use Kalosm to perform real time audio transcription.

Creating a Transcription Model
------------------------------

First, we need to create a transcription model. A transcription model is a machine learning model that can be used to transcribe audio data. Kalosm provides a `Whisper` struct that can be used to create a transcription model.

use kalosm::sound::\*;
// Create a new whisper model
let model \= Whisper::new().await?;

> Bonus: Download progress
> 
> If you need to update progress while you are downloading the model, you can use the whisper builder with the `build_with_loading_handler` method.
> 
> let model \= Whisper::builder()
>     .build\_with\_loading\_handler(|progress| match &progress {
>         ModelLoadingProgress::Downloading {            source,            progress: file\_loading\_progress,        } \=> {
>             let progress \= (progress.progress() \* 100.0) as u32;
>             let elapsed \= file\_loading\_progress.start\_time.elapsed().as\_secs\_f32();
>             println!("Downloading file {source} {progress}% ({elapsed}s)");
>         }        ModelLoadingProgress::Loading { progress } \=> {
>             let progress \= (progress \* 100.0) as u32;
>             println!("Loading model {progress}%");
>         }    })    .await?;

Recording Audio
---------------

Next, we need to record some audio from our environment. We can use the `MicInput` struct with the `stream` method to stream audio from our microphone:

// Stream audio from the microphone
let mic \= MicInput::default();
let stream \= mic.stream();

Transcribing Audio
------------------

Finally, we can use the `transcribe` method to transcribe the audio data that we recorded into text. The transcribe method takes some audio data and returns a stream of snippets of text along with the confidence of the transcription.

// Transcribe the audio.
let mut transcribed \= stream.transcribe(model);
// As the model transcribes the audio, print the text to the console.
transcribed.to\_std\_out().await?;

Conclusion
----------

In this chapter, we learned how to use Kalosm to transcribe audio data from our microphone. Audio data can be a powerful source of real time information. You can combine audio data with retrieval-augmented generation to create a chat bot that understands it's surroundings.