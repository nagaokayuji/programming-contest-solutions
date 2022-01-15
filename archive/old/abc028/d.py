import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, K = readmi()
    allc = N**3
    all_k = 1
    k_2 = 1 * 1 * (N-1) * 3
    k_1 = 1 * (K-1) * (N-K) * 6
    print((k_1+k_2+all_k)/allc)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
