# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 4.0

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /opt/homebrew/bin/cmake

# The command to remove a file.
RM = /opt/homebrew/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/jackwestbrook/pqc-journey/pqc_cpp

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/jackwestbrook/pqc-journey/pqc_cpp/build

# Include any dependencies generated for this target.
include CMakeFiles/mod_exp_bench.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/mod_exp_bench.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/mod_exp_bench.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/mod_exp_bench.dir/flags.make

CMakeFiles/mod_exp_bench.dir/codegen:
.PHONY : CMakeFiles/mod_exp_bench.dir/codegen

CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o: CMakeFiles/mod_exp_bench.dir/flags.make
CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o: /Users/jackwestbrook/pqc-journey/pqc_cpp/benches/mod_exp_bench.cpp
CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o: CMakeFiles/mod_exp_bench.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/jackwestbrook/pqc-journey/pqc_cpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o -MF CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o.d -o CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o -c /Users/jackwestbrook/pqc-journey/pqc_cpp/benches/mod_exp_bench.cpp

CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/jackwestbrook/pqc-journey/pqc_cpp/benches/mod_exp_bench.cpp > CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.i

CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/jackwestbrook/pqc-journey/pqc_cpp/benches/mod_exp_bench.cpp -o CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.s

CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o: CMakeFiles/mod_exp_bench.dir/flags.make
CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o: /Users/jackwestbrook/pqc-journey/pqc_cpp/number_theory/mod_exp.cpp
CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o: CMakeFiles/mod_exp_bench.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/jackwestbrook/pqc-journey/pqc_cpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o -MF CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o.d -o CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o -c /Users/jackwestbrook/pqc-journey/pqc_cpp/number_theory/mod_exp.cpp

CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/jackwestbrook/pqc-journey/pqc_cpp/number_theory/mod_exp.cpp > CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.i

CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/jackwestbrook/pqc-journey/pqc_cpp/number_theory/mod_exp.cpp -o CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.s

# Object files for target mod_exp_bench
mod_exp_bench_OBJECTS = \
"CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o" \
"CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o"

# External object files for target mod_exp_bench
mod_exp_bench_EXTERNAL_OBJECTS =

mod_exp_bench: CMakeFiles/mod_exp_bench.dir/benches/mod_exp_bench.cpp.o
mod_exp_bench: CMakeFiles/mod_exp_bench.dir/number_theory/mod_exp.cpp.o
mod_exp_bench: CMakeFiles/mod_exp_bench.dir/build.make
mod_exp_bench: /usr/local/lib/libbenchmark.a
mod_exp_bench: CMakeFiles/mod_exp_bench.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/Users/jackwestbrook/pqc-journey/pqc_cpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX executable mod_exp_bench"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/mod_exp_bench.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/mod_exp_bench.dir/build: mod_exp_bench
.PHONY : CMakeFiles/mod_exp_bench.dir/build

CMakeFiles/mod_exp_bench.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/mod_exp_bench.dir/cmake_clean.cmake
.PHONY : CMakeFiles/mod_exp_bench.dir/clean

CMakeFiles/mod_exp_bench.dir/depend:
	cd /Users/jackwestbrook/pqc-journey/pqc_cpp/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/jackwestbrook/pqc-journey/pqc_cpp /Users/jackwestbrook/pqc-journey/pqc_cpp /Users/jackwestbrook/pqc-journey/pqc_cpp/build /Users/jackwestbrook/pqc-journey/pqc_cpp/build /Users/jackwestbrook/pqc-journey/pqc_cpp/build/CMakeFiles/mod_exp_bench.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/mod_exp_bench.dir/depend

