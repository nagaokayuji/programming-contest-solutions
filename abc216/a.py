s = input().split('.')
x, y = s


if y <= '2':
    print(x + "-")
elif y <= '6':
    print(x)
else:
    print(x + "+")
