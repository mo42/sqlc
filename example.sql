SELECT
  date,
  column2
FROM
  'example.csv'
  JOIN 'join.csv' USING(column2)
WHERE
  joined_string = "Join string 3"
