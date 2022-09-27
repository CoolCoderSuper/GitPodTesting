try:
    num = 1
    res = num * 3 / 8
    print(res)
except NameError:
    print("Name Error")
except TypeError:
    print("Type Error")
else:
    print("No errors")
finally:
    print("Done")

def complex_thing(num1, num2):
    if num2 != 10:
        return num1 / num2
    else:
        raise ValueError("Cannot process number 10")

complex_thing(12, 10)