# Type-Safe CRUD Data Table with Prisma

This document explains how to build a type-safe, editable data table using Prisma as the database layer, including schema definition, generated client usage, UI integration, safe update operations, and backend API design patterns.

## Overview

Prisma provides:
- **Type-safe database client**: Auto-generated from schema
- **Declarative schema**: Single source of truth
- **Migration system**: Version-controlled database changes
- **Type inference**: Full TypeScript support
- **Query builder**: Intuitive API for database operations

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Prisma Schema (schema.prisma)                  │
│              Single source of truth                         │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ npx prisma generate
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Generated Prisma Client                        │
│              (@prisma/client)                               │
│              - Type-safe models                             │
│              - Query methods                                │
│              - Type definitions                             │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Import & Use
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Backend API Layer                              │
│              - Service layer                                │
│              - Route handlers                                │
│              - Type-safe operations                          │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ HTTP/API
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend (React/Next.js)                      │
│              - Typed API calls                              │
│              - Type-safe components                        │
│              - Data table UI                                │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Schema Definition in Prisma

### Basic Schema

```prisma
// prisma/schema.prisma

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
  
  // Relations
  posts     Post[]
  profile   Profile?
  
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

model Profile {
  id     Int    @id @default(autoincrement())
  bio    String?
  userId Int    @unique
  user   User   @relation(fields: [userId], references: [id], onDelete: Cascade)
  
  @@map("profiles")
}

enum Role {
  USER
  ADMIN
  MODERATOR
}
```

### Advanced Schema Features

```prisma
// prisma/schema.prisma

model User {
  id        Int      @id @default(autoincrement())
  email     String   @unique
  name      String
  age       Int?     @db.SmallInt
  
  // JSON field
  metadata  Json?
  
  // Array field (PostgreSQL)
  tags      String[]
  
  // Decimal for precise numbers
  balance   Decimal  @default(0) @db.Decimal(10, 2)
  
  // Custom default with function
  uuid      String   @default(uuid())
  
  // Timestamps
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  // Soft delete
  deletedAt DateTime?
  
  // Relations with cascade
  posts     Post[]   @relation("UserPosts")
  comments  Comment[]
  
  @@unique([email, name])
  @@index([email, role])
  @@map("users")
}

// Many-to-many relation
model Post {
  id        Int      @id @default(autoincrement())
  title     String
  tags      Tag[]
  
  @@map("posts")
}

model Tag {
  id    Int    @id @default(autoincrement())
  name  String @unique
  posts Post[]
  
  @@map("tags")
}
```

### Generate Prisma Client

```bash
# Generate Prisma Client from schema
npx prisma generate

# Create and apply migration
npx prisma migrate dev --name init

# Push schema changes (development only)
npx prisma db push
```

## Step 2: Generated Client Usage

### Initialize Prisma Client

```typescript
// lib/prisma.ts
import { PrismaClient } from '@prisma/client'

// Singleton pattern for Prisma Client
const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined
}

export const prisma = globalForPrisma.prisma ?? new PrismaClient({
  log: process.env.NODE_ENV === 'development' 
    ? ['query', 'error', 'warn'] 
    : ['error'],
})

if (process.env.NODE_ENV !== 'production') {
  globalForPrisma.prisma = prisma
}

// Disconnect on app shutdown
process.on('beforeExit', async () => {
  await prisma.$disconnect()
})
```

### Type-Safe CRUD Operations

```typescript
// lib/services/user.service.ts
import { prisma } from '../prisma'
import type { User, Prisma } from '@prisma/client'

// CREATE - Single record
export async function createUser(data: Prisma.UserCreateInput): Promise<User> {
  return prisma.user.create({
    data,
    include: {
      posts: true,
      profile: true,
    },
  })
}

// CREATE - Multiple records
export async function createManyUsers(
  data: Prisma.UserCreateManyInput[]
): Promise<{ count: number }> {
  return prisma.user.createMany({
    data,
    skipDuplicates: true, // Skip on unique constraint violations
  })
}

// READ - Find many with filters
export async function getUsers(params: {
  where?: Prisma.UserWhereInput
  orderBy?: Prisma.UserOrderByWithRelationInput[]
  take?: number
  skip?: number
  include?: Prisma.UserInclude
}): Promise<User[]> {
  return prisma.user.findMany({
    where: params.where,
    orderBy: params.orderBy,
    take: params.take,
    skip: params.skip,
    include: params.include,
  })
}

// READ - Find unique
export async function getUserById(
  id: number,
  include?: Prisma.UserInclude
): Promise<User | null> {
  return prisma.user.findUnique({
    where: { id },
    include,
  })
}

// READ - Find first
export async function getUserByEmail(
  email: string
): Promise<User | null> {
  return prisma.user.findFirst({
    where: { email },
  })
}

// UPDATE - Single record (type-safe)
export async function updateUser(
  id: number,
  data: Prisma.UserUpdateInput
): Promise<User> {
  return prisma.user.update({
    where: { id },
    data,
    include: {
      posts: true,
      profile: true,
    },
  })
}

// UPDATE - Multiple records
export async function updateManyUsers(
  where: Prisma.UserWhereInput,
  data: Prisma.UserUpdateManyMutationInput
): Promise<{ count: number }> {
  return prisma.user.updateMany({
    where,
    data,
  })
}

// UPSERT - Update or create
export async function upsertUser(
  where: Prisma.UserWhereUniqueInput,
  create: Prisma.UserCreateInput,
  update: Prisma.UserUpdateInput
): Promise<User> {
  return prisma.user.upsert({
    where,
    create,
    update,
  })
}

// DELETE - Single record
export async function deleteUser(id: number): Promise<User> {
  return prisma.user.delete({
    where: { id },
  })
}

// DELETE - Multiple records
export async function deleteManyUsers(
  where: Prisma.UserWhereInput
): Promise<{ count: number }> {
  return prisma.user.deleteMany({
    where,
  })
}
```

### Advanced Query Patterns

```typescript
// lib/services/user.service.ts
import { prisma } from '../prisma'
import type { Prisma } from '@prisma/client'

// Complex filtering
export async function searchUsers(query: string) {
  return prisma.user.findMany({
    where: {
      OR: [
        { name: { contains: query, mode: 'insensitive' } },
        { email: { contains: query, mode: 'insensitive' } },
      ],
      active: true,
      role: {
        in: ['USER', 'ADMIN'],
      },
    },
    include: {
      posts: {
        where: { published: true },
        take: 5,
        orderBy: { createdAt: 'desc' },
      },
      profile: true,
    },
    orderBy: [
      { createdAt: 'desc' },
      { name: 'asc' },
    ],
    take: 20,
    skip: 0,
  })
}

// Aggregations
export async function getUserStats() {
  const [total, active, admins] = await Promise.all([
    prisma.user.count(),
    prisma.user.count({ where: { active: true } }),
    prisma.user.count({ where: { role: 'ADMIN' } }),
  ])

  return { total, active, admins }
}

// Transactions
export async function transferUserPosts(
  fromUserId: number,
  toUserId: number
) {
  return prisma.$transaction(async (tx) => {
    // Update all posts
    await tx.post.updateMany({
      where: { authorId: fromUserId },
      data: { authorId: toUserId },
    })

    // Update user stats
    const fromUser = await tx.user.update({
      where: { id: fromUserId },
      data: { active: false },
    })

    const toUser = await tx.user.findUnique({
      where: { id: toUserId },
    })

    return { fromUser, toUser }
  })
}

// Raw SQL (when needed)
export async function getUsersWithRawQuery() {
  return prisma.$queryRaw<Array<{ id: number; name: string }>>`
    SELECT id, name 
    FROM users 
    WHERE active = true 
    ORDER BY created_at DESC 
    LIMIT 10
  `
}
```

## Step 3: UI Layer Integration with Typed APIs

### Next.js API Routes

```typescript
// app/api/users/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import type { Prisma } from '@prisma/client'

// GET - Fetch all users
export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams
    const page = parseInt(searchParams.get('page') || '1')
    const limit = parseInt(searchParams.get('limit') || '10')
    const search = searchParams.get('search') || ''
    const role = searchParams.get('role') as 'USER' | 'ADMIN' | 'MODERATOR' | null

    const where: Prisma.UserWhereInput = {
      ...(search && {
        OR: [
          { name: { contains: search, mode: 'insensitive' } },
          { email: { contains: search, mode: 'insensitive' } },
        ],
      }),
      ...(role && { role }),
      active: true,
    }

    const [users, total] = await Promise.all([
      prisma.user.findMany({
        where,
        take: limit,
        skip: (page - 1) * limit,
        orderBy: { createdAt: 'desc' },
        include: {
          posts: {
            select: { id: true, title: true },
            take: 3,
          },
          profile: true,
        },
      }),
      prisma.user.count({ where }),
    ])

    return NextResponse.json({
      users,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
    })
  } catch (error) {
    console.error('Error fetching users:', error)
    return NextResponse.json(
      { error: 'Failed to fetch users' },
      { status: 500 }
    )
  }
}

// POST - Create user
export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    
    // Type-safe validation
    const userData: Prisma.UserCreateInput = {
      email: body.email,
      name: body.name,
      age: body.age,
      role: body.role || 'USER',
      active: body.active ?? true,
    }

    const user = await prisma.user.create({
      data: userData,
      include: {
        posts: true,
        profile: true,
      },
    })

    return NextResponse.json(user, { status: 201 })
  } catch (error: any) {
    console.error('Error creating user:', error)
    
    // Handle Prisma unique constraint errors
    if (error.code === 'P2002') {
      return NextResponse.json(
        { error: 'Email already exists' },
        { status: 409 }
      )
    }

    return NextResponse.json(
      { error: 'Failed to create user' },
      { status: 500 }
    )
  }
}
```

```typescript
// app/api/users/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import type { Prisma } from '@prisma/client'

// GET - Fetch single user
export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    const user = await prisma.user.findUnique({
      where: { id },
      include: {
        posts: true,
        profile: true,
      },
    })

    if (!user) {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      )
    }

    return NextResponse.json(user)
  } catch (error) {
    console.error('Error fetching user:', error)
    return NextResponse.json(
      { error: 'Failed to fetch user' },
      { status: 500 }
    )
  }
}

// PUT/PATCH - Update user
export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    const body = await request.json()
    
    // Type-safe update data
    const updateData: Prisma.UserUpdateInput = {
      ...(body.name && { name: body.name }),
      ...(body.email && { email: body.email }),
      ...(body.age !== undefined && { age: body.age }),
      ...(body.role && { role: body.role }),
      ...(body.active !== undefined && { active: body.active }),
    }

    const user = await prisma.user.update({
      where: { id },
      data: updateData,
      include: {
        posts: true,
        profile: true,
      },
    })

    return NextResponse.json(user)
  } catch (error: any) {
    console.error('Error updating user:', error)
    
    if (error.code === 'P2025') {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      )
    }

    if (error.code === 'P2002') {
      return NextResponse.json(
        { error: 'Email already exists' },
        { status: 409 }
      )
    }

    return NextResponse.json(
      { error: 'Failed to update user' },
      { status: 500 }
    )
  }
}

// DELETE - Delete user
export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    await prisma.user.delete({
      where: { id },
    })

    return NextResponse.json({ success: true })
  } catch (error: any) {
    console.error('Error deleting user:', error)
    
    if (error.code === 'P2025') {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      )
    }

    return NextResponse.json(
      { error: 'Failed to delete user' },
      { status: 500 }
    )
  }
}
```

### React Components with Type Safety

```typescript
// components/UsersTable.tsx
'use client'

import { useState, useEffect } from 'react'
import type { User, Prisma } from '@prisma/client'

type UserWithRelations = Prisma.UserGetPayload<{
  include: { posts: true; profile: true }
}>

export function UsersTable() {
  const [users, setUsers] = useState<UserWithRelations[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [page, setPage] = useState(1)
  const [search, setSearch] = useState('')

  useEffect(() => {
    fetchUsers()
  }, [page, search])

  async function fetchUsers() {
    try {
      setLoading(true)
      const params = new URLSearchParams({
        page: page.toString(),
        limit: '10',
        ...(search && { search }),
      })

      const response = await fetch(`/api/users?${params}`)
      if (!response.ok) {
        throw new Error('Failed to fetch users')
      }

      const data = await response.json()
      setUsers(data.users)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred')
    } finally {
      setLoading(false)
    }
  }

  async function handleDelete(id: number) {
    if (!confirm('Are you sure you want to delete this user?')) {
      return
    }

    try {
      const response = await fetch(`/api/users/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete user')
      }

      // Refresh list
      fetchUsers()
    } catch (err) {
      alert(err instanceof Error ? err.message : 'Failed to delete user')
    }
  }

  if (loading) return <div>Loading...</div>
  if (error) return <div>Error: {error}</div>

  return (
    <div>
      <input
        type="text"
        placeholder="Search users..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />

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
              onDelete={handleDelete}
              onUpdate={fetchUsers}
            />
          ))}
        </tbody>
      </table>

      <div>
        <button onClick={() => setPage((p) => Math.max(1, p - 1))}>
          Previous
        </button>
        <span>Page {page}</span>
        <button onClick={() => setPage((p) => p + 1)}>Next</button>
      </div>
    </div>
  )
}

function UserRow({
  user,
  onDelete,
  onUpdate,
}: {
  user: UserWithRelations
  onDelete: (id: number) => void
  onUpdate: () => void
}) {
  const [editing, setEditing] = useState(false)
  const [formData, setFormData] = useState({
    name: user.name,
    email: user.email,
    age: user.age?.toString() || '',
    role: user.role,
  })

  async function handleUpdate() {
    try {
      const response = await fetch(`/api/users/${user.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: formData.name,
          email: formData.email,
          age: formData.age ? parseInt(formData.age) : null,
          role: formData.role,
        }),
      })

      if (!response.ok) {
        throw new Error('Failed to update user')
      }

      setEditing(false)
      onUpdate()
    } catch (err) {
      alert(err instanceof Error ? err.message : 'Failed to update user')
    }
  }

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
                role: e.target.value as 'USER' | 'ADMIN' | 'MODERATOR',
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
          <button onClick={handleUpdate}>Save</button>
          <button onClick={() => setEditing(false)}>Cancel</button>
        </td>
      </tr>
    )
  }

  return (
    <tr>
      <td>{user.id}</td>
      <td>{user.name}</td>
      <td>{user.email}</td>
      <td>{user.age || '-'}</td>
      <td>{user.role}</td>
      <td>{user.posts.length}</td>
      <td>
        <button onClick={() => setEditing(true)}>Edit</button>
        <button onClick={() => onDelete(user.id)}>Delete</button>
      </td>
    </tr>
  )
}
```

## Step 4: Safe Row Update Example

### Type-Safe Update Function

```typescript
// lib/services/user.service.ts
import { prisma } from '../prisma'
import type { Prisma } from '@prisma/client'

// Safe update with validation
export async function safeUpdateUser(
  id: number,
  updates: Partial<{
    name: string
    email: string
    age: number | null
    role: 'USER' | 'ADMIN' | 'MODERATOR'
    active: boolean
  }>
): Promise<{ success: boolean; user?: any; error?: string }> {
  try {
    // 1. Verify user exists
    const existingUser = await prisma.user.findUnique({
      where: { id },
    })

    if (!existingUser) {
      return { success: false, error: 'User not found' }
    }

    // 2. Validate email uniqueness if changing
    if (updates.email && updates.email !== existingUser.email) {
      const emailExists = await prisma.user.findUnique({
        where: { email: updates.email },
      })

      if (emailExists) {
        return { success: false, error: 'Email already exists' }
      }
    }

    // 3. Build type-safe update data
    const updateData: Prisma.UserUpdateInput = {}
    
    if (updates.name !== undefined) {
      updateData.name = updates.name
    }
    if (updates.email !== undefined) {
      updateData.email = updates.email
    }
    if (updates.age !== undefined) {
      updateData.age = updates.age
    }
    if (updates.role !== undefined) {
      updateData.role = updates.role
    }
    if (updates.active !== undefined) {
      updateData.active = updates.active
    }

    // 4. Perform update in transaction
    const user = await prisma.$transaction(async (tx) => {
      // Update user
      const updated = await tx.user.update({
        where: { id },
        data: updateData,
        include: {
          posts: true,
          profile: true,
        },
      })

      // Log update (example)
      await tx.$executeRaw`
        INSERT INTO audit_logs (action, entity_type, entity_id, created_at)
        VALUES ('UPDATE', 'User', ${id}, NOW())
      `

      return updated
    })

    return { success: true, user }
  } catch (error: any) {
    console.error('Error updating user:', error)
    
    // Handle Prisma errors
    if (error.code === 'P2025') {
      return { success: false, error: 'User not found' }
    }
    if (error.code === 'P2002') {
      return { success: false, error: 'Email already exists' }
    }

    return { success: false, error: 'Failed to update user' }
  }
}

// Optimistic update pattern
export async function optimisticUpdateUser(
  id: number,
  updates: Prisma.UserUpdateInput,
  optimisticData: Partial<User>
): Promise<User> {
  // This pattern is useful for UI updates
  // The UI can show optimistic data immediately
  // while the actual update happens in the background

  const user = await prisma.user.update({
    where: { id },
    data: updates,
  })

  return user
}
```

### API Route with Safe Update

```typescript
// app/api/users/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { safeUpdateUser } from '@/lib/services/user.service'
import { z } from 'zod'

// Validation schema
const updateUserSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  email: z.string().email().optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).optional(),
  active: z.boolean().optional(),
})

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    const body = await request.json()
    
    // Validate input
    const validation = updateUserSchema.safeParse(body)
    if (!validation.success) {
      return NextResponse.json(
        { error: 'Validation failed', details: validation.error.errors },
        { status: 400 }
      )
    }

    // Safe update
    const result = await safeUpdateUser(id, validation.data)

    if (!result.success) {
      return NextResponse.json(
        { error: result.error },
        { status: result.error === 'User not found' ? 404 : 409 }
      )
    }

    return NextResponse.json(result.user)
  } catch (error) {
    console.error('Error updating user:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}
```

## Step 5: Backend API Design and Patterns

### Service Layer Pattern

```typescript
// lib/services/base.service.ts
import { prisma } from '../prisma'
import type { Prisma } from '@prisma/client'

export abstract class BaseService<T, CreateInput, UpdateInput> {
  protected model: any

  constructor(model: any) {
    this.model = model
  }

  async findMany(params?: {
    where?: any
    orderBy?: any
    take?: number
    skip?: number
    include?: any
  }): Promise<T[]> {
    return this.model.findMany(params)
  }

  async findUnique(where: any, include?: any): Promise<T | null> {
    return this.model.findUnique({ where, include })
  }

  async create(data: CreateInput, include?: any): Promise<T> {
    return this.model.create({ data, include })
  }

  async update(where: any, data: UpdateInput, include?: any): Promise<T> {
    return this.model.update({ where, data, include })
  }

  async delete(where: any): Promise<T> {
    return this.model.delete({ where })
  }
}
```

```typescript
// lib/services/user.service.ts
import { BaseService } from './base.service'
import { prisma } from '../prisma'
import type { User, Prisma } from '@prisma/client'

export class UserService extends BaseService<
  User,
  Prisma.UserCreateInput,
  Prisma.UserUpdateInput
> {
  constructor() {
    super(prisma.user)
  }

  // Custom business logic methods
  async findByEmail(email: string): Promise<User | null> {
    return this.findUnique({ email })
  }

  async getActiveUsers(): Promise<User[]> {
    return this.findMany({
      where: { active: true },
      orderBy: { createdAt: 'desc' },
    })
  }

  async deactivateUser(id: number): Promise<User> {
    return this.update({ id }, { active: false })
  }
}

export const userService = new UserService()
```

### Repository Pattern

```typescript
// lib/repositories/user.repository.ts
import { prisma } from '../prisma'
import type { User, Prisma } from '@prisma/client'

export class UserRepository {
  async create(data: Prisma.UserCreateInput): Promise<User> {
    return prisma.user.create({ data })
  }

  async findById(id: number): Promise<User | null> {
    return prisma.user.findUnique({ where: { id } })
  }

  async findByEmail(email: string): Promise<User | null> {
    return prisma.user.findUnique({ where: { email } })
  }

  async findMany(params: {
    where?: Prisma.UserWhereInput
    orderBy?: Prisma.UserOrderByWithRelationInput[]
    take?: number
    skip?: number
  }): Promise<User[]> {
    return prisma.user.findMany(params)
  }

  async update(id: number, data: Prisma.UserUpdateInput): Promise<User> {
    return prisma.user.update({ where: { id }, data })
  }

  async delete(id: number): Promise<User> {
    return prisma.user.delete({ where: { id } })
  }

  async count(where?: Prisma.UserWhereInput): Promise<number> {
    return prisma.user.count({ where })
  }
}
```

### Error Handling Pattern

```typescript
// lib/errors/prisma-errors.ts
import { Prisma } from '@prisma/client'

export function handlePrismaError(error: unknown): {
  message: string
  statusCode: number
} {
  if (error instanceof Prisma.PrismaClientKnownRequestError) {
    switch (error.code) {
      case 'P2002':
        return {
          message: 'Unique constraint violation',
          statusCode: 409,
        }
      case 'P2025':
        return {
          message: 'Record not found',
          statusCode: 404,
        }
      case 'P2003':
        return {
          message: 'Foreign key constraint violation',
          statusCode: 400,
        }
      default:
        return {
          message: 'Database error',
          statusCode: 500,
        }
    }
  }

  return {
    message: 'Unknown error',
    statusCode: 500,
  }
}
```

### Validation Layer

```typescript
// lib/validators/user.validator.ts
import { z } from 'zod'

export const createUserSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).default('USER'),
  active: z.boolean().default(true),
})

export const updateUserSchema = z.object({
  email: z.string().email().optional(),
  name: z.string().min(1).max(100).optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).optional(),
  active: z.boolean().optional(),
})

export type CreateUserInput = z.infer<typeof createUserSchema>
export type UpdateUserInput = z.infer<typeof updateUserSchema>
```

### Complete API Route with All Patterns

```typescript
// app/api/users/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { userService } from '@/lib/services/user.service'
import { updateUserSchema } from '@/lib/validators/user.validator'
import { handlePrismaError } from '@/lib/errors/prisma-errors'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    const user = await userService.findUnique({ id }, {
      posts: true,
      profile: true,
    })

    if (!user) {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      )
    }

    return NextResponse.json(user)
  } catch (error) {
    const { message, statusCode } = handlePrismaError(error)
    return NextResponse.json({ error: message }, { status: statusCode })
  }
}

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    const body = await request.json()
    
    // Validate input
    const validation = updateUserSchema.safeParse(body)
    if (!validation.success) {
      return NextResponse.json(
        { error: 'Validation failed', details: validation.error.errors },
        { status: 400 }
      )
    }

    // Check if user exists
    const existing = await userService.findUnique({ id })
    if (!existing) {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      )
    }

    // Update user
    const user = await userService.update({ id }, validation.data, {
      posts: true,
      profile: true,
    })

    return NextResponse.json(user)
  } catch (error) {
    const { message, statusCode } = handlePrismaError(error)
    return NextResponse.json({ error: message }, { status: statusCode })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const id = parseInt(params.id)
    
    if (isNaN(id)) {
      return NextResponse.json(
        { error: 'Invalid user ID' },
        { status: 400 }
      )
    }

    await userService.delete({ id })
    return NextResponse.json({ success: true })
  } catch (error) {
    const { message, statusCode } = handlePrismaError(error)
    return NextResponse.json({ error: message }, { status: statusCode })
  }
}
```

## Best Practices

### 1. Use Type Inference

```typescript
// Infer types from Prisma operations
type UserWithPosts = Prisma.UserGetPayload<{
  include: { posts: true }
}>

// Use in functions
async function getUserWithPosts(id: number): Promise<UserWithPosts | null> {
  return prisma.user.findUnique({
    where: { id },
    include: { posts: true },
  })
}
```

### 2. Handle Prisma Errors

```typescript
try {
  await prisma.user.create({ data })
} catch (error) {
  if (error instanceof Prisma.PrismaClientKnownRequestError) {
    if (error.code === 'P2002') {
      // Handle unique constraint
    }
  }
}
```

### 3. Use Transactions for Complex Operations

```typescript
await prisma.$transaction(async (tx) => {
  const user = await tx.user.create({ data: userData })
  await tx.post.createMany({ data: postsData })
  return user
})
```

### 4. Optimize Queries

```typescript
// Select only needed fields
const users = await prisma.user.findMany({
  select: {
    id: true,
    name: true,
    email: true,
    // Don't fetch posts if not needed
  },
})

// Use include wisely
const user = await prisma.user.findUnique({
  where: { id },
  include: {
    posts: {
      take: 5, // Limit related records
      orderBy: { createdAt: 'desc' },
    },
  },
})
```

## Summary

**Prisma provides end-to-end type safety:**

1. **Schema Definition**: Declarative schema in `schema.prisma`
2. **Generated Client**: Type-safe client with full TypeScript support
3. **Service Layer**: Business logic with type safety
4. **API Routes**: Type-safe API endpoints
5. **Frontend**: Type-safe data fetching and updates

**Key Benefits:**
- **Type Safety**: From database to UI
- **Auto-completion**: IDE support for all operations
- **Compile-time Errors**: Catch errors before runtime
- **Refactoring Safety**: Schema changes propagate through types
- **Developer Experience**: Intuitive API with full type inference

This architecture ensures type safety at every layer, from database schema to UI components, with minimal boilerplate and maximum developer productivity.

