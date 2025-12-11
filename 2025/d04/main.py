from typing import List
import sys

def get_neighbour_amount(pos_x: int, pos_y: int, board: List[List[str]]) -> int:
    neighbours_amount = 0

    for i in range(-1, 2):
        neighbour_x = pos_x + i
        for j in range(-1, 2):
            neighbour_y = pos_y + j
            
            # Checking current pos
            if neighbour_x == pos_x and neighbour_y == pos_y:
                continue

            # Checking oob
            if (neighbour_x < 0 or neighbour_x >= len(board) 
                or neighbour_y < 0 or neighbour_y >= len(board[neighbour_x])):
                continue

            neighbours_amount += int(board[neighbour_x][neighbour_y] == "@")

    return neighbours_amount

def print_board_with_active_space(pos_x, pos_y, board, special_char):
    for x in range(0, len(board)):
        for y in range(0, len(board[x])):
            if x == pos_x and y == pos_y:
                print(special_char, end="")
            else:
                print(board[x][y], end="")
        print()


def part1(puzzle_input: str):
    board = []
    qualifying_spaces = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            board.append([char for char in line if char != "\n"])

    for x in range(0, len(board)):
        for y in range(0, len(board[x])):
            if board[x][y] != "@":
                continue

            qualifying_spaces += int(get_neighbour_amount(x, y, board) < 4)

    return qualifying_spaces

def part2(puzzle_input: str):
    board = []
    qualifying_spaces = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            board.append([char for char in line if char != "\n"])

    prev_qualifying_space = qualifying_spaces
    updated_board = board
    while True:
        for x in range(0, len(board)):
            for y in range(0, len(board[x])):
                updated_board[x][y] = board[x][y]
                if board[x][y] != "@":
                    continue

                if get_neighbour_amount(x, y, board) < 4:
                    updated_board[x][y] = "."
                    qualifying_spaces += 1

        if prev_qualifying_space == qualifying_spaces:
            break
        board = updated_board
        prev_qualifying_space = qualifying_spaces

    return qualifying_spaces

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")
