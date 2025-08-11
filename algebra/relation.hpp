#pragma once
#include "types.hpp"
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace algebra {

class Relation {
public:
  using Column = std::vector<Value>;

  std::map<std::string, size_t> column_names;
  std::vector<Column> columns;

  Relation() = default;

  size_t rows() const { return columns.empty() ? 0 : columns[0].size(); }

  static std::string to_string(const Value& val) {
    return std::visit(
        [](auto&& v) -> std::string {
          std::ostringstream oss;
          if constexpr (std::is_same_v<std::decay_t<decltype(v)>, bool>)
            oss << (v ? "true" : "false");
          else
            oss << v;
          return oss.str();
        },
        val);
  }

  void print(std::ostream& os = std::cout) const {
    if (columns.empty() || column_names.empty()) {
      os << "(empty relation)\n";
      return;
    }

    size_t num_columns = column_names.size();
    size_t num_rows = rows();

    // Create a vector of (index, name) pairs sorted by index
    std::vector<std::pair<int, std::string>> index_to_name;
    index_to_name.reserve(num_columns);
    for (const auto& [name, idx] : column_names) {
      index_to_name.emplace_back(idx, name);
    }
    std::sort(index_to_name.begin(), index_to_name.end(),
              [](const auto& a, const auto& b) { return a.first < b.first; });

    // Determine column widths
    std::vector<size_t> widths(num_columns);
    for (const auto& [idx, name] : index_to_name) {
      widths[idx] = name.size();
      for (const auto& val : columns[idx]) {
        std::string str = to_string(val);
        widths[idx] = std::max(widths[idx], str.size());
      }
    }

    // Print column headers
    for (size_t i = 0; i < num_columns; ++i) {
      os << std::left << std::setw(widths[i]) << index_to_name[i].second;
      if (i != num_columns - 1)
        os << " | ";
    }
    os << '\n';

    // Print separator
    for (size_t i = 0; i < num_columns; ++i) {
      os << std::string(widths[i], '-') << (i != num_columns - 1 ? "-+-" : "");
    }
    os << '\n';

    // Print rows
    for (size_t r = 0; r < num_rows; ++r) {
      for (size_t c = 0; c < num_columns; ++c) {
        int col_index = index_to_name[c].first;
        os << std::left << std::setw(widths[col_index])
           << to_string(columns[col_index][r]);
        if (c != num_columns - 1)
          os << " | ";
      }
      os << '\n';
    }
  }
};

} // namespace algebra
