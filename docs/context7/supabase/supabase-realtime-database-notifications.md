# Supabase Realtime: Database Update Notifications

## Overview

Supabase Realtime enables real-time notifications when database changes occur. It uses PostgreSQL's Change Data Capture (CDC) replication, WebSocket connections, and the Phoenix Framework (Elixir) to deliver instant updates to connected clients.

## Architecture

### Core Components

1. **PostgreSQL Replication**: Uses logical replication to capture database changes
2. **Realtime Server**: Elixir/Phoenix cluster that manages WebSocket connections
3. **WebSocket Connections**: Persistent connections between clients and Realtime servers
4. **Channels**: Logical groupings for subscriptions (table-specific or broadcast)

### How It Works

```
PostgreSQL Database
    ↓ (Logical Replication)
Replication Slot (supabase_realtime_replication_slot)
    ↓ (Change Data Capture)
Realtime Server (Elixir/Phoenix)
    ↓ (WebSocket)
Connected Clients
```

## Setup and Configuration

### 1. Enable Realtime for Tables

Realtime is **disabled by default** for new projects. Enable it by adding tables to the replication publication:

```sql
-- Add specific table to replication
ALTER PUBLICATION supabase_realtime ADD TABLE messages;

-- Or add all tables
CREATE PUBLICATION supabase_realtime FOR ALL TABLES;
```

### 2. Configure PostgreSQL Settings

Required PostgreSQL settings for CDC replication:

```sql
-- Enable logical replication
ALTER SYSTEM SET wal_level = 'logical';
ALTER SYSTEM SET max_replication_slots = 10;
ALTER SYSTEM SET max_wal_senders = 10;

-- Restart PostgreSQL for settings to take effect
```

### 3. Enable Row Level Security (RLS)

RLS must be enabled on tables for Realtime to work with authorization:

```sql
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;

-- Create policy for authenticated users
CREATE POLICY "Users can read messages"
  ON messages FOR SELECT
  TO authenticated
  USING (true);
```

### 4. Get Previous Data on Updates/Deletes (Optional)

To receive the previous row data on UPDATE and DELETE operations:

```sql
ALTER TABLE messages REPLICA IDENTITY FULL;
```

## Client-Side Implementation

### JavaScript/TypeScript Example

```typescript
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  'https://your-project.supabase.co',
  'your-anon-key'
);

// Subscribe to database changes on a specific table
const channel = supabase
  .channel('messages-channel')
  .on(
    'postgres_changes',
    {
      event: '*', // Listen to all events (INSERT, UPDATE, DELETE)
      schema: 'public',
      table: 'messages',
    },
    (payload) => {
      console.log('Change received!', payload);
      
      switch (payload.eventType) {
        case 'INSERT':
          console.log('New message:', payload.new);
          break;
        case 'UPDATE':
          console.log('Updated message:', payload.new);
          console.log('Previous values:', payload.old);
          break;
        case 'DELETE':
          console.log('Deleted message:', payload.old);
          break;
      }
    }
  )
  .subscribe();

// Listen to specific events only
const insertChannel = supabase
  .channel('new-messages')
  .on(
    'postgres_changes',
    {
      event: 'INSERT',
      schema: 'public',
      table: 'messages',
    },
    (payload) => {
      console.log('New message inserted:', payload.new);
    }
  )
  .subscribe();

// Filter by specific conditions (using PostgREST filters)
const filteredChannel = supabase
  .channel('user-messages')
  .on(
    'postgres_changes',
    {
      event: '*',
      schema: 'public',
      table: 'messages',
      filter: 'user_id=eq.current_user_id', // Only messages for current user
    },
    (payload) => {
      console.log('User message changed:', payload);
    }
  )
  .subscribe();
```

### Svelte Example

```typescript
import { onMount, onDestroy } from 'svelte';
import { supabase } from '$lib/supabase';

let messages = [];
let channel;

onMount(() => {
  // Subscribe to messages table changes
  channel = supabase
    .channel('messages-realtime')
    .on(
      'postgres_changes',
      {
        event: '*',
        schema: 'public',
        table: 'messages',
      },
      (payload) => {
        if (payload.eventType === 'INSERT') {
          messages = [...messages, payload.new];
        } else if (payload.eventType === 'UPDATE') {
          messages = messages.map((msg) =>
            msg.id === payload.new.id ? payload.new : msg
          );
        } else if (payload.eventType === 'DELETE') {
          messages = messages.filter((msg) => msg.id !== payload.old.id);
        }
      }
    )
    .subscribe();
});

onDestroy(() => {
  // Clean up subscription
  if (channel) {
    supabase.removeChannel(channel);
  }
});
```

## Event Types

### Database Change Events

1. **INSERT**: New row added
   ```typescript
   {
     eventType: 'INSERT',
     new: { id: 1, message: 'Hello', created_at: '...' },
     old: null
   }
   ```

2. **UPDATE**: Row modified
   ```typescript
   {
     eventType: 'UPDATE',
     new: { id: 1, message: 'Updated', created_at: '...' },
     old: { id: 1, message: 'Hello', created_at: '...' } // If REPLICA IDENTITY FULL
   }
   ```

3. **DELETE**: Row removed
   ```typescript
   {
     eventType: 'DELETE',
     new: null,
     old: { id: 1, message: 'Hello', created_at: '...' } // If REPLICA IDENTITY FULL
   }
   ```

## Channel Types

### 1. Postgres Changes (Database Updates)

Listen to specific table changes:

```typescript
const channel = supabase
  .channel('table-changes')
  .on(
    'postgres_changes',
    {
      event: '*', // or 'INSERT', 'UPDATE', 'DELETE'
      schema: 'public',
      table: 'your_table',
    },
    (payload) => {
      // Handle change
    }
  )
  .subscribe();
```

### 2. Broadcast Messages

Send custom messages between clients:

```typescript
// Subscribe to broadcasts
const channel = supabase
  .channel('room-1')
  .on('broadcast', { event: 'cursor-move' }, (payload) => {
    console.log('Cursor moved:', payload);
  })
  .subscribe();

// Send broadcast
channel.send({
  type: 'broadcast',
  event: 'cursor-move',
  payload: { x: 100, y: 200 },
});
```

### 3. Presence

Track which users are online:

```typescript
const channel = supabase
  .channel('room-1')
  .on('presence', { event: 'sync' }, () => {
    const state = channel.presenceState();
    console.log('Online users:', state);
  })
  .on('presence', { event: 'join' }, ({ key, newPresences }) => {
    console.log('User joined:', newPresences);
  })
  .on('presence', { event: 'leave' }, ({ key, leftPresences }) => {
    console.log('User left:', leftPresences);
  })
  .subscribe(async (status) => {
    if (status === 'SUBSCRIBED') {
      await channel.track({
        online_at: new Date().toISOString(),
        user: 'user-123',
      });
    }
  });
```

## Authorization and Security

### JWT Authentication

Realtime connections are authenticated using JWT tokens:

```typescript
const supabase = createClient(url, key, {
  realtime: {
    params: {
      eventsPerSecond: 10, // Rate limiting
    },
  },
});
```

### Row Level Security (RLS)

RLS policies control which changes users can receive:

```sql
-- Only allow users to receive messages they have access to
CREATE POLICY "Users receive own messages"
  ON messages FOR SELECT
  TO authenticated
  USING (auth.uid() = user_id);
```

### Authorization Functions

Realtime uses authorization functions to check permissions:

```sql
-- Check if user can read from topic
CREATE OR REPLACE FUNCTION realtime.can_read_topic(
  topic text,
  claims jsonb,
  user_role text,
  headers jsonb
) RETURNS boolean AS $$
BEGIN
  -- Custom authorization logic
  RETURN (claims->>'role' = 'authenticated');
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
```

## WebSocket Connection

### Connection URL

```
wss://[project-ref].supabase.co/realtime/v1/websocket?apikey=[anon-token]&log_level=info&vsn=1.0.0
```

### Connection Lifecycle

1. **Connect**: Client establishes WebSocket connection
2. **Authenticate**: JWT token verified
3. **Subscribe**: Client joins channels
4. **Receive Events**: Database changes streamed to client
5. **Disconnect**: Automatic cleanup after 30 seconds

## Advanced Features

### Filtering Changes

Filter changes using PostgREST syntax:

```typescript
const channel = supabase
  .channel('filtered-messages')
  .on(
    'postgres_changes',
    {
      event: '*',
      schema: 'public',
      table: 'messages',
      filter: 'status=eq.published', // Only published messages
    },
    (payload) => {
      // Handle filtered changes
    }
  )
  .subscribe();
```

### Multiple Subscriptions

Subscribe to multiple tables or events:

```typescript
const channel = supabase
  .channel('multi-table')
  .on(
    'postgres_changes',
    { event: '*', schema: 'public', table: 'messages' },
    handleMessages
  )
  .on(
    'postgres_changes',
    { event: '*', schema: 'public', table: 'notifications' },
    handleNotifications
  )
  .subscribe();
```

### Unsubscribe

Clean up subscriptions when done:

```typescript
// Remove specific channel
supabase.removeChannel(channel);

// Or unsubscribe from channel
channel.unsubscribe();
```

## Performance Considerations

### Rate Limiting

- Default: 10 events per second per client
- Configurable via connection params
- Prevents overwhelming clients with updates

### Connection Management

- Supabase automatically cleans up connections 30 seconds after disconnect
- Unsubscribe from unused channels to maintain performance
- Multiple clients can subscribe to the same channel efficiently

### Global Distribution

- Realtime cluster is globally distributed
- Clients connect to nearest node
- Messages propagate across the cluster automatically

## Error Handling

```typescript
const channel = supabase
  .channel('messages')
  .on(
    'postgres_changes',
    { event: '*', schema: 'public', table: 'messages' },
    (payload) => {
      // Handle success
    }
  )
  .subscribe((status) => {
    if (status === 'SUBSCRIBED') {
      console.log('Successfully subscribed');
    } else if (status === 'CHANNEL_ERROR') {
      console.error('Channel error');
    } else if (status === 'TIMED_OUT') {
      console.error('Connection timed out');
    } else if (status === 'CLOSED') {
      console.log('Channel closed');
    }
  });
```

## Use Cases

1. **Live Chat**: Real-time message delivery
2. **Collaborative Editing**: Multiple users editing simultaneously
3. **Live Dashboards**: Real-time data updates
4. **Notifications**: Instant notification delivery
5. **Presence**: Show who's online
6. **Live Updates**: Any data that needs to update in real-time

## Best Practices

1. **Enable RLS**: Always use Row Level Security for authorization
2. **Unsubscribe**: Clean up subscriptions when components unmount
3. **Filter Early**: Use filters to reduce unnecessary updates
4. **Handle Errors**: Implement proper error handling and reconnection logic
5. **Rate Limiting**: Be mindful of event frequency
6. **REPLICA IDENTITY**: Set to FULL if you need old values on updates/deletes

## Summary

Supabase Realtime provides real-time database notifications through:

1. **PostgreSQL CDC**: Logical replication captures database changes
2. **Realtime Server**: Elixir/Phoenix cluster manages WebSocket connections
3. **Channels**: Subscribe to specific tables or broadcast messages
4. **Authorization**: JWT and RLS control access to changes
5. **Global Distribution**: Efficient message delivery across regions

This architecture enables instant notifications when database updates occur, making it ideal for collaborative applications, live dashboards, and real-time features.

## References

- [Supabase Realtime Documentation](https://supabase.com/docs/guides/realtime)
- [Realtime Architecture](https://supabase.com/docs/guides/realtime/architecture)
- [Postgres Changes](https://supabase.com/docs/guides/realtime/postgres-changes)
- [Supabase Realtime GitHub](https://github.com/supabase/realtime)

