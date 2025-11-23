"""Split PGM logs into alpha / beta groups.

The input file has alpha and beta signature groups separated by 7 spaces.
This script writes two files next to the source logs:
- PGM_logs-2025.alpha.txt
- PGM_logs-2025.beta.txt

Each output contains one group per line (leading/trailing whitespace removed).
"""
from pathlib import Path


def main():
    src = Path(__file__).resolve().parents[1] / "cipher_pgm" / "PGM_logs-2025.txt"
    if not src.exists():
        raise SystemExit(f"Logs file not found: {src}")

    lines = src.read_text(encoding="utf-8").splitlines()

    alpha_out = []
    beta_out = []

    SEP = "       "  # seven spaces
    for lineno, line in enumerate(lines, start=1):
        # skip empty lines
        if not line.strip():
            continue
        parts = line.split(SEP)
        # If there are more than two parts, treat first as alpha, last as beta,
        # and any in between are additional alpha-like parts appended to alpha.
        if len(parts) == 1:
            # no separator found: put the whole line into alpha by default
            alpha_out.append(parts[0].strip())
        elif len(parts) == 2:
            alpha_out.append(parts[0].strip())
            beta_out.append(parts[1].strip())
        else:
            alpha_out.append(parts[0].strip())
            # middle parts -> join into alpha as well (conservative)
            for mid in parts[1:-1]:
                if mid.strip():
                    alpha_out.append(mid.strip())
            beta_out.append(parts[-1].strip())

    out_dir = src.parent
    a_path = out_dir / "PGM_logs-2025.alpha.txt"
    b_path = out_dir / "PGM_logs-2025.beta.txt"

    a_path.write_text("\n".join(alpha_out) + ("\n" if alpha_out else ""), encoding="utf-8")
    b_path.write_text("\n".join(beta_out) + ("\n" if beta_out else ""), encoding="utf-8")

    # summary
    print(f"Read {len(lines)} lines from {src}")
    print(f"Wrote {len(alpha_out)} alpha entries to {a_path}")
    print(f"Wrote {len(beta_out)} beta entries to {b_path}")
    print("alpha preview:")
    for l in alpha_out[:10]:
        print("  ", l)
    print("beta preview:")
    for l in beta_out[:10]:
        print("  ", l)


if __name__ == "__main__":
    main()
