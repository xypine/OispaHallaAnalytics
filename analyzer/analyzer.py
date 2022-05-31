# import random
from functools import reduce
import converter

time_default = 1652390000000
river_size = 0.1
river_max_length = 15

def jsbool(b: bool):
    return "true" if b else "false"

def a(data, write_time = True, write_complete = False):
    recordings = list(map( lambda i: i["recording"], data ))
    histories = list(map( lambda o: o["history"], recordings ))

    scores = list(map( lambda i: i["computed_score"], data ))
    dates = []
    for index, i in enumerate(data):
        # random.seed(len(histories[index]))
        # offset = random.random()*20000000
        val = i.get("timestamp_ms", time_default)
        dates.append(val)

    lenghts = []
    firsts = []
    lasts = []
    rep = []
    rep_c = []
    for index, frames in enumerate(histories):
        if write_time and dates[index] == time_default and not write_complete:
            pass
        else:
            states = list(map( lambda f: f[0], frames ))
            directions = list(map( lambda f: f[1], frames ))
            to_add = list(map( lambda f: f[2], frames ))
            lenghts.append(len(directions))
            firsts.append(directions[0])
            lasts.append(directions[-2])
            r = a_repeat_score(directions, 3)
            rep.append( r[0] )
            rep_c.append( r[1] )
    # print(f"Lenghts: {lenghts}")
    # print(f"Firsts: {firsts}")
    # print(f"Lasts: {lasts}")
    obj = {
        "data": 
        list(
            map(lambda i: 
                {
                    "time": dates[i],
                    "game_length": lenghts[i],
                    "score": scores[i],# if i < len(scores) else -1,
                    "won": jsbool(data[i]["won"]),
                    "abandoned": jsbool(data[i]["abandoned"]) if "abandoned" in data[i] else jsbool(False),
                    "move_first": firsts[i],
                    "move_last": lasts[i],
                    # "most_repeated": rep[i],
                    # "most_repeated_count": rep_c[i],
                    "repeat_score": float(rep_c[i]) / float(lenghts[i]),
                }
            , range(0, len(lenghts)))
            if write_time else
            map(lambda i: 
                {
                    "game_length": lenghts[i],
                    "score": scores[i],# if i < len(scores) else -1,
                    "won": jsbool(data[i]["won"]),
                    "abandoned": jsbool(data[i]["abandoned"]) if "abandoned" in data[i] else jsbool(False),
                    "move_first": firsts[i],
                    "move_last": lasts[i],
                    # "most_repeated": rep[i],
                    # "most_repeated_count": rep_c[i],
                    "repeat_score": float(rep_c[i]) / float(lenghts[i]),
                }
            , range(0, len(lenghts)))
        )
    }
    fname = "out_complete.csv" if write_complete else "out_time.csv" if write_time else "out_notime.csv"
    print(f"Writing {fname}...")
    converter.write_csv(obj, fname)

name_table = {
    "UP": "A",
    "RIGHT": "B",
    "DOWN": "C",
    "LEFT": "D",
    "END": "E"
}

def a_moves(data):
    recordings = list(map( lambda i: i["recording"], data ))
    histories = list(map( lambda o: o["history"], recordings ))

    data = list(
        reduce(
            lambda i, prev: prev + i
            ,
            list(
                map(lambda frames: 
                    list(map( lambda f: f[1], frames ))
                , histories)
            )
        )
    )

    obj = {
        "data": list(
            map(
                lambda i: {
                    "move": data[i]
                },
                range(len(data))
            )
        )
    }
    print(f"Writing out_moves_flat...")
    converter.write_csv(obj, "out_moves_flat.csv")

def move_n(n, i):
    return f"{name_table[n]}{i+1}"
def a_move_relations(data):
    recordings = list(map( lambda i: i["recording"], data ))
    histories = list(map( lambda o: o["history"], recordings ))

    lfrom = []
    lto = []
    for index, frames in enumerate(histories):
        states = list(map( lambda f: f[0], frames ))
        directions = list(map( lambda f: f[1], frames ))
        to_add = list(map( lambda f: f[2], frames ))
        
        for di, d in enumerate(directions):
            if di < len(directions) - 1 and di < river_max_length:
                n = move_n(d, di)
                n2v = directions[di+1]
                n2 = move_n(n2v, di+1)
                if di == 0:
                    lfrom.append("@0")
                    lto.append(n)
                lfrom.append(n)
                lto.append(n2)
    # print(lfrom)
    
    seen = []
    data = {}
    for i in range(0, len(lfrom)):
        f = lfrom[i]
        t = lto[i]
        if (f,t) in seen:
            o = data[(f,t)]
            o["Value"] = o["Value"] + river_size
            data[(f,t)] = o
        else:
            seen.append( (f,t) )
            data[ (f,t) ] = {
                "N1": f,
                "N2": t,
                "Value": river_size
            }
    
    obj = {
        "data": list(data.values())
    }
    print(f"Writing out_moves...")
    converter.write_csv(obj, "out_moves.csv")

def a_repeat_score(directions, group_size: int):
    counts = {}
    for i in range(len(directions) - group_size):
        group = []
        for g in range(group_size):
            direction = directions[i + g]
            group.append(direction)
        k = ",".join(group)
        if k in counts:
            counts[k] += 1
        else:
            counts[k] = 1
    if len(counts) > 0:
        m = max(counts, key=counts.get)
        return (m.split(","), counts[m])
    return ([], 0)

def filterd(data):
    return list(filter( lambda i: "recording" in i and "history" in i["recording"] and len(i["recording"]["history"]) > 1, data ))


def main():
    print("Loading games...")
    data: list = converter.load_all_and_merge("./data/")
    print(f"Found {len(data)} games...")
    # a_all

def a_all():
    print("Filtering data...")
    data = filterd(data)
    print(f"Filtered to {len(data)} games...")
    a(data, True)
    a(data, False)
    a(data, True, True)
    a_move_relations(data)
    a_moves(data)

if __name__ == "__main__":
    main()