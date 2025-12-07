#!/usr/bin/env bash
set -euo pipefail

# Configurable ranges
START_YEAR=2015
END_YEAR=2024

# Validate running at repo root
if [ ! -d ".git" ]; then
    echo "Run this from the repository root."
    exit 1
fi

# Remove build artifacts
find . -name Cargo.lock -delete
find . -type d -name target -exec rm -rf {} +

# Check if util exists
UTIL_EXISTS=false
if [ -d "util" ] && [ -f "util/Cargo.toml" ]; then
    UTIL_EXISTS=true
fi

ROOT=$(pwd)

for Y in $(seq "$START_YEAR" "$END_YEAR"); do
    # Create year directory if it doesn't exist
    if [ ! -d "$Y" ]; then
        mkdir "$Y"
    fi

    cd "$Y"

    # Rename existing days to zero-padded using plain mv
    for D in day-*; do
        [ -d "$D" ] || continue
        OLDNUM=$(echo "$D" | sed 's/day-//')
        NEWNUM=$(printf "%02d" "$((10#$OLDNUM))")
        NEWDIR="day-$NEWNUM"
        # Only rename if oldnum != newnum
        if [ "$NEWNUM" != "$OLDNUM" ]; then
            # Check if NEWDIR already exists to prevent conflicts
            if [ -d "$NEWDIR" ]; then
                echo "Error: Destination directory $NEWDIR already exists. Please resolve manually."
                exit 1
            fi
            mv "$D" "$NEWDIR"
        fi
    done

    # Create all 25 days if they don't exist
    for i in $(seq 1 25); do
        DAYNUM=$(printf "%02d" $i)
        DAYDIR="day-$DAYNUM"
        PKG_NAME="aoc-$Y-day-$DAYNUM"

        if [ ! -d "$DAYDIR" ]; then
            cargo new --bin --name "$PKG_NAME" "$DAYDIR"
        else
            # Ensure Cargo.toml name is correct
            if [ -f "$DAYDIR/Cargo.toml" ]; then
                if grep -q '^name = ' "$DAYDIR/Cargo.toml"; then
                    sed -i.bak "s/^name = .*/name = \"$PKG_NAME\"/" "$DAYDIR/Cargo.toml"
                    rm "$DAYDIR/Cargo.toml.bak"
                else
                    if grep -q '^\[package\]' "$DAYDIR/Cargo.toml"; then
                        sed -i.bak "/^\[package\]/a name = \"$PKG_NAME\"" "$DAYDIR/Cargo.toml"
                        rm "$DAYDIR/Cargo.toml.bak"
                    else
                        {
                            echo "[package]"
                            echo "name = \"$PKG_NAME\""
                            echo "version = \"0.1.0\""
                            echo "edition = \"2021\""
                            cat "$DAYDIR/Cargo.toml"
                        } > "$DAYDIR/Cargo.toml.tmp"
                        mv "$DAYDIR/Cargo.toml.tmp" "$DAYDIR/Cargo.toml"
                    fi
                fi
            else
                # Create Cargo.toml if missing
                cat > "$DAYDIR/Cargo.toml" <<EOF
[package]
name = "$PKG_NAME"
version = "0.1.0"
edition = "2021"
EOF
            fi
        fi

        # Add util dependency if util exists
        if $UTIL_EXISTS && ! grep -q 'util = { path = "../../../util" }' "$DAYDIR/Cargo.toml"; then
            if ! grep -q '^\[dependencies\]' "$DAYDIR/Cargo.toml"; then
                echo "" >> "$DAYDIR/Cargo.toml"
                echo "[dependencies]" >> "$DAYDIR/Cargo.toml"
            fi
            echo 'util = { path = "../../../util" }' >> "$DAYDIR/Cargo.toml"
        fi

        # Add use util::* and ensure fn main() in src/main.rs
        MAIN_RS="$DAYDIR/src/main.rs"
        if [ -f "$MAIN_RS" ]; then
            if $UTIL_EXISTS && ! grep -q '^use util::\*;' "$MAIN_RS"; then
                sed -i.bak '1s/^/use util::*;\n/' "$MAIN_RS"
                rm "$MAIN_RS.bak"
            fi
            if ! grep -q 'fn main()' "$MAIN_RS"; then
                echo "fn main() {}" >> "$MAIN_RS"
            fi
        else
            mkdir -p "$DAYDIR/src"
            if $UTIL_EXISTS; then
                echo -e "use util::*;\nfn main() {}" > "$MAIN_RS"
            else
                echo "fn main() {}" > "$MAIN_RS"
            fi
        fi
    done

    # Rebuild this year's Cargo.toml as a workspace
    DAYS=$(find . -maxdepth 1 -type d -name 'day-*' | sed 's|^\./||' | sort)
    {
        echo "[workspace]"
        echo "members = ["
        for D in $DAYS; do
            echo "    \"$D\","
        done
        echo "]"
    } > Cargo.toml

    cd "$ROOT"
done

# Rebuild top-level Cargo.toml
{
    echo "[workspace]"
    echo "members = ["
    if $UTIL_EXISTS; then
        echo "    \"util\","
    fi
    for Y in $(seq $START_YEAR $END_YEAR); do
        echo "    \"$Y\","
    done
    echo "]"
} > Cargo.toml

echo "All years and days have been created/updated successfully."
echo "Please run 'git add -A' and commit your changes."
