# Type-Safe CRUD Operations with React Admin

This document explains how React Admin supports type-safe CRUD operations on a SQL database via a typed data provider, including type definitions, view mappings, backend integration, and architectural patterns.

## Overview

React Admin provides:
- **Data Provider abstraction**: Unified interface for backend APIs
- **Type-safe operations**: TypeScript support for all CRUD operations
- **Declarative views**: List, Edit, Create, Show components
- **Automatic form handling**: Built-in form management with validation
- **Backend agnostic**: Works with REST, GraphQL, or custom APIs

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              React Admin UI Components                      │
│              - List, Edit, Create, Show                    │
│              - Type-safe hooks (useGetOne, useUpdate)       │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ Data Provider Interface
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Typed Data Provider                            │
│              - getList, getOne, create, update, delete      │
│              - TypeScript generics                          │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ HTTP/GraphQL
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Backend API                                    │
│              - REST / GraphQL / Custom                      │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ SQL Queries
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              SQL Database                                   │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Type Definitions from API Schema

### Define Resource Types

```typescript
// types/user.ts
export interface User {
  id: number
  email: string
  name: string
  age?: number
  role: 'USER' | 'ADMIN' | 'MODERATOR'
  active: boolean
  createdAt: string
  updatedAt: string
  posts?: Post[]
}

export interface Post {
  id: number
  title: string
  content?: string
  published: boolean
  authorId: number
  author?: User
  createdAt: string
  updatedAt: string
}

// Type for creating a user (omits auto-generated fields)
export type CreateUser = Omit<User, 'id' | 'createdAt' | 'updatedAt' | 'posts'>

// Type for updating a user (all fields optional except id)
export type UpdateUser = Partial<Omit<User, 'id' | 'createdAt' | 'updatedAt'>> & {
  id: number
}
```

### Type-Safe Data Provider Interface

```typescript
// types/dataProvider.ts
import { DataProvider, RaRecord } from 'react-admin'

// Extend base DataProvider with resource-specific types
export interface TypedDataProvider extends DataProvider {
  getList<T extends RaRecord = any>(
    resource: string,
    params: {
      pagination: { page: number; perPage: number }
      sort: { field: string; order: 'ASC' | 'DESC' }
      filter: any
      meta?: any
    }
  ): Promise<{ data: T[]; total: number }>

  getOne<T extends RaRecord = any>(
    resource: string,
    params: { id: string | number; meta?: any }
  ): Promise<{ data: T }>

  create<T extends RaRecord = any>(
    resource: string,
    params: { data: Omit<T, 'id'>; meta?: any }
  ): Promise<{ data: T }>

  update<T extends RaRecord = any>(
    resource: string,
    params: {
      id: string | number
      data: Partial<T>
      previousData?: T
      meta?: any
    }
  ): Promise<{ data: T }>

  delete<T extends RaRecord = any>(
    resource: string,
    params: { id: string | number; previousData?: T; meta?: any }
  ): Promise<{ data: T }>
}
```

### REST Data Provider with Types

```typescript
// dataProvider/restDataProvider.ts
import { fetchUtils } from 'react-admin'
import { stringify } from 'query-string'
import type { TypedDataProvider } from '../types/dataProvider'
import type { User, Post } from '../types/user'

const apiUrl = 'http://localhost:3000/api'
const httpClient = fetchUtils.fetchJson

export const restDataProvider: TypedDataProvider = {
  getList: async <T extends RaRecord = any>(
    resource: string,
    params: {
      pagination: { page: number; perPage: number }
      sort: { field: string; order: 'ASC' | 'DESC' }
      filter: any
    }
  ) => {
    const { page, perPage } = params.pagination
    const { field, order } = params.sort

    const query = {
      sort: JSON.stringify([field, order]),
      range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
      filter: JSON.stringify(params.filter),
    }

    const url = `${apiUrl}/${resource}?${stringify(query)}`
    const { json, headers } = await httpClient(url)

    return {
      data: json as T[],
      total: parseInt(headers.get('content-range')?.split('/').pop() || '0', 10),
    }
  },

  getOne: async <T extends RaRecord = any>(
    resource: string,
    params: { id: string | number }
  ) => {
    const url = `${apiUrl}/${resource}/${params.id}`
    const { json } = await httpClient(url)

    return { data: json as T }
  },

  create: async <T extends RaRecord = any>(
    resource: string,
    params: { data: Omit<T, 'id'> }
  ) => {
    const { json } = await httpClient(`${apiUrl}/${resource}`, {
      method: 'POST',
      body: JSON.stringify(params.data),
    })

    return { data: json as T }
  },

  update: async <T extends RaRecord = any>(
    resource: string,
    params: {
      id: string | number
      data: Partial<T>
      previousData?: T
    }
  ) => {
    const url = `${apiUrl}/${resource}/${params.id}`
    const { json } = await httpClient(url, {
      method: 'PUT',
      body: JSON.stringify(params.data),
    })

    return { data: json as T }
  },

  delete: async <T extends RaRecord = any>(
    resource: string,
    params: { id: string | number; previousData?: T }
  ) => {
    const url = `${apiUrl}/${resource}/${params.id}`
    const { json } = await httpClient(url, {
      method: 'DELETE',
    })

    return { data: json as T }
  },

  // Additional required methods
  getMany: async (resource, params) => {
    const query = {
      filter: JSON.stringify({ id: params.ids }),
    }
    const url = `${apiUrl}/${resource}?${stringify(query)}`
    const { json } = await httpClient(url)
    return { data: json }
  },

  getManyReference: async (resource, params) => {
    const { page, perPage } = params.pagination
    const { field, order } = params.sort

    const query = {
      sort: JSON.stringify([field, order]),
      range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
      filter: JSON.stringify({
        ...params.filter,
        [params.target]: params.id,
      }),
    }

    const url = `${apiUrl}/${resource}?${stringify(query)}`
    const { json, headers } = await httpClient(url)

    return {
      data: json,
      total: parseInt(headers.get('content-range')?.split('/').pop() || '0', 10),
    }
  },

  updateMany: async (resource, params) => {
    const query = {
      filter: JSON.stringify({ id: params.ids }),
    }
    const url = `${apiUrl}/${resource}?${stringify(query)}`
    const { json } = await httpClient(url, {
      method: 'PUT',
      body: JSON.stringify(params.data),
    })
    return { data: json }
  },

  deleteMany: async (resource, params) => {
    const query = {
      filter: JSON.stringify({ id: params.ids }),
    }
    const url = `${apiUrl}/${resource}?${stringify(query)}`
    const { json } = await httpClient(url, {
      method: 'DELETE',
    })
    return { data: json }
  },
}
```

## Step 2: List/Edit/Detail Views Map to Types

### List View with Type Safety

```typescript
// resources/users/UserList.tsx
import { List, DataTable, TextField, EmailField, BooleanField, EditButton, ShowButton } from 'react-admin'
import type { User } from '../../types/user'

export const UserList = () => (
  <List>
    <DataTable>
      <DataTable.Col source="id" />
      <DataTable.Col source="name" />
      <DataTable.Col source="email" field={EmailField} />
      <DataTable.Col source="age" />
      <DataTable.Col source="role" />
      <DataTable.Col source="active" field={BooleanField} />
      <DataTable.Col>
        <EditButton />
        <ShowButton />
      </DataTable.Col>
    </DataTable>
  </List>
)
```

### Edit View with Type Safety

```typescript
// resources/users/UserEdit.tsx
import { Edit, SimpleForm, TextInput, EmailInput, NumberInput, SelectInput, BooleanInput } from 'react-admin'
import type { User, UpdateUser } from '../../types/user'

export const UserEdit = () => (
  <Edit>
    <SimpleForm>
      <TextInput source="id" disabled />
      <TextInput source="name" />
      <EmailInput source="email" />
      <NumberInput source="age" />
      <SelectInput
        source="role"
        choices={[
          { id: 'USER', name: 'User' },
          { id: 'ADMIN', name: 'Admin' },
          { id: 'MODERATOR', name: 'Moderator' },
        ]}
      />
      <BooleanInput source="active" />
    </SimpleForm>
  </Edit>
)
```

### Create View with Type Safety

```typescript
// resources/users/UserCreate.tsx
import { Create, SimpleForm, TextInput, EmailInput, NumberInput, SelectInput, BooleanInput } from 'react-admin'
import type { CreateUser } from '../../types/user'

export const UserCreate = () => (
  <Create>
    <SimpleForm>
      <TextInput source="name" required />
      <EmailInput source="email" required />
      <NumberInput source="age" />
      <SelectInput
        source="role"
        defaultValue="USER"
        choices={[
          { id: 'USER', name: 'User' },
          { id: 'ADMIN', name: 'Admin' },
          { id: 'MODERATOR', name: 'Moderator' },
        ]}
      />
      <BooleanInput source="active" defaultValue={true} />
    </SimpleForm>
  </Create>
)
```

### Show (Detail) View with Type Safety

```typescript
// resources/users/UserShow.tsx
import {
  Show,
  SimpleShowLayout,
  TextField,
  EmailField,
  NumberField,
  BooleanField,
  DateField,
  ReferenceManyField,
  SingleFieldList,
  ChipField,
} from 'react-admin'
import type { User } from '../../types/user'

export const UserShow = () => (
  <Show>
    <SimpleShowLayout>
      <TextField source="id" />
      <TextField source="name" />
      <EmailField source="email" />
      <NumberField source="age" />
      <TextField source="role" />
      <BooleanField source="active" />
      <DateField source="createdAt" />
      <DateField source="updatedAt" />
      <ReferenceManyField reference="posts" target="authorId" label="Posts">
        <SingleFieldList>
          <ChipField source="title" />
        </SingleFieldList>
      </ReferenceManyField>
    </SimpleShowLayout>
  </Show>
)
```

### Type-Safe Custom Components

```typescript
// resources/users/components/UserStats.tsx
import { useListContext } from 'react-admin'
import type { User } from '../../../types/user'

export const UserStats = () => {
  const { data: users, isPending } = useListContext<User>()

  if (isPending) return <div>Loading...</div>

  // TypeScript knows users is User[]
  const activeCount = users.filter((u) => u.active).length
  const adminCount = users.filter((u) => u.role === 'ADMIN').length

  return (
    <div>
      <p>Total Users: {users.length}</p>
      <p>Active Users: {activeCount}</p>
      <p>Admins: {adminCount}</p>
    </div>
  )
}
```

## Step 3: Backend API Integration

### REST API Backend

```typescript
// backend/routes/users.ts (Express example)
import { Router } from 'express'
import { db } from '../db'
import type { User, CreateUser, UpdateUser } from '../types/user'

const router = Router()

// GET /api/users - List with pagination, sorting, filtering
router.get('/', async (req, res) => {
  const { page = 1, perPage = 10, sort, filter } = req.query

  const where = filter ? JSON.parse(filter as string) : {}
  const orderBy = sort ? JSON.parse(sort as string) : { createdAt: 'desc' }
  const skip = (Number(page) - 1) * Number(perPage)
  const take = Number(perPage)

  const [users, total] = await Promise.all([
    db.user.findMany({
      where,
      orderBy,
      skip,
      take,
    }),
    db.user.count({ where }),
  ])

  // Set Content-Range header for React Admin
  res.set('Content-Range', `users ${skip}-${skip + users.length - 1}/${total}`)
  res.json(users)
})

// GET /api/users/:id - Get single user
router.get('/:id', async (req, res) => {
  const user = await db.user.findUnique({
    where: { id: Number(req.params.id) },
    include: { posts: true },
  })

  if (!user) {
    return res.status(404).json({ error: 'User not found' })
  }

  res.json(user)
})

// POST /api/users - Create user
router.post('/', async (req, res) => {
  const userData: CreateUser = req.body

  // Validate input
  if (!userData.email || !userData.name) {
    return res.status(400).json({ error: 'Email and name are required' })
  }

  const user = await db.user.create({
    data: userData,
  })

  res.status(201).json(user)
})

// PUT /api/users/:id - Update user
router.put('/:id', async (req, res) => {
  const id = Number(req.params.id)
  const updateData: UpdateUser = req.body

  const user = await db.user.update({
    where: { id },
    data: updateData,
  })

  res.json(user)
})

// DELETE /api/users/:id - Delete user
router.delete('/:id', async (req, res) => {
  const id = Number(req.params.id)

  await db.user.delete({
    where: { id },
  })

  res.json({ id })
})

export default router
```

### GraphQL Data Provider

```typescript
// dataProvider/graphqlDataProvider.ts
import { ApolloClient, InMemoryCache, gql } from '@apollo/client'
import type { TypedDataProvider } from '../types/dataProvider'

const client = new ApolloClient({
  uri: 'http://localhost:4000/graphql',
  cache: new InMemoryCache(),
})

export const graphqlDataProvider: TypedDataProvider = {
  getList: async <T extends RaRecord = any>(resource: string, params: any) => {
    const { page, perPage } = params.pagination
    const { field, order } = params.sort

    const query = gql`
      query Get${resource}($limit: Int!, $offset: Int!, $orderBy: [${resource}_order_by!], $where: ${resource}_bool_exp) {
        ${resource}(limit: $limit, offset: $offset, order_by: $orderBy, where: $where) {
          id
          # ... other fields
        }
        ${resource}_aggregate(where: $where) {
          aggregate {
            count
          }
        }
      }
    `

    const result = await client.query({
      query,
      variables: {
        limit: perPage,
        offset: (page - 1) * perPage,
        orderBy: [{ [field]: order.toLowerCase() }],
        where: params.filter,
      },
    })

    return {
      data: result.data[resource] as T[],
      total: result.data[`${resource}_aggregate`].aggregate.count,
    }
  },

  getOne: async <T extends RaRecord = any>(resource: string, params: { id: string | number }) => {
    const query = gql`
      query Get${resource}ById($id: Int!) {
        ${resource}_by_pk(id: $id) {
          id
          # ... other fields
        }
      }
    `

    const result = await client.query({
      query,
      variables: { id: params.id },
    })

    return { data: result.data[`${resource}_by_pk`] as T }
  },

  create: async <T extends RaRecord = any>(resource: string, params: { data: Omit<T, 'id'> }) => {
    const mutation = gql`
      mutation Create${resource}($object: ${resource}_insert_input!) {
        insert_${resource}_one(object: $object) {
          id
          # ... other fields
        }
      }
    `

    const result = await client.mutate({
      mutation,
      variables: { object: params.data },
    })

    return { data: result.data[`insert_${resource}_one`] as T }
  },

  update: async <T extends RaRecord = any>(
    resource: string,
    params: { id: string | number; data: Partial<T> }
  ) => {
    const mutation = gql`
      mutation Update${resource}($id: Int!, $data: ${resource}_set_input!) {
        update_${resource}_by_pk(pk_columns: { id: $id }, _set: $data) {
          id
          # ... other fields
        }
      }
    `

    const result = await client.mutate({
      mutation,
      variables: { id: params.id, data: params.data },
    })

    return { data: result.data[`update_${resource}_by_pk`] as T }
  },

  delete: async <T extends RaRecord = any>(resource: string, params: { id: string | number }) => {
    const mutation = gql`
      mutation Delete${resource}($id: Int!) {
        delete_${resource}_by_pk(id: $id) {
          id
        }
      }
    `

    const result = await client.mutate({
      mutation,
      variables: { id: params.id },
    })

    return { data: result.data[`delete_${resource}_by_pk`] as T }
  },

  // ... other required methods
}
```

## Step 4: Sample Code for CRUD Operations

### Row Insertion (Create)

```typescript
// Using useCreate hook
import { useCreate } from 'react-admin'
import type { CreateUser } from '../types/user'

export const CreateUserButton = () => {
  const [create, { isLoading }] = useCreate<User>('users')

  const handleCreate = async () => {
    const newUser: CreateUser = {
      email: 'newuser@example.com',
      name: 'New User',
      age: 25,
      role: 'USER',
      active: true,
    }

    try {
      const { data } = await create('users', {
        data: newUser,
      })
      console.log('Created user:', data)
    } catch (error) {
      console.error('Failed to create user:', error)
    }
  }

  return (
    <button onClick={handleCreate} disabled={isLoading}>
      {isLoading ? 'Creating...' : 'Create User'}
    </button>
  )
}
```

```typescript
// Using dataProvider directly
import { useDataProvider } from 'react-admin'
import type { CreateUser } from '../types/user'

export const CreateUserForm = () => {
  const dataProvider = useDataProvider<TypedDataProvider>()

  const handleSubmit = async (formData: CreateUser) => {
    try {
      const { data } = await dataProvider.create<User>('users', {
        data: formData,
      })
      console.log('Created user:', data)
    } catch (error) {
      console.error('Failed to create user:', error)
    }
  }

  // ... form JSX
}
```

### Row Update

```typescript
// Using useUpdate hook
import { useUpdate } from 'react-admin'
import type { UpdateUser } from '../types/user'

export const UpdateUserButton = ({ userId }: { userId: number }) => {
  const [update, { isLoading }] = useUpdate<User>('users')

  const handleUpdate = async () => {
    const updates: UpdateUser = {
      id: userId,
      name: 'Updated Name',
      email: 'updated@example.com',
      active: false,
    }

    try {
      const { data } = await update('users', {
        id: userId,
        data: updates,
      })
      console.log('Updated user:', data)
    } catch (error) {
      console.error('Failed to update user:', error)
    }
  }

  return (
    <button onClick={handleUpdate} disabled={isLoading}>
      {isLoading ? 'Updating...' : 'Update User'}
    </button>
  )
}
```

```typescript
// Using dataProvider directly with optimistic update
import { useDataProvider, useNotify } from 'react-admin'

export const OptimisticUpdateButton = ({ userId }: { userId: number }) => {
  const dataProvider = useDataProvider<TypedDataProvider>()
  const notify = useNotify()

  const handleOptimisticUpdate = async () => {
    try {
      // Optimistic update: update cache immediately
      await dataProvider.update<User>(
        'users',
        {
          id: userId,
          data: { active: true },
        },
        {
          // Optimistic mode
          mutationMode: 'optimistic',
        }
      )
      notify('User updated successfully', { type: 'success' })
    } catch (error) {
      notify('Failed to update user', { type: 'error' })
    }
  }

  return <button onClick={handleOptimisticUpdate}>Toggle Active</button>
}
```

### Row Deletion

```typescript
// Using useDelete hook
import { useDelete } from 'react-admin'

export const DeleteUserButton = ({ userId }: { userId: number }) => {
  const [deleteOne, { isLoading }] = useDelete<User>('users')

  const handleDelete = async () => {
    if (!confirm('Are you sure you want to delete this user?')) {
      return
    }

    try {
      await deleteOne('users', {
        id: userId,
      })
      console.log('User deleted successfully')
    } catch (error) {
      console.error('Failed to delete user:', error)
    }
  }

  return (
    <button onClick={handleDelete} disabled={isLoading}>
      {isLoading ? 'Deleting...' : 'Delete User'}
    </button>
  )
}
```

```typescript
// Using dataProvider directly
import { useDataProvider, useNotify } from 'react-admin'

export const DeleteUserButton = ({ userId }: { userId: number }) => {
  const dataProvider = useDataProvider<TypedDataProvider>()
  const notify = useNotify()

  const handleDelete = async () => {
    if (!confirm('Are you sure?')) return

    try {
      await dataProvider.delete<User>('users', {
        id: userId,
      })
      notify('User deleted successfully', { type: 'success' })
    } catch (error) {
      notify('Failed to delete user', { type: 'error' })
    }
  }

  return <button onClick={handleDelete}>Delete</button>
}
```

### Complete CRUD Example

```typescript
// resources/users/UserList.tsx
import {
  List,
  DataTable,
  TextField,
  EmailField,
  BooleanField,
  EditButton,
  ShowButton,
  DeleteButton,
  CreateButton,
  TopToolbar,
  useDelete,
  useNotify,
} from 'react-admin'
import type { User } from '../../types/user'

const UserListActions = () => (
  <TopToolbar>
    <CreateButton />
  </TopToolbar>
)

export const UserList = () => {
  const notify = useNotify()
  const [deleteOne] = useDelete<User>('users')

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure you want to delete this user?')) {
      return
    }

    try {
      await deleteOne('users', { id })
      notify('User deleted successfully', { type: 'success' })
    } catch (error) {
      notify('Failed to delete user', { type: 'error' })
    }
  }

  return (
    <List actions={<UserListActions />}>
      <DataTable>
        <DataTable.Col source="id" />
        <DataTable.Col source="name" />
        <DataTable.Col source="email" field={EmailField} />
        <DataTable.Col source="age" />
        <DataTable.Col source="role" />
        <DataTable.Col source="active" field={BooleanField} />
        <DataTable.Col>
          <EditButton />
          <ShowButton />
          <DeleteButton mutationOptions={{ onSuccess: () => notify('Deleted!') }} />
        </DataTable.Col>
      </DataTable>
    </List>
  )
}
```

## Step 5: Architectural Choices and Tradeoffs

### 1. Data Provider Abstraction

**Choice**: Abstract backend API behind Data Provider interface

**Benefits:**
- **Backend agnostic**: Switch between REST, GraphQL, or custom APIs
- **Consistent API**: Same interface regardless of backend
- **Easy testing**: Mock data provider for unit tests
- **Type safety**: TypeScript generics ensure type safety

**Tradeoffs:**
- **Abstraction overhead**: Additional layer between UI and backend
- **Learning curve**: Developers must understand Data Provider pattern
- **Customization complexity**: Complex backends may require extensive customization

### 2. Type Safety Strategy

**Choice**: TypeScript generics with explicit type definitions

**Benefits:**
- **Compile-time safety**: Catch errors before runtime
- **Autocomplete**: IDE support for all operations
- **Refactoring safety**: Type changes propagate automatically
- **Documentation**: Types serve as inline documentation

**Tradeoffs:**
- **Type maintenance**: Must keep types in sync with backend
- **Initial setup**: Requires defining types for all resources
- **Complexity**: Generic types can be complex for beginners

### 3. View Component Mapping

**Choice**: Declarative components (List, Edit, Create, Show)

**Benefits:**
- **Rapid development**: Build CRUD interfaces quickly
- **Consistency**: Standardized UI patterns
- **Less boilerplate**: Built-in form handling, validation, routing
- **Accessibility**: Built-in a11y support

**Tradeoffs:**
- **Limited customization**: Harder to customize beyond defaults
- **Bundle size**: Includes Material-UI and other dependencies
- **Learning curve**: Must learn React Admin component API
- **Opinionated**: Follows specific patterns and conventions

### 4. Form Handling

**Choice**: Built-in form management with react-hook-form

**Benefits:**
- **Automatic validation**: Built-in validation support
- **State management**: Handles form state automatically
- **Error handling**: Built-in error display
- **Performance**: Optimized re-renders

**Tradeoffs:**
- **Limited flexibility**: Harder to implement complex custom forms
- **Dependency**: Tied to react-hook-form
- **Learning curve**: Must understand React Admin form patterns

### 5. Caching and State Management

**Choice**: React Query integration

**Benefits:**
- **Automatic caching**: Queries cached automatically
- **Optimistic updates**: Support for optimistic UI updates
- **Background refetching**: Automatic data synchronization
- **Request deduplication**: Multiple requests for same data are deduplicated

**Tradeoffs:**
- **Cache invalidation**: Must manage cache invalidation carefully
- **Complexity**: React Query adds complexity to state management
- **Bundle size**: Additional dependency

### 6. Backend Integration Patterns

**REST Pattern:**
```typescript
// Simple but requires custom implementation
const dataProvider = restDataProvider('http://api.example.com')
```

**GraphQL Pattern:**
```typescript
// More complex but type-safe with codegen
const dataProvider = graphqlDataProvider(client)
```

**Custom Pattern:**
```typescript
// Maximum flexibility but most work
const dataProvider = customDataProvider(apiClient)
```

**Tradeoffs:**
- **REST**: Simple but less type-safe
- **GraphQL**: Type-safe but requires schema setup
- **Custom**: Maximum control but most development effort

## Complete Example: Type-Safe Admin App

```typescript
// App.tsx
import { Admin, Resource } from 'react-admin'
import { restDataProvider } from './dataProvider/restDataProvider'
import { UserList, UserEdit, UserCreate, UserShow } from './resources/users'
import { PostList, PostEdit, PostCreate, PostShow } from './resources/posts'

const App = () => (
  <Admin dataProvider={restDataProvider}>
    <Resource
      name="users"
      list={UserList}
      edit={UserEdit}
      create={UserCreate}
      show={UserShow}
    />
    <Resource
      name="posts"
      list={PostList}
      edit={PostEdit}
      create={PostCreate}
      show={PostShow}
    />
  </Admin>
)

export default App
```

## Best Practices

### 1. Type Definitions

```typescript
// Define types once, use everywhere
export interface User {
  id: number
  email: string
  name: string
}

// Use in components
const { data } = useGetOne<User>('users', { id: 1 })
```

### 2. Data Provider Typing

```typescript
// Extend base DataProvider with generics
export interface TypedDataProvider extends DataProvider {
  getOne<T>(resource: string, params: { id: number }): Promise<{ data: T }>
}
```

### 3. View Components

```typescript
// Use type-safe hooks
const { data: user } = useGetOne<User>('users', { id: 1 })

// Type-safe context
const { data: users } = useListContext<User>()
```

### 4. Error Handling

```typescript
// Type-safe error handling
const [update, { error }] = useUpdate<User>('users')

if (error) {
  // error is typed
  console.error(error.message)
}
```

### 5. Custom Methods

```typescript
// Extend DataProvider with custom methods
export interface CustomDataProvider extends TypedDataProvider {
  archive: (resource: string, params: { id: number }) => Promise<any>
}

// Use with type safety
const dataProvider = useDataProvider<CustomDataProvider>()
await dataProvider.archive('users', { id: 1 })
```

## Summary

**React Admin provides type-safe CRUD operations through:**

1. **Type Definitions**: Define resource types from API schema
2. **Data Provider**: Typed interface for backend communication
3. **View Mapping**: List/Edit/Create/Show components map to types
4. **Backend Integration**: Works with REST, GraphQL, or custom APIs
5. **Type Safety**: TypeScript generics ensure end-to-end type safety

**Key Architectural Choices:**
- **Data Provider abstraction**: Backend agnostic, consistent API
- **Declarative views**: Rapid development, less boilerplate
- **TypeScript generics**: Compile-time safety, autocomplete
- **React Query integration**: Automatic caching and state management

**Tradeoffs:**
- **Abstraction overhead**: Additional layer between UI and backend
- **Limited customization**: Harder to customize beyond defaults
- **Type maintenance**: Must keep types in sync with backend
- **Bundle size**: Includes Material-UI and dependencies

This architecture provides a balance between rapid development, type safety, and flexibility, making it ideal for building admin interfaces quickly while maintaining type safety throughout the application.

