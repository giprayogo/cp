class Solution:
    # def partition(self, s: str) -> list[list[str]]:
    #     results = []
    #     config = [[s[0]]]

    #     def backtrack(index):
    #         if index == len(s):
    #             if config[-1] == config[-1][::-1]:
    #                 results.append(["".join(x) for x in config])
    #             return

    # NOTE: In my way this part is perhaps particularly expensive
    # Can add loop too! Also I used list instead of string > more space?
    #         char = s[index]
    #         config[-1].append(char)
    #         backtrack(index + 1)
    #         config[-1].pop()

    #         if config[-1] == config[-1][::-1]:
    #             config.append([char])
    #             backtrack(index + 1)
    #             config.pop()

    #     backtrack(1)
    #     return results

    # neetcode: same idea but better?
    # instead of drilling one by one,
    # only recurse on next partition
    def partition(self, s: str) -> list[list[str]]:
        results = []
        config = []

        def backtrack(index):
            if index == len(s):
                results.append(config.copy())
                return
            for j in range(index, len(s)):
                if self.is_pali(s[index : j + 1]):
                    config.append(s[index : j + 1])
                    backtrack(j + 1)
                    config.pop()

        backtrack(0)
        return results

    # neetcode: I guess this in theory more optimized than default comparison
    def is_pali(self, s):
        i, j = 0, len(s) - 1
        while i < j:
            if s[i] != s[j]:
                return False
            i, j = i + 1, j - 1
        return True


s = Solution()
print(s.partition("aab"))
print(s.partition("aa"))
