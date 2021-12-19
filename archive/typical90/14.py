def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
A = sorted(li())
B = sorted(li())

ans = sum(abs(x-y) for x, y in zip(A, B))
print(ans)
