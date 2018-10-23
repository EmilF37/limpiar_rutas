import json, os

os.walk('data/xalapa_bus_data/data')


data = json.load(open('data/xalapa_bus_data/data/001/route/route.geojson'))

Id=data["features"][0]["properties"]["id"]

#print(data)
