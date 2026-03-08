#!/bin/bash
#BSUB -J AC_psi_benchmark[1-10]
#BSUB -q hpc
#BSUB -n 1
#BSUB -R "span[hosts=1]"
#BSUB -R "rusage[mem=4GB]"
#BSUB -W 01:00
#BSUB -o /zhome/52/3/214404/dev/DTU/AC/mpc-project/experiments/psi/outputs/%J_%I.out
#BSUB -e /zhome/52/3/214404/dev/DTU/AC/mpc-project/experiments/psi/errors/%J_%I.err

cd $HOME/dev/DTU/AC/mpc-project/

echo "$COMMIT,$(./experiments/$EXPERIMENT_NAME/bin/benchmark_$COMMIT $LSB_JOBINDEX)" > experiments/$EXPERIMENT_NAME/results/${LSB_JOBINDEX}.csv
