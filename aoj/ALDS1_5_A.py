import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    n = int(input())
    a = list(mi())
    q = int(input())
    m = list(mi())

    dp = [False]*2002
    dp[0] = True
    a.sort()
    for x in a:
        dp2 = dp.copy()
        for i in range(2001):
            if i+x <= 2000:
                dp2[i+x] |= dp[i]
        dp = dp2

    for x in m:
        if dp[x]:
            print("yes")
        else:
            print("no")


if __name__ == '__main__':
    _solve()
