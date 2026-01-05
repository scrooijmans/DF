# Codebase to LSIF to Qdrant Functions Collection Implementation

## Overview

This document describes how we implemented the conversion of our Rust codebase to a searchable functions collection in Qdrant, following the approach outlined in the [Qdrant Search Through Codebase tutorial](docs/qdrant/search-through-codebase.md). The process involves three main steps:

1. **LSIF Generation**: Generate Language Server Index Format (LSIF) data from our Rust codebase
2. **LSIF to JSON Conversion**: Parse LSIF data to extract function metadata and code chunks
3. **Qdrant Ingestion**: Convert the extracted data to vectors and ingest into Qdrant

## 🔄 **Complete Workflow Overview**

Based on the Qdrant tutorial inspiration, here's our **fully automated** workflow:

### **Step 1: Automatic LSIF Generation (GitHub Actions)**
```bash
# GitHub Actions automatically runs on code changes
# Uses rust-analyzer to parse the entire codebase
# Generates mudrock-codebase.lsif with code intelligence data
```

### **Step 2: Automatic Processing (GitHub Actions)**
```bash
# Automatically downloads LSIF artifact from previous workflow
# Extracts function metadata using Python scripts
# Output: mudrock-functions-metadata.jsonl (auto-committed to repo)
```

### **Step 3: Local Ingestion (One Command)**
```bash
# Processed files are already in your repository
# Run one command to ingest to Qdrant
# Output: mudrock-functions collection in Qdrant
```

### **Step 4: Semantic Search (Application)**
```bash
# Query the collection with natural language
# "Find functions that convert slowness to velocity"
# Returns relevant code snippets with similarity scores
```

**Key Innovation**: We've **fully automated** the pipeline so that code changes automatically trigger LSIF generation, processing, and metadata extraction. Only the final Qdrant ingestion requires a single local command.

## 🚀 **Automated Workflow**

### **GitHub Actions Automation**

We use GitHub Actions to automatically generate LSIF data on code changes:

```yaml
# .github/workflows/lsif.yml
name: Generate LSIF for Codebase Analysis

on:
  # Manual trigger
  workflow_dispatch:
  
  # Automatic triggers
  push:
    branches: [ main, master ]
    paths:
      - 'crates/**'
      - 'src/**'
      - 'Cargo.toml'
      - 'Cargo.lock'
  
  # Weekly scheduled update
  schedule:
    - cron: '0 2 * * 1'  # Every Monday at 2 AM UTC
```

**Key Features:**
- **Automatic triggers**: Runs on pushes to main/master branch
- **Scheduled updates**: Weekly regeneration to keep data current
- **Manual trigger**: Available via GitHub Actions UI
- **Better naming**: Uses `mudrock-codebase.lsif` instead of `dump.lsif`
- **Full workspace**: Generates LSIF for entire codebase, not just one crate

### **Local Automation Scripts**

We provide automation scripts for local development:

#### **Windows Batch Script**
```bash
# Basic automation
scripts\update_lsif.bat download    # Download from GitHub Actions
scripts\update_lsif.bat extract     # Extract metadata
scripts\update_lsif.bat ingest      # Ingest to Qdrant
scripts\update_lsif.bat all         # Run complete workflow
```

#### **PowerShell Script (Advanced)**
```powershell
# Advanced automation with GitHub CLI integration
.\scripts\update_lsif.ps1 download  # Automatic download via GitHub CLI
.\scripts\update_lsif.ps1 extract   # Extract metadata
.\scripts\update_lsif.ps1 ingest    # Ingest to Qdrant
.\scripts\update_lsif.ps1 all       # Complete workflow
```

## 📁 **File Organization**

### **LSIF Output Directory**
```
lsif-output/
├── README.md                           # Documentation
├── mudrock-codebase.lsif              # Main LSIF file
├── mudrock-functions-metadata.jsonl   # Extracted metadata
└── rust_functions_metadata.json       # Alternative format
```

### **Scripts Directory**
```
scripts/
├── update_lsif.bat                    # Windows automation
├── update_lsif.ps1                    # PowerShell automation
└── python/
    ├── extract_rust_functions_to_qdrant.py
    ├── ingest_functions_to_qdrant.py
    └── test_qdrant_search.py          # Test script
```

## 🔧 **Implementation Details**

### 1. LSIF Generation (GitHub Actions)

The updated workflow generates LSIF data for the entire codebase:

```yaml
- name: Generate LSIF for entire codebase
  uses: sourcegraph/lsif-rust-action@main
  with:
    project_root: .  # Generate for entire workspace
    target: target/lsif

- name: Rename LSIF file
  run: |
    mv dump.lsif mudrock-codebase.lsif
    echo "Generated LSIF file: mudrock-codebase.lsif"
```

**Key Improvements:**
- **Full workspace coverage**: Generates LSIF for all crates
- **Better naming**: Uses descriptive filename
- **Artifact retention**: 30-day retention for artifacts
- **Metadata summary**: Provides file size and line count
- **PR comments**: Automatically comments on PRs with LSIF info

### 2. LSIF to JSON Conversion

We implemented a Python script (`scripts/extract_rust_functions_to_qdrant.py`) that follows the Qdrant tutorial approach to parse LSIF data:

#### LSIF Parsing Process

```python
def extract_code_chunks_from_lsif(lsif_path=LSIF_INPUT, output_jsonl_path=LSIF_OUTPUT):
    """
    Parse LSIF and extract function/method/struct/trait/enum chunks into JSONL, 
    following edges as in the Qdrant tutorial.
    """
    # 1. Collect all vertices and edges from LSIF
    id_to_vertex = {}
    id_to_document = {}
    id_to_range = {}
    id_to_hover = {}
    id_to_resultset = {}
    id_to_moniker = {}
    id_to_edge = collections.defaultdict(list)
    
    # 2. Parse LSIF file line by line
    with open(lsif_path, 'r', encoding='utf-8') as f:
        for line in f:
            obj = json.loads(line)
            if obj.get('type') == 'vertex':
                # Store different types of vertices
                label = obj.get('label')
                if label == 'document':
                    id_to_document[obj['id']] = obj
                elif label == 'range':
                    id_to_range[obj['id']] = obj
                elif label == 'hoverResult':
                    id_to_hover[obj['id']] = obj
                # ... other vertex types
            elif obj.get('type') == 'edge':
                # Store edges for navigation
                id_to_edge[obj.get('outV')].append(obj)
    
    # 3. Extract code chunks by following edges
    chunks = []
    for rid, r in id_to_range.items():
        # Find associated resultSet via 'next' edge
        result_set_id = None
        for edge in id_to_edge.get(rid, []):
            if edge.get('label') == 'next':
                result_set_id = edge.get('inV')
                break
        
        # Find hoverResult via 'textDocument/hover' edge from resultSet
        hover_id = None
        if result_set_id:
            for edge in id_to_edge.get(result_set_id, []):
                if edge.get('label') == 'textDocument/hover':
                    hover_id = edge.get('inV')
                    break
        
        # 4. Extract signature and docstring from hover contents
        docstring = ''
        signature = ''
        name = ''
        if hover_id:
            hover = id_to_hover.get(hover_id)
            if hover:
                contents = hover.get('result', {}).get('contents')
                # Parse markdown content to extract signature and docstring
                # ... parsing logic for different content formats
        
        # 5. Extract name from signature using regex
        if signature:
            m = re.match(r'.*fn\s+(\w+)', signature)
            if m:
                name = m.group(1)
            # ... similar patterns for struct, enum, trait
        
        # 6. Get file and line information
        doc_id = r.get('document')
        doc = id_to_document.get(doc_id, {})
        file_path = doc.get('uri', '').replace('file://', '')
        line_from = r.get('start', {}).get('line')
        line_to = r.get('end', {}).get('line')
        
        # 7. Extract code snippet from source file
        snippet = ''
        if file_path and line_from is not None and line_to is not None:
            try:
                with open(file_path, 'r', encoding='utf-8') as src:
                    lines = src.readlines()
                    snippet = ''.join(lines[line_from:line_to+1])
            except Exception:
                snippet = ''
        
        # 8. Create structured chunk
        if signature.strip().startswith(('fn ', 'struct ', 'enum ', 'trait ')):
            chunk = {
                'name': name,
                'signature': signature,
                'code_type': signature.strip().split(' ', 1)[0].title(),
                'docstring': docstring,
                'line': line_from,
                'line_from': line_from,
                'line_to': line_to,
                'context': {
                    'module': None,  # TODO: infer from path
                    'file_path': file_path,
                    'file_name': os.path.basename(file_path) if file_path else None,
                    'struct_name': None,  # TODO: infer if possible
                    'snippet': snippet
                }
            }
            chunks.append(chunk)
    
    # 9. Write to JSONL format
    with open(output_jsonl_path, 'w', encoding='utf-8') as out:
        for chunk in chunks:
            out.write(json.dumps(chunk) + '\n')
    
    return chunks
```

#### Text Conversion (Following Qdrant Tutorial)

We implemented the `textify_chunk` function that converts code chunks to natural language text, following the Qdrant tutorial approach:

```python
def textify_chunk(chunk: Dict[str, Any]) -> str:
    # Convert camel case / snake case to human readable form
    name = inflection.humanize(inflection.underscore(chunk.get("name", "")))
    signature = inflection.humanize(inflection.underscore(chunk.get("signature", "")))
    
    # Extract docstring
    docstring = chunk.get("docstring", "")
    docstring = f"that does {docstring} " if docstring else ""
    
    # Build context string
    context = chunk.get("context", {})
    context_str = f"module {context.get('module', '')} file {context.get('file_name', '')}"
    if context.get("struct_name"):
        struct_name = inflection.humanize(inflection.underscore(context["struct_name"]))
        context_str = f"defined in struct {struct_name} {context_str}"
    
    # Combine all parts
    text_representation = (
        f"{chunk.get('code_type', '')} {name} "
        f"{docstring}"
        f"defined as {signature} "
        f"{context_str}"
    )
    
    # Remove special characters and tokenize
    tokens = re.split(r"\W", text_representation)
    tokens = filter(lambda x: x, tokens)
    return " ".join(tokens)
```

### 3. Qdrant Ingestion

We implemented a separate script (`scripts/ingest_functions_to_qdrant.py`) to ingest the extracted function metadata into Qdrant:

#### Collection Creation

```python
def ingest_to_qdrant(functions, collection_name, qdrant_url, qdrant_api_key):
    client = QdrantClient(qdrant_url, api_key=qdrant_api_key)
    
    # Create collection with dual vector configuration
    client.recreate_collection(
        collection_name=collection_name,
        vectors_config={
            "text": models.VectorParams(
                size=client.get_embedding_size(model_name="sentence-transformers/all-MiniLM-L6-v2"),
                distance=models.Distance.COSINE,
            ),
            "code": models.VectorParams(
                size=client.get_embedding_size(model_name="jinaai/jina-embeddings-v2-base-code"),
                distance=models.Distance.COSINE,
            ),
        },
    )
```

#### Dual Embedding Strategy

Following the Qdrant tutorial, we use two different embedding models:

1. **Text Embeddings** (`sentence-transformers/all-MiniLM-L6-v2`):
   - For natural language queries about function purpose and behavior
   - Uses the `textify` conversion for human-readable text

2. **Code Embeddings** (`jinaai/jina-embeddings-v2-base-code`):
   - For code-specific queries and similarity
   - Uses raw code snippets and function signatures

#### Point Creation and Upload

```python
    points = []
    for func in functions:
        # Convert to natural language text
        text = textify_function_metadata(func)
        
        # Prepare code content for code embeddings
        code = func.get("description", "") + " " + func.get("name", "")
        
        points.append(
            models.PointStruct(
                id=uuid.uuid4().hex,
                vector={
                    "text": models.Document(
                        text=text, model="sentence-transformers/all-MiniLM-L6-v2"
                    ),
                    "code": models.Document(
                        text=code, model="jinaai/jina-embeddings-v2-base-code"
                    ),
                },
                payload=func,  # Store full metadata as payload
            )
        )
    
    # Upload with batching for performance
    client.upload_points(collection_name, points=points, batch_size=64)
```

## 🧪 **Testing and Validation**

### **Test Script**

We provide a comprehensive test script (`scripts/python/test_qdrant_search.py`) to validate the Qdrant search functionality:

```bash
# Test basic functionality
python scripts/python/test_qdrant_search.py

# Test specific query
python scripts/python/test_qdrant_search.py --query "slowness velocity"

# Test with custom collection
python scripts/python/test_qdrant_search.py --collection mudrock-functions
```

**Test Features:**
- **Collection validation**: Verifies collection exists and has data
- **Text search**: Tests natural language queries
- **Code search**: Tests code-specific queries with filters
- **Filtered search**: Tests filtering by code type (Function, Struct, etc.)
- **Geoscience queries**: Tests domain-specific queries

## 📋 **Usage Workflow**

### **Complete Automated Workflow**

1. **Automatic LSIF Generation**:
   ```bash
   # GitHub Actions automatically generates LSIF on code changes
   # No manual intervention required
   ```

2. **Download and Process**:
   ```bash
   # Windows
   scripts\update_lsif.bat all
   
   # PowerShell (with GitHub CLI)
   .\scripts\update_lsif.ps1 all
   ```

3. **Test Results**:
   ```bash
   python scripts/python/test_qdrant_search.py
   ```

### **Manual Workflow**

1. **Generate LSIF Data**:
   ```bash
   # Trigger GitHub Action manually or on code changes
   # Downloads mudrock-codebase.lsif artifact
   ```

2. **Extract Function Metadata**:
   ```bash
   # Extract from LSIF data
   python scripts/extract_rust_functions_to_qdrant.py --mode lsif --lsif lsif-output/mudrock-codebase.lsif --output lsif-output/mudrock-functions-metadata.jsonl

   # Or extract using regex parsing (alternative approach)
   python scripts/extract_rust_functions_to_qdrant.py --mode regex --output lsif-output/rust_functions_metadata.json
   ```

3. **Ingest to Qdrant**:
   ```bash
   # Ingest extracted metadata to Qdrant
   python scripts/ingest_functions_to_qdrant.py --input lsif-output/mudrock-functions-metadata.jsonl --collection mudrock-functions
   ```

## 📊 **Collection Schema**

The `mudrock-functions` collection in Qdrant contains:

- **Vectors**: Dual embeddings (text + code)
- **Payload**: Full function metadata including:
  - `name`: Function name
  - `signature`: Rust function signature
  - `code_type`: Function/Struct/Enum/Trait
  - `docstring`: Documentation string
  - `line_from`/`line_to`: Source code location
  - `context`: File path, module info, code snippet
  - `category`: High-level category (petrophysics, geophysics, etc.)

## 🔍 **Search Capabilities**

Once ingested, the collection supports:

1. **Natural Language Queries**: "Find functions that convert slowness to velocity"
2. **Code Similarity**: Find functions with similar implementations
3. **Hybrid Search**: Combine both text and code embeddings for better results
4. **Filtering**: By category, module, file path, etc.

## 🔗 **Integration with MudRock**

This functions collection integrates with our broader Qdrant-first architecture:

- **Function Registry**: Enables semantic search over scientific functions
- **Code Intelligence**: Provides context-aware function discovery
- **Orchestrator Integration**: Can be queried by the LangChain-style orchestrator
- **Dynamic Invocation**: Functions found via search can be dynamically invoked

## 🚀 **Future Enhancements**

1. **Module Path Inference**: Better extraction of module paths from file structure
2. **Struct Context**: Identify functions within struct implementations
3. **Parameter Parsing**: Extract and index function parameters for better search
4. **Example Extraction**: Parse and index usage examples from documentation
5. **Cross-Reference Support**: Link related functions and types
6. **Automated Updates**: Trigger LSIF generation on code changes
7. **Performance Optimization**: Parallel processing for large codebases
8. **Incremental Updates**: Only update changed functions instead of full regeneration

## 📝 **Cron Jobs vs GitHub Actions**

### **What are Cron Jobs?**

Cron jobs are **scheduled tasks** that run at specified intervals on Unix/Linux systems. They are:
- **Time-based**: Run at specific times (daily, weekly, etc.)
- **System-level**: Run on the local machine/server
- **Simple**: Basic command execution
- **Limited**: No complex workflows or integrations

### **GitHub Actions vs Cron Jobs**

For our LSIF automation, **GitHub Actions is better** because:

1. **Code-triggered**: Runs when code changes (not just time-based)
2. **Cloud-based**: No need for local server
3. **Rich ecosystem**: Integrates with GitHub API, notifications, etc.
4. **Artifact management**: Automatic storage and sharing of generated files
5. **Collaboration**: Team members can trigger and monitor
6. **Scalability**: No local resource constraints

### **When to Use Each**

**Use GitHub Actions for:**
- Code analysis and generation
- Build artifacts
- Documentation updates
- Team collaboration

**Use Cron Jobs for:**
- Local backups
- System maintenance
- Simple file operations
- When you need local execution
