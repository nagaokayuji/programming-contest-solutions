import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace


def _solve():
    S = input()
    N = len(S)
    notc = []
    for i in range(N):
        if S[i] != S[N-1-i]:
            notc.append(i)

    if N % 2 == 0:
        if (not notc) or (len(notc) > 2):
            print(25*N)
            return
        print(25*(N-2) + (24*2))
    else:
        if not notc:
            print((N-1)*25)
            return
        if len(notc) > 2:
            print(25*N)

        if len(notc) == 2:
            print(25*(N-2) + 24*2)
            return


if __name__ == '__main__':
    _solve()
