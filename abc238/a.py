import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    n = int(input())
    right = n*n
    left = 1
    for _ in range(n):
        left *= 2
        if left > right:
            yn(True)
            return
    yn(left > right)


if __name__ == '__main__':
    _solve()
