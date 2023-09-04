import json
obj = '{"Name": "Joe", "Age": 10, "Description": ""}'
res = json.loads(obj)
print(res["Name"])

pyObj = {"Name": "Joe", "Age": 10, "Description": ""}
strRes = json.dumps(pyObj)
print(strRes)