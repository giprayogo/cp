from typing import Optional, Self
from collections import deque


# LC Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __iter__(self):
        nodes = deque()
        nodes.append(self)
        while nodes:
            if all(x is None for x in nodes):
                return
            node = nodes.popleft()
            if node:
                yield node
                nodes.append(node.left)
                nodes.append(node.right)
            else:
                yield None

    def __getitem__(self, key: int) -> Optional[Self]:
        self_as_list = list(self)
        return self_as_list[key]

    @classmethod
    def from_list(cls, node_list: list[int | None]) -> Optional[Self]:
        nodes = [cls(x) if x is not None else None for x in node_list]
        length = len(nodes)
        for i, e in enumerate(nodes):
            if e is None:
                continue

            left_index, right_index = 2 * i + 1, 2 * i + 2
            if left_index < length and nodes[left_index] is not None:
                e.left = nodes[left_index]
            if right_index < length and nodes[right_index] is not None:
                e.right = nodes[right_index]
        return nodes[0] if nodes else None

    def __str__(self):
        return str([x.val if x is not None else "null" for x in list(self)])


class Solution:
    # Misinterpretation of the question: should include things not derived
    # fron the right-most branch!
    # def rightSideView(self, root: Optional[TreeNode]) -> list[int]:
    #     rights = []
    #     cursor = root
    #     while cursor:
    #         rights.append(cursor.val)
    #         if cursor.right:
    #             cursor = cursor.right
    #         elif cursor.left:
    #             cursor = cursor.left
    #         else:
    #             break
    #     return rights
    def rightSideView(self, root: Optional[TreeNode]) -> list[int]:
        rights = []
        level = deque()
        if root:
            level.append(root)
        while level:
            rights.append(level[-1].val)
            for _ in range(len(level)):
                node = level.popleft()
                if node.left:
                    level.append(node.left)
                if node.right:
                    level.append(node.right)
        return rights


s = Solution()

a = TreeNode.from_list([1, 2, 3, None, 5, None, 4])
print(a)
print(s.rightSideView(a))
print("=" * 88)
b = TreeNode.from_list([1, None, 3])
print(b)
print(s.rightSideView(b))
print("=" * 88)
c = TreeNode.from_list([])
print(c)
print(s.rightSideView(c))
print("=" * 88)
d = TreeNode.from_list([1, 2, 3, 4])
print(s.rightSideView(d))
print("=" * 88)
