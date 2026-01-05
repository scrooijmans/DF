Title: Auth Self-hosting Config | Supabase Docs

Description: How to configure and deploy Supabase Auth.

Self-Hosting

Auth Self-hosting Config

==========================

In a self-hosted environment, you do not have access to the Auth configuration such as third party OAuth provider settings through the Supabase dashboard. Instead, you configure them through the `docker-compose.yml` file. You can read more about the configuration in the Self-hosting guide.

You can find the complete list of available configuration parameters on the README of the Auth repository.

General
-------

General settings.

##### Parameters

GOTRUE\_SITE\_URL

REQUIRED

no type

The base URL of your website. Used as an allow-list for redirects and for constructing URLs used in emails.

GOTRUE\_EXTERNAL\_GITHUB\_ENABLED

REQUIRED

no type

Whether the external provider, GitHub in this case, is enabled or not.

GOTRUE\_EXTERNAL\_GITHUB\_CLIENT\_ID

REQUIRED

no type

The OAuth2 Client ID registered with the external provider.

GOTRUE\_EXTERNAL\_GITHUB\_SECRET

REQUIRED

no type

The OAuth2 Client Secret provided by the external provider when you registered.

GOTRUE\_EXTERNAL\_GITHUB\_REDIRECT\_URI

REQUIRED

no type

The URI a OAuth2 provider will redirect to with the `code` and `state` values.

GOTRUE\_URI\_ALLOW\_LIST

REQUIRED

no type

A comma separated list of URIs (e.g. `"https://foo.example.com,https://*.foo.example.com,https://bar.example.com"`) which are permitted as valid `redirect_to` destinations.

GOTRUE\_JWT\_EXP

REQUIRED

no type

How long tokens are valid for, in seconds. Defaults to 3600 (1 hour), maximum 604,800 seconds (one week).

GOTRUE\_DISABLE\_SIGNUP

REQUIRED

no type

Allow/disallow new user signups to your project.

GOTRUE\_EXTERNAL\_EMAIL\_ENABLED

REQUIRED

no type

Allow/disallow new user signups via email to your project.

GOTRUE\_MAILER\_SECURE\_EMAIL\_CHANGE\_ENABLED

REQUIRED

no type

If enabled, a user will be required to confirm any email change on both the old, and new email addresses. If disabled, only the new email is required to confirm.

GOTRUE\_MAILER\_AUTOCONFIRM

REQUIRED

no type

If enabled, users need to confirm their email address before signing in.

### Is this helpful?

No Yes