from template_code.codetemplate import CodeTemplate
import linecache
import subprocess
import csv
import xml.dom.minidom
# import numpy as np
# import matplotlib.pyplot as plt

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

def run_synthesis():
    program = "run_tcl.sh"
    process = subprocess.Popen(['sh', program])
    process.wait()

# def plot_data(data):
#     fig = plt.figure()
#     ax1 = plt.axes(projection='3d')
#     #ax2 = plt.axes(projection='3d')
#     x = data[:,0]
#     y = data[:,1]
#     z1 = data[:,2]
#     z2 = data[:,3]

#     ax1.scatter3D(x, y, z1, c=z1, cmap='Greens')
#     #ax2.plot3D(x, y, z2, 'red')

#     plt.show()
#     return 0

def main():
    default_multiply = CodeTemplate("template_code/templates/verilog_mult_karatsuba")
    data = []
    mySet = {(0,0)}
    for IN2 in [64]:
        for IN1 in [64]:
            if (IN1, IN2) in mySet or (IN2, IN1) in mySet:
                continue 
            mySet.add((IN1, IN2))
            default_multiply.set_variables(IN1_WIDTH = IN1, IN2_WIDTH = IN2)
            default_multiply.write_code('tmp/mult.v')
            run_synthesis()
            LUTs, DSPs = extract_data('tmp/synth.xml')
            data.append([IN2, IN1, LUTs, DSPs])

    extract_data('tmp/synth.xml')

    with open('data.csv', 'w') as os:
        writer = csv.writer(os)
        for row in data:
            writer.writerow(row)


    # data = np.array(data)
    # np.savetxt("numpy_data.csv", data, delimiter=",")
    # plot_data(data)

if __name__ == "__main__":
    main()
