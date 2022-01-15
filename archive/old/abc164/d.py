import sys
from collections import defaultdict, Counter, deque
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    S = list(map(int, list(input())))
    cnts = defaultdict(int)
    base = 1
    ss = [0]
    for x in S[::-1]:
        ss.append((ss[-1] + x*base) % MOD)
        base = base*10 % MOD
    for x in ss:
        cnts[x] += 1
    ans = 0
    for k, v in cnts.items():
        ans += v*(v-1)//2

    print(ans)
    return


if __name__ == '__main__':
    MOD = 2019
    _solve()
