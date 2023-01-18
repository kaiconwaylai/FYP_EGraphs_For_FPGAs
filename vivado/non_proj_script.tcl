set PROJECT_NAME              mult_diff_widths
#set PROJECT_CONSTRAINT_FILE ./fpga_constraint.xdc  no constrain rn
set OUTPUT_DIR tmp 
set PART xcu250-figd2104-2L-e
set SOURCE src
set FILE_NAME.v mult.v

file mkdir ${OUTPUT_DIR}

read_verilog [glob ${SOURCE}/*.v]

synth_design -top MULT -part ${PART}

report_utilization -file ${OUTPUT_DIR}/synth.rpt

exit