import sys
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
    N, M, D = inputti()

    if D == 0:
        print((N-D)/N/N * (M-1))
    else:
        print(2*(N-D)/N/N * (M-1))


if __name__ == '__main__':
    _solve()
