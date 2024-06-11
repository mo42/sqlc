# sqlc
POC: Compile SQL to type-checked C++.

- SQL parser based on [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).
- Generated C++ code uses [DataFrame](https://github.com/hosseinmoein/DataFrame)

## Example

Compiling this SQL statement
```sql
SELECT date, column2 FROM 'example.csv' WHERE column3 = 1089200 OR column2 = 3
```
yields this C++ code

```cpp
#include <DataFrame/DataFrame.h>

#include <iostream>

using namespace hmdf;

typedef ulong idx_t;

using SqlcDataFrame = StdDataFrame<idx_t>;

int main(int, char**) {
  SqlcDataFrame df;
  df.read("example.csv", io_format::csv2);
  auto where_functor = [](const idx_t&, const long& column3,
                          const int& column2) -> bool {
    return ((column3 == 1089200) || (column2 == 3));
  };
  auto where_df = df.get_data_by_sel<long, int, decltype(where_functor), int,
                                     std::string, long, double, ulong>(
      "column3", "column2", where_functor);
  std::vector<idx_t> idx = where_df.get_index();
  std::vector<std::string> date = where_df.get_column<std::string>("date");
  std::vector<int> column2 = where_df.get_column<int>("column2");
  SqlcDataFrame select;
  select.load_index(std::move(idx));
  select.load_column("date", std::move(date));
  select.load_column("column2", std::move(column2));
  std::cout << select.to_string<double>() << std::endl;
  return 0;
}
```
