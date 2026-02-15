CXX := g++
CXXFLAGS := -std=c++17 -Wall -O2
INCLUDES := -I./duckdb/src/include
LDFLAGS := -L./duckdb/build/release/src
LIBS := -lduckdb_static -lssl -lcrypto -lpthread -ldl
TARGET := dag
SRC := dag.cpp

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) $(INCLUDES) $^ -o $@ $(LDFLAGS) $(LIBS)

clean:
	rm -f $(TARGET)
