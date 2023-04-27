#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stack>
#include <memory>
#include <sstream>

#include "./include/operators.hpp"
#include "./include/helpers.hpp"

void trimParenthesis(std::string& word);
void squashRanges(std::string& word);

int main() {
    std::cout << "inside main \n";
    std::ifstream myFile;
    myFile.open("eggOutput.txt");
    std::cout << "open file \n";
    std::unique_ptr<Op> top;
    std::stack<Op*> stk;
    std::string word;
    while(myFile.good()) {
        myFile >> word;
        std::cout << word << "\n";
        trimParenthesis(word);
        auto newOp = Op::makeOperator(word);
        auto op = newOp.get();
        
        if(stk.empty()) {
            stk.push(op);
            top = std::move(newOp);
            continue;
        }

        if(stk.top()->addVal(newOp)) {
            stk.pop();
        }
        if(op->sz > 1) { 
            stk.push(op);
        }

    }
    std::stringstream ss;
    top->print(ss);
    std::string output = ss.str();
    std::cout << output << "\n\n";
    squashRanges(output);
    std::cout << output;
    std::cout << "\n";
    return 0;
}

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
    int msb = second.msb == first.width ? first.msb : second.msb;
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