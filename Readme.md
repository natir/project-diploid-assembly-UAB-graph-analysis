# Reference-free diploid assembly of a human genome breakpoint graph analysis

## Requirements

You need all this in your path (all this tools is avaible in bioconda):
- [Bandage](https://github.com/rrwick/Bandage)
- [Minimap2](https://github.com/lh3/minimap2)
- [Fpa](https://github.com/natir/fpa/)
- [Snakemake](https://snakemake.readthedocs.io/en/stable/)

You also need a [Rust toolchain](https://rustup.rs/) setup on your system

## Usage

### Build bin script

```
cargo build --release
```

### File setup

The directory **Must** contains exactly this file:
```
assemblies/HG00733_ccs.h1.fasta # The ccs assemblies for haplotype 1
assemblies/HG00733_ccs.h2.fasta # The ccs assemblies for haplotype 2
reads/HG00733_ccs_h1_cluster[1-25].fastq.gz # The ccs reads for haplotype 1 one file per cluster
reads/HG00733_ccs_h1_cluster[1-25].fastq.gz # The ccs reads for haplotype 2 one file per cluster
```

### Run analysis

```
snakemake -s pipeline/build_graph.snakefile assemblies/HG00733_ccs.h1.cluster1.fasta
snakemake -s pipeline/build_graph.snakefile all 
```

Graph for each cluster is avaible in directory `graph`

### Subgraph extraction

```
mkdir -p subgraph/h1
./script/generate_sub_graph.py -p sharedBreaks.HG00733.txt -m infos/HG00733_hgsvc_pbsq2-ccs_1000-pereg.h1-un.racon-p2.bed -g "graph/HG00733_ccs_h1_cluster8_fb_HG00733_ccs.h1.{cluster}_d5000.gfa" -d 10 -c 5000 -o subgraph/h1/

mkdir -p subgraph/h2
./script/generate_sub_graph.py -p sharedBreaks.HG00733.txt -m infos/HG00733_hgsvc_pbsq2-ccs_1000-pereg.h2-un.racon-p2.bed -g "graph/HG00733_ccs_h2_cluster8_fb_HG00733_ccs.h2.{cluster}_d5000.gfa" -d 10 -c 5000 -o subgraph/h2/
```

Subgraph are store in directory `subgraph/h1` and `subgraph/h2`
