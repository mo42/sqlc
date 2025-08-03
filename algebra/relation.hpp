#pragma once
#include "types.hpp"
#include <stdexcept>
#include <algorithm>
#include <string>
#include <unordered_map>
#include <vector>

namespace algebra {

class Relation {
public:
  using Column = std::vector<Value>;

  std::vector<std::string> column_names;
  std::vector<Column> columns;

  Relation() = default;

  size_t rows() const { return columns.empty() ? 0 : columns[0].size(); }

  void add_column(const std::string& name, Column col) {
    column_names.push_back(name);
    columns.push_back(std::move(col));
  }

  size_t col_index(const std::string& name) const {
    auto it = std::find(column_names.begin(), column_names.end(), name);
    if (it == column_names.end())
      throw std::runtime_error("Column not found");
    return std::distance(column_names.begin(), it);
  }

  const Value& at(size_t row, size_t col) const { return columns[col][row]; }
};

} // namespace algebra
