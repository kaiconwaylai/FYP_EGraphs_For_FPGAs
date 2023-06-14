//Alpha = 0.005. Cost: LUTs = 204. DSPs = 3. 

`timescale 1ns / 1ps
    module mult(
        input[31:0] IN1,
        input[31:0] IN2,
        output[63:0] OUTPUT
    );
wire [65:0] concat_26;
wire [41:0] subW_23;
wire [19:0] addW_20;
wire [17:0] slice_11;
wire [27:0] mul_8;
wire [13:0] slice_5;
wire [17:0] slice_25;
wire [40:0] subW_22;
wire [19:0] addW_19;
wire [45:0] concat_16;
wire [35:0] mul_13;
wire [13:0] slice_7;
wire [47:0] addW_24;
wire [39:0] mul_21;
wire [17:0] slice_15;
wire [17:0] slice_12;
assign concat_26 = {addW_24,slice_25};
assign subW_23 = subW_22 - mul_13;
assign addW_20 = slice_12 + slice_7;
assign slice_11 = IN1[17:0];
assign mul_8 = slice_5 * slice_7;
assign slice_5 = IN1[31:18];
assign slice_25 = mul_13[17:0];
assign subW_22 = mul_21 - mul_8;
assign addW_19 = slice_11 + slice_5;
assign concat_16 = {mul_8,slice_15};
assign mul_13 = slice_11 * slice_12;
assign slice_7 = IN2[31:18];
assign addW_24 = concat_16 + subW_23;
assign mul_21 = addW_19 * addW_20;
assign slice_15 = mul_13[35:18];
assign slice_12 = IN2[17:0];
assign OUTPUT = concat_26;
    endmodule