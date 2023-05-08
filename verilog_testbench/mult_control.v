`timescale 1 ns/1 ps
module mult_verilog (input [31:0] IN1, input [31:0] IN2, output [63:0] OUTPUT);
  assign OUTPUT = IN1*IN2;
endmodule


