from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

ROOT = Path(__file__).resolve().parent
CSV_PATH = ROOT / "results.csv"
PLOTS_DIR = ROOT / "plots"
REPORT_PATH = ROOT / "analysis_summary.md"


sns.set_theme(style="whitegrid")


def load_data() -> pd.DataFrame:
    df = pd.read_csv(CSV_PATH)
    for col in ["x_size", "y_size", "i_size", "mean_ms", "stddev_ms"]:
        df[col] = pd.to_numeric(df[col])
    return df.sort_values(["x_size", "y_size", "i_size"]).reset_index(drop=True)


def save_plot_runtime_vs_y_unbalanced(df: pd.DataFrame) -> None:
    scenario = df[(df["x_size"] == 1) & (df["i_size"] == 1)].copy()
    fig, ax = plt.subplots(figsize=(8, 4.8))
    sns.lineplot(data=scenario, x="y_size", y="mean_ms", marker="o", ax=ax)
    ax.set_xscale("log", base=2)
    ax.set_xlabel("Client set size |Y| (log2 scale)")
    ax.set_ylabel("Runtime (ms)")
    ax.set_title("Unbalanced PSI runtime: fixed |X|=1, |I|=1")
    ax.set_xticks(sorted(scenario["y_size"].unique()))
    ax.get_xaxis().set_major_formatter(plt.ScalarFormatter())
    fig.tight_layout()
    fig.savefig(PLOTS_DIR / "runtime_vs_y_unbalanced_x1_i1.png", dpi=220)
    plt.close(fig)


def save_plot_runtime_vs_x_fixed_y(df: pd.DataFrame, y_fixed: int = 2048) -> None:
    scenario = df[(df["y_size"] == y_fixed) & (df["i_size"] == 1)].copy()
    fig, ax = plt.subplots(figsize=(8, 4.8))
    sns.lineplot(data=scenario, x="x_size", y="mean_ms", marker="o", ax=ax)
    ax.set_xscale("log", base=2)
    ax.set_xlabel("Server set size |X| (log2 scale)")
    ax.set_ylabel("Runtime (ms)")
    ax.set_title(f"Runtime growth when |Y| is fixed to {y_fixed} (|I|=1)")
    ax.set_xticks(sorted(scenario["x_size"].unique()))
    ax.get_xaxis().set_major_formatter(plt.ScalarFormatter())
    fig.tight_layout()
    fig.savefig(PLOTS_DIR / f"runtime_vs_x_fixed_y{y_fixed}_i1.png", dpi=220)
    plt.close(fig)


def save_plot_intersection_impact(df: pd.DataFrame) -> None:
    balanced = df[df["x_size"] == df["y_size"]].copy()
    selected_x = [16, 64, 256, 1024, 2048]
    subset = balanced[balanced["x_size"].isin(selected_x)]

    fig, ax = plt.subplots(figsize=(8.6, 5.2))
    sns.lineplot(
        data=subset,
        x="i_size",
        y="mean_ms",
        hue="x_size",
        marker="o",
        palette="viridis",
        ax=ax,
    )
    ax.set_xscale("log", base=2)
    ax.set_xlabel("Intersection size |I| (log2 scale)")
    ax.set_ylabel("Runtime (ms)")
    ax.set_title("Impact of intersection size for balanced PSI (|X|=|Y|)")
    ax.set_xticks(sorted(subset["i_size"].unique()))
    ax.get_xaxis().set_major_formatter(plt.ScalarFormatter())
    ax.legend(title="|X|=|Y|", loc="best")
    fig.tight_layout()
    fig.savefig(PLOTS_DIR / "runtime_vs_intersection_balanced.png", dpi=220)
    plt.close(fig)


def save_plot_heatmap(df: pd.DataFrame) -> None:
    aggregated = (
        df.groupby(["x_size", "y_size"], as_index=False)["mean_ms"].mean().rename(columns={"mean_ms": "mean_over_i_ms"})
    )
    pivot = aggregated.pivot(index="x_size", columns="y_size", values="mean_over_i_ms")

    fig, ax = plt.subplots(figsize=(10.0, 7.2))
    sns.heatmap(
        pivot,
        cmap="mako",
        linewidths=0.4,
        cbar_kws={"label": "Average runtime over available |I| values (ms)"},
        ax=ax,
    )
    ax.set_xlabel("|Y|")
    ax.set_ylabel("|X|")
    ax.set_title("Runtime landscape across tested set-size combinations")
    fig.tight_layout()
    fig.savefig(PLOTS_DIR / "runtime_heatmap_x_y_avg_over_i.png", dpi=220)
    plt.close(fig)


def write_summary(df: pd.DataFrame) -> None:
    unbalanced = df[(df["x_size"] == 1) & (df["i_size"] == 1)].sort_values("y_size")
    fixed_y = df[(df["y_size"] == 2048) & (df["i_size"] == 1)].sort_values("x_size")
    balanced = df[(df["x_size"] == df["y_size"]) & (df["x_size"].isin([16, 64, 256, 1024, 2048]))]

    lines = [
        "# PSI Benchmark Analysis Summary",
        "",
        "The report requirement asks for runtime behavior when set sizes vary, especially when increasing |X| while keeping |Y| fixed.",
        "",
        "## Data overview",
        f"- Number of measurements: {len(df)}",
        f"- Tested |X| values: {sorted(df['x_size'].unique().tolist())}",
        f"- Tested |Y| values: {sorted(df['y_size'].unique().tolist())}",
        f"- Tested |I| values: {sorted(df['i_size'].unique().tolist())}",
        "",
        "## Key observations from the generated plots",
        (
            f"- Unbalanced case (|X|=1, |I|=1): runtime rises from {unbalanced['mean_ms'].iloc[0]} ms at |Y|={unbalanced['y_size'].iloc[0]} "
            f"to {unbalanced['mean_ms'].iloc[-1]} ms at |Y|={unbalanced['y_size'].iloc[-1]}."
        ),
        (
            f"- Fixed |Y|=2048 with |I|=1: runtime rises from {fixed_y['mean_ms'].iloc[0]} ms at |X|={fixed_y['x_size'].iloc[0]} "
            f"to {fixed_y['mean_ms'].iloc[-1]} ms at |X|={fixed_y['x_size'].iloc[-1]}."
        ),
        "- For balanced runs (|X|=|Y|), changing |I| has only a small effect compared to changing |X| and |Y|.",
        "- The heatmap shows that runtime is dominated by set cardinalities, with larger |X| and |Y| combinations giving the highest costs.",
        "",
        "## Generated artifacts",
        "- plots/runtime_vs_y_unbalanced_x1_i1.png",
        "- plots/runtime_vs_x_fixed_y2048_i1.png",
        "- plots/runtime_vs_intersection_balanced.png",
        "- plots/runtime_heatmap_x_y_avg_over_i.png",
    ]

    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    PLOTS_DIR.mkdir(parents=True, exist_ok=True)
    df = load_data()
    save_plot_runtime_vs_y_unbalanced(df)
    save_plot_runtime_vs_x_fixed_y(df, y_fixed=2048)
    save_plot_intersection_impact(df)
    save_plot_heatmap(df)
    write_summary(df)
    print(f"Wrote plots to: {PLOTS_DIR}")
    print(f"Wrote summary to: {REPORT_PATH}")


if __name__ == "__main__":
    main()
