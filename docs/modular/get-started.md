Title: Quickstart | Modular

Description: A quickstart guide to run a GenAI model locally with Modular.

Skip to main content

/

DocsCode

Nightly

Nightly

Work in progress

v25.4

Jun 18, 2025/ Stable release

Log in

Search

Collapse sidebar

In this quickstart guide, you'll learn how to install Modular in a Python environment and run inference with a GenAI model. We'll first use our Python API to run offline inference, then start a local endpoint and use the OpenAI Python API to send inference requests.

System requirements:

Mac

Linux

WSL

Docker

Set up your project​
--------------------

First, install the `max` CLI and Python library in a virtual environment.

You have a few options below but we recommend installing with `pixi` for the most reliable experience.

*   pixi
*   uv
*   pip
*   conda

1.  If you don't have it, install `pixi`:

```
curl -fsSL https://pixi.sh/install.sh | sh
```

```
curl -fsSL https://pixi.sh/install.sh | sh
```

Then restart your terminal for the changes to take effect.

2.  Create a project:

```
pixi init quickstart \  -c https://conda.modular.com/max-nightly/ -c conda-forge \  && cd quickstart
```

```
pixi init quickstart \  -c https://conda.modular.com/max-nightly/ -c conda-forge \  && cd quickstart
```

3.  Install the `modular` conda package:

*   Nightly
*   Stable

```
pixi add modular
```

```
pixi add modular
```

```
pixi add "modular=25.4"
```

```
pixi add "modular=25.4"
```

4.  Start the virtual environment:

```
pixi shell
```

```
pixi shell
```

1.  If you don't have it, install `uv`:

```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Then restart your terminal to make `uv` accessible.

2.  Create a project:

```
uv init quickstart && cd quickstart
```

```
uv init quickstart && cd quickstart
```

3.  Create and start a virtual environment:

```
uv venv && source .venv/bin/activate
```

```
uv venv && source .venv/bin/activate
```

4.  Install the `modular` Python package:

*   Nightly
*   Stable

```
uv pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --index-url https://dl.modular.com/public/nightly/python/simple/ \  --index-strategy unsafe-best-match --prerelease allow
```

```
uv pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --index-url https://dl.modular.com/public/nightly/python/simple/ \  --index-strategy unsafe-best-match --prerelease allow
```

```
uv pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --extra-index-url https://modular.gateway.scarf.sh/simple/ \  --index-strategy unsafe-best-match
```

```
uv pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --extra-index-url https://modular.gateway.scarf.sh/simple/ \  --index-strategy unsafe-best-match
```

1.  Create a project folder:

```
mkdir quickstart && cd quickstart
```

```
mkdir quickstart && cd quickstart
```

2.  Create and activate a virtual environment:

```
python3 -m venv .venv/quickstart \  && source .venv/quickstart/bin/activate
```

```
python3 -m venv .venv/quickstart \  && source .venv/quickstart/bin/activate
```

3.  Install the `modular` Python package:

*   Nightly
*   Stable

```
pip install --pre modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --index-url https://dl.modular.com/public/nightly/python/simple/
```

```
pip install --pre modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --index-url https://dl.modular.com/public/nightly/python/simple/
```

```
pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --extra-index-url https://modular.gateway.scarf.sh/simple/
```

```
pip install modular \  --extra-index-url https://download.pytorch.org/whl/cpu \  --extra-index-url https://modular.gateway.scarf.sh/simple/
```

1.  If you don't have it, install conda. A common choice is with `brew`:

```
brew install miniconda
```

```
brew install miniconda
```

2.  Initialize `conda` for shell interaction:

```
conda init
```

```
conda init
```

If you're on a Mac, instead use:

```
conda init zsh
```

```
conda init zsh
```

Then restart your terminal for the changes to take effect.

3.  Create a project:

```
conda create -n quickstart
```

```
conda create -n quickstart
```

4.  Start the virtual environment:

```
conda activate quickstart
```

```
conda activate quickstart
```

5.  Install the `modular` conda package:

*   Nightly
*   Stable

```
conda install -c conda-forge -c https://conda.modular.com/max-nightly/ modular
```

```
conda install -c conda-forge -c https://conda.modular.com/max-nightly/ modular
```

```
conda install -c conda-forge -c https://conda.modular.com/max/ modular
```

```
conda install -c conda-forge -c https://conda.modular.com/max/ modular
```

When using `pip`, we use the `--index-url` argument to ensure that `torch` installs CPU dependencies only, avoiding a lot of unnecessary GPU packages. This is a temporary workaround until we can remove all dependencies on PyTorch.

Run offline inference​
----------------------

You can run inference locally with the `max` Python API. Just specify the Hugging Face model you want and then generate results with one or more prompts.

In this example, we use a Llama 3.1 model that's not gated on Hugging Face, so you don't need an access token:

offline-inference.py

```
from max.entrypoints.llm import LLMfrom max.pipelines import PipelineConfigdef main():    model_path = "modularai/Llama-3.1-8B-Instruct-GGUF"    pipeline_config = PipelineConfig(model_path=model_path)    llm = LLM(pipeline_config)    prompts = [        "In the beginning, there was",        "I believe the meaning of life is",        "The fastest way to learn python is",    ]    print("Generating responses...")    responses = llm.generate(prompts, max_new_tokens=50)    for i, (prompt, response) in enumerate(zip(prompts, responses)):        print(f"========== Response {i} ==========")        print(prompt + response)        print()if __name__ == "__main__":    main()
```

```
from max.entrypoints.llm import LLMfrom max.pipelines import PipelineConfigdef main():    model_path = "modularai/Llama-3.1-8B-Instruct-GGUF"    pipeline_config = PipelineConfig(model_path=model_path)    llm = LLM(pipeline_config)    prompts = [        "In the beginning, there was",        "I believe the meaning of life is",        "The fastest way to learn python is",    ]    print("Generating responses...")    responses = llm.generate(prompts, max_new_tokens=50)    for i, (prompt, response) in enumerate(zip(prompts, responses)):        print(f"========== Response {i} ==========")        print(prompt + response)        print()if __name__ == "__main__":    main()
```

Run it and you should see a response similar to this:

```
python offline-inference.py
```

```
python offline-inference.py
```

```
========== Response 0 ==========In the beginning, there was Andromeda. The Andromeda galaxy, that is. It's the closest major galaxy to our own Milky Way, and it's been a source of fascination for astronomers and space enthusiasts for centuries. But what if I told you that there's========== Response 1 ==========I believe the meaning of life is to find your gift. The purpose of life is to give it away to others.I believe that the meaning of life is to find your gift. The purpose of life is to give it away to others.I believe that the meaning of life is========== Response 2 ==========The fastest way to learn python is to practice with real-world projects. Here are some ideas for projects that you can use to learn Python:1.  **Command Line Calculator**: Create a command line calculator that can perform basic arithmetic operations like addition, subtraction, multiplication, and division.
```

```
========== Response 0 ==========In the beginning, there was Andromeda. The Andromeda galaxy, that is. It's the closest major galaxy to our own Milky Way, and it's been a source of fascination for astronomers and space enthusiasts for centuries. But what if I told you that there's========== Response 1 ==========I believe the meaning of life is to find your gift. The purpose of life is to give it away to others.I believe that the meaning of life is to find your gift. The purpose of life is to give it away to others.I believe that the meaning of life is========== Response 2 ==========The fastest way to learn python is to practice with real-world projects. Here are some ideas for projects that you can use to learn Python:1.  **Command Line Calculator**: Create a command line calculator that can perform basic arithmetic operations like addition, subtraction, multiplication, and division.
```

More information about this API is available in the offline inference guide.

Run inference with an endpoint​
-------------------------------

Now let's start a local server that runs the model using an OpenAI-compatible endpoint:

1.  Install the `openai` client library:

*   pixi
*   uv
*   pip
*   conda

```
pixi add openai
```

```
pixi add openai
```

```
uv add openai
```

```
uv add openai
```

```
pip install openai
```

```
pip install openai
```

```
conda install -c conda-forge openai
```

```
conda install -c conda-forge openai
```

2.  Start the endpoint with the `max` CLI:

```
max serve --model-path=modularai/Llama-3.1-8B-Instruct-GGUF
```

```
max serve --model-path=modularai/Llama-3.1-8B-Instruct-GGUF
```

3.  Create a new file that sends an inference request:

generate-text.py

```
from openai import OpenAIclient = OpenAI(    base_url="http://0.0.0.0:8000/v1",    api_key="EMPTY",)completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=[        {          "role": "user",          "content": "Who won the world series in 2020?"        },    ],)print(completion.choices[0].message.content)
```

```
from openai import OpenAIclient = OpenAI(    base_url="http://0.0.0.0:8000/v1",    api_key="EMPTY",)completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=[        {          "role": "user",          "content": "Who won the world series in 2020?"        },    ],)print(completion.choices[0].message.content)
```

Notice that the `OpenAI` API requires the `api_key` argument, but our endpoint doesn't use it.

4.  Wait until the previous `max serve` command prints this message:

```
Server ready on http://0.0.0.0:8000 (Press CTRL+C to quit)
```

```
Server ready on http://0.0.0.0:8000 (Press CTRL+C to quit)
```

Then run the Python script from another teriminal and you should see results like this:

```
python generate-text.py
```

```
python generate-text.py
```

```
The Los Angeles Dodgers won the 2020 World Series. They defeated the Tampa Bay Rays in the series 4 games to 2. This was the Dodgers' first World Series title since 1988.
```

```
The Los Angeles Dodgers won the 2020 World Series. They defeated the Tampa Bay Rays in the series 4 games to 2. This was the Dodgers' first World Series title since 1988.
```

That's it. You just served Llama 3 on your local CPU and ran inference using our OpenAI-compatible Serve API.

You can also deploy the same endpoint to a cloud GPU using our Docker container.

To run a different model, change the `--model-path` to something else from our model repository.

Keep going​
-----------

There's still a lot more to learn. Here are some directions you can go:

### Docs​

Serving

Try more serving features like function calling, tool use, structured output, and more.

Deploying

Try a tutorial to deploy a model on a cloud GPU using our Docker container.

Developing

Discover all the ways you can customize your AI deployments, such as writing custom ops and GPU kernels in Mojo.

Mojo manual

Learn to program in Mojo, a Pythonic systems programming language that allows you to write code for both CPUs and GPUs.

### Resources​

Model repo

Hundreds of GenAI models accelerated with Modular.

Tutorials

Step-by-step procedures to develop and deploy with Modular.

Recipes

Turn-key applications that use GenAI models with Modular.

GPU puzzles

A hands-on guide to mastering GPU programming with Mojo.

Stay in touch​
--------------

#### Get the latest updates

Stay up to date with announcements and releases. We're moving fast over here.

#### Talk to an AI Expert

Connect with our product experts to explore how we can help you deploy and serve AI models with high performance, scalability, and cost-efficiency.

Book a call

Was this page helpful?

Thank you! We'll create more content like this.

Thank you for helping us improve!

😔 What went wrong?

Some code doesn’t work

It includes inaccurate information

It's missing information I need

It was difficult to understand

Other

Submit

*   Set up your project
*   Run offline inference
*   Run inference with an endpoint
*   Keep going
*   Docs
*   Resources
*   Stay in touch

PRODUCT

Mammoth

MAX

Mojo

Editions

Install

QUICK START

Documentation

Tutorials

Model Repo

Recipes

GPU Puzzles

Quickstart

SOLUTIONS

AI Agents

AI Inference

Batch processing

Chatbots

Code Generation

RAG & CAG

Research

DEVELOPERS

Modular Changelog

Tech Talks

Mojo🔥 Playground

Hackathons

AI Coding Assistant

FAQ

CONNECT

Blog

Community

Community Forum

Community Highlights

Report Issue

COMPANY

About Us

Culture

Careers

Talk to Us

@ Copyright - Modular Inc - 2025

Terms,

Privacy

&

Acceptable Use