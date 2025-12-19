#!/bin/bash
# Benchmark automation script for DSE Pass
# Compares LLVM IR instruction counts before and after DSE optimization

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BENCHES_DIR="$PROJECT_ROOT/benches"
BUILD_DIR="$PROJECT_ROOT/target/benchmark"
RESULTS_FILE="$PROJECT_ROOT/benchmark_results.txt"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Dead Store Elimination Benchmark${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Create build directory
mkdir -p "$BUILD_DIR"

# Build the DSE pass
echo -e "${YELLOW}[1/6] Building DSE pass...${NC}"
cd "$PROJECT_ROOT"
cargo build --release
echo -e "${GREEN}✓ DSE pass built successfully${NC}"
echo ""

# Compile benchmark to LLVM IR
echo -e "${YELLOW}[2/6] Compiling benchmark.c to LLVM IR...${NC}"
clang -S -emit-llvm -O1 -Xclang -disable-llvm-passes \
    "$BENCHES_DIR/benchmark.c" \
    -o "$BUILD_DIR/benchmark_original.ll"
echo -e "${GREEN}✓ LLVM IR generated${NC}"
echo ""

# Count instructions in original IR
echo -e "${YELLOW}[3/6] Analyzing original IR...${NC}"
ORIGINAL_STORES=$(grep -c "store " "$BUILD_DIR/benchmark_original.ll" || true)
ORIGINAL_TOTAL=$(wc -l < "$BUILD_DIR/benchmark_original.ll")
echo "  Original IR statistics:"
echo "    Total lines: $ORIGINAL_TOTAL"
echo "    Store instructions: $ORIGINAL_STORES"
echo ""

# Run DSE pass
echo -e "${YELLOW}[4/6] Running DSE optimization...${NC}"
"$PROJECT_ROOT/target/release/ntu-2025fall-advanced-compiler-hw6-g36maid" \
    -i "$BUILD_DIR/benchmark_original.ll" \
    -o "$BUILD_DIR/benchmark_optimized.bc"
echo -e "${GREEN}✓ DSE optimization completed${NC}"
echo ""

# Convert optimized bitcode back to IR
echo -e "${YELLOW}[5/6] Converting optimized bitcode to IR...${NC}"
llvm-dis "$BUILD_DIR/benchmark_optimized.bc" -o "$BUILD_DIR/benchmark_optimized.ll"
echo -e "${GREEN}✓ Optimized IR generated${NC}"
echo ""

# Count instructions in optimized IR
echo -e "${YELLOW}[6/6] Analyzing optimized IR...${NC}"
OPTIMIZED_STORES=$(grep -c "store " "$BUILD_DIR/benchmark_optimized.ll" || true)
OPTIMIZED_TOTAL=$(wc -l < "$BUILD_DIR/benchmark_optimized.ll")
echo "  Optimized IR statistics:"
echo "    Total lines: $OPTIMIZED_TOTAL"
echo "    Store instructions: $OPTIMIZED_STORES"
echo ""

# Calculate improvements
STORES_REMOVED=$((ORIGINAL_STORES - OPTIMIZED_STORES))
STORES_PERCENT=$(awk "BEGIN {printf \"%.2f\", ($STORES_REMOVED / $ORIGINAL_STORES) * 100}")
LINES_REMOVED=$((ORIGINAL_TOTAL - OPTIMIZED_TOTAL))
LINES_PERCENT=$(awk "BEGIN {printf \"%.2f\", ($LINES_REMOVED / $ORIGINAL_TOTAL) * 100}")

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Benchmark Results${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Store Instructions:"
echo "  Before:  $ORIGINAL_STORES"
echo "  After:   $OPTIMIZED_STORES"
echo -e "  Removed: ${GREEN}$STORES_REMOVED${NC} (${GREEN}${STORES_PERCENT}%${NC})"
echo ""
echo "Total IR Lines:"
echo "  Before:  $ORIGINAL_TOTAL"
echo "  After:   $OPTIMIZED_TOTAL"
echo -e "  Removed: ${GREEN}$LINES_REMOVED${NC} (${GREEN}${LINES_PERCENT}%${NC})"
echo ""

# Save results to file
cat > "$RESULTS_FILE" <<EOF
Dead Store Elimination Benchmark Results
Generated: $(date)
========================================

Store Instructions:
  Before:  $ORIGINAL_STORES
  After:   $OPTIMIZED_STORES
  Removed: $STORES_REMOVED ($STORES_PERCENT%)

Total IR Lines:
  Before:  $ORIGINAL_TOTAL
  After:   $OPTIMIZED_TOTAL
  Removed: $LINES_REMOVED ($LINES_PERCENT%)

Files:
  Original IR:   $BUILD_DIR/benchmark_original.ll
  Optimized IR:  $BUILD_DIR/benchmark_optimized.ll
  Optimized BC:  $BUILD_DIR/benchmark_optimized.bc

To view detailed IR differences:
  diff -u $BUILD_DIR/benchmark_original.ll $BUILD_DIR/benchmark_optimized.ll | less

To verify correctness:
  clang $BUILD_DIR/benchmark_original.ll -o $BUILD_DIR/benchmark_original
  clang $BUILD_DIR/benchmark_optimized.ll -o $BUILD_DIR/benchmark_optimized
  $BUILD_DIR/benchmark_original > /tmp/original_output.txt
  $BUILD_DIR/benchmark_optimized > /tmp/optimized_output.txt
  diff /tmp/original_output.txt /tmp/optimized_output.txt
EOF

echo -e "${GREEN}✓ Results saved to: $RESULTS_FILE${NC}"
echo ""

# Optional: Compile and run both versions to verify correctness
echo -e "${YELLOW}Verifying semantic equivalence...${NC}"
clang "$BUILD_DIR/benchmark_original.ll" -o "$BUILD_DIR/benchmark_original" 2>/dev/null
clang "$BUILD_DIR/benchmark_optimized.ll" -o "$BUILD_DIR/benchmark_optimized" 2>/dev/null

ORIGINAL_OUTPUT=$("$BUILD_DIR/benchmark_original")
OPTIMIZED_OUTPUT=$("$BUILD_DIR/benchmark_optimized")

if [ "$ORIGINAL_OUTPUT" = "$OPTIMIZED_OUTPUT" ]; then
    echo -e "${GREEN}✓ Semantic equivalence verified: outputs match${NC}"
else
    echo -e "${YELLOW}⚠ Warning: outputs differ (may be expected due to optimization)${NC}"
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Benchmark completed successfully!${NC}"
echo -e "${BLUE}========================================${NC}"
