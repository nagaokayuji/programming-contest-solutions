from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


class DSU:
    def __init__(self, n: int = 0):
        self._n = n
        self.parent_or_size = [-1] * n

    def merge(self, a: int, b: int):
        x = self.leader(a)
        y = self.leader(b)

        if x == y:
            return x

        if x < y:
            x, y = y, x

        self.parent_or_size[x] += self.parent_or_size[y]
        self.parent_or_size[y] = x
        return x

    def same(self, a: int, b: int):
        return self.leader(a) == self.leader(b)

    def leader(self, a: int):
        parent = self.parent_or_size[a]
        while parent >= 0:
            if self.parent_or_size[parent] < 0:
                return parent
            self.parent_or_size[a], a, parent = (
                self.parent_or_size[parent],
                self.parent_or_size[parent],
                self.parent_or_size[self.parent_or_size[parent]]
            )

        return a

    def size(self, a: int):
        return -self.parent_or_size[self.leader(a)]

    def groups(self):
        leader_buf = [self.leader(i) for i in range(self._n)]
        result = [[] for _ in range(self._n)]
        for i in range(self._n):
            result[leader_buf[i]].append(i)
        return list(filter(lambda r: r, result))


Q = int(input())
N = 1 << 20
dsu = DSU(N)
field = [-1]*N
head = 0
for _ in range(Q):
    t, x = mi()
    if t == 1:
        h = x % N
        if field[h] == -1:
            assert not dsu.same(h, (h+1) % N)
            field[h] = x
            dsu.merge(h, (h+1) % N)
            continue
        # find
        r = dsu.leader(h)
        if field[r] == -1:
            field[r] = x
            assert not dsu.same(r, (r+1) % N)
            dsu.merge(r, (r+1) % N)
            continue

        r = dsu.leader((r+1) % N)
        dsu.merge(r, (r+1) % N)
        while field[head] != -1:
            head += 1
        r = head
        field[r] = x
        dsu.merge(r, (r+1) % N)
        head += 1
    else:
        print(field[x % N])
