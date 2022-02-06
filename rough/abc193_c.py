N = int(input())


ex = set()
for a in range(2, N+1):
    if a*a > N:
        break
    for b in range(2, N+1):
        if a**b > N:
            break
        ex.add(a**b)

print(N-len(ex))
