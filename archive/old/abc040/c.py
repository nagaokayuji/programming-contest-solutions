import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    A = readli()
    dp = [INF]*(N+1)
    dp[0] = 0
    for i in range(N-1):
        dp[i+1] = min(dp[i+1], dp[i]+abs(A[i+1]-A[i]))
        if i+2 < N:
            dp[i+2] = min(dp[i+2], dp[i]+abs(A[i+2]-A[i]))
    print(dp[N-1])


if __name__ == '__main__':
    _solve()
