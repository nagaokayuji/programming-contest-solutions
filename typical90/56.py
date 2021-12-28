from pprint import pprint
import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, S = mi()
AB = np.array([li() for _ in range(N)])


@njit(void(i8, i8, i8[:], i8[:]))
def solve(N, S, A, B):
    dp = np.zeros((N+1, S+1), np.int64)
    dp[0][0] = 1

    for i in range(N):
        a, b = A[i], B[i]
        for s in range(S+1):
            if s+b <= S:
                dp[i+1][s+b] |= dp[i][s]
            if s+a <= S:
                dp[i+1][s+a] |= dp[i][s]

    if not dp[N][S]:
        print("Impossible")
        return

    back = [""]*N
    now = S
    for i in range(N, 0, -1):
        a, b = A[i-1], B[i-1]
        if now >= a and dp[i-1][now-a]:
            back[i-1] = "A"
            now -= a
        else:
            # back = np.append(back, "B")
            back[i-1] = "B"
            now -= b

    print("".join(back))


solve(N, S, AB[:, 0], AB[:, 1])
