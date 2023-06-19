import subprocess
import os
import shutil
import linecache
import csv
import xml.dom.minidom
import time

cur_dir = os.getcwd()
egg_exe = cur_dir + "/egg/run_egg.sh"
egg_output = cur_dir + "/egg/output/verilog"
tb_exe = cur_dir + "/verilog_testbench/run.sh"
tb_v_src = cur_dir + "/verilog_testbench/mult.v"
synthesise = cur_dir + "/python/run_tcl.sh"
synthesis_v_src = "./tmp/mult.v"

def run_egg(bw):
    os.chdir(cur_dir + "/egg")
    process = subprocess.Popen(['sh', egg_exe, str(bw)])
    os.chdir(cur_dir)
    process.wait()

def run_synth():
    process = subprocess.Popen(['sh', synthesise])
    process.wait()

def run_testing(bw):
    os.chdir(cur_dir + "/verilog_testbench")
    process = subprocess.Popen(['sh', tb_exe, str(bw)])
    os.chdir(cur_dir)
    process.wait()

def extract_data(path):
    doc = xml.dom.minidom.parse(path)
    for sec in doc.getElementsByTagName('section'):
        if sec.getAttribute("title") == "CLB Logic":
            CLB = sec
        elif sec.getAttribute("title") == "ARITHMETIC":
            ARITHMETIC = sec

    LUTs = (CLB.childNodes[1].childNodes[5].childNodes[3].getAttribute('contents'))
    DSPs = (ARITHMETIC.childNodes[1].childNodes[3].childNodes[3].getAttribute('contents'))

    return int(LUTs), int(DSPs)

def main():
    if not os.path.exists('./tmp'):
        os.mkdir('tmp')
        
    make_top_level('./tmp')
    open('tmp/mult.v','w')
    
    for bw in [32]:
        run_egg(bw)
        for filename in os.listdir(egg_output):
            fname = os.path.join(egg_output, filename)
            if not os.path.isfile(fname):
                continue
            shutil.copyfile(fname, tb_v_src)
            shutil.copyfile(fname, synthesis_v_src)
            run_testing(bw)
            if os.path.exists('./verilog_testbench/failed_testing.txt'):
                with open('data.csv', 'a') as ostream:
                    ostream.write("Failed tests for bitwidth: {}".format(bw))
                    ostream.write("More information in ./verilog_testbench/failed_testing.txt")
                    break;
            run_synth()
            
            luts, dsps = extract_data('tmp/synth.xml')
            with open('data.csv', 'a') as ostream:
                writer = csv.writer(ostream)
                writer.writerow([bw, luts, dsps])
        

def make_top_level(path):
    with open(path + '/top_level.v', 'w') as fs:
        fs.write("`timescale 1ns / 1ps \n\
module top_level( \n\
    input clk,\n\
    output out\n\
    );\n\
    reg IN1;\n\
    reg IN2;\n\
    wire OUTPUT;\n\
                \n\
    (* dont_touch = \"yes\" *)\n\
    mult multiplier(\n\
        IN1,\n\
        IN2,\n\
        OUTPUT\n\
        );     \n\
endmodule\
            ")

if __name__ == "__main__":
    main()
