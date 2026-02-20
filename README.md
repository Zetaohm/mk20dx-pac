# mk20dx-pac

Patched Rust Peripheral Access Crates (PACs) for the NXP Kinetis MK20DX128 and MK20DX256 microcontrollers, as used on the [Teensy 3.0](https://www.pjrc.com/store/teensy3.html) and [Teensy 3.1/3.2](https://www.pjrc.com/store/teensy31.html) development boards.

Generated with [svd2rust](https://github.com/rust-embedded/svd2rust) from NXP vendor SVD files, with correctness and ergonomics patches applied via [svdtools](https://github.com/rust-embedded/svdtools).

## Target Chips

| Crate | Chip | Board | Core | Flash | RAM | DMA Ch |
|-------|------|-------|------|-------|-----|--------|
| `mk20d5` | MK20DX128VLH5 | Teensy 3.0 | Cortex-M4 @ 50 MHz | 128K | 16K | 4 |
| `mk20d7` | MK20DX256VLH7 | Teensy 3.1/3.2 | Cortex-M4 @ 72 MHz | 256K | 64K | 16 |

## Usage

Add the appropriate crate to your `Cargo.toml`:

```toml
[dependencies]
mk20d7 = { git = "https://github.com/zetaohm/mk20dx-pac", features = ["rt"] }
```

Then access peripheral registers:

```rust
#![no_std]
#![no_main]

use mk20d7;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = mk20d7::Peripherals::take().unwrap();

    // Enable clock gate for GPIO port C
    peripherals.SIM.scgc5.modify(|_, w| w.portc().enabled());

    // Configure pin C5 as GPIO output
    peripherals.PORTC.pcr[5].write(|w| w.mux().gpio());
    peripherals.PTC.pddr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });

    loop {}
}
```

## SVD Patches Applied

The vendor SVD files contain known bugs common across NXP Kinetis parts. All patches are cross-referenced against the [Teensyduino `kinetis.h`](https://github.com/PaulStoffregen/cores/blob/master/teensy3/kinetis.h) header and the K20 Sub-Family Reference Manuals.

### Correctness Patches (MK20D5 only)

| Bug | Patch | Impact |
|-----|-------|--------|
| SIM_SOPT5 UART TX source field widths (1-bit, should be 2-bit) | `patches/mk20d5/sim/sopt5_uart_txsrc.yaml` | Values 2-3 silently truncated |
| FMC cache register addresses (used 72MHz layout on 50MHz part) | `patches/mk20d5/fmc/cache_addresses.yaml` | All FMC cache accesses past way 0 hit wrong addresses |

The MK20D7 SVD required no correctness patches.

### Ergonomics Patches (both variants)

| Improvement | Patch Files |
|------------|------------|
| DMAMUX source channel enums (43/50 named DMA request sources) | `patches/*/dmamux/source_enums.yaml` |
| DMA TCD clustering (per-channel struct access) | `patches/*/dma/tcd_cluster.yaml` |
| Semantic enum names for PORT MUX, FTM, ADC, MCG, SIM, DMA ATTR | `patches/common/` (6 files) + `patches/mk20d7/` (2 files) |

For full details, see [docs/RESEARCH_FINDINGS.md](docs/RESEARCH_FINDINGS.md) and [docs/STATUS.md](docs/STATUS.md).

## Building from Source

### Prerequisites

```bash
cargo install svdtools svd2rust form
```

### Build

```bash
make all    # patch -> svd2rust -> fmt -> cargo check (both variants)
```

Individual steps:

```bash
make patch          # Apply svdtools YAML patches to SVD files
make svd2rust       # Generate Rust crate code from patched SVDs
make check          # cargo check both generated crates
make patch-mk20d5   # Patch only MK20D5
make check-mk20d7   # Check only MK20D7
```

## Project Structure

```
mk20dx-pac/
├── svd/                # Vendor SVD files (unmodified originals)
├── devices/            # Top-level svdtools YAML (one per variant)
├── patches/
│   ├── common/         # Shared patches (both variants)
│   ├── mk20d5/         # MK20DX128-specific patches
│   └── mk20d7/         # MK20DX256-specific patches
├── mk20d5/             # Generated PAC crate (Teensy 3.0)
├── mk20d7/             # Generated PAC crate (Teensy 3.1/3.2)
├── reference/          # kinetis.h header + K20 reference manuals
├── scripts/            # Comparison and validation tools
├── docs/               # Project documentation
└── Makefile
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Credits

- SVD files from [cmsis-svd-data](https://github.com/cmsis-svd/cmsis-svd-data) (NXP/Freescale)
- Cross-referenced against [Teensyduino `kinetis.h`](https://github.com/PaulStoffregen/cores/blob/master/teensy3/kinetis.h) by Paul Stoffregen
- Built with the [svd2rust](https://github.com/rust-embedded/svd2rust) ecosystem
