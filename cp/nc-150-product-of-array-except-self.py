"""Products of Array Except Self."""

import itertools


class Solution:
    def productExceptSelf(self, nums: list[int]) -> list[int]:
        """"""
        # For
        # nums = [a, b, c, d]
        # They want
        # [bcd, acd, abd, abc]
        # with O(n) complexity, and without division
        # nums length is between 2-1000 (inclusive), with each element between -20 and 20 (inclusive)
        #
        # Brute force way by looping for each element is O(n*n)

        # I can accumulate a prefix-array of length n-1, with initial value of 1
        # prefix = [1, a, ab, abc]
        # I also have suffix array
        # suffix = [1, d, dc, dcb] -> reverse [bcd, cd, d, 1]
        # if you multiply both arrray, then you got the answer in O(n+n+n) ~ O(n)
        # With additional O(n) space for the suffix array; the prefix array is just chain of iterators.
        # And another O(n) for the answer array
        n = len(nums)
        prefix = itertools.islice(itertools.accumulate(nums, lambda x, y: x * y, initial=1), n)
        # NOTE: See solution #3, I won't need the O(n) if I just looped the suffix
        suffix = list(itertools.accumulate(reversed(nums), lambda x, y: x * y, initial=1))[:-1]
        return [x * y for x, y in zip(prefix, reversed(suffix))]


# Default cases
assert Solution().productExceptSelf([1, 2, 4, 6]) == [48, 24, 12, 8]
assert Solution().productExceptSelf([-1, 0, 1, 2, 3]) == [0, -6, 0, 0, 0]
print("OK")
