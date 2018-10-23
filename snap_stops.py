import json
import os
#Id=data["features"][0]["properties"]["id"]

for root, dirs, files in os.walk('data/xalapa_bus_data/data'):
    print(root)
    if not root.endswith('route'):
        continue

    file_path = os.path.join(root, 'route.geojson')
    print(file_path)


