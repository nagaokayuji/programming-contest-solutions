from math import gcd
A, B, C, D = map(int, input().split())


def lcm(a, b):
    return a*b//gcd(a, b)


def f(x):
    return x - (x//C + x//D - x//lcm(C, D))


print(f(B)-f(A-1))
