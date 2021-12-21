from numba import njit, b1, i1, i4, i8, f8
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


def li(): return list(mi())


N = int(input())
A, B, C = tuple(sorted(li())[::-1])

#


@njit("i8(i8,i8,i8,i8)")
def solve(N, A, B, C):
    ans = 10000
    for ac in range(10000):
        if ac*A > N:
            break
        for bc in range(10000):
            left = N - A*ac - B*bc
            if left < 0:
                break
            if left % C == 0:
                ans = min(ans, ac+bc+left//C)
    return ans


print(solve(N, A, B, C))
