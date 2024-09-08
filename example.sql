SELECT
  date AS date_alias,
  column2
FROM
  'example.csv'
  JOIN 'join.csv' USING(column2)
WHERE
  joined_string = "Join string 3"
ORDER BY date_alias ASC, column2 DESC
LIMIT 2
