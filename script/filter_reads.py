#!/usr/bin/env python3

import csv
import sys
import gzip
import argparse

from collections import defaultdict

from Bio import SeqIO

def get_tig2posread(read2tig):
    result = defaultdict(list)

    reader = csv.reader(read2tig, delimiter="\t")
    for row in reader:
        if int(row[3]) - int(row[2]) > 0.7 * int(row[1]):
            result[(row[5], int(row[6]))].append((int(row[7]), int(row[8]), row[0], int(row[2]), int(row[1]) - int(row[3]), row[4]))

    return result

def main(args=None):

    if args is None:
        args = sys.argv[1:]

    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--paf", help="path to overlap file")
    parser.add_argument("-o", "--output", help="path where select reads is write")
    parser.add_argument("-a", "--assignation", help="path to write reads assignation")
    parser.add_argument("-d", "--distance", help="distance to extremity")

    args = parser.parse_args(args)

    tig2readspos = get_tig2posread(open(args.paf))

    read2tig = dict()
    with open(args.output, "w") as fh:
        for tig, val in tig2readspos.items():
            t = max(tig[1]*0.05, float(args.distance))
            for v in val:
                if v[0] > t and v[1] < tig[1] - t:
                    pass
                else:
                    read2tig[tig] = v[2]
                    fh.write(f"{v[2]}\n")

    with open(args.assignation, 'w') as fh:
        fh.write("tig,read\n")
        for tig, read in read2tig.items():
            fh.write(f"{tig[0]},{read}\n")
            
if __name__ == "__main__":
    main(args=sys.argv[1:])
