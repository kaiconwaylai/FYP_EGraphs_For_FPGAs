#include <unordered_map>

#include "../include/operators.hpp"

std::unique_ptr<Op> Op::makeOperator(std::string op) {
    const std::unordered_map<std::string, Operator> stringToOperator = {{"concat", Operator::CONCAT}, {"+", Operator::ADD},{"*", Operator::MUL}, {"-", Operator::SUB}, {"slice", Operator::SLICE}};
    auto iter = stringToOperator.find(op);
    if (iter != stringToOperator.end()) {
        switch(iter->second) {
            case Operator::CONCAT:
                return std::make_unique<Concat>();
            case Operator::ADD:
                return std::make_unique<Add>();
            case Operator::MUL:
                return std::make_unique<Mul>();
            case Operator::SUB:
                return std::make_unique<Sub>();
            case Operator::SLICE:
                return std::make_unique<Slice>();
            default:
                return std::make_unique<Primitive>(op);
        }
    }
    return std::make_unique<Primitive>(op);
}