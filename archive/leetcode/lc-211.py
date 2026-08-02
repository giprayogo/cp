from collections import deque


class TrieNode:
    def __init__(self):
        self.term = False
        self.children: dict[str, TrieNode] = {}


class WordDictionary:
    def __init__(self):
        self.root = TrieNode()

    def addWord(self, word: str) -> None:
        cursor = self.root
        for char in word:
            cursor = cursor.children.setdefault(char, TrieNode())
        cursor.term = True

    # NOTE: BFS
    def search(self, word: str) -> bool:
        cursors: deque[TrieNode] = deque()
        cursors.append(self.root)
        for char in word:
            for _ in range(len(cursors)):
                cursor = cursors.popleft()
                if char == ".":
                    for child in cursor.children.values():
                        cursors.append(child)
                else:
                    child = cursor.children.get(char, None)
                    if child:
                        cursors.append(child)

        for cursor in cursors:
            if cursor.term:
                return True
        return False


word_dictionary = WordDictionary()
word_dictionary.addWord("bad")
word_dictionary.addWord("dad")
word_dictionary.addWord("mad")
assert not word_dictionary.search("pad")
assert word_dictionary.search("bad")
assert word_dictionary.search(".ad")
assert word_dictionary.search("b..")
