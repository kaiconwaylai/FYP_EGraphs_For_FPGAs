#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stack>
#include <memory>
#include <sstream>
#include <unordered_map>

void trimParenthesis(std::string& word) {
    while(word.front() == '(') {
        word.erase(word.begin());
    }

    while(word.back() == ')') {
        word.erase(word.end()-1);
    }
}

struct BitRange {
    int msb;
    int lsb;
    int width;
    void processRange(std::string range) {
        auto colon = range.find(':');
        auto top = range.substr(0,colon);
        auto bottom = range.substr(colon+1);
        msb = stoi(top);
        lsb = stoi(bottom);
        width = msb-lsb;
    }

    std::string str() {
        return "[" + std::to_string(msb) + ":" + std::to_string(lsb) + "]";
    }
};

BitRange squashRange(BitRange first, BitRange second) {
    int msb = first.lsb + second.msb;
    int lsb = msb-second.width;

    return BitRange{msb,lsb,msb-lsb};
}

void squashRanges(std::string& word) {
    bool eos = false;
    while(!eos) {
        int state = 0;
        BitRange firstRange, secondRange;
        int start, end;

        for(int i = 0; i < word.size(); i++) {
            char curr = word[i];
            std::string range;
            bool consecutiveRange = false;
            if(curr == '[') {
                start = i;
                curr = word[++i];
                while(curr != ']') {
                    range.push_back(curr);
                    curr = word[++i];
                }
                firstRange.processRange(range);
                consecutiveRange = true;
            }
            if(consecutiveRange && word[i+1] == '[') {
                range = "";
                i += 2;
                while(word[i] != ']') {
                    range.push_back(word[i++]);
                }
                end = i;
                secondRange.processRange(range);

                auto NewRange = squashRange(firstRange, secondRange);
                word.replace(start, end-start+1, NewRange.str());
                break;
            }

            if(i == word.size()-1) {
                eos = true;
            }
        }
    }
}