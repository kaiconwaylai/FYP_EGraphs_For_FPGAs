@REM call set install_root="C:\Xilinx\Vivado\2020.2"

@REM call set PATH=%install_root%/lib/win64.o;%install_root%/tps/mingw/6.2.0/win64.o/nt/x86_64-w64-mingw32/lib

@REM call %install_root%\bin\xelab work.mult_verilog -prj mult.prj -dll -s mult

@REM call "%install_root%\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -I%install_root%\data\xsim\include\ -O3 -c -o xsi_loader.o xsi_loader.cpp

@REM call "%install_root%\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -I%install_root%\data\xsim\include\ -O3 -c -o testbench.o testbench.cpp

@REM call "%install_root%\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -o run_xsim testbench.o xsi_loader.o

@REM call run_xsim

@REM call del *.o
@REM call del *.pb
@REM call del *.log

call del /F /Q xsim.dir
call del *.o
call del run_xsim.exe
call del failed_testing.txt

call set install_root="C:\Xilinx\Vivado\2020.2"

call set PATH=C:\Xilinx\Vivado\2020.2/lib/win64.o;C:\Xilinx\Vivado\2020.2/tps/mingw/6.2.0/win64.o/nt/x86_64-w64-mingw32/lib


call C:\Xilinx\Vivado\2020.2\bin\xelab work.mult -prj mult.prj -dll -s mult

call "C:\Xilinx\Vivado\2020.2\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -I C:\Xilinx\Vivado\2020.2\data\xsim\include\ -O3 -c -o xsi_loader.o ./src/xsi_loader.cpp

call "C:\Xilinx\Vivado\2020.2\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -I C:\Xilinx\Vivado\2020.2\data\xsim\include\ -O3 -c -o testbench.o multiplierTestbench.cpp

call "C:\Xilinx\Vivado\2020.2\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -I C:\Xilinx\Vivado\2020.2\data\xsim\include\ -O3 -c -o register.o ./src/Register.cpp

call "C:\Xilinx\Vivado\2020.2\tps\mingw\6.2.0\win64.o\nt\bin\g++.exe" -o run_xsim testbench.o xsi_loader.o register.o

call run_xsim %1

call del *.o
call del *.pb
call del *.log
call del run_xsim.exe