#include <iostream>
#include <string>

//to reduce runtime complexity, one could replace this function with one that just computes n*(n+1)/2 and returns that, but that defeats the purpose fo the exersise
int add_nums(int n) {
    int out=0;
    for (auto i=1; i<=n; ++i)
        out+=i;
    return out;
}

int main() {
    std::cout << "What number would you like to sum up to?\n";
    std::string ans;
    std::cin >> ans;
    int n = std::stoi(ans);
    int result = {add_nums(n)};
    std::cout << "The sum of the first " << n << " numbers is: " << result << ".\n";
}
