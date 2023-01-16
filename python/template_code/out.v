`timescale 1ns / 1ps

module mult(
    input[32 - 1:0] a,
    input[32 - 1:0] b,
    output[64 - 1:0] out
    );
    assign out = a*b;
endmodule