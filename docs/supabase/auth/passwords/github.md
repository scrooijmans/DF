Hi All,

I recently had to go through this in a dev environment and this is what I had to do in order to correct a postgres password change, huge shout out to @jpampa-dev for his excellent SQL Snippet.

I will show the step by step instructions I did using PSQL

Using PSQL:
Step 1) Reset your password to what it was before this attempted change, for reference, the default .env password is:
your-super-secret-and-long-postgres-password
Step 2) Ensure your self-hosted supabase containers are running and connect to your docker supabase-db instance
sudo docker exec -it supabase-db /bin/bash
Step 3) Connect to the _supabase database in your docker instance using the supabase_admin user (if prompted, the password is the .env password)
psql -U supabase_admin -d _supabase
Step 4) Paste @jpampa-dev 's SQL Snippet, replacing $new_passwd with the new password you plan to use

alter user anon with password '$new_passwd';
alter user authenticated with password '$new_passwd';
alter user authenticator with password '$new_passwd';
alter user dashboard_user with password '$new_passwd';
alter user pgbouncer with password '$new_passwd';
alter user pgsodium_keyholder with password '$new_passwd';
alter user pgsodium_keyiduser with password '$new_passwd';
alter user pgsodium_keymaker with password '$new_passwd';
alter user postgres with password '$new_passwd';
alter user service_role with password '$new_passwd';
alter user supabase_admin with password '$new_passwd';
alter user supabase_auth_admin with password '$new_passwd';
alter user supabase_functions_admin with password '$new_passwd';
alter user supabase_read_only_user with password '$new_passwd';
alter user supabase_replication_admin with password '$new_passwd';
alter user supabase_storage_admin with password '$new_passwd';

UPDATE _analytics.source_backends
SET config = jsonb_set(config, '{url}', '"postgresql://supabase_admin:$new_passwd@db:5432/postgres"', 'false')
WHERE type='postgres';
Step 5) Shutdown the Self-Hosted Supabase Docker Containers (see above)
Step 6) Change .env's POSTGRES_PASSWORD variable to the new password you plan to use.
Step 7) Start the Self-Hosted Supabase Docker Containers

********************
Cannot Change Dashboard password on local
Hi and thanks for your time reading this.

I installed Supabase with Docker locally. Run normally. Then i tried to change my dashboard password in .env by follow this instruction on their website. But after changing, nothing work. The system still require to use old pass word. Do you have any idea why this happen
https://supabase.com/docs/guides/self-hosting/docker#dashboard-authentication
"

You can access the Supabase Dashboard through the API gateway on port 8000
. For example: http://<your-ip>:8000
, or localhost:8000 if you are running Docker locally.

You will be prompted for a username and password. By default, the credentials are:

Username: supabase

Password: this_password_is_insecure_and_should_be_updated

You should change these credentials as soon as possible using the instructions below."


Upvote
4

Downvote

2
Go to comments


Share

Report
Report
u/google avatar
google
•
Promoted

Gemini in Android Studio predicts as you type, so you can modify, optimize and add code. Download to start using Gemini to help your productivity.
Download
developer.android.com
Thumbnail image: Gemini in Android Studio predicts as you type, so you can modify, optimize and add code. Download to start using Gemini to help your productivity.
Join the conversation
Sort by:

Best

Search Comments
Expand comment search
Comments Section
Timmedy
•
1y ago
I just did this a couple of days ago, dont ask me why, but you cant use all special characters in the password. I tried a couple from password generators and none of them worked. Made my own with .()- and it worked just fine.

I also had issues with the postgres password, make sure thats the first thing you change and never touch it again, or you will have some services in a restarting loop (someone correct me if there is a fix for this, i couldnt find one)

****************************************


Improve documentation
Topic: Troubleshooting PostgreSQL Password Changes in Docker Compose
Problem Description: "Password Authentication Failed" After Updating .env
When developing locally with Docker Compose, you might encounter an error like FATAL: password authentication failed for user "postgres" after changing your PostgreSQL password (e.g., POSTGRES_PASSWORD in your .env file) and then running docker compose up -d.

Why this happens:
PostgreSQL containers, when started for the first time, read environment variables like POSTGRES_PASSWORD to initialize the database and set the user's password. This database data, including the password, is then stored persistently in a Docker volume.

If you later change the password in your .env file, the already existing database instance in the volume will still be using the old password. docker compose up -d will restart the container with the new environment variable, but it won't automatically update the password within the pre-existing database. This mismatch causes the authentication error.

Solution: Guidelines for Changing PostgreSQL Passwords
To successfully update your PostgreSQL password in your local Docker Compose environment, choose one of the following methods:

Method 1: Resetting Your Local Database (Recommended for Development)

This is the simplest way to update credentials and is ideal if your local database data is not critical and can be easily re-seeded or is just for temporary testing. This method will remove all your existing data from the PostgreSQL database.

Stop and Remove All Services and Volumes:

Bash

docker compose down --volumes
This command stops all services and, importantly, removes the persistent data volume associated with PostgreSQL, deleting the old database data (and its old password).

Update Password in .env:
Open your project's .env file and change the POSTGRES_PASSWORD (and any other relevant POSTGRES_USER, POSTGRES_DB variables if you're also changing those) to your desired new value.

Restart Docker Compose:

Bash

docker compose up -d
The PostgreSQL container will now re-initialize with the new password from your updated .env file.

Method 2: Changing Password Without Losing Existing Data (Advanced)

Use this method if you have critical local development data in your PostgreSQL instance that you cannot afford to lose. This involves manually updating the password directly within the running database.

Ensure Current Password is Set in .env:
Make sure your .env file currently contains the old password that the running PostgreSQL database is still using. You'll need this to connect initially.

Start Docker Compose Services (if not running):

Bash

docker compose up -d
Access the PostgreSQL Container Shell:
First, find the name of your PostgreSQL container (e.g., using docker ps). Then, execute a shell inside it:

Bash

docker exec -it bash
(Replace with the actual name, e.g., supabase_db_1)

Connect to PostgreSQL CLI (psql):
Once inside the container, connect to the database. You'll typically use the postgres user and database, or supabase_admin for Supabase. You'll be prompted for the old password.

Bash

psql -U -d <database_name>
(e.g., psql -U postgres -d postgres or psql -U supabase_admin -d postgres)

Change the User's Password via SQL:
Execute the following SQL command, replacing and 'your_new_password_here' with your specific details:

SQL

ALTER USER WITH PASSWORD 'your_new_password_here';
-- Example for the default 'postgres' user:
-- ALTER USER postgres WITH PASSWORD 'my_super_new_secure_password';
Exit psql and Container Shell:

SQL

\q
exit
Update .env with New Password:
Now, open your .env file and update POSTGRES_PASSWORD (and any other relevant variables) to match the new password you just set via SQL.

Restart Services:

Bash

docker compose restart
Your services should now connect successfully using the newly updated password.

I hope this helps you and works ,feel free to connect me @CoderNasim2023 for any help, i would love to help you on this @InfoSecMastermind