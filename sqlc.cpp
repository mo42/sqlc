#include <iostream>
#include <fstream>
#include <string>
#include <sstream>

#include <duckdb.hpp>

static std::string replace_extension(const std::string &path,
                                     const std::string &new_ext) {
    auto pos = path.find_last_of('.');
    if (pos == std::string::npos) {
        return path + new_ext;
    }
    return path.substr(0, pos) + new_ext;
}

static void replace_all(std::string &str,
                        const std::string &from,
                        const std::string &to) {
    size_t pos = 0;
    while ((pos = str.find(from, pos)) != std::string::npos) {
        str.replace(pos, from.length(), to);
        pos += to.length();
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input_sql_file>" << std::endl;
        return 1;
    }

    std::string input_path = argv[1];
    std::string output_path = replace_extension(input_path, ".cpp");
    std::string csv_path = replace_extension(input_path, ".csv");

    std::ifstream sql_file(input_path);
    if (!sql_file.is_open()) {
        std::cerr << "Error: Could not open " << input_path << std::endl;
        return 1;
    }

    std::stringstream buffer;
    buffer << sql_file.rdbuf();
    std::string raw_sql = buffer.str();

    {
        duckdb::DuckDB db(nullptr);
        duckdb::Connection con(db);

        auto prep = con.Prepare(raw_sql);
        if (!prep->success) {
            std::cerr << "SQL syntax error in " << input_path << ":\n"
                      << prep->GetError() << std::endl;
            return 1;
        }
    }

    std::string cpp_template = R"CPP(
#include <duckdb.hpp>
#include <iostream>

int main() {
    duckdb::DuckDB db(nullptr);
    duckdb::Connection con(db);

    std::string sql_query = R"SQL(
{{SQL}}
)SQL";

    std::string copy_sql =
        "COPY (" + sql_query + ") TO '{{CSV_PATH}}' "
        "(FORMAT CSV, HEADER TRUE);";

    auto result = con.Query(copy_sql);

    if (result->HasError()) {
        std::cerr << "SQL Error: " << result->GetError() << std::endl;
        return 1;
    }
    return 0;
}
)CPP";

    replace_all(cpp_template, "{{SQL}}", raw_sql);
    replace_all(cpp_template, "{{CSV_PATH}}", csv_path);

    std::ofstream out_file(output_path);
    if (!out_file.is_open()) {
        std::cerr << "Error: Could not create " << output_path << std::endl;
        return 1;
    }

    out_file << cpp_template;
    return 0;
}
