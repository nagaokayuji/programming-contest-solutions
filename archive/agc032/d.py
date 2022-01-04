import sys
from pprint import pprint
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


def _solve():
    N = inputi()

    target = N+1 if N % 2 == 0 else N

    ans = []
    for i in range(1, N+1):
        for j in range(i+1, N+1):
            if i+j == target:
                continue
            ans.append((i, j))

    print(len(ans))
    for a, b in ans:
        print(a, b)


if __name__ == '__main__':
    _solve()
