Title: Qdrant | 🦜️🔗 LangChain

Description: Qdrant (read: quadrant) is a vector similarity search engine. It provides a production-ready service with a convenient API to store, search, and manage vectors with additional payload and extended filtering support. It makes it useful for all sorts of neural network or semantic-based matching, faceted search, and other applications. 

Skip to main content

**Our Building Ambient Agents with LangGraph course is now available on LangChain Academy!**

On this page

> Qdrant (read: quadrant) is a vector similarity search engine. It provides a production-ready service with a convenient API to store, search, and manage vectors with additional payload and extended filtering support. It makes it useful for all sorts of neural network or semantic-based matching, faceted search, and other applications.

This documentation demonstrates how to use Qdrant with LangChain for dense (i.e., embedding-based), sparse (i.e., text search) and hybrid retrieval. The `QdrantVectorStore` class supports multiple retrieval modes via Qdrant's new Query API. It requires you to run Qdrant v1.10.0 or above.

Setup​
------

There are various modes of how to run `Qdrant`, and depending on the chosen one, there will be some subtle differences. The options include:

*   Local mode, no server required
*   Docker deployments
*   Qdrant Cloud

Please see the installation instructions here.

```
pip install -qU langchain-qdrant
```

### Credentials​

There are no credentials needed to run the code in this notebook.

If you want to get best in-class automated tracing of your model calls you can also set your LangSmith API key by uncommenting below:

```
# os.environ["LANGSMITH_API_KEY"] = getpass.getpass("Enter your LangSmith API key: ")# os.environ["LANGSMITH_TRACING"] = "true"
```

Initialization​
---------------

### Local mode​

The Python client provides the option to run the code in local mode without running the Qdrant server. This is great for testing things out and debugging or storing just a small amount of vectors. The embeddings can be kept fully in-memory or persisted on-disk.

#### In-memory​

For some testing scenarios and quick experiments, you may prefer to keep all the data in-memory only, so it gets removed when the client is destroyed - usually at the end of your script/notebook.

Select embeddings model:

OpenAI▾

*   OpenAI
*   Azure
*   Google Gemini
*   Google Vertex
*   AWS
*   HuggingFace
*   Ollama
*   Cohere
*   MistralAI
*   Nomic
*   NVIDIA
*   Voyage AI
*   IBM watsonx
*   Fake

```
pip install -qU langchain-openai
```

```
import getpassimport osif not os.environ.get("OPENAI_API_KEY"):  os.environ["OPENAI_API_KEY"] = getpass.getpass("Enter API key for OpenAI: ")from langchain_openai import OpenAIEmbeddingsembeddings = OpenAIEmbeddings(model="text-embedding-3-large")
```

```
from langchain_qdrant import QdrantVectorStorefrom qdrant_client import QdrantClientfrom qdrant_client.http.models import Distance, VectorParamsclient = QdrantClient(":memory:")client.create_collection(    collection_name="demo_collection",    vectors_config=VectorParams(size=3072, distance=Distance.COSINE),)vector_store = QdrantVectorStore(    client=client,    collection_name="demo_collection",    embedding=embeddings,)
```

#### On-disk storage​

Local mode, without using the Qdrant server, may also store your vectors on-disk so they persist between runs.

```
client = QdrantClient(path="/tmp/langchain_qdrant")client.create_collection(    collection_name="demo_collection",    vectors_config=VectorParams(size=3072, distance=Distance.COSINE),)vector_store = QdrantVectorStore(    client=client,    collection_name="demo_collection",    embedding=embeddings,)
```

### On-premise server deployment​

No matter if you choose to launch Qdrant locally with a Docker container or select a Kubernetes deployment with the official Helm chart, the way you're going to connect to such an instance will be identical. You'll need to provide a URL pointing to the service.

```
url = "<---qdrant url here --->"docs = []  # put docs hereqdrant = QdrantVectorStore.from_documents(    docs,    embeddings,    url=url,    prefer_grpc=True,    collection_name="my_documents",)
```

### Qdrant Cloud​

If you prefer not to keep yourself busy with managing the infrastructure, you can choose to set up a fully-managed Qdrant cluster on Qdrant Cloud. There is a free forever 1GB cluster included for trying out. The main difference with using a managed version of Qdrant is that you'll need to provide an API key to secure your deployment from being accessed publicly. The value can also be set in a `QDRANT_API_KEY` environment variable.

```
url = "<---qdrant cloud cluster url here --->"api_key = "<---api key here--->"qdrant = QdrantVectorStore.from_documents(    docs,    embeddings,    url=url,    prefer_grpc=True,    api_key=api_key,    collection_name="my_documents",)
```

Using an existing collection​
-----------------------------

To get an instance of `langchain_qdrant.Qdrant` without loading any new documents or texts, you can use the `Qdrant.from_existing_collection()` method.

```
qdrant = QdrantVectorStore.from_existing_collection(    embedding=embeddings,    collection_name="my_documents",    url="http://localhost:6333",)
```

Manage vector store​
--------------------

Once you have created your vector store, we can interact with it by adding and deleting different items.

### Add items to vector store​

We can add items to our vector store by using the `add_documents` function.

```
from uuid import uuid4from langchain_core.documents import Documentdocument_1 = Document(    page_content="I had chocolate chip pancakes and scrambled eggs for breakfast this morning.",    metadata={"source": "tweet"},)document_2 = Document(    page_content="The weather forecast for tomorrow is cloudy and overcast, with a high of 62 degrees Fahrenheit.",    metadata={"source": "news"},)document_3 = Document(    page_content="Building an exciting new project with LangChain - come check it out!",    metadata={"source": "tweet"},)document_4 = Document(    page_content="Robbers broke into the city bank and stole $1 million in cash.",    metadata={"source": "news"},)document_5 = Document(    page_content="Wow! That was an amazing movie. I can't wait to see it again.",    metadata={"source": "tweet"},)document_6 = Document(    page_content="Is the new iPhone worth the price? Read this review to find out.",    metadata={"source": "website"},)document_7 = Document(    page_content="The top 10 soccer players in the world right now.",    metadata={"source": "website"},)document_8 = Document(    page_content="LangGraph is the best framework for building stateful, agentic applications!",    metadata={"source": "tweet"},)document_9 = Document(    page_content="The stock market is down 500 points today due to fears of a recession.",    metadata={"source": "news"},)document_10 = Document(    page_content="I have a bad feeling I am going to get deleted :(",    metadata={"source": "tweet"},)documents = [    document_1,    document_2,    document_3,    document_4,    document_5,    document_6,    document_7,    document_8,    document_9,    document_10,]uuids = [str(uuid4()) for _ in range(len(documents))]
```

**API Reference:**Document

```
vector_store.add_documents(documents=documents, ids=uuids)
```

### Delete items from vector store​

```
vector_store.delete(ids=[uuids[-1]])
```

```
True
```

Query vector store​
-------------------

Once your vector store has been created and the relevant documents have been added, you will most likely wish to query it during the running of your chain or agent.

### Query directly​

The simplest scenario for using the Qdrant vector store is to perform a similarity search. Under the hood, our query will be encoded into vector embeddings and used to find similar documents in a Qdrant collection.

```
results = vector_store.similarity_search(    "LangChain provides abstractions to make working with LLMs easy", k=2)for res in results:    print(f"* {res.page_content} [{res.metadata}]")
```

```
* Building an exciting new project with LangChain - come check it out! [{'source': 'tweet', '_id': 'd3202666-6f2b-4186-ac43-e35389de8166', '_collection_name': 'demo_collection'}]* LangGraph is the best framework for building stateful, agentic applications! [{'source': 'tweet', '_id': '91ed6c56-fe53-49e2-8199-c3bb3c33c3eb', '_collection_name': 'demo_collection'}]
```

`QdrantVectorStore` supports 3 modes for similarity searches. They can be configured using the `retrieval_mode` parameter.

*   Dense Vector Search (default)
*   Sparse Vector Search
*   Hybrid Search

### Dense Vector Search​

Dense vector search involves calculating similarity via vector-based embeddings. To search with only dense vectors:

*   The `retrieval_mode` parameter should be set to `RetrievalMode.DENSE`. This is the default behavior.
*   A dense embeddings value should be provided to the `embedding` parameter.

```
from langchain_qdrant import QdrantVectorStore, RetrievalModefrom qdrant_client import QdrantClientfrom qdrant_client.http.models import Distance, VectorParams# Create a Qdrant client for local storageclient = QdrantClient(path="/tmp/langchain_qdrant")# Create a collection with dense vectorsclient.create_collection(    collection_name="my_documents",    vectors_config=VectorParams(size=3072, distance=Distance.COSINE),)qdrant = QdrantVectorStore(    client=client,    collection_name="my_documents",    embedding=embeddings,    retrieval_mode=RetrievalMode.DENSE,)qdrant.add_documents(documents=documents, ids=uuids)query = "How much money did the robbers steal?"found_docs = qdrant.similarity_search(query)found_docs
```

### Sparse Vector Search​

To search with only sparse vectors:

*   The `retrieval_mode` parameter should be set to `RetrievalMode.SPARSE`.
*   An implementation of the `SparseEmbeddings` interface using any sparse embeddings provider has to be provided as a value to the `sparse_embedding` parameter.

The `langchain-qdrant` package provides a FastEmbed based implementation out of the box.

To use it, install the FastEmbed package.

```
%pip install -qU fastembed
```

```
from langchain_qdrant import FastEmbedSparse, QdrantVectorStore, RetrievalModefrom qdrant_client import QdrantClient, modelsfrom qdrant_client.http.models import Distance, SparseVectorParams, VectorParamssparse_embeddings = FastEmbedSparse(model_name="Qdrant/bm25")# Create a Qdrant client for local storageclient = QdrantClient(path="/tmp/langchain_qdrant")# Create a collection with sparse vectorsclient.create_collection(    collection_name="my_documents",    vectors_config={"dense": VectorParams(size=3072, distance=Distance.COSINE)},    sparse_vectors_config={        "sparse": SparseVectorParams(index=models.SparseIndexParams(on_disk=False))    },)qdrant = QdrantVectorStore(    client=client,    collection_name="my_documents",    sparse_embedding=sparse_embeddings,    retrieval_mode=RetrievalMode.SPARSE,    sparse_vector_name="sparse",)qdrant.add_documents(documents=documents, ids=uuids)query = "How much money did the robbers steal?"found_docs = qdrant.similarity_search(query)found_docs
```

### Hybrid Vector Search​

To perform a hybrid search using dense and sparse vectors with score fusion,

*   The `retrieval_mode` parameter should be set to `RetrievalMode.HYBRID`.
*   A dense embeddings value should be provided to the `embedding` parameter.
*   An implementation of the `SparseEmbeddings` interface using any sparse embeddings provider has to be provided as a value to the `sparse_embedding` parameter.

Note that if you've added documents with the `HYBRID` mode, you can switch to any retrieval mode when searching, since both the dense and sparse vectors are available in the collection.

```
from langchain_qdrant import FastEmbedSparse, QdrantVectorStore, RetrievalModefrom qdrant_client import QdrantClient, modelsfrom qdrant_client.http.models import Distance, SparseVectorParams, VectorParamssparse_embeddings = FastEmbedSparse(model_name="Qdrant/bm25")# Create a Qdrant client for local storageclient = QdrantClient(path="/tmp/langchain_qdrant")# Create a collection with both dense and sparse vectorsclient.create_collection(    collection_name="my_documents",    vectors_config={"dense": VectorParams(size=3072, distance=Distance.COSINE)},    sparse_vectors_config={        "sparse": SparseVectorParams(index=models.SparseIndexParams(on_disk=False))    },)qdrant = QdrantVectorStore(    client=client,    collection_name="my_documents",    embedding=embeddings,    sparse_embedding=sparse_embeddings,    retrieval_mode=RetrievalMode.HYBRID,    vector_name="dense",    sparse_vector_name="sparse",)qdrant.add_documents(documents=documents, ids=uuids)query = "How much money did the robbers steal?"found_docs = qdrant.similarity_search(query)found_docs
```

If you want to execute a similarity search and receive the corresponding scores you can run:

```
results = vector_store.similarity_search_with_score(    query="Will it be hot tomorrow", k=1)for doc, score in results:    print(f"* [SIM={score:3f}] {doc.page_content} [{doc.metadata}]")
```

```
* [SIM=0.531834] The weather forecast for tomorrow is cloudy and overcast, with a high of 62 degrees. [{'source': 'news', '_id': '9e6ba50c-794f-4b88-94e5-411f15052a02', '_collection_name': 'demo_collection'}]
```

For a full list of all the search functions available for a `QdrantVectorStore`, read the API reference

### Metadata filtering​

Qdrant has an extensive filtering system with rich type support. It is also possible to use the filters in Langchain, by passing an additional param to both the `similarity_search_with_score` and `similarity_search` methods.

```
from qdrant_client import modelsresults = vector_store.similarity_search(    query="Who are the best soccer players in the world?",    k=1,    filter=models.Filter(        should=[            models.FieldCondition(                key="page_content",                match=models.MatchValue(                    value="The top 10 soccer players in the world right now."                ),            ),        ]    ),)for doc in results:    print(f"* {doc.page_content} [{doc.metadata}]")
```

```
* The top 10 soccer players in the world right now. [{'source': 'website', '_id': 'b0964ab5-5a14-47b4-a983-37fa5c5bd154', '_collection_name': 'demo_collection'}]
```

### Query by turning into retriever​

You can also transform the vector store into a retriever for easier usage in your chains.

```
retriever = vector_store.as_retriever(search_type="mmr", search_kwargs={"k": 1})retriever.invoke("Stealing from the bank is a crime")
```

```
[Document(metadata={'source': 'news', '_id': '50d8d6ee-69bf-4173-a6a2-b254e9928965', '_collection_name': 'demo_collection'}, page_content='Robbers broke into the city bank and stole $1 million in cash.')]
```

Usage for retrieval-augmented generation​
-----------------------------------------

For guides on how to use this vector store for retrieval-augmented generation (RAG), see the following sections:

*   Tutorials
*   How-to: Question and answer with RAG
*   Retrieval conceptual docs

Customizing Qdrant​
-------------------

There are options to use an existing Qdrant collection within your LangChain application. In such cases, you may need to define how to map Qdrant point into the LangChain `Document`.

### Named vectors​

Qdrant supports multiple vectors per point by named vectors. If you work with a collection created externally or want to have the differently named vector used, you can configure it by providing its name.

```
from langchain_qdrant import RetrievalModeQdrantVectorStore.from_documents(    docs,    embedding=embeddings,    sparse_embedding=sparse_embeddings,    location=":memory:",    collection_name="my_documents_2",    retrieval_mode=RetrievalMode.HYBRID,    vector_name="custom_vector",    sparse_vector_name="custom_sparse_vector",)
```

### Metadata​

Qdrant stores your vector embeddings along with the optional JSON-like payload. Payloads are optional, but since LangChain assumes the embeddings are generated from the documents, we keep the context data, so you can extract the original texts as well.

By default, your document is going to be stored in the following payload structure:

```
{    "page_content": "Lorem ipsum dolor sit amet",    "metadata": {        "foo": "bar"    }}
```

You can, however, decide to use different keys for the page content and metadata. That's useful if you already have a collection that you'd like to reuse.

```
QdrantVectorStore.from_documents(    docs,    embeddings,    location=":memory:",    collection_name="my_documents_2",    content_payload_key="my_page_content_key",    metadata_payload_key="my_meta",)
```

API reference​
--------------

For detailed documentation of all `QdrantVectorStore` features and configurations head to the API reference: https://python.langchain.com/api\_reference/qdrant/qdrant/langchain\_qdrant.qdrant.QdrantVectorStore.html

Related​
--------

*   Vector store conceptual guide
*   Vector store how-to guides

*   Setup
*   Credentials
*   Initialization
*   Local mode
*   On-premise server deployment
*   Qdrant Cloud
*   Using an existing collection
*   Manage vector store
*   Add items to vector store
*   Delete items from vector store
*   Query vector store
*   Query directly
*   Dense Vector Search
*   Sparse Vector Search
*   Hybrid Vector Search
*   Metadata filtering
*   Query by turning into retriever
*   Usage for retrieval-augmented generation
*   Customizing Qdrant
*   Named vectors
*   Metadata
*   API reference
*   Related