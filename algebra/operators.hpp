#pragma once
#include "relation.hpp"
#include <functional>
#include <stdexcept>

namespace algebra {

template <typename T>
inline T col(const Relation& r, const std::vector<Value>& row,
             const std::string& name) {
  return std::get<T>(row[r.column_names.at(name)]);
}

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
    auto it = r.column_names.find(col);
    if (it == r.column_names.end()) {
      throw std::runtime_error("Column not found in projection: " + col);
    }
    indices.push_back(it->second);
    result.column_names[col] = static_cast<size_t>(result.column_names.size());
  }

  for (size_t idx : indices) {
    result.columns.push_back(r.columns[idx]);
  }

  return result;
}

Relation rename(const Relation& r,
                const std::unordered_map<std::string, std::string>& renames) {
  Relation result;
  result.columns = r.columns;

  // Rebuild column_names map with renamed keys
  for (const auto& [old_name, idx] : r.column_names) {
    auto it = renames.find(old_name);
    if (it != renames.end()) {
      result.column_names[it->second] = idx;
    } else {
      result.column_names[old_name] = idx;
    }
  }

  return result;
}

Relation limit(const Relation& rel, size_t n) {
    Relation result;
    result.column_names = rel.column_names;
    size_t rows_to_keep = std::min(n, rel.rows());
    result.columns.reserve(rel.columns.size());
    for (const auto& col : rel.columns) {
        result.columns.emplace_back(col.begin(), col.begin() + rows_to_keep);
    }
    return result;
}

} // namespace algebra
