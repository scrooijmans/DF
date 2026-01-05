Title: JavaScript: Sign in a user | Supabase Docs

Description: Supabase API reference for JavaScript: Sign in a user

JavaScript: Sign in a user
==========================

*   A user can sign up either via email or OAuth.
*   If you provide `email` without a `password`, the user will be sent a magic link.
*   The magic link's destination URL is determined by the SITE\_URL config variable. To change this, you can go to Authentication -> Settings on supabase.com/dashboard
*   Specifying a `provider` will open the browser to the relevant login page.

Examples
--------

### Sign in with email and password

```js
const { user, session, error } = await supabase.auth.signIn({
email: '[email protected]',
password: 'example-password',
})
```

### Sign in with magic link.

```js
const { user, session, error } = await supabase.auth.signIn({
email: '[email protected]'
})
```

### Sign in using third-party providers.

```js
const { user, session, error } = await supabase.auth.signIn({
// provider can be 'github', 'google', 'gitlab', and more
provider: 'github'
})
```

### Sign in with phone and password

```js
const { user, session, error } = await supabase.auth.signIn({
phone: '+13334445555',
password: 'some-password',
})
```

### Sign in using a third-party provider with redirect

```js
const { user, session, error } = await supabase.auth.signIn({
provider: 'github'
}, {
redirectTo: 'https://example.com/welcome'
})
```

### Sign in with scopes

```js
const { user, session, error } = await supabase.auth.signIn({
provider: 'github'
}, {
scopes: 'repo gist notifications'
})
const oAuthToken = session.provider_token // use to access provider API
```

### Sign in using a refresh token (e.g. in React Native).

```js
// An example using Expo's `AuthSession`
const redirectUri = AuthSession.makeRedirectUri({ useProxy: false });
const provider = 'google';

AuthSession.startAsync({
authUrl: `https://MYSUPABASEAPP.supabase.co/auth/v1/authorize?provider=${provider}&redirect_to=${redirectUri}`,
returnUrl: redirectUri,
}).then(async (response: any) => {
if (!response) return;
const { user, session, error } = await supabase.auth.signIn({
refreshToken: response.params?.refresh_token,
});
});
```