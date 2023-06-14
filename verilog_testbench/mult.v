//Alpha = 0.012. Cost: LUTs = 542. DSPs = 48.  

`timescale 1ns / 1ps
    module mult(
        input[63:0] IN1,
        input[63:0] IN2,
        output[127:0] OUTPUT
    );
wire [31:0] slice_23;
wire [68:0] subW_20;
wire [33:0] addW_17;
wire [95:0] concat_14;
wire [31:0] slice_11;
wire [97:0] addW_22;
wire [67:0] mul_19;
wire [31:0] slice_13;
wire [31:0] slice_10;
wire [63:0] mul_7;
wire [31:0] slice_4;
wire [129:0] concat_24;
wire [69:0] subW_21;
wire [33:0] addW_18;
wire [63:0] mul_12;
wire [31:0] slice_6;
assign slice_23 = mul_12[31:0];
assign subW_20 = mul_19 - mul_7;
assign addW_17 = slice_10 + slice_4;
assign concat_14 = {mul_7,slice_13};
assign slice_11 = IN2[31:0];
assign addW_22 = concat_14 + subW_21;
assign mul_19 = addW_17 * addW_18;
assign slice_13 = mul_12[63:32];
assign slice_10 = IN1[31:0];
assign mul_7 = slice_4 * slice_6;
assign slice_4 = IN1[63:32];
assign concat_24 = {addW_22,slice_23};
assign subW_21 = subW_20 - mul_12;
assign addW_18 = slice_11 + slice_6;
assign mul_12 = slice_10 * slice_11;
assign slice_6 = IN2[63:32];
assign OUTPUT = concat_24;
    endmodule