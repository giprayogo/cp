class Solution:
    def subsetsWithDup(self, nums: list[int]) -> list[list[int]]:
        results = []
        config = []
        rejects = []

        def backtrack(index):
            if index == len(nums):
                results.append(config.copy())
                return

            n = nums[index]
            if n not in rejects:
                config.append(n)
                backtrack(index + 1)
                config.pop()
            rejects.append(n)
            backtrack(index + 1)
            rejects.pop()

        backtrack(0)
        return results

    # Neetcode's solution: instead of building reject list,
    # sort first, then only backtrack once when rejected.
    # basically identical results!


s = Solution()
print(s.subsetsWithDup([1, 2, 2]))
print(s.subsetsWithDup([0]))
print(s.subsetsWithDup([4, 4, 4, 1, 4]))
print(s.subsetsWithDup([4, 1, 4]))
