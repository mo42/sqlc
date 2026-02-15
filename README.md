# dag

`dag` is a lightweight SQL build orchestrator.

It analyzes dependencies between SQL files, detects cycles, determines a
correct execution order, and then builds and runs each query using an existing
SQL-to-C++ compiler (`sqlc`).

## Motivation

An ML pipeline can consist of many preparatory SQL queries with complex
transformations and dependencies. For example:

- `source.sql` produces `source.csv`
- `transform.sql` references `source.csv`
- `final.sql` reads `transform.csv`

Manually managing execution order quickly becomes error-prone.

`dag` solves this by scanning for SQL files, inferring dependencies, detecting
cycles, compiling and executing queries in the correct order.

## Usage

```sh
./dag
Usage:
  dag compile <dir>
  dag run <dir>
```

## Example

```sh
./dag compile test-simple-number
[compile] numbers
[compile] squares
[compile] summary
./dag run test-simple-number
[run] numbers
[run] squares
[run] summary
```
