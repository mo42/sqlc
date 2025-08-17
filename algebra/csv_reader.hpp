#pragma once
#include "relation.hpp"
#include <fstream>
#include <sstream>
#include <stdexcept>

namespace algebra {

Relation load_csv(const std::string& filename, char delimiter = ',') {
  Relation rel;
  std::ifstream file(filename);
  std::string line;

  if (!std::getline(file, line))
    throw std::runtime_error("Empty file");

  std::stringstream ss(line);
  std::string col;
  size_t i = 0;
  while (std::getline(ss, col, delimiter)) {
    rel.column_names[col] = i++;
  }

  size_t ncols = rel.column_names.size();
  std::vector<Relation::Column> cols(ncols);

  while (std::getline(file, line)) {
    std::stringstream rowss(line);
    std::string cell;
    for (size_t i = 0; i < ncols; ++i) {
      if (!std::getline(rowss, cell, delimiter)) {
        break;
      }
      /* TODO All column types are std::string
      if (cell == "true" || cell == "false")
        cols[i].emplace_back(cell == "true");
      else if (cell.find('.') != std::string::npos)
        cols[i].emplace_back(std::stod(cell));
      else {
        try {
          cols[i].emplace_back(std::stoll(cell));
        } catch (...) {
          cols[i].emplace_back(cell);
        }
      }
      */
      cols[i].emplace_back(cell);
    }
  }

  for (size_t i = 0; i < ncols; ++i)
    rel.columns.push_back(std::move(cols[i]));

  return rel;
}

} // namespace algebra
