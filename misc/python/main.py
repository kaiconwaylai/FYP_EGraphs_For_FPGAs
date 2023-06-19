from template_code.codetemplate import CodeTemplate
import linecache
import subprocess
import csv
import xml.dom.minidom

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

def run_vivado():
    program = "run_tcl.sh"
    process = subprocess.Popen(['sh', program])
    process.wait()

def main():
    default_multiply = CodeTemplate("template_code/templates/verilog_mult_karatsuba")
    data = []
    mySet = {(0,0)}
    for IN2 in range(18,129):
        for IN1 in range(IN2,IN2+1):
            if (IN1, IN2) in mySet or (IN2, IN1) in mySet:
                continue 
            mySet.add((IN1, IN2))
            
            default_multiply.set_variables(IN1_WIDTH = IN1, IN2_WIDTH = IN2)
            default_multiply.write_code('tmp/mult.v')
            
            run_vivado()
            
            LUTs, DSPs = extract_data('tmp/synth.xml')
            print("Width:{}, LUTS:{}, DSPs:{}".format(IN1,LUTs,DSPs))
            
            data.append([IN2, IN1, LUTs, DSPs])
            with open('data.csv', 'a') as os:
                writer = csv.writer(os)
                writer.writerow([IN2, IN1, LUTs, DSPs])

    with open('data_complete.csv', 'w') as os:
        writer = csv.writer(os)
        for row in data:
            writer.writerow(row)

if __name__ == "__main__":
    main()
