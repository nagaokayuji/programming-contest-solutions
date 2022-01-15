import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    H, W = readmi()
    C = [readli() for _ in range(10)]
    A = [readli() for _ in range(H)]

    for k in range(10):
        for i in range(10):
            for j in range(10):
                C[i][j] = min(C[i][j], C[i][k]+C[k][j])

    ans = 0
    for row in A:
        for c in row:
            if c == -1:
                continue
            ans += C[c][1]
    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
