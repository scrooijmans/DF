# Epicenter & Whispering Architecture and Call Stacks

This document describes Epicenter's tech stack, local-first architecture, and how Epicenter Whispering implements desktop transcription with a shortcut-driven workflow. It highlights overlapping technologies with MudRock's stack and provides concrete examples of how shared libraries are used.

## Epicenter Tech Stack Overview

### Frontend

- **Framework**: **Svelte 5** (using runes: `$state`, `$derived`, `$effect`)
- **TypeScript**: Full type safety
- **State Management**: **TanStack Query** (React Query) for server state
- **Build Tool**: Vite
- **Desktop Framework**: **Tauri** (Rust backend + web frontend)
- **Styling**: Modern CSS (likely Tailwind or similar)

### Backend

- **Language**: **Rust**
- **Desktop Framework**: **Tauri**
- **IPC**: Tauri commands for frontend-backend communication
- **Audio Processing**: **cpal** (Cross-Platform Audio Library) for recording
- **Transcription**: **whisper.cpp** (Rust bindings) for local transcription
- **Error Handling**: `thiserror` + `serde` for error serialization

### Local-First Data Storage

**Core Philosophy**: All data stored locally in plain text and SQLite files

#### Storage Architecture

```
Epicenter Data Folder
├── vault.db (SQLite)          # Main database
├── notes/                      # Plain text notes
│   ├── note1.md
│   └── note2.md
├── transcripts/                # Transcription data
│   └── transcript1.txt
└── .git/                       # Optional Git version control
```

#### Technologies for Local-First

1. **SQLite** (via `better-sqlite3` or `drizzle-orm`)
   - Single-file database
   - No server required
   - ACID transactions
   - Full-text search support

2. **Drizzle ORM**
   - Type-safe SQL queries
   - Schema migrations
   - SQLite-specific optimizations

3. **Plain Text Files**
   - Markdown notes
   - Human-readable
   - Git-friendly
   - Searchable with `ripgrep`

4. **IndexedDB** (Browser)
   - Blob storage for audio recordings
   - Large file support
   - Browser-native persistence

5. **ArkType** (Data Validation)
   - Runtime type validation
   - Schema parsing
   - Type-safe data handling

## Epicenter Whispering Architecture

### Overview

**Epicenter Whispering** is a transcription app built on top of Epicenter that provides:

- Voice-activated transcription
- Desktop shortcut integration
- Local-first storage
- Direct API integration (no middleman servers)

### Tech Stack Overlap with MudRock

| Technology         | Epicenter/Whispering  | MudRock                           | Usage                  |
| ------------------ | --------------------- | --------------------------------- | ---------------------- |
| **Tauri**          | ✅ Desktop wrapper    | ✅ Desktop wrapper                | Native desktop apps    |
| **Svelte 5**       | ✅ Frontend framework | ✅ Frontend framework             | Reactive UI with runes |
| **TypeScript**     | ✅ Type safety        | ✅ Type safety                    | Type-safe development  |
| **Vite**           | ✅ Build tool         | ✅ Build tool                     | Fast builds            |
| **SQLite**         | ✅ Local database     | ✅ Local database (via PowerSync) | Local-first storage    |
| **Drizzle ORM**    | ✅ Type-safe queries  | ⚠️ Kysely (similar)               | Database queries       |
| **IndexedDB**      | ✅ Blob storage       | ✅ Dexie wrapper                  | Browser storage        |
| **TanStack Query** | ✅ State management   | ⚠️ Not used                       | Server state caching   |

## Three-Layer Architecture

### 1. Services Layer (Pure Functions)

**Location**: `apps/app/src/lib/services/`

**Characteristics**:

- Pure functions with no UI dependencies
- Platform abstraction (desktop vs web)
- No reactivity
- Direct database/file system access

**Example: Transcription Service**

```typescript
// services/transcription/openai.ts
export function createOpenAITranscriptionService() {
  return {
    async transcribe(
      audioBlob: Blob,
      options: {
        apiKey: string;
        model: string;
        prompt?: string;
        temperature?: number;
      },
    ): Promise<Result<string, WhisperingError>> {
      // Direct API call - no UI dependencies
      const formData = new FormData();
      formData.append("file", audioBlob);
      formData.append("model", options.model);

      const response = await fetch(
        "https://api.openai.com/v1/audio/transcriptions",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${options.apiKey}`,
          },
          body: formData,
        },
      );

      if (!response.ok) {
        return Err(
          WhisperingErr({
            title: "Transcription Failed",
            description: await response.text(),
          }),
        );
      }

      const data = await response.json();
      return Ok(data.text);
    },
  };
}
```

**Example: Database Service**

```typescript
// services/db.ts
import { Ok, Err, Result } from "@epicenter/result";
import { database } from "./database"; // Drizzle instance

export async function getAllRecordings(): Promise<
  Result<Recording[], DbError>
> {
  try {
    const recordings = await database.recordings.findMany();
    return Ok(recordings);
  } catch (error) {
    return Err(new DbError("Failed to fetch recordings"));
  }
}

export async function createRecording(
  recording: Omit<Recording, "id" | "createdAt">,
): Promise<Result<Recording, DbError>> {
  try {
    const [newRecording] = await database.recordings
      .insert(recording)
      .returning();
    return Ok(newRecording);
  } catch (error) {
    return Err(new DbError("Failed to create recording"));
  }
}
```

### 2. Query Layer (Reactivity + Caching)

**Location**: `apps/app/src/lib/query/`

**Characteristics**:

- Adds reactivity (Svelte 5 runes)
- Caching (TanStack Query)
- Runtime dependency injection
- UI-aware state management

**Example: Recording Queries**

```typescript
// query/recordings.ts
import { defineQuery, defineMutation } from "./query-builder";
import * as services from "../services/db";

export const recordings = {
  // Query: Fetch all recordings
  getAllRecordings: defineQuery({
    queryKey: ["recordings", "all"] as const,
    queryFn: async () => {
      const result = await services.getAllRecordings();
      if (result.error) throw result.error;
      return result.data;
    },
  }),

  // Mutation: Create recording
  createRecording: defineMutation({
    mutationKey: ["recordings", "create"] as const,
    mutationFn: async (recording: Omit<Recording, "id">) => {
      const result = await services.createRecording(recording);
      if (result.error) throw result.error;
      return result.data;
    },
    onSuccess: () => {
      // Invalidate queries to refetch
      queryClient.invalidateQueries({ queryKey: ["recordings"] });
    },
  }),

  // Mutation: Update recording
  updateRecording: defineMutation({
    mutationKey: ["recordings", "update"] as const,
    mutationFn: async (recording: Recording) => {
      const result = await services.updateRecording(recording);
      if (result.error) throw result.error;
      return result.data;
    },
  }),
};
```

**Example: Transcription Mutation**

```typescript
// query/transcription.ts
export const transcription = {
  transcribeRecording: defineMutation({
    resultMutationFn: async (recording: Recording) => {
      // Step 1: Update status
      await recordings.updateRecording.execute({
        ...recording,
        transcriptionStatus: "TRANSCRIBING",
      });

      // Step 2: Perform transcription
      const { data, error } = await transcribeBlob(recording.blob);

      // Step 3: Update with results
      await recordings.updateRecording.execute({
        ...recording,
        transcribedText: data,
        transcriptionStatus: error ? "FAILED" : "DONE",
      });

      return error ? Err(error) : Ok(data);
    },
  }),
};
```

### 3. Frontend Layer (UI Components)

**Location**: `apps/app/src/routes/` and `apps/app/src/lib/components/`

**Characteristics**:

- Svelte 5 components
- Uses Query Layer for data
- Reactive UI updates
- User interactions

**Example: Recording List Component**

```svelte
<!-- routes/recordings/+page.svelte -->
<script lang="ts">
  import { createQuery, createMutation } from '@tanstack/svelte-query';
  import { rpc } from '$lib/query';

  // Use Query Layer
  const recordingsQuery = createQuery(rpc.recordings.getAllRecordings.options);
  const transcribeMutation = createMutation(
    rpc.transcription.transcribeRecording.options
  );

  async function handleTranscribe(recordingId: string) {
    const recording = $recordingsQuery.data?.find(r => r.id === recordingId);
    if (recording) {
      await transcribeMutation.mutateAsync(recording);
    }
  }
</script>

{#if $recordingsQuery.isLoading}
  <p>Loading recordings...</p>
{:else if $recordingsQuery.data}
  {#each $recordingsQuery.data as recording}
    <div class="recording">
      <p>{recording.name}</p>
      <button on:click={() => handleTranscribe(recording.id)}>
        Transcribe
      </button>
    </div>
  {/each}
{/if}
```

## Shortcut → Speak → Text → Desktop Transcription Flow

### Complete Call Stack

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       │ 1. User presses keyboard shortcut
       │    (e.g., Cmd+Shift+Space)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 2. Shortcut handler detects keypress                         │
│    - Global keyboard listener (Tauri or web)                 │
│    - Checks if shortcut matches configured binding           │
│                                                               │
│ 3. Command execution:                                         │
│    rpc.commands.pushToTalk.execute()                         │
│    OR                                                         │
│    rpc.commands.toggleManualRecording.execute()              │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke) or Direct Service Call
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Query Layer (RPC)                                            │
│                                                               │
│ 4. RPC command handler:                                       │
│    - Validates request                                        │
│    - Calls appropriate service                                │
│    - Handles errors                                           │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Service Call
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Services Layer                                               │
│                                                               │
│ 5. Recording Service:                                         │
│    - Platform detection (Tauri vs Web)                       │
│    - Starts audio recording                                   │
│    - Returns recording ID                                     │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri Command (if desktop)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust) - Tauri Command                               │
│                                                               │
│ 6. #[tauri::command]                                         │
│    fn start_recording(                                       │
│      device: String,                                         │
│      output_path: PathBuf                                    │
│    ) -> Result<String, Error>                               │
│                                                               │
│ 7. Audio Recording (cpal):                                   │
│    - Get default audio input device                          │
│    - Configure audio stream                                  │
│    - Start recording to file or memory                       │
│    - Return recording ID                                     │
│                                                               │
│ 8. Store recording metadata:                                 │
│    - Create temporary file path                              │
│    - Initialize WAV writer                                   │
│    - Start audio stream                                      │
│    - Store stream in HashMap for later access                │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return recording ID
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 9. Receive recording ID                                      │
│ 10. Update UI:                                               │
│     - Show recording indicator                               │
│     - Display recording duration                             │
│     - Show "Stop" button                                     │
│                                                               │
│ 11. User speaks...                                           │
│                                                               │
│ 12. User presses shortcut again (or Stop button)            │
│ 13. Command: rpc.commands.stopRecording.execute()           │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 14. #[tauri::command]                                        │
│     fn stop_recording(                                       │
│       recording_id: String                                   │
│     ) -> Result<Vec<u8>, Error>                             │
│                                                               │
│ 15. Stop audio stream:                                       │
│     - Find stream in HashMap                                 │
│     - Stop stream                                            │
│     - Finalize WAV file                                      │
│     - Read audio data into Vec<u8>                           │
│     - Return audio blob                                      │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return audio blob (Vec<u8>)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 16. Receive audio blob                                       │
│ 17. Store in IndexedDB:                                      │
│     - Convert Vec<u8> to Blob                                │
│     - Store in IndexedDB                                     │
│     - Get blob URL for later access                          │
│                                                               │
│ 18. Create recording in database:                            │
│     rpc.recordings.createRecording.execute({                 │
│       blob: audioBlob,                                       │
│       timestamp: new Date(),                                 │
│       transcriptionStatus: 'PENDING'                         │
│     })                                                       │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Database Insert (Drizzle ORM)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Database (SQLite)                                            │
│                                                               │
│ 19. Insert recording:                                        │
│     INSERT INTO recordings (                                 │
│       id, blob_url, timestamp, status                        │
│     ) VALUES (?, ?, ?, ?)                                    │
│                                                               │
│ 20. Return created recording                                 │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return recording
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 21. Auto-transcribe (if enabled):                            │
│     rpc.transcription.transcribeRecording.execute(recording) │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Transcription Service Call
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Services Layer - Transcription                               │
│                                                               │
│ 22. Update status to 'TRANSCRIBING':                         │
│     await recordings.updateRecording.execute({               │
│       ...recording,                                          │
│       transcriptionStatus: 'TRANSCRIBING'                    │
│     })                                                       │
│                                                               │
│ 23. Transcribe audio:                                        │
│     - Get transcription service (OpenAI, Groq, Local)        │
│     - Fetch audio blob from IndexedDB                        │
│     - Send to transcription API                              │
│     - OR use local whisper.cpp (Rust backend)                │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Option A: External API (OpenAI/Groq)
       │ Option B: Local Transcription (Tauri)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Option A: External API                                       │
│                                                               │
│ 24. POST to transcription API:                               │
│     - https://api.openai.com/v1/audio/transcriptions         │
│     - OR https://api.groq.com/v1/audio/transcriptions        │
│     - Send audio blob directly                               │
│     - Use user's API key (no middleman)                      │
│                                                               │
│ 25. Receive transcription text                               │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ OR
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Option B: Local Transcription (Rust Backend)                │
│                                                               │
│ 24. #[tauri::command]                                        │
│     fn transcribe_audio(                                     │
│       audio_path: PathBuf,                                   │
│       model_path: PathBuf                                    │
│     ) -> Result<String, WhisperCppError>                    │
│                                                               │
│ 25. Load Whisper model:                                      │
│     - Load whisper.cpp model                                 │
│     - Initialize context                                     │
│                                                               │
│ 26. Process audio:                                           │
│     - Convert audio to format whisper expects                │
│     - Run inference                                          │
│     - Extract text segments                                  │
│                                                               │
│ 27. Return transcription text                                │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return transcription text
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 28. Update recording with transcription:                     │
│     await recordings.updateRecording.execute({               │
│       ...recording,                                          │
│       transcribedText: transcriptionText,                    │
│       transcriptionStatus: 'DONE'                            │
│     })                                                       │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Database Update
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Database (SQLite)                                            │
│                                                               │
│ 29. UPDATE recordings SET                                   │
│     transcribed_text = ?,                                    │
│     transcription_status = 'DONE'                            │
│     WHERE id = ?                                             │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return updated recording
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (Svelte 5)                                          │
│                                                               │
│ 30. Display transcription:                                   │
│     - Show text in UI                                        │
│     - Copy to clipboard (if configured)                      │
│     - Insert into active application (if configured)         │
│                                                               │
│ 31. Desktop Integration (Tauri):                             │
│     - Get active window                                      │
│     - Send text to clipboard                                 │
│     - OR simulate typing (accessibility API)                 │
│     - OR insert at cursor position                           │
└─────────────────────────────────────────────────────────────┘
```

## Key Implementation Details

### 1. Platform Detection for Code Sharing

**Pattern**: Build-time dependency injection

```typescript
// Platform detection
export const ClipboardServiceLive = window.__TAURI_INTERNALS__
  ? createClipboardServiceDesktop() // Tauri APIs
  : createClipboardServiceWeb(); // Browser APIs

// Usage
const clipboard = ClipboardServiceLive;
await clipboard.writeText(transcriptionText);
```

**MudRock Overlap**: Same pattern can be used for Tauri vs web differences

### 2. RPC Command Pattern

**Pattern**: Abstracted command execution

```typescript
// query/commands.ts
export const commands = {
  pushToTalk: {
    execute: async () => {
      // Implementation
    },
  },
  toggleManualRecording: {
    execute: async () => {
      // Implementation
    },
  },
};

// Usage
await rpc.commands.pushToTalk.execute();
```

**MudRock Overlap**: Similar to your Tauri command pattern, but with an RPC abstraction layer

### 3. Result Type for Error Handling

**Pattern**: Functional error handling

```typescript
import { Ok, Err, Result } from "@epicenter/result";

async function transcribe(
  blob: Blob,
): Promise<Result<string, TranscriptionError>> {
  try {
    const text = await api.transcribe(blob);
    return Ok(text);
  } catch (error) {
    return Err(new TranscriptionError(error.message));
  }
}
```

**MudRock Overlap**: Similar to Rust's `Result<T, E>` pattern, but in TypeScript

### 4. Database Schema with Drizzle ORM

**Pattern**: Type-safe SQL queries

```typescript
// schema.ts
import { sqliteTable, text, integer, blob } from "drizzle-orm/sqlite-core";

export const recordings = sqliteTable("recordings", {
  id: text("id").primaryKey(),
  blobUrl: text("blob_url").notNull(), // IndexedDB URL
  transcribedText: text("transcribed_text"),
  transcriptionStatus: text("transcription_status").notNull(),
  timestamp: integer("timestamp", { mode: "timestamp" }).notNull(),
});

// Usage
const allRecordings = await db.select().from(recordings);
```

**MudRock Overlap**: Similar to your Kysely usage, but Drizzle is more modern

### 5. IndexedDB for Blob Storage

**Pattern**: Store large files in browser storage

```typescript
// Store audio blob
const blobUrl = await storeBlobInIndexedDB(audioBlob);
// Returns: "blob:indexeddb://recordings/abc123"

// Retrieve later
const blob = await getBlobFromIndexedDB(blobUrl);
```

**MudRock Overlap**: You use Dexie (IndexedDB wrapper), same concept

### 6. Tauri Command for Audio Recording

**Pattern**: Native audio access via Rust

```rust
// src-tauri/src/main.rs
use cpal::traits::{DeviceTrait, HostTrait};
use std::collections::HashMap;
use std::sync::Mutex;

static ACTIVE_RECORDINGS: Mutex<HashMap<String, cpal::Stream>> =
    Mutex::new(HashMap::new());

#[tauri::command]
fn start_recording(
    device: String,
    output_path: PathBuf
) -> Result<String, String> {
    let host = cpal::default_host();
    let device = host.input_devices()?
        .find(|d| d.name().ok() == Some(device.clone()))
        .ok_or("Device not found")?;

    let config = device.default_input_config()?;
    let recording_id = nanoid::nanoid!();

    // Start recording in async task
    tauri::async_runtime::spawn(async move {
        let mut wav_writer = WavWriter::new(&output_path, &config)?;

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                wav_writer.write_samples(data);
            },
            |err| eprintln!("Stream error: {}", err),
            None
        )?;

        stream.play()?;
        ACTIVE_RECORDINGS.lock().unwrap()
            .insert(recording_id.clone(), stream);

        Ok::<(), Box<dyn std::error::Error>>(());
    });

    Ok(recording_id)
}

#[tauri::command]
fn stop_recording(recording_id: String) -> Result<Vec<u8>, String> {
    let mut recordings = ACTIVE_RECORDINGS.lock().unwrap();
    let stream = recordings.remove(&recording_id)
        .ok_or("Recording not found")?;

    drop(stream); // Stop the stream

    // Read the WAV file
    let audio_data = std::fs::read(&output_path)?;
    Ok(audio_data)
}
```

**MudRock Overlap**: Similar to your Tauri commands for UDFs, but for audio

### 7. Local Transcription with whisper.cpp

**Pattern**: On-device AI inference

```rust
// src-tauri/src/main.rs
use whisper_rs::{FullContext, WhisperContext, WhisperContextParameters};

#[tauri::command]
fn transcribe_audio_local(
    audio_path: PathBuf,
    model_path: PathBuf
) -> Result<String, WhisperCppError> {
    // Load model
    let ctx = WhisperContext::new_with_params(
        &model_path.to_string_lossy(),
        WhisperContextParameters::default()
    ).map_err(|e| WhisperCppError::ModelLoadError {
        message: format!("Failed to load model: {}", e),
    })?;

    // Load audio
    let audio_data = load_audio_file(&audio_path)?;

    // Create context
    let mut state = ctx.create_state()
        .map_err(|e| WhisperCppError::ModelLoadError {
            message: format!("Failed to create state: {}", e),
        })?;

    // Run inference
    state.full(&ctx, &audio_data[..])
        .map_err(|e| WhisperCppError::TranscriptionError {
            message: format!("Transcription failed: {}", e),
        })?;

    // Extract text
    let num_segments = state.full_n_segments();
    let mut text = String::new();

    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)
            .map_err(|e| WhisperCppError::TranscriptionError {
                message: format!("Failed to get segment: {}", e),
            })?;
        text.push_str(&segment);
    }

    Ok(text)
}
```

**MudRock Overlap**: Similar pattern for running Rust ML models (like your UDFs)

### 8. Desktop Integration (Clipboard/Insertion)

**Pattern**: System-level text insertion

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn insert_text_at_cursor(text: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Use macOS Accessibility API
        // (Simplified - actual implementation is more complex)
        use core_foundation::string::CFString;
        use core_foundation::dictionary::CFDictionary;

        // Get active application
        // Simulate typing
        // Insert text
    }

    #[cfg(target_os = "windows")]
    {
        // Use Windows API
        // Send input events
    }

    #[cfg(target_os = "linux")]
    {
        // Use X11/Wayland APIs
        // Send keyboard events
    }

    Ok(())
}
```

**MudRock Overlap**: Similar to your Tauri plugins for native dialogs

## Local-First Architecture Benefits

### 1. **No Middleman Servers**

- Audio sent directly to user's API key
- No data collection
- User controls their data

### 2. **Offline Support**

- Local transcription (whisper.cpp)
- All data stored locally
- Works without internet

### 3. **Privacy**

- Recordings stay on device
- IndexedDB for blob storage
- SQLite for metadata
- Optional Git version control

### 4. **Performance**

- No network latency for local operations
- Fast database queries (SQLite)
- Instant UI updates

### 5. **Data Ownership**

- Plain text files (human-readable)
- SQLite database (portable)
- Easy backups
- Version control friendly

## Comparison with MudRock Architecture

### Similarities

1. **Tauri Desktop Apps**: Both use Tauri for native desktop experience
2. **Svelte 5**: Both use Svelte 5 with runes
3. **TypeScript**: Both use TypeScript for type safety
4. **Local-First**: Both prioritize local data storage
5. **SQLite**: Both use SQLite for local database
6. **IndexedDB**: Both use IndexedDB for blob storage (MudRock via Dexie)

### Differences

1. **State Management**:
   - Epicenter: TanStack Query
   - MudRock: Svelte stores + Automerge

2. **Database ORM**:
   - Epicenter: Drizzle ORM
   - MudRock: Kysely

3. **Sync Strategy**:
   - Epicenter: Plain text + Git (optional)
   - MudRock: Automerge CRDT + PowerSync

4. **Backend Focus**:
   - Epicenter: Audio processing, transcription
   - MudRock: Geoscience data, analytics

## Key Takeaways for MudRock

### 1. **Three-Layer Architecture**

- Services (pure functions)
- Query Layer (reactivity + caching)
- Frontend (UI components)

### 2. **Platform Detection Pattern**

```typescript
const service = window.__TAURI_INTERNALS__ ? desktopImpl() : webImpl();
```

### 3. **RPC Command Pattern**

- Abstract commands behind RPC interface
- Consistent execution pattern
- Easy to test and mock

### 4. **Result Type Pattern**

- Functional error handling
- Type-safe error propagation
- Similar to Rust's `Result<T, E>`

### 5. **Local-First Storage**

- SQLite for metadata
- IndexedDB for blobs
- Plain text for human-readable data

### 6. **Tauri Command Best Practices**

- Use `async_runtime::spawn` for long operations
- Return `Result<T, E>` for error handling
- Use `serde` for serialization
- Emit events for progress updates

This architecture enables Epicenter Whispering to provide a seamless, local-first transcription experience with desktop integration, cutting out middleman servers and giving users full control over their data.
