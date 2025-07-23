class Solution:
    def exist(self, board: list[list[str]], word: str) -> bool:
        # NOTE: Technically this part is optional (can also start from everything)
        starts = []
        visited = []
        for i, row in enumerate(board):
            for j, char in enumerate(row):
                if char == word[0]:
                    starts.append((i, j))
        ni = len(board)
        nj = len(board[0])  # rectangular

        # def backtrack(index, i, j, visited):
        def backtrack(index, i, j):
            # NOTE: Remember that condition is short-circuited: I can join it with the later
            if i >= ni or j >= nj or i < 0 or j < 0:
                return False
            char_at = board[i][j]
            if (i, j) in visited or char_at != word[index]:
                return False

            visited.append((i, j))
            if index == len(word) - 1:
                return True

            # Insight from neetcode: I can use -single- visited if I pop it here!
            # return (
            #     backtrack(index + 1, i + 1, j, visited.copy())
            #     or backtrack(index + 1, i - 1, j, visited.copy())
            #     or backtrack(index + 1, i, j + 1, visited.copy())
            #     or backtrack(index + 1, i, j - 1, visited.copy())
            # )
            result = (
                backtrack(index + 1, i + 1, j)
                or backtrack(index + 1, i - 1, j)
                or backtrack(index + 1, i, j + 1)
                or backtrack(index + 1, i, j - 1)
            )
            visited.pop()
            return result

        # In neetcode solution there's a bit of optimization whether
        # to start from back/front based on letter frequency... not mandatory I guess.

        for i, j in starts:
            # k = backtrack(0, i, j, [])
            k = backtrack(0, i, j)
            if k:
                return True
        return False


s = Solution()
board = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
assert s.exist(board, "ABCCED")
assert s.exist(board, "SEE")
assert not s.exist(board, "ABCB")
board = [["a", "a"]]
assert not s.exist(board, "aaa")
board = [["a", "b"], ["c", "d"]]
assert s.exist(board, "cdba")
board = [["A", "B", "C", "E"], ["S", "F", "E", "S"], ["A", "D", "E", "E"]]
assert s.exist(board, "ABCESEEEFS")
