class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        def is_valid(state):
            return true

        def get_queens(state):
            return []

        def search(state, solution):
            if is_valid(state):
                solution.append(state[:])

            for queen in queens(state):
                state.add(queen)
                search(state, solutions)
                state.remove(queen)

        def solve():
            solutions = []
            state = set()
            search(state, solutions)
            return solutions
