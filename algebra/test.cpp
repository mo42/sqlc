#include "csv_reader.hpp"
#include "operators.hpp"
#include "relation.hpp"

using namespace algebra;

int main() {
  Relation r = load_csv("data.csv");

  Relation rr = select(r, [](const std::vector<Value>& row) {
    return std::get<int64_t>(row[0]) > 100;
  });
}
