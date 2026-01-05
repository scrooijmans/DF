It turned out to be quite easy. Other answers were misleading, talking about copying data and stuff, but if you are just starting and don't already have data, none of that is necessary. You just need to create a directory to a folder called postgres-data and add a line with /<path to location>/postgres-data:/var/lib/postgresql/data under db volumes. Now the data stays the same whenever you use docker compose down/up. The first time you start it this way, the necessary files are automatically created in postgres-data and you are done. Side note... I recommend making the directory outside of your Supabase folder, so you can easily use it on updated versions instead of copying/moving data.

Here is what you need to do in docker-compose.yml:

  db:
    container_name: supabase-db
    image: supabase/postgres:14.1.0.89
    healthcheck:
      test: pg_isready -U postgres -h localhost
      interval: 5s
      timeout: 5s
      retries: 10
    command:
      - postgres
      - -c
      - config_file=/etc/postgresql/postgresql.conf
      - -c
      - log_min_messages=fatal # prevents Realtime polling queries from appearing in logs
    restart: unless-stopped
    ports:
      - ${POSTGRES_PORT}:5432
    environment:
      POSTGRES_HOST: /var/run/postgresql
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - ./volumes/db/roles.sql:/docker-entrypoint-initdb.d/roles.sql
      - /<path to location>/postgres-data:/var/lib/postgresql/data ### This is the added line