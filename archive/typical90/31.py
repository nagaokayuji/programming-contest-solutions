from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
W = li()
B = li()
# grundy = [[None]*1444 for _ in range(55)]
grundy = defaultdict(int)

for w in range(52):
    for b in range(1442):
        s = set()
        if w >= 1:
            s.add(grundy[w-1, b+w])
        if b >= 2:
            for i in range(1, b//2 + 1):
                s.add(grundy[w, b-i])

        def mex(s):
            i = 0
            while True:
                if i not in s:
                    return i
                i += 1

        grundy[w, b] = mex(s)

ans = 0
for w, b in zip(W, B):
    ans ^= grundy[w, b]

if ans > 0:
    print('First')
else:
    print("Second")
