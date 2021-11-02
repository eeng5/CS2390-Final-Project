import requests

if __name__ == "__main__":
  kinan = {
    "id": 1,
    "name": "Kinan",
    "identity": "TA",
    "hometown": "Damascus",
    "age": 0
  }

  kinan2 = {
    "id": 1,
    "name": "Kinan Bab",
    "identity": "CSCI 2390 TA",
    "hometown": "Damascus",
    "age": 0
  }

  print("Initial DB content")
  print(requests.get("http://localhost:8000/heroes").text)
  print("")

  print("Inserting one element")
  print(requests.post("http://localhost:8000/hero", json=kinan).text)
  print("")

  print("Updating element")
  print(requests.put("http://localhost:8000/hero/1", json=kinan2).text)
  print("")

  print("DB content")
  print(requests.get("http://localhost:8000/heroes").text)
  print("")

  print("Deleting element")
  print(requests.delete("http://localhost:8000/hero/1").text)
  print("")

  print("DB content")
  print(requests.get("http://localhost:8000/heroes").text)
  print("")
