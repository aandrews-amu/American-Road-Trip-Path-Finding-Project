"""
50 States Project
Abigail Andrews, Emma Sheridan, and Wil Troxel
CS151-PO
"""
import urllib.request
import json

file_type = 'json'
origins = 'Boston,MA|Charlestown,MA'
destinations = 'Lexington,MA|Concord,MA'
GMAPS_API_KEY = 'AIzaSyC66FIK42qhujShvQ5ALsNFUF5jD9A8bEs'

url = ('https://maps.googleapis.com/maps/api/distancematrix/'
       + file_type
       + '?'
       + 'origins=' + origins
       + '&destinations=' + destinations
       + '&key=' + GMAPS_API_KEY
       )

response = urllib.request.urlopen(url)
response_json = json.loads(response.read())

if __name__ == '__main__':
    print(response_json)
