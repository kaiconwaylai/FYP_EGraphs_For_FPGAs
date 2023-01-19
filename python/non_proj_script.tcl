set OUTPUT_DIR tmp 
set PART xcvu9p-flga2104-2-i
set SOURCE tmp
set FILE_NAME.v mult.v

set_param general.maxBackupLogs 0

file mkdir ${OUTPUT_DIR}

read_verilog [glob tmp/mult.v]

synth_design -top MULT -part ${PART}

report_utilization -file ${OUTPUT_DIR}/synth.xml -format xml

exit