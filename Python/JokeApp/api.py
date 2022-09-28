import requests

class APICall:
    def call(self, method, api):
        if method.lower() == "get":
            r = requests.get(api)
            if r.ok == True:
                return r.json()["value"]
            else:
                raise TypeError("API error")

class ChuckNorris(APICall):
    url = "https://api.chucknorris.io/jokes/random?category="
    categories = ["animal", "dev"]

    def __init__(self, category):
        self.category = category
        if not self.category in self.categories:
            raise TypeError("Invalid category")

    def get(self):
        url = f"{self.url}{self.category}"
        return self.call("get", url)