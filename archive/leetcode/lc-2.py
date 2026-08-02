from typing import Optional, TypeVar, Type, Iterable, Iterator

T = TypeVar("T", bound="ListNode")


class ListNode:
    """
    LeetCode's definition for singly-linked list.
    Plus I added some methods for easier debugging.
    """

    def __init__(self, val: int = 0, next: Optional[Type[T]] = None):
        self.val = val
        self.next = next

    def __iter__(self) -> Iterator[int]:
        cursor = self
        yield cursor.val
        while cursor.next is not None:
            cursor = cursor.next
            yield cursor.val

    @classmethod
    def try_from_iterable(cls: Type[T], elements: Iterable) -> Optional[T]:
        dummy_head = ListNode(0)
        cursor = dummy_head
        for e in elements:
            cursor.next = ListNode(e)
            cursor = cursor.next
        return dummy_head.next


class Solution:
    # def addTwoNumbers(
    #     self, l1: Optional[ListNode], l2: Optional[ListNode]
    # ) -> Optional[ListNode]:
    #     n = 0

    #     heads = [l1, l2]
    #     for _l in heads:
    #         cur = _l
    #         _n = 0
    #         mul = 1
    #         while cur:
    #             _n += mul * cur.val
    #             mul *= 10
    #             cur = cur.next
    #         n += _n

    #     prev = None
    #     max_mul = 10 ** (len(str(n)) - 1)
    #     while max_mul > 0:
    #         digit = n // max_mul
    #         n -= digit * max_mul
    #         max_mul //= 10

    #         node = ListNode(int(digit))
    #         node.next = prev
    #         prev = node

    #     return node
    # With carry ala' neetcode (shorter though? -> nope misread the digit order)
    def addTwoNumbers(self, l1, l2):
        carry = 0
        dummy = ListNode(0)
        cur = dummy
        while l1 or l2 or carry:
            if l1:
                l1v = l1.val
                l1 = l1.next
            else:
                l1v = 0
            if l2:
                l2v = l2.val
                l2 = l2.next
            else:
                l2v = 0

            val = l1v + l2v + carry
            digit = val % 10
            carry = val // 10

            node = ListNode(digit)
            cur.next = node
            cur = cur.next

        return dummy.next


s = Solution()

e1l1 = ListNode.try_from_iterable([2, 4, 3])
e1l2 = ListNode.try_from_iterable([5, 6, 4])
print(list(e1l1), list(e1l2))
print(list(s.addTwoNumbers(e1l1, e1l2)))

e2l1 = ListNode.try_from_iterable([0])
e2l2 = ListNode.try_from_iterable([0])
print(list(e2l1), list(e2l2))
print(list(s.addTwoNumbers(e2l1, e2l2)))

e3l1 = ListNode.try_from_iterable([9, 9, 9, 9, 9, 9, 9])
e3l2 = ListNode.try_from_iterable([9, 9, 9, 9])
print(list(e3l1), list(e3l2))
print(list(s.addTwoNumbers(e3l1, e3l2)))
