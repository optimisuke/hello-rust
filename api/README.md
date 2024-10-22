# api

## sqlx

Type Overrides: Output Columns
Type overrides are also available for output columns, utilizing the SQL standard’s support for arbitrary text in column names:

Force Not-Null
Selecting a column foo as "foo!" (Postgres / SQLite) or foo as `foo!` (MySQL) overrides inferred nullability and forces the column to be treated as NOT NULL; this is useful e.g. for selecting expressions in Postgres where we cannot infer nullability:

https://docs.rs/sqlx/latest/sqlx/macro.query.html#nullability-output-columns

## env

```.env
DATABASE_URL=sqlite://todos.db
```
