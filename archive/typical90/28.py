from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


MX = 1000

field = [[0]*(MX+1) for _ in range(MX+1)]


def add(lx, ly, rx, ry):
    field[lx][ly] += 1
    field[lx][ry] -= 1
    field[rx][ly] -= 1
    field[rx][ry] += 1


N = int(input())
for _ in range(N):
    lx, ly, rx, ry = mi()
    add(lx, ly, rx, ry)

for i in range(1, MX+1):
    for j in range(MX+1):
        field[i][j] += field[i-1][j]

for i in range(MX+1):
    for j in range(1, MX+1):
        field[i][j] += field[i][j-1]

cnts = [0]*(N+1)
for x in range(MX+1):
    for y in range(MX+1):
        cnts[field[x][y]] += 1


print(*cnts[1:])
