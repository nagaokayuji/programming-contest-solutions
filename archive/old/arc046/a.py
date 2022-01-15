N = int(input())

cnt = 0
for x in range(1, 10**8):
    s = str(x)
    if all(map(lambda x: x == s[0], s)):
        cnt += 1
        if cnt == N:
            print(x)
            exit()
