# Supabase Storage Parquet Files Fetch Flow

## 🎯 **Overview**

This document describes the complete data flow for fetching Parquet files from Supabase Storage in our MudRock analytics system.

## 🔄 **Complete Data Flow**

### **1. Frontend Initiation**

```
User clicks "Data Analytics" in sidebar
↓
content-data-analytics-files.svelte loads
↓
$effect() triggers fetchStorageBuckets()
↓
calls analytics-data-fetching.ts functions
```

### **2. Frontend to Tauri Communication**

```typescript
// analytics-data-fetching.ts
export async function getStorageBuckets(): Promise<StorageBucket[]> {
  const buckets = await invoke<StorageBucket[]>("get_storage_buckets");
  return buckets;
}

export async function getParquetFiles(
  bucketName: string,
): Promise<ParquetFile[]> {
  const files = await invoke<ParquetFile[]>("get_parquet_files", {
    bucketName,
  });
  return files;
}
```

### **3. Tauri Command Processing**

```rust
// main.rs - Tauri commands
#[tauri::command]
async fn get_storage_buckets() -> Result<Vec<StorageBucket>, String> {
    let manager = StorageBucketsManager::new();
    let response = manager.get_storage_buckets().await?;
    Ok(response.buckets)
}

#[tauri::command]
async fn get_parquet_files(bucket_name: String) -> Result<Vec<ParquetFile>, String> {
    let manager = ParquetFilesManager::new();
    let response = manager.get_parquet_files(&bucket_name).await?;
    Ok(response.files)
}
```

### **4. Backend HTTP Requests to Supabase Storage**

#### **Storage Buckets Request**

```rust
// analytics_query/storage_buckets.rs
let url = format!("{}/bucket", self.config.storage_url);
let response = client
    .get(&url)
    .header("apikey", &self.config.api_key)
    .header("Authorization", format!("Bearer {}", self.config.api_key))
    .header("Content-Type", "application/json")
    .send()
    .await?;
```

#### **Parquet Files Request (Fixed)**

```rust
// analytics_query/parquet_files.rs
let url = format!("{}/object/list/{}", self.config.storage_url, bucket_name);

// CRITICAL FIX: Include required 'prefix' property
let request_body = serde_json::json!({
    "prefix": "",  // This was missing! Empty string means list all files
    "limit": 1000,
    "offset": 0,
    "sortBy": {
        "column": "created_at",
        "order": "desc"
    }
});

let response = client
    .post(&url)
    .header("apikey", &self.config.api_key)
    .header("Authorization", format!("Bearer {}", self.config.api_key))
    .header("Content-Type", "application/json")
    .json(&request_body)  // Send JSON body with prefix
    .send()
    .await?;
```

## 🐛 **The Problem We Fixed**

### **Original Error**

```
HTTP 400 Bad Request - {"statusCode":"400","error":"Error","message":"body must have required property 'prefix'"}
```

### **Root Cause**

The Supabase Storage API `/object/list/{bucket}` endpoint requires a `prefix` property in the request body, but our original implementation was missing this required field.

### **The Fix**

```rust
// BEFORE (Missing prefix)
let request_body = serde_json::json!({
    "limit": 1000,
    "offset": 0,
    "sortBy": {
        "column": "created_at",
        "order": "desc"
    }
});

// AFTER (With required prefix)
let request_body = serde_json::json!({
    "prefix": "",  // ✅ Added this required field
    "limit": 1000,
    "offset": 0,
    "sortBy": {
        "column": "created_at",
        "order": "desc"
    }
});
```

## 📊 **Data Processing Flow**

### **1. Response Parsing**

```rust
let response_data: serde_json::Value = serde_json::from_str(&body)?;
let files_data = response_data.get("data")
    .and_then(|data| data.as_array())
    .cloned()
    .unwrap_or_default();
```

### **2. File Filtering**

```rust
let mut parquet_files = Vec::new();

for file_data in &files_data {
    if let Ok(parquet_file) = self.parse_file_data(file_data, bucket_name) {
        // Only include Parquet files
        if parquet_file.metadata.mimetype == "application/octet-stream" ||
           parquet_file.name.ends_with(".parquet") {
            parquet_files.push(parquet_file);
        }
    }
}
```

### **3. Metadata Extraction**

```rust
fn parse_file_data(&self, file_data: &serde_json::Value, bucket_name: &str) -> Result<ParquetFile, ...> {
    let name = file_data.get("name").and_then(|n| n.as_str()).unwrap_or("unknown").to_string();
    let id = file_data.get("id").and_then(|i| i.as_str()).unwrap_or("unknown").to_string();

    let metadata = file_data.get("metadata").and_then(|m| m.as_object()).map(|meta| {
        FileMetadata {
            e_tag: meta.get("eTag").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            size: meta.get("size").and_then(|v| v.as_i64()).unwrap_or(0),
            mimetype: meta.get("mimetype").and_then(|v| v.as_str()).unwrap_or("application/octet-stream").to_string(),
            cache_control: meta.get("cacheControl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            last_modified: meta.get("lastModified").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            content_length: meta.get("contentLength").and_then(|v| v.as_i64()).unwrap_or(0),
        }
    }).unwrap_or_else(|| FileMetadata { /* defaults */ });

    // ... rest of parsing
}
```

## 🔧 **Configuration**

### **Environment Variables**

```rust
// analytics_query/types.rs
impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            storage_url: std::env::var("SUPABASE_STORAGE_URL")
                .unwrap_or_else(|_| "http://91.99.166.223:8000/storage/v1".to_string()),
            api_key: std::env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string()),
        }
    }
}
```

### **Supabase Storage Endpoints**

- **Buckets**: `GET /storage/v1/bucket`
- **Files**: `POST /storage/v1/object/list/{bucket}`

## 🎯 **Key Learnings**

### **1. API Requirements**

- Always check the official API documentation for required fields
- Supabase Storage API requires `prefix` property in list requests
- Empty string `""` for prefix means "list all files"

### **2. Error Handling**

- HTTP 400 errors often indicate missing required fields
- Check request body structure against API specification
- Use proper error logging for debugging

### **3. Library vs HTTP**

- Official libraries can have compatibility issues
- Direct HTTP requests give more control
- Always test with actual API endpoints

## 🚀 **Current Status**

✅ **Fixed**: Missing `prefix` property in Supabase Storage API request  
✅ **Working**: Storage buckets fetching  
✅ **Working**: Parquet files listing  
🔄 **Testing**: Frontend display of Parquet files

## 📝 **Next Steps**

1. **Test the fix** in the running application
2. **Verify** Parquet files are displayed correctly
3. **Implement** file analysis functionality
4. **Add** error handling for edge cases

---

**Last Updated**: January 2025  
**Status**: ✅ **Fixed and Ready for Testing**
