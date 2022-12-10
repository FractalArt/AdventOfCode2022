with open('data/day_07.txt', 'r') as f:

    current = "/"
    parents = []
    dir_sizes = {}
    files_handled = []

    for line in f.readlines():
        line = line.strip()
        print(line)

        if line.startswith("$ cd /"):
            current = "/"
            if not "/" in dir_sizes:
                dir_sizes["/"] = 0
            parents = []
        elif line.startswith("$ cd .."):
            current = parents.pop()
        elif line.startswith("$ cd"):
            d = line.split(' ')[-1]
            d_full = "-".join(parents) + "-" + d if len(parents) else d
            if d_full not in dir_sizes:
                dir_sizes[d_full] = 0
            parents.append(current)
            current = d
        elif line.startswith("$ ls"):
            continue
        elif line.startswith("dir"):
            d = line.split(' ')[-1]
            d_full = "-".join(parents) + "-" + d if len(parents) else d
            if d_full not in dir_sizes:
                dir_sizes[d_full] = 0
        else:
            size, file = line.split(' ')
            size = int(size)
            for idx, parent in enumerate(parents):
                parent_name = "-".join(parents[:idx+1])
                file_name = parent_name + "-" + file
                if file_name not in files_handled:
                    if parent_name not in dir_sizes:
                        dir_sizes[parent_name] = size
                    else:
                        dir_sizes[parent_name] += size
                    files_handled.append(file_name)
            current_name = "-".join(parents) + "-" + current if len(parents) else current
            file_name = current_name + "-" + file
            if file_name not in files_handled:
                if current_name not in dir_sizes:
                    dir_sizes[current_name] = size
                else:
                    dir_sizes[current_name] += size
                
                files_handled.append(file_name)

    sum_small = 0
    
    for key, val in dir_sizes.items():
        if val <= 100000:
            sum_small += val

    print(sum_small)


            

