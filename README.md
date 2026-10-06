rust implementation of md5sum of readname in fastq.gz file.
it is aimed to yield same md5sum of following bash pipeline.

`zcat input.fastq.gz | awk 'NR % 4 == 1' | md5sum`
