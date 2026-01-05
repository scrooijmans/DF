# Build a User Management App with Svelte | Supabase Docs
* * *

This tutorial demonstrates how to build a basic user management app. The app authenticates and identifies the user, stores their profile information in the database, and allows the user to log in, update their profile details, and upload a profile photo. The app uses:

*   [Supabase Database](https://supabase.com/docs/guides/database) - a Postgres database for storing your user data and [Row Level Security](about:/docs/guides/auth#row-level-security) so data is protected and users can only access their own information.
*   [Supabase Auth](https://supabase.com/docs/guides/auth) - allow users to sign up and log in.
*   [Supabase Storage](https://supabase.com/docs/guides/storage) - allow users to upload a profile photo.

![Supabase User Management example](https://supabase.com/docs/img/user-management-demo.png)

Before you start building you need to set up the Database and API. You can do this by starting a new Project in Supabase and then creating a "schema" inside the database.

### Create a project

1.  [Create a new project](https://supabase.com/dashboard) in the Supabase Dashboard.
2.  Enter your project details.
3.  Wait for the new database to launch.

### Set up the database schema

Now set up the database schema. You can use the "User Management Starter" quickstart in the SQL Editor, or you can copy/paste the SQL from below and run it.

1.  Go to the [SQL Editor](https://supabase.com/dashboard/project/_/sql) page in the Dashboard.
2.  Click **User Management Starter** under the **Community > Quickstarts** tab.
3.  Click **Run**.

### Get API details

Now that you've created some database tables, you are ready to insert data using the auto-generated API.

To do this, you need to get the Project URL and key. Get the URL from [the API settings section](https://supabase.com/dashboard/project/_/settings/api) of a project and the key from the [the API Keys section of a project's Settings page](https://supabase.com/dashboard/project/_/settings/api-keys/).

Start building the Svelte app from scratch.

### Initialize a Svelte app

You can use the Vite Svelte TypeScript Template to initialize an app called `supabase-svelte`:

```

1
2
3
npm create vite@latest supabase-svelte -- --template svelte-tscd supabase-sveltenpm install
```


Install the only additional dependency: [supabase-js](https://github.com/supabase/supabase-js)

```

1
npm install @supabase/supabase-js
```


Finally, save the environment variables in a `.env`. All you need are the API URL and the key that you copied [earlier](#get-api-details).

```

1
2
VITE_SUPABASE_URL=YOUR_SUPABASE_URLVITE_SUPABASE_PUBLISHABLE_KEY=YOUR_SUPABASE_PUBLISHABLE_KEY
```


Now you have the API credentials in place, create a helper file to initialize the Supabase client. These variables will be exposed on the browser, and that's fine since you have [Row Level Security](about:/docs/guides/auth#row-level-security) enabled on the Database.

###### src/supabaseClient.ts

```

1
2
3
4
5
6
import { createClient } from '@supabase/supabase-js'const supabaseUrl = import.meta.env.VITE_SUPABASE_URLconst supabasePublishableKey = import.meta.env.VITE_SUPABASE_PUBLISHABLE_KEYexport const supabase = createClient(supabaseUrl, supabasePublishableKey)
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/supabaseClient.ts)

### App styling (optional)

Optionally, update the CSS file `src/app.css` to make the app look nice. You can find the full contents of this file [on GitHub](https://raw.githubusercontent.com/supabase/supabase/master/examples/user-management/svelte-user-management/src/app.css).

### Set up a login component

Set up a Svelte component to manage logins and sign ups. It uses Magic Links, so users can sign in with their email without using passwords.

###### src/lib/Auth.svelte

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
<script lang="ts">  import { supabase } from "../supabaseClient";  let loading = $state(false);  let email = $state("");  const handleLogin = async () => {    try {      loading = true;      const { error } = await supabase.auth.signInWithOtp({ email });      if (error) throw error;      alert("Check your email for login link!");    } catch (error) {      if (error instanceof Error) {        alert(error.message);      }    } finally {      loading = false;    }  };</script><div class="row flex-center flex">  <div class="col-6 form-widget" aria-live="polite">    <h1 class="header">Supabase + Svelte</h1>    <p class="description">Sign in via magic link with your email below</p>    <form class="form-widget" onsubmit={(e) => { e.preventDefault(); handleLogin(); }}>      <div>        <label for="email">Email</label>        <input          id="email"          class="inputField"          type="email"          placeholder="Your email"          bind:value={email}        />      </div>      <div>        <button          type="submit"          class="button block"          aria-live="polite"          disabled={loading}        >          <span>{loading ? "Loading" : "Send magic link"}</span>        </button>      </div>    </form>  </div></div>
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/lib/Auth.svelte)

### Account page

After a user is signed in, allow them to edit their profile details and manage their account. Create a new component for that called `Account.svelte`.

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
<script lang="ts">  import { onMount } from "svelte";  import type { AuthSession } from "@supabase/supabase-js";  import { supabase } from "../supabaseClient";  // ...  interface Props {    session: AuthSession;  }  let { session }: Props = $props();  // ...  let username = $state<string | null>(null);  let website = $state<string | null>(null);  let avatarUrl = $state<string | null>(null);  onMount(() => {    getProfile();  });  const getProfile = async () => {    try {      loading = true;      const { user } = session;      const { data, error, status } = await supabase        .from("profiles")        .select("username, website, avatar_url")        .eq("id", user.id)        .single();      if (error && status !== 406) throw error;// ...      if (data) {        username = data.username;        website = data.website;        avatarUrl = data.avatar_url;      }    } catch (error) {      if (error instanceof Error) {        alert(error.message);      }    } finally {      loading = false;    }  };  const updateProfile = async () => {    try {      loading = true;      const { user } = session;        // ...        id: user.id,        username,        website,        avatar_url: avatarUrl,        updated_at: new Date().toISOString(),      };      const { error } = await supabase.from("profiles").upsert(updates);      if (error) {        throw error;      }    } catch (error) {      if (error instanceof Error) {        alert(error.message);      }    } finally {      loading = false;    }// ...</script><form onsubmit={(e) => { e.preventDefault(); updateProfile(); }} class="form-widget">  <div>Email: {session.user.email}</div>  <div>    <Avatar bind:url={avatarUrl} size={150} onupload={updateProfile} />    <label for="username">Name</label>    <input id="username" type="text" bind:value={username} />  </div>  <div>    <label for="website">Website</label>    <input id="website" type="text" bind:value={website} />  </div>  <div>    <button type="submit" class="button primary block" disabled={loading}>      {loading ? "Saving ..." : "Update profile"}    </button>  </div>  <button    type="button"    class="button block"    onclick={() => supabase.auth.signOut()}  >    Sign Out  </button></form>
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/lib/Account.svelte)

### Launch!

Now that you have all the components in place, update `App.svelte`:

###### src/App.svelte

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
<script lang="ts">  import { onMount } from 'svelte'  import { supabase } from './supabaseClient'  import type { AuthSession } from '@supabase/supabase-js'  import Account from './lib/Account.svelte'  import Auth from './lib/Auth.svelte'  let session = $state<AuthSession | null>(null)  onMount(() => {    supabase.auth.getSession().then(({ data }) => {      session = data.session    })    supabase.auth.onAuthStateChange((_event, _session) => {      session = _session    })  })</script><div class="container" style="padding: 50px 0 100px 0">  {#if !session}  <Auth />  {:else}  <Account {session} />  {/if}</div>
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/App.svelte)

Once that's done, run this in a terminal window:

And then open the browser to [localhost:5173](http://localhost:5173/) and you should see the completed app.

![Supabase Svelte](https://supabase.com/docs/img/supabase-svelte-demo.png)

Every Supabase project is configured with [Storage](https://supabase.com/docs/guides/storage) for managing large files like photos and videos.

### Create an upload widget

Create an avatar for the user so that they can upload a profile photo. Start by creating a new component:

###### src/lib/Avatar.svelte

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
<script lang="ts">  import { supabase } from "../supabaseClient";  interface Props {    size: number;    url?: string | null;    onupload?: () => void;  }  let { size, url = $bindable(null), onupload }: Props = $props();  let avatarUrl = $state<string | null>(null);  let uploading = $state(false);  let files = $state<FileList>();  const downloadImage = async (path: string) => {    try {      const { data, error } = await supabase.storage        .from("avatars")        .download(path);      if (error) {        throw error;      }      const url = URL.createObjectURL(data);      avatarUrl = url;    } catch (error) {      if (error instanceof Error) {        console.log("Error downloading image: ", error.message);      }    }  };  const uploadAvatar = async () => {    try {      uploading = true;      if (!files || files.length === 0) {        throw new Error("You must select an image to upload.");      }      const file = files[0];      const fileExt = file.name.split(".").pop();      const filePath = `${Math.random()}.${fileExt}`;      const { error } = await supabase.storage        .from("avatars")        .upload(filePath, file);      if (error) {        throw error;      }      url = filePath;      onupload?.();    } catch (error) {      if (error instanceof Error) {        alert(error.message);      }    } finally {      uploading = false;    }  };  $effect(() => {    if (url) downloadImage(url);  });</script><div style="width: {size}px" aria-live="polite">  {#if avatarUrl}    <img      src={avatarUrl}      alt={avatarUrl ? "Avatar" : "No image"}      class="avatar image"      style="height: {size}px, width: {size}px"    />  {:else}    <div class="avatar no-image" style="height: {size}px, width: {size}px"></div>  {/if}  <div style="width: {size}px">    <label class="button primary block" for="single">      {uploading ? "Uploading ..." : "Upload avatar"}    </label>    <span style="display:none">      <input        type="file"        id="single"        accept="image/*"        bind:files        onchange={uploadAvatar}        disabled={uploading}      />    </span>  </div></div>
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/lib/Avatar.svelte)

### Add the new widget

And then you can add the widget to the Account page:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
<script lang="ts">  // ...  import Avatar from "./Avatar.svelte";    // ...    } finally {      loading = false;    }  // ...  };  // ...  </div>  <button    type="button"    class="button block"    onclick={() => supabase.auth.signOut()}  >    Sign Out  </button></form>
```


[View source](https://github.com/supabase/supabase/blob/031c5644e7d6d3068206e7ce4ab6905f1634af20/examples/user-management/svelte-user-management/src/lib/Account.svelte)

At this stage you have a fully functional application!