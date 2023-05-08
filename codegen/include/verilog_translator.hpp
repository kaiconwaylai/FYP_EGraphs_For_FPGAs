#include <iostream>
#include <string>

void printModuleBody(std::ostream& os);

void printModule(std::ostream& os, unsigned IN1_WIDTH, unsigned IN2_WIDTH, unsigned OUTPUT_WIDTH) {

    os << "`timescale 1 ns/1 ps\n";
    os << "module mult_verilog (input [" << IN1_WIDTH-1 << ":0] IN1, input [" << IN2_WIDTH-1 << ":0] IN2, output [" << OUTPUT_WIDTH-1 << ":0] OUTPUT);\n";

    printModuleBody(os);

    os << "endmodule\n";
};


std::string simplifyExpression(std::string expression) {

    


}
