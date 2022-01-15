A, B, W = map(int, input().split())
W *= 1000

a = []
for cnt in range(1, W+2):
    if A*cnt <= W <= B*cnt:
        a.append(cnt)

if not a:
    print("UNSATISFIABLE")
else:
    print(a[0], a[-1])
