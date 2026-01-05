# Cargo Test Commands Explained

## What is `cargo test`?

`cargo test` is Rust's built-in testing command that compiles and runs all tests in your project. It's part of Cargo's testing framework and is essential for ensuring code quality and correctness.

## Basic Usage

```bash
cargo test
```

This command:
1. **Compiles** your code in test mode
2. **Runs** all tests (unit tests, integration tests, doc tests)
3. **Reports** results (passed, failed, ignored)
4. **Captures** output by default (hides `println!` and debug statements)

## Command Breakdown: `cargo test -- --nocapture`

### The `--` (Double Dash)

The `--` is a **separator** that tells Cargo:
- Everything before `--` are **Cargo options** (for Cargo itself)
- Everything after `--` are **test runner options** (passed to the test binary)

**Why we need it:**
- `cargo test` is a Cargo command
- `--nocapture` is a test runner option
- Without `--`, Cargo would try to interpret `--nocapture` as a Cargo option
- With `--`, we're saying "pass `--nocapture` to the test runner, not to Cargo"

### The `--nocapture` Flag

By default, `cargo test` **captures** (hides) output from:
- `println!` statements
- `dbg!` macros
- `eprintln!` statements
- Debug output

**Why we need `--nocapture`:**
- **See debug output**: View all `println!` and debug statements
- **Debug test failures**: Understand what's happening during test execution
- **Monitor progress**: See real-time output from long-running tests
- **Development**: Essential for understanding how your code works

## Examples

### Basic Test (Output Hidden)
```bash
cargo test
```
**Output:**
```
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

### Test with Full Output
```bash
cargo test -- --nocapture
```
**Output:**
```
🚀 Testing LAS parser - Parse single file
Reading LAS file: tests/test_data/logs/1.2/sample_wrapped.las
✅ Successfully parsed LAS file
Processing well: ANY ET AL OIL WELL #12
Found 35 curves
DEPT: first value: 1783.5 FT (1 DEPTH)
CALI: first value: 101.78 MM (CALI)
...
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

## Common Test Commands

### Run All Tests
```bash
cargo test
```

### Run Tests with Full Output
```bash
cargo test -- --nocapture
```

### Run Specific Test
```bash
cargo test test_name -- --nocapture
```

### Run Tests in Specific Package
```bash
cargo test --package package_name -- --nocapture
```

### Run Tests in Specific Directory
```bash
cd path/to/crate && cargo test -- --nocapture
```

### Run Tests with Filter
```bash
cargo test -- --nocapture --test test_file_name
```

## Why This Matters for LAS Processing

In our LAS processing tests, we use `--nocapture` because:

1. **Debug Information**: We need to see curve detection results
2. **Progress Monitoring**: Watch as files are parsed
3. **Error Diagnosis**: Understand why tests might fail
4. **Development**: See what's happening during curve mapping
5. **Statistics**: View curve statistics and data validation

## Test Output Types

### Without `--nocapture` (Default)
- Only shows test results summary
- Hides all `println!` output
- Good for CI/CD pipelines
- Clean, minimal output

### With `--nocapture`
- Shows all debug output
- Displays `println!` statements
- Essential for development
- Helps with debugging

## Best Practices

### For Development
```bash
cargo test -- --nocapture
```
- Use when developing and debugging
- See all output and debug information
- Understand what your code is doing

### For CI/CD
```bash
cargo test
```
- Use in automated pipelines
- Clean output for logs
- Faster execution

### For Specific Testing
```bash
cargo test test_specific_function -- --nocapture
```
- Test individual functions
- Debug specific issues
- Focus on particular functionality

## LAS Parser Example

Our LAS parser tests benefit greatly from `--nocapture` because they:

1. **Parse LAS files** and show curve information
2. **Detect LAS versions** (1.2, 2.0, 3.0)
3. **Extract curve data** with units and descriptions
4. **Calculate statistics** (min, max, mean, NaN counts)
5. **Validate data integrity** across different file formats

Without `--nocapture`, we'd only see:
```
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

With `--nocapture`, we see the full story:
```
🚀 Testing LAS parser - Parse single file
Reading LAS file: tests/test_data/logs/1.2/sample_wrapped.las
✅ Successfully parsed LAS file
Processing well: ANY ET AL OIL WELL #12
Found 35 curves
DEPT: first value: 1783.5 FT (1 DEPTH)
CALI: first value: 101.78 MM (CALI)
DFAR: first value: 0.883 G/CM3 (DFAR)
...
Detected LAS version: Some(2.0)
Is LAS 2.0: true
Found depth data with 5 points
Depth curve data (first 10 values): [1783.5, 1783.75, 1784.0, 1784.25, 1784.5]
test test_real_las_file_sample_wrapped_1_2 ... ok
```

## Summary

- **`cargo test`**: Runs all tests with output captured (hidden)
- **`--`**: Separates Cargo options from test runner options
- **`--nocapture`**: Shows all debug output and `println!` statements
- **Essential for development**: Understanding what your code does
- **Not needed for CI/CD**: Clean output for automated pipelines

The combination `cargo test -- --nocapture` is perfect for development and debugging, especially when working with complex data processing like LAS file parsing.
