# Type-Safe Editable SQL Data Table with Remix, Zod, and TypeScript

This document explains how to build a type-safe, editable SQL data table using Remix, Zod, and TypeScript, including action loaders, schemas, validation, UI wiring, and complete call stacks for CRUD operations.

## Overview

The combination of Remix, Zod, and TypeScript enables:
- **Type-safe forms**: Shared types between client and server
- **Automatic validation**: Zod schemas validate on both sides
- **SQL integration**: Type-safe database operations
- **Progressive enhancement**: Forms work without JavaScript
- **Server-side type reuse**: Single source of truth for types

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Client (React)                           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Form UI    │  │  Table UI    │  │  Validation  │     │
│  │              │  │              │  │  (Zod)      │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            │                                 │
│                            ▼                                 │
│                    Remix Form/Route                           │
└────────────────────────────┼─────────────────────────────────┘
                             │
                             │ HTTP Request
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    Server (Remix)                           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Loader     │  │   Action     │  │  Validation  │     │
│  │              │  │              │  │  (Zod)      │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            │                                 │
│                            ▼                                 │
│                    SQL Database                              │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Define Zod Schemas

### Base Schema

```typescript
// app/schemas/user.schema.ts
import { z } from 'zod';

// Base user schema
export const UserSchema = z.object({
	id: z.number().int().positive().optional(),
	name: z.string().min(1, 'Name is required').max(100),
	email: z.string().email('Invalid email address'),
	age: z.number().int().positive().max(150),
	role: z.enum(['admin', 'user', 'guest']).default('user'),
	createdAt: z.date().optional(),
	updatedAt: z.date().optional()
});

// Infer TypeScript type from schema
export type User = z.infer<typeof UserSchema>;

// Schema for creating a new user (id and timestamps optional)
export const CreateUserSchema = UserSchema.omit({
	id: true,
	createdAt: true,
	updatedAt: true
});

export type CreateUser = z.infer<typeof CreateUserSchema>;

// Schema for updating a user (all fields optional except id)
export const UpdateUserSchema = UserSchema.partial().required({
	id: true
});

export type UpdateUser = z.infer<typeof UpdateUserSchema>;

// Schema for form data (strings from FormData)
export const UserFormSchema = z.object({
	id: z.string().optional(),
	name: z.string().min(1, 'Name is required').max(100),
	email: z.string().email('Invalid email address'),
	age: z.string().regex(/^\d+$/, 'Age must be a number').transform(Number),
	role: z.enum(['admin', 'user', 'guest']).default('user')
});

// Transform form data to user data
export const parseUserForm = (formData: FormData): CreateUser => {
	const raw = Object.fromEntries(formData);
	const parsed = UserFormSchema.parse(raw);
	
	return {
		name: parsed.name,
		email: parsed.email,
		age: parsed.age,
		role: parsed.role
	};
};
```

### Advanced Schema with Relations

```typescript
// app/schemas/product.schema.ts
import { z } from 'zod';

export const ProductSchema = z.object({
	id: z.number().int().positive().optional(),
	name: z.string().min(1).max(200),
	description: z.string().max(1000).optional(),
	price: z.number().positive(),
	stock: z.number().int().nonnegative(),
	categoryId: z.number().int().positive(),
	active: z.boolean().default(true),
	createdAt: z.date().optional(),
	updatedAt: z.date().optional()
});

export type Product = z.infer<typeof ProductSchema>;

// Form schema (handles string inputs)
export const ProductFormSchema = z.object({
	id: z.string().optional(),
	name: z.string().min(1, 'Name is required').max(200),
	description: z.string().max(1000).optional(),
	price: z.string().regex(/^\d+(\.\d{1,2})?$/, 'Invalid price').transform(Number),
	stock: z.string().regex(/^\d+$/, 'Stock must be a number').transform(Number),
	categoryId: z.string().regex(/^\d+$/, 'Category ID must be a number').transform(Number),
	active: z.string().transform(val => val === 'true' || val === 'on')
});

export const parseProductForm = (formData: FormData): Omit<Product, 'id' | 'createdAt' | 'updatedAt'> => {
	const raw = Object.fromEntries(formData);
	return ProductFormSchema.parse(raw);
};
```

## Step 2: Database Layer with Type Safety

### Type-Safe Database Functions

```typescript
// app/lib/db/users.server.ts
import { db } from '~/lib/db.server';
import type { User, CreateUser, UpdateUser } from '~/schemas/user.schema';

// Get all users
export async function getUsers(): Promise<User[]> {
	const rows = await db.query('SELECT * FROM users ORDER BY created_at DESC');
	return rows.map(row => ({
		id: row.id,
		name: row.name,
		email: row.email,
		age: row.age,
		role: row.role,
		createdAt: row.created_at,
		updatedAt: row.updated_at
	}));
}

// Get user by ID
export async function getUserById(id: number): Promise<User | null> {
	const row = await db.queryOne('SELECT * FROM users WHERE id = ?', [id]);
	if (!row) return null;
	
	return {
		id: row.id,
		name: row.name,
		email: row.email,
		age: row.age,
		role: row.role,
		createdAt: row.created_at,
		updatedAt: row.updated_at
	};
}

// Create user
export async function createUser(data: CreateUser): Promise<User> {
	const result = await db.query(
		`INSERT INTO users (name, email, age, role, created_at, updated_at)
		 VALUES (?, ?, ?, ?, NOW(), NOW())
		 RETURNING *`,
		[data.name, data.email, data.age, data.role]
	);
	
	const row = result[0];
	return {
		id: row.id,
		name: row.name,
		email: row.email,
		age: row.age,
		role: row.role,
		createdAt: row.created_at,
		updatedAt: row.updated_at
	};
}

// Update user
export async function updateUser(data: UpdateUser): Promise<User> {
	const updates: string[] = [];
	const values: any[] = [];
	
	if (data.name !== undefined) {
		updates.push('name = ?');
		values.push(data.name);
	}
	if (data.email !== undefined) {
		updates.push('email = ?');
		values.push(data.email);
	}
	if (data.age !== undefined) {
		updates.push('age = ?');
		values.push(data.age);
	}
	if (data.role !== undefined) {
		updates.push('role = ?');
		values.push(data.role);
	}
	
	updates.push('updated_at = NOW()');
	values.push(data.id);
	
	await db.query(
		`UPDATE users SET ${updates.join(', ')} WHERE id = ?`,
		values
	);
	
	const updated = await getUserById(data.id);
	if (!updated) throw new Error('User not found after update');
	
	return updated;
}

// Delete user
export async function deleteUser(id: number): Promise<void> {
	await db.query('DELETE FROM users WHERE id = ?', [id]);
}
```

## Step 3: Remix Route with Loader and Actions

### Complete Route File

```typescript
// app/routes/users.tsx
import type { ActionFunctionArgs, LoaderFunctionArgs } from '@remix-run/node';
import { json, redirect } from '@remix-run/node';
import { Form, useLoaderData, useActionData, useNavigation } from '@remix-run/react';
import { UserSchema, CreateUserSchema, UpdateUserSchema, parseUserForm } from '~/schemas/user.schema';
import { getUsers, createUser, updateUser, deleteUser } from '~/lib/db/users.server';
import type { User } from '~/schemas/user.schema';

// Loader: Fetch users for display
export async function loader({ request }: LoaderFunctionArgs) {
	const users = await getUsers();
	return json({ users });
}

// Action: Handle form submissions (create, update, delete)
export async function action({ request }: ActionFunctionArgs) {
	const formData = await request.formData();
	const intent = formData.get('intent') as string;

	try {
		switch (intent) {
			case 'create': {
				// Parse and validate form data
				const raw = Object.fromEntries(formData);
				const result = CreateUserSchema.safeParse(raw);

				if (!result.success) {
					return json(
						{
							errors: result.error.flatten().fieldErrors,
							values: raw
						},
						{ status: 400 }
					);
				}

				// Create user
				const user = await createUser(result.data);
				return redirect(`/users?created=${user.id}`);
			}

			case 'update': {
				const raw = Object.fromEntries(formData);
				const id = Number(formData.get('id'));

				// Validate ID
				if (isNaN(id)) {
					return json(
						{ errors: { id: ['Invalid user ID'] }, values: raw },
						{ status: 400 }
					);
				}

				// Parse and validate update data
				const updateData = {
					id,
					name: raw.name as string,
					email: raw.email as string,
					age: Number(raw.age),
					role: raw.role as 'admin' | 'user' | 'guest'
				};

				const result = UpdateUserSchema.safeParse(updateData);

				if (!result.success) {
					return json(
						{
							errors: result.error.flatten().fieldErrors,
							values: raw
						},
						{ status: 400 }
					);
				}

				// Update user
				await updateUser(result.data);
				return redirect(`/users?updated=${id}`);
			}

			case 'delete': {
				const id = Number(formData.get('id'));

				if (isNaN(id)) {
					return json({ error: 'Invalid user ID' }, { status: 400 });
				}

				// Delete user
				await deleteUser(id);
				return redirect(`/users?deleted=${id}`);
			}

			default:
				return json({ error: 'Invalid intent' }, { status: 400 });
		}
	} catch (error) {
		console.error('Action error:', error);
		return json(
			{ error: error instanceof Error ? error.message : 'An error occurred' },
			{ status: 500 }
		);
	}
}

// Component: Users table with inline editing
export default function UsersPage() {
	const { users } = useLoaderData<typeof loader>();
	const actionData = useActionData<typeof action>();
	const navigation = useNavigation();

	const isSubmitting = navigation.state === 'submitting';

	return (
		<div>
			<h1>Users</h1>

			{/* Create User Form */}
			<CreateUserForm actionData={actionData} isSubmitting={isSubmitting} />

			{/* Users Table */}
			<table>
				<thead>
					<tr>
						<th>ID</th>
						<th>Name</th>
						<th>Email</th>
						<th>Age</th>
						<th>Role</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{users.map((user) => (
						<UserRow
							key={user.id}
							user={user}
							actionData={actionData}
							isSubmitting={isSubmitting}
						/>
					))}
				</tbody>
			</table>
		</div>
	);
}

// Create User Form Component
function CreateUserForm({
	actionData,
	isSubmitting
}: {
	actionData: any;
	isSubmitting: boolean;
}) {
	return (
		<Form method="post" className="create-form">
			<input type="hidden" name="intent" value="create" />
			
			<div>
				<label>
					Name:
					<input
						type="text"
						name="name"
						defaultValue={actionData?.values?.name}
						disabled={isSubmitting}
					/>
				</label>
				{actionData?.errors?.name && (
					<span className="error">{actionData.errors.name[0]}</span>
				)}
			</div>

			<div>
				<label>
					Email:
					<input
						type="email"
						name="email"
						defaultValue={actionData?.values?.email}
						disabled={isSubmitting}
					/>
				</label>
				{actionData?.errors?.email && (
					<span className="error">{actionData.errors.email[0]}</span>
				)}
			</div>

			<div>
				<label>
					Age:
					<input
						type="number"
						name="age"
						defaultValue={actionData?.values?.age}
						disabled={isSubmitting}
					/>
				</label>
				{actionData?.errors?.age && (
					<span className="error">{actionData.errors.age[0]}</span>
				)}
			</div>

			<div>
				<label>
					Role:
					<select name="role" defaultValue={actionData?.values?.role || 'user'} disabled={isSubmitting}>
						<option value="user">User</option>
						<option value="admin">Admin</option>
						<option value="guest">Guest</option>
					</select>
				</label>
				{actionData?.errors?.role && (
					<span className="error">{actionData.errors.role[0]}</span>
				)}
			</div>

			<button type="submit" disabled={isSubmitting}>
				{isSubmitting ? 'Creating...' : 'Create User'}
			</button>
		</Form>
	);
}

// User Row Component (Inline Editing)
function UserRow({
	user,
	actionData,
	isSubmitting
}: {
	user: User;
	actionData: any;
	isSubmitting: boolean;
}) {
	const isEditing = actionData?.editingId === user.id;

	return (
		<tr>
			<td>{user.id}</td>
			{isEditing ? (
				<>
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
							defaultValue={user.age}
							form={`edit-form-${user.id}`}
						/>
					</td>
					<td>
						<select name="role" defaultValue={user.role} form={`edit-form-${user.id}`}>
							<option value="user">User</option>
							<option value="admin">Admin</option>
							<option value="guest">Guest</option>
						</select>
					</td>
					<td>
						<Form method="post" id={`edit-form-${user.id}`}>
							<input type="hidden" name="intent" value="update" />
							<input type="hidden" name="id" value={user.id} />
							<button type="submit" disabled={isSubmitting}>
								Save
							</button>
						</Form>
						<Form method="post">
							<input type="hidden" name="intent" value="cancel" />
							<button type="submit">Cancel</button>
						</Form>
					</td>
				</>
			) : (
				<>
					<td>{user.name}</td>
					<td>{user.email}</td>
					<td>{user.age}</td>
					<td>{user.role}</td>
					<td>
						<Form method="post">
							<input type="hidden" name="intent" value="edit" />
							<input type="hidden" name="id" value={user.id} />
							<button type="submit">Edit</button>
						</Form>
						<Form method="post">
							<input type="hidden" name="intent" value="delete" />
							<input type="hidden" name="id" value={user.id} />
							<button type="submit" disabled={isSubmitting}>
								Delete
							</button>
						</Form>
					</td>
				</>
			)}
		</tr>
	);
}
```

## Step 4: Complete Call Stacks

### Call Stack: Insert (Create)

```
1. User fills form and clicks "Create User"
   ↓
2. Browser: Form submission (POST /users)
   ↓
3. Remix: Route action() function called
   ↓
4. action(): Extract formData from request
   ↓
5. action(): Parse formData to object
   ↓
6. action(): CreateUserSchema.safeParse(raw)
   ↓
7. Zod: Validate schema
   - If invalid → Return json({ errors, values }, 400)
   - If valid → Continue
   ↓
8. action(): createUser(result.data)
   ↓
9. db/users.server.ts: createUser()
   - Execute SQL INSERT
   - Return created user
   ↓
10. action(): redirect(`/users?created=${user.id}`)
   ↓
11. Remix: Navigate to /users
   ↓
12. loader(): getUsers() - Fetch updated list
   ↓
13. Component: Re-render with new user
```

### Call Stack: Edit (Update)

```
1. User clicks "Edit" button
   ↓
2. Browser: Form submission (POST /users, intent=edit)
   ↓
3. Remix: Route action() function called
   ↓
4. action(): Extract intent='edit' from formData
   ↓
5. action(): Set editingId in actionData
   ↓
6. action(): Return json({ editingId: user.id })
   ↓
7. Component: UserRow detects isEditing=true
   ↓
8. Component: Render edit form with current values
   ↓
9. User modifies fields and clicks "Save"
   ↓
10. Browser: Form submission (POST /users, intent=update)
   ↓
11. Remix: Route action() function called
   ↓
12. action(): Extract formData, intent='update'
   ↓
13. action(): Parse id and update data
   ↓
14. action(): UpdateUserSchema.safeParse(updateData)
   ↓
15. Zod: Validate schema
   - If invalid → Return json({ errors, values }, 400)
   - If valid → Continue
   ↓
16. action(): updateUser(result.data)
   ↓
17. db/users.server.ts: updateUser()
   - Execute SQL UPDATE
   - Return updated user
   ↓
18. action(): redirect(`/users?updated=${id}`)
   ↓
19. Remix: Navigate to /users
   ↓
20. loader(): getUsers() - Fetch updated list
   ↓
21. Component: Re-render with updated user
```

### Call Stack: Delete

```
1. User clicks "Delete" button
   ↓
2. Browser: Form submission (POST /users, intent=delete)
   ↓
3. Remix: Route action() function called
   ↓
4. action(): Extract intent='delete' from formData
   ↓
5. action(): Extract and validate id
   ↓
6. action(): deleteUser(id)
   ↓
7. db/users.server.ts: deleteUser()
   - Execute SQL DELETE
   ↓
8. action(): redirect(`/users?deleted=${id}`)
   ↓
9. Remix: Navigate to /users
   ↓
10. loader(): getUsers() - Fetch updated list
   ↓
11. Component: Re-render without deleted user
```

## Step 5: Server-Side Type Reuse

### Shared Types Module

```typescript
// app/types/user.types.ts
import type { z } from 'zod';
import { UserSchema, CreateUserSchema, UpdateUserSchema } from '~/schemas/user.schema';

// Export inferred types
export type User = z.infer<typeof UserSchema>;
export type CreateUser = z.infer<typeof CreateUserSchema>;
export type UpdateUser = z.infer<typeof UpdateUserSchema>;

// Export schemas for runtime validation
export { UserSchema, CreateUserSchema, UpdateUserSchema };
```

### Type-Safe API Functions

```typescript
// app/lib/api/users.server.ts
import type { User, CreateUser, UpdateUser } from '~/types/user.types';
import { UserSchema, CreateUserSchema, UpdateUserSchema } from '~/types/user.types';
import * as dbUsers from '~/lib/db/users.server';

// Type-safe API wrapper
export class UserAPI {
	// Get all users (returns typed array)
	static async getAll(): Promise<User[]> {
		return dbUsers.getUsers();
	}

	// Get user by ID (returns typed user or null)
	static async getById(id: number): Promise<User | null> {
		return dbUsers.getUserById(id);
	}

	// Create user (validates input, returns typed user)
	static async create(data: unknown): Promise<User> {
		const validated = CreateUserSchema.parse(data);
		return dbUsers.createUser(validated);
	}

	// Update user (validates input, returns typed user)
	static async update(data: unknown): Promise<User> {
		const validated = UpdateUserSchema.parse(data);
		return dbUsers.updateUser(validated);
	}

	// Delete user (validates ID)
	static async delete(id: unknown): Promise<void> {
		const validated = UserSchema.shape.id.parse(id);
		return dbUsers.deleteUser(validated);
	}
}
```

### Type-Safe Loader and Action

```typescript
// app/routes/users.tsx
import type { ActionFunctionArgs, LoaderFunctionArgs } from '@remix-run/node';
import { json, redirect } from '@remix-run/node';
import { UserAPI } from '~/lib/api/users.server';
import type { User } from '~/types/user.types';

// Loader returns typed data
export async function loader({ request }: LoaderFunctionArgs) {
	const users = await UserAPI.getAll(); // Type: User[]
	return json({ users });
}

// Action uses typed validation
export async function action({ request }: ActionFunctionArgs) {
	const formData = await request.formData();
	const intent = formData.get('intent') as string;

	switch (intent) {
		case 'create': {
			const raw = Object.fromEntries(formData);
			const result = await UserAPI.create(raw); // Type-safe
			return redirect(`/users?created=${result.id}`);
		}

		case 'update': {
			const raw = Object.fromEntries(formData);
			const result = await UserAPI.update(raw); // Type-safe
			return redirect(`/users?updated=${result.id}`);
		}

		case 'delete': {
			const id = formData.get('id');
			await UserAPI.delete(id); // Type-safe
			return redirect(`/users?deleted=${id}`);
		}

		default:
			return json({ error: 'Invalid intent' }, { status: 400 });
	}
}
```

## Step 6: Advanced Patterns

### Optimistic UI Updates

```typescript
// app/routes/users.tsx
import { useFetcher } from '@remix-run/react';

function UserRow({ user }: { user: User }) {
	const fetcher = useFetcher();
	const isDeleting = fetcher.formData?.get('intent') === 'delete';

	if (isDeleting) {
		return null; // Hide row immediately
	}

	return (
		<tr>
			{/* ... row content ... */}
			<td>
				<fetcher.Form method="post">
					<input type="hidden" name="intent" value="delete" />
					<input type="hidden" name="id" value={user.id} />
					<button type="submit">Delete</button>
				</fetcher.Form>
			</td>
		</tr>
	);
}
```

### Client-Side Validation (Progressive Enhancement)

```typescript
// app/components/UserForm.tsx
import { useActionData, useNavigation } from '@remix-run/react';
import { UserFormSchema } from '~/schemas/user.schema';
import { useState } from 'react';

export function UserForm() {
	const actionData = useActionData<typeof action>();
	const navigation = useNavigation();
	const [errors, setErrors] = useState<Record<string, string>>({});

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		const formData = new FormData(e.currentTarget);
		const raw = Object.fromEntries(formData);

		// Client-side validation (progressive enhancement)
		const result = UserFormSchema.safeParse(raw);
		if (!result.success) {
			e.preventDefault();
			const fieldErrors: Record<string, string> = {};
			result.error.errors.forEach((err) => {
				if (err.path[0]) {
					fieldErrors[err.path[0] as string] = err.message;
				}
			});
			setErrors(fieldErrors);
			return;
		}

		// Clear errors if valid
		setErrors({});
	};

	return (
		<Form method="post" onSubmit={handleSubmit}>
			{/* Form fields with client-side error display */}
			<div>
				<label>
					Name:
					<input type="text" name="name" />
				</label>
				{errors.name && <span className="error">{errors.name}</span>}
				{actionData?.errors?.name && (
					<span className="error">{actionData.errors.name[0]}</span>
				)}
			</div>
			{/* ... other fields ... */}
		</Form>
	);
}
```

### Batch Operations

```typescript
// app/routes/users.tsx
export async function action({ request }: ActionFunctionArgs) {
	const formData = await request.formData();
	const intent = formData.get('intent') as string;

	if (intent === 'batch-delete') {
		const ids = formData.getAll('ids').map(Number);
		
		// Validate all IDs
		const validIds = ids.filter(id => !isNaN(id) && id > 0);
		
		// Delete in transaction
		await db.transaction(async (tx) => {
			for (const id of validIds) {
				await deleteUser(id);
			}
		});
		
		return redirect('/users?batch-deleted');
	}

	// ... other intents
}
```

## Best Practices

### 1. Separate Form Schemas from Domain Schemas

```typescript
// Domain schema (database)
export const UserSchema = z.object({
	id: z.number(),
	name: z.string(),
	email: z.string().email(),
	age: z.number()
});

// Form schema (handles string inputs)
export const UserFormSchema = z.object({
	id: z.string().optional(),
	name: z.string(),
	email: z.string().email(),
	age: z.string().transform(Number)
});
```

### 2. Use safeParse for Error Handling

```typescript
// ✅ GOOD: Use safeParse
const result = Schema.safeParse(data);
if (!result.success) {
	return json({ errors: result.error.flatten() }, { status: 400 });
}

// ❌ BAD: parse throws exceptions
try {
	const data = Schema.parse(raw);
} catch (error) {
	// Harder to handle
}
```

### 3. Reuse Types Across Layers

```typescript
// Single source of truth
export const UserSchema = z.object({ /* ... */ });
export type User = z.infer<typeof UserSchema>;

// Use in database layer
function createUser(data: User): Promise<User> { /* ... */ }

// Use in API layer
function getUser(id: number): Promise<User | null> { /* ... */ }

// Use in UI layer
function UserCard({ user }: { user: User }) { /* ... */ }
```

### 4. Validate at Boundaries

```typescript
// Validate at API boundary
export async function action({ request }: ActionFunctionArgs) {
	const formData = await request.formData();
	const raw = Object.fromEntries(formData);
	
	// Validate immediately
	const result = Schema.safeParse(raw);
	if (!result.success) {
		return json({ errors: result.error.flatten() }, { status: 400 });
	}
	
	// Use validated data
	const user = await createUser(result.data);
	return json({ user });
}
```

### 5. Type-Safe Error Handling

```typescript
// Define error response type
type ActionData = 
	| { success: true; user: User }
	| { success: false; errors: Record<string, string[]>; values?: Record<string, any> };

export async function action({ request }: ActionFunctionArgs): Promise<Response> {
	const result = Schema.safeParse(/* ... */);
	
	if (!result.success) {
		return json<ActionData>({
			success: false,
			errors: result.error.flatten().fieldErrors,
			values: raw
		}, { status: 400 });
	}
	
	return json<ActionData>({
		success: true,
		user: await createUser(result.data)
	});
}
```

## Complete Example: Editable Data Table

```typescript
// app/routes/users.tsx
import type { ActionFunctionArgs, LoaderFunctionArgs } from '@remix-run/node';
import { json, redirect } from '@remix-run/node';
import { Form, useLoaderData, useActionData, useNavigation } from '@remix-run/react';
import { CreateUserSchema, UpdateUserSchema } from '~/schemas/user.schema';
import { UserAPI } from '~/lib/api/users.server';
import type { User } from '~/types/user.types';

export async function loader({ request }: LoaderFunctionArgs) {
	const users = await UserAPI.getAll();
	return json({ users });
}

export async function action({ request }: ActionFunctionArgs) {
	const formData = await request.formData();
	const intent = formData.get('intent') as string;
	const raw = Object.fromEntries(formData);

	try {
		switch (intent) {
			case 'create': {
				const result = CreateUserSchema.safeParse(raw);
				if (!result.success) {
					return json(
						{ errors: result.error.flatten().fieldErrors, values: raw },
						{ status: 400 }
					);
				}
				const user = await UserAPI.create(result.data);
				return redirect(`/users?created=${user.id}`);
			}

			case 'update': {
				const updateData = {
					id: Number(raw.id),
					name: raw.name as string,
					email: raw.email as string,
					age: Number(raw.age),
					role: raw.role as 'admin' | 'user' | 'guest'
				};
				const result = UpdateUserSchema.safeParse(updateData);
				if (!result.success) {
					return json(
						{ errors: result.error.flatten().fieldErrors, values: raw },
						{ status: 400 }
					);
				}
				await UserAPI.update(result.data);
				return redirect(`/users?updated=${updateData.id}`);
			}

			case 'delete': {
				const id = Number(formData.get('id'));
				if (isNaN(id)) {
					return json({ error: 'Invalid ID' }, { status: 400 });
				}
				await UserAPI.delete(id);
				return redirect(`/users?deleted=${id}`);
			}

			default:
				return json({ error: 'Invalid intent' }, { status: 400 });
		}
	} catch (error) {
		return json(
			{ error: error instanceof Error ? error.message : 'An error occurred' },
			{ status: 500 }
		);
	}
}

export default function UsersPage() {
	const { users } = useLoaderData<typeof loader>();
	const actionData = useActionData<typeof action>();
	const navigation = useNavigation();
	const isSubmitting = navigation.state === 'submitting';

	return (
		<div>
			<h1>Users</h1>
			
			<CreateUserForm actionData={actionData} isSubmitting={isSubmitting} />
			
			<table>
				<thead>
					<tr>
						<th>ID</th>
						<th>Name</th>
						<th>Email</th>
						<th>Age</th>
						<th>Role</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{users.map((user) => (
						<UserRow
							key={user.id}
							user={user}
							actionData={actionData}
							isSubmitting={isSubmitting}
						/>
					))}
				</tbody>
			</table>
		</div>
	);
}

function CreateUserForm({ actionData, isSubmitting }: any) {
	return (
		<Form method="post" className="mb-4">
			<input type="hidden" name="intent" value="create" />
			<div>
				<label>Name: <input type="text" name="name" defaultValue={actionData?.values?.name} /></label>
				{actionData?.errors?.name && <span>{actionData.errors.name[0]}</span>}
			</div>
			<div>
				<label>Email: <input type="email" name="email" defaultValue={actionData?.values?.email} /></label>
				{actionData?.errors?.email && <span>{actionData.errors.email[0]}</span>}
			</div>
			<div>
				<label>Age: <input type="number" name="age" defaultValue={actionData?.values?.age} /></label>
				{actionData?.errors?.age && <span>{actionData.errors.age[0]}</span>}
			</div>
			<button type="submit" disabled={isSubmitting}>Create</button>
		</Form>
	);
}

function UserRow({ user, actionData, isSubmitting }: any) {
	return (
		<tr>
			<td>{user.id}</td>
			<td>{user.name}</td>
			<td>{user.email}</td>
			<td>{user.age}</td>
			<td>{user.role}</td>
			<td>
				<Form method="post">
					<input type="hidden" name="intent" value="update" />
					<input type="hidden" name="id" value={user.id} />
					<input type="text" name="name" defaultValue={user.name} />
					<input type="email" name="email" defaultValue={user.email} />
					<input type="number" name="age" defaultValue={user.age} />
					<button type="submit" disabled={isSubmitting}>Update</button>
				</Form>
				<Form method="post">
					<input type="hidden" name="intent" value="delete" />
					<input type="hidden" name="id" value={user.id} />
					<button type="submit" disabled={isSubmitting}>Delete</button>
				</Form>
			</td>
		</tr>
	);
}
```

## Summary

**Remix + Zod + TypeScript provides:**

1. **Type-Safe Schemas**: Zod schemas define data structure and validation rules
2. **Type Inference**: `z.infer<typeof Schema>` generates TypeScript types
3. **Server-Side Validation**: Actions validate with Zod before database operations
4. **Client-Side Types**: Components use inferred types for type safety
5. **Progressive Enhancement**: Forms work without JavaScript
6. **Error Handling**: Zod's `safeParse` provides structured error responses
7. **Type Reuse**: Single schema generates types for database, API, and UI layers

**Key Patterns:**
- Define Zod schemas for domain models
- Create separate form schemas for string inputs
- Use `safeParse` for error handling
- Export inferred types for reuse
- Validate at action boundaries
- Type loaders and actions with inferred types

This architecture ensures end-to-end type safety from the database to the UI, with automatic validation and clear error handling.

