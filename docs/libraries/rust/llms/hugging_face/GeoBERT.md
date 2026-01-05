# botryan96/GeoBERT · Hugging Face
GeoBERT is a NER model that was fine-tuned from SciBERT on the Geoscientific Corpus dataset. The model was trained on the Labeled Geoscientific Corpus dataset (~1 million sentences).

[](#intended-uses)Intended uses
-------------------------------

The NER product in this model has a goal to identify four main semantic types or categories related to Geosciences.

1.  GeoPetro for any entities that belong to all terms in Geosciences
2.  GeoMeth for all tools or methods associated with Geosciences
3.  GeoLoc to identify geological locations
4.  GeoTime for identifying the geological time scale entities

### [](#training-hyperparameters)Training hyperparameters

The following hyperparameters were used during training:

*   optimizer: {'name': 'AdamWeightDecay', 'learning\_rate': {'class\_name': 'PolynomialDecay', 'config': {'initial\_learning\_rate': 2e-05, 'decay\_steps': 14000, 'end\_learning\_rate': 0.0, 'power': 1.0, 'cycle': False, 'name': None}}, 'decay': 0.0, 'beta\_1': 0.9, 'beta\_2': 0.999, 'epsilon': 1e-08, 'amsgrad': False, 'weight\_decay\_rate': 0.01}
*   training\_precision: mixed\_float16

### [](#framework-versions)Framework versions

*   Transformers 4.22.1
*   TensorFlow 2.10.0
*   Datasets 2.4.0
*   Tokenizers 0.12.1

[](#model-performances-metric-seqeval)Model performances (metric: seqeval)
--------------------------------------------------------------------------


|entity  |precision|recall|f1    |
|--------|---------|------|------|
|GeoLoc  |0.9727   |0.9591|0.9658|
|GeoMeth |0.9433   |0.9447|0.9445|
|GeoPetro|0.9767   |0.9745|0.9756|
|GeoTime |0.9695   |0.9666|0.9680|


[](#how-to-use-geobert-with-huggingface)How to use GeoBERT with HuggingFace
---------------------------------------------------------------------------

##### [](#load-geobert-and-its-sub-word-tokenizer-)Load GeoBERT and its sub-word tokenizer :

```
from transformers import AutoTokenizer, AutoModelForTokenClassification

tokenizer = AutoTokenizer.from_pretrained("botryan96/GeoBERT")
model = AutoModelForTokenClassification.from_pretrained("botryan96/GeoBERT")

#Define the pipeline
from transformers import pipeline
ner_machine = pipeline('ner',model = models,tokenizer=tokenizer, aggregation_strategy="simple")

#Define the sentence
sentence = 'In North America, the water storage in the seepage face model is higher than the base case because positive pore pressure is requisite for drainage through a seepage face boundary condition. The result from the resistivity data supports the notion, especially in the northern part of the Sandstone Sediment formation. The active formation of America has a big potential for Oil and Gas based on the seismic section, has been activated since the Paleozoic'

#Deploy the NER Machine
ner_machine(sentence)

```
