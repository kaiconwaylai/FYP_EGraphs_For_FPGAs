set OUTPUT_DIR tmp 
set PART xcu250-figd2104-2L-e
set SOURCE tmp
set FILE_NAME.v mult.v

set_param general.maxBackupLogs 0

file mkdir ${OUTPUT_DIR}

read_verilog [glob tmp/mult.v]

synth_design -top MULT -part ${PART}

opt_design

report_utilization -file ${OUTPUT_DIR}/synth.xml -format xml

exit