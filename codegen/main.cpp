#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stack>
#include <memory>
#include <sstream>
#include <string>

struct Op {
public:
    bool complete = false;
    int sz = 0;
    int count = 0;
    std::unique_ptr<Op> children[3];
    
    Op(int size) : sz(size) {};
    virtual ~Op() {};

    virtual void print(std::ostream& os) const {
        os << "We have a problem \n";
    };

    bool addVal(std::unique_ptr<Op>& ptr) {
        children[count] = std::move(ptr);

        count++;
        if(count == sz) {
            complete = true;
        }
        return complete;
    }
};

struct Slice : Op {
    Slice() : Op(3) {};
    ~Slice() = default;
    void print(std::ostream& os) const override {
        children[0]->print(os);
        os << "[";
        children[1]->print(os);
        os << ":";
        children[2]->print(os);
        os << "]";
    }
};

struct Primitive : Op {
    std::string val = "";
    Primitive(std::string v) : Op(0), val(v) {}
    ~Primitive() = default;
    void print(std::ostream& os) const override {
        os << val;
    }
};

struct Concat : Op {
    Concat() : Op(2) {};
    void print(std::ostream& os) const override {
        os << "{";
        children[0]->print(os);
        os << ",";
        children[1]->print(os);
        os << "}"; 
    }
};

struct Mul : Op {
    Mul() : Op(3) {};
    void print(std::ostream& os) const override {
        os << "(";
        children[1]->print(os);
        os << " *";
        //children[0]->print(os);
        os << " ";
        children[2]->print(os);
        os << ")"; 
    }
};

struct Add : Op {
    Add() : Op(2) {}; 
    void print(std::ostream& os) const override {
        os << "(";
        children[0]->print(os);
        os << " + ";
        children[1]->print(os); 
        os << ")";
    }
};

struct Sub : Op {
    Sub() : Op(2) {}; 
    void print(std::ostream& os) const override {
        children[0]->print(os);
        os << " - ";
        children[1]->print(os); 
    }
};

void trimParenthesis(std::string& word);
void squashRanges(std::string& word);

int main() {
    std::ifstream myfile;
    myfile.open("file.txt");
    std::unique_ptr<Op> top;
    std::stack<Op*> stk;
    std::string word;
    while(myfile.good()) {
        myfile >> word;
        trimParenthesis(word);
        if(word == "concat") { 
            auto newOp = std::unique_ptr<Op>(new Concat());
            if(stk.empty()) {
                stk.push(newOp.get());
                top = std::move(newOp);
                continue;
            }

            auto ptr = newOp.get();
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }
            stk.push(ptr);
        
        } else if(word == "+") {
            auto newOp = std::unique_ptr<Op>(new Add());

            auto ptr = newOp.get();
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }
            stk.push(ptr);
        } else if(word == "*") {
            auto newOp = std::unique_ptr<Op>(new Mul()); 

            auto ptr = newOp.get();
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }
            stk.push(ptr);
        } else if(word == "slice") {
            auto newOp = std::unique_ptr<Op>(new Slice());

            auto ptr = newOp.get();
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }
            stk.push(ptr);
        } else if(word == "-") {
            auto newOp = std::unique_ptr<Op>(new Sub());

            auto ptr = newOp.get();
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }
            stk.push(ptr);
        } else {
            auto newOp = std::unique_ptr<Op>(new Primitive(word));
            if(stk.top()->addVal(newOp)) {
                stk.pop();
            }

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