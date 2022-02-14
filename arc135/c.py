import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    ''''''
    N, = li()
    A = np.array(li(), np.int64)

    @njit(void(), cache=True)
    def solve():
        bs = np.zeros(32, np.int64)
        for x in A:
            for b in range(32):
                if x >> b & 1:
                    bs[b] += 1

        sayoed = 0
        for x in A:
            dif = 0
            x = x ^ sayoed
            for b in range(31, -1, -1):
                if x >> b & 1:
                    dif -= (1 << b)*(bs[b] - (N-(bs[b])))
            if dif > 0:
                sayoed ^= x
                for b in range(31, -1, -1):
                    if x >> b & 1:
                        bs[b] = N-bs[b]

        ans = 0
        for b in range(31):
            ans += (1 << b) * bs[b]
        print(ans)
    solve()


if __name__ == '__main__':
    _solve()
