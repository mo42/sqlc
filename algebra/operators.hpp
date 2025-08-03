#pragma once
#include "relation.hpp"
#include <functional>
#include <stdexcept>

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

Relation project(const Relation& r, const std::vector<std::string>& cols) {
  Relation result;
  std::vector<size_t> indices;

  for (const auto& col : cols) {
    auto it = std::find(r.column_names.begin(), r.column_names.end(), col);
    if (it == r.column_names.end()) {
      throw std::runtime_error("Column not found in projection: " + col);
    }
    indices.push_back(std::distance(r.column_names.begin(), it));
    result.column_names.push_back(col);
  }

  for (size_t idx : indices) {
    result.columns.push_back(r.columns[idx]);
  }

  return result;
}

Relation rename(const Relation& r,
                const std::unordered_map<std::string, std::string>& renames) {
  Relation result = r;
  for (size_t i = 0; i < result.column_names.size(); ++i) {
    const auto& old_name = result.column_names[i];
    auto it = renames.find(old_name);
    if (it != renames.end()) {
      result.column_names[i] = it->second;
    }
  }
  return result;
}

} // namespace algebra
