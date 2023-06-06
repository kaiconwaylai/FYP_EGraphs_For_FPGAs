//Alpha = 0.016. Cost: LUTs = 857. DSPs = 50.  

`timescale 1ns / 1ps
    module mult(
        input[122:0] IN1,
        input[122:0] IN2,
        output[245:0] OUTPUT
    );
wire [245:0] mul_3;
assign mul_3 = IN1 * IN2;
assign OUTPUT = mul_3;
    endmodule