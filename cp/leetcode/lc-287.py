# Notes from personal wiki
# My take on the hare-tortoise algorithm and position (short)
# The path before the cycle is a distraction.
# Pointer that is 2x with 1x always meet on the same spot.
# When path before cycle is 0 length, this is obvious, for path longer than that,
# imagine folding the path into the cycle itself, now you can see that both will
# meet at the same spot mapped into the cycle. Hence starting new pointer will
# at the same speed will make the slow pointer meet the new slow pointer at the cycle entry.
# Not self obvious! For me! And there are no good explanation online!


class Solution(object):
    # Credit neetcode for the hint
    # I solved it before, but I didn't think of fast-slow pointer solution!
    # Technically this is correct: constant space; my previous solution was
    # accepted but technically incorrect!
    # See third constraint, for the hint!
    def findDuplicate(self, nums):
        fast = slow = 0
        while True:
            slow = nums[slow]
            fast = nums[nums[fast]]
            if slow == fast:
                break
        # Now find the branch
        # Spam the cycle with pointers
        # spams = []
        # while slow not in spams:
        #     spams.append(slow)
        #     slow = nums[slow]
        # Move another pointer from beginning until found
        # Missing trick from neetcode: apparently just this one is ok?
        fast = 0
        # while fast not in spams:
        while True:
            slow = nums[slow]
            fast = nums[fast]
            if fast == slow:
                return fast


s = Solution()
a = [1, 3, 4, 2, 2]
print(s.findDuplicate(a))
b = [3, 1, 3, 4, 2]
print(s.findDuplicate(b))
