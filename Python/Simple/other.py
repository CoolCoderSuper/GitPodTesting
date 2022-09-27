def if_stuff(num):
    if num > 10:
        print("Greater")
    elif num < 10:
        print("Less")
    elif num <= 10:
       print("LessEqual")
    elif num >= 10:
        print("GreaterEqual")
    elif num == 10:
        print("Equal")
    else:
        print("N/A")

def other_if_stuff(str):
    if str == "Joe" or str == "Bob":
        print("Cool")
    elif str.startswith("J") and str.endswith("e"): 
        print("Sort of cool")
    else:
        print("Not cool")

def while_stuff():
    i = 0
    while i < 10:
        print("Wait")
        i += 1
    else: 
        print("Done")

def for_stuff():
    items = ["Joe", "Bob", "Robert"]
    for item in items:
        print("Hello " + item)
    else: 
        print("Bye")

def lamda_stuff():
    thing = lambda: 3 * 3
    print(thing())
    thing2 = lambda x: x * 3
    print(thing2(2))
