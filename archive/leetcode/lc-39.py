class Solution:
    def combinationSum(self, candidates: list[int], target: int) -> list[list[int]]:
        results = []  # Technically this one too no?
        config = []  # Also this can be passed in the DFS itself

        # Neetcode optimization: I can pass away the sum
        # def dfs(index):
        def dfs(index, total):
            if total == target:
                results.append(config.copy())
                return
            elif total > target:
                return
            elif index >= len(candidates):
                return

            config.append(candidates[index])
            dfs(index, total + candidates[index])
            config.pop()
            dfs(index + 1, total)

        dfs(0, 0)
        return results


s = Solution()
print(s.combinationSum([2, 3, 6, 7], 7))
print(s.combinationSum([2, 3, 5], 8))
print(s.combinationSum([2], 1))
