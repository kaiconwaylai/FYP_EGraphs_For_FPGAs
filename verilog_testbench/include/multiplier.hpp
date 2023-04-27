#pragma once

#include "Register.hpp"
#include <algorithm>
#include <string>

std::string logicShiftLeft(const std::string& in, unsigned shift) {
    auto firstOne = in.find('1');
    if(firstOne == std::string::npos) {
        return "0";
    }
    return in.substr() + std::string(shift,'0');
} 

std::string binaryAdd(std::string out, std::string in) {
    bool carry = 0;
    int sum = 0;
    int i = out.size()-1, j = in.size()-1;
    std::string result;
    while(j > -1 || i > -1) {
        sum = (i > -1 ? out[i]-'0' : 0) + (j > -1 ? in[j]-'0' : 0) + carry;
        switch(sum) {
            case 3: result.push_back('1'); break;
            case 2: result.push_back('0'); carry = 1; break;
            case 1: result.push_back('1'); carry = 0; break;
            case 0: result.push_back('0'); break;
            default: throw "Invalid input, not a binary number";
        }
        j--; i--;
    }
    // OVERFLOW
    if(carry) {
        result.push_back('1');
    }
    std::reverse(result.begin(), result.end());
    return result;
}

std::string multiply(const Input& A, const Input& B) {

    std::string result = "0";

    auto l = A.getValue();
    auto r = B.getValue();

    unsigned outputWidth = A.getWidth() + B.getWidth();

    if(l.size() < r.size()) 
        swap(l,r);

    int j = r.size()-1;
    while(j > -1) {
        if(r[j] == '1') {
            int shift = r.size()-1-j;
            auto shifted = logicShiftLeft(l, shift);
            result = binaryAdd(result, shifted);
        }
        j--;
    }

    result = std::string(outputWidth - result.length(), '0') + result;

    return result;
}

