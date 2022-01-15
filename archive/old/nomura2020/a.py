import sys
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    h1, m1, h2, m2, k = readti()

    st = h1*60+m1
    ed = h2*60+m2
    print(ed-st-k)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
