import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    ''''''
    a, b = li()
    a -= 1
    b -= 1
    yn(((a+1) % 10 == b) or ((b+1) % 10 == a))


if __name__ == '__main__':
    _solve()
