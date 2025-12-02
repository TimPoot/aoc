import sys

def part1(puzzle_input: str):
    sum_invalid = 0

    with open(puzzle_input, "r") as in_file:
        for pair in in_file.readline().split(","):

            l_bound = int(pair.split("-")[0])
            u_bound = int(pair.split("-")[1])
            for i in range(l_bound, u_bound + 1):
                i_str = str(i)
                for char_idx in range(1, int(len(i_str)/2) + 1): # only check if length is equal to or smaller then length/2
                    substring = i_str[:char_idx]
                    if i_str.replace((substring + substring), "") == "":
                        sum_invalid += i
                        break

    return sum_invalid

def part2(puzzle_input: str):
    sum_invalid = 0

    with open(puzzle_input, "r") as in_file:
        for pair in in_file.readline().split(","):

            l_bound = int(pair.split("-")[0])
            u_bound = int(pair.split("-")[1])
            for i in range(l_bound, u_bound + 1):
                i_str = str(i)

                for char_idx in range(1, int(len(i_str)/2) + 1): # only check if length is equal to or smaller then length/2
                    substring = i_str[:char_idx]
                    invalid_found = False

                    for n in range(0, len(i_str)):
                        n_times = n + 2
                        substring_mult = substring * n_times

                        if len(substring_mult) > len(i_str):
                            break

                        if i_str.replace(substring_mult, "") == "":
                            invalid_found = True
                            sum_invalid += i
                            break

                    if invalid_found:
                        break

    return sum_invalid

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")