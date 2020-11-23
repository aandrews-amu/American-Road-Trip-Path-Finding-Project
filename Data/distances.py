"""
Code taken directly from Randal S. Olson "Computing the optimal road trip across the U.S."
- Creates a tsv file with to point, from point, distance, and duration for all combinations 
Source: https://github.com/rhiever/Data-Analysis-and-Machine-Learning-Projects/blob/master/optimal-road-trip/Computing%20the%20optimal%20road%20trip%20across%20the%20U.S..ipynb
"""
from itertools import combinations
import googlemaps
import pandas as pd
import numpy as np
import random
from data import all_waypoints

gmaps = googlemaps.Client(key="AIzaSyC66FIK42qhujShvQ5ALsNFUF5jD9A8bEs")

waypoint_distances = {}
waypoint_durations = {}

for (waypoint1, waypoint2) in combinations(all_waypoints, 2):
    try:
        route = gmaps.distance_matrix(origins=[waypoint1],
                                      destinations=[waypoint2],
                                      mode="driving",  # Change this to "walking" for walking directions,
                                      # "bicycling" for biking directions, etc.
                                      language="English",
                                      units="metric")

        # "distance" is in meters
        distance = route["rows"][0]["elements"][0]["distance"]["value"]

        # "duration" is in seconds
        duration = route["rows"][0]["elements"][0]["duration"]["value"]

        waypoint_distances[frozenset([waypoint1, waypoint2])] = distance
        waypoint_durations[frozenset([waypoint1, waypoint2])] = duration

    except Exception as e:
        print("Error with finding the route between %s and %s." %
              (waypoint1, waypoint2))

with open("my-waypoints-dist-dur.tsv", "w") as out_file:
    out_file.write("\t".join(["waypoint1",
                              "waypoint2",
                              "distance_m",
                              "duration_s"]))

    for (waypoint1, waypoint2) in waypoint_distances.keys():
        out_file.write("\n" +
                       "\t".join([waypoint1,
                                  waypoint2,
                                  str(waypoint_distances[frozenset(
                                      [waypoint1, waypoint2])]),
                                  str(waypoint_durations[frozenset([waypoint1, waypoint2])])]))

waypoint_distances = {}
waypoint_durations = {}
all_waypoints = set()

waypoint_data = pd.read_csv("my-waypoints-dist-dur.tsv", sep="\t")

for i, row in waypoint_data.iterrows():
    waypoint_distances[frozenset(
        [row.waypoint1, row.waypoint2])] = row.distance_m
    waypoint_durations[frozenset(
        [row.waypoint1, row.waypoint2])] = row.duration_s
    all_waypoints.update([row.waypoint1, row.waypoint2])

print(waypoint_distances)
