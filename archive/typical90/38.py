import sys
from math import gcd
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


A, B = mi()

g = gcd(A, B)
lcm = A*B//g

if lcm > 10**18:
    print("Large")
else:
    print(lcm)
