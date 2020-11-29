# 50 States Project

By Abigail Andrews, Emma Sheridan, and Wil Troxel <br />
<br />
We implemented a cross-country road trip planner using a constraint satisfaction problem combined with a branch and bound algorithm. We ran our code over 10 national park destinations to generate a roadtrip path plan. We also used code from Randy Olsen to run a genetic algorithm over our 10 destinations. Both algorithms successfully generated the same optimized path. We also implemented unit tests to ensure the correctness of our algorithm. We used timer functions to time the branch and bound and genetic algorithms to find that the genetic algorithm was much faster that the branch and bound algorithm, as expected given the worst case exponential run time of branch and bound (~8 seconds versus ~24 seconds). Below is a summary of our code files.
<br />

### Constraint Satisfaction Problem using Branch and Bound

#### src/opt.rs

- Implements a csp containing timestamp variables where each variable is associated with a corresponding vertex in the graph, except that the first and last timestamp visit the same vertex
- Constrains the csp so that each variable has a different value
- Uses the branch and bound solver to find the path with minimal distance by passing in a cost function
- Generates an ordered list representing the order to visit each vertex in the graph as well as the total cost for the trip

#### src/opt/pub_tests.rs

- Tests written to verify the correctness of the code
- test_tsp_4 tests our roadtrip function on 10 national park destinations where costs were generated using the Google Maps Distance Matrix API
- Implemented timers to time function calls to our csp using bnb

#### src/csp.rs, src/fd.rs, src/lib.rs

- Code taken from HW5 to implement our constraint optimization problem, our branch and bound solver, our fd constraint type, and our library functions

#### src/main.rs

- Runs our branch and bound solver on 10 selected locations, prints the order to visit the locations and the total distance, also calculates the time it takes to compute the route

### Genetic Algorithm

#### trip.py

- Code taken from Randal S. Olson "Computing the optimal road trip across the U.S." (© Randal S. Olson http://www.randalolson.com/)
- Calls GoogleMaps distance matrix API to fetch distance and duration data between all combinations of data points
- Outputs the from point, to point, distance, and duration to my-waypoints-dist-dur.tsv file to later be parsed by algorithm
- Runs genetic algorithm over the given waypoints for 5000 generations and 100 as population size
- Outputs an ordered list of waypoints representing the best found path after all the generations
- We added code to time the genetic function

#### my-waypoints-dist-dur7.numbers

- Example output generated by the API calls

### How to run

To run the branch and bound solver, you'll need the whole final-project folder. Run main.rs to see solver work for the 10 selected locations.

### What we learned

### What we wish went differently
