# from collections import deque
class Solution:
    # I sure seems to love BFS for some reasons
    # def subsets(self, nums: list[int]) -> list[list[int]]:
    #     stack = deque()
    #     subsets = [[x] for x in nums]
    #     stack.extend(subsets)
    #     subsets.append([])
    #     while stack:
    #         for i in range(len(stack)):
    #             subset = stack.popleft()
    #             for num in nums:
    #                 if num > subset[-1]:
    #                     _subset = subset + [num]
    #                     subsets.append(_subset)
    #                     stack.append(_subset)
    #     return subsets

    # NeetCode does show DFS also possible (my impl is quite a bit different though)
    # And not relying on ordering at all so technically more general...
    # def subsets(self, nums: list[int]) -> list[list[int]]:
    #     def dfs(node) -> list:
    #         children = [node]
    #         for num in nums:
    #             if (
    #                 not node or num > node[-1]
    #             ):  # This rule: relies on int being an ordered set
    #                 children += dfs(node + [num])
    #         return children

    #     return dfs([])

    # Try this way also: instead of having a rule based on comparison,
    # all accept / reject are part of the combination
    def subsets(self, nums: list[int]) -> list[list[int]]:
        subsets = []
        config = []

        def dfs(index):
            if index >= len(nums):
                subsets.append(config.copy())
                return
            # Branching is achieved by recursion here
            config.append(nums[index])
            dfs(index + 1)
            config.pop()
            dfs(index + 1)

        dfs(0)
        return subsets


s = Solution()
a = [1, 2, 3]
print(s.subsets(a))
b = [0]
print(s.subsets(b))
