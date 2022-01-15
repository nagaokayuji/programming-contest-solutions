import sys
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
    A, B = inputmi()

    if N <= A:
        print("Takahashi")
        return

    if A == B:
        if N % (A+1) == 0:
            print("Aoki")
        else:
            print("Takahashi")
        return

    if A > B:
        print("Takahashi")
        return

    if A < B:
        print("Aoki")


if __name__ == '__main__':
    _solve()
