#include <iostream>
#include <cstring>      // for std::memcpy
#include "SecureBuffer.hpp"

int main() {
    // ✅ Construct a buffer of size 16
    SecureBuffer buf1(16);

    // ✅ Fill buffer with test data
    std::memcpy(buf1.data(), "test_secret", 11);  // no null terminator needed

    // ✅ Read back data
    std::cout << "Original buffer contents: ";
    for (std::size_t i = 0; i < buf1.size(); ++i) {
        std::cout << static_cast<char>(buf1.data()[i]);
    }
    std::cout << "\n";

    // ✅ Copy construction
    SecureBuffer buf2 = buf1;
    std::cout << "Copied buffer contents: ";
    for (std::size_t i = 0; i < buf2.size(); ++i) {
        std::cout << static_cast<char>(buf2.data()[i]);
    }
    std::cout << "\n";

    // ✅ Move construction
    SecureBuffer buf3 = std::move(buf1);
    std::cout << "Moved buffer contents: ";
    for (std::size_t i = 0; i < buf3.size(); ++i) {
        std::cout << static_cast<char>(buf3.data()[i]);
    }
    std::cout << "\n";

    // ✅ Assign (copy)
    SecureBuffer buf4(16);
    buf4 = buf2;
    std::cout << "Assigned buffer contents: ";
    for (std::size_t i = 0; i < buf4.size(); ++i) {
        std::cout << static_cast<char>(buf4.data()[i]);
    }
    std::cout << "\n";

    // ✅ Assign (move)
    buf4 = std::move(buf3);
    std::cout << "After move-assign, buf4 contents: ";
    for (std::size_t i = 0; i < buf4.size(); ++i) {
        std::cout << static_cast<char>(buf4.data()[i]);
    }
    std::cout << "\n";

    // ✅ Optional: show that moved-from buffers are empty
    std::cout << "buf1 size after move: " << buf1.size() << "\n";
    std::cout << "buf3 size after move: " << buf3.size() << "\n";

    return 0;
}
