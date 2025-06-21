#include <cstddef>

template <typename T>


class SinglyLinkedList  {
    private:
        struct Node {
            T data;
            Node* next;
            Node(const T& val) : data {val}, next {nullptr} {}
            Node(const T& val, Node* old_head) : data {val}, next {old_head} {}
        };
        Node* head;
        std::size_t len;
    public:
        SinglyLinkedList() : head{nullptr}, len{0} {}

        ~SinglyLinkedList() {clear();}

        //  Adds to the head
        void push_front(const T& value) {
            Node* new_head = new Node(value, head);
            head = new_head;
            ++len;
        }  

        // Adds to the tail
        void push_back(const T& value) {
            Node* new_tail = new Node(value);
            if (head==nullptr) {
                head = new_tail;
            }
            else {
                Node* temp = head;
                while(temp->next != nullptr) {
                    temp = temp->next;
                }
                temp->next = new_tail;
            }
            ++len;
            
        }   

        // Removes from the front
        void pop_front() {
            if(head!=nullptr) {
                Node* temp = head; // This is so that I can avoid a memory leak and delete the old head.
                head = head->next;
                --len;
                delete temp;
            }
        }                

        // Returns the number of elements
        std::size_t size() const {
            return len;
        }
        
        // Returns pointer to the data if found, else nullptr
        T* find(const T& value) {
            Node* temp = head;
            while(temp!= nullptr) {
                if(temp->data==value){
                    return &temp->data;
                }
                else {
                    temp = temp->next;
                }
            }
            return nullptr;
        }

        void clear() {
            while(head!= nullptr){
                Node* temp =  head;
                head = head->next;
                delete temp;
            }
            len = 0;
        }
};