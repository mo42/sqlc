SELECT
  date,
  column2
FROM
  'example.csv'
  JOIN 'join.csv' USING(column2)
WHERE
  column3 = 1089200 OR column2 = 3
