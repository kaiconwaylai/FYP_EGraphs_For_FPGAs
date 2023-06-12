import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def main():

    normal = pd.read_csv('data/1-128normal.csv')
    karatsuba = pd.read_csv('data/1-128karatsuba.csv')
    # LUT_Graph = normal.plot(x = 'IN1', y = 'LUTs', label = 'Default', title = 'LUT usage against Input Width')
    # karatsuba.plot(ax = LUT_Graph, x = 'IN1', y = 'LUTs', ylabel = 'LUTs', label = 'Karatsuba')
    # DSP_Graph = normal.plot(x = 'IN1', y = 'DSPs', label = 'Default', title = 'DSP usage against Input Width')
    # karatsuba.plot(ax = DSP_Graph, x = 'IN1', y = 'DSPs', ylabel = 'DSPs', label = 'Karatsuba')
    fig, ax1 = plt.subplots()

    color = 'tab:red'
    ax1.set_xlabel('IN1')
    ax1.set_ylabel('DSPs', color=color)
    ax1.plot(normal.IN1, normal.DSPs, color=color)
    ax1.tick_params(axis='y', labelcolor=color)

    ax2 = ax1.twinx()  # instantiate a second axes that shares the same x-axis

    color = 'tab:blue'
    ax2.set_ylabel('LUTs', color=color)  # we already handled the x-label with ax1
    ax2.plot(normal.IN1, normal.LUTs, color=color)
    ax2.tick_params(axis='y', labelcolor=color)

    fig.tight_layout()  # otherwise the right y-label is slightly clipped
    plt.title('Resource Utilisation for Standard Multiplier')
    plt.show()

    return 0

if __name__ == "__main__":
    main()