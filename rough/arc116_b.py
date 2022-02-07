import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, = li()
    A = li()
    A.sort()
    ans = 0
    sm = 0
    ans += A[-1]**2
    ans %= MOD
    if N == 1:
        print(ans)
        return
    for i in range(N-1, 0, -1):
        sm += sm + A[i]
        sm %= MOD
        ans += (sm+A[i-1])*A[i-1]
        ans %= MOD
    print(ans)


if __name__ == '__main__':
    MOD = 998244353
    _solve()
