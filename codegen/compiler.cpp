#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stack>
#include <memory>
#include <sstream>
#include <unordered_map>

#include "./include/operators.hpp"
#include "./include/helpers.hpp"

int main() {
    std::ifstream myFile("eggOutput.txt");

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
    myFile.close();
    
    printAllDups(top.get());
    std::stringstream ss;
    top->print(ss);
    std::string output = ss.str();
    std::cout << output << "\n\n";
    squashRanges(output);
    std::cout << output;
    std::cout << "\n";
    return 0;
}