import sys

def find_intersections(seed_range, cur_map_ranges):
    
    intersections = []
    rerun_ranges = []
    seed_range_start = seed_range[0]
    seed_range_end = seed_range[1]
    
    # map the entire seed range to the current mappings' ranges
    for map_range in cur_map_ranges:
        
        # identify start and end of the intersection
        intersection_start = max(seed_range_start, map_range[1])
        intersection_end = min(seed_range_end, map_range[2])
        
        # if there is an intersection...
        if intersection_start <= intersection_end:
                    
            # re-run any sub-range that occurs before the intersection, as it might map to other map ranges. 
            if seed_range_start < intersection_start:
                rerun_ranges.append((seed_range_start, intersection_start - 1))
            
            # grab the intersection and add it to new ranges list
            intersection = (intersection_start + (map_range[0] - map_range[1]), intersection_end + (map_range[0] - map_range[1]))
            intersections.append(intersection)
            
            # move seed_range_start so we can search remainder of seed range
            seed_range_start = intersection_end + 1
            

    # any remaining seed range is greater than mapping max value, so it will map to itself. 
    if seed_range_start < seed_range_end:
        intersections.append((seed_range_start, seed_range_end))
        
    intersections = sorted(intersections, key=lambda x: x[0])
    return intersections, rerun_ranges



def part2():
    with open('data/day_5_input.txt', 'r') as file:
        # Read the content of the file
        file_content = file.read()

        # Split the content based on "\n\n"
        sections = file_content.split('\n\n')

    # create x-to-x mappings
    map_ranges = []
    for section_idx, section in enumerate(sections[1:]):
        tuples = [tuple for tuple in section.split("\n")[1:] if tuple]
        map_ranges.append([])
        for tuple in tuples:
            numbers = [int(i) for i in tuple.split()]
            cur_range = (numbers[0], numbers[1], numbers[1] + numbers[2] - 1)
            map_ranges[section_idx].append(cur_range)  
    
    # create all seed ranges
    seeds = [int(seed.strip()) for seed in sections[0].split("seeds: ")[-1].split(" ") if seed]
    seed_ranges = []
    for idx in range(0, len(seeds), 2):
        cur_range = (seeds[idx], seeds[idx] + seeds[idx + 1] - 1)
        seed_ranges.append(cur_range)
    seed_ranges = sorted(seed_ranges, key=lambda x: x[0])
    
    # go through each x-to-x map and re-map the current seed ranges to it. 
    for idx, cur_map_range in enumerate(map_ranges):
        new_seed_ranges = []
        
        # re-map all seed ranges based on current map
        for seed_range in seed_ranges:
            mapped_ranges_found, rerun_ranges = find_intersections(seed_range, cur_map_range)
            new_seed_ranges += mapped_ranges_found
            
            # if there were any sub-ranges that need mapping, keep rerunning till mapped.
            while rerun_ranges:
                for rerun_range in rerun_ranges:
                    mapped_ranges_found, rerun_ranges = find_intersections(rerun_range, cur_map_range)
                    new_seed_ranges += mapped_ranges_found
        
        seed_ranges = new_seed_ranges 
    
    return min(seed_ranges, key=lambda x: x[0])[0]



if __name__=="__main__":
    print(part2())
