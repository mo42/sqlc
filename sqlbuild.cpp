#include <filesystem>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <cstdlib>

namespace fs = std::filesystem;

std::string read_file(const fs::path &p) {
    std::ifstream in(p);
    std::string s((std::istreambuf_iterator<char>(in)),
                   std::istreambuf_iterator<char>());
    return s;
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

int main(int argc, char **argv) {
    if (argc != 2) {
        std::cerr << "Usage: sqlbuild <sql_directory>\n";
        return 1;
    }

    fs::path dir = argv[1];
    std::unordered_map<std::string, fs::path> sql_files;

    for (auto &e : fs::directory_iterator(dir)) {
        if (e.path().extension() == ".sql") {
            sql_files[e.path().stem().string()] = e.path();
        }
    }

    if (sql_files.empty()) {
        std::cerr << "No .sql files found\n";
        return 1;
    }

    std::unordered_map<std::string,
        std::vector<std::string>> graph;

    for (auto &[name, path] : sql_files) {
        auto deps = extract_csv_dependencies(read_file(path));
        for (auto &d : deps) {
            if (sql_files.count(d)) {
                graph[name].push_back(d);
            }
        }
    }

    std::unordered_map<std::string, State> state;
    std::vector<std::string> order;

    for (auto &[name, _] : sql_files) {
        if (state[name] == State::UNVISITED) {
            if (!dfs(name, graph, state, order)) {
                std::cerr << "Cyclic dependency detected\n";
                return 1;
            }
        }
    }

    for (const auto &name : order) {
        fs::path sql = sql_files[name];
        fs::path cpp = dir / (name + ".cpp");
        fs::path bin = dir / name;

        std::cout << "==> " << name << "\n";

        std::string sqlc_cmd =
            "./sqlc " + sql.string();
        if (std::system(sqlc_cmd.c_str()) != 0) {
            std::cerr << "sqlc failed: " << name << "\n";
            return 1;
        }

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

        if (std::system(bin.string().c_str()) != 0) {
            std::cerr << "Execution failed: " << name << "\n";
            return 1;
        }
    }
    return 0;
}
