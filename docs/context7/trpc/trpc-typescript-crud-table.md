# Type-Safe CRUD Data Table with tRPC

This document explains the architecture and design of a CRUD application using tRPC for a type-safe data table, including type definitions, client type generation, UI integration, validation, error handling, and complete call stacks.

## Overview

tRPC provides:
- **End-to-end type safety**: TypeScript types flow from server to client
- **No code generation**: Types are inferred at compile time
- **Zod integration**: Runtime validation with type inference
- **React Query integration**: Built-in caching and state management
- **Error handling**: Structured error types

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Server (tRPC Router)                         │
│              - Procedure definitions                        │
│              - Zod schemas                                 │
│              - Type inference                              │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Export AppRouter type
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Client (React)                                │
│              - Import AppRouter type                       │
│              - Type inference                              │
│              - useQuery, useMutation hooks                 │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ HTTP/WebSocket
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Database (Prisma/SQL)                         │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Backend Type Definitions

### Initialize tRPC

```typescript
// server/trpc.ts
import { initTRPC, TRPCError } from '@trpc/server'
import { z } from 'zod'
import type { Context } from './context'

// Initialize tRPC
const t = initTRPC.context<Context>().create()

// Base procedure
export const publicProcedure = t.procedure

// Protected procedure (requires authentication)
export const protectedProcedure = t.procedure.use(async (opts) => {
  if (!opts.ctx.user) {
    throw new TRPCError({
      code: 'UNAUTHORIZED',
      message: 'You must be logged in',
    })
  }
  return opts.next({
    ctx: {
      ...opts.ctx,
      user: opts.ctx.user, // Now guaranteed to be defined
    },
  })
})

// Router helper
export const router = t.router
```

### Context Definition

```typescript
// server/context.ts
import type { inferAsyncReturnType } from '@trpc/server'
import { db } from './db'

export async function createContext(opts: { req: Request; res: Response }) {
  // Get user from session/cookie
  const user = await getUserFromSession(opts.req)
  
  return {
    db,
    user,
    req: opts.req,
    res: opts.res,
  }
}

export type Context = inferAsyncReturnType<typeof createContext>
```

### User Router with Type Definitions

```typescript
// server/routers/user.ts
import { z } from 'zod'
import { router, publicProcedure, protectedProcedure } from '../trpc'
import { TRPCError } from '@trpc/server'
import { db } from '../db'

// Input validation schemas
const createUserSchema = z.object({
  email: z.string().email('Invalid email address'),
  name: z.string().min(1, 'Name is required').max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).default('USER'),
  active: z.boolean().default(true),
})

const updateUserSchema = z.object({
  id: z.number().int().positive(),
  email: z.string().email().optional(),
  name: z.string().min(1).max(100).optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).optional(),
  active: z.boolean().optional(),
})

const getUserSchema = z.object({
  id: z.number().int().positive(),
})

const getUsersSchema = z.object({
  page: z.number().int().positive().default(1),
  limit: z.number().int().positive().max(100).default(10),
  search: z.string().optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).optional(),
})

const deleteUserSchema = z.object({
  id: z.number().int().positive(),
})

// User router
export const userRouter = router({
  // Query: Get all users with pagination
  list: publicProcedure
    .input(getUsersSchema)
    .query(async ({ input, ctx }) => {
      const { page, limit, search, role } = input
      
      const where = {
        active: true,
        ...(search && {
          OR: [
            { name: { contains: search, mode: 'insensitive' } },
            { email: { contains: search, mode: 'insensitive' } },
          ],
        }),
        ...(role && { role }),
      }

      const [users, total] = await Promise.all([
        ctx.db.user.findMany({
          where,
          take: limit,
          skip: (page - 1) * limit,
          orderBy: { createdAt: 'desc' },
          include: {
            posts: {
              select: { id: true, title: true },
              take: 3,
            },
          },
        }),
        ctx.db.user.count({ where }),
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
    }),

  // Query: Get single user by ID
  byId: publicProcedure
    .input(getUserSchema)
    .query(async ({ input, ctx }) => {
      const user = await ctx.db.user.findUnique({
        where: { id: input.id },
        include: {
          posts: true,
        },
      })

      if (!user) {
        throw new TRPCError({
          code: 'NOT_FOUND',
          message: `User with id ${input.id} not found`,
        })
      }

      return user
    }),

  // Mutation: Create user
  create: protectedProcedure
    .input(createUserSchema)
    .mutation(async ({ input, ctx }) => {
      // Check if email already exists
      const existing = await ctx.db.user.findUnique({
        where: { email: input.email },
      })

      if (existing) {
        throw new TRPCError({
          code: 'CONFLICT',
          message: 'Email already exists',
        })
      }

      const user = await ctx.db.user.create({
        data: input,
        include: {
          posts: true,
        },
      })

      return user
    }),

  // Mutation: Update user
  update: protectedProcedure
    .input(updateUserSchema)
    .mutation(async ({ input, ctx }) => {
      const { id, ...updateData } = input

      // Verify user exists
      const existing = await ctx.db.user.findUnique({
        where: { id },
      })

      if (!existing) {
        throw new TRPCError({
          code: 'NOT_FOUND',
          message: `User with id ${id} not found`,
        })
      }

      // Check email uniqueness if changing
      if (updateData.email && updateData.email !== existing.email) {
        const emailExists = await ctx.db.user.findUnique({
          where: { email: updateData.email },
        })

        if (emailExists) {
          throw new TRPCError({
            code: 'CONFLICT',
            message: 'Email already exists',
          })
        }
      }

      // Authorization: Only admins or the user themselves can update
      if (ctx.user.role !== 'ADMIN' && ctx.user.id !== id) {
        throw new TRPCError({
          code: 'FORBIDDEN',
          message: 'You can only update your own profile',
        })
      }

      const user = await ctx.db.user.update({
        where: { id },
        data: updateData,
        include: {
          posts: true,
        },
      })

      return user
    }),

  // Mutation: Delete user
  delete: protectedProcedure
    .input(deleteUserSchema)
    .mutation(async ({ input, ctx }) => {
      const user = await ctx.db.user.findUnique({
        where: { id: input.id },
      })

      if (!user) {
        throw new TRPCError({
          code: 'NOT_FOUND',
          message: `User with id ${input.id} not found`,
        })
      }

      // Authorization: Only admins can delete
      if (ctx.user.role !== 'ADMIN') {
        throw new TRPCError({
          code: 'FORBIDDEN',
          message: 'Only admins can delete users',
        })
      }

      await ctx.db.user.delete({
        where: { id: input.id },
      })

      return { success: true }
    }),
})
```

### App Router

```typescript
// server/routers/_app.ts
import { router } from '../trpc'
import { userRouter } from './user'
import { postRouter } from './post'

export const appRouter = router({
  user: userRouter,
  post: postRouter,
})

// Export type for client inference
export type AppRouter = typeof appRouter
```

## Step 2: Client Type Generation

### How tRPC Generates Client Types

tRPC uses TypeScript's type inference to generate client types:

1. **Server exports `AppRouter` type**: `export type AppRouter = typeof appRouter`
2. **Client imports the type**: `import type { AppRouter } from './server/routers/_app'`
3. **TypeScript infers all types**: Inputs, outputs, and procedure paths are automatically inferred
4. **No code generation needed**: Types are available at compile time

### Client Setup

```typescript
// utils/trpc.ts
import { createTRPCReact } from '@trpc/react-query'
import type { inferRouterInputs, inferRouterOutputs } from '@trpc/server'
import type { AppRouter } from '../server/routers/_app'

// Create tRPC React hooks
export const trpc = createTRPCReact<AppRouter>()

// Type inference helpers
export type RouterInputs = inferRouterInputs<AppRouter>
export type RouterOutputs = inferRouterOutputs<AppRouter>

// Specific type exports for convenience
export type UserListInput = RouterInputs['user']['list']
export type UserListOutput = RouterOutputs['user']['list']
export type UserByIdInput = RouterInputs['user']['byId']
export type UserByIdOutput = RouterOutputs['user']['byId']
export type UserCreateInput = RouterInputs['user']['create']
export type UserCreateOutput = RouterOutputs['user']['create']
export type UserUpdateInput = RouterInputs['user']['update']
export type UserUpdateOutput = RouterOutputs['user']['update']
export type UserDeleteInput = RouterInputs['user']['delete']
export type UserDeleteOutput = RouterOutputs['user']['delete']
```

### tRPC Client Provider

```typescript
// app/providers.tsx
'use client'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { httpBatchLink } from '@trpc/client'
import { useState } from 'react'
import { trpc } from '../utils/trpc'

export function TRPCProvider({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => new QueryClient())
  const [trpcClient] = useState(() =>
    trpc.createClient({
      links: [
        httpBatchLink({
          url: '/api/trpc',
          // Optional: Add headers
          headers: async () => {
            return {
              authorization: `Bearer ${getToken()}`,
            }
          },
        }),
      ],
    })
  )

  return (
    <trpc.Provider client={trpcClient} queryClient={queryClient}>
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    </trpc.Provider>
  )
}
```

## Step 3: UI Table with Full Type Safety

### Users Table Component

```typescript
// components/UsersTable.tsx
'use client'

import { useState } from 'react'
import { trpc } from '../utils/trpc'
import type { RouterOutputs } from '../utils/trpc'

type User = RouterOutputs['user']['list']['users'][0]

export function UsersTable() {
  const [page, setPage] = useState(1)
  const [search, setSearch] = useState('')
  const [editingId, setEditingId] = useState<number | null>(null)

  // Type-safe query hook
  // Input type is automatically inferred from server schema
  const { data, isLoading, error, refetch } = trpc.user.list.useQuery({
    page,
    limit: 10,
    search: search || undefined,
  })

  // Type-safe mutation hooks
  const createMutation = trpc.user.create.useMutation({
    onSuccess: () => {
      // Invalidate and refetch
      utils.user.list.invalidate()
      refetch()
    },
  })

  const updateMutation = trpc.user.update.useMutation({
    onSuccess: () => {
      setEditingId(null)
      utils.user.list.invalidate()
      refetch()
    },
  })

  const deleteMutation = trpc.user.update.useMutation({
    onSuccess: () => {
      utils.user.list.invalidate()
      refetch()
    },
  })

  // Get tRPC utils for cache management
  const utils = trpc.useUtils()

  const handleCreate = async (data: {
    email: string
    name: string
    age?: number
    role: 'USER' | 'ADMIN' | 'MODERATOR'
  }) => {
    try {
      await createMutation.mutateAsync(data)
    } catch (error) {
      // Error is typed as TRPCClientError<AppRouter>
      console.error('Failed to create user:', error)
    }
  }

  const handleUpdate = async (id: number, data: Partial<User>) => {
    try {
      await updateMutation.mutateAsync({ id, ...data })
    } catch (error) {
      console.error('Failed to update user:', error)
    }
  }

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure you want to delete this user?')) {
      return
    }

    try {
      await deleteMutation.mutateAsync({ id })
    } catch (error) {
      console.error('Failed to delete user:', error)
    }
  }

  if (isLoading) return <div>Loading...</div>
  if (error) return <div>Error: {error.message}</div>
  if (!data) return null

  return (
    <div>
      <div>
        <input
          type="text"
          placeholder="Search users..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        <button onClick={() => handleCreate({ email: 'new@example.com', name: 'New User', role: 'USER' })}>
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
          {data.users.map((user) => (
            <UserRow
              key={user.id}
              user={user}
              editing={editingId === user.id}
              onEdit={() => setEditingId(user.id)}
              onCancel={() => setEditingId(null)}
              onUpdate={(data) => handleUpdate(user.id, data)}
              onDelete={() => handleDelete(user.id)}
              isUpdating={updateMutation.isPending}
              isDeleting={deleteMutation.isPending}
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
          Page {data.pagination.page} of {data.pagination.totalPages}
        </span>
        <button
          onClick={() => setPage((p) => p + 1)}
          disabled={page >= data.pagination.totalPages}
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
  user: User
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
    age: user.age?.toString() || '',
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
            {isUpdating ? 'Saving...' : 'Save'}
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
      <td>{user.age || '-'}</td>
      <td>{user.role}</td>
      <td>{user.posts.length}</td>
      <td>
        <button onClick={onEdit}>Edit</button>
        <button onClick={onDelete} disabled={isDeleting}>
          {isDeleting ? 'Deleting...' : 'Delete'}
        </button>
      </td>
    </tr>
  )
}
```

### Create User Form

```typescript
// components/CreateUserForm.tsx
'use client'

import { useState } from 'react'
import { trpc } from '../utils/trpc'
import type { RouterInputs } from '../utils/trpc'

type CreateUserInput = RouterInputs['user']['create']

export function CreateUserForm() {
  const [formData, setFormData] = useState<CreateUserInput>({
    email: '',
    name: '',
    age: undefined,
    role: 'USER',
    active: true,
  })
  const [errors, setErrors] = useState<Record<string, string>>({})

  const utils = trpc.useUtils()
  const createMutation = trpc.user.create.useMutation({
    onSuccess: () => {
      // Invalidate and refetch
      utils.user.list.invalidate()
      // Reset form
      setFormData({ email: '', name: '', role: 'USER', active: true })
      setErrors({})
    },
    onError: (error) => {
      // Handle validation errors
      if (error.data?.zodError) {
        const fieldErrors: Record<string, string> = {}
        error.data.zodError.fieldErrors.forEach((err, field) => {
          fieldErrors[field] = err[0]
        })
        setErrors(fieldErrors)
      }
    },
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setErrors({})

    try {
      await createMutation.mutateAsync(formData)
    } catch (error) {
      // Error handling is done in onError callback
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
        {errors.email && <span style={{ color: 'red' }}>{errors.email}</span>}
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
        {errors.name && <span style={{ color: 'red' }}>{errors.name}</span>}
      </div>

      <div>
        <label>
          Age:
          <input
            type="number"
            value={formData.age || ''}
            onChange={(e) =>
              setFormData({
                ...formData,
                age: e.target.value ? parseInt(e.target.value) : undefined,
              })
            }
          />
        </label>
        {errors.age && <span style={{ color: 'red' }}>{errors.age}</span>}
      </div>

      <div>
        <label>
          Role:
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
        </label>
        {errors.role && <span style={{ color: 'red' }}>{errors.role}</span>}
      </div>

      <button type="submit" disabled={createMutation.isPending}>
        {createMutation.isPending ? 'Creating...' : 'Create User'}
      </button>

      {createMutation.error && (
        <div style={{ color: 'red' }}>
          {createMutation.error.message}
        </div>
      )}
    </form>
  )
}
```

## Step 4: API Usage Patterns

### Query Patterns

```typescript
// Basic query
const { data, isLoading, error } = trpc.user.list.useQuery({
  page: 1,
  limit: 10,
})

// Query with options
const { data } = trpc.user.byId.useQuery(
  { id: 1 },
  {
    enabled: !!userId, // Conditional fetching
    staleTime: 5000, // Cache for 5 seconds
    refetchOnWindowFocus: false,
  }
)

// Dependent queries
const user = trpc.user.byId.useQuery({ id: userId })
const posts = trpc.post.list.useQuery(
  { userId },
  { enabled: !!user.data } // Only fetch when user is loaded
)
```

### Mutation Patterns

```typescript
// Basic mutation
const mutation = trpc.user.create.useMutation()

// Mutation with callbacks
const mutation = trpc.user.update.useMutation({
  onSuccess: (data) => {
    // Invalidate related queries
    utils.user.list.invalidate()
    utils.user.byId.invalidate({ id: data.id })
    // Show success message
    toast.success('User updated successfully')
  },
  onError: (error) => {
    // Handle error
    toast.error(error.message)
  },
})

// Optimistic updates
const mutation = trpc.user.update.useMutation({
  onMutate: async (newData) => {
    // Cancel outgoing refetches
    await utils.user.byId.cancel({ id: newData.id })
    
    // Snapshot previous value
    const previous = utils.user.byId.getData({ id: newData.id })
    
    // Optimistically update
    utils.user.byId.setData({ id: newData.id }, (old) => ({
      ...old!,
      ...newData,
    }))
    
    return { previous }
  },
  onError: (err, newData, context) => {
    // Rollback on error
    utils.user.byId.setData({ id: newData.id }, context?.previous)
  },
  onSettled: () => {
    // Refetch to ensure consistency
    utils.user.byId.invalidate({ id: mutation.variables?.id })
  },
})
```

### Cache Management

```typescript
const utils = trpc.useUtils()

// Invalidate queries
utils.user.list.invalidate()
utils.user.byId.invalidate({ id: 1 })

// Set query data directly
utils.user.byId.setData({ id: 1 }, newUserData)

// Reset query
utils.user.list.reset()

// Prefetch data
utils.user.byId.prefetch({ id: 1 })
```

## Step 5: Validation Strategies

### Server-Side Validation

```typescript
// server/routers/user.ts
import { z } from 'zod'

// Zod schema with custom validation
const createUserSchema = z.object({
  email: z.string().email('Invalid email address'),
  name: z.string().min(1, 'Name is required').max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).default('USER'),
}).refine(
  (data) => {
    // Custom validation logic
    if (data.role === 'ADMIN' && !data.email.endsWith('@company.com')) {
      return false
    }
    return true
  },
  {
    message: 'Admin users must have company email',
    path: ['email'],
  }
)

export const userRouter = router({
  create: protectedProcedure
    .input(createUserSchema) // Automatic validation
    .mutation(async ({ input, ctx }) => {
      // input is typed and validated
      const user = await ctx.db.user.create({ data: input })
      return user
    }),
})
```

### Client-Side Validation

```typescript
// components/CreateUserForm.tsx
import { z } from 'zod'
import type { RouterInputs } from '../utils/trpc'

// Reuse server schema on client
const createUserSchema = z.object({
  email: z.string().email('Invalid email address'),
  name: z.string().min(1, 'Name is required').max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).default('USER'),
})

export function CreateUserForm() {
  const [formData, setFormData] = useState<RouterInputs['user']['create']>({
    email: '',
    name: '',
    role: 'USER',
  })
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    // Client-side validation
    const result = createUserSchema.safeParse(formData)
    
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      result.error.errors.forEach((err) => {
        if (err.path[0]) {
          fieldErrors[err.path[0] as string] = err.message
        }
      })
      setErrors(fieldErrors)
      return
    }

    // Proceed with mutation
    await createMutation.mutateAsync(result.data)
  }

  // ...
}
```

### Shared Validation Schemas

```typescript
// shared/validations/user.ts
import { z } from 'zod'

// Export schemas for reuse on client and server
export const createUserSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
  age: z.number().int().min(0).max(150).optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).default('USER'),
})

export const updateUserSchema = z.object({
  id: z.number().int().positive(),
  email: z.string().email().optional(),
  name: z.string().min(1).max(100).optional(),
  age: z.number().int().min(0).max(150).nullable().optional(),
  role: z.enum(['USER', 'ADMIN', 'MODERATOR']).optional(),
})

// Type inference
export type CreateUserInput = z.infer<typeof createUserSchema>
export type UpdateUserInput = z.infer<typeof updateUserSchema>
```

## Step 6: Error Handling

### Server-Side Error Handling

```typescript
// server/routers/user.ts
import { TRPCError } from '@trpc/server'

export const userRouter = router({
  update: protectedProcedure
    .input(updateUserSchema)
    .mutation(async ({ input, ctx }) => {
      try {
        const user = await ctx.db.user.update({
          where: { id: input.id },
          data: input,
        })
        return user
      } catch (error: any) {
        // Handle Prisma errors
        if (error.code === 'P2025') {
          throw new TRPCError({
            code: 'NOT_FOUND',
            message: 'User not found',
          })
        }
        if (error.code === 'P2002') {
          throw new TRPCError({
            code: 'CONFLICT',
            message: 'Email already exists',
          })
        }
        // Re-throw unknown errors
        throw error
      }
    }),
})
```

### Client-Side Error Handling

```typescript
// components/UsersTable.tsx
import { TRPCClientError } from '@trpc/client'
import type { AppRouter } from '../server/routers/_app'

export function UsersTable() {
  const updateMutation = trpc.user.update.useMutation({
    onError: (error: TRPCClientError<AppRouter>) => {
      // Type-safe error handling
      switch (error.data?.code) {
        case 'NOT_FOUND':
          toast.error('User not found')
          break
        case 'CONFLICT':
          toast.error('Email already exists')
          break
        case 'FORBIDDEN':
          toast.error('You do not have permission to update this user')
          break
        case 'UNAUTHORIZED':
          toast.error('You must be logged in')
          break
        default:
          toast.error(error.message || 'An error occurred')
      }

      // Handle Zod validation errors
      if (error.data?.zodError) {
        const fieldErrors: Record<string, string> = {}
        error.data.zodError.fieldErrors.forEach((err, field) => {
          fieldErrors[field] = err[0]
        })
        setErrors(fieldErrors)
      }
    },
  })

  // ...
}
```

### Error Formatting

```typescript
// server/trpc.ts
import { TRPCError, initTRPC } from '@trpc/server'
import { ZodError } from 'zod'

const t = initTRPC.context<Context>().create({
  errorFormatter({ shape, error }) {
    return {
      ...shape,
      data: {
        ...shape.data,
        zodError:
          error.code === 'BAD_REQUEST' && error.cause instanceof ZodError
            ? error.cause.flatten()
            : null,
      },
    }
  },
})
```

## Step 7: Complete Call Stack

### Call Stack: User Edit → Table → tRPC Mutation → DB

```
1. User clicks "Edit" button in table
   ↓
2. React: UserRow.onEdit() called
   ↓
3. React: setEditingId(user.id) - Switch to edit mode
   ↓
4. React: UserRow renders edit form with current values
   ↓
5. User modifies fields (name, email, age, role)
   ↓
6. React: onChange handlers update formData state
   ↓
7. User clicks "Save" button
   ↓
8. React: UserRow.onUpdate() called with formData
   ↓
9. React: handleUpdate(user.id, formData) in UsersTable
   ↓
10. tRPC Client: updateMutation.mutateAsync({ id, ...formData })
   ↓
11. tRPC Client: HTTP POST /api/trpc/user.update
    {
      "0": {
        "json": {
          "id": 1,
          "name": "Updated Name",
          "email": "updated@example.com",
          "age": 30,
          "role": "ADMIN"
        }
      }
    }
   ↓
12. tRPC Server: Receives request at /api/trpc/user.update
   ↓
13. tRPC Server: Router resolves to userRouter.update procedure
   ↓
14. tRPC Server: Input validation
    a. Zod schema validation (updateUserSchema)
    b. If invalid → Return BAD_REQUEST error with zodError
    c. If valid → Continue
   ↓
15. tRPC Server: Context creation
    a. Extract user from session/cookie
    b. Create context with db, user, req, res
   ↓
16. tRPC Server: Procedure execution
    a. protectedProcedure middleware checks authentication
    b. If not authenticated → Throw UNAUTHORIZED error
    c. If authenticated → Continue
   ↓
17. tRPC Server: Business logic
    a. Verify user exists: db.user.findUnique({ where: { id } })
    b. If not found → Throw NOT_FOUND error
    c. Check email uniqueness (if changing)
    d. Authorization check (admin or self)
    e. If unauthorized → Throw FORBIDDEN error
   ↓
18. Prisma Client: db.user.update()
    a. Generate SQL: UPDATE users SET ... WHERE id = ?
    b. Execute transaction
   ↓
19. PostgreSQL: Execute UPDATE statement
    a. Lock row
    b. Update columns
    c. Return updated row
   ↓
20. Prisma Client: Transform result to User object
   ↓
21. tRPC Server: Return user object
   ↓
22. HTTP Response: { "result": { "data": { ...user } } }
   ↓
23. tRPC Client: Parse response
   ↓
24. React Query: Update cache
    a. onSuccess callback executes
    b. utils.user.list.invalidate()
    c. utils.user.byId.invalidate({ id })
   ↓
25. React: setEditingId(null) - Exit edit mode
   ↓
26. React: refetch() - Refetch user list
   ↓
27. tRPC Client: HTTP POST /api/trpc/user.list
   ↓
28. tRPC Server: Execute user.list query
   ↓
29. Prisma Client: db.user.findMany()
   ↓
30. PostgreSQL: Execute SELECT statement
   ↓
31. HTTP Response: { "result": { "data": { users: [...], pagination: {...} } } }
   ↓
32. React Query: Update cache with new data
   ↓
33. React: Re-render UsersTable with updated data
   ↓
34. React: UserRow displays updated values
```

### Error Path in Call Stack

```
If error occurs at step 14 (validation):
   ↓
15. Zod validation fails
   ↓
16. tRPC throws BAD_REQUEST error with zodError
   ↓
17. Error formatter adds zodError to error.data
   ↓
18. HTTP Response: { "error": { "code": "BAD_REQUEST", "data": { "zodError": {...} } } }
   ↓
19. tRPC Client: Parse error
   ↓
20. React Query: onError callback executes
   ↓
21. React: setErrors() with field errors
   ↓
22. React: Re-render form with error messages
```

## Best Practices

### 1. Type Inference

```typescript
// Use inference helpers
type UserListOutput = RouterOutputs['user']['list']
type User = UserListOutput['users'][0]

// Use in components
function UserCard({ user }: { user: User }) {
  // user is fully typed
}
```

### 2. Shared Schemas

```typescript
// Define schemas once, use everywhere
export const createUserSchema = z.object({...})

// Server
.input(createUserSchema)

// Client
const result = createUserSchema.safeParse(formData)
```

### 3. Error Handling

```typescript
// Use error codes for consistent handling
throw new TRPCError({
  code: 'NOT_FOUND',
  message: 'User not found',
})

// Handle on client
if (error.data?.code === 'NOT_FOUND') {
  // Handle not found
}
```

### 4. Cache Management

```typescript
// Invalidate related queries
utils.user.list.invalidate()
utils.user.byId.invalidate({ id })

// Optimistic updates
utils.user.byId.setData({ id }, newData)
```

### 5. Authorization

```typescript
// Use protected procedures
export const protectedProcedure = t.procedure.use(async (opts) => {
  if (!opts.ctx.user) {
    throw new TRPCError({ code: 'UNAUTHORIZED' })
  }
  return opts.next()
})
```

## Summary

**tRPC provides end-to-end type safety:**

1. **Backend Types**: Defined in Zod schemas and TypeScript
2. **Client Types**: Automatically inferred from AppRouter
3. **UI Integration**: Full type safety with useQuery/useMutation
4. **Validation**: Zod schemas on both client and server
5. **Error Handling**: Structured errors with type safety
6. **No Code Generation**: Types inferred at compile time

**Key Benefits:**
- **Type Safety**: From database to UI
- **Developer Experience**: Autocomplete and type checking
- **Runtime Validation**: Zod ensures data integrity
- **Error Handling**: Structured, type-safe errors
- **Performance**: Optimized with React Query caching

This architecture ensures type safety at every layer, from database operations to UI components, with minimal boilerplate and maximum developer productivity.

