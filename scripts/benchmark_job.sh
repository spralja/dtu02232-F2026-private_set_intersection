#!/bin/bash
#BSUB -J AC_psi_benchmark[1-10]
#BSUB -q hpc
#BSUB -n 1
#BSUB -R "span[hosts=1]"
#BSUB -R "rusage[mem=4GB]"
#BSUB -W 01:00
#BSUB -o $HOME/dev/DTU/AC/mpc-project/experiments/psi/outputs/%J_%I.out
#BSUB -e $HOME/dev/DTU/AC/mpc-project/experiments/psi/errors/%J_%I.err

cd $HOME/dev/DTU/AC/mpc-project/

COMMIT=$(git rev-parse --short HEAD)

./experiments/psi/benchmark_$COMMIT $LSB_JOBINDEX
