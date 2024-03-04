# Caching Server For Kubes and Recipes

## Running

```bash
go build
./kube_cache
```

## Dotenv setup

```dotenv
PGHOST=don't put secrets in main
PGUSER=don't put secrets in main
PGPORT=5432
PGDATABASE=don't put secrets in main
PGPASSWORD=don't put secrets in main

DATABASE_URL=$PGUSER:$PGPASSWORD@tcp($PGHOST:$PGPORT)/$PGDATABASE?sslmode=require
DATABASE_URL=host=$PGHOST user=$PGUSER password=$PGPASSWORD dbname=$PGDATABASE port=$PGPORT sslmode=require TimeZone=Europe/London

OPENAI_KEY=don't put secrets in main
OPENAI_ENDPOINT=https://don't put secrets in main.openai.azure.com/
OPENAI_MODEL_ID=GPTthreeeee
```
