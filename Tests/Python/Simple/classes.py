class Person:
    Name = ""
    Age = 0
    Description = ""

    def __init__(self, name):
        self.Name = name

    def gen_Description(self):
        desc = self.Name + " is " + str(self.Age) + " years old."
        self.Description = desc
        return desc

    def cool_Static():
        print("No instance.")