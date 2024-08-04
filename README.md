# sqlc
POC: Compile SQL to type-checked C++.

- SQL parser based on [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).
- Generated C++ code uses [DataFrame](https://github.com/hosseinmoein/DataFrame)

## Example

Compiling this SQL statement
```sql
SELECT
  date,
  column2
FROM
  'example.csv'
  JOIN 'join.csv' USING(column2)
WHERE
  joined_string = "Join string 3"
```
yields C++ code that implements the semantics using DataFrame and can be
compiled into an executable.
