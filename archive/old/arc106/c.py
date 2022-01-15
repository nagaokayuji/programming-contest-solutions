import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, M = readti()

    nxt = 2
    if M == 0:
        for _ in range(N):
            print(nxt, nxt+1)
            nxt += 2
        return

    if M < 0 or N == M or N == M+1:
        print(-1)
        return
    if N <= 3 and M >= 2:
        print(-1)
        return

    for _ in range(N-M-2):
        print(nxt, nxt+1)
        nxt = nxt+2

    print(nxt+1, 10**9)
    nxt += 2

    for _ in range(M+1):
        print(nxt, nxt+1)
        nxt += 2


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
