import sys
import os

def check_file(filename: str):
    if not os.path.isfile(filename):
        print(f"ERROR file does not exist: '{filename}'")
        sys.exit()

def read_data(filename: str):
    return []


args = sys.argv
if len(args) != 3:
    print("Script needs 2 arguments: [SLAB PATH] [SOLUTION PATH]")
    sys.exit() 

slab_file = args[1]
check_file(slab_file)
sol_file = args[2]
check_file(sol_file)

