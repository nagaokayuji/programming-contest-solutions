import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readli1(): return list(readmi1())
def readti(): return tuple(readmi())
def readti1(): return tuple(readmi1())
def readi(): return int(input())


def _solve():
    H, W = readti()
    a, b = min(H, W), max(H, W)
    if a % 3 == 0 or b % 3 == 0:
        print(0)
        return

    ans = INF

    for _ in range(2):
        s1 = b * ((a+2)//3)
        s2 = ((b+1)//2) * (a - ((a+2)//3))
        s3 = a*b-s1-s2
        ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))

        s1 = b * ((a+2)//3)
        s2 = b * (a - ((a+2)//3)+1)//2
        s3 = a*b-s1-s2
        ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))

        if a//3 > 0:
            s1 = b * (a//3)
            s2 = ((b+1)//2) * (a - (a//3))
            s3 = a*b-s1-s2
            ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))

        a, b = b, a

    print(ans)


if __name__ == '__main__':
    _solve()
