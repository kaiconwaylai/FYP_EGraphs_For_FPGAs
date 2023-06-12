import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def main():

    normal = pd.read_csv('data/1-128normal.csv')
    #karatsuba = pd.read_csv('data/1-128karatsuba.csv')
    LUT_Graph = normal.plot(x = 'IN1', y = 'LUTs', label = 'Default', title = 'LUT usage against Input Width')
    #karatsuba.plot(ax = LUT_Graph, x = 'IN1', y = 'LUTs', ylabel = 'LUTs', label = 'Karatsuba')
    DSP_Graph = normal.plot(x = 'IN1', y = 'DSPs', label = 'Default', title = 'DSP usage against Input Width')
    #karatsuba.plot(ax = DSP_Graph, x = 'IN1', y = 'DSPs', ylabel = 'DSPs', label = 'Karatsuba')

    plt.show()

    return 0

if __name__ == "__main__":
    main()