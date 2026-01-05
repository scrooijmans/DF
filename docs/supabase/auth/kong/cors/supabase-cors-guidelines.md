Title: CORS (Cross-Origin Resource Sharing) support for Invoking from the browser | Supabase Docs

Description: Add CORS headers to invoke Edge Functions from the browser.

Edge Functions

CORS (Cross-Origin Resource Sharing) support for Invoking from the browser

============================================================================

To invoke edge functions from the browser, you need to handle CORS Preflight requests.

See the example on GitHub.

### Recommended setup#

We recommend adding a `cors.ts` file within a `_shared` folder which makes it easy to reuse the CORS headers across functions:

```
1234export const corsHeaders = {  'Access-Control-Allow-Origin': '*',  'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',}
```

You can then import and use the CORS headers within your functions:

```
123456789101112131415161718192021222324252627import { corsHeaders } from '../_shared/cors.ts'console.log(`Function "browser-with-cors" up and running!`)Deno.serve(async (req) => {  // This is needed if you're planning to invoke your function from a browser.  if (req.method === 'OPTIONS') {    return new Response('ok', { headers: corsHeaders })  }  try {    const { name } = await req.json()    const data = {      message: `Hello ${name}!`,    }    return new Response(JSON.stringify(data), {      headers: { ...corsHeaders, 'Content-Type': 'application/json' },      status: 200,    })  } catch (error) {    return new Response(JSON.stringify({ error: error.message }), {      headers: { ...corsHeaders, 'Content-Type': 'application/json' },      status: 400,    })  }})
```

### Is this helpful?

No Yes