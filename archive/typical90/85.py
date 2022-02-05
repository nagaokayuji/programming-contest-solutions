
from pprint import pprint
import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())

'''
'''


@njit(i8[:](i8), cache=True)
def f(K):
    fs = set()
    fs.add(1)
    for x in range(1, int(K**0.5) + 2):
        if K % x == 0:
            fs.add(x)
            fs.add(K//x)
    ll = np.array(list(fs), np.int64)
    ll.sort()
    return ll


@njit(i8(i8, i8, i8[:]), cache=True)
def g(n, K, ll):
    n = len(ll)
    ans = 0
    for i in range(n):
        for j in range(i, n):
            if ll[j] > K//ll[i]:
                continue
            if K % (ll[i]*ll[j]) != 0:
                continue
            t = K//ll[i]//ll[j]
            if t >= ll[j]:
                ans += 1
    return ans


def solve():
    '''
    '''
    K = readi()
    ll = f(K)
    ans = g(len(ll), K, ll)
    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    solve()
