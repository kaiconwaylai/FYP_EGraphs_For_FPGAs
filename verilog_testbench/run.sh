#!/bin/bash -xvf

#Set VIVADO_BIN_DIR to the directory which has vivado executable

## the paths etc have been butchered to try get it to work in wsl idk how / if its possible;S

#set VIVADO_BIN_DIR="$RDI_ROOT/prep/rdi/vivado/bin"

VIVADO_BIN_DIR="/mnt/applications/Xilinx/20.1/Vivado/2020.1/bin"

OUT_SIM_SNAPSHOT="mult"
XSI_INCLUDE_DIR="$VIVADO_BIN_DIR/../data/xsim/include"
GCC_COMPILER="/usr/bin/g++"
XSIM_ELAB="xelab"
OUT_EXE="run_sim"

# Start clean
rm -rf xsim.dir xsim.log xelab* $OUT_EXE
rm failed_testing.txt

# Compile the HDL design into a simulatable Shared Library
/mnt/applications/Xilinx/20.1/Vivado/2020.1/bin/xelab work.mult -prj mult.prj -dll -s mult -debug wave

# Compile the C++ code that interfaces with XSI of ISim
#$GCC_COMPILER -I$XSI_INCLUDE_DIR  -g -c -o xsi_loader.o xsi_loader.cpp
/mnt/applications/Xilinx/20.1/Vivado/2020.1/tps/lnx64/gcc-6.2.0/bin/g++ -I /mnt/applications/Xilinx/20.1/Vivado/2020.1/data/xsim/include -O3 -c -o xsi_loader.o ./src/xsi_loader.cpp
/mnt/applications/Xilinx/20.1/Vivado/2020.1/tps/lnx64/gcc-6.2.0/bin/g++ -I /mnt/applications/Xilinx/20.1/Vivado/2020.1/data/xsim/include -O3 -c -o register.o ./src/Register.cpp
# Compile the program that needs to simulate the HDL design
#$GCC_COMPILER -I$XSI_INCLUDE_DIR  -g -c -o testbench.o testbench.cpp
/mnt/applications/Xilinx/20.1/Vivado/2020.1/tps/lnx64/gcc-6.2.0/bin/g++ -I /mnt/applications/Xilinx/20.1/Vivado/2020.1/data/xsim/include -O3 -c -o testbench.o multiplierTestbench.cpp

/mnt/applications/Xilinx/20.1/Vivado/2020.1/tps/lnx64/gcc-6.2.0/bin/g++ -ldl -lrt -o run_sim testbench.o xsi_loader.o register.o

# Run the program
./run_sim $1

find . -name '*.o' -delete
rm run_sim
