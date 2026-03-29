# PSI Benchmark Analysis Summary

The report requirement asks for runtime behavior when set sizes vary, especially when increasing |X| while keeping |Y| fixed.

## Data overview
- Number of measurements: 364
- Tested |X| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
- Tested |Y| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
- Tested |I| values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]

## Key observations from the generated plots
- Unbalanced case (|X|=1, |I|=1): runtime rises from 0 ms at |Y|=1 to 295 ms at |Y|=2048.
- Fixed |Y|=2048 with |I|=1: runtime rises from 295 ms at |X|=1 to 595 ms at |X|=2048.
- For balanced runs (|X|=|Y|), changing |I| has only a small effect compared to changing |X| and |Y|.
- The heatmap shows that runtime is dominated by set cardinalities, with larger |X| and |Y| combinations giving the highest costs.

## Generated artifacts
- plots/runtime_vs_y_unbalanced_x1_i1.png
- plots/runtime_vs_x_fixed_y2048_i1.png
- plots/runtime_vs_intersection_balanced.png
- plots/runtime_heatmap_x_y_avg_over_i.png
