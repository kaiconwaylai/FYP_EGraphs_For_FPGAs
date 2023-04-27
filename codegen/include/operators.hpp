#pragma once

#include <iostream>
#include <string>
#include <stack>
#include <memory>

struct Op {
public:
    bool complete = false;
    int sz = 0;
    int count = 0;
    std::unique_ptr<Op> children[3];
    
    Op(int size) : sz(size) {};
    virtual ~Op() {};

    static std::unique_ptr<Op> makeOperator(std::string op);

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


enum class Operator {
    CONCAT,
    ADD,
    MUL,
    SUB,
    SLICE,
    PRIMITIVE
};


