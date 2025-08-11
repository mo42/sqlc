#include "csv_reader.hpp"
#include "operators.hpp"
#include "relation.hpp"

using namespace algebra;

int main() {
  Relation r = load_csv("data.csv");
  std::cout << std::endl << "Relation r:" << std::endl;
  r.print();
  Relation rr = select(r, [r](const std::vector<Value>& row) {
    return col<int64_t>(r, row, "a") >= 100;
  });
  std::cout << std::endl << "Relation rr:" << std::endl;
  rr.print();
  Relation rrr = rename(rr, {{"d", "dd"}});
  std::cout << std::endl << "Relation rrr:" << std::endl;
  rrr.print();
  Relation rrrr = project(rrr, {"a", "b"});
  std::cout << std::endl << "Relation rrrr:" << std::endl;
  rrrr.print();
}
