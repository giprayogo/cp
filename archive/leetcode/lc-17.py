maps = {
    "2": "abc",
    "3": "def",
    "4": "ghi",
    "5": "jkl",
    "6": "mno",
    "7": "pqrs",
    "8": "tuv",
    "9": "wxyz",
}


class Solution:
    def letterCombinations(self, digits: str) -> list[str]:
        results = []
        config = []
        if not digits:
            return results

        def dfs(index):
            if index == len(digits):
                results.append("".join(config))
                return
            number = digits[index]
            for letter in maps[number]:
                config.append(letter)
                dfs(index + 1)
                config.pop()

        dfs(0)
        return results


s = Solution()
print(s.letterCombinations("23"))
print(s.letterCombinations(""))
print(s.letterCombinations("2"))
