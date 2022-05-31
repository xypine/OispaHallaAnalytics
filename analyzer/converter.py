from functools import reduce
import json
import os

from iteration_utilities import unique_everseen

def load(path):
    try:
        with open(path, "r") as f:
            content = json.load(f)
        gcount = len(content["data"])
        print(f"loaded {gcount} games from {path}...")
        return content
    except:
        print(f"WARNING: failed to load {path}")
        return {
            "data": []
        }

def load_all_and_merge(directory):
    data = {}
    files = [pos_json for pos_json in os.listdir(directory) if pos_json.endswith('.json')]
    data = list(map(lambda fname: load(directory + fname), files))
    print("De-duplicating games...")
    return reduce(lambda i, prev: list(unique_everseen( prev + i )), list(map(lambda o: o["data"], data))) # we use extend to remove duplicates


def write_csv(content, path):
    import csv
    csv_blacklist = ["recording"]
    with open(path, "w") as f:
        writer = csv.writer(f, delimiter=',')
        writer.writerow(filter(lambda x: not (x in csv_blacklist),list(content["data"][0])))
        for entry in content["data"]:
            i = filter(lambda x: not (x in csv_blacklist), list(entry))
            writer.writerow(map(lambda x: entry[x], i))


if __name__ == "__main__":
    print("Opening & Parsing file...")
    content = load_all_and_merge("./data/")
    print("Done!")

    print("Saving .csv...")
    write_csv({
        "data": content
    }, "./data/data.csv")