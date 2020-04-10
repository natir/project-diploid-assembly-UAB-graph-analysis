#!/usr/bin/env python3

import csv
import sys
import gzip
import argparse

from Bio import SeqIO

def main(args=None):

    if args is None:
        args = sys.argv[1:]

    parser = argparse.ArgumentParser()
    parser.add_argument("-g", "--gfa", help="path to graph file")
    parser.add_argument("-r", "--reads", help="path to indexed fasta")
    parser.add_argument("-o", "--output", help="path to write edited graph")

    args = parser.parse_args(args)

    with gzip.open(args.reads, "rt") as reads_hl:
        reads = {record.id: record.seq for record in SeqIO.parse(reads_hl, "fasta")}
    
    with open(args.output, "w") as out:
        with open(args.gfa) as gfa:
            for line in gfa:
                if line.startswith("S"):
                    row = line.split("\t")
                    print(f"S\t{row[1]}\t{reads[row[1]]}", file=out)
                else:
                    print(f"{line}", file=out, end="")
    
if __name__ == "__main__":
    main(args=sys.argv[1:])
