Title: Self-Hosting | Supabase Docs

Description: Supabase is the Postgres development platform providing all the backend features you need to build a product.

Auth Server Reference

Self-Hosting Auth
=================

The Supabase Auth Server (GoTrue) is a JSON Web Token (JWT)-based API for managing users and issuing access tokens.

GoTrue is an open-source API written in Golang, that acts as a self-standing API service for handling user registration and authentication for JAM projects. It's based on OAuth2 and JWT and handles user signup, authentication, and custom user data.

### Client libraries#

*   JavaScript
*   Dart

### Additional links#

*   Source code
*   Known bugs and issues
*   Auth guides

Generates an email action link.
-------------------------------

post`/admin/generate_link`

### Body

*   dataOptionalobject

Object schema

*   emailOptionalstring

*   new\_emailOptionalstring

*   passwordOptionalstring

*   redirect\_toOptionalstring

*   typeOptionalstring

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
12345678910111213141516171819202122232425262728293031323334353637383940414243444546474849{  "action_link": "lorem",  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "email_otp": "lorem",  "hashed_token": "lorem",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "redirect_to": "lorem",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  },  "verification_type": "lorem"}
```

Get a user.
-----------

get`/admin/user/{user_id}`

### Path parameters

*   user\_idRequired

The user's id

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Update a user.
--------------

put`/admin/user/{user_id}`

### Path parameters

*   user\_idRequired

The user's id

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Deletes a user.
---------------

delete`/admin/user/{user_id}`

### Path parameters

*   user\_idRequired

The user's id

### Response codes

*   200
*   401

### Response (200)

schema

```
1{}
```

List all users.
---------------

get`/admin/users`

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
12345678910111213141516171819202122232425262728293031323334353637383940414243444546474849{  "aud": "lorem",  "users": [    {      "app_metadata": {        "property1": null,        "property2": null      },      "aud": "lorem",      "banned_until": "2021-12-31T23:34:00Z",      "confirmation_sent_at": "2021-12-31T23:34:00Z",      "confirmed_at": "2021-12-31T23:34:00Z",      "created_at": "2021-12-31T23:34:00Z",      "email": "lorem",      "email_change_sent_at": "2021-12-31T23:34:00Z",      "email_confirmed_at": "2021-12-31T23:34:00Z",      "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",      "identities": [        {          "created_at": "2021-12-31T23:34:00Z",          "id": "lorem",          "identity_data": {            "property1": null,            "property2": null          },          "last_sign_in_at": "2021-12-31T23:34:00Z",          "provider": "lorem",          "updated_at": "2021-12-31T23:34:00Z",          "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"        }      ],      "invited_at": "2021-12-31T23:34:00Z",      "last_sign_in_at": "2021-12-31T23:34:00Z",      "new_email": "lorem",      "new_phone": "lorem",      "phone": "lorem",      "phone_change_sent_at": "2021-12-31T23:34:00Z",      "phone_confirmed_at": "2021-12-31T23:34:00Z",      "reauthentication_sent_at": "2021-12-31T23:34:00Z",      "recovery_sent_at": "2021-12-31T23:34:00Z",      "role": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_metadata": {        "property1": null,        "property2": null      }    }  ]}
```

Returns the created user.
-------------------------

post`/admin/users`

### Body

*   app\_metadataOptionalobject

Object schema

*   audOptionalstring

*   ban\_durationOptionalstring

*   emailOptionalstring

*   email\_confirmOptionalboolean

*   passwordOptionalstring

*   phoneOptionalstring

*   phone\_confirmOptionalboolean

*   roleOptionalstring

*   user\_metadataOptionalobject

Object schema

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Receives the redirect from an external provider during the OAuth authentication process. Starts the process of creating an access and refresh token.
----------------------------------------------------------------------------------------------------------------------------------------------------

get`/callback`

### Response codes

*   302

The healthcheck endpoint for gotrue. Returns the current gotrue version.
------------------------------------------------------------------------

get`/health`

### Response codes

*   200

### Response (200)

exampleschema

```
12345{  "description": "lorem",  "name": "lorem",  "version": "lorem"}
```

Sends an invite link to the user.
---------------------------------

post`/invite`

### Body

*   dataOptionalobject

Object schema

*   emailOptionalstring

### Response codes

*   200

### Response (200)

schema

```
1{}
```

Logs out the user.
------------------

post`/logout`

### Response codes

*   204

### Response (204)

schema

```
1{}
```

Passwordless sign-in method for email or phone.
-----------------------------------------------

post`/otp`

### Body

*   create\_userOptionalboolean

*   dataOptionalobject

Object schema

*   emailOptionalstring

*   phoneOptionalstring

### Response codes

*   200

### Response (200)

schema

```
1{}
```

Sends a password recovery email link to the user's email.
---------------------------------------------------------

post`/recover`

### Body

*   emailOptionalstring

### Response codes

*   200

### Response (200)

schema

```
1{}
```

Returns the configuration settings for the gotrue server.
---------------------------------------------------------

get`/settings`

### Response codes

*   200

### Response (200)

exampleschema

```
12345678910111213141516171819202122232425262728{  "disable_signup": true,  "external": {    "apple": true,    "azure": true,    "bitbucket": true,    "discord": true,    "email": true,    "facebook": true,    "github": true,    "gitlab": true,    "google": true,    "keycloak": true,    "linkedin": true,    "notion": true,    "phone": true,    "saml": true,    "slack": true,    "spotify": true,    "twitch": true,    "twitter": true,    "workos": true,    "zoom": true  },  "mailer_autoconfirm": true,  "phone_autoconfirm": true,  "sms_provider": "lorem"}
```

Password-based signup with either email or phone.
-------------------------------------------------

post`/signup`

### Body

*   dataOptionalobject

Object schema

*   emailOptionalstring

*   passwordOptionalstring

*   phoneOptionalstring

### Response codes

*   200

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Signs in a user with a password.
--------------------------------

post`/token?grant_type=password`

### Body

*   emailOptionalstring

*   passwordOptionalstring

*   phoneOptionalstring

### Response codes

*   200

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344454647484950{  "access_token": "lorem",  "expires_in": 42,  "refresh_token": "lorem",  "token_type": "lorem",  "user": {    "app_metadata": {      "property1": null,      "property2": null    },    "aud": "lorem",    "banned_until": "2021-12-31T23:34:00Z",    "confirmation_sent_at": "2021-12-31T23:34:00Z",    "confirmed_at": "2021-12-31T23:34:00Z",    "created_at": "2021-12-31T23:34:00Z",    "email": "lorem",    "email_change_sent_at": "2021-12-31T23:34:00Z",    "email_confirmed_at": "2021-12-31T23:34:00Z",    "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",    "identities": [      {        "created_at": "2021-12-31T23:34:00Z",        "id": "lorem",        "identity_data": {          "property1": null,          "property2": null        },        "last_sign_in_at": "2021-12-31T23:34:00Z",        "provider": "lorem",        "updated_at": "2021-12-31T23:34:00Z",        "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"      }    ],    "invited_at": "2021-12-31T23:34:00Z",    "last_sign_in_at": "2021-12-31T23:34:00Z",    "new_email": "lorem",    "new_phone": "lorem",    "phone": "lorem",    "phone_change_sent_at": "2021-12-31T23:34:00Z",    "phone_confirmed_at": "2021-12-31T23:34:00Z",    "reauthentication_sent_at": "2021-12-31T23:34:00Z",    "recovery_sent_at": "2021-12-31T23:34:00Z",    "role": "lorem",    "updated_at": "2021-12-31T23:34:00Z",    "user_metadata": {      "property1": null,      "property2": null    }  }}
```

Refreshes a user's refresh token.
---------------------------------

post`/token?grant_type=refresh_token`

### Body

*   refresh\_tokenOptionalstring

### Response codes

*   200

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344454647484950{  "access_token": "lorem",  "expires_in": 42,  "refresh_token": "lorem",  "token_type": "lorem",  "user": {    "app_metadata": {      "property1": null,      "property2": null    },    "aud": "lorem",    "banned_until": "2021-12-31T23:34:00Z",    "confirmation_sent_at": "2021-12-31T23:34:00Z",    "confirmed_at": "2021-12-31T23:34:00Z",    "created_at": "2021-12-31T23:34:00Z",    "email": "lorem",    "email_change_sent_at": "2021-12-31T23:34:00Z",    "email_confirmed_at": "2021-12-31T23:34:00Z",    "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",    "identities": [      {        "created_at": "2021-12-31T23:34:00Z",        "id": "lorem",        "identity_data": {          "property1": null,          "property2": null        },        "last_sign_in_at": "2021-12-31T23:34:00Z",        "provider": "lorem",        "updated_at": "2021-12-31T23:34:00Z",        "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"      }    ],    "invited_at": "2021-12-31T23:34:00Z",    "last_sign_in_at": "2021-12-31T23:34:00Z",    "new_email": "lorem",    "new_phone": "lorem",    "phone": "lorem",    "phone_change_sent_at": "2021-12-31T23:34:00Z",    "phone_confirmed_at": "2021-12-31T23:34:00Z",    "reauthentication_sent_at": "2021-12-31T23:34:00Z",    "recovery_sent_at": "2021-12-31T23:34:00Z",    "role": "lorem",    "updated_at": "2021-12-31T23:34:00Z",    "user_metadata": {      "property1": null,      "property2": null    }  }}
```

Get information for the logged-in user.
---------------------------------------

get`/user`

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Returns the updated user.
-------------------------

put`/user`

### Body

*   app\_metadataOptionalobject

Object schema

*   dataOptionalobject

Object schema

*   emailOptionalstring

*   nonceOptionalstring

*   passwordOptionalstring

*   phoneOptionalstring

### Response codes

*   200
*   401

### Response (200)

exampleschema

```
1234567891011121314151617181920212223242526272829303132333435363738394041424344{  "app_metadata": {    "property1": null,    "property2": null  },  "aud": "lorem",  "banned_until": "2021-12-31T23:34:00Z",  "confirmation_sent_at": "2021-12-31T23:34:00Z",  "confirmed_at": "2021-12-31T23:34:00Z",  "created_at": "2021-12-31T23:34:00Z",  "email": "lorem",  "email_change_sent_at": "2021-12-31T23:34:00Z",  "email_confirmed_at": "2021-12-31T23:34:00Z",  "id": "fbdf5a53-161e-4460-98ad-0e39408d8689",  "identities": [    {      "created_at": "2021-12-31T23:34:00Z",      "id": "lorem",      "identity_data": {        "property1": null,        "property2": null      },      "last_sign_in_at": "2021-12-31T23:34:00Z",      "provider": "lorem",      "updated_at": "2021-12-31T23:34:00Z",      "user_id": "fbdf5a53-161e-4460-98ad-0e39408d8689"    }  ],  "invited_at": "2021-12-31T23:34:00Z",  "last_sign_in_at": "2021-12-31T23:34:00Z",  "new_email": "lorem",  "new_phone": "lorem",  "phone": "lorem",  "phone_change_sent_at": "2021-12-31T23:34:00Z",  "phone_confirmed_at": "2021-12-31T23:34:00Z",  "reauthentication_sent_at": "2021-12-31T23:34:00Z",  "recovery_sent_at": "2021-12-31T23:34:00Z",  "role": "lorem",  "updated_at": "2021-12-31T23:34:00Z",  "user_metadata": {    "property1": null,    "property2": null  }}
```

Verifies a sign up.
-------------------

post`/verify`

### Body

*   emailOptionalstring

*   phoneOptionalstring

*   redirect\_toOptionalstring

*   tokenOptionalstring

*   typeOptionalstring