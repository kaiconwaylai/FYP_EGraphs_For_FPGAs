#include "multiplier.hpp"
#include <iostream>

int main() {

    std::vector<int> a = {0,0,1,1,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,0};
    std::vector<int> b =     {0,0,0,0,1,1,1,1,1,1,1,0,0,1,1,0,1,0};

    for(auto i : a) {
        std::cout << i;
    }
    std::cout << '\n';
    for(auto i : b) {
        std::cout << i;
    }
    std::cout << '\n';

    Register A(a.size(),a);
    Register B(b.size(), b);
    
    auto res = multiply(A,B);
    auto ress = res.getValue();
    for(auto i : ress) {
        std::cout << i;
    }
    std::cout << std::endl;

}