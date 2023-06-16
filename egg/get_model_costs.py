import subprocess
import time
import os

def run_egg(bw):
    process = subprocess.Popen(['sh', './run_egg.sh', str(bw)])
    process.wait()

def main():
    start_prog = time.time()
    times_and_sizes = []
    
    for bw in (2**n for n in range(5,11)):
        start_egg = time.time()
        run_egg(bw)
        end_egg = time.time()
        times_and_sizes.append((bw, end_egg-start_egg, len(os.listdir('./output/verilog'))))
    end_prog = time.time()
    
    with open("egg_times.txt", 'w') as fs:
        fs.write("prog time: {}\n".format(end_prog-start_prog))
        for x,y,z in times_and_sizes:
            fs.write("{},{},{}\n".format(x,y,z))

    
if __name__ == "__main__":
    main()
