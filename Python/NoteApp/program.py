import sys
import notes
import data
import json
#TODO: Fix the delete
data.init()
items = list(data.deserialize(data.load()))
a = sys.argv[1]
if a == "-add":
    item = notes.Note(data.max_notes(items) + 1)
    item.Value = sys.argv[2]
    items.append(item)
elif a == "-remove":
    data.first(items, sys.argv[2])
elif a == "-list":
    print(json.dumps((data.serialize(items)), indent=4))
data.save(data.serialize(items))