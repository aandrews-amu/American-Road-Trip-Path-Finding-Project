"""
Abigail Andrews, Emma Sheridan, and Wil Troxel
50 States Project 
"""

# {frozenset({'Acadia, Maine', 'Arches, Utah'}): 4158437, frozenset({'Acadia, Maine', 'Badlands, South Dakota'}): 3361923, frozenset({'Acadia, Maine', 'Big Bend, Texas'}): 4122920, frozenset({'Acadia, Maine', 'Biscayne, Florida'}): 2841353, frozenset({'Acadia, Maine', 'Black Canyon of the Gunnison, Colorado'}): 4015523, frozenset({'Bryce Canyon, Utah', 'Acadia, Maine'}): 4466669, frozenset({'Canyonlands, Utah', 'Acadia, Maine'}): 4177816, frozenset({'Acadia, Maine', 'Capitol Reef, Utah'}): 4283137, frozenset({'Badlands, South Dakota', 'Arches, Utah'}): 1157035, frozenset({'Big Bend, Texas', 'Arches, Utah'}): 1422964, frozenset({'Biscayne, Florida', 'Arches, Utah'}): 3756536, frozenset({'Black Canyon of the Gunnison, Colorado', 'Arches, Utah'}): 288098, frozenset({'Bryce Canyon, Utah', 'Arches, Utah'}): 396884, frozenset({'Canyonlands, Utah', 'Arches, Utah'}): 42136, frozenset({'Capitol Reef, Utah', 'Arches, Utah'}): 213353, frozenset({'Big Bend, Texas', 'Badlands, South Dakota'}): 1780866, frozenset({'Biscayne, Florida', 'Badlands, South Dakota'}): 3381334, frozenset({'Badlands, South Dakota', 'Black Canyon of the Gunnison, Colorado'}): 1014852, frozenset({'Bryce Canyon, Utah', 'Badlands, South Dakota'}): 1468900, frozenset({'Canyonlands, Utah', 'Badlands, South Dakota'}): 1177146, frozenset({'Capitol Reef, Utah', 'Badlands, South Dakota'}): 1282467, frozenset({'Big Bend, Texas', 'Biscayne, Florida'}): 2844076, frozenset({'Big Bend, Texas', 'Black Canyon of the Gunnison, Colorado'}): 1337443, frozenset({'Bryce Canyon, Utah', 'Big Bend, Texas'}): 1738584, frozenset({'Canyonlands, Utah', 'Big Bend, Texas'}): 1471158, frozenset({'Capitol Reef, Utah', 'Big Bend, Texas'}): 1622912, frozenset({'Biscayne, Florida', 'Black Canyon of the Gunnison, Colorado'}): 3514331, frozenset({'Bryce Canyon, Utah', 'Biscayne, Florida'}): 4036594, frozenset({'Canyonlands, Utah', 'Biscayne, Florida'}): 3730705, frozenset({'Capitol Reef, Utah', 'Biscayne, Florida'}): 4021595, frozenset({'Bryce Canyon, Utah', 'Black Canyon of the Gunnison, Colorado'}): 597235, frozenset({'Canyonlands, Utah', 'Black Canyon of the Gunnison, Colorado'}): 308383, frozenset({'Capitol Reef, Utah', 'Black Canyon of the Gunnison, Colorado'}): 413704, frozenset({'Bryce Canyon, Utah', 'Canyonlands, Utah'}): 416132, frozenset({'Bryce Canyon, Utah', 'Capitol Reef, Utah'}): 183661, frozenset({'Canyonlands, Utah', 'Capitol Reef, Utah'}): 232774}
from typing import List, Optional, Callable, Tuple
from random import choices, randint, randrange, random
import random
from data import all_waypoints
# from userdata import all_waypoints

# generate a random roadtrip


def generate_random_roadtrip():
    new_random_roadtrip = list(all_waypoints)
    random.shuffle(new_random_trip)
    return tuple(new_random_roadtrip)

# generate a list of with pop_size number of road trips
# takes no parameters but pop_size would be 100


def create_population(pop_size):
    random_pop = []
    for roadtrip in range(pop_size):
        random_pop.append(generate_random_roadtrip())
    return random_pop


def fitness(path):
    fitness = 0.0
    for i in range(len(path)):
        waypoint_from = path[i - 1]
        waypoint_to = path[i]
        fitness += waypoint_durations[frozenset([waypoint_from, waypoint_to])]
    return fitness

# modified tournament selection -- select 10 roadtrips at random and pick the best two


def select_pair(population):
    fitnesses = []

    # randomly pick 10 from population
    tournament = choices(
        population,
        k=10
    )

    # find fitness for each
    for i in tournament:
        fitnesses.append(fitness(i))
    fitnesses.sort()

    # return best 2
    return (fitnesses[0], fitnesses[1])


def crossover(waypoint_a, waypoint_b):


def mutate():


def genetic_algorithm():

    #     new_random_agent = list(all_waypoints)
    #     random.shuffle(new_random_agent)
    #     return tuple(new_random_agent)


Genome = List[int]
Population = List[Genome]
PopulateFunc = Callable[[], Population]
FitnessFunc = Callable[[Genome], int]
SelectionFunc = Callable[[Population, FitnessFunc], Tuple[Genome, Genome]]
CrossoverFunc = Callable[[Genome, Genome], Tuple[Genome, Genome]]
MutationFunc = Callable[[Genome], Genome]
PrinterFunc = Callable[[Population, int, FitnessFunc], None]


def generate_genome(length: int) -> Genome:
    return choices([0, 1], k=length)


def generate_population(size: int, genome_length: int) -> Population:
    return [generate_genome(genome_length) for _ in range(size)]


def single_point_crossover(a: Genome, b: Genome) -> Tuple[Genome, Genome]:
    if len(a) != len(b):
        raise ValueError("Genomes a and b must be of same length")

    length = len(a)
    if length < 2:
        return a, b

    p = randint(1, length - 1)
    return a[0:p] + b[p:], b[0:p] + a[p:]


def mutation(genome: Genome, num: int = 1, probability: float = 0.5) -> Genome:
    for _ in range(num):
        index = randrange(len(genome))
        genome[index] = genome[index] if random(
        ) > probability else abs(genome[index] - 1)
    return genome


def population_fitness(population: Population, fitness_func: FitnessFunc) -> int:
    return sum([fitness_func(genome) for genome in population])


def selection_pair(population: Population, fitness_func: FitnessFunc) -> Population:

    return choices(
        population=population,
        weights=[fitness_func(gene) for gene in population],
        k=2
    )


def sort_population(population: Population, fitness_func: FitnessFunc) -> Population:
    return sorted(population, key=fitness_func, reverse=True)


def genome_to_string(genome: Genome) -> str:
    return "".join(map(str, genome))


def print_stats(population: Population, generation_id: int, fitness_func: FitnessFunc):
    print("GENERATION %02d" % generation_id)
    print("=============")
    print("Population: [%s]" % ", ".join(
        [genome_to_string(gene) for gene in population]))
    print("Avg. Fitness: %f" % (population_fitness(
        population, fitness_func) / len(population)))
    sorted_population = sort_population(population, fitness_func)
    print(
        "Best: %s (%f)" % (genome_to_string(sorted_population[0]), fitness_func(sorted_population[0])))
    print("Worst: %s (%f)" % (genome_to_string(sorted_population[-1]),
                              fitness_func(sorted_population[-1])))
    print("")

    return sorted_population[0]


def run_evolution(
        populate_func: PopulateFunc,
        fitness_func: FitnessFunc,
        fitness_limit: int,
        selection_func: SelectionFunc = selection_pair,
        crossover_func: CrossoverFunc = single_point_crossover,
        mutation_func: MutationFunc = mutation,
        generation_limit: int = 100,
        printer: Optional[PrinterFunc] = None) \
        -> Tuple[Population, int]:
    population = populate_func()

    for i in range(generation_limit):
        population = sorted(
            population, key=lambda genome: fitness_func(genome), reverse=True)

        if printer is not None:
            printer(population, i, fitness_func)

        if fitness_func(population[0]) >= fitness_limit:
            break

        next_generation = population[0:2]

        for j in range(int(len(population) / 2) - 1):
            parents = selection_func(population, fitness_func)
            offspring_a, offspring_b = crossover_func(parents[0], parents[1])
            offspring_a = mutation_func(offspring_a)
            offspring_b = mutation_func(offspring_b)
            next_generation += [offspring_a, offspring_b]

        population = next_generation

    return population, i

# import random
# from data import all_waypoints
# from distances import *


# def compute_fitness(solution):
#     """
#         This function returns the total distance traveled on the current road trip.

#         The genetic algorithm will favor road trips that have shorter
#         total distances traveled.
#     """

#     solution_fitness = 0.0

#     for index in range(len(solution)):
#         waypoint1 = solution[index - 1]
#         waypoint2 = solution[index]
#         solution_fitness += waypoint_distances[frozenset(
#             [waypoint1, waypoint2])]

#     return solution_fitness


# def generate_random_agent():
#     """
#         Creates a random road trip from the waypoints.
#     """

#     new_random_agent = list(all_waypoints)
#     random.shuffle(new_random_agent)
#     return tuple(new_random_agent)


# def mutate_agent(agent_genome, max_mutations=3):
#     """
#         Applies 1 - `max_mutations` point mutations to the given road trip.

#         A point mutation swaps the order of two waypoints in the road trip.
#     """

#     agent_genome = list(agent_genome)
#     num_mutations = random.randint(1, max_mutations)

#     for mutation in range(num_mutations):
#         swap_index1 = random.randint(0, len(agent_genome) - 1)
#         swap_index2 = swap_index1

#         while swap_index1 == swap_index2:
#             swap_index2 = random.randint(0, len(agent_genome) - 1)

#         agent_genome[swap_index1], agent_genome[swap_index2] = agent_genome[swap_index2], agent_genome[swap_index1]

#     return tuple(agent_genome)


# def shuffle_mutation(agent_genome):
#     """
#         Applies a single shuffle mutation to the given road trip.

#         A shuffle mutation takes a random sub-section of the road trip
#         and moves it to another location in the road trip.
#     """

#     agent_genome = list(agent_genome)

#     start_index = random.randint(0, len(agent_genome) - 1)
#     length = random.randint(2, 20)

#     genome_subset = agent_genome[start_index:start_index + length]
#     # genome_subset.reverse() # gives us a worse route if we reverse the subsets
#     agent_genome = agent_genome[:start_index] + \
#         agent_genome[start_index + length:]

#     insert_index = random.randint(
#         0, len(agent_genome) + len(genome_subset) - 1)
#     agent_genome = agent_genome[:insert_index] + \
#         genome_subset + agent_genome[insert_index:]

#     return tuple(agent_genome)


# def generate_random_population(pop_size):
#     """
#         Generates a list with `pop_size` number of random road trips.
#     """

#     random_population = []
#     for agent in range(pop_size):
#         random_population.append(generate_random_agent())
#     return random_population


# def run_genetic_algorithm(generations=5000, population_size=100):
#     """
#         The core of the Genetic Algorithm.

#         `generations` and `population_size` must be a multiple of 10.
#     """

#     population_subset_size = int(population_size / 10.)
#     generations_10pct = int(generations / 10.)

#     # Create a random population of `population_size` number of solutions.
#     population = generate_random_population(population_size)

#     # For `generations` number of repetitions...
#     for generation in range(generations):

#         # Compute the fitness of the entire current population
#         population_fitness = {}

#         for agent_genome in population:
#             if agent_genome in population_fitness:
#                 continue

#             population_fitness[agent_genome] = compute_fitness(agent_genome)

#         # Take the top 10% shortest road trips and produce offspring each from them
#         new_population = []
#         for rank, agent_genome in enumerate(sorted(population_fitness,
#                                                    key=population_fitness.get)[:population_subset_size]):

#             if (generation % generations_10pct == 0 or generation == generations - 1) and rank == 0:
#                 print("Generation %d best: %d | Unique genomes: %d" % (generation,
#                                                                        population_fitness[agent_genome],
#                                                                        len(population_fitness)))
#                 print(agent_genome)
#                 print("")

#             # Create 1 exact copy of each of the top road trips
#             new_population.append(agent_genome)

#             # Create 2 offspring with 1-3 point mutations
#             # for offspring in range(2):
#             #    new_population.append(mutate_agent(agent_genome, 3))

#             # Create 7 offspring with a single shuffle mutation
#             for offspring in range(7):
#                 new_population.append(shuffle_mutation(agent_genome))

#         # Replace the old population with the new population of offspring
#         for i in range(len(population))[::-1]:
#             del population[i]

#         population = new_population


# if __name__ == '__main__':
#     run_genetic_algorithm(generations=5000, population_size=100)
