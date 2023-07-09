class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result = []

        if (len(n) == 1):
            return [n[:]]

        for i in range(len(n)):
            n = n.pop(0)
            solveq = self.solveNQueens(n)

            for solve in solveq:
                solve.append(n)
            result.extend(solveq)
            n.append(n)

        return result
