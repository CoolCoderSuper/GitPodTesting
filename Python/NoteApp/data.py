import json
import os
import notes

def init():
    if os.path.exists("notes.json") == False:
        f = open("notes.json", "w")
        f.write("[]")
        f.close()

def save(dict):
    f = open("notes.json", "w")
    f.write(json.dumps(dict))
    f.close()

def load():
    f = open("notes.json")
    r = json.loads(f.read())
    f.close()
    return r

def deserialize(dict):
    items = list(dict)
    results = []
    for item in items:
        r = notes.Note(item["Id"])
        r.Value = item["Value"]
        results.append(r)
    return results

def serialize(dict):
    items = list(dict)
    results = []
    for item in items:
        results.append({"Id": item.Id, "Value": item.Value})
    return results

def max_notes(dict):
    nums = []
    for item in dict:
       nums.append(item.Id)
    return max(nums)

def where(dict, id):
    items = []
    for item in dict:
        if item.Id == id:
            items.append(item)
    return items

def first(dict, id):
    return where(dict, id)[0]