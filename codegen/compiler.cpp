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
    std::ifstream myFile;
    myFile.open("eggOutput.txt");

    std::unique_ptr<Op> top;
    std::stack<Op*> stk;
    std::string word;
    while(myFile.good()) {
        myFile >> word;
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

        if(op->getSize() != 0) { 
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