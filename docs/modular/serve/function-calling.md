Title: Function calling and tool use | Modular

Description: Implement OpenAI-compatible function calling and tool use for agentic GenAI workflows

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

Function calling is a feature available with some large language models (LLMs) that allows them to call external program functions (or tools). This allows the model to interact with external systems to retrieve new data for use as input or execute other tasks. This is a foundational building block for agentic AI applications, in which an LLM can chain together various functions to achieve complex objectives.

Function calling is also called "tool use" because the manner in which you tell the LLM what functions are available is with a `tools` parameter in the request body.

Function calling is enabled by default with MAX, but its availability is model-dependent and will produce valid output only if the model is pretrained to return tool-use responses.

When to use function calling​
-----------------------------

You should use function calling when you want your LLM to:

*   **Fetch data**: Such as fetch weather data, stock prices, or news updates from a database. The model will call a function to get information, and then incorporate that data into its final response.

*   **Perform actions**: Such as modify application states, invoke workflows, or call upon other AI systems. The model will call another tool to perform an action, effectively handing off the request after it determines what the user wants.

How function calling works​
---------------------------

When you send an inference request to a model that supports function calling, you can specify which functions are available to the model using the `tools` body parameter.

The `tools` parameter provides information that allows the LLM to understand:

*   What each function can do
*   How to call each function (the arguments it accepts/requires)

For example, here's a request with the chat completions API that declares an available function named `get_weather()`:

```
from openai import OpenAIdef get_weather(city: str) -> str:    print("Get weather:", city)client = OpenAI(base_url="http://0.0.0.0:8000/v1", api_key="EMPTY")tools = [{    "type": "function",    "function": {        "name": "get_weather",        "description": "Get current temperature for a given location.",        "parameters": {            "type": "object",            "properties": {                "location": {                    "type": "string",                    "description": "City and country e.g. Bogotá, Colombia"                }            },            "required": [                "location"            ],            "additionalProperties": False        },        "strict": True    }}]messages = [  {    "role": "user",    "content": "What's the weather like in San Francisco today?"  }]completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=messages,    tools=tools)
```

```
from openai import OpenAIdef get_weather(city: str) -> str:    print("Get weather:", city)client = OpenAI(base_url="http://0.0.0.0:8000/v1", api_key="EMPTY")tools = [{    "type": "function",    "function": {        "name": "get_weather",        "description": "Get current temperature for a given location.",        "parameters": {            "type": "object",            "properties": {                "location": {                    "type": "string",                    "description": "City and country e.g. Bogotá, Colombia"                }            },            "required": [                "location"            ],            "additionalProperties": False        },        "strict": True    }}]messages = [  {    "role": "user",    "content": "What's the weather like in San Francisco today?"  }]completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=messages,    tools=tools)
```

Let's take a closer look at each parameter shown in the `tools` property:

*   `type`: Currently this is always `function`
*   `function`: Definition of the function
*   `name`: The function name used by the LLM to call it
*   `description`: A function description that helps the LLM understand when to use it
*   `parameters`: Definition of the function parameters
*   `type`: Defines this as an object containing parameters
*   `properties`: Lists all possible function arguments and their types
*   `required`: Specifies which function arguments are required

This format follows the OpenAI function calling specification to specify functions as tools that a model can use.

Using this information, the model will decide whether to call any functions specified in `tools`. In this case, we expect the model to call `get_weather()` and incorporate that information into its final response. So, the initial `completion` response from above includes a `tool_calls` parameter like this:

```
print(completion.choices[0].message.tool_calls)
```

```
print(completion.choices[0].message.tool_calls)
```

```
[ChatCompletionMessageToolCall(  id='call_a175692d9ff54554',  function=Function(    arguments='{      "location": "San Francisco, USA"    }',    name='get_weather'  ),  type='function')]
```

```
[ChatCompletionMessageToolCall(  id='call_a175692d9ff54554',  function=Function(    arguments='{      "location": "San Francisco, USA"    }',    name='get_weather'  ),  type='function')]
```

From here, you must parse the `tool_calls` body and execute the function as appropriate. For example:

```
import jsontool_call = completion.choices[0].message.tool_calls[0]args = json.loads(tool_call.function.arguments)result = get_weather(args["location"])
```

```
import jsontool_call = completion.choices[0].message.tool_calls[0]args = json.loads(tool_call.function.arguments)result = get_weather(args["location"])
```

If the function is designed to **fetch data** for the model, you should call the function and then call the model again with the function results appended as a message using the `tool` role.

If the function is designed to **perform an action**, then you don't need to call the model again.

For detail about how to execute the function and feed the results back to the model, see the OpenAI docs about handling function calls.

The OpenAI function calling spec is compatible with multiple agent frameworks, such as AutoGen, CrewAI, and more.

caution

MAX currently doesn't support streaming with function calling. If using a model that provides streaming, be sure to set the `stream` parameter to `False` when making requests with function calling.

Supported models​
-----------------

Function calling is model-dependent and will produce valid output only if the model is pretrained to return tool use responses. Here are just a few that we've verified work with function calling:

*   `modularai/Llama-3.1-8B-Instruct-GGUF`
*   Meta's Llama 3.1 models & evals collection
*   Meta's Llama 3.2 language models & evals collection

The Meta Llama 3 models are hosted in gated repositories on Hugging Face. You must have a Hugging Face account with access to these repositories and an access token configured in your environment to deploy these models.

Quickstart​
-----------

Here's how you can quickly try the example code from above using a locally-hosted endpoint:

1.  Create a virtual environment and install the `max` CLI:

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
pixi init function-calling \  -c https://conda.modular.com/max-nightly/ -c conda-forge \  && cd function-calling
```

```
pixi init function-calling \  -c https://conda.modular.com/max-nightly/ -c conda-forge \  && cd function-calling
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
uv init function-calling && cd function-calling
```

```
uv init function-calling && cd function-calling
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
mkdir function-calling && cd function-calling
```

```
mkdir function-calling && cd function-calling
```

2.  Create and activate a virtual environment:

```
python3 -m venv .venv/function-calling \  && source .venv/function-calling/bin/activate
```

```
python3 -m venv .venv/function-calling \  && source .venv/function-calling/bin/activate
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
conda create -n function-calling
```

```
conda create -n function-calling
```

4.  Start the virtual environment:

```
conda activate function-calling
```

```
conda activate function-calling
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

2.  Start an endpoint with a model that supports function calling:

```
max serve --model-path=modularai/Llama-3.1-8B-Instruct-GGUF
```

```
max serve --model-path=modularai/Llama-3.1-8B-Instruct-GGUF
```

3.  Wait until you see this message:

```
Server ready on http://0.0.0.0:8000 (Press CTRL+C to quit)
```

```
Server ready on http://0.0.0.0:8000 (Press CTRL+C to quit)
```

Then open a new terminal send a request with the `tools` parameter:

*   Python
*   curl

First install the `openai` API (make sure your current working directory is still the `function-calling` directory):

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
conda install openai
```

```
conda install openai
```

Then, create a program to send a request specifying the available `get_weather()` function:

function-calling.py

```
from openai import OpenAIimport jsondef get_weather(city: str) -> str:    print("Get weather:", city)client = OpenAI(base_url="http://0.0.0.0:8000/v1", api_key="EMPTY")tools = [{    "type": "function",    "function": {        "name": "get_weather",        "description": "Get current temperature for a given location.",        "parameters": {            "type": "object",            "properties": {                "location": {                    "type": "string",                    "description": "City and country e.g. Bogotá, Colombia"                }            },            "required": [                "location"            ],            "additionalProperties": False        },        "strict": True    }}]messages = [  {    "role": "user",    "content": "What's the weather like in San Francisco today?"  }]completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=messages,    tools=tools)tool_call = completion.choices[0].message.tool_calls[0]args = json.loads(tool_call.function.arguments)result = get_weather(args["location"])
```

```
from openai import OpenAIimport jsondef get_weather(city: str) -> str:    print("Get weather:", city)client = OpenAI(base_url="http://0.0.0.0:8000/v1", api_key="EMPTY")tools = [{    "type": "function",    "function": {        "name": "get_weather",        "description": "Get current temperature for a given location.",        "parameters": {            "type": "object",            "properties": {                "location": {                    "type": "string",                    "description": "City and country e.g. Bogotá, Colombia"                }            },            "required": [                "location"            ],            "additionalProperties": False        },        "strict": True    }}]messages = [  {    "role": "user",    "content": "What's the weather like in San Francisco today?"  }]completion = client.chat.completions.create(    model="modularai/Llama-3.1-8B-Instruct-GGUF",    messages=messages,    tools=tools)tool_call = completion.choices[0].message.tool_calls[0]args = json.loads(tool_call.function.arguments)result = get_weather(args["location"])
```

Run it and the `get_weather()` function should print the argument received (make sure you're in the virtual environment—for example, first run `pixi  shell`):

```
python function-calling.py
```

```
python function-calling.py
```

```
Get weather: San Francisco, USA
```

```
Get weather: San Francisco, USA
```

Use the following `curl` command to send a request specifying the available `get_weather()` function:

```
curl -N http://0.0.0.0:8000/v1/chat/completions \-H "Content-Type: application/json" \-d '{    "model": "modularai/Llama-3.1-8B-Instruct-GGUF",    "stream": false,    "messages": [        {"role": "system", "content": "You are a helpful assistant."},        {"role": "user", "content": "What is the weather like in Boston today?"}    ],    "tools": [    {      "type": "function",      "function": {        "name": "get_weather",        "description": "Get the current weather in a given location",        "parameters": {          "type": "object",          "properties": {            "location": {              "type": "string",              "description": "The city and state, e.g. Los Angeles, CA"            }          },          "required": ["location"]        }      }    }  ],  "tool_choice": "auto"}'
```

```
curl -N http://0.0.0.0:8000/v1/chat/completions \-H "Content-Type: application/json" \-d '{    "model": "modularai/Llama-3.1-8B-Instruct-GGUF",    "stream": false,    "messages": [        {"role": "system", "content": "You are a helpful assistant."},        {"role": "user", "content": "What is the weather like in Boston today?"}    ],    "tools": [    {      "type": "function",      "function": {        "name": "get_weather",        "description": "Get the current weather in a given location",        "parameters": {          "type": "object",          "properties": {            "location": {              "type": "string",              "description": "The city and state, e.g. Los Angeles, CA"            }          },          "required": ["location"]        }      }    }  ],  "tool_choice": "auto"}'
```

You should receive a response similar to this:

```
"tool_calls": [  {    "id": "call_ac73df14fe184349",    "type": "function",    "function": {        "name": "get_weather",        "arguments": "{\"location\": \"Boston, MA\"}"    }  }]
```

```
"tool_calls": [  {    "id": "call_ac73df14fe184349",    "type": "function",    "function": {        "name": "get_weather",        "arguments": "{\"location\": \"Boston, MA\"}"    }  }]
```

For a more complete walkthrough of how to handle a `tools_call` response and send the function results back to the LLM as input, see the OpenAI docs about handling function calls.

Next steps​
-----------

Now that you know the basics of function calling, you can get started with MAX on GPUs.

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

*   When to use function calling
*   How function calling works
*   Supported models
*   Quickstart
*   Next steps

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