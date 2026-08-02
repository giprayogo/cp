# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def isValidBST(self, root: TreeNode | None) -> bool:
        # Valid BST is defined as:
        # - left subtree contain only nodes with key strictly less than the node's key
        # - right subtree strictly greater
        # - left and right subtree must also binary search tree (i.e. above definition)
        #
        # Constraints:
        # - Number of nodes is within [1, 10^4]
        # - -2^31 <= node.val <= 2^3 -1
        #
        # Observation:
        # - Any nodes without any subnodes is automatically valid
        # - Try DFS route:
        #   - A node is valid if
        #     - Both subnodes are valid
        #     - Max value from left tree is less than the current node
        #     - Min value from right tree is more than the current node
        # - I assume that an empty tree is a valid BST (might be wrong)
        # - I assume also that the tree does not contains two same integer (because it says _strictly_)
        #   - Wrong assumption: should return False on =
        #
        # The "brute force" DFS
        # is_valid, _, _ = self.dfs(root)
        # return is_valid

        if root is None:
            return True

        # In-order traversal!
        last_visited = None
        is_valid = True

        def dfs_in_order(node: TreeNode):
            nonlocal last_visited
            nonlocal is_valid

            if node.left is not None:
                dfs_in_order(node.left)

            # Compare current visited node with last visited
            if last_visited is not None:
                is_valid = is_valid and node.val > last_visited
            # Update last visited
            last_visited = node.val

            if node.right is not None:
                dfs_in_order(node.right)

        dfs_in_order(root)

        return is_valid

    def dfs(self, node: TreeNode | None) -> tuple[bool, int | None, int | None]:
        # DFS... without any consideration of traversal order. Brute force, if you will.
        if node is None:
            return True, None, None

        is_valid = True

        is_left_valid, left_min_val, left_max_val = self.dfs(node.left)
        if left_max_val is not None and node.val <= left_max_val:
            is_valid = False

        is_right_valid, right_min_val, right_max_val = self.dfs(node.right)
        if right_min_val is not None and node.val >= right_min_val:
            is_valid = False

        min_val = node.val
        max_val = node.val

        if left_min_val is not None:
            min_val = min(min_val, left_min_val)
        if right_min_val is not None:
            min_val = min(min_val, right_min_val)

        if left_max_val is not None:
            max_val = max(max_val, left_max_val)
        if right_max_val is not None:
            max_val = max(max_val, right_max_val)

        return (
            is_valid and is_left_valid and is_right_valid,
            min_val,
            max_val,
        )


node_1 = TreeNode(1)
node_3 = TreeNode(3)
node_2 = TreeNode(2, node_1, node_3)

assert Solution().isValidBST(node_2)

node_1 = TreeNode(1)
node_3 = TreeNode(3)
node_6 = TreeNode(6)
node_4 = TreeNode(4, node_3, node_6)
node_5 = TreeNode(5, node_1, node_4)

assert not Solution().isValidBST(node_5)

print("OK")
