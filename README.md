# actix-sqlx-pg

Example Actix application using SQLX for PostgreSQL

## About

This is a minimal Actix web application configured to use SQLX for making typesafe queries,
without relying on an ORM like [Diesel](https://diesel.rs/).

The included `docker-compose.yaml` is for provisioning a local PostgreSQL instance
if you do not want to pre-provision a Railway database, or if you would like to
avoid using your database provisioned on Railway as a development database.

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/4lz789?referralCode=vMAgAq)
