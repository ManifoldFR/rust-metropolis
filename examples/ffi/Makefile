CC = g++

CFLAGS = -g --std=c++11 -Wall
ROOT_DIR = ../..

RM = rm
INCLUDES = -I../../cffi
LIB_INCLUDE = -Wl,-rpath,$(ROOT_DIR)/target/debug/ -L$(ROOT_DIR)/target/debug/
LIBRARIES = -lmetropolis

all: main.cpp
	$(CC) main.cpp $(CFLAGS) $(INCLUDES) $(LIB_INCLUDE) $(LIBRARIES) -o main

clean:
	$(RM) $(TARGET)
