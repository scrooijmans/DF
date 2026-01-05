Title: Kalosm

Description: Floneum is a graph editor for local LLM workflows. 

Text Recognition
================

Kalosm allows developers to extract text information from images using optical character recognition (OCR). This guide demonstrates how to perform single-line OCR using Kalosm's vision module.

Adding dependencies
-------------------

Before we get started, we need to add an additional crate for image loading. Add the following line to your `Cargo.toml` file:

\[dependencies\]
# Your Kalosm dependency added in the start of the documentation...
image \= "0.24.7"

Creating an OCR Model
---------------------

Kalosm's `vision` module provides functionality for text recognition in images. In this example, the `Ocr::builder()` method is used to create an OCR model that can transcribe single lines of text.

use kalosm::vision::\*;
let mut model \= Ocr::builder().build().await.unwrap();

Loading Image
-------------

Next, we need to load an image that contains text. The `image` crate provides the open method to load an image from a file path, or the Reader for more advanced loading options.

let image \= image::open("examples/ocr.png").unwrap();

Replace the file path with the location of your image. This loaded image will be processed for text recognition.

Recognizing Text
----------------

Finally, we can use the `recognize_text` method to extract text information from the image. The `recognize_text` method takes an `OcrInferenceSettings` struct as input. This struct contains the image to be processed, as well as other settings that can be used to customize the OCR process.

let text \= model    .recognize\_text(OcrInferenceSettings::new(image))
.unwrap();
println!("{}", text);

Conclusion
----------

This example provides a basic structure for performing single-line OCR using Kalosm's vision module. You can combine text recognition with an LLM to analyze complex documents or photos.