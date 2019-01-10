CC = g++
CFLAGS = -Wall
LIBS = -L ../../target/debug/ -lmetropolis --std=c++11
INCLUDES = -I ../

all: test.cpp
	$(CC) $(CFLAGS) -o test test.cpp $(INCLUDES) $(LIBS)

clean:
	$(RM) test
