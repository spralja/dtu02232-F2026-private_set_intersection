# PSI Benchmark Analysis Summary

The runtime behavior when set sizes vary, especially when increasing |Y| while keeping |X| fixed.

## Data overview
- Number of measurements: 364
- Tested |X| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
- Tested |Y| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
- Tested |I| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]

## Key observations from the generated plots
- Unbalanced case (|Y|=1, |I|=1): runtime rises from 0 ms at |X|=1 to 190 ms at |X|=2048.
- Fixed |X|=2048 with |I|=1: runtime rises from 190 ms at |Y|=1 to 599 ms at |Y|=2048.
- For balanced runs (|X|=|Y|), changing |I| has only a small effect compared to changing |Y| and |X|.
- The heatmap shows that runtime is dominated by set cardinalities, with larger |X| and |Y| combinations giving the highest costs.

## Generated artifacts
- plots/runtime_vs_x_unbalanced_y1_i1.png
- plots/runtime_vs_y_fixed_x2048_i1.png
- plots/runtime_vs_intersection_balanced.png
- plots/runtime_heatmap_x_y_avg_over_i.png
