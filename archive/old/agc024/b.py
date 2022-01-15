import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    A = [readi()-1 for _ in range(N)]
    dc = {}
    for i, x in enumerate(A):
        dc[x] = i

    dp = [1]*N

    for i in range(N-2, -1, -1):
        if dc[i] < dc[i+1]:
            dp[i] = dp[i+1] + 1

    print(N-max(dp))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
