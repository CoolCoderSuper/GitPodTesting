def string_tests():
    test = "hello world\n" * 3 + 'bye'
    print(test[0])
    print(test[-1])
    print(test)
    print(f"val: '{test}'")
