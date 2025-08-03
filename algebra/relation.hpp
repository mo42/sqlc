#pragma once
#include "types.hpp"
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <sstream>
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

    // Determine column widths
    std::vector<size_t> widths(num_columns);
    for (size_t i = 0; i < num_columns; ++i) {
      widths[i] = column_names[i].size();
      for (const auto& val : columns[i]) {
        std::string str = to_string(val);
        widths[i] = std::max(widths[i], str.size());
      }
    }

    // Print column headers
    for (size_t i = 0; i < num_columns; ++i) {
      os << std::left << std::setw(widths[i]) << column_names[i];
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
        os << std::left << std::setw(widths[c]) << to_string(columns[c][r]);
        if (c != num_columns - 1)
          os << " | ";
      }
      os << '\n';
    }
  }
};

} // namespace algebra
