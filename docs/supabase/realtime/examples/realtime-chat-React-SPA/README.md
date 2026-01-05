Title: Realtime Chat

Description: Real-time chat component for collaborative applications

Docs

Realtime Chat

# Realtime Chat

Real-time chat component for collaborative applications

## Installation

Open in

## Folder structure

- components

- chat-message.tsx

- realtime-chat.tsx

- hooks

- use-chat-scroll.tsx

- use-realtime-chat.tsx

- lib

- supabase

- client.ts

```ts
1import { cn } from '@/lib/utils'
2import type { ChatMessage } from '@/hooks/use-realtime-chat'
3
4interface ChatMessageItemProps {
5  message: ChatMessage
6  isOwnMessage: boolean
7  showHeader: boolean
8}
9
10export const ChatMessageItem = ({ message, isOwnMessage, showHeader }: ChatMessageItemProps) => {
11  return (
12    <div className={`flex mt-2 ${isOwnMessage ? 'justify-end' : 'justify-start'}`}>
13      <div
14        className={cn('max-w-[75%] w-fit flex flex-col gap-1', {
15          'items-end': isOwnMessage,
16        })}
17      >
18        {showHeader && (
19          <div
20            className={cn('flex items-center gap-2 text-xs px-3', {
21              'justify-end flex-row-reverse': isOwnMessage,
22            })}
23          >
24            <span className={'font-medium'}>{message.user.name}</span>
25            <span className="text-foreground/50 text-xs">
26              {new Date(message.createdAt).toLocaleTimeString('en-US', {
27                hour: '2-digit',
28                minute: '2-digit',
29                hour12: true,
30              })}
31            </span>
32          </div>
33        )}
34        <div
35          className={cn(
36            'py-2 px-3 rounded-xl text-sm w-fit',
37            isOwnMessage ? 'bg-primary text-primary-foreground' : 'bg-muted text-foreground'
38          )}
39        >
40          {message.content}
41        </div>
42      </div>
43    </div>
44  )
45}
```

## Introduction

The Realtime Chat component provides a complete chat interface that enables users to exchange messages in real-time within a shared room.

## Usage

### Basic usage

```
import { RealtimeChat } from '@/components/realtime-chat'

export default function ChatPage() {
return <RealtimeChat roomName="my-chat-room" username="john_doe" />
}
```

Copy

### With initial messages

```
import { RealtimeChat } from '@/components/realtime-chat'
import { useMessagesQuery } from '@/hooks/use-messages-query'

export default function ChatPage() {
const { data: messages } = useMessagesQuery()

return <RealtimeChat roomName="my-chat-room" username="john_doe" messages={messages} />
}
```

Copy

### Storing messages

```
import { RealtimeChat } from '@/components/realtime-chat'
import { useMessagesQuery } from '@/hooks/use-messages-query'
import { storeMessages } from '@/lib/store-messages'

export default function ChatPage() {
const { data: messages } = useMessagesQuery()
const handleMessage = (messages: ChatMessage[]) => {
// Store messages in your database
await storeMessages(messages)
}

return <RealtimeChat roomName="my-chat-room" username="john_doe" onMessage={handleMessage} />
}
```

Copy

## Features

- Real-time message synchronization
- Message persistence support with `onMessage` and `messages` props
- Customizable message appearance
- Automatic scroll-to-bottom on new messages
- Room-based isolation for scoped conversations
- Low-latency updates using Supabase Realtime

## Props

| Prop         | Type                                | Description                                                   |
| ------------ | ----------------------------------- | ------------------------------------------------------------- |
| `roomName`   | `string`                            | Unique identifier for the shared chat room.                   |
| `username`   | `string`                            | Name of the current user; used to identify message senders.   |
| `onMessage?` | `(messages: ChatMessage[]) => void` | Optional callback to handle messages, useful for persistence. |
| `messages?`  | `ChatMessage[]`                     | Optional initial messages to display in the chat.             |

### ChatMessage type

```
interface ChatMessage {
id: string
content: string
user: {
name: string
}
createdAt: string
}
```

Copy

## Further reading

- Realtime Broadcast
- Realtime authorization
