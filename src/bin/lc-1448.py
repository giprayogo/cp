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
    def goodNodes(self, root: TreeNode) -> int:
        count = 0  # Note: the count can be dfs-ed too

        def dfs(node: TreeNode, maxval):
            nonlocal count
            if node.val >= maxval:
                count += 1
                maxval = node.val
            if node.left:
                dfs(node.left, maxval)
            if node.right:
                dfs(node.right, maxval)

        dfs(root, root.val)
        return count


s = Solution()
a = TreeNode.from_list([3, 1, 4, 3, None, 1, 5])
print(a)
print(s.goodNodes(a))
print("=" * 88)

b = TreeNode.from_list([3, 3, None, 4, 2])
print(b)
print(s.goodNodes(b))
print("=" * 88)

c = TreeNode.from_list([1])
print(c)
print(s.goodNodes(c))
print("=" * 88)

# TODO: fix tree generation
# d = TreeNode.from_list([2, None, 4, 10, 8, None, None, 4])
# print(d)
# print(s.goodNodes(d))
# print("=" * 88)
