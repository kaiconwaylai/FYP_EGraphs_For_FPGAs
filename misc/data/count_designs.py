import pandas as pd
import csv
import matplotlib.pyplot as plt
import numpy as np
from collections import Counter, defaultdict

def main():

    data = pd.read_csv('small_Steps.csv')
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
    
def main2():
    norm = pd.read_csv('32-256.csv')
    small = pd.read_csv('small_Steps.csv')
    costs = pd.read_csv('costs.txt')

    dic1 = {}
    dic2 = {}
    dic3 = {}
    
    default_512 = (24230,900)
    default_1024 = (1065019,0)
    
    for diction, df in [(dic1,norm), (dic2, small), (dic3, costs)]:
        x = df[['WIDTH','LUTs', 'DSPs']].groupby(['WIDTH'])
        for name, grouped in x:
            lst = []
            for _,y in grouped.iterrows():
                lst.append((y['LUTs'], y['DSPs']))
            diction[name[0]] = lst
    
    for bw in [1024]:
        normal = dic1[bw];         
        normal.sort(key = lambda i:i[1])
        more = dic2[bw]; more.sort(key = lambda i:i[1])
        cos = dic3[bw]; cos.sort(key = lambda i:i[1])
    
        plt.figure(bw)
        plt.xlabel('DSPs', fontsize=10)
        plt.ylabel('LUTs', fontsize=10)
        
        l1, d1 = zip(*normal)
        l2, d2 = zip(*more)
        l3, d3 = zip(*cos)
        
        plt.plot(d1, l1, label = 'Default Alpha', marker='s')
        plt.plot(d2, l2, label = 'Small Alpha', marker='.')
        plt.plot(d3, l3, label = 'Cost Model', marker='x')
        if bw == 512:
            plt.plot(900, 24230, label = 'Default Multiplier', marker='D')
        elif bw == 1024:
            #plt.plot(0, 1065019, label = 'Default Multiplier', marker='D')
            a = 1
            
        #plt.plot(d1[-1], l1[-1], label='Default Multiplier', marker='D')
        plt.legend()

    plt.show()
        
        
    
if __name__ == "__main__":
    main2()