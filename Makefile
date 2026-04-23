CC = gcc
CFLAGS = -Wall -Wextra -std=c11 -Iinclude -O2

SRC_DIR = src
OBJ_DIR = build
INC_DIR = include

SOURCES = $(wildcard $(SRC_DIR)/*.c)
OBJECTS = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SOURCES))
EXECUTABLE = $(OBJ_DIR)/blind_engine

all: clean_build $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

run: all
	./$(EXECUTABLE)

clean:
	rm -rf $(OBJ_DIR)/*.o $(EXECUTABLE)

clean_build:
	@mkdir -p $(OBJ_DIR)

.PHONY: all run clean clean_build
