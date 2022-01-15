N = int(input())
AB = [tuple(map(int, input().split())) for _ in range(N)]
AB.sort()
a1 = sum(AB[0])
a2 = max(AB[0][0], sorted(AB[1:], key=lambda x: x[1])[0][1])
print(min(a1, a2))
