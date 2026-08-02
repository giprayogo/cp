class Solution:
    def findKthBit(self, n: int, k: int) -> str:
        # kth bit of tne nth string
        # Technically naive operation is doable!
        # But the string length grow as 2*(n+1), every time,
        # so it scales by 2**n
        # What I should do is finding the pattern? or...
        # For example, we know that the length of the nth string is 2(Sn-1) + 1,
        # One easy way in python is to do actual bitflip, since in python we get unlimited integer
        # Wait: k is 1-indexed, and so is nth; annoying!
        # 0
        # 0 1 2
        # 0 1 2 3 4
        #
        # I don't like 1-index
        k -= 1
        n -= 1

        sn = 1
        for _ in range(n):
            sn = 2 * sn + 1

        # Map the bytes index, recursively, count how many bit flips
        nflips = 0
        while True:
            if k == sn // 2:
                break
            elif k < sn // 2:
                pass  # no change for left hand side
            else:
                k = sn - k - 1
                # Flips only happen on the right side
                nflips += 1

            sn = (sn - 1) // 2

        print(f"{sn=}, {nflips=}")
        return str(int((nflips % 2) == (sn == 1)))


print(Solution().findKthBit(3, 1))
print(Solution().findKthBit(4, 11))
print(Solution().findKthBit(1, 1))
print(Solution().findKthBit(3, 2))  # exception when it goes to the last pivot
