# mk20dx-pac: Project Status

**Last updated:** 2026-02-19 (Phase 5 ergonomics in progress)

---

## Target Hardware

| Crate | Chip | Board | SVD Sub-Family | Ref Manual | Clock |
|-------|------|-------|----------------|------------|-------|
| `mk20d5` | MK20DX128VLH5 | Teensy 3.0 | MK20D5 | K20P64M50SF0RM (50MHz) | 50 MHz |
| `mk20d7` | MK20DX256VLH7 | Teensy 3.1/3.2 | MK20D7 | K20P64M72SF1RM (72MHz) | 72 MHz |

**Important:** These are different K20 sub-families with separate reference manuals. The 50MHz and 72MHz parts share most peripherals but differ in some hardware details (notably FMC cache geometry). Always check the correct sub-family reference manual when verifying register definitions.

---

## Phase 1: Setup and SVD Baseline — COMPLETE

### 1.1 Files Obtained
- `svd/MK20D5.svd` (1.5 MB) — from cmsis-svd-data (Freescale directory)
- `svd/MK20D7.svd` (3.1 MB) — from cmsis-svd-data (Freescale directory)
- `reference/kinetis.h` (383 KB) — from PaulStoffregen/cores (teensy3)
- `reference/core_pins.h` (86 KB) — from PaulStoffregen/cores
- `reference/mk20dx128.c` (54 KB) — from PaulStoffregen/cores
- `reference/K20P64M72SF1RM.pdf` (7.7 MB) — 72MHz K20 reference manual from PJRC
- `reference/K20P64M50SF0RM.pdf` (7.1 MB) — 50MHz K20 reference manual from PJRC

### 1.2 Tooling Installed
| Tool | Version | Source |
|------|---------|--------|
| svdtools | 0.3.19 | cargo install |
| svd2rust | 0.33.4 | cargo install |
| form | 0.12.1 | cargo install |
| cmsis-svd | 0.6 | uv (Python) |
| pymupdf | 1.27.1 | uv (Python) |
| uv | 0.8.11 | Homebrew |
| Rust targets | thumbv7em-none-eabi, thumbv7em-none-eabihf | rustup |

### 1.3 Vanilla svd2rust Build Results

Both crates compile with **zero errors** from unpatched SVDs.

| | mk20d5 (Teensy 3.0) | mk20d7 (Teensy 3.1/3.2) |
|---|---|---|
| svd2rust | OK (SPI0 type name conflict warning) | OK (SPI0 + SPI1 type name conflict warnings) |
| form output | 562 .rs files | 860 .rs files |
| cargo check | OK (1544 warnings) | OK (3325 warnings) |
| Warning types | 1543 lifetime elision + 1 cfg | 3324 lifetime elision + 1 cfg |

Warnings are all benign — lifetime syntax style from svd2rust codegen and a missing `critical-section` Cargo feature.

The SPI type name conflict warnings confirm the documented `svd2rust#16` overloaded-register issue.

### 1.4 Project Structure
```
mk20dx-pac/
├── svd/MK20D5.svd, MK20D7.svd
├── reference/
│   ├── kinetis.h, core_pins.h, mk20dx128.c
│   ├── K20P64M72SF1RM.pdf (72MHz ref manual)
│   ├── K20P64M50SF0RM.pdf (50MHz ref manual)
│   ├── refman_chapters/        (extracted 72MHz chapters, gitignored)
│   └── refman_50mhz_chapters/  (extracted 50MHz chapters, gitignored)
├── devices/mk20d5.yaml, mk20d7.yaml
├── patches/common/, mk20d5/, mk20d7/
├── scripts/
│   ├── compare_header_svd.py
│   ├── missing_summary.py
│   └── extract_refman_chapters.py
├── reports/mk20d5_comparison.json, mk20d7_comparison.json
├── mk20d5/Cargo.toml + src/ (generated)
├── mk20d7/Cargo.toml + src/ (generated)
├── Makefile
├── pyproject.toml + uv.lock (Python deps via uv)
└── .gitignore
```

---

## Phase 2: Comparison Script — COMPLETE

### Script: `scripts/compare_header_svd.py`

Parses kinetis.h (with preprocessor-aware filtering for chip variants) and SVD files, then cross-references:
- Register addresses (by absolute address)
- Register sizes (8/16/32-bit via struct typedefs)
- Bitfield widths and offsets (parameterized macro fields)
- IRQ numbers

**Capabilities:**
- Preprocessor state machine handles `#if defined()`, `#ifdef`, `#elif`, `#else`, `#endif` nesting
- Variant-specific symbol tables (`KINETISK`, `HAS_KINETIS_*` feature flags)
- K64/K66-only peripheral filtering (ENET, SDHC, USBHS, CAN1, FTM3, DAC1, etc.)
- Struct typedef parsing for register size verification (UART, MCG, I2C, SPI, LPUART)
- Handles struct instances with both direct hex addresses and named address constants
- Peripheral-aware field matching (avoids cross-peripheral false positives)
- JSON + human-readable report output

**Known limitations:**
- Does not parse `#if NUM > VALUE` style preprocessor conditions
- Some kinetis.h registers from the shared `KINETISK` section belong to K64/K66-only peripherals but aren't guarded by `#if defined(HAS_...)` — these show as false MISSING_IN_SVD
- DMA channels beyond what the chip supports (MK20DX128: 4, MK20DX256: 16) appear as MISSING_IN_SVD because kinetis.h defines up to 32 for K66
- FMC register matching in MK20D7 can produce false "matches" — the script matches by address first, so if a different register exists at the same address (e.g., TAGVDW0S2 vs TAGVDW1S0), it silently accepts the match

### Script: `scripts/extract_refman_chapters.py`

Extracts K20 reference manual PDFs into per-chapter markdown files using PyMuPDF. Parses the PDF table of contents to identify chapter boundaries, extracts text page-by-page. Outputs to `reference/refman_chapters/` (72MHz) or `reference/refman_50mhz_chapters/` (50MHz).

### Comparison Results: MK20D7 (Teensy 3.1/3.2) — Unpatched

| Metric | Value |
|--------|-------|
| Header registers parsed | 1546 |
| Header bitmasks parsed | 1066 |
| Header param fields parsed | 191 |
| Header IRQs parsed | 69 |
| SVD registers | 1594 |
| SVD fields | 6625 |
| SVD interrupts | 72 |
| **Registers matched** | **949** |
| Registers missing from SVD | 224 (mostly DMA ch16-31, AXBS, MCM overflow from kinetis.h) |
| K64/K66-only skipped | 375 |
| **IRQs matched** | **67/69** |
| IRQ mismatches | IRQ_MCG (name mismatch), IRQ_SOFTWARE (not in SVD) |
| **Fields matched** | **115/117** |
| Field width mismatches | 2 (DMA TCD BITER/CITER — false positive, see below) |

**Verdict: MK20D7 SVD is clean.** No real bugs found. Register sizes correct. No IRQ +16 offset bug. The 2 field width "mismatches" are false positives from comparing ELINKYES vs ELINKNO register variants.

### Comparison Results: MK20D5 (Teensy 3.0) — After Patching

| Metric | Before Patches | After Patches |
|--------|---------------|---------------|
| **Registers matched** | **592** | **604** |
| Registers missing from SVD | 567 | 567 |
| K64/K66-only skipped | 375 | 375 |
| **Address mismatches** | **12** (all FMC) | **0** |
| **Field width mismatches** | **4** (2 SIM + 2 DMA) | **2** (DMA false positives only) |
| **IRQs matched** | **43/45** | **43/45** |

---

## Phase 3: SVD Patches — COMPLETE (Correctness)

### Confirmed Bugs Found and Patched

#### Bug 1: SIM_SOPT5 UART TX Source Field Widths (MK20D5 only)

| | Detail |
|---|---|
| **Patch file** | `patches/mk20d5/sim/sopt5_uart_txsrc.yaml` |
| **Affected registers** | SIM_SOPT5 UART0TXSRC, SIM_SOPT5 UART1TXSRC |
| **Bug** | bitWidth=1 in MK20D5.svd, should be bitWidth=2 |
| **Impact** | Writing values 2 or 3 to these fields would silently truncate to 0 or 1 |
| **Evidence** | kinetis.h: `(((n) & 3) << 0)` and `(((n) & 3) << 4)` = 2-bit masks |
| | MK20D7.svd: correctly defines both as bitWidth=2 |
| | K20P64M50SF0RM chapter 12: SIM_SOPT5 shows 2-bit fields |
| **Not present in MK20D7** | MK20D7.svd already has the correct bitWidth=2 |

#### Bug 2: FMC Cache Register Addresses (MK20D5 only)

| | Detail |
|---|---|
| **Patch file** | `patches/mk20d5/fmc/cache_addresses.yaml` |
| **Affected registers** | 8 TAG registers (TAGVDW1-3 S0,S1) + 8 DATA registers (DATAW0-3 S0,S1) |
| **Bug** | MK20D5.svd uses the 72MHz K20 FMC layout (4-way, 8-set, wide way-spacing) but the MK20DX128 is a 50MHz part with a smaller FMC (4-way, 2-set, dense contiguous layout) |
| **Impact** | All FMC cache tag/data register accesses after way 0 would read/write wrong addresses |
| **Root cause** | The MK20D5 SVD was generated from (or copied from) the 72MHz sub-family's register map |

**FMC Address Details:**

| Register | MK20D5 SVD (wrong) | 50MHz Ref Manual (correct) | Difference |
|----------|-------------------|---------------------------|------------|
| TAGVDW0S0 | 0x100 | 0x100 | OK |
| TAGVDW1S0 | **0x120** | **0x108** | -0x18 |
| TAGVDW2S0 | **0x140** | **0x110** | -0x30 |
| TAGVDW3S0 | **0x160** | **0x118** | -0x48 |
| DATAW0S0 | **0x204** | **0x200** | -0x04 |
| DATAW0S1 | **0x20C** | **0x204** | -0x08 |
| DATAW1S0 | **0x244** | **0x208** | -0x3C |
| DATAW2S0 | **0x284** | **0x210** | -0x74 |
| DATAW3S0 | **0x2C4** | **0x218** | -0xAC |

Additionally, the DATA register `dimIncrement` was corrected from 0x8 (64-bit U/L split) to 0x4 (single 32-bit data word), matching the 50MHz FMC's 32-bit cache line size.

**Evidence:**
- K20P64M50SF0RM (50MHz ref manual), chapter 27: "4-way, 2-set, 32-bit line size cache" with dense contiguous addresses
- K20P64M72SF1RM (72MHz ref manual), chapter 27: "4-way, 8-set, 64-bit line size cache" with wide spacing — correct for MK20D7
- kinetis.h: matches the 50MHz ref manual addresses exactly
- **Not present in MK20D7**: MK20D7.svd's wider layout is correct for the 72MHz part

**Why the MK20D5 and MK20D7 differ:**

The 50MHz and 72MHz K20 sub-families have physically different FMC cache hardware:

| Feature | 50MHz (MK20D5) | 72MHz (MK20D7) |
|---------|----------------|----------------|
| Cache geometry | 4-way, 2-set | 4-way, 8-set |
| Cache line size | 32-bit | 64-bit |
| TAG way spacing | 0x08 (dense) | 0x20 (sparse) |
| DATA way spacing | 0x08 (dense, single word) | 0x40 (sparse, upper+lower) |
| DATA registers | DATAWxSy (32-bit) | DATAWxSyU + DATAWxSyL (64-bit) |
| PFB1CR register | Not present | Present at offset 0x008 |
| Total cache entries | 8 (4×2) | 32 (4×8) |

### Known SVD Bugs Verified NOT Present

All known Kinetis SVD bugs from community reports were checked against both SVDs and the K20 reference manuals:

| Known Bug | MK20D5.svd | MK20D7.svd | Source |
|-----------|-----------|-----------|--------|
| IRQ +16 offset (all device IRQs shifted by 16) | **Not present** — all 43 IRQs match kinetis.h | **Not present** — all 67 matched IRQs correct | Ref manual IRQ tables, kinetis.h enum |
| UART register size (should be 8-bit, not 32-bit) | **Correct** — all UART regs are 8-bit | **Correct** — all UART regs are 8-bit | kinetis.h KINETISK_UART_t struct (uint8_t) |
| MCG register size (should be 8-bit) | **Correct** — all MCG regs are 8-bit | **Correct** — all MCG regs are 8-bit | kinetis.h KINETIS_MCG_t struct (uint8_t) |
| I2C register size (should be 8-bit) | **Correct** — all I2C regs are 8-bit | **Correct** — all I2C regs are 8-bit | kinetis.h KINETIS_I2C_t struct (uint8_t) |
| DMA NBYTES_MLOFFYES (should be 10-bit) | **Correct** — 10-bit | **Correct** — 10-bit | kinetis.h mask 0x3FF |
| DMA NBYTES_MLOFFNO (should be 30-bit) | **Correct** — 30-bit | **Correct** — 30-bit | kinetis.h mask 0x3FFFFFFF |
| FTM_CONF BDMMODE (should be [7:6]) | **Correct** — offset=6, width=2 | **Correct** — offset=6, width=2 | kinetis.h `(((n) & 3) << 6)` |
| FTM_CONF NUMTOF (should be [4:0]) | **Correct** — offset=0, width=5 | **Correct** — offset=0, width=5 | kinetis.h `(((n) & 31) << 0)` |
| DMAMUX SPI0_TX source (should be 15/17) | N/A — no enums | N/A — no enums | Ergonomics only, no values to fix |

### Comparison Script False Positives (Not Bugs)

| Issue | Explanation |
|-------|-------------|
| DMA TCD BITER_ELINKYES/ELINKNO field width (×2) | Overloaded registers at the same address (svd2rust#16). ELINKYES has 9-bit BITER, ELINKNO has 15-bit BITER. The comparison script matches ELINKYES header fields against the ELINKNO SVD register. Both variants are correctly defined. |
| ~567 MISSING_IN_SVD (MK20D5) | Mostly: DMA channels 4-31 (kinetis.h defines 32, MK20DX128 has 4), K64/K66-only peripherals in shared KINETISK section (ADC1, CAN0, FTM2, DAC0, etc.), ARM core registers (SCB, NVIC, SYST), AXBS/MCM/AIPS not in MK20D5 SVD |
| ~224 MISSING_IN_SVD (MK20D7) | DMA channels 16-31, ARM core registers, some missing peripherals (AXBS, MCM) |

### Patch Pipeline

Full pipeline for MK20D5 (patched):
```
svdtools patch devices/mk20d5.yaml → svd/MK20D5.svd.patched
svd2rust -i svd/MK20D5.svd.patched → mk20d5/lib.rs
form -i lib.rs -o src/ → 562 .rs files
cargo check --target thumbv7em-none-eabi → OK (1544 warnings)
```

MK20D7 required no correctness patches — builds clean from unpatched SVD. (Ergonomics patches added in Phase 5.)

### Patches Still Needed

**Priority 1 — Blocking/Correctness: ALL DONE**
- [x] SIM_SOPT5 UART TX source field widths (MK20D5) — patched
- [x] FMC cache register addresses (MK20D5) — patched
- [x] Verify IRQ numbering — confirmed correct, no +16 offset bug
- [x] Verify UART/MCG/I2C register sizes — confirmed correct (8-bit)
- [x] Verify DMA NBYTES field widths — confirmed correct
- [x] Verify FTM_CONF field positions — confirmed correct

**Priority 2 — Ergonomics:**
- [ ] Register array collection (FTM channels, DMA TCDs, PORT PCR registers)
- [ ] Register clustering (DMA TCD — blocked by svd2rust#16)
- [x] Enumerated values for common fields — investigated, most already present (see Phase 5)
- [ ] Peripheral name prefix stripping
- [x] DMAMUX source enumerated values — patched (see Phase 5)

---

## Reference Manuals

Two separate reference manuals exist for the two K20 sub-families:

| Document | Sub-Family | Chips Covered | Pages |
|----------|-----------|--------------|-------|
| K20P64M50SF0RM (Rev 2, Feb 2012) | 50MHz K20 | MK20DX32, MK20DX64, MK20DX128 (VLH5) | ~1100 |
| K20P64M72SF1RM (Rev 1.1, Dec 2012) | 72MHz K20 | MK20DX64, MK20DX128, MK20DX256 (VLH7) | 1377 |

Both PDFs downloaded from PJRC and extracted to per-chapter markdown using `scripts/extract_refman_chapters.py`. The extracted chapters are gitignored but can be regenerated with:
```
uv run python3 scripts/extract_refman_chapters.py --pdf reference/K20P64M72SF1RM.pdf --outdir reference/refman_chapters
uv run python3 scripts/extract_refman_chapters.py --pdf reference/K20P64M50SF0RM.pdf --outdir reference/refman_50mhz_chapters
```

Key chapters for SVD verification:
- Chapter 3: Chip Configuration (IRQ tables, peripheral availability)
- Chapter 12: SIM (System Integration Module)
- Chapter 21: eDMA (Direct Memory Access Controller)
- Chapter 27: FMC (Flash Memory Controller) — differs between sub-families

---

## Phase 4: Validation — COMPLETE

### 4.1 Post-Patch Comparison Reports

Both comparison reports regenerated from patched SVDs (MK20D5 uses `svd/MK20D5.svd.patched`, MK20D7 uses unpatched `svd/MK20D7.svd`).

| Metric | MK20D5 (post-patch) | MK20D7 |
|--------|-------------------|--------|
| Registers matched | 604 | 949 |
| Address mismatches | **0** | **0** |
| Field width mismatches | 2 (DMA false positives) | 2 (DMA false positives) |
| IRQs matched | 43/45 | 67/69 |
| Missing from SVD | 569 | 224 |

All ADDRESS_MISMATCH issues are resolved. The only FIELD_WIDTH_MISMATCH entries are the known DMA TCD BITER/CITER false positives (ELINKYES vs ELINKNO register variants at the same address).

### 4.2 Peripheral Base Address Verification

Script `scripts/validate_phase4.py` verified all peripheral base addresses against K20 reference manual chapter 4 memory maps.

- **MK20D5**: 44 peripherals verified, 0 failures
- **MK20D7**: 48 peripherals verified, 0 failures

Key peripherals confirmed correct:

| Peripheral | Base Address | Verified |
|-----------|-------------|----------|
| DMA | 0x40008000 | OK |
| FMC | 0x4001F000 | OK |
| FTFL | 0x40020000 | OK |
| SPI0 | 0x4002C000 | OK |
| FTM0 | 0x40038000 | OK |
| PIT | 0x40037000 | OK |
| ADC0 | 0x4003B000 | OK |
| SIM | 0x40047000 | OK |
| PORTA-E | 0x40049000-4004D000 | OK |
| MCG | 0x40064000 | OK |
| I2C0 | 0x40066000 | OK |
| UART0 | 0x4006A000 | OK |
| WDOG | 0x40052000 | OK |
| USB0 | 0x40072000 | OK |

### 4.3 IRQ Table Verification

IRQ numbers verified against reference manual chapter 3 interrupt vector tables.

- **MK20D5**: 16 spot-checks passed, 0 failures. Max IRQ=44, within expected range (0-45).
- **MK20D7**: 16 spot-checks passed, 0 failures. Max IRQ=91, within expected range (0-94).

Known absent IRQs (not bugs):
- MCG IRQ (38/84): MCG peripheral in SVD doesn't declare an interrupt element
- Software IRQ (45/94): ARM-only vector, no peripheral source in SVD

### 4.4 Cross-Variant Consistency

Shared peripherals compared between MK20D5 and MK20D7 for register layout consistency.

- **33 peripherals** verified consistent (identical register names, offsets, sizes, and fields)
- **0 unexpected differences** found

Known/expected differences (not errors):
- **DMA**: MK20D5 has 4 channels, MK20D7 has 16
- **FMC**: Different cache geometry (50MHz 2-set vs 72MHz 8-set)
- **UART0/1**: MK20D5 SVD includes extra CEA709 (IR) registers
- **SIM**: MK20D7 has extra SCGC1/2/3 for additional peripherals
- **PORTs**: MK20D7 has digital filter registers (DFER/DFCR/DFWR)
- **ADC0**: MK20D7 has PGA register
- **PDB0**: MK20D7 has 2 channels + DAC trigger

### 4.5 Generated Code Spot-Check

Manual read-through of critical peripherals in generated Rust code:

| Peripheral | Check | Result |
|-----------|-------|--------|
| UART0 (MK20D7) | Register type = u8 | `type Ux = u8` confirmed |
| UART0 (MK20D7) | BDH/BDL/C1/C2/S1/S2 offsets | 0x00-0x05 (byte-packed), correct |
| SPI0 (MK20D7) | Base address | 0x4002_c000, correct |
| SPI0 (MK20D7) | MCR/TCR/CTAR0 offsets | 0x00/0x08/0x0c, correct |
| FTM0 (MK20D7) | Channel registers | CSC[0..8] at 0x0c (stride 8), CV[0..8] at 0x10, correct |
| FTM0 (MK20D7) | CONF offset | 0x84, correct |
| DMA TCD (MK20D7) | NBYTES_MLOFFYES.NBYTES | `FieldWriter<..., 10, u16>`, 10-bit confirmed |
| DMA TCD (MK20D7) | MLOFF field | 20-bit at [10:29], correct |
| FMC (MK20D5) | TAGVDW0-3 offsets | 0x100/0x108/0x110/0x118 (dense), correct |
| FMC (MK20D5) | DATAW0-3 offsets | 0x200/0x208/0x210/0x218 (dense), correct |
| FMC (MK20D5) | Set count | 2 per way (array of 2), correct for 50MHz |

### 4.6 Full Rebuild

Clean rebuild completed successfully:
```
svdtools patch → MK20D5.svd.patched (2 patches), MK20D7.svd.patched (0 patches)
svd2rust → mk20d5/src/ (562 files), mk20d7/src/ (860 files)
cargo check mk20d5 → OK (1544 warnings)
cargo check mk20d7 → OK (3325 warnings)
```

Warnings are all benign (1543/3324 lifetime elision + 1 cfg each).

---

## Phase 5: Ergonomics Patches — IN PROGRESS

### 5.1 DMAMUX SOURCE Enumerated Values — COMPLETE

The DMAMUX CHCFG SOURCE field was a bare 6-bit field with zero enums, requiring raw numeric literals for every DMA channel configuration. Named source enums were added for both variants.

| | MK20D5 (Teensy 3.0) | MK20D7 (Teensy 3.1/3.2) |
|---|---|---|
| **Patch file** | `patches/mk20d5/dmamux/source_enums.yaml` | `patches/mk20d7/dmamux/source_enums.yaml` |
| **Source enum variants** | 43 (including Disabled + 10 AlwaysOn) | 50 (including Disabled + 10 AlwaysOn) |
| **Named DMA sources** | 32 | 39 |
| **Source** | kinetis.h lines 88-129 (`__MK20DX128__`) | kinetis.h lines 220-268 (`__MK20DX256__`) |

MK20D5 lacks these peripherals compared to MK20D7: SPI1 (RX/TX), I2C1, FTM2 (CH0/CH1), ADC1, CMP2.

**Generated API example (before vs after):**
```rust
// Before: raw numeric value, easy to get wrong
dmamux.chcfg[0].write(|w| unsafe { w.source().bits(16) });

// After: named enum variant, self-documenting
dmamux.chcfg[0].write(|w| w.source().variant(Source::Spi0rx));
```

Reader returns `Option<Source>` (sparse enum — not all 64 values are named). Writer methods are safe (no `unsafe` needed).

### 5.2 Existing Enumerated Values — Already Complete

Investigation of the MK20D7 SVD revealed that most peripherals already have comprehensive enumerated values:

| Peripheral | Field | Status |
|-----------|-------|--------|
| SIM | SCGC1-7 clock gate bits | Already has `Enabled`/`Disabled` enums |
| PORT | PCR MUX field | Already has `Alt0`-`Alt7` pin mux enums |
| UART0 | C1/C2/S1/S2/BDH control fields | Already has named bit enums |
| FTM0 | SC CLKS, PS fields | Already has clock source and prescaler enums |
| ADC0 | SC1n/CFG1/CFG2 fields | Already has mode, clock, resolution enums |
| GPIO | PDDR direction bits | Already has `Input`/`Output` enums |

No additional enum patches needed for these peripherals.

### 5.3 Remaining Ergonomics Work

| Item | Status | Notes |
|------|--------|-------|
| DMA TCD clustering | Blocked | svd2rust#16 — overloaded registers at same offset prevent proper clustering |
| Register array collection | Future | FTM channels, PORT PCR registers |
| Peripheral name prefix stripping | Future | Lower priority, cosmetic |

---

## What's Next

1. **Phase 5 continued**: Additional ergonomics patches (register arrays, prefix stripping)
2. **Phase 6: Publishing** — crate metadata, docs, crates.io

---

## File Inventory

| File | Purpose | Status |
|------|---------|--------|
| `svd/MK20D5.svd` | Vendor SVD (unmodified) | Downloaded from cmsis-svd-data |
| `svd/MK20D7.svd` | Vendor SVD (unmodified) | Downloaded from cmsis-svd-data |
| `svd/MK20D5.svd.patched` | Patched SVD (generated) | Generated by svdtools, gitignored |
| `reference/kinetis.h` | Teensyduino register defs | Downloaded from PaulStoffregen/cores |
| `reference/core_pins.h` | Pin mappings | Downloaded |
| `reference/mk20dx128.c` | Startup code | Downloaded |
| `reference/K20P64M72SF1RM.pdf` | 72MHz K20 ref manual | Downloaded from PJRC |
| `reference/K20P64M50SF0RM.pdf` | 50MHz K20 ref manual | Downloaded from PJRC |
| `reference/refman_chapters/` | Extracted 72MHz chapters (51 files) | Generated, gitignored |
| `reference/refman_50mhz_chapters/` | Extracted 50MHz chapters (49 files) | Generated, gitignored |
| `devices/mk20d5.yaml` | svdtools device config | Includes 3 patches |
| `devices/mk20d7.yaml` | svdtools device config | Includes 1 patch |
| `patches/mk20d5/sim/sopt5_uart_txsrc.yaml` | SIM_SOPT5 field width fix | Applied, verified |
| `patches/mk20d5/fmc/cache_addresses.yaml` | FMC address + dimIncrement fix | Applied, verified |
| `patches/mk20d5/dmamux/source_enums.yaml` | DMAMUX SOURCE field enums (43 variants) | Applied, verified |
| `patches/mk20d7/dmamux/source_enums.yaml` | DMAMUX SOURCE field enums (50 variants) | Applied, verified |
| `scripts/compare_header_svd.py` | Header↔SVD comparison | Working, produces JSON+text reports |
| `scripts/missing_summary.py` | Quick MISSING_IN_SVD summary | Utility script |
| `scripts/extract_refman_chapters.py` | PDF→Markdown chapter extraction | Working |
| `scripts/validate_phase4.py` | Phase 4 validation (base addrs, IRQs, cross-variant) | Passing, all checks green |
| `reports/mk20d5_comparison.json` | MK20D5 comparison results | Post-patch, 0 address mismatches |
| `reports/mk20d7_comparison.json` | MK20D7 comparison results | Clean, 0 real issues |
| `mk20d5/Cargo.toml` | Generated crate manifest | Targets thumbv7em-none-eabi |
| `mk20d7/Cargo.toml` | Generated crate manifest | Targets thumbv7em-none-eabi |
| `mk20d5/src/` | Generated PAC code | 562 files, compiles OK |
| `mk20d7/src/` | Generated PAC code | 860 files, compiles OK |
| `Makefile` | Build automation | patch, svd2rust, fmt, check targets |
| `pyproject.toml` | Python project (uv) | cmsis-svd + pymupdf dependencies |
