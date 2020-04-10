#!/usr/bin/env python3

import csv
import sys
import argparse

def main(args=None):

    if args is None:
        args = sys.argv[1:]

    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gfa", help="path to graph file")
    parser.add_argument("-a", "--assignation", help="path eads assignation to contig")
    parser.add_argument("-o", "--output", help="path to write edited graph")

    args = parser.parse_args(args)

    with open(args.output, 'w') as out:
        with open(args.gfa) as gfa:
            for line in gfa:
                out.write(line)
            
            with open(args.assignation) as assignation:
                tigs = dict()
                reader = csv.DictReader(assignation)
                for row in reader:
                    tigs[row["tig"]] = row["tig_len"]
                    out.write("L\t{}\t+\t{}\t+\t10M\n".format(row["tig"], row["read"]))

                for (tig, len) in tigs.items():
                    out.write("S\t{}\t*\tLN:i:{}\n".format(tig, len))
            
    
if __name__ == "__main__":
    main(args=sys.argv[1:])
