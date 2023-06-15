//Alpha = 0.0369. Cost: LUTs = 0. DSPs = 0.  

`timescale 1ns / 1ps
    module mult(
        input[31:0] IN1,
        input[31:0] IN2,
        output[63:0] OUTPUT
        );
        
        wire [25:0] x_1 = IN1[25:0];
        wire [5:0] x_2 = IN1[31:26];
        
        wire [8:0] x_3 = IN1[8:0];
        wire [8:0] x_4 = IN1[17:9];
        wire [7:0] x_5 = IN1[25:18];

        wire [19:0] y_1 = IN2[19:0];
        wire [5:0] y_2 = IN2[25:20];
        wire [5:0] y_3 = IN2[31:26];
        
        wire [8:0] y_4 = IN2[8:0];
        wire [8:0] y_5 = IN2[17:9];
        wire [6:0] y_6 = IN2[24:18];
        wire [6:0] y_7 = IN2[31:25];
        
        assign OUTPUT = x_1*y_1 + (y_2*x_3 << 20) + (y_3*x_3 << 26) + (y_2*x_4 << 29) +(y_2*x_4 << 35)
            + (y_2*x_5 << 38) + (y_3*x_5 << 44) + (x_2*y_4 << 26) + (x_2*y_5 << 35) + (x_2*y_6 << 44) + (x_2*y_7 << 51);

    endmodule