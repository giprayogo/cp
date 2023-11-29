from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# TODO: Recursive solution?
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        if head is None:
            return head

        left = head
        right = head
        # NOTE: TLE
        while left.next is not None:
            while right.next.next is not None:
                right = right.next
            if left == right:
                break
            temp = left.next
            left.next = right.next
            right.next = None
            left.next.next = temp
            right = left = temp


def to_listnode(a: list) -> Optional[ListNode]:
    li = iter(a)
    head = None
    try:
        head = ListNode(next(li))
        cursor = head
        for e in li:
            cursor.next = ListNode(e)
            cursor = cursor.next
    except StopIteration:
        return head
    return head


def to_list(ln: Optional[ListNode]) -> list:
    a = []
    cursor = ln
    if cursor is None:
        return a
    a.append(cursor.val)
    while cursor.next is not None:
        cursor = cursor.next
        a.append(cursor.val)
    return a


def main():
    s = Solution()

    ln0 = to_listnode([])
    ln1 = to_listnode([1])
    ln2 = to_listnode([1, 2, 3, 4])
    ln3 = to_listnode([1, 2, 3, 4, 5])
    ln4 = to_listnode(list(range(11)))

    for ln in [ln0, ln1, ln2, ln3, ln4]:
    # for ln in [ln2]:
        print("-" * 88)
        print(to_list(ln))
        s.reorderList(ln)
        print(to_list(ln))


if __name__ == "__main__":
    main()
