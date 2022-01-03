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
    minus = []
    plus = []
    for x in A:
        if x >= 0:
            plus.append(x)
        else:
            minus.append(x)
    plus.sort(reverse=True)
    minus.sort()

    # solve
    if not plus:
        plus.append(minus.pop())
    if not minus:
        minus.append(plus.pop())
    now = minus[0]
    anss = []

    for x in plus[:-1]:
        anss.append((now, x))
        now -= x
    last = plus.pop()
    anss.append((last, now))
    now = last-now
    for x in minus[1:]:
        anss.append((now, x))
        now -= x
    print(now)
    for a, b in anss:
        print(a, b)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
