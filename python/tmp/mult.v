`timescale 1ns / 1ps

module mult(
    input[2 - 1:0] a,
    input[2 - 1:0] b,
    output[4 - 1:0] out
    );
    assign out = a*b;
endmodule