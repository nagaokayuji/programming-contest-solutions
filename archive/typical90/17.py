from pprint import pprint
import sys


def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


class FenwickTree:
    '''
    Fenwick Tree (BIT)
    0-indexed
    '''

    def __init__(self, n: int = 10**6):
        '''
        initialize n length
        '''
        self._n = n
        self.data = [0]*n

    def add(self, p: int, x):
        '''
        adds x
        '''
        p += 1
        while p <= self._n:
            self.data[p-1] += x
            p += p & -p

    def sum(self, left: int, right: int):
        '''
        gets sum of [l,r)
        '''
        return self._sum(right) - self._sum(left)

    def _sum(self, r: int):
        if r < 0:
            return 0
        s = 0
        while r > 0:
            s += self.data[r-1]
            r -= r & -r
        return s


N, M = mi()

LR = [tuple(mi()) for _ in range(M)]
LR.sort(key=lambda x: (x[1], -x[0]))

t1 = FenwickTree()
t2 = FenwickTree()

ans = 0
for l, r in LR:
    ans += t1._sum(l) - t2._sum(l+1)
    t1.add(l, 1)
    t2.add(r, 1)

print(ans)
