class Solution:
    # def permute(self, nums: list[int]) -> list[list[int]]:
    #     results = []

    #     def loop(config):
    #         if len(config) == len(nums):
    #             results.append(config)
    #             return
    #         for i in nums:
    #             if i not in config:
    #                 loop(config + [i])

    #     loop([])
    #     return results

    # Neat from neetcode:
    # - the function itself is the recursive
    # - no need for explicit if by popping nums around (not sure if better)
    # def permute(self, nums: list[int]) -> list[list[int]]:
    #     results = []

    #     if len(nums) == 1:
    #         return [nums.copy()]

    #     for i in range(len(nums)):
    #         n = nums.pop(0)

    #         result = self.permute(nums)
    #         for r in result:
    #             r.append(n)
    #         results.extend(result)
    #         nums.append(n)
    #     return results

    # Other (@leetcode) do suceed framing this into regular backtrack
    def permute(self, nums: list[int]) -> list[list[int]]:
        results = []
        config = []

        def backtrack():  # Do not so fixated on this backtrack having a parameter
            if len(config) == len(nums):
                results.append(config.copy())
            for i in nums:
                if i not in config:
                    config.append(i)
                    backtrack()
                    config.pop()

        backtrack()
        return results


s = Solution()
print(s.permute([1, 2, 3]))
print(s.permute([0, 1]))
print(s.permute([1]))
