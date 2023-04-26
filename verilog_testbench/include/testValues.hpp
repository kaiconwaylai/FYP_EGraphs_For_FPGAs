#pragma once

#include <vector>
#include <string>

struct Test {
    std::string IN1;
    std::string IN2;
    std::string EXPECTED;
};

std::vector<Test> TestCases {
    {"00110011111111100000", "000011111110011010", "011001110101011001000110011000000"},
    {"1", "1101010101010101010", "1101010101010101010"},
    {"0", "1010101010101", "0"},
    {"101101010", "1100111110", "01001001010110101100"}
};


