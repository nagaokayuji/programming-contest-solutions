import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, = li()
    if N % 2 == 1:
        print(0)
        return

    ans = 0

    N //= 10
    ans += N

    while N:
        ans += N//5
        N //= 5
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
