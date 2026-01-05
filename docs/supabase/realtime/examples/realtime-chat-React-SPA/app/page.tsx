"use client"

import { RealtimeChat } from "@/components/realtime-chat"
import { useState } from "react"

export default function Page() {
  const [username, setUsername] = useState("")
  const [hasJoined, setHasJoined] = useState(false)

  if (!hasJoined) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-background">
        <div className="w-full max-w-md p-8 space-y-4">
          <div className="space-y-2 text-center">
            <h1 className="text-3xl font-bold">Join Chat</h1>
            <p className="text-muted-foreground">Enter your name to start chatting</p>
          </div>
          <form
            onSubmit={(e) => {
              e.preventDefault()
              if (username.trim()) {
                setHasJoined(true)
              }
            }}
            className="space-y-4"
          >
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Your name"
              className="w-full px-4 py-2 rounded-lg border border-border bg-background text-foreground"
              autoFocus
            />
            <button
              type="submit"
              disabled={!username.trim()}
              className="w-full px-4 py-2 rounded-lg bg-primary text-primary-foreground font-medium disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Join Chat
            </button>
          </form>
        </div>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-screen">
      <header className="border-b border-border p-4">
        <div className="flex items-center justify-between max-w-4xl mx-auto">
          <h1 className="text-xl font-semibold">Realtime Chat</h1>
          <div className="text-sm text-muted-foreground">
            Logged in as <span className="font-medium text-foreground">{username}</span>
          </div>
        </div>
      </header>
      <main className="flex-1 max-w-4xl w-full mx-auto overflow-hidden">
        <RealtimeChat roomName="general" username={username} />
      </main>
    </div>
  )
}
