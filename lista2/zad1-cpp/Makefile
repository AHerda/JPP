CC = g++
CFLAGS = -Wall -Wextra -g -O3 -std=c++17

SRCS = main.cpp gf.cpp
OBJS = $(SRCS:.cpp=.o)
TARGET = main

# Default target
all: $(TARGET)

# Compile source files into object files
$(OBJS): $(SRCS)
	$(CC) $(CFLAGS) -c $^

# Link object files into executable
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

# Clean up object files and executable
clean:
	rm -f $(OBJS) $(TARGET)
