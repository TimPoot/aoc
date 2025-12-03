import sys

def part1(puzzle_input: str):
    total_joltage = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            batteries = [int(value) for value in line if value != "\n"]

            for joltage in range(9, 0, -1):
                try:
                    joltage_idx = batteries.index(joltage)
                except:
                    continue

                batteries_left = batteries[joltage_idx + 1:]
                if batteries_left == []:
                    continue

                max_joltage = int(str(joltage) + str(max(batteries_left)))
                total_joltage += max_joltage
                break

    return total_joltage

def part2(puzzle_input: str):
    total_joltage = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            batteries = [int(value) for value in line if value != "\n"]

            digits_required = 12
            current_jolt_string = ""
            batteries_left = batteries
            while digits_required > 0:
                max_joltage = max(batteries_left)
                max_joltage_idx = batteries_left.index(max_joltage)

                if len(batteries_left) - max_joltage_idx - 1 < digits_required:
                    continue


    return total_joltage

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")
