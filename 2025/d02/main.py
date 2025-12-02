import sys

def part1(puzzle_input: str):
    with open(puzzle_input, "r") as in_file:
        for pair in in_file.readline().split(","):

            l_bound = int(pair.split("-")[0])
            u_bound = int(pair.split("-")[1])
            for i in range(l_bound, u_bound + 1):
                i_str = str(i)
                for char_idx in range(1, int(len(i_str)/2) + 1): # only check if length is equal to or smaller then length/2
                    substring = i_str[:char_idx]
                    substring_idx = i_str.find((substring + substring), )
                    
                    if substring_idx != -1:
                        print(i_str)

    return "TOD"

def part2(puzzle_input: str):
    with open(puzzle_input, "r") as in_file:
        return "TODO"

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")