#pragma once
#include "types.hpp"
#include <algorithm>
#include <stdexcept>
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
};

} // namespace algebra
