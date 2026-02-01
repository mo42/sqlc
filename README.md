# sqlbuild

`sqlbuild` is a lightweight SQL build orchestrator.

It analyzes dependencies between SQL files, detects cycles, determines a
correct execution order, and then builds and runs each query using an existing
SQL-to-C++ compiler (`sqlc`).

## Motivation

A ML pipeline can consist of many SQL queries with complex transformations and dependencies. For example:

- `source.sql` produces `source.csv`
- `transform.sql` references `source.csv`
- `final.sql` reads `transform.csv`

Manually managing execution order quickly becomes error-prone.

`sqlbuild` solves this by scanning for SQL files, inferring dependencies, detecting cycles, compiling and executing queries in the correct order.
