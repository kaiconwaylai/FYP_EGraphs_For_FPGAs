#!/bin/bash -xvf

#Set VIVADO_BIN_DIR to the directory which has vivado executable

## the paths etc have been butchered to try get it to work in wsl idk how / if its possible;


#set VIVADO_BIN_DIR="$RDI_ROOT/prep/rdi/vivado/bin"

VIVADO_BIN_DIR="/mnt/c/Xilinx/Vivado/2020.2/bin"

OUT_SIM_SNAPSHOT="mult"
XSI_INCLUDE_DIR="$VIVADO_BIN_DIR/../data/xsim/include"
GCC_COMPILER="/usr/bin/g++"
XSIM_ELAB="xelab"
OUT_EXE="run_simulation"

# Start clean
#rm -rf xsim.dir xsim.log xelab* $OUT_EXE

# Compile the HDL design into a simulatable Shared Library
/mnt/c/Xilinx/Vivado/2020.2/bin/xelab work.mult_verilog -prj mult.prj -dll -s $OUT_SIM_SNAPSHOT -debug wave

# Compile the C++ code that interfaces with XSI of ISim
#$GCC_COMPILER -I$XSI_INCLUDE_DIR  -g -c -o xsi_loader.o xsi_loader.cpp
$GCC_COMPILER -I$XSI_INCLUDE_DIR  -O3 -c -o xsi_loader.o ./src/xsi_loader.cpp
$GCC_COMPILER -I$XSI_INCLUDE_DIR  -O3 -c -o register.o ./src/Register.cpp
# Compile the program that needs to simulate the HDL design
#$GCC_COMPILER -I$XSI_INCLUDE_DIR  -g -c -o testbench.o testbench.cpp
$GCC_COMPILER -I$XSI_INCLUDE_DIR  -O3 -c -o testbench.o multiplierTestbench.cpp

$GCC_COMPILER -ldl -lrt  -o $OUT_EXE testbench.o xsi_loader.o register.o

# Run the program
./$OUT_EXE $1

find . -name '*.o' -delete
rm run_simulation
