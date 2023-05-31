//Alpha = 0. Cost: LUTs = 229. DSPs = 12.  

`timescale 1ns / 1ps
    module mult(
        input[35:0] IN1,
        input[35:0] IN2,
        output[71:0] OUTPUT
    );
wire [17:0] slice_23;
wire [36:0] subW_20;
wire [18:0] addW_17;
wire [53:0] concat_14;
wire [17:0] slice_11;
wire [54:0] addW_22;
wire [37:0] mul_19;
wire [17:0] slice_13;
wire [17:0] slice_10;
wire [35:0] mul_7;
wire [17:0] slice_4;
wire [72:0] concat_24;
wire [36:0] subW_21;
wire [18:0] addW_18;
wire [35:0] mul_12;
wire [17:0] slice_6;
assign slice_23 = mul_12[17:0];
assign subW_20 = mul_19 - mul_7;
assign addW_17 = slice_10 + slice_4;
assign concat_14 = {mul_7,slice_13};
assign slice_11 = IN2[17:0];
assign addW_22 = concat_14 + subW_21;
assign mul_19 = addW_17 * addW_18;
assign slice_13 = mul_12[35:18];
assign slice_10 = IN1[17:0];
assign mul_7 = slice_4 * slice_6;
assign slice_4 = IN1[35:18];
assign concat_24 = {addW_22,slice_23};
assign subW_21 = subW_20 - mul_12;
assign addW_18 = slice_11 + slice_6;
assign mul_12 = slice_10 * slice_11;
assign slice_6 = IN2[35:18];
assign OUTPUT = concat_24;
    endmodule