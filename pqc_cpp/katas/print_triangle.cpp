#include <iostream>
#include <string>
#include <iomanip>

int main(){
    
    std::string str;
    std::cout << "Enter the height of your triangle. \n";
    std::cin >> str;
    int n = std::stoi(str);

    for(int i=1; i<=n;++i){
            str = "";
            for(int k=0;k<i;++k){
                str+= '*';
            }
        std::cout << std::right << std::setw(n) << str << "\n";
    }
}