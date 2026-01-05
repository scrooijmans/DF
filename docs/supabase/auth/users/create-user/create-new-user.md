Title: JavaScript: Create a new user | Supabase Docs

Description: Supabase API reference for JavaScript: Create a new user

JavaScript: Create a new user
=============================

Creates a new user.

Be aware that if a user account exists in the system you may get back an error message that attempts to hide this information from the user. This method has support for PKCE via email signups. The PKCE flow cannot be used when autoconfirm is enabled.

*   By default, the user needs to verify their email address before logging in. To turn this off, disable **Confirm email** in your project.
*   **Confirm email** determines if users need to confirm their email address after signing up.
*   If **Confirm email** is enabled, a `user` is returned but `session` is null.
*   If **Confirm email** is disabled, both a `user` and a `session` are returned.
*   When the user confirms their email address, they are redirected to the `SITE_URL` by default. You can modify your `SITE_URL` or add additional redirect URLs in your project.
*   If signUp() is called for an existing confirmed user:
*   When both **Confirm email** and **Confirm phone** (even when phone provider is disabled) are enabled in your project, an obfuscated/fake user object is returned.
*   When either **Confirm email** or **Confirm phone** (even when phone provider is disabled) is disabled, the error message, `User already registered` is returned.
*   To fetch the currently logged-in user, refer to `getUser()`.

Parameters
----------

*   ### credentials

(Required)

Examples
--------

### Sign up with an email and password

```js
const { data, error } = await supabase.auth.signUp({
email: '[email protected]',
password: 'example-password',
})
```

### Sign up with a phone number and password (SMS)

```js
const { data, error } = await supabase.auth.signUp({
phone: '123456789',
password: 'example-password',
options: {
channel: 'sms'
}
})
```

### Sign up with a phone number and password (whatsapp)

```js
const { data, error } = await supabase.auth.signUp({
phone: '123456789',
password: 'example-password',
options: {
channel: 'whatsapp'
}
})
```

### Sign up with additional user metadata

```js
const { data, error } = await supabase.auth.signUp(
{
email: '[email protected]',
password: 'example-password',
options: {
data: {
first_name: 'John',
age: 27,
}
}
}
)
```

### Sign up with a redirect URL

```js
const { data, error } = await supabase.auth.signUp(
{
email: '[email protected]',
password: 'example-password',
options: {
emailRedirectTo: 'https://example.com/welcome'
}
}
)
```