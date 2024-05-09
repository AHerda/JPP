cmake_minimum_required(VERSION 3.12)
project(Lista3-zad1)

# Add your source files here
set(SOURCES
	main.cpp
	../../lista2/zad1-cpp/gf.cpp
	#User.cpp
)

# Add library files
# set(LIBS
# 	../../lista2/zad1-cpp/gf.cpp
#)

# Create the executable
add_executable(main ${SOURCES})
# add_library(mylib ${LIBS})
