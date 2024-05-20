# sqlc
POC: Compile SQL to C++

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
