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
    static inline int instanceCounter = 0;
    int id;
    
    Op(int size) : sz(size), id(instanceCounter++) {};
    virtual ~Op() {};

    static std::unique_ptr<Op> makeOperator(const std::string& op);

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

    virtual std::string val() const {return "";};
    int getSize() const {return sz;};
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
    std::string val() const {
        return "slc" + children[0]->val() + children[1]->val() + children[2]->val();
    }
};

struct Primitive : Op {
    std::string value = "";
    Primitive(std::string v) : Op(0), value(v) {}
    ~Primitive() = default;
    void print(std::ostream& os) const override {
        os << value;
    }
    std::string val() const {
        return value;
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
    std::string val() const {
        return "cct" + children[0]->val() + children[1]->val();
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
    std::string val() const {
        return "mul" + children[0]->val() + children[1]->val() + children[2]->val();
    }
};

struct Add : Op {
    Add() : Op(3) {}; 
    void print(std::ostream& os) const override {
        os << "(";
        children[1]->print(os);
        os << " + ";
        children[2]->print(os); 
        os << ")";
    }
    std::string val() const {
        return "add" + children[0]->val() + children[1]->val() + children[2]->val();
    }
};

struct Sub : Op {
    Sub() : Op(3) {}; 
    void print(std::ostream& os) const override {
        children[1]->print(os);
        os << " - ";
        children[2]->print(os); 
    }
    std::string val() const {
        return "sub" + children[0]->val() + children[1]->val() + children[2]->val();
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


