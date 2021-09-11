p = list(map(int, input().split()))

ans = []
for x in p:
    ans.append(chr(ord('a') + x-1))

print("".join(ans))
