def func():
    global x
    x = 3
    print(x)
    print(y)

y = 1
func()
print(y)
print(x)

def other_func():
    t = 9
    def nest_func():
        d = t * 7
    nest_func()