#pragma once
#include "relation.hpp"
#include <functional>

namespace algebra {

Relation select(const Relation& r,
                std::function<bool(const std::vector<Value>&)> predicate) {
  Relation result;
  result.column_names = r.column_names;
  result.columns.resize(r.columns.size());

  for (size_t i = 0; i < r.rows(); ++i) {
    std::vector<Value> row;
    for (const auto& col : r.columns)
      row.push_back(col[i]);

    if (predicate(row)) {
      for (size_t j = 0; j < r.columns.size(); ++j)
        result.columns[j].push_back(r.columns[j][i]);
    }
  }

  return result;
}
} // namespace algebra
