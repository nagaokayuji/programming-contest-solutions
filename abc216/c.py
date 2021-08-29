n = int(input())


ans = []

while n:
    if n % 2 == 0:
        ans.append('B')
        n //= 2
    else:
        n -= 1
        ans.append('A')


ans.reverse()
print("".join(ans))
