import sys
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def i(): return int(input())


N = i()
A = li()

s = sum(A)
if s % 10 != 0:
    print("No")
    exit()

target = s//10
l = 0
r = 1
B = set(list(np.cumsum([0]+A[:]+A[:])))
for a in B:
    if (target+a) in B:
        print("Yes")
        exit()
print("No")
