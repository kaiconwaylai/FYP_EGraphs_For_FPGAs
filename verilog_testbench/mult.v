`timescale 1ns / 1ps

module mult_verilog (
    input[31:0] IN1,
    input[31:0] IN2,
    output[63:0] OUTPUT
    );

    wire [31:0] z0 = (IN1[15:0] * IN2[15:0]);
    wire [31:0] z2 = (IN1[31:16] * IN2[31:16]);
    wire [32:0] z1 = ((IN1[15:0] + IN1[31:16]) * (IN2[15:0] + IN2[31:16])) - (z2 + z0);
    assign OUTPUT = {{z2, z0[31:16]} + z1 , z0[15:0]};
endmodule