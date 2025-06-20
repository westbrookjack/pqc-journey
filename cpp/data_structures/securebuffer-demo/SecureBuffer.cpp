#include "SecureBuffer.hpp"

//rewrites all data values in the pointer of a SecureBuffer to 0 so they aren't kept in RAM
// uses a volatile pointer p so that the c++ compiler will no skip over this step
void SecureBuffer::secure_zero() {
    volatile uint8_t* p = data_;
    for (std::size_t i = 0; i < sz; ++i) {
        p[i]=0; 
    }
    
}


//constructor
SecureBuffer::SecureBuffer(std::size_t s)
    : data_{new uint8_t[s]}, sz{s} {}

//destructor
SecureBuffer::~SecureBuffer() {
    secure_zero();
    delete[] data_;
    data_ = nullptr;
    sz = 0;
}

// Copy constructor 
SecureBuffer::SecureBuffer(const SecureBuffer& a)
    :data_{new uint8_t[a.sz]},
    sz{a.sz}

{
    for(std::size_t i=0; i<sz; ++i){
        data_[i] = a.data_[i];
    }
}

// Copy assignment
SecureBuffer& SecureBuffer::operator=(const SecureBuffer& a) {
    if (this != &a) {
        secure_zero();
        delete[] data_;
        sz = a.sz;
        data_ = new uint8_t[sz];
        for (std::size_t i = 0; i < sz; ++i) {
            data_[i]=a.data_[i]; 
        }
    }
    return *this;
}

// Move constructor
SecureBuffer::SecureBuffer(SecureBuffer&& a) noexcept
    :data_{a.data_},
    sz{a.sz}
{
    a.data_ = nullptr;
    a.sz = 0;
}

//Move assignment
SecureBuffer& SecureBuffer::operator=(SecureBuffer&& a) noexcept {
    if (this != &a) {
        secure_zero();
        delete[] data_;
    
        data_ = a.data_;
        sz = a.sz;
    
        a.data_ = nullptr;
        a.sz = 0;
    }
    return *this;
}

std::size_t SecureBuffer::size() const{
    return sz;
}

uint8_t* SecureBuffer:: data() {
    return data_;
}

const uint8_t* SecureBuffer:: data() const{
    return data_;
}