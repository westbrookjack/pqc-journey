// "clang++ -std=c++20 -I. -o tests tests.cpp" is how I compile the tests


#define CATCH_CONFIG_MAIN
#include "catch.hpp"

#include "../data_structures/singly_linked_list.hpp"



TEST_CASE("List starts empty") {
    SinglyLinkedList<int> list;
    REQUIRE(list.size() == 0);
}

TEST_CASE("push_back inserts correctly") {
    SinglyLinkedList<int> list;
    list.push_back(42);
    REQUIRE(list.size() == 1);
    REQUIRE(*list.find(42) == 42);
}
