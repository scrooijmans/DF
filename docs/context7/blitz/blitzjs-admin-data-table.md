# Blitz.js Admin App with Type-Safe Data Table

This document explains how to build a type-safe admin application with an editable data table using Blitz.js, including typed RPC queries/mutations, database access, validation, and complete CRUD operations.

## Overview

Blitz.js provides:
- **Zero-API data layer**: Import server functions directly in client components
- **Type-safe RPC**: Automatic type inference from server to client
- **Prisma integration**: Type-safe database access
- **Zod validation**: Input validation with type enforcement
- **Built-in auth**: Session management and authorization

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Client Components (React)                      │
│              - Import server queries/mutations              │
│              - Type-safe hooks (useQuery, useMutation)      │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Build-time RPC transformation
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              RPC Layer (Blitz)                              │
│              - Queries (queries/ folder)                    │
│              - Mutations (mutations/ folder)                │
│              - Type inference                               │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Runtime HTTP calls
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Server Resolvers                               │
│              - resolver.pipe()                              │
│              - resolver.zod() validation                     │
│              - resolver.authorize()                          │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Prisma Client
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              SQL Database (PostgreSQL)                      │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Schema Definition (Prisma)

### Database Schema

```prisma
// db/schema.prisma

generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id        Int      @id @default(autoincrement())
  email     String   @unique
  name      String
  age       Int?
  role      Role     @default(USER)
  active    Boolean  @default(true)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  posts     Post[]
  
  @@index([email])
  @@index([role])
  @@map("users")
}

model Post {
  id        Int      @id @default(autoincrement())
  title     String
  content   String?
  published Boolean  @default(false)
  authorId  Int
  author    User     @relation(fields: [authorId], references: [id], onDelete: Cascade)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  @@index([authorId])
  @@index([published])
  @@map("posts")
}

enum Role {
  USER
  ADMIN
  MODERATOR
}
```

### Generate Prisma Client

```bash
# Generate Prisma Client
blitz prisma generate

# Run migrations
blitz prisma migrate dev

# Or push schema (development)
blitz prisma db push
```

## Step 2: Typed RPC Queries and Mutations

### How Blitz Generates Types

Blitz.js uses a "Zero-API" approach:
1. **Server functions** are defined in `queries/` and `mutations/` folders
2. **Client imports** these functions directly
3. **Build-time transformation** replaces imports with RPC calls
4. **Type inference** preserves TypeScript types across the boundary

### Query Resolver

```typescript
// app/users/queries/getUsers.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import * as z from "zod"

// Input validation schema
const GetUsersInput = z.object({
  page: z.number().int().positive().default(1),
  limit: z.number().int().positive().max(100).default(10),
  search: z.string().optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).optional(),
})

// Type inference: GetUsersInput type is automatically inferred
export default resolver.pipe(
  resolver.zod(GetUsersInput), // Validates and types input
  resolver.authorize(), // Requires authentication
  async (input, ctx) => {
    // input is now typed as z.infer<typeof GetUsersInput>
    const { page, limit, search, role } = input
    
    const where = {
      active: true,
      ...(search && {
        OR: [
          { name: { contains: search, mode: "insensitive" } },
          { email: { contains: search, mode: "insensitive" } },
        ],
      }),
      ...(role && { role }),
    }

    const [users, total] = await Promise.all([
      db.user.findMany({
        where,
        take: limit,
        skip: (page - 1) * limit,
        orderBy: { createdAt: "desc" },
        include: {
          posts: {
            select: { id: true, title: true },
            take: 3,
          },
        },
      }),
      db.user.count({ where }),
    ])

    return {
      users,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
    }
  }
)
```

### Single User Query

```typescript
// app/users/queries/getUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import * as z from "zod"
import { NotFoundError } from "blitz"

const GetUserInput = z.object({
  id: z.number().int().positive(),
})

export default resolver.pipe(
  resolver.zod(GetUserInput),
  resolver.authorize(),
  async (input, ctx) => {
    const user = await db.user.findUnique({
      where: { id: input.id },
      include: {
        posts: true,
      },
    })

    if (!user) {
      throw new NotFoundError(`User with id ${input.id} not found`)
    }

    return user
  }
)
```

### Create Mutation

```typescript
// app/users/mutations/createUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import * as z from "zod"

const CreateUserInput = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).default("USER"),
  active: z.boolean().default(true),
})

export default resolver.pipe(
  resolver.zod(CreateUserInput),
  resolver.authorize(),
  async (input, ctx) => {
    // Check if email already exists
    const existing = await db.user.findUnique({
      where: { email: input.email },
    })

    if (existing) {
      throw new Error("Email already exists")
    }

    const user = await db.user.create({
      data: input,
      include: {
        posts: true,
      },
    })

    return user
  }
)
```

### Update Mutation

```typescript
// app/users/mutations/updateUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import * as z from "zod"
import { NotFoundError } from "blitz"

const UpdateUserInput = z.object({
  id: z.number().int().positive(),
  email: z.string().email().optional(),
  name: z.string().min(1).max(100).optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).optional(),
  active: z.boolean().optional(),
})

export default resolver.pipe(
  resolver.zod(UpdateUserInput),
  resolver.authorize(),
  async (input, ctx) => {
    const { id, ...updateData } = input

    // Verify user exists
    const existing = await db.user.findUnique({
      where: { id },
    })

    if (!existing) {
      throw new NotFoundError(`User with id ${id} not found`)
    }

    // Check email uniqueness if changing
    if (updateData.email && updateData.email !== existing.email) {
      const emailExists = await db.user.findUnique({
        where: { email: updateData.email },
      })

      if (emailExists) {
        throw new Error("Email already exists")
      }
    }

    const user = await db.user.update({
      where: { id },
      data: updateData,
      include: {
        posts: true,
      },
    })

    return user
  }
)
```

### Delete Mutation

```typescript
// app/users/mutations/deleteUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import * as z from "zod"
import { NotFoundError } from "blitz"

const DeleteUserInput = z.object({
  id: z.number().int().positive(),
})

export default resolver.pipe(
  resolver.zod(DeleteUserInput),
  resolver.authorize(),
  async (input, ctx) => {
    const user = await db.user.findUnique({
      where: { id: input.id },
    })

    if (!user) {
      throw new NotFoundError(`User with id ${input.id} not found`)
    }

    await db.user.delete({
      where: { id: input.id },
    })

    return { success: true }
  }
)
```

## Step 3: Database Access Layer

### Prisma Client Setup

```typescript
// db/index.ts
import { PrismaClient } from "@prisma/client"

export * from "@prisma/client"

// Singleton pattern for Prisma Client
const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined
}

export const db = globalForPrisma.prisma ?? new PrismaClient({
  log: process.env.NODE_ENV === "development" 
    ? ["query", "error", "warn"] 
    : ["error"],
})

if (process.env.NODE_ENV !== "production") {
  globalForPrisma.prisma = db
}

export default db
```

### Type-Safe Database Operations

```typescript
// app/users/queries/getUsers.ts
import db from "db"
import type { Prisma } from "@prisma/client"

// Type-safe where clause
const where: Prisma.UserWhereInput = {
  active: true,
  role: "ADMIN",
}

// Type-safe include
const include: Prisma.UserInclude = {
  posts: {
    select: {
      id: true,
      title: true,
    },
  },
}

const users = await db.user.findMany({
  where,
  include,
  orderBy: { createdAt: "desc" },
})
```

## Step 4: Validation and Type Enforcement

### Zod Schema Validation

```typescript
// app/users/validations.ts
import * as z from "zod"

// Shared validation schemas
export const CreateUserSchema = z.object({
  email: z.string().email("Invalid email address"),
  name: z.string().min(1, "Name is required").max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).default("USER"),
  active: z.boolean().default(true),
})

export const UpdateUserSchema = z.object({
  id: z.number().int().positive(),
  email: z.string().email().optional(),
  name: z.string().min(1).max(100).optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).optional(),
  active: z.boolean().optional(),
})

export const GetUsersSchema = z.object({
  page: z.number().int().positive().default(1),
  limit: z.number().int().positive().max(100).default(10),
  search: z.string().optional(),
  role: z.enum(["USER", "ADMIN", "MODERATOR"]).optional(),
})

// Type inference
export type CreateUserInput = z.infer<typeof CreateUserSchema>
export type UpdateUserInput = z.infer<typeof UpdateUserSchema>
export type GetUsersInput = z.infer<typeof GetUsersSchema>
```

### Using Validation in Resolvers

```typescript
// app/users/mutations/createUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import { CreateUserSchema } from "../validations"

export default resolver.pipe(
  resolver.zod(CreateUserSchema), // Automatic validation and type inference
  resolver.authorize(),
  async (input, ctx) => {
    // input is typed as CreateUserInput
    // Validation errors are automatically handled
    const user = await db.user.create({
      data: input,
    })
    return user
  }
)
```

### Custom Validation with resolver.pipe

```typescript
// app/users/mutations/createUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import { CreateUserSchema } from "../validations"

export default resolver.pipe(
  resolver.zod(CreateUserSchema),
  resolver.authorize(),
  // Custom validation step
  async (input, ctx) => {
    // Business logic validation
    if (input.role === "ADMIN" && !ctx.session.role?.includes("SUPER_ADMIN")) {
      throw new Error("Only super admins can create admin users")
    }
    return input
  },
  async (input, ctx) => {
    // Database operation
    const user = await db.user.create({
      data: input,
    })
    return user
  }
)
```

## Step 5: Editable Data Table Integration

### Users Table Component

```typescript
// app/users/components/UsersTable.tsx
"use client"

import { useState } from "react"
import { useQuery, useMutation, invalidateQuery } from "@blitzjs/rpc"
import getUsers from "../queries/getUsers"
import updateUser from "../mutations/updateUser"
import deleteUser from "../mutations/deleteUser"
import createUser from "../mutations/createUser"
import type { User } from "db"

export default function UsersTable() {
  const [page, setPage] = useState(1)
  const [search, setSearch] = useState("")
  const [editingId, setEditingId] = useState<number | null>(null)

  // Type-safe query hook
  const [{ users, pagination }, { refetch, isLoading }] = useQuery(
    getUsers,
    {
      page,
      limit: 10,
      search: search || undefined,
    }
  )

  // Type-safe mutation hooks
  const [updateUserMutation, { isLoading: isUpdating }] = useMutation(updateUser)
  const [deleteUserMutation, { isLoading: isDeleting }] = useMutation(deleteUser)
  const [createUserMutation, { isLoading: isCreating }] = useMutation(createUser)

  const handleCreate = async (data: {
    email: string
    name: string
    age?: number
    role: "USER" | "ADMIN" | "MODERATOR"
  }) => {
    try {
      await createUserMutation(data)
      // Invalidate and refetch
      await invalidateQuery(getUsers)
      refetch()
    } catch (error) {
      alert(error instanceof Error ? error.message : "Failed to create user")
    }
  }

  const handleUpdate = async (id: number, data: Partial<User>) => {
    try {
      await updateUserMutation({ id, ...data })
      setEditingId(null)
      // Invalidate and refetch
      await invalidateQuery(getUsers)
      refetch()
    } catch (error) {
      alert(error instanceof Error ? error.message : "Failed to update user")
    }
  }

  const handleDelete = async (id: number) => {
    if (!confirm("Are you sure you want to delete this user?")) {
      return
    }

    try {
      await deleteUserMutation({ id })
      // Invalidate and refetch
      await invalidateQuery(getUsers)
      refetch()
    } catch (error) {
      alert(error instanceof Error ? error.message : "Failed to delete user")
    }
  }

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <div>
        <input
          type="text"
          placeholder="Search users..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        <button onClick={() => handleCreate({ email: "new@example.com", name: "New User", role: "USER" })}>
          Create User
        </button>
      </div>

      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Age</th>
            <th>Role</th>
            <th>Posts</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <UserRow
              key={user.id}
              user={user}
              editing={editingId === user.id}
              onEdit={() => setEditingId(user.id)}
              onCancel={() => setEditingId(null)}
              onUpdate={(data) => handleUpdate(user.id, data)}
              onDelete={() => handleDelete(user.id)}
              isUpdating={isUpdating}
              isDeleting={isDeleting}
            />
          ))}
        </tbody>
      </table>

      <div>
        <button
          onClick={() => setPage((p) => Math.max(1, p - 1))}
          disabled={page === 1}
        >
          Previous
        </button>
        <span>
          Page {pagination.page} of {pagination.totalPages}
        </span>
        <button
          onClick={() => setPage((p) => p + 1)}
          disabled={page >= pagination.totalPages}
        >
          Next
        </button>
      </div>
    </div>
  )
}

function UserRow({
  user,
  editing,
  onEdit,
  onCancel,
  onUpdate,
  onDelete,
  isUpdating,
  isDeleting,
}: {
  user: User & { posts: Array<{ id: number; title: string }> }
  editing: boolean
  onEdit: () => void
  onCancel: () => void
  onUpdate: (data: Partial<User>) => void
  onDelete: () => void
  isUpdating: boolean
  isDeleting: boolean
}) {
  const [formData, setFormData] = useState({
    name: user.name,
    email: user.email,
    age: user.age?.toString() || "",
    role: user.role,
    active: user.active,
  })

  if (editing) {
    return (
      <tr>
        <td>{user.id}</td>
        <td>
          <input
            type="text"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          />
        </td>
        <td>
          <input
            type="email"
            value={formData.email}
            onChange={(e) => setFormData({ ...formData, email: e.target.value })}
          />
        </td>
        <td>
          <input
            type="number"
            value={formData.age}
            onChange={(e) => setFormData({ ...formData, age: e.target.value })}
          />
        </td>
        <td>
          <select
            value={formData.role}
            onChange={(e) =>
              setFormData({
                ...formData,
                role: e.target.value as "USER" | "ADMIN" | "MODERATOR",
              })
            }
          >
            <option value="USER">User</option>
            <option value="ADMIN">Admin</option>
            <option value="MODERATOR">Moderator</option>
          </select>
        </td>
        <td>{user.posts.length}</td>
        <td>
          <button
            onClick={() =>
              onUpdate({
                name: formData.name,
                email: formData.email,
                age: formData.age ? parseInt(formData.age) : null,
                role: formData.role,
                active: formData.active,
              })
            }
            disabled={isUpdating}
          >
            {isUpdating ? "Saving..." : "Save"}
          </button>
          <button onClick={onCancel}>Cancel</button>
        </td>
      </tr>
    )
  }

  return (
    <tr>
      <td>{user.id}</td>
      <td>{user.name}</td>
      <td>{user.email}</td>
      <td>{user.age || "-"}</td>
      <td>{user.role}</td>
      <td>{user.posts.length}</td>
      <td>
        <button onClick={onEdit}>Edit</button>
        <button onClick={onDelete} disabled={isDeleting}>
          {isDeleting ? "Deleting..." : "Delete"}
        </button>
      </td>
    </tr>
  )
}
```

### Form Component with Validation

```typescript
// app/users/components/CreateUserForm.tsx
"use client"

import { useState } from "react"
import { useMutation, invalidateQuery } from "@blitzjs/rpc"
import { validateZodSchema } from "blitz"
import createUser from "../mutations/createUser"
import getUsers from "../queries/getUsers"
import { CreateUserSchema } from "../validations"

export default function CreateUserForm() {
  const [formData, setFormData] = useState({
    email: "",
    name: "",
    age: "",
    role: "USER" as "USER" | "ADMIN" | "MODERATOR",
  })
  const [errors, setErrors] = useState<Record<string, string>>({})

  const [createUserMutation, { isLoading }] = useMutation(createUser)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setErrors({})

    // Client-side validation
    const validation = CreateUserSchema.safeParse({
      ...formData,
      age: formData.age ? parseInt(formData.age) : undefined,
    })

    if (!validation.success) {
      const fieldErrors: Record<string, string> = {}
      validation.error.errors.forEach((err) => {
        if (err.path[0]) {
          fieldErrors[err.path[0] as string] = err.message
        }
      })
      setErrors(fieldErrors)
      return
    }

    try {
      await createUserMutation(validation.data)
      // Reset form
      setFormData({ email: "", name: "", age: "", role: "USER" })
      // Invalidate query cache
      await invalidateQuery(getUsers)
    } catch (error) {
      alert(error instanceof Error ? error.message : "Failed to create user")
    }
  }

  return (
    <form onSubmit={handleSubmit}>
      <div>
        <label>
          Email:
          <input
            type="email"
            value={formData.email}
            onChange={(e) => setFormData({ ...formData, email: e.target.value })}
          />
        </label>
        {errors.email && <span style={{ color: "red" }}>{errors.email}</span>}
      </div>

      <div>
        <label>
          Name:
          <input
            type="text"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          />
        </label>
        {errors.name && <span style={{ color: "red" }}>{errors.name}</span>}
      </div>

      <div>
        <label>
          Age:
          <input
            type="number"
            value={formData.age}
            onChange={(e) => setFormData({ ...formData, age: e.target.value })}
          />
        </label>
        {errors.age && <span style={{ color: "red" }}>{errors.age}</span>}
      </div>

      <div>
        <label>
          Role:
          <select
            value={formData.role}
            onChange={(e) =>
              setFormData({
                ...formData,
                role: e.target.value as "USER" | "ADMIN" | "MODERATOR",
              })
            }
          >
            <option value="USER">User</option>
            <option value="ADMIN">Admin</option>
            <option value="MODERATOR">Moderator</option>
          </select>
        </label>
        {errors.role && <span style={{ color: "red" }}>{errors.role}</span>}
      </div>

      <button type="submit" disabled={isLoading}>
        {isLoading ? "Creating..." : "Create User"}
      </button>
    </form>
  )
}
```

## Step 6: Call Stacks for CRUD Operations

### Call Stack: Create

```
1. User fills form and clicks "Create User"
   ↓
2. React: CreateUserForm.handleSubmit()
   ↓
3. Client-side validation: CreateUserSchema.safeParse()
   - If invalid → Show errors, stop
   - If valid → Continue
   ↓
4. useMutation hook: createUserMutation(validation.data)
   ↓
5. Blitz RPC: HTTP POST /api/rpc/createUser
   {
     "params": [validation.data]
   }
   ↓
6. Server: app/users/mutations/createUser.ts
   ↓
7. resolver.pipe() execution:
   a. resolver.zod(CreateUserSchema) - Validate input
   b. resolver.authorize() - Check authentication
   c. Custom validation - Check business rules
   d. Database operation - db.user.create()
   ↓
8. Prisma Client: Execute SQL INSERT
   ↓
9. PostgreSQL: Insert row, return created record
   ↓
10. Prisma: Transform to User object
   ↓
11. Resolver: Return user object
   ↓
12. HTTP Response: { "result": user }
   ↓
13. useMutation: Update loading state, return result
   ↓
14. Component: invalidateQuery(getUsers)
   ↓
15. Component: Reset form, show success
```

### Call Stack: Read

```
1. Component mounts or refetch() called
   ↓
2. useQuery hook: useQuery(getUsers, { page, limit, search })
   ↓
3. Check RPC cache
   - If cached and fresh → Return cached data
   - If not cached or stale → Continue
   ↓
4. Blitz RPC: HTTP POST /api/rpc/getUsers
   {
     "params": [{ page: 1, limit: 10, search: "..." }]
   }
   ↓
5. Server: app/users/queries/getUsers.ts
   ↓
6. resolver.pipe() execution:
   a. resolver.zod(GetUsersInput) - Validate input
   b. resolver.authorize() - Check authentication
   c. Database query - db.user.findMany()
   ↓
7. Prisma Client: Execute SQL SELECT
   ↓
8. PostgreSQL: Return rows
   ↓
9. Prisma: Transform to User[] with relations
   ↓
10. Resolver: Return { users, pagination }
   ↓
11. HTTP Response: { "result": { users, pagination } }
   ↓
12. useQuery: Store in cache, update state
   ↓
13. React: Re-render with data
```

### Call Stack: Update

```
1. User clicks "Edit" → User modifies fields → User clicks "Save"
   ↓
2. React: UserRow.onUpdate()
   ↓
3. useMutation hook: updateUserMutation({ id, ...data })
   ↓
4. Blitz RPC: HTTP POST /api/rpc/updateUser
   {
     "params": [{ id: 1, name: "...", email: "..." }]
   }
   ↓
5. Server: app/users/mutations/updateUser.ts
   ↓
6. resolver.pipe() execution:
   a. resolver.zod(UpdateUserInput) - Validate input
   b. resolver.authorize() - Check authentication
   c. Verify user exists - db.user.findUnique()
   d. Check email uniqueness (if changing)
   e. Database update - db.user.update()
   ↓
7. Prisma Client: Execute SQL UPDATE
   ↓
8. PostgreSQL: Update row, return updated record
   ↓
9. Prisma: Transform to User object
   ↓
10. Resolver: Return user object
   ↓
11. HTTP Response: { "result": user }
   ↓
12. useMutation: Update loading state, return result
   ↓
13. Component: invalidateQuery(getUsers)
   ↓
14. Component: setEditingId(null), refetch()
   ↓
15. React: Re-render with updated data
```

### Call Stack: Delete

```
1. User clicks "Delete" → Confirms deletion
   ↓
2. React: UsersTable.handleDelete(id)
   ↓
3. useMutation hook: deleteUserMutation({ id })
   ↓
4. Blitz RPC: HTTP POST /api/rpc/deleteUser
   {
     "params": [{ id: 1 }]
   }
   ↓
5. Server: app/users/mutations/deleteUser.ts
   ↓
6. resolver.pipe() execution:
   a. resolver.zod(DeleteUserInput) - Validate input
   b. resolver.authorize() - Check authentication
   c. Verify user exists - db.user.findUnique()
   d. Database delete - db.user.delete()
   ↓
7. Prisma Client: Execute SQL DELETE
   ↓
8. PostgreSQL: Delete row
   ↓
9. Resolver: Return { success: true }
   ↓
10. HTTP Response: { "result": { success: true } }
   ↓
11. useMutation: Update loading state, return result
   ↓
12. Component: invalidateQuery(getUsers)
   ↓
13. Component: refetch()
   ↓
14. React: Re-render without deleted user
```

## Step 7: Advanced Patterns

### Authorization with Roles

```typescript
// app/users/mutations/updateUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import { UpdateUserSchema } from "../validations"

export default resolver.pipe(
  resolver.zod(UpdateUserSchema),
  resolver.authorize(), // Basic auth
  async (input, ctx) => {
    // Role-based authorization
    if (input.role === "ADMIN" && ctx.session.role !== "SUPER_ADMIN") {
      throw new Error("Only super admins can assign admin role")
    }

    // Owner or admin can update
    if (input.id !== ctx.session.userId && ctx.session.role !== "ADMIN") {
      throw new Error("Unauthorized")
    }

    return await db.user.update({
      where: { id: input.id },
      data: input,
    })
  }
)
```

### Optimistic Updates

```typescript
// app/users/components/UsersTable.tsx
const [updateUserMutation] = useMutation(updateUser, {
  onSuccess: async () => {
    // Invalidate cache
    await invalidateQuery(getUsers)
    refetch()
  },
})

// Optimistic update
const handleUpdate = async (id: number, data: Partial<User>) => {
  // Update UI immediately
  setUsers((prev) =>
    prev.map((u) => (u.id === id ? { ...u, ...data } : u))
  )

  try {
    await updateUserMutation({ id, ...data })
  } catch (error) {
    // Revert on error
    refetch()
    alert("Failed to update user")
  }
}
```

### Error Handling

```typescript
// app/users/mutations/createUser.ts
import { resolver } from "@blitzjs/rpc"
import db from "db"
import { CreateUserSchema } from "../validations"
import { AuthenticationError, AuthorizationError } from "blitz"

export default resolver.pipe(
  resolver.zod(CreateUserSchema),
  resolver.authorize(),
  async (input, ctx) => {
    try {
      const user = await db.user.create({
        data: input,
      })
      return user
    } catch (error: any) {
      // Handle Prisma errors
      if (error.code === "P2002") {
        throw new Error("Email already exists")
      }
      throw error
    }
  }
)
```

### Pagination with usePaginatedQuery

```typescript
// app/users/components/UsersTable.tsx
import { usePaginatedQuery } from "@blitzjs/rpc"
import getUsers from "../queries/getUsers"

export default function UsersTable() {
  const {
    results: users,
    isLoading,
    nextPage,
    previousPage,
    hasMore,
    count,
  } = usePaginatedQuery(getUsers, {
    limit: 10,
    search: search || undefined,
  })

  return (
    <div>
      <table>
        {/* ... */}
      </table>
      <button onClick={previousPage} disabled={!hasMore}>
        Previous
      </button>
      <button onClick={nextPage} disabled={!hasMore}>
        Next
      </button>
    </div>
  )
}
```

## Best Practices

### 1. Shared Validation Schemas

```typescript
// app/users/validations.ts
// Export schemas for reuse in forms and resolvers
export const CreateUserSchema = z.object({...})
export const UpdateUserSchema = z.object({...})
```

### 2. Type Inference

```typescript
// Types are automatically inferred from Zod schemas
type CreateUserInput = z.infer<typeof CreateUserSchema>
// Use in components for type safety
```

### 3. Cache Management

```typescript
// Invalidate after mutations
await invalidateQuery(getUsers)

// Or use setQueryData for optimistic updates
setQueryData(getUsers, variables, newData)
```

### 4. Error Handling

```typescript
// Use Blitz error types
import { NotFoundError, AuthenticationError } from "blitz"

// Handle in components
try {
  await mutation(data)
} catch (error) {
  if (error instanceof NotFoundError) {
    // Handle not found
  }
}
```

### 5. Authorization

```typescript
// Use resolver.authorize() for basic auth
resolver.authorize()

// Or custom authorization
if (!ctx.session.role?.includes("ADMIN")) {
  throw new AuthorizationError()
}
```

## Summary

**Blitz.js provides end-to-end type safety:**

1. **Typed RPC**: Server functions are imported directly in client components
2. **Type Inference**: TypeScript types flow from server to client automatically
3. **Validation**: Zod schemas validate input and enforce types
4. **Database Access**: Prisma provides type-safe database operations
5. **Authorization**: Built-in session management and authorization

**Key Benefits:**
- **Zero API boilerplate**: No manual API endpoints
- **Type safety**: Full TypeScript support across the stack
- **Developer experience**: Intuitive API with automatic type inference
- **Performance**: Optimized RPC calls with caching
- **Security**: Built-in validation and authorization

This architecture ensures type safety from database to UI, with minimal boilerplate and maximum developer productivity.

