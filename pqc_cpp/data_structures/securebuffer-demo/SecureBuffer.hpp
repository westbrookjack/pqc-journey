#include <cstdint>
#include <cstddef>

class SecureBuffer {

    private:
        uint8_t* data_;
        std::size_t sz;
        void secure_zero();
    
    public:
        SecureBuffer(std::size_t s); //constructor
        ~SecureBuffer(); //destructor

        SecureBuffer(const SecureBuffer& a); //copy constructor
        SecureBuffer& operator=(const SecureBuffer& a); //copy assignment

        SecureBuffer(SecureBuffer&& a) noexcept; //move constructor
        SecureBuffer& operator=(SecureBuffer&& a) noexcept; //move assignment

        //gets the size of the data sz
        std::size_t size() const;
        // gets the data data_ for writing
        uint8_t* data();
        // gets the data data_, for reading only
        // first const means the returned data points to a const, and the second const means that no data will be modified
        const uint8_t* data() const;
};