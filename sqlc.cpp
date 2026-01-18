#include <iostream>
#include <fstream>
#include <string>
#include <sstream>

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input_sql_file> [output_cpp_file]" << std::endl;
        return 1;
    }

    std::string input_path = argv[1];
    std::string output_path = (argc >= 3) ? argv[2] : "output.cpp";

    std::ifstream sql_file(input_path);
    if (!sql_file.is_open()) {
        std::cerr << "Error: Could not open " << input_path << std::endl;
        return 1;
    }

    std::stringstream buffer;
    buffer << sql_file.rdbuf();
    std::string raw_sql = buffer.str();

    std::ofstream out_file(output_path);
    if (!out_file.is_open()) {
        std::cerr << "Error: Could not create " << output_path << std::endl;
        return 1;
    }

    out_file << R"cpp_code(
#include <duckdb.hpp>
#include <iostream>
#include <iomanip>

int main() {
    duckdb::DuckDB db(nullptr);
    duckdb::Connection con(db);

    std::string sql_query = R"sql_delimiter()cpp_code";

    out_file << raw_sql;
    out_file << R"cpp_code()sql_delimiter";
    auto result = con.Query(sql_query);

    if (result->HasError()) {
        std::cerr << "SQL Error: " << result->GetError() << std::endl;
        return 1;
    }
    auto &collection = result->Collection();
    for (auto &row : collection.GetRows()) {
        for (size_t i = 0; i < collection.ColumnCount(); i++) {
            std::cout << row.GetValue(i).ToString() << "\t";
        }
        std::cout << "\n";
    }

    return 0;
}
)cpp_code";
    return 0;
}
