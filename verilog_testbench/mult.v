//Alpha = 0.037. Cost: LUTs = 0. DSPs = 0.  

`timescale 1ns / 1ps
    module mult(
        input[225:0] IN1,
        input[225:0] IN2,
        output[451:0] OUTPUT
    );
wire [451:0] mul_3;
assign mul_3 = IN1 * IN2;
assign OUTPUT = mul_3;
    endmodule