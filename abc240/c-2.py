import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, X = li()
    dp = 1
    for _ in range(N):
        a, b = li()
        dp = (dp << a) | (dp << b)
    yn(dp>>X&1)



if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
