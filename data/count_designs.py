import pandas as pd
import csv
import matplotlib.pyplot as plt
import numpy as np
from collections import Counter, defaultdict

def main():

    data = pd.read_csv('small_Steps.csv')
    
    bw = 32
    # with open('newdadad.txt', 'w') as fs:
    #     for group in data.groupby(['WIDTH'])['DSPs'].apply(list):
    #         #print(group)
    #         dsps = list(dict.fromkeys(group))
    #         dsps.sort()
    #         lowest = dsps[0]
    #         highest = dsps[-1]
    #         count = len(dsps)
    #         dsps = [dsps[n]/highest for n in range(0,len(dsps))]
            
    #         # diffs = [dsps[n]-dsps[n-1] for n in range(1,len(dsps))]
    #         # score = 0.
    #         # for x in diffs:
    #         #     score += (x/highest)**2
    #         # score = score/(1.0/(count-1.0))
            
    #         # fs.write("{} , {}\n".format(bw, score))
    #         # bw += 1

    dict = {}
    x = data[['WIDTH','LUTs', 'DSPs']].groupby(['WIDTH'])
    for name, grouped in x:
        lst = []
        for _,y in grouped.iterrows():
            lst.append((y['LUTs'], y['DSPs']))
        dict[name[0]] = lst

    fig, ax = plt.subplots()
    for bw, vals in dict.items():
        vals.sort(key = lambda i:i[1])
        max_dsps = max(vals, key=lambda item: item[1])[1]
        max_luts = max(vals, key=lambda item: item[0])[0]
        normalised = [(x/max_luts, y/max_dsps) for x,y in vals]
        luts, dsps = zip(*normalised)

        plt.xlabel('DSPs', fontsize=10)
        plt.ylabel('LUTs', fontsize=10)
        plt.plot(dsps, luts, label = str(bw) + ' bit multiplier', marker='o')
        plt.legend()


    df = pd.read_csv('512_more.csv')
    df = df.sort_values(by=['DSPs'])
    print(df)
    ax = df.plot(kind='line', x='DSPs',y='LUTs',color='blue', marker='o', legend=False)
    ax.set_ylabel('LUTs')
    
    # df = pd.read_csv('64_small_step.csv')
    # df = df.sort_values(by=['DSPs'])
    # print(df)
    # ax = df.plot(kind='line', x='DSPs',y='LUTs',color='blue', marker='o', legend=False)
    # ax.set_ylabel('LUTs')
    
    plt.show()
if __name__ == "__main__":
    main()