`timescale 1 ns/1 ps
module mult_verilog (input [21:0] IN1, input [33:0] IN2, output [55:0] OUTPUT);
  assign OUTPUT = IN1*IN2;
endmodule


