set PROJECT_NAME              mult_diff_widths
#set PROJECT_CONSTRAINT_FILE ./fpga_constraint.xdc  no constrain rn
set DIR_OUTPUT tmp 
set PART partxcu250-figd2104-2L-e
set FOLDER src
set FILE_NAME.v mult.v

file mkdir ${DIR_OUTPUT}


# need to figure out which fpga device currently put one from impress
create_project -force ${PROJECT_NAME} ${DIR_OUTPUT}/${PROJECT_NAME} -part ${PART}

add_files ${FOLDER}/${FILE_NAME.v}

import_files -force

#import_files -fileset constrs_1 -force -norecurse ${PROJECT_CONSTRAINT_FILE}

update_compile_order -fileset sources_1

launch_runs synth_1

wait_on_run synth_1

open_run synth_1 -name netlist_1
