import subprocess
import time



def run_egg(bw):
    process = subprocess.Popen(['sh', './run_egg.sh', str(bw)])
    process.wait()

def main():
    start_prog = time.time()
    egg_times = []
    for bw in range(32,257):
        start_egg = time.time()
        run_egg(bw)
        end_egg = time.time()
        egg_times.append((bw, end_egg-start_egg))
    end_prog = time.time()
    
    with open("egg_times.txt", 'w') as fs:
        fs.write("prog time: {}".format(end_prog-start_prog))
        for x,y in egg_times:
            fs.write("{},{}".format(x,y))




if __name__ == "__main__":
    main()
