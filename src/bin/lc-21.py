#!/usr/bin/env python
from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def mergeTwoLists(
        self, list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        # NOTE: Much shorter with dummy node
        if list1 is None or list2 is None:
            if list1 is not None:
                return list1
            elif list2 is not None:
                return list2
            return None
        if list1.val < list2.val:
            head = list1
            list1 = list1.next
        else:
            head = list2
            list2 = list2.next

        cursor = head
        while True:
            if list1 is None or list2 is None:
                # NOTE: Can use python "or"
                if list1 is not None:
                    cursor.next = list1
                elif list2 is not None:
                    cursor.next = list2
                break
            if list1.val < list2.val:
                cursor.next = list1
                cursor = cursor.next
                list1 = list1.next
            else:
                cursor.next = list2
                cursor = cursor.next
                list2 = list2.next
        return head


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
    ln1 = to_listnode([1, 2, 4])
    ln2 = to_listnode([1, 3, 4])
    solution = get_solution(ln1, ln2)
    print(to_list(solution))

    ln1 = to_listnode([])
    ln2 = to_listnode([])
    solution = get_solution(ln1, ln2)
    print(to_list(solution))

    ln1 = to_listnode([])
    ln2 = to_listnode([0])
    solution = get_solution(ln1, ln2)
    print(to_list(solution))
    


def get_solution(ln1, ln2):
    s = Solution()
    return s.mergeTwoLists(ln1, ln2)


if __name__ == "__main__":
    main()
