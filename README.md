# sqlc - Compile SQL to Type-Checked C++

Proof-of-concept for a compiler that translates SQL queries to type-checked C++ code.

## Motivation
- Performance: build ML pipelines than run faster than those written in Python with Pandas ([DataFrame performance](https://github.com/hosseinmoein/DataFrame?tab=readme-ov-file#performance)).
- Security: no need for a SQL runtime, just run a single-purpose C++ program that can be audited.
- Integration: run SQL on (embedded) systems that don't support a heavy DBMS but can handle self-contained C++ programs.

## Technical Details
- SQL parser based on [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).
- Generated C++ code uses [DataFrame](https://github.com/hosseinmoein/DataFrame)

## Installation
```sh
git clone https://github.com/mo42/sqlc.git
cd sqlc
cargo build --release
```

## Example

Compiling the example SQL file:
```sh
cargo run -- example.sql > example.cpp
```

```sql
SELECT
  date,
  column2
FROM
  'example.csv'
  JOIN 'join.csv' USING(column2)
WHERE
  joined_string = "Join string 3"
ORDER BY date ASC, column2 DESC
```

```cpp
#include <DataFrame/DataFrame.h>

#include <iostream>
using namespace hmdf;
typedef ulong idx_t;
using SqlcDataFrame = StdDataFrame<idx_t>;
int main(int, char**) {
  SqlcDataFrame df_main;
  df_main.read("example.csv", io_format::csv2);
  SqlcDataFrame df_join0;
  df_join0.read("join.csv", io_format::csv2);
  SqlcDataFrame df = df_main.join_by_column<decltype(df_join0), int, double,
                                            std::string, int, long, ulong>(
      df_join0, "column2", hmdf::join_policy::inner_join);
  auto where_functor = [](const idx_t&,
                          const std::string& joined_string) -> bool {
    return (joined_string == "Join string 3");
  };
  auto where_df = df.get_data_by_sel<std::string, decltype(where_functor),
                                     double, std::string, int, long, ulong>(
      "joined_string", where_functor);
  where_df.sort<std::string, int, double, std::string, int, long, ulong>(
      "date", sort_spec::ascen, "column2", sort_spec::desce);
  std::vector<idx_t> idx = where_df.get_index();
  std::vector<std::string> date = where_df.get_column<std::string>("date");
  std::vector<int> column2 = where_df.get_column<int>("column2");
  SqlcDataFrame select;
  select.load_index(std::move(idx));
  select.load_column("date", std::move(date));
  select.load_column("column2", std::move(column2));
  select.write<std::ostream, std::string, int>(std::cout, hmdf::io_format::csv,
                                               5, false, 100);
  return 0;
}
```

## Documentation

Order of execution of SQL statements:
1. `FROM` and `JOIN`
2. `WHERE`
3. `GROUP BY`
4. `HAVING`
5. `SELECT` (`2 * col1 AS col_new1` and window functions)
6. `ORDER BY`
7. `LIMIT`
