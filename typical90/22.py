import sys
from math import sqrt, gcd, factorial
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


A, B, C = mi()


ans = 0

g = gcd(A, gcd(B, C))

print(A//g + B//g + C//g - 3)
