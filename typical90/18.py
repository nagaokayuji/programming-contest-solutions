from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
import math
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


T = int(input())
L, X, Y = mi()
Q = int(input())


pi = 3.141592653589793


def solve(t):
    x = 0
    y = L/2 * math.sin(-t/T * pi*2)
    z = L/2 - L/2*math.cos(t/T * pi*2)
    # print(x, y, z)
    dis = ((x-X)**2 + (y-Y)**2)**0.5
    print(math.atan2(z, dis)*180/pi)


for _ in range(Q):
    solve(int(input()))
