class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        "something"
        # Given list of strings strs, I should group all anagrams
        # Constraints:
        # - 1 <= strs.length <= 1000
        # - 0 <= strs[i].length <= 100
        # - strs[i] is made up of lowercase English letters
        # I should aim for O(m*n) time complexity and O(m) space, where
        # m is the number of strings and n is the length of the longest string
        #
        # Given that space complexity I guess the solution should only store the indices,
        # before constructing the solution list.
        #
        # Trivial solution is to construct a list with counts of how many letters within each word,
        # each 26 in length.
        # But this would mean comparing _all_ strings, thus O(m^2*n)
        #
        # What could work is somehow having a hashable representation of each string,
        # then use it as a key to a dictionary.
        # The values of that dictionary would be our final answer.
        #
        # Sorting the letters won't work, since it will become O(n log n).
        #
        # Just use 26-length tuple!
        groups = {}
        for i, str in enumerate(strs):
            id = [0] * 26
            offset = 97
            for j in str:
                id[ord(j) - offset] += 1
            id = tuple(id)
            groups.setdefault(id, [])
            groups[id].append(i)
        return [[strs[i] for i in g] for g in groups.values()]


strs = ["act", "pots", "tops", "cat", "stop", "hat"]
print(Solution().groupAnagrams(strs))
strs = ["x"]
print(Solution().groupAnagrams(strs))
strs = [""]
print(Solution().groupAnagrams(strs))
