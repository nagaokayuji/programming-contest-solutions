from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


# W, N = mi()
# LR = [tuple(mi()) for _ in range(N)]


class SegmentTree:
    import typing

    def __init__(self,
                 n: int,
                 initial: any,
                 op: typing.Callable[[typing.Any, typing.Any], typing.Any]):
        m = 1
        while m <= (n+1):
            m *= 2

        self.m = m
        self.array = [initial] * (2*m)
        self.op = op
        self.initial = initial

    def update(self, index: int, val: any):
        pos = self.m + index
        self.array[pos] = val  # update
        while pos > 1:  # update forward root
            pos >>= 1
            self.array[pos] = self.op(self.array[pos*2], self.array[pos*2+1])

    def add(self, index: int, val: any):
        self.update(index, self.array[self.m+index]+val)

    def get(self, l: int, r: int):
        '''
        get value of [l,r)
        '''
        assert(l < r)
        ret = self.initial
        l += self.m
        r += self.m
        while l < r:
            if l & 1:  # その要素を使う
                ret = self.op(ret, self.array[l])
                l += 1
            l >>= 1
            if r & 1:
                ret = self.op(ret, self.array[r-1])
                r -= 1
            r >>= 1
        return ret


# N, Q = mi()
# a = li()


# seg = SegmentTree(N, 0, lambda x, y: x+y)
# for i, x in enumerate(a):
#     seg.update(i, x)

# for _ in range(Q):
#     t, a, b = mi()
#     if t == 0:
#         seg.add(a, b)
#     else:
#         print(seg.get(a, b))

# # pprint(seg.array)
seg = SegmentTree(4, 0, lambda a, b: a+b)
seg.update(0, 1)
seg.update(1, 2)
seg.update(2, 3)
seg.update(3, 4)
seg.add(1, 4)
for l in range(4):
    for r in range(l, 4):
        sum = 0
        for v in range(l, r+1):
            sum += seg.get(v, v+1)
        assert sum == seg.get(l, r+1)

pprint(seg.array)
