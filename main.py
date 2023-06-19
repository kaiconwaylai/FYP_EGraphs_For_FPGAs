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

def run_synth(bw):
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
    start_prog = time.time()
    if not os.path.exists('./tmp'):
        os.mkdir('tmp')
        
    make_top_level('./tmp')
    open('tmp/mult.v','w')
    
    testing_times = []
    synth_times = []
    egg_times = []


    for bw in [32]:
        start_egg = time.time()
        run_egg(bw)
        end_egg = time.time()
        for filename in os.listdir(egg_output):
            fname = os.path.join(egg_output, filename)
            if not os.path.isfile(fname):
                continue
            shutil.copyfile(fname, tb_v_src)
            shutil.copyfile(fname, synthesis_v_src)
            start_testing = time.time()
            run_testing(bw)
            end_testing = time.time()
            start_synth = time.time()
            run_synth(bw)
            end_synth = time.time()
            
            testing_times.append((bw,(end_testing-start_testing)))
            synth_times.append(end_synth-start_synth)
            
            luts, dsps = extract_data('tmp/synth.xml')
            with open('data.csv', 'a') as ostream:
                writer = csv.writer(ostream)
                writer.writerow([bw, luts, dsps])
        egg_times.append((bw,end_egg-start_egg))
        
    end_prog = time.time()
    prog_time = end_prog - start_prog
    
    with open('times.txt', 'w') as fs:
        fs.write("prog execution time: {}\n".format(prog_time))
        total_test = 0
        total_synth = 0
        total_egg = 0
        for (bw,t1),t2 in zip(testing_times, synth_times):
            fs.write("BW: {}. Test time: {}, Synth time: {}\n".format(bw, t1,t2))
            total_test += t1
            total_synth += t2
        for bw, t in egg_times:
            total_egg += t
            fs.write("BW: {}, Egg time: {}\n".format(bw,t))
            
        fs.write("Total test time: {}\n".format(total_test))
        fs.write("Total synth time: {}\n".format(total_synth))
        fs.write("Non synth / test / egg time: {}\n".format(prog_time-total_test-total_synth-total_egg))


def make_top_level(path):
    with open(path + '/top_level.v', 'w') as fs:
        fs.write("`timescale 1ns / 1ps \
                module top_level( \
                input clk,\
                output out\
                );\
                reg IN1;\
                reg IN2;\
                wire OUTPUT;\
                \
                (* dont_touch = \"yes\" *)\
                mult multiplier(\
                    IN1,\
                    IN2,\
                    OUTPUT\
                );     \
            endmodule\
            ")

if __name__ == "__main__":
    main()
