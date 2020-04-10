rule index_asm:
    input:
        asm = "assemblies/{asm}.fasta"
    output:
        index = "assemblies/{asm}.mmi"
    threads:
        48
    shell:
        "minimap2 -t {threads} -x map-pb -d {output} {input}"

rule map_reads2asm:
    input:
        reads = "reads/{reads}.fastq.gz",
        asm = "assemblies/{asm}.mmi",
    output:
        "mapping/{reads}_2_{asm}.paf"
    threads:
        48
    shell:
        "minimap2 -t {threads} -x map-pb --eqx -m 5000 --secondary=no {input.asm} {input.reads} > {output}"
        
rule separate_contig_by_cluster:
    input:
        asm = "assemblies/{codename}_{readtype}.{haplo}.fasta",
    output:
        "assemblies/{codename}_{readtype}.{haplo}.cluster1.fasta"
    shell:
        "./target/release/separate_by_chr -a {input.asm} -p assemblies/{wildcards.codename}_{wildcards.readtype}.{wildcards.haplo}."

rule map_reads2reads:
    input:
        "reads/{filename}.fastq.gz"
    output:
        "overlap/{filename}.paf"
    threads:
        48
    shell:
        "minimap2 -t {threads} -x ava-pb {input} {input} > {output}"

rule map_ctgreads2ctgreads:
    input:
        "reads+ctg/{read}_fb_{tig}.fasta"
    output:
        "graph/{read}_fb_{tig}.paf"
    threads:
        48
    shell:
        "minimap2 -t {threads} -x ava-pb {input} {input} > {output}"
                
rule filter_reads:
    input:
        overlap = "mapping/{reads}_2_{asm}.paf",
        reads = "reads/{reads}.fastq.gz",
        assembly = "assemblies/{asm}.fasta",
        self_ovl = "overlap/{reads}.paf"
    output:
        reads = "reads+ctg/{reads}_fb_{asm}_d{distance}.fasta",
        assignation = "assignation/{reads}_fb_{asm}_d{distance}.csv"
    wildcard_constraints:
        distance = "\d+"
    shell:
        "./target/release/filter_fastx -m {input.overlap} -r {input.self_ovl} -i {input.reads} -A {input.assembly} -o {output.reads} -a {output.assignation} -d {wildcards.distance}"
            
rule ovl2ovl_graph:
    input:
        "graph/{filename}.paf"
    output:
        "graph/{filename}.gfa"
    shell:
        "fpa -i {input} -o /dev/null drop -l 2000 gfa -o {output}"

rule add_seq:
    input:
        graph = rules.ovl2ovl_graph.output,
        reads = "reads+ctg/{filename}.fasta"
    output:
        "graph/{filename}_with_seq.gfa"
    shell:
        "./script/add_sequence_in_gfa.py -g {input.graph} -r {input.reads} -o {output}"

ruleorder: add_seq > ovl2ovl_graph
ruleorder: add_seq > ovl2ovl_graph
rule all:
    input:
        ["graph/HG00733_ccs_h1_cluster{}_fb_HG00733_ccs.h1.cluster{}_d5000_with_seq.gfa".format(i, i) for i in range(1, 26)],
        ["graph/HG00733_ccs_h2_cluster{}_fb_HG00733_ccs.h2.cluster{}_d5000_with_seq.gfa".format(i, i) for i in range(1, 26)]
