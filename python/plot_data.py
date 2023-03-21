import numpy as np
import matplotlib.pyplot as plt


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


if __name__ == "__main__":
    print("Hello World")