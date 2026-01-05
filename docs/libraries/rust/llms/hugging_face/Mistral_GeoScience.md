# JCChen0331/Mistral-7B-Geoscience-v0.1 · Hugging Face
[](#model-card-for-model-id)Model Card for Model ID
---------------------------------------------------

This model is fine-tuned on content from Nature Geoscience papers, enhancing its performance in geoscience-related tasks.

[](#model-details)Model Details
-------------------------------

### [](#model-description)Model Description

\-**Model Name**: Mistral-7B-Geoscience-v0.1

[](#uses)Uses
-------------

[](#training-details)Training Details
-------------------------------------

### [](#training-data)Training Data

Nature Geoscience paper text (~30K samples)

### [](#evaluation)Evaluation

Answer:

_Question_:

How to measure Hg concentration in a rock sample?

_Answer_:

`GPT-4`: "To measure Hg concentration in a rock sample, one typically uses instrumental methods like atomic absorption spectrometry (AAS) or inductively coupled plasma mass spectrometry (ICP-MS)....."

`Pre-trained Mistral-7B`: "The mercury content of the samples is determined by atomic absorption spectrophotometry. The method involves dissolving 1 gm of the powdered sample in concentrated nitric acid and then adding an excess amount of potassium iodide solution (KI). This forms a complex with mercury, which can be measured at 253.7 nm wavelength using a flame atomizer....."

`Mistral-7B-Geoscience-v0.1`: "The analytical method used for the determination of Hg concentrations is based on cold vapor atomic fluorescence spectrometry (CVAFS). The CVAFS technique has been widely applied and validated as an accurate and precise method for measuring trace amounts of mercury in environmental samples."