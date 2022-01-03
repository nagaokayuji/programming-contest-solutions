from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    a, b, c, d, e, f = readmi()
    dp = [[False]*(f+1) for _ in range(32)]

    dp[0][0] = True

    for water in range(31):
        water_g = water * 100
        if water_g > f:
            break
        for sugar in range(f+1):
            # e/100 < sugar/water
            if e*water < sugar:
                break
            if water_g + sugar+c <= f:
                dp[water][sugar+c] |= (e*water >= sugar+c) and dp[water][sugar]
            if water_g + sugar+d <= f:
                dp[water][sugar+d] |= (e*water >= sugar+d) and dp[water][sugar]
            if 100*(water+a) + sugar <= f:
                dp[water+a][sugar] |= dp[water][sugar]
            if 100*(water+b) + sugar <= f:
                dp[water+b][sugar] |= dp[water][sugar]

    water_sugar_max, sugar_max, nodo = 0, 0, 0
    for water in range(1, 31):
        for sugar in range(f+1):
            sum_ = water*100+sugar
            if sum_ > f:
                break
            if e*water < sugar:
                break
            if dp[water][sugar] and (sugar/(sum_) >= nodo):
                water_sugar_max = sum_
                sugar_max = sugar
                nodo = sugar/(sum_)

    assert (water_sugar_max <= f)
    print(water_sugar_max, sugar_max)


if __name__ == '__main__':
    _solve()
