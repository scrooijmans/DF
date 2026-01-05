Daniel
dhew

1
May 23
Problem: By nature of being a frontend builder, WeWeb has no ability to manage app users after deployment. If you need to add or remove users, you must do so directly with whatever tools you are using for authentication, which is often Xano or Supabase for no-coders. In the case of Supabase, this means Supabase Auth.

For Supabase Auth, to securely create/delete users from a frontend running in a user’s browser, you need to use an Edge Function. Edge functions operate as ‘middleware’ in this case, doing sensitive work without revealing sensitive credentials to users.

Goal: Securely add and remove Supabase Auth users via an ‘admin’ page in your WeWeb app.

Note: This is what I came up with through AI coding tools. Your use of all this information is at your own risk.

I assume you have Supabase (and Supabase Auth) setup and connected to WeWeb already (using WeWeb’s native plugin). I’m skipping over a couple setup steps around creation of roles, users_roles, and user_profile tables–check WeWeb’s Supabase specific videos for this, if needed.

Create New User

Create a new Edge Function in Supabase. Call it “create-user” (or whatever you want). Replace all the starter code with the below, or similar.
Notes on what this does.

Creates a single user at a time

Function to ‘normalize’ the user’s full name

Check authorization of user requesting use of the function

Ensure calling user is logged in and has a specific role ID. I manually specified this role ID, and it comes from the WeWeb created role and users_roles tables (basically, create an “admin” role in WeWeb, lookup its role ID in Supabase’s table editor (the role ID should be a unique, long value, such as 17e6v-dk8euw-3r2r332-dsdsaw), then copy and paste below, where is says INSERT YOUR REQUIRED ROLE ID INSIDE THESE QUOTES
Allow calling user to specify the role ID of the new users the function is about to create. Same as above, but this time you want to list the role ids the new user could be given. This can prevent the “admin” user from creating other admins, for example. Insert these role IDs where it says INSERT ASSIGNABLE ROLE ID INSIDE THESE QUOTES

If create/invite the user works, it will insert a new row into the users_roles table.
Function will check to ensure a user with that email does not already exist.

Function ‘invites’ a new user vs ‘create’. This is a technical difference, but the user will exist in the database after the function succeeds either way. Note, you will need to have an email system setup in Supabase to make this usable in production systems. More on that at the end of this post.

You can explicitly set your WeWeb apps login/landing/home page URL inside the quotes where is says https://YOUR DOMAIN NAME.weweb-preview.io/
If create/invite works, it will update the user_profile table with the new user’s name. Note, this is a table you create on your own, as an alternative to relying on Supabase raw_user_meta_data. When you use the ‘invite’ method of creating a user you can not directly access the raw_user_meta_data, so you need a second action either way and I opted to just update a public table instead.

Further note: I have a “database function” in Supabase setup (in the public schema) to automatically create an entry in the user_profile table whenever a new user is added to the auth.users table. This is why it is an ‘update’ and not an ‘insert’ action. There is a WeWeb video on how to create this simple database function in Supabase.
This function accepts 3 pieces of info: new user’s email address, target role ID, and name. Example of how to send that shown below code.

import "jsr:@supabase/functions-js/edge-runtime.d.ts";
import { serve } from "https://deno.land/std@0.131.0/http/server.ts";
import { createClient } from "jsr:@supabase/supabase-js";
// Helper: normalize name to title case
function toTitleCase(str) {
  return str.toLowerCase().split(/\s+/).map((word)=>word.charAt(0).toUpperCase() + word.slice(1)).join(" ");
}
// Helper: log error
function logError(message, details) {
  console.error("[ERROR]", new Date().toISOString(), message, details ?? "");
}
// Helper: log info
function logInfo(message, details) {
  console.log("[INFO]", new Date().toISOString(), message, details ?? "");
}
// Helper: extract JWT user ID
function getUserFromAuthHeader(req) {
  const authHeader = req.headers.get("authorization");
  if (!authHeader || !authHeader.startsWith("Bearer ")) return null;
  const token = authHeader.replace("Bearer ", "");
  const payload = JSON.parse(atob(token.split(".")[1]));
  return payload.sub || null;
}
// Helper: check if caller has required role
async function userHasRequiredRole(userId, requiredRoleId) {
  const { data, error } = await supabaseClient.from("users_roles").select("role_id").eq("user_id", userId);
  if (error) {
    logError("Failed to fetch user roles", error);
    return false;
  }
  return data.some((r)=>r.role_id === requiredRoleId);
}
// Supabase setup
const supabaseClient = createClient(Deno.env.get("SUPABASE_URL"), Deno.env.get("SUPABASE_SERVICE_ROLE_KEY"));
// CORS headers
const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
  "Content-Type": "application/json"
};
// Main function
serve(async (req)=>{
  if (req.method === "OPTIONS") {
    return new Response("ok", {
      headers: corsHeaders
    });
  }
  const REQUIRED_CALLER_ROLE = "INSERT YOUR REQUIRED ROLE ID INSIDE THESE QUOTES";
  const ALLOWED_ASSIGNABLE_ROLE_IDS = [
    "INSERT ASSIGNABLE ROLE ID INSIDE THESE QUOTES",
    "INSERT ANOTHER ASSIGNABLE ROLE ID INSIDE THESE QUOTES"
  ];
  const callerUserId = getUserFromAuthHeader(req);
  if (!callerUserId) {
    logError("Missing or invalid authorization token.");
    return new Response(JSON.stringify({
      error: "Unauthorized"
    }), {
      headers: corsHeaders,
      status: 401
    });
  }
  const isAuthorized = await userHasRequiredRole(callerUserId, REQUIRED_CALLER_ROLE);
  if (!isAuthorized) {
    logInfo(`User ${callerUserId} does not have permission to invite users.`);
    return new Response(JSON.stringify({
      error: "Forbidden"
    }), {
      headers: corsHeaders,
      status: 403
    });
  }
  let email;
  let targetRoleId;
  let name;
  try {
    const body = await req.json();
    email = body.email?.trim();
    targetRoleId = body.role_id?.trim();
    name = toTitleCase(body.name?.trim());
    if (!email || !targetRoleId || !name) {
      throw new Error("Missing email, role_id, or name");
    }
  } catch (e) {
    logError("Invalid request body or name normalization failed", e);
    return new Response(JSON.stringify({
      error: "Invalid request body"
    }), {
      headers: corsHeaders,
      status: 400
    });
  }
  if (!ALLOWED_ASSIGNABLE_ROLE_IDS.includes(targetRoleId)) {
    logError(`Unauthorized role_id assignment attempt by ${callerUserId}`);
    return new Response(JSON.stringify({
      error: "Invalid role_id"
    }), {
      headers: corsHeaders,
      status: 403
    });
  }
  // Check if user already exists
  logInfo(`Checking if user already exists: ${email}`);
  const { data: userListData, error: listError } = await supabaseClient.auth.admin.listUsers({
    email
  });
  if (listError) {
    logError("Error checking existing users", listError);
    return new Response(JSON.stringify({
      error: "Failed to check user existence"
    }), {
      headers: corsHeaders,
      status: 500
    });
  }
  const existingUser = userListData?.users?.find((u)=>u.email === email);
  if (existingUser) {
    logInfo(`User with email ${email} already exists. Skipping invite.`);
    return new Response(JSON.stringify({
      message: "User already exists"
    }), {
      headers: corsHeaders,
      status: 200
    });
  }
  // Invite user
  logInfo(`Inviting new user: ${email}`);
  const { data: inviteData, error: inviteError } = await supabaseClient.auth.admin.inviteUserByEmail(email, {
    redirectTo: "https://YOUR DOMAIN NAME.weweb-preview.io/"
  });
  if (inviteError || !inviteData?.user?.id) {
    logError("Failed to invite user", inviteError || inviteData);
    return new Response(JSON.stringify({
      error: "User invite failed"
    }), {
      headers: corsHeaders,
      status: 500
    });
  }
  const newUserId = inviteData.user.id;
  // Assign role
  const { error: insertRoleError } = await supabaseClient.from("users_roles").insert([
    {
      user_id: newUserId,
      role_id: targetRoleId
    }
  ]);
  if (insertRoleError) {
    logError("Failed to assign role to invited user", insertRoleError);
    return new Response(JSON.stringify({
      error: "Role assignment failed"
    }), {
      headers: corsHeaders,
      status: 500
    });
  }
  // Insert full name into user_profile table
  const { error: profileUpdateError } = await supabaseClient.from("user_profile").update({
    full_name: name
  }).eq("id", newUserId);
  if (profileUpdateError) {
    logError("Failed to insert user profile", profileUpdateError);
    return new Response(JSON.stringify({
      error: "Profile update failed"
    }), {
      headers: corsHeaders,
      status: 500
    });
  }
  logInfo("User invited, role assigned, and profile data set", {
    newUserId,
    role: targetRoleId,
    name: name
  });
  return new Response(JSON.stringify(inviteData.user), {
    headers: corsHeaders,
    status: 200
  });
});

Build a form in WeWeb (preferably on a private page with role restriction to the Admin role you set earlier) to create a user:
Page level protection
Page level protection
1662×1038 74.3 KB
Simple form to create a new user
Simple form to create a new user
1918×722 28.3 KB
Create workflow, on form container submit
Workflow of form
Workflow of form
726×942 33.8 KB
Add a header with key “Authorization” and value:
Value of Authorization key in header
Value of Authorization key in header
874×228 10.9 KB
Note that this dynamic variable can be found by drilling into the Supabase Auth tab in the formula builder. Expand “User” then “Session” and click “access_token”.

Showing Supabase Auth tab in formula builder
Showing Supabase Auth tab in formula builder
888×280 11.8 KB
Create the body of the request object. It must be formatted just like this, as an object with these specific keys, matching the keys in the edge function’s code:
Showing format of request "body"
Showing format of request "body"
1506×1538 285 KB
Testing and debugging.

When you execute this workflow in WeWeb, watch the log tab there first. If it doesnt work, you should get some useful error info. If it is not detailed enough, you can look in Supabase. There you want to go to the “Edge Functions” > “Functions” screen, select the function in question, then click on the “Invocations” tab first.

This will show you issues around authorization (the calling user isn’t logged in or isn’t the right role).

For more detailed errors, relating to the execution of the function inside Supabase, go to the “Logs” tab. You will see “ERROR” in red if there is something wrong with the function’s code. Ex:

Example Supabase Log showing a code error inside the function itself
Example Supabase Log showing a code error inside the function itself
2588×176 22.9 KB
Note: Free Supabase plans only show logs for the last 24 hours–so try to do your testing and debugging within that window.

Delete Users

This is very similar to the previous, so I will not show each step. However, I have this setup to allow deleting multiple users at a time, so the body of the request from WeWeb is different.

Requires a certain role ID to execute this function. Replace INSERT YOUR REQUIRED ROLE ID INSIDE THESE QUOTES in the code with the role_id you want to use.
This specific script removes the users from Supabase Auth, the users_roles table, and user_profile table. Your table names may be different or you may have fewer.
Here is the Supabase Edge Function code:

import "jsr:@supabase/functions-js/edge-runtime.d.ts";
import { serve } from "https://deno.land/std@0.131.0/http/server.ts";
import { createClient } from "jsr:@supabase/supabase-js";
// Helper: log error
function logError(message, details) {
  console.error("[ERROR]", new Date().toISOString(), message, details ?? "");
}
// Helper: log info
function logInfo(message, details) {
  console.log("[INFO]", new Date().toISOString(), message, details ?? "");
}
// Helper: extract JWT user ID
function getUserFromAuthHeader(req) {
  const authHeader = req.headers.get("authorization");
  if (!authHeader || !authHeader.startsWith("Bearer ")) return null;
  const token = authHeader.replace("Bearer ", "");
  try {
    const payload = JSON.parse(atob(token.split(".")[1]));
    return payload.sub || null;
  } catch  {
    return null;
  }
}
// Helper: check if user has required role
async function userHasRequiredRole(userId, requiredRoleId) {
  const { data, error } = await supabaseClient.from("users_roles").select("role_id").eq("user_id", userId);
  if (error) {
    logError("Failed to fetch user roles", error);
    return false;
  }
  return data.some((r)=>r.role_id === requiredRoleId);
}
// Supabase setup
const supabaseClient = createClient(Deno.env.get("SUPABASE_URL"), Deno.env.get("SUPABASE_SERVICE_ROLE_KEY"));
// CORS headers
const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
  "Content-Type": "application/json"
};
// Main function
serve(async (req)=>{
  if (req.method === "OPTIONS") {
    return new Response("ok", {
      headers: corsHeaders
    });
  }
  const REQUIRED_ROLE_ID = "INSERT YOUR REQUIRED ROLE ID INSIDE THESE QUOTES";
  const callerUserId = getUserFromAuthHeader(req);
  if (!callerUserId) {
    logError("Missing or invalid authorization token.");
    return new Response(JSON.stringify({
      error: "Unauthorized"
    }), {
      headers: corsHeaders,
      status: 401
    });
  }
  const isAuthorized = await userHasRequiredRole(callerUserId, REQUIRED_ROLE_ID);
  if (!isAuthorized) {
    logInfo(`User ${callerUserId} does not have permission to delete users.`);
    return new Response(JSON.stringify({
      error: "Forbidden"
    }), {
      headers: corsHeaders,
      status: 403
    });
  }
  let userIds = [];
  try {
    const body = await req.json();
    userIds = body.user_ids;
    if (!Array.isArray(userIds) || userIds.length === 0) {
      throw new Error("Invalid or empty user_ids array.");
    }
  } catch (e) {
    logError("Failed to parse request body", e);
    return new Response(JSON.stringify({
      error: "Invalid request body"
    }), {
      headers: corsHeaders,
      status: 400
    });
  }
  const results = {};
  for (const userId of userIds){
    try {
      // Delete from Supabase Auth
      const { error: authDeleteError } = await supabaseClient.auth.admin.deleteUser(userId);
      if (authDeleteError) throw authDeleteError;
      // Delete from users_roles
      const { error: roleDeleteError } = await supabaseClient.from("users_roles").delete().eq("user_id", userId);
      if (roleDeleteError) throw roleDeleteError;
      // Delete from user_profile
      const { error: profileDeleteError } = await supabaseClient.from("user_profile").delete().eq("user_id", userId);
      if (profileDeleteError) throw profileDeleteError;
      results[userId] = "Deleted";
    } catch (err) {
      logError(`Failed to delete user ${userId}`, err);
      results[userId] = `Error: ${err.message}`;
    }
  }
  return new Response(JSON.stringify({
    results
  }), {
    headers: corsHeaders,
    status: 200
  });
});

To select members to delete, I use a datagrid. But you can use whatever method you desire (a ‘select’ form element, a search element, etc.). Eventually you just need to trigger a workflow that runs the edge function action.

The headers are the same as the create user version.

The request body changes to an array of user_ids:

Delete multiple users request body format
Delete multiple users request body format
1496×1232 104 KB
Misc notes
The invite users method used in the create users section will email the new users a magic link to login. You will need to setup an email provider in Supabase to do this. You go to “Authentication” > “Emails” in Supabase and select “SMTP Settings” tab at the top.

You can use any mail provider, such as resend.

In order to complete setup with your selected mail provider, you will likely need to have your own domain name and make some DNS entries with your registrar to verify you own the domain. If you do not complete these steps (done with the email provider), your invite emails will leave Supabase but be held by the email provider…and you will not learn this until you check the email provider’s logs.

There are several guides online to help with this setup process–this note is simply to alert you the setup process requires a few steps before it is operational.

Also,

I used Chat GPT quite a bit to generate this code, asking it to iterate as I added features and ran into errors. It was quite effective and very useful in both explaining what portions of code does and suggesting additional steps it could take to improve it. If you want to tweak this code or add other features, I would encourage you to copy and paste it into chat gpt, or similar, and see what it tells you as a first step…I learned a lot by doing that.