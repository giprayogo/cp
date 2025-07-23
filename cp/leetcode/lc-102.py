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
    # def levelOrder(self, root: Optional[TreeNode]) -> list[list[int]]:
    #     levels = []
    #     nodes = deque()
    #     backstack = deque()
    #     nodes.append(root)
    #     while nodes:
    #         level = []
    #         while nodes:
    #             node = nodes.popleft()
    #             if node:
    #                 level.append(node.val)
    #                 backstack.append(node.left)
    #                 backstack.append(node.right)
    #         nodes, backstack = backstack, nodes
    #         if level:
    #             levels.append(level)
    #     return levels

    # A bit nicer solution from NeetCode:
    # - Don't add None to queue
    # - Use single deque, consuming levels by consuming by initial deque length!
    def levelOrder(self, root: Optional[TreeNode]) -> list[list[int]]:
        levels = []
        nodes = deque()
        if root:
            nodes.append(root)
        while nodes:
            level = []
            for _ in range(len(nodes)):
                node = nodes.popleft()
                level.append(node.val)

                if node.left:
                    nodes.append(node.left)
                if node.right:
                    nodes.append(node.right)
            levels.append(level)
        return levels


s = Solution()

a = TreeNode.from_list([3, 9, 20, None, None, 15, 7])
print(a)
print(s.levelOrder(a))

print("=" * 88)
b = TreeNode.from_list([1])
print(b)
print(s.levelOrder(b))

print("=" * 88)
c = TreeNode.from_list([])
print(c)
print(s.levelOrder(c))
