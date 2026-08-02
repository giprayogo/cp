class TrieNode:
    def __init__(self):
        self.end = False
        self.childs: dict[str, TrieNode] = {}


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        cursor = self.root
        for char in word:
            cursor = cursor.childs.setdefault(char, TrieNode())
        cursor.end = True

    def search(self, word: str) -> bool:
        cursor = self.root
        for char in word:
            cursor = cursor.childs.get(char, None)
            if cursor is None:
                return False
        return cursor.end

    def startsWith(self, prefix: str) -> bool:
        cursor = self.root
        for char in prefix:
            cursor = cursor.childs.get(char, None)
            if cursor is None:
                return False
        return True


# Your Trie object will be instantiated and called as such:
trie = Trie()
trie.insert("apple")
assert trie.search("apple")
assert not trie.search("app")
assert trie.startsWith("app")
trie.insert("app")
assert trie.search("app")
