import pandas as pd

def csv_to_hashmap(input_file):
    data = pd.read_csv(input_file)
    with open('out.txt', 'w') as dst:
        for i,row in data.iterrows():
            dst.write("(({},{}), fpga::Cost{{dsp: {}, lut: {}}}),\n".format(row[0],row[1],row[3],row[2]))



csv_to_hashmap('slice_data.csv')
