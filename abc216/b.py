n = int(input())
st = [tuple(input().split()) for _ in range(n)]

flg = False
for i in range(n):
    for j in range(i+1, n):
        if st[i] == st[j]:
            flg = True

if flg:
    print("Yes")
else:
    print("No")
