# FYP_EGraphs_For_FPGAs


## Dependencies

 - rust \& cargo
 - cbc - solver as described in \url{https://docs.rs/egg/latest/egg/struct.LpExtractor.html}
 - Xilinx - Vivado + licences 
 - python3
 - Top level script will only work in a linux environment
 - Various python scripts inside the directory require numpy / pandas and matplotlib although they are not needed for main script

## Directories

### egg
This directory contains source code to run the e-graph creation, extraction and verilog generation. Implementation is split between files in src and an output folder should be generated when run. Using `cargo build` or `cargo run` should install and run the program with an optional command line argument to provide the bitwidth of the multiplier that will be optimised. 

`output/verilog/` should contain various `mult\_{x}.v` files which provide complete verilog modules for a multiplier. The `x` value represents which iteration the design was created at.

### verilog_testbench

This contains a program that can run the test suite for the multiplier. `README` contains the original instructions from the Vivado example for running the program. 

    #!/bin/csh -xvf
    #setenv LD_LIBRARY_PATH $RDI_ROOT/prep/rdi/vivado/lib/lnx64.g:${LD_LIBRARY_PATH}

    #setenv LD_LIBRARY_PATH $RDI_ROOT/prep/rdi/vivado/lib/lnx64.o:${LD_LIBRARY_PATH}
    setenv LD_LIBRARY_PATH $XILINX_VIVADO/lib/lnx64.o:${LD_LIBRARY_PATH}

Above shows the code provided in `set\_env.csh` with the original Vivado XSI example.
The testbench can be run from the `run.bat` or `run.sh` scripts depending on the environment, and will test the `mult` module contained in `mult.v` against a standard multiplier. The names of inputs and outputs inside the verilog file should be matched by the string in the `Input` and `Output` constructors.


### misc/python

The purpose of this folder was to collect data for the analysis portion of the report, including normal multiplication resource requirements and addition/subtraction. This folder is not invoked by the main program.

## Main program

The top level `main.py` file executes the script contained in `/egg` and then tests the design with the script in `/verilog\_testbench` before synthesising and optimising the design and recording the resultant utilisation. The main loop dictates for which bitwidths the script will run and relies on calling the file while inside the same directory. Resource usage is displayed in `data.csv`. 

