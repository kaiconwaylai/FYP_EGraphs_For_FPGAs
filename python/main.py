from template_code.codetemplate import CodeTemplate
import pandas as pd
import linecache
import subprocess
import numpy as np
import matplotlib.pyplot as plt

def extract_data(path):
    lines = [37, 90]
    rows = []
    with open(path, 'r') as fp:        
        for i,line in enumerate(fp):
        # check line number
            if i in lines:
                rows.append(line.strip())
            elif i > 95:
                break
            i += 1
    LUT_row = rows[0].split('|')
    DSP_row = rows[1].split('|')
    return int(LUT_row[2]), int(DSP_row[2])

def run_synthesis():
    program = "run_tcl.sh"
    process = subprocess.Popen(['sh', program])
    process.wait()

def plot_data(data):
    fig = plt.figure()
    ax1 = plt.axes(projection='3d')
    #ax2 = plt.axes(projection='3d')
    x = data[:,0]
    y = data[:,1]
    z1 = data[:,2]
    z2 = data[:,3]

    ax1.scatter3D(x, y, z1, c=z1, cmap='Greens')
    #ax2.plot3D(x, y, z2, 'red')

    plt.show()
    return 0

def main():
    default_multiply = CodeTemplate("template_code/templates/verilog_mult")
    data = []
    mySet = {(0,0)}
    for IN2 in [32,64]:
        for IN1 in [32,64]:
            if (IN1, IN2) in mySet or (IN2, IN1) in mySet:
                continue 
            mySet.add((IN1, IN2))
            default_multiply.set_variables(IN1_WIDTH = IN1, IN2_WIDTH = IN2, OUT_WIDTH = IN1+IN2)
            default_multiply.write_code('tmp/mult.v')
            run_synthesis()
            LUTs, DSPs = extract_data('tmp/synth.rpt')
            data.append([IN1, IN2, LUTs, DSPs])

    data = np.array(data)
    np.savetxt("data.csv", data, delimiter=",")
    #plot_data(data)

if __name__ == "__main__":
    main()
