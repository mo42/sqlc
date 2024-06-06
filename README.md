# sqlc
POC: Compile SQL to type-checked C++.

- SQL parser based on [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).
- Generated C++ code uses [DataFrame](https://github.com/hosseinmoein/DataFrame)

Currently supported (selection and projection):
```sql
SELECT
  column_1,
  column_2,
  ...
  column_n,
FROM
  file.csv
WHERE
  column_k = value_1 AND column_l = value_2
```
