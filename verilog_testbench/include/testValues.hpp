#pragma once

#include <vector>
#include <string>
#include "multiplier.hpp"

struct Test {
    std::string IN1;
    std::string IN2;
    std::string EXPECTED;
};

std::vector<Test> standardiseUnitTests(unsigned outputWidth, unsigned IN1, unsigned IN2) {

    std::vector<Test> unitTests {
        {"00110011111111100000", "000011111110011010", "011001110101011001000110011000000"},
        {"1", "1101010101010101010", "1101010101010101010"},
        {"0", "1010101010101", "0"},
        {"101101010", "1100111110", "01001001010110101100"}
    };

    auto l = std::string(IN1, '1');
    auto r = std::string(IN2, '1');

    unitTests.push_back(Test{l,r,multiply(l,r, outputWidth)});

    for(auto& tc : unitTests) {
        int pad = outputWidth - tc.EXPECTED.length();
        if(pad < 1) {
            continue;
        } else {
            tc.EXPECTED = std::string(pad, '0') + tc.EXPECTED;
        }
    }
    return unitTests;
}


