import sys
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    s1 = list(input())
    s2 = list(input())
    s3 = list(input())
    cs = set(s1+s2+s3)
    if len(cs) > 10:
        print("UNSOLVABLE")
        return
    lcs = list(cs)
    perms = list(permutations(range(10)))
    for p in perms:
        d = {}
        for i, c in enumerate(lcs):
            d[c] = p[i]
        if d[s1[0]] == 0 or d[s2[0]] == 0 or d[s3[0]] == 0:
            continue

        s1n, s2n, s3n = 0, 0, 0
        for x in s1:
            s1n *= 10
            s1n += d[x]
        for x in s2:
            s2n *= 10
            s2n += d[x]
        for x in s3:
            s3n *= 10
            s3n += d[x]
        if s1n+s2n == s3n:
            print(s1n)
            print(s2n)
            print(s3n)
            return
    print("UNSOLVABLE")


if __name__ == '__main__':
    _solve()
