import sys
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


class FenwickTree:
    def __init__(self, n: int = 10**6):
        self._n = n
        self.data = [0] * n

    def add(self, p: int, x):
        p += 1
        while p <= self._n:
            self.data[p-1] += x
            p += p & -p

    def sum(self, left: int, right: int):
        return self._sum(right) - self._sum(left)

    def _sum(self, r: int):
        s = 0
        while r > 0:
            s += self.data[r-1]
            r -= r & -r
        return s


def _solve():
    N, K = mi()
    A = [int(input()) for _ in range(N)]
    S = list(np.cumsum([0] + A))
    sk = S.copy()
    for i in range(len(sk)):
        sk[i] = sk[i] - K*i
    dic = {v: i for i, v in enumerate(sorted(set(sk)))}
    B = list(map(lambda x: dic[x], sk))
    # print(B)
    ans = 0
    fwt = FenwickTree(N+1)
    for i, x in enumerate(B):
        ans += fwt.sum(0, x+1)
        fwt.add(x, 1)
    print(ans)


if __name__ == '__main__':
    _solve()
