import sys
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    S = [int(ord(x)-ord('a')) for x in list(input())]
    T = [int(ord(x)-ord('a')) for x in list(input())]
    g = [set() for _ in range(26)]
    rg = [set() for _ in range(26)]
    for a, b in zip(S, T):
        g[a].add(b)
        rg[b].add(a)
    for i in range(26):
        g[i] = list(g[i])
        rg[i] = list(rg[i])
    for i in range(26):
        if len(g[i]) > 1 or len(rg[i]) > 1:
            print("No")
            return
    print("Yes")


if __name__ == '__main__':
    _solve()
