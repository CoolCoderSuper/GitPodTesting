import os
#TODO: Look into these more
if os.path.exists("file.txt") == False:
    open("file.txt", "x").close()
os.remove("file.txt")