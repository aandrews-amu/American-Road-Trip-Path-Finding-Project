from data import all_waypoints


def print_options(waypoints):
    """
    :param all_waypoints: list of strings representing location data
    """
    for i in range(0, len(waypoints)):
        print(str(i) + ": " + waypoints[i] + "\n")


def choose_type():
    """
    """
    acceptable = False
    while not acceptable:
        user_types = input(
            "Please enter one of the following: parks, cities, both. \n")
        if user_types == "parks" or user_types == "cities" or user_types == "both":
            acceptable = True
        else:
            print("Please enter a valid input.")
    return user_types


def choose_options(types):
    done = False
    user_data = []

    # get data
    if types == "parks":
        data = all_waypoints
    elif types == "cities":
        data = all_waypoints
    else:
        data = all_waypoints

    # print waypoint options
    print_options(data)

    while not done:
        acceptable1 = False
        acceptable2 = False
        if len(user_data) >= 69:
            print("Reached limit.")
            done = True

        while not acceptable1:
            point = input(
                "Type a single integer to add a location to your desired list.\n")
            try:
                point = int(point)
                if point < len(data):
                    acceptable1 = True
            except ValueError:
                print("Invalid input.")

        while not acceptable2:
            loop = input("Would you like to input more location? (y/n)\n")
            if loop == "y" or loop == "n":
                acceptable2 = True
            else:
                print("Invalid input.")

        user_data.append(data[point])

        if loop == "n":
            done = True

    return user_data


my_type = choose_type()
finale = choose_options(my_type)
print(finale)
