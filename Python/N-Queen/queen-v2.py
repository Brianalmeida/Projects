class Solution:

    """
    First left example: [1, 3, 0, 2]
    Second right example: [2, 0, 3, 1]
    """

    def solveNQueens(self, n: int) -> List[List[str]]:
        solutions = []
        state = []
        self.search(state, solutions, n)
        return solutions


    def is_valid(self, state, n):
        return len(state) == n

    def get_queens(self, state, n):
        if not state:
            return range(n)

        # find the next position
        position = len(state)
        queens = set(range(n))

        for row, col in enumerate(state):
            queens.discard(col)
            dist = position - row

            queens.discard(col + dist)
            queens.discard(col - dist)

        return queens

    def search(self, state, solution, n):
        if self.is_valid(state, n):
            state_string = self.state_to_str(state, n)
            solutions.append(state_string)
            return

        for queen in self.get_queens(state, n):

            state.append(queen)
            self.search(state, solutions, n)
            state.pop()

    def state_to_str(self, state, n):
        ret = []
        for i in state:
            string = '.' * i + 'Q' + '.' * (n - i - 1)
            ret.append(string)
        return ret
