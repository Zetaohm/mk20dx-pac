# mk20dx-pac: Strategy

## Goal

Produce correct, published Rust Peripheral Access Crates (PACs) for the MK20DX128 (Teensy 3.0) and MK20DX256 (Teensy 3.1/3.2) by patching the vendor SVD files using the battle-tested Teensyduino `kinetis.h` as ground truth.

---

## Phase 1: Setup and SVD Baseline

### 1.1 Obtain the SVD Files

Acquire both SVD files:

| SVD File | Chip | Teensy Board | Sources |
|----------|------|-------------|---------|
| `MK20D5.svd` | MK20DX128 | Teensy 3.0 | cmsis-svd-data, NXP MCUXpresso SDK, Keil CMSIS Pack |
| `MK20D7.svd` | MK20DX256 | Teensy 3.1/3.2 | cmsis-svd-data, exrook/mk20d7, NXP MCUXpresso SDK |

Compare all available versions of each to identify which is most recent. Store as `svd/MK20D5.svd` and `svd/MK20D7.svd`.

### 1.2 Obtain kinetis.h

Download `kinetis.h` from `PaulStoffregen/cores` (`teensy3/kinetis.h`) as our reference. Also grab `core_pins.h`, `mk20dx128.c`, and the struct-based peripheral headers for supplementary cross-referencing. Store in `reference/`.

### 1.3 Attempt Vanilla svd2rust

Run `svd2rust` on both unpatched SVDs to establish a baseline:
- Does each compile at all?
- How large are the generated crates?
- What errors/warnings does svd2rust produce?
- What compilation errors does the generated code have?
- How much do the two SVDs differ from each other? (Shared patches vs variant-specific)

This gives us a concrete inventory of what needs fixing and how much patch reuse is possible between MK20D5 and MK20D7.

### 1.4 Install Tooling

```bash
cargo install svdtools svd2rust form
pip3 install cmsis-svd
```

---

## Phase 2: Build the Comparison Script

### 2.1 Parse kinetis.h

Write a Python script (`scripts/compare_header_svd.py`) that extracts from `kinetis.h`:

**Register addresses** — regex for:
```c
#define REG_NAME  (*(volatile uint32_t *)0xADDRESS)
#define REG_NAME  (*(volatile uint16_t *)0xADDRESS)
#define REG_NAME  (*(volatile uint8_t *)0xADDRESS)
```

Output: `{name: str, address: int, size: 8|16|32}`

**Bitmask constants** — regex for:
```c
#define REG_FIELD  ((uint32_t)0xMASK)
```

Output: `{name: str, mask: int}` → derive bit offset and width from mask

**Parameterized fields** — regex for:
```c
#define REG_FIELD(n)  (((n) & MASK) << SHIFT)
```

Output: `{name: str, mask: int, shift: int}` → derive bit offset and width

**Struct typedefs** — parse for MCG, PIT, SPI, I2C, UART, LPUART:
```c
typedef struct __attribute__((packed)) {
    volatile uint8_t REG1;
    volatile uint8_t REG2;
    // ...
} PERIPH_t;
#define PERIPH_INSTANCE  (*(PERIPH_t *)0xBASE)
```

Output: `{peripheral: str, base: int, registers: [{name, offset, size}]}`

**IRQ enumerations** — parse the `enum` block inside `#ifdef __MK20DX256__`:
```c
enum { IRQ_DMA_CH0 = 0, IRQ_DMA_CH1 = 1, ... };
```

Output: `{name: str, number: int}`

### 2.2 Parse MK20D7.svd

Using the `cmsis-svd` Python library, extract:
- Peripherals: name, base address, interrupt associations
- Registers: name, address offset, size, access, reset value
- Fields: name, bit offset, bit width, enumerated values
- Interrupts: name, number

### 2.3 Cross-Reference and Report

For each peripheral in kinetis.h:
1. Find matching peripheral in SVD (by base address)
2. Compare every register address
3. Compare every bitfield (width, offset, mask)
4. Flag discrepancies categorized as:
   - `ADDRESS_MISMATCH` — register at wrong address
   - `SIZE_MISMATCH` — register size wrong (e.g., 32-bit in SVD but 8-bit in header)
   - `FIELD_WIDTH_MISMATCH` — bitfield width wrong
   - `FIELD_OFFSET_MISMATCH` — bitfield position wrong
   - `MISSING_IN_SVD` — register/field in header but not in SVD
   - `MISSING_IN_HEADER` — register/field in SVD but not in header (review needed)
   - `IRQ_MISMATCH` — interrupt number doesn't match

Output: JSON report + human-readable summary.

### 2.4 Filter for MK20DX128 and MK20DX256

kinetis.h covers 5 chip variants. The comparison script must support two filter modes:

**MK20DX128 (Teensy 3.0):**
- Use the IRQ enum and DMAMUX sources from the `__MK20DX128__` block (lines 37-142)
- Use all register definitions inside `#if defined(KINETISK)` blocks
- 4 DMA channels, 46 interrupts

**MK20DX256 (Teensy 3.1/3.2):**
- Use the IRQ enum and DMAMUX sources from the `__MK20DX256__` block (lines 145-283)
- Use all register definitions inside `#if defined(KINETISK)` blocks
- 16 DMA channels, 95 interrupts

Both share the same peripheral register definitions (everything after line 868). The chip-specific sections differ in IRQ numbers, DMA channel counts, and DMAMUX source mappings.

Exclude registers gated behind `__MK64FX512__`, `__MK66FX1M0__`, or `__MKL26Z64__`.

This is the trickiest part of the parser — it needs to understand enough preprocessor logic to select the right definitions.

---

## Phase 3: Generate SVD Patches

### 3.1 Project Structure

```
mk20dx-pac/
├── svd/
│   ├── MK20D5.svd                 # Vendor SVD for MK20DX128 (unmodified)
│   └── MK20D7.svd                 # Vendor SVD for MK20DX256 (unmodified)
├── devices/
│   ├── mk20d5.yaml                # Top-level svdtools patch (Teensy 3.0)
│   └── mk20d7.yaml                # Top-level svdtools patch (Teensy 3.1/3.2)
├── patches/
│   ├── common/                    # Shared patches (both variants)
│   │   ├── irq_offset.yaml        # Fix IRQ numbering
│   │   ├── uart/
│   │   │   └── register_size.yaml # Fix register size to 8-bit
│   │   ├── ftm/
│   │   │   └── conf_fields.yaml   # Fix FTM_CONF field definitions
│   │   └── ...
│   ├── mk20d5/                    # MK20DX128-specific patches
│   │   └── ...
│   └── mk20d7/                    # MK20DX256-specific patches
│       ├── dma/
│       │   ├── nbytes_width.yaml  # Fix MLOFFYES bitwidth
│       │   └── dmamux_sources.yaml
│       └── ...
├── reference/
│   ├── kinetis.h                  # Teensyduino reference
│   ├── core_pins.h
│   └── mk20dx128.c
├── scripts/
│   ├── compare_header_svd.py      # Automated comparison
│   └── generate_patches.py        # (Optional) auto-generate YAML from discrepancies
├── mk20d5/                        # Generated PAC crate for MK20DX128
├── mk20d7/                        # Generated PAC crate for MK20DX256
├── Makefile
└── README.md
```

### 3.2 Patch Priorities

Based on known bugs and likely SVD issues, patches should be developed in this order:

**Priority 1: Blocking (prevents compilation or causes incorrect code)**
1. IRQ offset fix (all IRQs shifted by 16)
2. Duplicate enumerated value names (if present)
3. Overloaded registers at same offset (svd2rust#16 workaround)
4. Any registers with wrong size (8-bit vs 32-bit, especially UART, MCG, I2C)

**Priority 2: Correctness (compiles but produces wrong behavior)**
5. Register address mismatches
6. Bitfield width errors (DMA NBYTES, etc.)
7. Bitfield offset errors
8. Missing registers/fields
9. Wrong reset values
10. Wrong access attributes (read-only vs read-write)

**Priority 3: Ergonomics (works but API is awkward)**
11. Register grouping into arrays (FTM channels, DMA TCDs, etc.)
12. Register grouping into clusters
13. Adding enumerated values for common fields
14. Stripping redundant peripheral name prefixes

### 3.3 Iterative Patch Development

For each peripheral:
1. Run comparison script → get discrepancy report
2. Cross-reference discrepancies against K20 Reference Manual (PDF) to confirm which source is correct
3. Write svdtools YAML patch
4. Run `svdtools patch` → verify patched SVD
5. Run `svd2rust` → verify generated code compiles
6. Repeat

### 3.4 Known Patches to Write Immediately

From the kinetis.h correction history and Kinetis-wide SVD bugs:

```yaml
# patches/irq_offset.yaml — Fix all IRQ numbers (subtract 16)
# This is a known Kinetis-wide SVD bug documented in cmsis-svd#33.
# Every device-specific IRQ value is offset by +16.
# Need to verify this against the MK20D7.svd specifically.

# patches/dma/nbytes_width.yaml
DMA:
  TCD0_NBYTES_MLOFFYES:
    _modify:
      NBYTES:
        bitWidth: 10    # Was likely 5 in SVD

# patches/uart/register_size.yaml
# UART registers are 8-bit, not 32-bit.
# The kinetis.h struct typedef confirms:
#   volatile uint8_t BDH, BDL, C1, C2, S1, S2, C3, D, MA1, MA2, C4, C5
# SVD must use <size>8</size> for each register.
```

---

## Phase 4: Generate and Validate the PAC

### 4.1 Build Pipeline

```makefile
DEVICES = mk20d5 mk20d7

.PHONY: patch svd2rust form check all clean

patch: $(DEVICES:%=patch-%)
patch-%:
	svdtools patch devices/$*.yaml

svd2rust: $(DEVICES:%=svd2rust-%)
svd2rust-%: patch-%
	cd $* && svd2rust -i ../svd/$*.svd.patched
	cd $* && form -i lib.rs -o src/
	rm $*/lib.rs

form: $(DEVICES:%=form-%)
form-%:
	cd $* && cargo fmt

check: $(DEVICES:%=check-%)
check-%:
	cd $* && cargo check

all: patch svd2rust form check
```

### 4.2 Validation Steps

For each variant (mk20d5, mk20d7):

1. **Compilation:** `cargo check` passes with no errors
2. **Size sanity:** Generated crates should be manageable (K20 is much smaller than i.MX RT)
3. **Peripheral spot-checks:** For each peripheral, verify:
   - Base address matches kinetis.h
   - Register offsets match kinetis.h
   - Key bitfield widths match kinetis.h
   - Register sizes match (8-bit for UART/MCG/I2C, 32-bit for most others)
4. **IRQ table verification:** Compare generated interrupt enum against kinetis.h IRQ enum for each variant
5. **DMA channel count:** mk20d5 should have 4 DMA channels, mk20d7 should have 16
6. **Cross-variant consistency:** Shared peripherals should have identical register definitions

### 4.3 Hardware Validation (If Teensy 3.x Available)

Write minimal test programs that exercise each peripheral:
1. GPIO blink (PORT + GPIO registers)
2. SysTick timer
3. UART loopback
4. SPI loopback
5. I2C scan
6. ADC read
7. FTM PWM output
8. PIT timer interrupt
9. DMA memory-to-memory transfer

These don't need to be exhaustive — just enough to prove the register addresses and field definitions are correct for the most-used peripherals. Test on whichever Teensy 3.x boards are available.

---

## Phase 5: HAL Layer (Future)

Once the PAC is validated, a HAL crate can be built on top using `embedded-hal` traits. This is out of scope for the initial effort but the PAC should be designed to support it.

The HAL would provide:
- `embedded-hal` trait implementations (GPIO, SPI, I2C, UART, ADC, PWM, timers)
- Clock configuration API
- DMA transfer API
- USB device API

---

## Phase 6: Publishing

### 6.1 Crate Metadata

Two crates will be published from this repository:

**mk20d5** (Teensy 3.0):
```toml
[package]
name = "mk20d5"
version = "0.1.0"
edition = "2021"
description = "Peripheral Access Crate for MK20DX128 (Teensy 3.0)"
license = "MIT OR Apache-2.0"
repository = "https://github.com/USER/mk20dx-pac"
categories = ["embedded", "hardware-support", "no-std"]
keywords = ["arm", "cortex-m", "teensy", "kinetis", "nxp"]

[dependencies]
cortex-m = "0.7"
vcell = "0.1"
```

**mk20d7** (Teensy 3.1/3.2):
```toml
[package]
name = "mk20d7"
version = "0.1.0"
edition = "2021"
description = "Peripheral Access Crate for MK20DX256 (Teensy 3.1/3.2)"
license = "MIT OR Apache-2.0"
repository = "https://github.com/USER/mk20dx-pac"
categories = ["embedded", "hardware-support", "no-std"]
keywords = ["arm", "cortex-m", "teensy", "kinetis", "nxp"]

[dependencies]
cortex-m = "0.7"
vcell = "0.1"
```

### 6.2 Documentation

- README with: what this repo is, which chips/boards it targets, how to use each crate, known limitations
- Link to K20 Reference Manual
- Credit to PaulStoffregen/cores for reference register definitions
- Note which SVD patches were applied and why
- Document differences between the two variants

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| SVD has more bugs than kinetis.h corrections reveal | High | Medium | Cross-reference against K20 reference manual PDF for every peripheral |
| kinetis.h is wrong for some obscure peripheral | Low | Low | kinetis.h has been battle-tested since 2012 across hundreds of thousands of boards |
| svd2rust generates code that compiles but has runtime bugs | Medium | High | Hardware validation with real Teensy 3.x boards |
| Preprocessor parsing of kinetis.h misses conditional definitions | Medium | Medium | Manual review of comparison script output; start with simple peripherals |
| Overloaded registers (svd2rust#16) can't be represented | High | Medium | Document which registers are affected; may need unsafe raw pointer access for those |
| The project scope creeps into a full HAL | Medium | Low | Strictly scope Phase 1-4 as PAC only; HAL is a separate future project |

---

## Estimated Effort Breakdown

| Phase | Work |
|-------|------|
| Phase 1: Setup | Obtain files, attempt vanilla build, install tools |
| Phase 2: Comparison Script | Python script to parse kinetis.h and SVD, cross-reference |
| Phase 3: SVD Patches | Write YAML patches for each discrepancy found |
| Phase 4: Validate | Compile, spot-check, optionally test on hardware |
| Phase 5: HAL | Future scope — not part of initial effort |
| Phase 6: Publish | Crate metadata, docs, publish to crates.io |

The comparison script (Phase 2) is the highest-leverage piece — once it works, it will mechanically identify most issues. Manual review against the reference manual will catch the rest.

---

## Open Questions

1. **Which SVD version to start from?** The cmsis-svd-data copy may be very old. Should we extract a newer one from the MCUXpresso SDK or Keil pack?

2. **Patch sharing between variants:** How much of the MK20D5 and MK20D7 SVDs are identical? If the peripheral register sections are largely the same, most patches can live in `patches/common/` with minimal variant-specific overrides. Need to diff the two SVDs to quantify.

3. **License for SVD patches:** stm32-rs uses MIT OR Apache-2.0. NXP's SVD files have their own license terms. Need to check if redistribution of a patched SVD is permitted.

4. **Minimum Supported Rust Version (MSRV):** svd2rust output targets specific Rust editions. Pin to a reasonable MSRV (e.g., 1.76+).

## Resolved Questions

- **Target variants:** MK20DX128 (Teensy 3.0) and MK20DX256 (Teensy 3.1/3.2) only. No K64/K66 support.
- **Crate naming:** `mk20d5` and `mk20d7` (matches SVD sub-family names, following community convention). Repository name: `mk20dx-pac`.
- **Repository scope:** PAC only. HAL is a separate future project.
