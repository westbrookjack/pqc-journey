#include <iostream>
#include <string>

int binary_max (int x, int y){
    if(x<y)
        return y;
    else
        return x;
}

int trinary_max(int x, int y, int z){
    int m = binary_max(x,y);
    int n = binary_max(m, z);
    return n;
}

int main(){
    std::string ans;
    std::string str;
    int arr[3];
    std::cout << "Enter three integers, each separated by a comma, with no whitespace. \n";
    std::cin >> ans;
    for(int j=0; j<3; ++j) {
        str = "";
        for(int i=0; i<ans.length(); ++i) {
            if(ans[i]!=',') {
                str+=ans[i];
            }
            else {
                ans.erase(0,i+1);
                break;
            }
        }
        arr[j] = std::stoi(str);
    }

    std::cout << "The maximum of your three integers is: " << trinary_max(arr[0], arr[1], arr[2]) << ".\n";

}