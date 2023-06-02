import subprocess
import os
import shutil

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


def main():
    if not os.path.exists('./tmp'):
        os.mkdir('tmp')
    open('tmp/mult.v','w')

    for bw in range(123, 124):
        run_egg(bw)
        print("egg ran\n")
        for filename in os.listdir(egg_output):
            fname = os.path.join(egg_output, filename)
            if not os.path.isfile(fname):
                continue
            shutil.copyfile(fname, tb_v_src)
            shutil.copyfile(fname, synthesis_v_src)
            run_testing(bw)
            run_synth(bw)







if __name__ == "__main__":
    main()