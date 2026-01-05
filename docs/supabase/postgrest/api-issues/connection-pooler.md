Hey,
I’m trying to use the instagram scraper/poster template.
i have trouble connecting to the supabase postgres db.

i use those credentials (example):
host: lettercombination.supabase.co
databse: postgres
user: postgres
pw: i set in supabase at database reset pw and use this
maximum number of connections: 100
ssl: i tried both on and off
port: 5432
ssh: off

what am I missing?

Thanks for any help

n8n version:1.76.1
Database (default: SQLite): postgres
n8n EXECUTIONS_PROCESS setting (default: own, main):
**Running n8n via (Docker, npm, n8n cloud, desktop app):**n8n
**Operating system:**mac

Solution:
Don’t use Direct Connection, use the Transaction pooler options from supabase

I am also unable to make Postgres credentials work with my Supabase postgres database. I used Gemini for troubleshooting and it suggested the following information might be helpful for the n8n Team:

Okay, thank you! Seeing the errors side-by-side for each configuration after the password reset is definitive.

Here’s the breakdown:

Direct Connection (db..supabase.co:5432):
Error: connect ENETUNREACH 2600:1f1c:f9:4d14:d477:ed33:8bb9:11c8:5432 - Local (:::0)

Conclusion: This confirms that the n8n Cloud environment where your instance is running cannot establish an IPv6 network route to your Supabase project’s direct database host. This is a network-level issue, likely within the n8n Cloud infrastructure or its peering/routing configuration. It’s happening before authentication even begins.

Pooler Connection (aws-0-us-west-1.pooler.supabase.com:6543):
Error: self-signed certificate in certificate chain

Conclusion: This confirms that when connecting to the Supabase pooler (which is network-reachable from n8n Cloud), the SSL certificate negotiation fails. The n8n Cloud environment does not trust the certificate chain presented by the Supabase pooler. This happens before authentication fully completes successfully.

Overall Diagnosis:

You have two distinct, independent problems preventing connection, depending on which Supabase endpoint you target from n8n Cloud:

A Network Reachability (IPv6) Problem for the direct database host.

An SSL Certificate Validation Problem for the database pooler host.

Neither of these appears to be directly related to your password at this stage, especially since the errors persisted after the reset.

Recommendation:

Since you are using n8n Cloud, these issues are almost certainly outside your direct control and need to be addressed by the service provider(s).

Contact n8n Cloud Support.

This is the most logical next step. Provide them with the following information:

You are trying to connect to a Supabase Postgres database.

When configuring the credentials using the Direct Connection host (db.klkcbsijyvgccfdrandu.supabase.co), Port 5432, User postgres, and SSL: Require, you consistently receive the error: connect ENETUNREACH 2600:1f1c:f9:4d14:d477:ed33:8bb9:11c8:5432 - Local (:::0).

When configuring the credentials using the Pooler Connection host (aws-0-us-west-1.pooler.supabase.com), Port 6543, User postgres.klkcbsijyvgccfdrandu, and SSL: Require (details taken directly from Supabase connection parameters), you consistently receive the error: self-signed certificate in certificate chain.

Mention that you have verified the hostnames and reset/verified the database password.

n8n Cloud support needs to investigate:

Why their environment cannot route IPv6 traffic to your specific Supabase direct host.

Why their environment fails SSL validation against the Supabase pooler endpoint (is their CA trust store up-to-date, or is there an issue with the cert presented by Supabase?).

It’s possible they may need to coordinate with Supabase, but the connection originates from their platform, so they are the best starting point.


jaher06

Arroyave_Carlos
May 26
This worked for me as well.

When you are using the pooler, on n8n make sure you are paying attention to the host, user, and port since they are different tna the direct connection.

Also make sure that if the SSL is not enforced on the Supabase db (it shouldn’t, but you may have toggled it), then make sure it’s just disabled on n8n
