#!/usr/bin/env python3

# std import
import os
import sys
import argparse
import subprocess
from collections import defaultdict

from typing import Set, List

class keydefaultdict(defaultdict):
    def __missing__(self, key):
        if self.default_factory is None:
            raise KeyError( key )
        else:
            ret = self[key] = self.default_factory(key)
            return ret

# main function to parse argument
def main(args=None):

    if args is None:
        args = sys.argv[1:]

    parser = argparse.ArgumentParser()

    parser.add_argument("-p", "--position", help="Position path")
    parser.add_argument("-m", "--mapping", help="Mapping of contig against reference path")
    parser.add_argument("-g", "--graph", help="Graph path format need to respect python format syntaxe chromosome name is include in place of '{chrom}'")
    parser.add_argument("-d", "--depth", help="Depth around nodes", default=5, type=int)
    parser.add_argument("-c", "--position-correction", help="Modification apply on position to be sure we catch tig around position", default=1_000, type=int)
    parser.add_argument("-o", "--out-prefix", help="Prefix of output")

    args = parser.parse_args(args)

    run(args.position, args.mapping, args.graph, args.depth, args.position_correction, args.out_prefix)
    
# working function
def run(positions_path: str, mapping_path: str, graph_path_format: str, depth: int, correction: int, out_path_prefix: str) -> None:
    # get avaible path
    avaible_tig_collection = keydefaultdict(lambda graph_path: tig_in_graph(graph_path))

    # for each line
    #     get position
    #     get tig at position
    #     get avaible tig at position
    #     generate subgraph image
    #     generate subgraph
    with open(positions_path) as pos_fh:
        next(pos_fh) # skip header
        for line in pos_fh:
            (chrom, begin, end) = line.split()[:3]
            (begin, end) = (int(begin), int(end))
            graph_path = graph_path_format.format(chrom=chrom)

            if not os.path.exists(graph_path):
                continue
            
            avaible_tig = avaible_tig_collection[graph_path]
            tigs = avaible_tig & tig_on_positions(chrom, begin-correction, end+correction, mapping_path)
            if tigs:
                generate_image(sorted(list(tigs)), graph_path, out_path_prefix, f"{chrom}:{begin}-{end}_{','.join(tigs)}", depth)
                generate_subgraph(sorted(list(tigs)), graph_path, out_path_prefix, f"{chrom}:{begin}-{end}_{','.join(tigs)}", depth)
            else:
                print(f"Error durring extraction of tig around region '{chrom}:{begin}-{end}' no tigs around position is present in graph file.", file=sys.stderr)


def tig_in_graph(graph_path: str) -> Set[str]:
    with open(graph_path) as graph_fh:
        return {line.split("\t")[1] for line in graph_fh if line.startswith("S")};

    
def tig_on_positions(chrom: str, begin: int, end: int, mapping_path: str) -> set:
    p = subprocess.Popen(["samtools", "view", mapping_path, f"{chrom}:{begin}-{end}"], stdout=subprocess.PIPE, universal_newlines=True)

    stdout, stderr = p.communicate()

    status = p.wait()

    if status == 0:
        return {line.split('\t')[0] for line in stdout.split('\n')}
    else:
        print(f"Error durring extraction of tig around region '{chrom}:{begin}-{end}' with samtools on file '{mapping_path}', status {status}", file=sys.stderr)
        print(f"{stderr}", file=sys.stderr)
        return {}

    
def generate_subgraph(tigs: List[str], graph_path: str, prefix: str, suffix: str, depth: int):
    p = subprocess.Popen(["Bandage", "reduce", graph_path, f"{prefix}_{suffix}.gfa", "--scope", "aroundnodes", "--nodes", f"{','.join(tigs)}", "--distance", f"{depth}", "--colour", "uniform", "--edgelen", "50"], stdout=subprocess.PIPE, universal_newlines=True)

    stdout, stderr = p.communicate()

    status = p.wait()

    if status != 0:
        print(f"Error durring generation of subgraph around region '{suffix}' with Bandage and tigs: '{tigs}', status {status}", file=sys.stderr)
        print(f"{stderr}", file=sys.stderr)

    
def generate_image(tigs: List[str], graph_path: str, prefix: str, suffix: str, depth: int):
    p = subprocess.Popen(["Bandage", "image", graph_path, f"{prefix}_{suffix}.svg", "--scope", "aroundnodes", "--nodes", f"{','.join(tigs)}", "--distance", f"{depth}", "--colour", "uniform", "--edgelen", "50"], stdout=subprocess.PIPE, universal_newlines=True)

    stdout, stderr = p.communicate()

    status = p.wait()

    if status != 0:
        print(f"Error durring generation of image of graph around region '{suffix}' with Bandage and tigs: '{tigs}', status {status}", file=sys.stderr)
        print(f"{stderr}", file=sys.stderr)

        
if __name__ == "__main__":
    main(sys.argv[1:])
