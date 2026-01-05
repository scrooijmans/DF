# Type-Safe Data Table with Hasura (PostgreSQL + GraphQL)

This document explains how to build a type-safe, editable data table using Hasura GraphQL Engine, PostgreSQL, and TypeScript, including schema definition, GraphQL schema generation, frontend type generation, CRUD operations, and real-time subscriptions.

## Overview

Hasura automatically generates a GraphQL API from your PostgreSQL schema, providing:

- **Auto-generated GraphQL schema**: Based on your database tables
- **Type-safe operations**: TypeScript types generated from GraphQL schema
- **Real-time subscriptions**: Live queries for instant updates
- **CRUD operations**: Queries and mutations for all operations
- **Relationship queries**: Automatic handling of foreign keys

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL Database                            │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Tables     │  │  Views      │  │  Functions  │      │
│  │              │  │              │  │            │      │
│  └──────┬───────┘  └──────┬──────┘  └──────┬─────┘      │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            │                                 │
│                            ▼                                 │
│                    Hasura GraphQL Engine                     │
│                    (Auto-generates schema)                   │
└────────────────────────────┼─────────────────────────────────┘
                             │
                             │ GraphQL API
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend (TypeScript)                          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ GraphQL     │  │  TypeScript  │  │  React      │      │
│  │ Codegen     │  │  Types       │  │  Components │      │
│  └──────┬──────┘  └──────┬──────┘  └──────┬─────┘      │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            │                                 │
│                            ▼                                 │
│                    Type-Safe Data Table                      │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: PostgreSQL Schema Definition

### Define Tables

```sql
-- Create users table
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  age INTEGER CHECK (age > 0 AND age < 150),
  role TEXT DEFAULT 'user' CHECK (role IN ('admin', 'user', 'guest')),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Create articles table with foreign key
CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT,
  author_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
  published BOOLEAN DEFAULT false,
  rating INTEGER CHECK (rating >= 0 AND rating <= 5),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_articles_author_id ON articles(author_id);
CREATE INDEX idx_articles_published ON articles(published);
CREATE INDEX idx_users_email ON users(email);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to tables
CREATE TRIGGER update_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_articles_updated_at
  BEFORE UPDATE ON articles
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

### Track Tables in Hasura

After creating tables in PostgreSQL, track them in Hasura:

1. **Via Hasura Console**: Navigate to Data → Tables → Track All
2. **Via Metadata API**: Use Hasura's metadata API to track tables programmatically

```bash
# Track users table
curl -X POST \
  http://localhost:8080/v1/metadata \
  -H 'Content-Type: application/json' \
  -H 'X-Hasura-Admin-Secret: myadminsecretkey' \
  -d '{
    "type": "pg_track_table",
    "args": {
      "source": "default",
      "schema": "public",
      "name": "users"
    }
  }'

# Track articles table
curl -X POST \
  http://localhost:8080/v1/metadata \
  -H 'Content-Type: application/json' \
  -H 'X-Hasura-Admin-Secret: myadminsecretkey' \
  -d '{
    "type": "pg_track_table",
    "args": {
      "source": "default",
      "schema": "public",
      "name": "articles"
    }
  }'
```

## Step 2: GraphQL Schema Generation

Hasura automatically generates a GraphQL schema from your PostgreSQL tables. The schema includes:

### Auto-Generated Query Types

```graphql
# Query all users
type Query {
	users(
		distinct_on: [users_select_column!]
		where: users_bool_exp
		order_by: [users_order_by!]
		limit: Int
		offset: Int
	): [users!]!

	# Query single user by primary key
	users_by_pk(id: Int!): users

	# Query articles
	articles(
		distinct_on: [articles_select_column!]
		where: articles_bool_exp
		order_by: [articles_order_by!]
		limit: Int
		offset: Int
	): [articles!]!

	articles_by_pk(id: Int!): articles
}

# User type (auto-generated from table)
type users {
	id: Int!
	name: String!
	email: String!
	age: Int
	role: String!
	created_at: timestamptz!
	updated_at: timestamptz!

	# Relationship (auto-generated from foreign key)
	articles: [articles!]!
}

# Article type (auto-generated from table)
type articles {
	id: Int!
	title: String!
	content: String
	author_id: Int
	published: Boolean!
	rating: Int
	created_at: timestamptz!
	updated_at: timestamptz!

	# Relationship (auto-generated from foreign key)
	author: users
}
```

### Auto-Generated Mutation Types

```graphql
type Mutation {
  # Insert users
  insert_users(
    objects: [users_insert_input!]!
    on_conflict: users_on_conflict
  ): users_mutation_response

  insert_users_one(
    object: users_insert_input!
    on_conflict: users_on_conflict
  ): users

  # Update users
  update_users(
    where: users_bool_exp!
    _set: users_set_input
    _inc: users_inc_input
  ): users_mutation_response

  update_users_by_pk(
    pk_columns: users_pk_columns_input!
    _set: users_set_input
    _inc: users_inc_input
  ): users

  # Delete users
  delete_users(
    where: users_bool_exp!
  ): users_mutation_response

  delete_users_by_pk(id: Int!): users

  # Similar mutations for articles...
  insert_articles(...): articles_mutation_response
  update_articles(...): articles_mutation_response
  delete_articles(...): articles_mutation_response
}

# Mutation response type
type users_mutation_response {
  affected_rows: Int!
  returning: [users!]!
}
```

### Auto-Generated Subscription Types

```graphql
type Subscription {
  # Live query subscriptions (same structure as queries)
  users(
    distinct_on: [users_select_column!]
    where: users_bool_exp
    order_by: [users_order_by!]
    limit: Int
    offset: Int
  ): [users!]!

  users_by_pk(id: Int!): users

  articles(...): [articles!]!
  articles_by_pk(id: Int!): articles
}
```

## Step 3: Frontend Type Generation

### Install GraphQL Code Generator

```bash
npm install -D @graphql-codegen/cli @graphql-codegen/typescript @graphql-codegen/typescript-operations @graphql-codegen/typescript-react-apollo
```

### Configure Code Generator

Create `codegen.yml`:

```yaml
schema: http://localhost:8080/v1/graphql
documents: 'src/**/*.graphql'
generates:
  src/generated/graphql.ts:
    plugins:
      - typescript
      - typescript-operations
      - typescript-react-apollo
    config:
      withHooks: true
      withComponent: false
      withHOC: false
      skipTypename: true
      scalars:
        timestamptz: string
        uuid: string
```

### Define GraphQL Operations

Create `src/graphql/users.graphql`:

```graphql
# Query all users
query GetUsers($limit: Int, $offset: Int, $orderBy: [users_order_by!]) {
	users(limit: $limit, offset: $offset, order_by: $orderBy) {
		id
		name
		email
		age
		role
		created_at
		updated_at
		articles {
			id
			title
			published
		}
	}
}

# Query single user
query GetUser($id: Int!) {
	users_by_pk(id: $id) {
		id
		name
		email
		age
		role
		created_at
		updated_at
		articles {
			id
			title
			content
			published
			rating
		}
	}
}

# Insert user mutation
mutation InsertUser($object: users_insert_input!) {
	insert_users_one(object: $object) {
		id
		name
		email
		age
		role
		created_at
		updated_at
	}
}

# Update user mutation
mutation UpdateUser($id: Int!, $changes: users_set_input!) {
	update_users_by_pk(pk_columns: { id: $id }, _set: $changes) {
		id
		name
		email
		age
		role
		created_at
		updated_at
	}
}

# Delete user mutation
mutation DeleteUser($id: Int!) {
	delete_users_by_pk(id: $id) {
		id
	}
}
```

Create `src/graphql/users.subscription.graphql`:

```graphql
# Subscribe to users (live query)
subscription SubscribeUsers($limit: Int, $offset: Int, $orderBy: [users_order_by!]) {
	users(limit: $limit, offset: $offset, order_by: $orderBy) {
		id
		name
		email
		age
		role
		created_at
		updated_at
		articles {
			id
			title
			published
		}
	}
}

# Subscribe to single user
subscription SubscribeUser($id: Int!) {
	users_by_pk(id: $id) {
		id
		name
		email
		age
		role
		created_at
		updated_at
		articles {
			id
			title
			content
			published
			rating
		}
	}
}
```

### Generate TypeScript Types

Add to `package.json`:

```json
{
	"scripts": {
		"codegen": "graphql-codegen --config codegen.yml",
		"codegen:watch": "graphql-codegen --config codegen.yml --watch"
	}
}
```

Run code generation:

```bash
npm run codegen
```

This generates `src/generated/graphql.ts` with:

- TypeScript types for all GraphQL types
- Typed hooks for React (if using React Apollo)
- Type-safe operation types

### Generated Types Example

```typescript
// Generated types from GraphQL schema
export type Users = {
	__typename?: 'users';
	id: number;
	name: string;
	email: string;
	age?: Maybe<number>;
	role: string;
	created_at: string;
	updated_at: string;
	articles: Array<Articles>;
};

export type GetUsersQuery = {
	__typename?: 'Query';
	users: Array<Users>;
};

export type GetUsersQueryVariables = {
	limit?: Maybe<number>;
	offset?: Maybe<number>;
	orderBy?: Maybe<Array<Users_Order_By>>;
};

// Generated React hooks
export function useGetUsersQuery(
	baseOptions?: Apollo.QueryHookOptions<GetUsersQuery, GetUsersQueryVariables>
) {
	return Apollo.useQuery<GetUsersQuery, GetUsersQueryVariables>(GetUsersDocument, baseOptions);
}

export function useSubscribeUsersSubscription(
	baseOptions?: Apollo.SubscriptionHookOptions<
		SubscribeUsersSubscription,
		SubscribeUsersSubscriptionVariables
	>
) {
	return Apollo.useSubscription<SubscribeUsersSubscription, SubscribeUsersSubscriptionVariables>(
		SubscribeUsersDocument,
		baseOptions
	);
}
```

## Step 4: CRUD Operations

### Setup Apollo Client

```typescript
// src/lib/apollo-client.ts
import { ApolloClient, InMemoryCache, createHttpLink, split } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { getMainDefinition } from '@apollo/client/utilities';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';

// HTTP link for queries and mutations
const httpLink = createHttpLink({
	uri: 'http://localhost:8080/v1/graphql'
});

// WebSocket link for subscriptions
const wsLink = new GraphQLWsLink(
	createClient({
		url: 'ws://localhost:8080/v1/graphql',
		connectionParams: {
			headers: {
				'x-hasura-admin-secret': 'myadminsecretkey'
			}
		}
	})
);

// Auth link (add admin secret or JWT token)
const authLink = setContext((_, { headers }) => {
	return {
		headers: {
			...headers,
			'x-hasura-admin-secret': 'myadminsecretkey'
		}
	};
});

// Split link: use WebSocket for subscriptions, HTTP for queries/mutations
const splitLink = split(
	({ query }) => {
		const definition = getMainDefinition(query);
		return definition.kind === 'OperationDefinition' && definition.operation === 'subscription';
	},
	wsLink,
	authLink.concat(httpLink)
);

export const apolloClient = new ApolloClient({
	link: splitLink,
	cache: new InMemoryCache()
});
```

### Create: Insert User

```typescript
// src/components/UserForm.tsx
import { useMutation } from '@apollo/client';
import { InsertUserDocument, InsertUserMutation, InsertUserMutationVariables } from '../generated/graphql';

export function CreateUserForm() {
  const [insertUser, { loading, error }] = useMutation<
    InsertUserMutation,
    InsertUserMutationVariables
  >(InsertUserDocument);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);

    try {
      const { data } = await insertUser({
        variables: {
          object: {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            age: parseInt(formData.get('age') as string),
            role: formData.get('role') as string,
          },
        },
      });

      console.log('User created:', data?.insert_users_one);
      // Reset form or navigate
    } catch (err) {
      console.error('Error creating user:', err);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" name="name" placeholder="Name" required />
      <input type="email" name="email" placeholder="Email" required />
      <input type="number" name="age" placeholder="Age" required />
      <select name="role">
        <option value="user">User</option>
        <option value="admin">Admin</option>
        <option value="guest">Guest</option>
      </select>
      <button type="submit" disabled={loading}>
        {loading ? 'Creating...' : 'Create User'}
      </button>
      {error && <div>Error: {error.message}</div>}
    </form>
  );
}
```

### Read: Query Users

```typescript
// src/components/UsersTable.tsx
import { useQuery } from '@apollo/client';
import {
  GetUsersDocument,
  GetUsersQuery,
  GetUsersQueryVariables,
} from '../generated/graphql';

export function UsersTable() {
  const { data, loading, error, refetch } = useQuery<
    GetUsersQuery,
    GetUsersQueryVariables
  >(GetUsersDocument, {
    variables: {
      limit: 10,
      offset: 0,
      orderBy: [{ created_at: 'desc' }],
    },
  });

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Age</th>
            <th>Role</th>
            <th>Articles</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {data?.users.map((user) => (
            <tr key={user.id}>
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
              <td>{user.age}</td>
              <td>{user.role}</td>
              <td>{user.articles.length}</td>
              <td>
                <button onClick={() => handleEdit(user.id)}>Edit</button>
                <button onClick={() => handleDelete(user.id)}>Delete</button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <button onClick={() => refetch()}>Refresh</button>
    </div>
  );
}
```

### Update: Update User

```typescript
// src/components/EditUserForm.tsx
import { useMutation, useQuery } from '@apollo/client';
import {
  GetUserDocument,
  GetUserQuery,
  GetUserQueryVariables,
  UpdateUserDocument,
  UpdateUserMutation,
  UpdateUserMutationVariables,
} from '../generated/graphql';

export function EditUserForm({ userId }: { userId: number }) {
  const { data, loading: queryLoading } = useQuery<
    GetUserQuery,
    GetUserQueryVariables
  >(GetUserDocument, {
    variables: { id: userId },
  });

  const [updateUser, { loading: mutationLoading, error }] = useMutation<
    UpdateUserMutation,
    UpdateUserMutationVariables
  >(UpdateUserDocument);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);

    try {
      const { data } = await updateUser({
        variables: {
          id: userId,
          changes: {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            age: parseInt(formData.get('age') as string),
            role: formData.get('role') as string,
          },
        },
      });

      console.log('User updated:', data?.update_users_by_pk);
    } catch (err) {
      console.error('Error updating user:', err);
    }
  };

  if (queryLoading) return <div>Loading...</div>;
  if (!data?.users_by_pk) return <div>User not found</div>;

  const user = data.users_by_pk;

  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        name="name"
        defaultValue={user.name}
        required
      />
      <input
        type="email"
        name="email"
        defaultValue={user.email}
        required
      />
      <input
        type="number"
        name="age"
        defaultValue={user.age || ''}
        required
      />
      <select name="role" defaultValue={user.role}>
        <option value="user">User</option>
        <option value="admin">Admin</option>
        <option value="guest">Guest</option>
      </select>
      <button type="submit" disabled={mutationLoading}>
        {mutationLoading ? 'Updating...' : 'Update User'}
      </button>
      {error && <div>Error: {error.message}</div>}
    </form>
  );
}
```

### Delete: Delete User

```typescript
// src/components/DeleteUserButton.tsx
import { useMutation } from '@apollo/client';
import {
  DeleteUserDocument,
  DeleteUserMutation,
  DeleteUserMutationVariables,
  GetUsersDocument,
} from '../generated/graphql';

export function DeleteUserButton({ userId }: { userId: number }) {
  const [deleteUser, { loading, error }] = useMutation<
    DeleteUserMutation,
    DeleteUserMutationVariables
  >(DeleteUserDocument, {
    refetchQueries: [{ query: GetUsersDocument }], // Refetch list after delete
  });

  const handleDelete = async () => {
    if (!confirm('Are you sure you want to delete this user?')) {
      return;
    }

    try {
      const { data } = await deleteUser({
        variables: { id: userId },
      });

      console.log('User deleted:', data?.delete_users_by_pk);
    } catch (err) {
      console.error('Error deleting user:', err);
    }
  };

  return (
    <button onClick={handleDelete} disabled={loading}>
      {loading ? 'Deleting...' : 'Delete'}
    </button>
  );
}
```

## Step 5: Subscriptions/Live Updates

### Setup WebSocket Connection

Hasura supports GraphQL subscriptions over WebSocket. The Apollo Client setup above already includes WebSocket support.

### Subscribe to Users (Live Query)

```typescript
// src/components/UsersTableLive.tsx
import { useSubscription } from '@apollo/client';
import {
  SubscribeUsersDocument,
  SubscribeUsersSubscription,
  SubscribeUsersSubscriptionVariables,
} from '../generated/graphql';

export function UsersTableLive() {
  const { data, loading, error } = useSubscription<
    SubscribeUsersSubscription,
    SubscribeUsersSubscriptionVariables
  >(SubscribeUsersDocument, {
    variables: {
      limit: 10,
      offset: 0,
      orderBy: [{ created_at: 'desc' }],
    },
  });

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <h2>Users (Live Updates)</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Age</th>
            <th>Role</th>
            <th>Articles</th>
          </tr>
        </thead>
        <tbody>
          {data?.users.map((user) => (
            <tr key={user.id}>
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
              <td>{user.age}</td>
              <td>{user.role}</td>
              <td>{user.articles.length}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
```

### Subscribe to Single User

```typescript
// src/components/UserDetailLive.tsx
import { useSubscription } from '@apollo/client';
import {
  SubscribeUserDocument,
  SubscribeUserSubscription,
  SubscribeUserSubscriptionVariables,
} from '../generated/graphql';

export function UserDetailLive({ userId }: { userId: number }) {
  const { data, loading, error } = useSubscription<
    SubscribeUserSubscription,
    SubscribeUserSubscriptionVariables
  >(SubscribeUserDocument, {
    variables: { id: userId },
  });

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;
  if (!data?.users_by_pk) return <div>User not found</div>;

  const user = data.users_by_pk;

  return (
    <div>
      <h2>{user.name}</h2>
      <p>Email: {user.email}</p>
      <p>Age: {user.age}</p>
      <p>Role: {user.role}</p>
      <h3>Articles ({user.articles.length})</h3>
      <ul>
        {user.articles.map((article) => (
          <li key={article.id}>
            {article.title} {article.published ? '(Published)' : '(Draft)'}
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### Complete Data Table with Live Updates

```typescript
// src/components/UsersDataTable.tsx
import { useState } from 'react';
import { useSubscription, useMutation } from '@apollo/client';
import {
  SubscribeUsersDocument,
  SubscribeUsersSubscription,
  SubscribeUsersSubscriptionVariables,
  InsertUserDocument,
  InsertUserMutation,
  InsertUserMutationVariables,
  UpdateUserDocument,
  UpdateUserMutation,
  UpdateUserMutationVariables,
  DeleteUserDocument,
  DeleteUserMutation,
  DeleteUserMutationVariables,
} from '../generated/graphql';

export function UsersDataTable() {
  const [editingId, setEditingId] = useState<number | null>(null);
  const [page, setPage] = useState(0);
  const pageSize = 10;

  // Live subscription to users
  const { data, loading, error } = useSubscription<
    SubscribeUsersSubscription,
    SubscribeUsersSubscriptionVariables
  >(SubscribeUsersDocument, {
    variables: {
      limit: pageSize,
      offset: page * pageSize,
      orderBy: [{ created_at: 'desc' }],
    },
  });

  // Mutations
  const [insertUser] = useMutation<
    InsertUserMutation,
    InsertUserMutationVariables
  >(InsertUserDocument);

  const [updateUser] = useMutation<
    UpdateUserMutation,
    UpdateUserMutationVariables
  >(UpdateUserDocument);

  const [deleteUser] = useMutation<
    DeleteUserMutation,
    DeleteUserMutationVariables
  >(DeleteUserDocument);

  const handleCreate = async (formData: FormData) => {
    try {
      await insertUser({
        variables: {
          object: {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            age: parseInt(formData.get('age') as string),
            role: formData.get('role') as string,
          },
        },
      });
    } catch (err) {
      console.error('Error creating user:', err);
    }
  };

  const handleUpdate = async (id: number, formData: FormData) => {
    try {
      await updateUser({
        variables: {
          id,
          changes: {
            name: formData.get('name') as string,
            email: formData.get('email') as string,
            age: parseInt(formData.get('age') as string),
            role: formData.get('role') as string,
          },
        },
      });
      setEditingId(null);
    } catch (err) {
      console.error('Error updating user:', err);
    }
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure?')) return;
    try {
      await deleteUser({ variables: { id } });
    } catch (err) {
      console.error('Error deleting user:', err);
    }
  };

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <h1>Users (Live Updates)</h1>

      {/* Create Form */}
      <form
        onSubmit={(e) => {
          e.preventDefault();
          handleCreate(new FormData(e.currentTarget));
          e.currentTarget.reset();
        }}
      >
        <input type="text" name="name" placeholder="Name" required />
        <input type="email" name="email" placeholder="Email" required />
        <input type="number" name="age" placeholder="Age" required />
        <select name="role">
          <option value="user">User</option>
          <option value="admin">Admin</option>
          <option value="guest">Guest</option>
        </select>
        <button type="submit">Create</button>
      </form>

      {/* Table */}
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Age</th>
            <th>Role</th>
            <th>Articles</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {data?.users.map((user) => (
            <tr key={user.id}>
              {editingId === user.id ? (
                <>
                  <td>{user.id}</td>
                  <td>
                    <input
                      type="text"
                      name="name"
                      defaultValue={user.name}
                      form={`edit-form-${user.id}`}
                    />
                  </td>
                  <td>
                    <input
                      type="email"
                      name="email"
                      defaultValue={user.email}
                      form={`edit-form-${user.id}`}
                    />
                  </td>
                  <td>
                    <input
                      type="number"
                      name="age"
                      defaultValue={user.age || ''}
                      form={`edit-form-${user.id}`}
                    />
                  </td>
                  <td>
                    <select
                      name="role"
                      defaultValue={user.role}
                      form={`edit-form-${user.id}`}
                    >
                      <option value="user">User</option>
                      <option value="admin">Admin</option>
                      <option value="guest">Guest</option>
                    </select>
                  </td>
                  <td>{user.articles.length}</td>
                  <td>
                    <form
                      id={`edit-form-${user.id}`}
                      onSubmit={(e) => {
                        e.preventDefault();
                        handleUpdate(user.id, new FormData(e.currentTarget));
                      }}
                    >
                      <button type="submit">Save</button>
                    </form>
                    <button onClick={() => setEditingId(null)}>Cancel</button>
                  </td>
                </>
              ) : (
                <>
                  <td>{user.id}</td>
                  <td>{user.name}</td>
                  <td>{user.email}</td>
                  <td>{user.age}</td>
                  <td>{user.role}</td>
                  <td>{user.articles.length}</td>
                  <td>
                    <button onClick={() => setEditingId(user.id)}>Edit</button>
                    <button onClick={() => handleDelete(user.id)}>Delete</button>
                  </td>
                </>
              )}
            </tr>
          ))}
        </tbody>
      </table>

      {/* Pagination */}
      <div>
        <button
          onClick={() => setPage((p) => Math.max(0, p - 1))}
          disabled={page === 0}
        >
          Previous
        </button>
        <span>Page {page + 1}</span>
        <button
          onClick={() => setPage((p) => p + 1)}
          disabled={(data?.users.length || 0) < pageSize}
        >
          Next
        </button>
      </div>
    </div>
  );
}
```

## Call Stacks

### Call Stack: Create (Insert)

```
1. User fills form and clicks "Create"
   ↓
2. React: Form submission handler
   ↓
3. Apollo Client: insertUser mutation
   ↓
4. HTTP Request: POST /v1/graphql
   {
     "query": "mutation InsertUser($object: users_insert_input!) { ... }",
     "variables": { "object": { ... } }
   }
   ↓
5. Hasura: Parse GraphQL mutation
   ↓
6. Hasura: Validate against GraphQL schema
   ↓
7. Hasura: Generate SQL INSERT statement
   ↓
8. PostgreSQL: Execute INSERT
   ↓
9. PostgreSQL: Return inserted row
   ↓
10. Hasura: Transform to GraphQL response
   ↓
11. HTTP Response: { "data": { "insert_users_one": { ... } } }
   ↓
12. Apollo Client: Update cache
   ↓
13. Subscription: Broadcast update to all subscribers
   ↓
14. React: Re-render with new data (if using subscription)
```

### Call Stack: Read (Query)

```
1. Component mounts or refetch called
   ↓
2. Apollo Client: useQuery hook
   ↓
3. Check Apollo cache
   - If cached and fresh → Return cached data
   - If not cached or stale → Continue
   ↓
4. HTTP Request: POST /v1/graphql
   {
     "query": "query GetUsers($limit: Int, ...) { ... }",
     "variables": { "limit": 10, ... }
   }
   ↓
5. Hasura: Parse GraphQL query
   ↓
6. Hasura: Validate against GraphQL schema
   ↓
7. Hasura: Generate SQL SELECT statement
   ↓
8. PostgreSQL: Execute SELECT
   ↓
9. PostgreSQL: Return rows
   ↓
10. Hasura: Transform to GraphQL response
   ↓
11. HTTP Response: { "data": { "users": [...] } }
   ↓
12. Apollo Client: Store in cache
   ↓
13. React: Re-render with data
```

### Call Stack: Update

```
1. User clicks "Edit" → User modifies fields → User clicks "Save"
   ↓
2. React: Form submission handler
   ↓
3. Apollo Client: updateUser mutation
   ↓
4. HTTP Request: POST /v1/graphql
   {
     "query": "mutation UpdateUser($id: Int!, $changes: users_set_input!) { ... }",
     "variables": { "id": 1, "changes": { ... } }
   }
   ↓
5. Hasura: Parse GraphQL mutation
   ↓
6. Hasura: Validate against GraphQL schema
   ↓
7. Hasura: Generate SQL UPDATE statement
   ↓
8. PostgreSQL: Execute UPDATE
   ↓
9. PostgreSQL: Return updated row
   ↓
10. Hasura: Transform to GraphQL response
   ↓
11. HTTP Response: { "data": { "update_users_by_pk": { ... } } }
   ↓
12. Apollo Client: Update cache
   ↓
13. Subscription: Broadcast update to all subscribers
   ↓
14. React: Re-render with updated data (if using subscription)
```

### Call Stack: Delete

```
1. User clicks "Delete" → Confirms deletion
   ↓
2. React: Delete handler
   ↓
3. Apollo Client: deleteUser mutation
   ↓
4. HTTP Request: POST /v1/graphql
   {
     "query": "mutation DeleteUser($id: Int!) { ... }",
     "variables": { "id": 1 }
   }
   ↓
5. Hasura: Parse GraphQL mutation
   ↓
6. Hasura: Validate against GraphQL schema
   ↓
7. Hasura: Generate SQL DELETE statement
   ↓
8. PostgreSQL: Execute DELETE
   ↓
9. PostgreSQL: Return affected rows
   ↓
10. Hasura: Transform to GraphQL response
   ↓
11. HTTP Response: { "data": { "delete_users_by_pk": { ... } } }
   ↓
12. Apollo Client: Remove from cache
   ↓
13. Subscription: Broadcast update to all subscribers
   ↓
14. React: Re-render without deleted item (if using subscription)
```

### Call Stack: Subscription (Live Query)

```
1. Component mounts
   ↓
2. Apollo Client: useSubscription hook
   ↓
3. WebSocket: Connect to ws://localhost:8080/v1/graphql
   ↓
4. WebSocket: Send subscription message
   {
     "type": "start",
     "payload": {
       "query": "subscription SubscribeUsers { ... }",
       "variables": { ... }
     }
   }
   ↓
5. Hasura: Parse GraphQL subscription
   ↓
6. Hasura: Validate against GraphQL schema
   ↓
7. Hasura: Register subscription
   ↓
8. Hasura: Execute initial query
   ↓
9. PostgreSQL: Execute SELECT
   ↓
10. Hasura: Send initial data via WebSocket
   ↓
11. Apollo Client: Store in cache and trigger re-render
   ↓
12. [User performs mutation elsewhere]
   ↓
13. Hasura: Detect database change (via triggers/listeners)
   ↓
14. Hasura: Re-execute subscription query
   ↓
15. PostgreSQL: Execute SELECT
   ↓
16. Hasura: Compare with previous result
   ↓
17. Hasura: If changed, send update via WebSocket
   ↓
18. Apollo Client: Update cache
   ↓
19. React: Re-render with new data
```

## Best Practices

### 1. Use GraphQL Fragments

```graphql
# src/graphql/fragments/user.fragment.graphql
fragment UserFields on users {
	id
	name
	email
	age
	role
	created_at
	updated_at
}

# Use in queries
query GetUsers {
	users {
		...UserFields
		articles {
			id
			title
		}
	}
}
```

### 2. Optimize Queries with Field Selection

```graphql
# Only fetch needed fields
query GetUserNames {
	users {
		id
		name
		# Don't fetch email, age, etc. if not needed
	}
}
```

### 3. Use Pagination

```graphql
query GetUsersPaginated($limit: Int!, $offset: Int!) {
	users(limit: $limit, offset: $offset) {
		id
		name
		email
	}
}
```

### 4. Handle Errors Gracefully

```typescript
const { data, loading, error } = useQuery(GetUsersDocument);

if (error) {
  // Handle network errors, GraphQL errors, etc.
  return <ErrorDisplay error={error} />;
}
```

### 5. Optimistic Updates

```typescript
const [updateUser] = useMutation(UpdateUserDocument, {
	optimisticResponse: {
		update_users_by_pk: {
			id: userId,
			name: newName
			// ... other fields
		}
	}
});
```

## Summary

**Hasura + PostgreSQL + GraphQL + TypeScript provides:**

1. **Auto-Generated GraphQL Schema**: Based on PostgreSQL tables
2. **Type-Safe Frontend**: TypeScript types generated from GraphQL schema
3. **CRUD Operations**: Queries and mutations for all operations
4. **Real-Time Updates**: Subscriptions for live data
5. **Relationship Queries**: Automatic handling of foreign keys
6. **Type Safety**: End-to-end type safety from database to UI

**Key Workflow:**

1. Define PostgreSQL schema
2. Track tables in Hasura
3. Hasura auto-generates GraphQL schema
4. Define GraphQL operations (.graphql files)
5. Generate TypeScript types (GraphQL Code Generator)
6. Use typed hooks in React components
7. Subscriptions provide real-time updates

This architecture ensures type safety, real-time updates, and efficient data operations with minimal boilerplate code.
