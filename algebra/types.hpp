#pragma once
#include <string>
#include <variant>
#include <vector>

namespace algebra {

using Value = std::variant<int64_t, double, std::string, bool>;

enum class ColumnType { Int, Double, String, Bool };

inline std::string to_string(const Value& v) {
  return std::visit(
      [](auto&& val) -> std::string {
        using T = std::decay_t<decltype(val)>;
        if constexpr (std::is_same_v<T, std::string>)
          return val;
        else if constexpr (std::is_same_v<T, bool>)
          return val ? "true" : "false";
        else
          return std::to_string(val);
      },
      v);
}

} // namespace algebra
