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

    now = 1
    ans = 0
    sum_ = sum(A)
    for x in A:
        ans += now
        sum_ -= x
        now = min((now-x)*2, sum_)

    if now:
        print(-1)
        return
    print(ans)


if __name__ == '__main__':
    _solve()
