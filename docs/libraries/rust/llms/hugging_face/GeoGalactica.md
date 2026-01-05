# geobrain-ai/geogalactica · Hugging Face
[](#ge🌏galactica-a-scientific-large-language-model-in-geoscience)Ge🌏Galactica: A Scientific Large Language Model in Geoscience
--------------------------------------------------------------------------------------------------------------------------------

GeoGalactica is from further pre-training of Galactica -- a top-performing LLM trained with a large number of scientific documents.

[](#model-details)Model Details
-------------------------------

In this work, we take the initial step to leverage LLM for science, through a rather straightforward approach. We try to specialize an open-sourced LLM into geoscience, by further pre-training the model with a vast amount of texts in geoscience, as well as supervised fine-tuning (SFT) the resulting model with our custom collected instruction tuning dataset. These efforts result in a model GeoGalactica consisting of **30 billion parameters**. To our best knowledge, it is the largest language model for the geoscience domain.

### [](#model-description)Model Description

*   **Developed by:** Shanghai Jiao Tong University.
*   **Shared by \[optional\]:** [GeoBRAIN.ai](https://www.geobrain-ai.com/)
*   **Model type:** Further pre-train and Supervised Fine-tuning
*   **Language(s) (NLP):** English
*   **License:** Apache License 2.0
*   **Finetuned from model:** [Galactica](https://huggingface.co/facebook/galactica-30b)

### [](#model-sources)Model Sources

*   **Repository:** [geobrain-ai/geogalactica](https://github.com/geobrain-ai/geogalactica)
*   **Paper:** [GeoGalactica: A Scientific Large Language Model in Geoscience](https://arxiv.org/abs/2401.00434)

[](#training-details)Training Details
-------------------------------------

### [](#training-data)Training Data

*   **Further pre-train:** A geoscience-related text corpus containing **65** billion tokens curated from extensive data sources, preserving as the largest geoscience-specific text corpus.
*   **Supervised Fine-tuning:** [daven3/geosignal](https://huggingface.co/datasets/daven3/geosignal)
*   **Tool Learning:** [zthang/geotools](https://github.com/zthang/geotools)

#### [](#training-hyperparameters)Training Hyperparameters

*   **Model setup (30B parameters)**
    
    *   num layers: 48
    *   num attention heads: 56
    *   hidden size: 7168
    *   max position embeddings: 2048
    *   layernorm epsilon: 1e-5
*   **Regularization Setup**
    
    *   optimizer: Adam
    *   attention dropout: 0.1
    *   hidden dropout: 0.1
    *   weight decay: 0.1
    *   clip-grad: 1.0
    *   adam \\beta\_{1}: 0.9
    *   adam \\beta\_{2}: 0.95
    *   adam \\epsilon: 1e-8
*   **Training Setup**
    
    *   micro-batch-size: 1
    *   global-batch-size: 4096
    *   recompute-activations: True (gradient checkpointing)
    *   train-samples: 30M (~65B token)
    *   disable-bias-linear: True (turn off the bias of _**nn.linear**_)
    *   seed: 42
    *   save-interval: 100
*   **Learning Rate Setup**
    
    *   lr-decay-style: linear
    *   lr-warmup-steps: 100
    *   lr: 1e-5
    *   min-lr: 1e-7
*   **Mixed Precision Setup**
    
    *   FP16: False
    *   BF16: False
*   **Training regime:** fp32
    
*   **Parallel Configuration**
    
    *   tensor-model-parallel-size: 4
    *   pipeline-model-parallel-size: 16
    *   distributed-backend: NCCL
    *   sequence-parallel: True

[](#evaluation-data)Evaluation Data
-----------------------------------

*   **MMLU:** Massive Multitask Language Understanding, a large-scale research initiative aimed at improving language models' understanding and reasoning abilities across a diverse range of subjects and tasks.
*   **GeoBench:** The benchmark mentioned in K2. The data can be access on [daven3/geobench](https://huggingface.co/datasets/daven3/geobench).
*   **Human Evaluation:** Selected questions.

[](#environmental-impact)Environmental Impact
---------------------------------------------

During our cumulative training, 1,488,137.26 DCU hours were consumed, resulting in cumulative carbon emissions of 212 tCO\_{2}eq. Our work provides a foundational model for subsequent geoscience researchers to fine-tune their smaller models, potentially reducing carbon emissions in their future work.

*   **Hardware Type:** DCU
*   **Hours used:** 1,488,137.26
*   **Cloud Provider:** Advanced Computing East China Sub-center

[](#ethical-considerations)Ethical Considerations
-------------------------------------------------

This model inherits from Galactica, and in the training corpus, we have conducted sufficient data governance to ensure that the training data embodies geographical community, transparency, inclusiveness, respect for privacy, and topic neutrality.

[](#citation-optional)Citation \[optional\]
-------------------------------------------

[](#model-card-author-and-contact)Model Card Author and Contact
---------------------------------------------------------------

GeoGalactica is a research preview intended for non-commercial use only. Please contact us if you find any issues. For details, you can email [Cheng Deng](mailto:davendw@sjtu.edu.cn)