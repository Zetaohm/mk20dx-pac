# mk20dx-pac: Research Findings

## Table of Contents

1. [Target Hardware](#1-target-hardware)
2. [Existing Rust Support](#2-existing-rust-support)
3. [NXP SVD File Quality](#3-nxp-svd-file-quality)
4. [Known Bugs in NXP Kinetis SVDs](#4-known-bugs-in-nxp-kinetis-svds)
5. [kinetis.h as a Reference Source](#5-kinetish-as-a-reference-source)
6. [Known Corrections in kinetis.h](#6-known-corrections-in-kinetish)
7. [SVD Patching Ecosystem and Tooling](#7-svd-patching-ecosystem-and-tooling)
8. [svdtools YAML Patch Format Reference](#8-svdtools-yaml-patch-format-reference)
9. [Existing Kinetis SVD Patch Projects](#9-existing-kinetis-svd-patch-projects)
10. [Comparison Tooling](#10-comparison-tooling)
11. [Community Vendor SVD Quality Rankings](#11-community-vendor-svd-quality-rankings)
12. [Key URLs and Resources](#12-key-urls-and-resources)

---

## 1. Target Hardware

This project targets the two MK20DX variants used in the Teensy 3.0, 3.1, and 3.2:

### Target Chips

| Chip | Teensy Board | Flash | RAM | SVD Sub-Family | SVD File |
|------|-------------|-------|-----|----------------|----------|
| **MK20DX128VLH5** | Teensy 3.0 | 128 KB | 16 KB | MK20D5 | `MK20D5.svd` |
| **MK20DX256VLH7** | Teensy 3.1/3.2 | 256 KB | 64 KB | MK20D7 | `MK20D7.svd` |

Both are ARM Cortex-M4 chips from the NXP (formerly Freescale) Kinetis K20 family, clocked at up to 72 MHz (MK20DX256) or 48 MHz (MK20DX128). They share the same peripheral set with differences primarily in memory size, clock speed, and DMA channel count.

- **Reference Manual:** K20 Sub-Family Reference Manual (K20P64M72SF1RM)
  - Available at: https://www.pjrc.com/teensy/K20P64M72SF1RM.pdf

### Peripherals Available on MK20DX128/MK20DX256

From the Teensyduino core `kinetis.h` feature flags:

| Peripheral | Details |
|------------|---------|
| UART | UART0 (with FIFO), UART1 (with FIFO), UART2 (no FIFO) |
| SPI | SPI0, SPI1 (DSPI) |
| I2C | I2C0, I2C1 |
| ADC | ADC0, ADC1 |
| FTM | FTM0, FTM1, FTM2 (FlexTimer/PWM) |
| PIT | 4-channel Periodic Interrupt Timer |
| DMA | eDMA with 16 channels |
| TSI | Touch Sense Input |
| CMP | Comparator(s) |
| DAC | DAC0, DAC1 |
| PDB | Programmable Delay Block |
| USB | USB OTG (full-speed) |
| CAN | FlexCAN |
| I2S | SAI (audio) |
| RTC | Real-Time Clock |
| WDOG | Watchdog |
| Flash | FTFL |
| GPIO | Ports A-E |
| LLWU | Low-Leakage Wakeup (16 channels) |
| MCG | Multipurpose Clock Generator |

**Not present on MK20DX128/MK20DX256:** ENET, SDHC, USBHS, LPUART, FTM3, RNG/RNGA (only on K64/K66).

### Differences Between MK20DX128 and MK20DX256

| Feature | MK20DX128 (Teensy 3.0) | MK20DX256 (Teensy 3.1/3.2) |
|---------|------------------------|----------------------------|
| Flash | 128 KB | 256 KB |
| RAM | 16 KB | 64 KB |
| Max Clock | 48 MHz | 72 MHz |
| DMA Channels | 4 | 16 |
| Interrupts | 46 | 95 |
| kinetis.h lines | 37-142 | 145-283 |
| kinetis.h symbol | `__MK20DX128__` | `__MK20DX256__` |

Both chips share the same peripheral register definitions (gated behind `#if defined(KINETISK)` in kinetis.h). The differences are in the chip-specific sections: IRQ enumerations, DMA MUX source mappings, and feature flags.

---

## 2. Existing Rust Support

### PAC Crates (None Maintained)

| Repository | Status | Published | Notes |
|------------|--------|-----------|-------|
| [exrook/mk20d7](https://github.com/exrook/mk20d7) | Abandoned (1 commit, Feb 2018) | No | svd2rust-generated, unpatched SVD, 0 stars |
| [dfrankland/mk20d7](https://github.com/dfrankland/mk20d7) | Abandoned (last update ~2023) | No | svd2rust-generated, unpatched SVD |

Neither repository applies any SVD patches. Neither is published on crates.io.

### Teensy 3.x Rust Projects

- **teensy3 crate**: Provides boilerplate for Rust on Teensy 3.1/3.2 but wraps the Teensyduino C API via `unsafe` FFI bindings. Not a native Rust PAC.
- **Branan Riley's "Exploring Rust on Teensy" blog series**: Bare-metal Rust tutorials for Teensy 3.x (2017). Good reference for boot sequence and linker scripts but does not produce a reusable PAC.
  - https://branan.github.io/teensy/2017/01/12/bootup.html

### Teensy 4.x Rust Projects (For Comparison)

The Teensy 4.0/4.1 (i.MX RT1062) has a mature Rust ecosystem:

| Project | URL | Notes |
|---------|-----|-------|
| teensy4-rs | https://github.com/mciantyre/teensy4-rs | BSP + examples |
| teensy4-bsp | https://docs.rs/teensy4-bsp | Published crate |
| imxrt-hal | https://github.com/imxrt-rs/imxrt-hal | HAL layer |
| imxrt-ral | https://github.com/imxrt-rs/imxrt-ral | Custom RAL (abandoned svd2rust) |

The imxrt-rs project **abandoned svd2rust** because the i.MX RT SVD produced a 932,000-line crate requiring 16 GB RAM and 8 minutes to compile. They built a custom Register Access Layer instead. The K20 SVD is much smaller, so this should not be an issue for us.

### Related Kinetis PAC Crates

| Crate | Chip | URL | Notes |
|-------|------|-----|-------|
| k66 | MK66FX1M0 (Teensy 3.6) | https://lib.rs/crates/k66 | Published on crates.io |
| mkl25z4 | MKL25Z4 | https://crates.io/crates/mkl25z4 | Published on crates.io |

No published crate exists for K20/MK20D5/MK20D7.

---

## 3. NXP SVD File Quality

### Community Consensus

NXP SVD files are widely regarded as buggy across all product families. Key quotes:

- **lpc-rs maintainer (Hanno Braun):** Described NXP LPC SVD files as **"so buggy"** that the project rejected using vanilla (unmodified) SVD files entirely. ([lpc-pac#4](https://github.com/lpc-rs/lpc-pac/issues/4))

- **NXP LPC82x developer:** Found so many bugs they recommended **"anyone who considers using them do the same"** (stay away). ([NXP Community](https://community.nxp.com/t5/LPC-Microcontrollers/Numerous-problems-in-LPC82x-SVD-file/m-p/770669))

- **svd2rust maintainer**, in response to NXP SVD producing broken code: **"Broken SVD leads to broken code."** ([svd#263](https://github.com/rust-embedded/svd/issues/263))

- **Rust embedded working group (wg#101):** Noted NXP is actually **better than most vendors at updating SVD files** and maintaining quality, but obtaining them is tedious (bundled in MCUXpresso SDK, using `.xml` extension, per-package-variant instead of per-chip). ([wg#101](https://github.com/rust-embedded/wg/issues/101))

### SVD Distribution Challenges

- Freescale refused to distribute SVD files through the official ARM CMSIS channel
- SVD files were bundled in Kinetis Design Studio, later MCUXpresso SDK
- MCUXpresso SDK SVDs use `.xml` extension instead of `.svd`
- NXP tends to have one SVD per package/variant, not per chip
- SVD files in the cmsis-svd repository are **incredibly out of date** since the Freescale/NXP acquisition

### SVD File Sources

| Source | URL | Notes |
|--------|-----|-------|
| cmsis-svd-data | https://github.com/cmsis-svd/cmsis-svd-data | `data/Freescale/MK20D7.svd` — likely stale |
| MCUXpresso SDK | https://mcuxpresso.nxp.com/ | Bundled with SDK, `.xml` extension |
| Keil CMSIS Pack | https://www.keil.com/dd2/Pack/ | NXP DFP packs contain SVDs |
| kinetis-chip-equates | https://github.com/nimblemachines/kinetis-chip-equates | Community collection from KSDK 1.3.0 and DFP packs |
| exrook/mk20d7 repo | https://github.com/exrook/mk20d7 | Contains `MK20D7.svd` directly |

There are 5 MK20-family SVDs in the cmsis-svd-data repository:
`MK20D5.svd`, `MK20D7.svd`, `MK20D10.svd`, `MK20DZ10.svd`, `MK20F12.svd`

**We need both `MK20D5.svd` (Teensy 3.0) and `MK20D7.svd` (Teensy 3.1/3.2).**

---

## 4. Known Bugs in NXP Kinetis SVDs

### Confirmed Kinetis-Wide Issues

#### IRQ Offset Error
All Freescale Kinetis device-specific IRQs were incorrectly offset by 16 in the SVD files. This is a fundamental error affecting the interrupt vector table representation across the entire Kinetis family.
- Source: [cmsis-svd#33](https://github.com/cmsis-svd/cmsis-svd/issues/33)

#### Incorrect Register Grouping (Missing Clusters)
Freescale Kinetis SVD files represent what should logically be an array of grouped registers (clusters) as separate dimensioned arrays of individual registers. For example, a timer peripheral with 6 channels defines separate `<dim>` arrays for channel status/control registers and channel value registers, instead of using the SVD `<cluster>` element to group them. This produces bloated generated code: `C0SC`, `C0V`, `C1SC`, `C1V`... instead of `CHANNELS[n]`.
- Source: [cmsis-svd#33](https://github.com/cmsis-svd/cmsis-svd/issues/33)

#### Overloaded Registers (svd2rust#16)
When two registers share the same memory offset (e.g., input capture mode vs. output compare mode on timer peripherals), svd2rust can only emit one of them. The K64 and K66 PAC crates explicitly document that "due to svd2rust Issue 16 there are some registers missing."
- Source: [svd2rust#16](https://github.com/rust-embedded/svd2rust/issues/16)

### Bugs Found in Other NXP Families (Same SVD Generation Process)

These represent systemic issues in NXP's SVD generation tooling and are likely present in K20 SVDs:

#### i.MX RT SVD Defects (Reported to NXP, Acknowledged)

| Bug | Details | NXP Response |
|-----|---------|--------------|
| USBCMD[ATDTW] bit offset wrong | Offset was 12, should be 14 per reference manual | Acknowledged, will update future SDK |
| PIT[LDVALx] bit width wrong | SVD says 24-bit, actually 32-bit | Acknowledged, suggested direct register writes |

Source: [imxrt-ral#20](https://github.com/imxrt-rs/imxrt-ral/issues/20)

#### NXP LPC SVD Bugs (LPC82x, MCUXpresso SDK 2.6.0)

8 specific bugs found in a single SVD file:
1. Spurious registers that don't exist on the device (`PINASSIGN_DATA`, `FMSTAT`, `FMSTATCLR`)
2. Incorrect reset values (e.g., `PDAWAKECFG`)
3. Wrong access specifications (e.g., USART `STAT` register)
4. Duplicate enumerated value names
5. Register sizes specified as 1-bit wide when they should be 32-bit
6. MRT timer value resolution of 30 bits specified as 24 bits
7. Missing bits in control registers (`SEL_EXTCLK`, `ADC_RST_N`, `DMA_RST_N`)
8. Missing registers entirely (`IOCONCLKDIV5-0`)

Sources:
- [lpc-pac#40](https://github.com/lpc-rs/lpc-pac/issues/40)
- [NXP Community LPC82x thread](https://community.nxp.com/t5/LPC-Microcontrollers/Numerous-problems-in-LPC82x-SVD-file/m-p/770669)

#### S32K344 SVD
Duplicate enumerated value names with different numeric values, producing uncompilable svd2rust output.
- Source: [svd#263](https://github.com/rust-embedded/svd/issues/263)

### svd2rust Issues Triggered by NXP SVDs

| Issue | Description | Impact |
|-------|-------------|--------|
| [#16](https://github.com/rust-embedded/svd2rust/issues/16) | Overloaded registers (same offset) | Missing register definitions |
| [#60](https://github.com/rust-embedded/svd2rust/issues/60) | Enum name collisions | Compilation errors |
| [#115](https://github.com/rust-embedded/svd2rust/issues/115) | Highly repetitive output from derivedFrom | Code bloat |
| [#137](https://github.com/rust-embedded/svd2rust/issues/137) | Wrong offsets for byte-sized GPIO registers | Incorrect peripheral access |
| [#166](https://github.com/rust-embedded/svd2rust/issues/166) | Freescale CI builds timing out | Large SVD files |
| [#352](https://github.com/rust-embedded/svd2rust/issues/352) | Duplicate enum names with different values | Uncompilable output |

---

## 5. kinetis.h as a Reference Source

### File Overview

| Property | Value |
|----------|-------|
| **URL** | https://github.com/PaulStoffregen/cores/blob/master/teensy3/kinetis.h |
| **Repository** | [PaulStoffregen/cores](https://github.com/PaulStoffregen/cores) (564 stars, 404 forks, actively maintained) |
| **File Size** | 383,162 bytes, 5,905 lines |
| **Total `#define` directives** | 4,768 |
| **Memory-mapped register addresses** | ~1,950 |
| **Bitfield/bitmask constants** | ~3,686 |
| **Parameterized bitfield macros** | ~295 |
| **Struct typedefs** | 7 peripherals |

### Chip Variants Covered

| Preprocessor Symbol | Chip | Teensy Board | Lines |
|---------------------|------|--------------|-------|
| `__MK20DX128__` | MK20DX128 | Teensy 3.0 | 37-142 |
| `__MK20DX256__` | MK20DX256 | Teensy 3.1/3.2 | 145-283 |
| `__MKL26Z64__` | MKL26Z64 | Teensy-LC | 286-376 |
| `__MK64FX512__` | MK64FX512 | Teensy 3.5 | 377-554 |
| `__MK66FX1M0__` | MK66FX1M0 | Teensy 3.6 | 556-868 |

Lines 145-283 define MK20DX256-specific: IRQ vector enumerations, DMA MUX source mappings, DMA channel counts, and `HAS_KINETIS_*` feature flags.

Lines 868+ define peripheral registers shared across K-series variants using `#if defined(KINETISK)` vs `#if defined(KINETISL)`.

### Register Definition Styles

**Style 1: Flat `#define` (most peripherals)**
```c
#define SIM_SCGC4    (*(volatile uint32_t *)0x40048034)
#define SIM_SCGC4_USBOTG    ((uint32_t)0x00040000)
```

**Style 2: Struct typedef (7 peripherals)**
```c
typedef struct __attribute__((packed)) {
    volatile uint8_t BDH;
    volatile uint8_t BDL;
    volatile uint8_t C1;
    // ...
} KINETISK_UART_t;
#define KINETISK_UART0  (*(KINETISK_UART_t *)0x4006A000)
#define UART0_BDH       (KINETISK_UART0.BDH)
```

Struct typedefs exist for: `KINETIS_MCG_t`, `KINETISK_PIT_CHANNEL_t`, `KINETISK_SPI_t`, `KINETISL_SPI_t`, `KINETIS_I2C_t`, `KINETISK_UART_t`, `KINETISK_LPUART_t`.

These struct typedefs are especially valuable for SVD patching because they reveal:
- Exact register sizes (8-bit vs 32-bit)
- Precise packing and padding between registers
- Reserved/unused byte offsets

### Register Definitions Per Peripheral

| Peripheral | Define Count |
|------------|-------------|
| DMA (eDMA + DMAMUX + TCDs) | 1,082 |
| USB (OTG + HS + PHY + DCD) | 442 |
| UART (0-5 + bitfields) | 352 |
| FTM (FlexTimer 0-3) | 329 |
| ENET (Ethernet) | 209 |
| PORT (pin control A-E) | 190 |
| SIM (System Integration Module) | 188 |
| SDHC | 171 |
| LLWU (Low-Leakage Wakeup) | 165 |
| SPI (DSPI 0-2) | 162 |
| I2C (0-2) | 124 |
| I2S/SAI | 111 |
| ADC (0-1) | 84 |
| DAC (0-1) | 81 |
| LPUART | 64 |
| CAN (FlexCAN 0-1) | 62 |
| TPM (Timer/PWM) | 61 |
| TSI (Touch Sense) | 58 |
| MCG (Clock) | 54 |
| CMP (Comparator) | 46 |
| PDB | 35 |
| RTC | 34 |
| SCB (System Control Block) | 34 |
| GPIO | 30 |
| SMC | 27 |
| NVIC | 27 |
| WDOG | 26 |
| PIT | 25 |
| RCM | 20 |
| PMC | 16 |
| VREF | 12 |

### Other Teensyduino Files with Register Definitions

| File | Size | Content |
|------|------|---------|
| [core_pins.h](https://github.com/PaulStoffregen/cores/blob/master/teensy3/core_pins.h) | 85KB, 2563 lines | Pin-to-PORT register mappings, FTM channel-to-pin mappings, digital read/write macros |
| [pins_arduino.h](https://github.com/PaulStoffregen/cores/blob/master/teensy3/pins_arduino.h) | 11KB | Arduino pin number to analog channel mappings (A0-A20) |
| [pins_teensy.c](https://github.com/PaulStoffregen/cores/blob/master/teensy3/pins_teensy.c) | 44KB | Analog init, PWM PORT_PCR_MUX configs, analogRead/analogWrite register programming |
| [mk20dx128.c](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx128.c) | 54KB | Startup code: clock init (MCG), watchdog, SIM_SCGC, PLL configuration |
| [mk20dx256.ld](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx256.ld) | - | Memory map: FLASH 0x00000000 (256K), RAM 0x1FFF8000 (64K) |
| [DMAChannel.h](https://github.com/PaulStoffregen/cores/blob/master/teensy3/DMAChannel.h) | 36KB | DMA TCD register access wrappers |
| [SPIFIFO.h](https://github.com/PaulStoffregen/cores/blob/master/teensy3/SPIFIFO.h) | 17KB | Direct SPI FIFO register access |
| [HardwareSerial.h](https://github.com/PaulStoffregen/cores/blob/master/teensy3/HardwareSerial.h) | 23KB | UART register access patterns |
| serial1.c-serial6_lpuart.c | Various | UART register programming, baud rate calculations |

The `mk20dx128.c` startup code is particularly valuable because it shows the actual register initialization sequence and clock configuration values known to work on real hardware.

### MK20DX256 Feature Flags

```c
#define KINETISK              // Cortex-M4 K-series
#define HAS_KINETISK_UART0    // UART0 with FIFO
#define HAS_KINETISK_UART1    // UART1 with FIFO
#define HAS_KINETISK_UART2    // UART2 (no FIFO)
// I2C0, I2C1
// ADC0, ADC1
// LLWU 16 channels
// TSI
// FTFL flash
// 16 DMA channels
// 95 interrupts
```

---

## 6. Known Corrections in kinetis.h

The following bugs were found and fixed in the Teensyduino core over the years. These are exactly the kinds of errors the SVD file likely still contains.

### Critical Address Corrections

#### RNG Base Address Completely Wrong
- **Commit:** [663d7eaa](https://github.com/PaulStoffregen/cores/commit/663d7eaa)
- **Issue:** [cores#136](https://github.com/PaulStoffregen/cores/issues/136)
- **Bug:** RNGA registers at 0x400A0000 (wrong)
- **Fix:** Correct base is 0x40029000
```c
// BEFORE (wrong)
#define RNG_CR  (*(volatile uint32_t *)0x400A0000)
#define RNG_SR  (*(volatile uint32_t *)0x400A0004)
#define RNG_ER  (*(volatile uint32_t *)0x400A0008)
#define RNG_OR  (*(volatile uint32_t *)0x400A000C)

// AFTER (correct)
#define RNG_CR  (*(volatile uint32_t *)0x40029000)
#define RNG_SR  (*(volatile uint32_t *)0x40029004)
#define RNG_ER  (*(volatile uint32_t *)0x40029008)
#define RNG_OR  (*(volatile uint32_t *)0x4002900C)
```
- **Note:** The clock gate enable bits were also confused between SIM_SCGC3 and SIM_SCGC6.
- **Relevance to MK20DX256:** The MK20DX256 does NOT have RNG/RNGA, so this specific fix does not apply. However, it demonstrates that NXP's base addresses can be completely wrong.

### DMA MUX Source Value Errors

#### SPI0_TX DMAMUX Source Duplicated
- **Commit:** [310f3598](https://github.com/PaulStoffregen/cores/commit/310f3598)
- **Bug:** `DMAMUX_SOURCE_SPI0_TX` was 14 (same as `SPI0_RX`)
- **Fix:** Corrected to 15
```c
// BEFORE
#define DMAMUX_SOURCE_SPI0_TX   14   // WRONG: same as SPI0_RX!

// AFTER
#define DMAMUX_SOURCE_SPI0_TX   15   // Correct
```

#### SPI1/SPI2 DMA Mux for Teensy 3.5
- **Commit:** [490d83c9](https://github.com/PaulStoffregen/cores/commit/490d83c9)
- **Bug:** SPI1 RX/TX defined as separate sources when they are combined
```c
// BEFORE
#define DMAMUX_SOURCE_SPI1_RX   16
#define DMAMUX_SOURCE_SPI1_TX   17

// AFTER
#define DMAMUX_SOURCE_SPI1      16   // combined RX/TX
#define DMAMUX_SOURCE_SPI2      17
```

### Bitfield Width/Mask Errors

#### DMA NBYTES Transfer Count Width
- **Commits:** [70a24272](https://github.com/PaulStoffregen/cores/commit/70a24272), [1ec34883](https://github.com/PaulStoffregen/cores/commit/1ec34883)
- **Bug 1:** `DMA_TCD_NBYTES_MLOFFNO_NBYTES` had no mask at all
- **Fix 1:** Added mask 0x3FFFFFFF (30-bit field)
- **Bug 2:** `DMA_TCD_NBYTES_MLOFFYES_NBYTES` mask was 0x1F (5 bits)
- **Fix 2:** Corrected to 0x3FF (10-bit field)
```c
// BEFORE
#define DMA_TCD_NBYTES_MLOFFNO_NBYTES(n)  ((uint32_t)(n))                  // no mask!
#define DMA_TCD_NBYTES_MLOFFYES_NBYTES(n) ((uint32_t)((n) & 0x1F))         // 5 bits!

// AFTER
#define DMA_TCD_NBYTES_MLOFFNO_NBYTES(n)  ((uint32_t)((n) & 0x3FFFFFFF))   // 30 bits
#define DMA_TCD_NBYTES_MLOFFYES_NBYTES(n) ((uint32_t)((n) & 0x3FF))        // 10 bits
```
- **SVD Impact:** The SVD bitfield definition must use `bitWidth=10` not `bitWidth=5` for MLOFFYES_NBYTES.

### Missing Macro Parameters

#### FTM_CONF Missing Parameter Names
- **Commit:** [6d441162](https://github.com/PaulStoffregen/cores/commit/6d441162)
- **Bug:** Macros referenced `(n)` without accepting it as a parameter
```c
// BEFORE (broken — `n` is undefined)
#define FTM_CONF_BDMMODE   (((n) & 3) << 6)
#define FTM_CONF_NUMTOF    (((n) & 31) << 0)

// AFTER (fixed)
#define FTM_CONF_BDMMODE(n) (((n) & 3) << 6)
#define FTM_CONF_NUMTOF(n)  (((n) & 31) << 0)
```
- **SVD Impact:** Confirms FTM_CONF BDMMODE is bits [7:6] (2-bit) and NUMTOF is bits [4:0] (5-bit). Verify SVD matches.

#### SDHC_XFERTYP_RSPTYP Missing Parameter
- **Commit:** [61dce339](https://github.com/PaulStoffregen/cores/commit/61dce339)
```c
// BEFORE
#define SDHC_XFERTYP_RSPTYP   (uint32_t)(((n) & 0x3)<<16)

// AFTER
#define SDHC_XFERTYP_RSPTYP(n) (uint32_t)(((n) & 0x3)<<16)
```

### Peripheral Availability Corrections

- **Teensy 3.5 LLWU:** Changed from `HAS_KINETIS_LLWU_32CH` to `HAS_KINETIS_LLWU_16CH` for MK64FX512. ([PR#204](https://github.com/PaulStoffregen/cores/pull/204))
- **TeensyLC CMP1/CMP2:** Removed CMP1 and CMP2 definitions (doesn't have those comparators). (Commit 94346ade)
- **I2C2 Clock Gate:** `SIM_SCGC4_I2C2` reported as belonging to `SIM_SCGC4` but actually belongs to `SIM_SCGC1`. ([cores#149](https://github.com/PaulStoffregen/cores/issues/149))
- **K66 SMC_STOPCTRL:** Missing register (uses `SMC_STOPCTRL` instead of `SMC_VLLSCTRL`). ([PR#237](https://github.com/PaulStoffregen/cores/pull/237))

### Community-Reported Missing Definitions

- **Flexbus registers missing:** `FB_CSAR0`, `FB_CSCR0`, `FB_CSMR0`, `FB_CSPMCR` and bitfield macros. ([PJRC Forum](https://forum.pjrc.com/threads/52088-Bug-report-missing-flexbus-memory-map-definitions-in-kinetis-h))
- **Third-party headers with wrong interrupt definitions:** The PJRC bare-metal guide notes that some third-party header files have wrong interrupt numbers (e.g., `INT_PIT1` incorrect) while Teensyduino's `kinetis.h` is correct.

---

## 7. SVD Patching Ecosystem and Tooling

### svdtools

- **URL:** https://github.com/rust-embedded/svdtools
- **Description:** "Python package to handle vendor-supplied, often buggy SVD files"
- **Installation:**
  ```bash
  pip3 install --upgrade --user svdtools   # Python version
  cargo install svdtools                    # Rust version (faster)
  ```
- **Usage:** `svdtools patch device.yaml` — applies YAML patches, outputs `.svd.patched`

### Exemplar Projects

#### stm32-rs (Gold Standard)
- **URL:** https://github.com/stm32-rs/stm32-rs
- **Structure:**
  ```
  stm32-rs/
  ├── svd/vendor/          # Vendor SVD zips
  ├── devices/
  │   ├── stm32f405.yaml   # Per-device patch file
  │   └── patches/         # 52+ peripheral patch subdirectories
  │       ├── adc/
  │       ├── dma/
  │       ├── gpio/
  │       ├── spi/
  │       ├── tim/
  │       └── ...
  ├── fields/              # Enumerated value definitions
  ├── collect/             # Array/cluster collection
  ├── scripts/makecrates.py
  └── Makefile
  ```
- **Design principle:** Common fixes factored into `patches/`, enumerated values into `fields/`, array/cluster collection into `collect/`. Device YAML files compose these shared pieces.
- **Build pipeline:** `make extract` → `make patch` → `make svd2rust` → `make form`

#### lpc-rs/lpc-pac (NXP-specific, Minimal Patches)
- **URL:** https://github.com/lpc-rs/lpc-pac
- **Approach:** Minimal patches — only fix what would prevent code generation or produce incorrect code
- **Structure:**
  ```
  lpc-pac/
  ├── svd/vendor/
  ├── devices/
  │   ├── lpc54606.yaml
  │   └── common_patches/
  ├── peripherals/
  └── Makefile
  ```

#### gd32-rs (Good Template for New Projects)
- **URL:** https://github.com/gd32-rust/gd32-rs
- **Notable scripts:**
  - `periphtemplate.py` — Creates an empty peripheral YAML based on an SVD file, with registers and fields ready to be populated. Auto-generates `Enabled`/`Disabled` entries for single-bit fields.
  - `matchperipherals.py` — Identifies which existing peripheral patches apply to a new SVD.

### Tool Versions to Pin

Based on real projects:
```
svdtools = 0.5.0
svd2rust = 0.37.1
form = 0.13.0
```

---

## 8. svdtools YAML Patch Format Reference

### Device-Level Directives

| Directive | Purpose |
|-----------|---------|
| `_svd` | Path to source SVD file (required) |
| `_include` | Include other YAML files (recursive) |
| `_modify` | Alter existing device/peripheral/register/field properties |
| `_add` | Add new peripherals |
| `_delete` | Remove peripherals |
| `_copy` | Copy peripherals from same or different SVD |
| `_derive` | Set up `derivedFrom` relationships |
| `_rebase` | Reorganize derivation hierarchy |
| `_clear_fields` | Clear all field definitions (use `"*"`) |

### Peripheral-Level Operations

**Modify registers/fields:**
```yaml
PERIPHERAL:
  _modify:
    OLD_NAME:
      name: NEW_NAME
      description: "Updated description"
    SOME_REGISTER:
      access: read-write
      resetValue: 0x00000000
```

**Add registers/fields/interrupts:**
```yaml
PERIPHERAL:
  _add:
    NEW_REGISTER:
      description: "A new register"
      addressOffset: 0x04
      access: read-write
      resetValue: 0x00000000
      fields:
        FIELD1:
          description: "Example field"
          bitOffset: 16
          bitWidth: 4
    _interrupts:
      MY_INTERRUPT:
        description: "An example interrupt"
        value: 100
```

**Delete registers/interrupts (wildcards supported):**
```yaml
PERIPHERAL:
  _delete:
    - UNWANTED_REG
    - REGS_*
```

**Strip prefixes/suffixes:**
```yaml
PERIPHERAL:
  _strip: PERIPH_         # "PERIPH_CR" becomes "CR"
  _strip_end: _REG        # "CONTROL_REG" becomes "CONTROL"
```

### Register-Level Operations

**Collect into arrays:**
```yaml
PERIPHERAL:
  _array:
    MY_REG*: {}            # MY_REG0, MY_REG1, ... become MY_REG[n]
```

**Group into clusters:**
```yaml
PERIPHERAL:
  _cluster:
    CHANNEL%s:
      CTRL: {}
      DATA: {}
      STATUS: {}
```

**Merge/split fields:**
```yaml
REGISTER:
  _merge:
    - "FIELD_*"            # Merge FIELD_0, FIELD_1, etc.
  _split:
    WIDE_FIELD: {}         # Split into individual bits
```

### Field-Level Operations

**Add enumerated values:**
```yaml
REGISTER:
  FIELD_NAME:
    Disabled: [0, "Feature is disabled"]
    Enabled: [1, "Feature is enabled"]
    _write_constraint: "enum"
```

**Separate read/write enumerations:**
```yaml
REGISTER:
  FIELD_NAME:
    _read:
      Low: [0, "Signal is low"]
      High: [1, "Signal is high"]
    _write:
      Clear: [0, "Clear the flag"]
      Set: [1, "Set the flag"]
```

### Pattern Matching

- `*` — zero or more characters
- `?` — exactly one character
- `[ABC]` — character alternatives
- `PERIPH1,PERIPH2` — comma-separated list
- `?~OPTIONAL_PERIPH` — suppress error if not found

### Style Conventions

- Enumerated value names use past tense: `Enabled`, `Disabled`, `Masked`
- Descriptions start with a capital letter, do not end with a period
- Boolean YAML values like `On`, `Off`, `Yes`, `No` must be quoted

---

## 9. Existing Kinetis SVD Patch Projects

**There is no dedicated Kinetis K20 SVD patch repository.** This is a gap in the ecosystem.

### What Exists

| Project | Family | Patches? | URL |
|---------|--------|----------|-----|
| exrook/mk20d7 | K20 | None | https://github.com/exrook/mk20d7 |
| dfrankland/mk20d7 | K20 | None | https://github.com/dfrankland/mk20d7 |
| stv0g/k66 | K66 | None (documents missing regs) | https://github.com/stv0g/k66 |
| lpc-rs/lpc-pac | LPC | Yes (svdtools YAML) | https://github.com/lpc-rs/lpc-pac |
| imxrt-rs/imxrt-ral | i.MX RT | Yes (custom RAL + patches) | https://github.com/imxrt-rs/imxrt-ral |
| lpc55/lpc55-pac | LPC55 | Yes | https://github.com/lpc55/lpc55-pac |
| kinetis-chip-equates | Multiple | N/A (Forth equates) | https://github.com/nimblemachines/kinetis-chip-equates |
| Masmiseim36/Kinetis | Multiple | N/A (Crossworks IDE) | https://github.com/Masmiseim36/Kinetis |

---

## 10. Comparison Tooling

### No Off-The-Shelf C Header ↔ SVD Comparison Tool Exists

However, the building blocks are available:

#### Approach A: SVD → C Header, Then Diff

1. Generate C headers from SVD using:
   - [SVDConv](https://arm-software.github.io/CMSIS_5/SVD/html/svd_SVDConv_pg.html) — ARM's official utility
   - [svdtoheaders](https://github.com/robertlipe/svdtoheaders) — Python, generates NuttX-style headers
2. Diff the generated headers against `kinetis.h`

#### Approach B: Parse Both, Compare Programmatically

1. Parse SVD with [cmsis-svd Python library](https://pypi.org/project/cmsis-svd/)
2. Parse `kinetis.h` with regex (format is very regular)
3. Compare and report discrepancies

**Regex patterns for kinetis.h:**
```python
# Register addresses:
#define REG_NAME  (*(volatile uint32_t *)0xADDRESS)

# Bitmask constants:
#define REG_FIELD  ((uint32_t)0xMASK)

# Parameterized fields:
#define REG_FIELD(n)  (((n) & MASK) << SHIFT)
```

#### Available Libraries

| Library | Purpose | URL |
|---------|---------|-----|
| cmsis-svd | Parse SVD to Python objects | https://pypi.org/project/cmsis-svd/ |
| SVDSuite | Full parse/validate/generate toolkit | https://github.com/ARMify-Project/SVDSuite |
| svd2py | SVD to Python dict | https://pypi.org/project/svd2py/ |
| pycparser | C99 parser (for struct typedefs) | https://github.com/eliben/pycparser |

---

## 11. Community Vendor SVD Quality Rankings

Based on consensus from the Rust embedded working group, forum discussions, and project maintainer experiences:

| Rank | Vendor | Quality Notes |
|------|--------|---------------|
| 1 | **Nordic (nRF)** | Generally good quality SVDs, relatively few patches needed |
| 2 | **ST (STM32)** | Many errors in vendor SVDs, but stm32-rs community has extensively patched them |
| 3 | **NXP (Kinetis/LPC/i.MX RT)** | Regular updates from vendor but systemic quality issues; no stm32-rs equivalent for Kinetis |

NXP is notable for:
- **Positive:** Actually updates SVD files regularly; has a process for keeping them equivalent in quality
- **Negative:** Tedious to obtain (bundled in SDK); per-package-variant instead of per-chip; systemic bit width/offset errors; no community patch ecosystem for Kinetis

---

## 12. Key URLs and Resources

### SVD Files and Tools

| Resource | URL |
|----------|-----|
| svdtools (patching tool) | https://github.com/rust-embedded/svdtools |
| svdtools YAML format reference | https://github.com/rust-embedded/svdtools/blob/master/README.md |
| svd2rust | https://github.com/rust-embedded/svd2rust |
| cmsis-svd Python parser | https://github.com/cmsis-svd/cmsis-svd |
| cmsis-svd-data (SVD file collection) | https://github.com/cmsis-svd/cmsis-svd-data |
| SVDSuite | https://github.com/ARMify-Project/SVDSuite |
| ARM SVDConv docs | https://arm-software.github.io/CMSIS_5/SVD/html/svd_SVDConv_pg.html |

### Exemplar Patch Projects

| Resource | URL |
|----------|-----|
| stm32-rs (gold standard) | https://github.com/stm32-rs/stm32-rs |
| lpc-rs/lpc-pac (NXP) | https://github.com/lpc-rs/lpc-pac |
| gd32-rs (good template) | https://github.com/gd32-rust/gd32-rs |
| imxrt-rs/imxrt-ral | https://github.com/imxrt-rs/imxrt-ral |

### Teensy / Kinetis References

| Resource | URL |
|----------|-----|
| kinetis.h (Teensyduino) | https://github.com/PaulStoffregen/cores/blob/master/teensy3/kinetis.h |
| PaulStoffregen/cores | https://github.com/PaulStoffregen/cores |
| K20 Reference Manual | https://www.pjrc.com/teensy/K20P64M72SF1RM.pdf |
| Kinetis SVD file collection | https://github.com/nimblemachines/kinetis-chip-equates |
| NXP MCUXpresso SDK | https://mcuxpresso.nxp.com/ |
| Keil CMSIS Packs | https://www.keil.com/dd2/Pack/ |

### Existing MK20D7 PAC Attempts

| Resource | URL |
|----------|-----|
| exrook/mk20d7 | https://github.com/exrook/mk20d7 |
| dfrankland/mk20d7 | https://github.com/dfrankland/mk20d7 |
| k66 crate (related) | https://lib.rs/crates/k66 |

### Community Discussions

| Topic | URL |
|-------|-----|
| NXP SVD defect reporting | https://github.com/imxrt-rs/imxrt-ral/issues/20 |
| LPC SVD bugs | https://github.com/lpc-rs/lpc-pac/issues/40 |
| LPC SVD "so buggy" | https://github.com/lpc-rs/lpc-pac/issues/4 |
| Kinetis IRQ offset error | https://github.com/cmsis-svd/cmsis-svd/issues/33 |
| svd2rust overloaded registers | https://github.com/rust-embedded/svd2rust/issues/16 |
| Freescale SVD updates stale | https://github.com/cmsis-svd/cmsis-svd/issues/29 |
| Embedded WG SVD management | https://github.com/rust-embedded/wg/issues/101 |
| svd2rust enormous crate | https://users.rust-lang.org/t/svd2rust-generates-an-enormous-crate/32372 |
| NXP Community LPC82x bugs | https://community.nxp.com/t5/LPC-Microcontrollers/Numerous-problems-in-LPC82x-SVD-file/m-p/770669 |
| NXP Kinetis SVD thread | https://community.nxp.com/t5/Kinetis-Microcontrollers/CMSIS-SVD-files-for-Kinetis/m-p/239202 |
