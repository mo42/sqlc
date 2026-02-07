#include <duckdb.hpp>

#include <filesystem>
#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <cstdlib>

namespace fs = std::filesystem;

std::string read_file(const fs::path &p) {
    std::ifstream in(p);
    return std::string((std::istreambuf_iterator<char>(in)),
                        std::istreambuf_iterator<char>());
}

static std::string replace_extension(const std::string &path,
                                     const std::string &ext) {
    auto pos = path.find_last_of('.');
    if (pos == std::string::npos)
        return path + ext;
    return path.substr(0, pos) + ext;
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

bool compile_sql_to_cpp(const fs::path &sql_path) {
    std::string raw_sql = read_file(sql_path);

    {
        duckdb::DuckDB db(nullptr);
        duckdb::Connection con(db);
        auto prep = con.Prepare(raw_sql);
        if (!prep->success) {
            std::cerr << "SQL syntax error in " << sql_path << "\n"
                      << prep->GetError() << "\n";
            return false;
        }
    }

    std::string output_cpp = replace_extension(sql_path.string(), ".cpp");
    std::string output_csv = replace_extension(sql_path.string(), ".csv");

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
        std::cerr << result->GetError() << std::endl;
        return 1;
    }
    return 0;
}
)CPP";

    replace_all(cpp_template, "{{SQL}}", raw_sql);
    replace_all(cpp_template, "{{CSV_PATH}}", output_csv);

    std::ofstream out(output_cpp);
    out << cpp_template;

    return true;
}

std::unordered_set<std::string>
extract_csv_dependencies(const std::string &sql) {
    std::unordered_set<std::string> deps;
    std::regex csv(R"(['"]([^'"]+)\.csv['"])");

    for (auto it = std::sregex_iterator(sql.begin(), sql.end(), csv);
         it != std::sregex_iterator(); ++it) {
        deps.insert((*it)[1]);
    }
    return deps;
}

enum class State { UNVISITED, VISITING, VISITED };

bool dfs(const std::string &node,
         const std::unordered_map<std::string,
             std::vector<std::string>> &graph,
         std::unordered_map<std::string, State> &state,
         std::vector<std::string> &order) {

    if (state[node] == State::VISITING)
        return false;

    if (state[node] == State::VISITED)
        return true;

    state[node] = State::VISITING;

    auto it = graph.find(node);
    if (it != graph.end()) {
        for (const auto &dep : it->second) {
            if (!dfs(dep, graph, state, order))
                return false;
        }
    }

    state[node] = State::VISITED;
    order.push_back(node);
    return true;
}

bool resolve_order(
    const fs::path &dir,
    std::unordered_map<std::string, fs::path> &sql_files,
    std::vector<std::string> &order) {

    for (auto &e : fs::directory_iterator(dir)) {
        if (e.path().extension() == ".sql") {
            sql_files[e.path().stem().string()] = e.path();
        }
    }

    if (sql_files.empty()) {
        std::cerr << "No SQL files found\n";
        return false;
    }

    std::unordered_map<std::string,
        std::vector<std::string>> graph;

    for (auto &[name, path] : sql_files) {
        auto deps = extract_csv_dependencies(read_file(path));
        for (auto &d : deps) {
            if (sql_files.count(d))
                graph[name].push_back(d);
        }
    }

    std::unordered_map<std::string, State> state;

    for (auto &[name, _] : sql_files) {
        if (state[name] == State::UNVISITED) {
            if (!dfs(name, graph, state, order)) {
                std::cerr << "Cyclic dependency detected\n";
                return false;
            }
        }
    }

    return true;
}

int cmd_compile(const fs::path &dir) {
    std::unordered_map<std::string, fs::path> sql_files;
    std::vector<std::string> order;

    if (!resolve_order(dir, sql_files, order))
        return 1;

    for (const auto &name : order) {
        fs::path sql = sql_files[name];
        fs::path cpp = dir / (name + ".cpp");
        fs::path bin = dir / name;

        std::cout << "[compile] " << name << "\n";

        if (!compile_sql_to_cpp(sql))
            return 1;

        std::string compile_cmd =
            "g++ " + cpp.string() + " -o " + bin.string() +
            " -I./duckdb/src/include"
            " -L./duckdb/build/release/src"
            " -lduckdb_static"
            " -lssl -lcrypto"
            " -lpthread -ldl"
            " -std=c++17";

        if (std::system(compile_cmd.c_str()) != 0) {
            std::cerr << "Compilation failed: " << name << "\n";
            return 1;
        }
    }

    return 0;
}

int cmd_run(const fs::path &dir) {
    std::unordered_map<std::string, fs::path> sql_files;
    std::vector<std::string> order;

    if (!resolve_order(dir, sql_files, order))
        return 1;

    for (const auto &name : order) {
        fs::path bin = dir / name;

        std::cout << "[run] " << name << "\n";

        if (std::system(bin.string().c_str()) != 0) {
            std::cerr << "Execution failed: " << name << "\n";
            return 1;
        }
    }

    return 0;
}

int main(int argc, char **argv) {

    if (argc < 3) {
        std::cerr << "Usage:\n";
        std::cerr << "  dag compile <dir>\n";
        std::cerr << "  dag run <dir>\n";
        return 1;
    }

    std::string sub = argv[1];
    fs::path dir = argv[2];

    if (sub == "compile")
        return cmd_compile(dir);

    if (sub == "run")
        return cmd_run(dir);

    std::cerr << "Unknown command: " << sub << "\n";
    return 1;
}
