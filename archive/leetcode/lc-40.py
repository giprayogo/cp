class Solution:
    def combinationSum2(self, candidates: list[int], target: int) -> list[list[int]]:
        results = []
        config = []
        rejects = []

        def backtrack(index):
            if sum(config) == target:
                results.append(config.copy())
                return
            elif sum(config) > target:
                return
            if index == len(candidates):
                return

            if candidates[index] not in rejects:
                config.append(candidates[index])
                backtrack(index + 1)
                config.pop()

            rejects.append(candidates[index])
            backtrack(index + 1)
            rejects.pop()

        backtrack(0)
        return results


s = Solution()
print(s.combinationSum2([10, 1, 2, 7, 6, 1, 5], 8))
print(s.combinationSum2([2, 5, 2, 1, 2], 5))
