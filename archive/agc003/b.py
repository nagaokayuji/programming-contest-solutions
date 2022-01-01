import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = int(input())
    A = [int(input()) for _ in range(N)]
    pairs = 0
    pre = 0
    for i in range(N):
        pairs += min(pre, A[i])
        A[i] -= min(pre, A[i])
        pairs += A[i]//2
        A[i] %= 2
        pre = A[i]

    print(pairs)


if __name__ == '__main__':
    _solve()
