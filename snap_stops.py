import json
import os

for root, dirs, files in os.walk('data/xalapa_bus_data/data'):
    if not root.endswith('/route'):
        continue

    file_path = os.path.join(root, 'route.geojson')

    print(file_path)
