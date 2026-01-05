Title: Use Supabase with SvelteKit | Supabase Docs

Description: Learn how to create a Supabase project, add some sample data to your database, and query the data from a SvelteKit app.

Getting Started

Use Supabase with SvelteKit

=============================

Learn how to create a Supabase project, add some sample data to your database, and query the data from a SvelteKit app.

---

1

### Create a Supabase project

Go to database.new and create a new Supabase project.

Alternatively, you can create a project using the Management API:

```
1234567891011121314151617# First, get your access token from https://supabase.com/dashboard/account/tokensexport SUPABASE_ACCESS_TOKEN="your-access-token"# List your organizations to get the organization IDcurl -H "Authorization: Bearer $SUPABASE_ACCESS_TOKEN" \  https://api.supabase.com/v1/organizations# Create a new project (replace <org-id> with your organization ID)curl -X POST https://api.supabase.com/v1/projects \  -H "Authorization: Bearer $SUPABASE_ACCESS_TOKEN" \  -H "Content-Type: application/json" \  -d '{    "organization_id": "<org-id>",    "name": "My Project",    "region": "us-east-1",    "db_pass": "<your-secure-password>"  }'
```

When your project is up and running, go to the Table Editor, create a new table and insert some data.

Alternatively, you can run the following snippet in your project's SQL Editor. This will create a `instruments` table with some sample data.

```
12345678910111213-- Create the tablecreate table instruments (  id bigint primary key generated always as identity,  name text not null);-- Insert some sample data into the tableinsert into instruments (name)values  ('violin'),  ('viola'),  ('cello');alter table instruments enable row level security;
```

Make the data in your table publicly readable by adding an RLS policy:

```
1234create policy "public can read instruments"on public.instrumentsfor select to anonusing (true);
```

2

### Create a SvelteKit app

Create a SvelteKit app using the `npm create` command.

###### Terminal

```
1npx sv create my-app
```

3

### Install the Supabase client library

The fastest way to get started is to use the `supabase-js` client library which provides a convenient interface for working with Supabase from a SvelteKit app.

Navigate to the SvelteKit app and install `supabase-js`.

###### Terminal

```
1cd my-app && npm install @supabase/supabase-js
```

4

### Declare Supabase Environment Variables

Create a `.env` file at the root of your project and populate with your Supabase connection variables:

###### Project URL

No project found

###### Publishable key

No project found

###### Anon key

No project found

.env

```
12PUBLIC_SUPABASE_URL=<SUBSTITUTE_SUPABASE_URL>PUBLIC_SUPABASE_PUBLISHABLE_KEY=<SUBSTITUTE_SUPABASE_PUBLISHABLE_KEY>
```

5

### Create the Supabase client

Create a `src/lib` directory in your SvelteKit app, create a file called `supabaseClient.js` and add the following code to initialize the Supabase client:

src/lib/supabaseClient.jssrc/lib/supabaseClient.ts

```
1234import { createClient } from '@supabase/supabase-js';import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_PUBLISHABLE_KEY } from '$env/static/public';export const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_PUBLISHABLE_KEY)
```

6

### Query data from the app

Use `load` method to fetch the data server-side and display the query results as a simple list.

Create `+page.server.js` file in the `src/routes` directory with the following code.

src/routes/+page.server.jssrc/routes/+page.server.ts

```
12345678import { supabase } from "$lib/supabaseClient";  export async function load() {    const { data } = await supabase.from("instruments").select();    return {      instruments: data ?? [],    };  }
```

Replace the existing content in your `+page.svelte` file in the `src/routes` directory with the following code.

###### src/routes/+page.svelte

```
123456789<script>  let { data } = $props();</script><ul>  {#each data.instruments as instrument}    <li>{instrument.name}</li>  {/each}</ul>
```

7

### Start the app

Start the app and go to http://localhost:5173 in a browser and you should see the list of instruments.

###### Terminal

```
1npm run dev
```

## Next steps#

- Set up Auth for your app
- Insert more data into your database
- Upload and serve static files using Storage
