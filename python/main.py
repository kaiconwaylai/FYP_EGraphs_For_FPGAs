import sys
sys.path.insert(0, '/home/kc319/Documents/FYP_EGraphs_For_FPGAs/python/template_code')
from CodeTemplate import CodeTemplate
import pandas as pd

def extract_data():
    # with open('/home/kc319/Documents/FYP_EGraphs_For_FPGAs/vivado/tmp/synth.rpt', 'r') as rpt:
    #     report = rpt.read()
    
    data = pd.read_fwf('/home/kc319/Documents/FYP_EGraphs_For_FPGAs/vivado/tmp/synth.rpt', delimiter='|')
    display(data)


# | LUT2     |   47 |                 CLB |
# | DSP48E2  |    4 |          Arithmetic |
# | LUT1     |    1 |                 CLB |

def main():
    default_multiply = CodeTemplate("template_code/templates/verilog_mult")
    print(default_multiply)
    extract_data()

if __name__ == "__main__":
    main()