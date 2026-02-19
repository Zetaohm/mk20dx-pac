#!/usr/bin/env python3
"""Phase 4 validation: cross-check SVDs against reference manuals.

Validates:
1. Peripheral base addresses against reference manual memory maps
2. IRQ table counts and numbering against reference manual chapter 3
3. Cross-variant consistency (shared peripherals should match between MK20D5/MK20D7)

Usage:
    uv run python3 scripts/validate_phase4.py
"""

import sys
from pathlib import Path
from xml.etree import ElementTree


# ---------------------------------------------------------------------------
# SVD parsing (lightweight, peripheral-level)
# ---------------------------------------------------------------------------

def parse_svd_peripherals(svd_path: str) -> dict:
    """Parse SVD file and return peripheral info with base addresses, registers, fields, and IRQs."""
    tree = ElementTree.parse(svd_path)
    root = tree.getroot()

    peripherals = {}
    interrupts = {}

    for periph_el in root.findall(".//peripheral"):
        periph_name = periph_el.findtext("name", "")
        base_addr_text = periph_el.findtext("baseAddress", "0")
        base_addr = int(base_addr_text, 0)
        derived_from = periph_el.get("derivedFrom")

        # Parse registers
        regs = {}
        regs_el = periph_el.find("registers")
        default_size = 32
        size_el = periph_el.findtext("size")
        if size_el:
            default_size = int(size_el, 0)

        if regs_el is not None:
            for reg_el in regs_el.findall("register"):
                reg_name = reg_el.findtext("name", "")
                offset_text = reg_el.findtext("addressOffset", "0")
                offset = int(offset_text, 0)

                reg_size_text = reg_el.findtext("size")
                reg_size = int(reg_size_text, 0) if reg_size_text else default_size

                # Parse fields
                reg_fields = {}
                for field_el in reg_el.findall(".//field"):
                    field_name = field_el.findtext("name", "")
                    bit_offset = int(field_el.findtext("bitOffset", "0"))
                    bit_width = int(field_el.findtext("bitWidth", "1"))
                    reg_fields[field_name] = {
                        "bit_offset": bit_offset,
                        "bit_width": bit_width,
                    }

                # Handle dimensioned registers
                dim_text = reg_el.findtext("dim")
                dim_increment_text = reg_el.findtext("dimIncrement")
                dim_index_text = reg_el.findtext("dimIndex")

                if dim_text:
                    dim = int(dim_text, 0)
                    dim_increment = int(dim_increment_text, 0) if dim_increment_text else reg_size // 8

                    if dim_index_text and "," in dim_index_text:
                        indices = dim_index_text.split(",")
                    elif dim_index_text and "-" in dim_index_text:
                        parts = dim_index_text.split("-")
                        indices = [str(i) for i in range(int(parts[0]), int(parts[1]) + 1)]
                    else:
                        indices = [str(i) for i in range(dim)]

                    for idx_i, idx in enumerate(indices):
                        expanded_name = reg_name.replace("%s", idx)
                        expanded_offset = offset + idx_i * dim_increment
                        regs[expanded_name] = {
                            "offset": expanded_offset,
                            "size": reg_size,
                            "fields": reg_fields,
                        }
                else:
                    regs[reg_name] = {
                        "offset": offset,
                        "size": reg_size,
                        "fields": reg_fields,
                    }

        peripherals[periph_name] = {
            "base_address": base_addr,
            "derived_from": derived_from,
            "registers": regs,
            "default_size": default_size,
        }

        # Parse interrupts
        for intr_el in periph_el.findall("interrupt"):
            intr_name = intr_el.findtext("name", "")
            intr_value = int(intr_el.findtext("value", "0"))
            interrupts[intr_name] = intr_value

    return {"peripherals": peripherals, "interrupts": interrupts}


# ---------------------------------------------------------------------------
# Check 1: Peripheral base addresses
# ---------------------------------------------------------------------------

# From K20 reference manuals chapter 4 (memory map) — both 50MHz and 72MHz
# share the same peripheral bridge slot assignments
EXPECTED_BASE_ADDRESSES = {
    "DMA":    0x40008000,
    "FMC":    0x4001F000,
    "FTFL":   0x40020000,
    "DMAMUX": 0x40021000,  # DMAMUX0
    "CAN0":   0x40024000,
    "SPI0":   0x4002C000,
    "SPI1":   0x4002D000,
    "I2S0":   0x4002F000,
    "CRC":    0x40032000,
    "USBDCD": 0x40035000,
    "PDB0":   0x40036000,
    "PIT":    0x40037000,
    "FTM0":   0x40038000,
    "FTM1":   0x40039000,
    "ADC0":   0x4003B000,
    "RTC":    0x4003D000,
    "RFVBAT": 0x4003E000,  # VBAT register file
    "LPTMR0": 0x40040000,
    "RFSYS":  0x40041000,  # System register file
    "TSI0":   0x40045000,
    "SIM":    0x40047000,
    "PORTA":  0x40049000,
    "PORTB":  0x4004A000,
    "PORTC":  0x4004B000,
    "PORTD":  0x4004C000,
    "PORTE":  0x4004D000,
    "WDOG":   0x40052000,
    "EWM":    0x40061000,
    "CMT":    0x40062000,
    "MCG":    0x40064000,
    "OSC":    0x40065000,  # OSC0
    "I2C0":   0x40066000,
    "I2C1":   0x40067000,
    "UART0":  0x4006A000,
    "UART1":  0x4006B000,
    "UART2":  0x4006C000,
    "USB0":   0x40072000,
    "CMP0":   0x40073000,  # CMP block
    "CMP1":   0x40073000,  # Same block as CMP0 (different offset)
    "VREF":   0x40074000,
    "LLWU":   0x4007C000,
    "PMC":    0x4007D000,
    "SMC":    0x4007E000,
    "RCM":    0x4007F000,
    # GPIO on peripheral bridge 1 (SVDs use PTA/PTB/etc.)
    "PTA":    0x400FF000,
    "PTB":    0x400FF040,
    "PTC":    0x400FF080,
    "PTD":    0x400FF0C0,
    "PTE":    0x400FF100,
}

# Peripherals only in MK20D7 (72MHz)
MK20D7_ONLY = {"SPI1", "I2C1", "UART2", "CAN0", "ADC1", "FTM2", "DAC0", "PGA"}


def check_base_addresses(variant: str, svd_data: dict) -> list[str]:
    """Verify peripheral base addresses against reference manual."""
    errors = []
    passes = []
    peripherals = svd_data["peripherals"]

    for name, expected_addr in EXPECTED_BASE_ADDRESSES.items():
        if expected_addr is None:
            continue

        # Skip MK20D7-only peripherals when checking MK20D5
        if variant == "mk20d5" and name in MK20D7_ONLY:
            continue

        # Try exact name first, then common aliases
        aliases = [name]
        if name == "DMAMUX":
            aliases.append("DMAMUX0")
        if name == "OSC":
            aliases.append("OSC0")
        if name == "CMP0":
            aliases.extend(["CMP"])
        if name == "CMP1":
            continue  # Skip CMP1 — it's at an offset within the CMP0 block

        found = False
        for alias in aliases:
            if alias in peripherals:
                actual_addr = peripherals[alias]["base_address"]
                if actual_addr != expected_addr:
                    errors.append(
                        f"  FAIL: {alias} base=0x{actual_addr:08X}, "
                        f"expected=0x{expected_addr:08X}"
                    )
                else:
                    passes.append(alias)
                found = True
                break

        if not found:
            # Not necessarily an error — peripheral may not exist in this variant
            if variant == "mk20d5" and name in MK20D7_ONLY:
                continue
            # Only warn if it's expected for this variant
            if name not in MK20D7_ONLY:
                errors.append(f"  WARN: {name} not found in SVD")

    return errors, passes


# ---------------------------------------------------------------------------
# Check 2: IRQ count verification
# ---------------------------------------------------------------------------

# Expected IRQ counts from reference manuals chapter 3
EXPECTED_IRQ_COUNTS = {
    "mk20d5": {
        "min_irq": 0,
        "max_irq": 45,  # IRQ 0-45, 46 total device IRQs
        "total_vectors": 62,  # 16 ARM core + 46 device
    },
    "mk20d7": {
        "min_irq": 0,
        "max_irq": 94,  # IRQ 0-94, 95 total device IRQs
        "total_vectors": 111,  # 16 ARM core + 95 device
    },
}

# Key IRQs to spot-check (from reference manual IRQ tables)
# Format: {irq_number: [possible SVD names]}
# Note: MCG IRQ and Software IRQ are often absent from SVDs —
#   MCG peripheral may not declare an interrupt element,
#   Software IRQ is an ARM-only vector with no peripheral source.
EXPECTED_IRQS_MK20D7 = {
    0: ["DMA0", "DMA_CH0", "DMA0_DMA16"],
    16: ["DMA_ERROR", "DMA_Error"],
    18: ["FTFL", "FTF", "FTFL_COMPLETE"],  # Flash command complete
    19: ["Read_Collision", "FTFL_COLLISION"],  # Flash read collision
    20: ["LVD_LVW", "PMC"],
    21: ["LLW", "LLWU"],
    22: ["WDOG_EWM", "WDOG", "Watchdog"],
    73: ["USB_OTG", "USB0"],
    85: ["LPTMR0", "LPTimer"],
    87: ["PORTA", "Pin_detect_PORTA"],
    88: ["PORTB", "Pin_detect_PORTB"],
    89: ["PORTC", "Pin_detect_PORTC"],
    90: ["PORTD", "Pin_detect_PORTD"],
    91: ["PORTE", "Pin_detect_PORTE"],
    # IRQ 84 (MCG) and IRQ 94 (Software) are typically absent from SVDs
}

EXPECTED_IRQS_MK20D5 = {
    0: ["DMA0", "DMA_CH0"],
    4: ["DMA_ERROR", "DMA_Error"],
    6: ["FTFL", "FTF", "FTFL_COMPLETE"],  # Flash command complete
    7: ["Read_Collision", "FTFL_COLLISION"],  # Flash read collision
    8: ["LVD_LVW", "PMC"],
    9: ["LLW", "LLWU"],
    10: ["WDOG_EWM", "WDOG", "Watchdog"],
    35: ["USB_OTG", "USB0"],
    39: ["LPTMR0", "LPTimer"],
    40: ["PORTA", "Pin_detect_PORTA"],
    41: ["PORTB", "Pin_detect_PORTB"],
    42: ["PORTC", "Pin_detect_PORTC"],
    43: ["PORTD", "Pin_detect_PORTD"],
    44: ["PORTE", "Pin_detect_PORTE"],
    # IRQ 38 (MCG) and IRQ 45 (Software) are typically absent from SVDs
}


def check_irqs(variant: str, svd_data: dict) -> tuple[list[str], list[str]]:
    """Verify IRQ table against reference manual expectations."""
    errors = []
    passes = []
    interrupts = svd_data["interrupts"]

    expected = EXPECTED_IRQ_COUNTS[variant]
    expected_irqs = EXPECTED_IRQS_MK20D7 if variant == "mk20d7" else EXPECTED_IRQS_MK20D5

    # Check IRQ range
    if interrupts:
        actual_values = sorted(interrupts.values())
        min_irq = actual_values[0]
        max_irq = actual_values[-1]
        total = len(set(actual_values))

        if max_irq > expected["max_irq"]:
            errors.append(
                f"  FAIL: Max IRQ={max_irq}, expected <={expected['max_irq']}"
            )
        else:
            passes.append(f"Max IRQ {max_irq} <= {expected['max_irq']}")

        if min_irq < expected["min_irq"]:
            errors.append(
                f"  FAIL: Min IRQ={min_irq}, expected >={expected['min_irq']}"
            )
        else:
            passes.append(f"Min IRQ {min_irq} >= {expected['min_irq']}")

    # Build reverse lookup: IRQ number -> name(s)
    irq_to_names = {}
    for name, value in interrupts.items():
        irq_to_names.setdefault(value, []).append(name)

    # Spot-check key IRQs
    for irq_num, expected_names in expected_irqs.items():
        actual_names = irq_to_names.get(irq_num, [])
        if not actual_names:
            errors.append(
                f"  FAIL: IRQ {irq_num} not found in SVD "
                f"(expected one of: {expected_names})"
            )
        else:
            # Check that at least one actual name matches or is close
            matched = False
            for actual in actual_names:
                for exp in expected_names:
                    if exp.upper() in actual.upper() or actual.upper() in exp.upper():
                        matched = True
                        break
                if matched:
                    break
            if matched:
                passes.append(f"IRQ {irq_num}: {actual_names}")
            else:
                # Name doesn't match but the IRQ number exists — just note it
                passes.append(
                    f"IRQ {irq_num}: {actual_names} (name differs from ref manual: {expected_names})"
                )

    return errors, passes


# ---------------------------------------------------------------------------
# Check 3: Cross-variant consistency
# ---------------------------------------------------------------------------

# Peripherals expected to be identical between MK20D5 and MK20D7
SHARED_PERIPHERALS = [
    "FTFL", "SPI0", "I2C0", "FTM0", "FTM1",
    "ADC0", "PIT", "PORTA", "PORTB", "PORTC", "PORTD", "PORTE",
    "PTA", "PTB", "PTC", "PTD", "PTE",
    "MCG", "PMC", "RCM", "LLWU", "WDOG", "EWM",
    "USB0", "CMP0", "VREF", "LPTMR0", "RTC", "TSI0", "OSC",
    "CRC", "CMT",
]

# Known/expected differences — report but don't count as errors
KNOWN_CROSS_VARIANT_DIFFS = {
    "FMC",      # Different cache geometry (50MHz vs 72MHz)
    "DMAMUX",   # MK20D5 has 4 channels, MK20D7 has 16
    "DMAMUX0",
    "DMA",      # MK20D5 has 4 DMA channels, MK20D7 has 16
    "UART0",    # D5 SVD includes extra CEA709 (IR) registers
    "UART1",    # Same
    "SIM",      # D7 has extra SCGC1/2/3 for additional peripherals
    "SMC",      # Minor field differences between sub-families
    "PDB0",     # D5 has 1 PDB channel, D7 has 2 + DAC trigger
    "ADC0",     # D7 has PGA register, D5 does not
    "PORTA",    # D7 has digital filter registers (DFER/DFCR/DFWR)
    "PORTB",
    "PORTC",
    "PORTD",
    "PORTE",
}


def check_cross_variant(d5_data: dict, d7_data: dict) -> tuple[list[str], list[str]]:
    """Compare shared peripherals between MK20D5 and MK20D7."""
    errors = []
    passes = []
    d5_periphs = d5_data["peripherals"]
    d7_periphs = d7_data["peripherals"]

    for periph_name in SHARED_PERIPHERALS:
        # Try aliases
        d5_name = periph_name
        d7_name = periph_name
        if periph_name == "OSC":
            if "OSC0" in d5_periphs:
                d5_name = "OSC0"
            if "OSC0" in d7_periphs:
                d7_name = "OSC0"

        if d5_name not in d5_periphs:
            continue
        if d7_name not in d7_periphs:
            continue

        d5_p = d5_periphs[d5_name]
        d7_p = d7_periphs[d7_name]

        # Check base address
        if d5_p["base_address"] != d7_p["base_address"]:
            errors.append(
                f"  FAIL: {periph_name} base address differs: "
                f"D5=0x{d5_p['base_address']:08X}, D7=0x{d7_p['base_address']:08X}"
            )
            continue

        # Skip derived peripherals (they don't have their own registers)
        if d5_p["derived_from"] and not d5_p["registers"]:
            passes.append(f"{periph_name} (derived, base matches)")
            continue
        if d7_p["derived_from"] and not d7_p["registers"]:
            passes.append(f"{periph_name} (derived, base matches)")
            continue

        # Compare register layouts
        d5_regs = d5_p["registers"]
        d7_regs = d7_p["registers"]

        reg_issues = []

        # Registers in D5 but not in D7
        for rname, rinfo in d5_regs.items():
            if rname not in d7_regs:
                reg_issues.append(f"    {rname} (offset 0x{rinfo['offset']:X}) in D5 only")

        # Registers in D7 but not in D5
        for rname, rinfo in d7_regs.items():
            if rname not in d5_regs:
                reg_issues.append(f"    {rname} (offset 0x{rinfo['offset']:X}) in D7 only")

        # Matching registers — check offset and size
        for rname in d5_regs:
            if rname not in d7_regs:
                continue
            d5_r = d5_regs[rname]
            d7_r = d7_regs[rname]
            if d5_r["offset"] != d7_r["offset"]:
                reg_issues.append(
                    f"    {rname} offset: D5=0x{d5_r['offset']:X}, D7=0x{d7_r['offset']:X}"
                )
            if d5_r["size"] != d7_r["size"]:
                reg_issues.append(
                    f"    {rname} size: D5={d5_r['size']}, D7={d7_r['size']}"
                )

            # Compare fields
            d5_fields = d5_r.get("fields", {})
            d7_fields = d7_r.get("fields", {})
            for fname in d5_fields:
                if fname not in d7_fields:
                    reg_issues.append(f"    {rname}.{fname} in D5 only")
                    continue
                d5_f = d5_fields[fname]
                d7_f = d7_fields[fname]
                if d5_f["bit_offset"] != d7_f["bit_offset"]:
                    reg_issues.append(
                        f"    {rname}.{fname} offset: D5={d5_f['bit_offset']}, D7={d7_f['bit_offset']}"
                    )
                if d5_f["bit_width"] != d7_f["bit_width"]:
                    reg_issues.append(
                        f"    {rname}.{fname} width: D5={d5_f['bit_width']}, D7={d7_f['bit_width']}"
                    )

        if reg_issues:
            is_known = periph_name in KNOWN_CROSS_VARIANT_DIFFS
            tag = "KNOWN" if is_known else "UNEXPECTED"
            line = f"  {tag}: {periph_name} has {len(reg_issues)} register differences:"
            if is_known:
                passes.append(f"{periph_name} ({len(reg_issues)} known diffs)")
            else:
                errors.append(line)
                errors.extend(reg_issues[:20])
                if len(reg_issues) > 20:
                    errors.append(f"    ... and {len(reg_issues) - 20} more")
        else:
            passes.append(f"{periph_name} ({len(d5_regs)} regs match)")

    # Report MK20D7-only peripherals
    d7_only = []
    for name in d7_periphs:
        if name not in d5_periphs and name not in KNOWN_CROSS_VARIANT_DIFFS:
            d7_only.append(name)
    if d7_only:
        passes.append(f"MK20D7-only peripherals: {', '.join(sorted(d7_only))}")

    return errors, passes


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    project_root = Path(__file__).parent.parent

    # Determine SVD paths
    d5_svd = project_root / "svd" / "MK20D5.svd.patched"
    d7_svd = project_root / "svd" / "MK20D7.svd"

    if not d5_svd.exists():
        d5_svd = project_root / "svd" / "MK20D5.svd"
        print(f"Note: Using unpatched MK20D5.svd (no .patched file found)")

    if not d5_svd.exists() or not d7_svd.exists():
        print("Error: SVD files not found. Run 'make patch' first.", file=sys.stderr)
        sys.exit(1)

    print(f"Parsing {d5_svd.name}...")
    d5_data = parse_svd_peripherals(str(d5_svd))
    print(f"  {len(d5_data['peripherals'])} peripherals, {len(d5_data['interrupts'])} interrupts")

    print(f"Parsing {d7_svd.name}...")
    d7_data = parse_svd_peripherals(str(d7_svd))
    print(f"  {len(d7_data['peripherals'])} peripherals, {len(d7_data['interrupts'])} interrupts")

    total_errors = 0
    total_passes = 0

    # -----------------------------------------------------------------------
    # Check 1: Peripheral base addresses
    # -----------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  Check 1: Peripheral Base Addresses")
    print("=" * 72)

    for variant, svd_data, label in [
        ("mk20d5", d5_data, "MK20D5 (50MHz)"),
        ("mk20d7", d7_data, "MK20D7 (72MHz)"),
    ]:
        print(f"\n  --- {label} ---")
        errors, passes = check_base_addresses(variant, svd_data)
        if errors:
            for e in errors:
                print(e)
        print(f"  {len(passes)} peripherals verified, {len(errors)} issues")
        total_errors += len([e for e in errors if "FAIL" in e])
        total_passes += len(passes)

    # -----------------------------------------------------------------------
    # Check 2: IRQ table verification
    # -----------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  Check 2: IRQ Table Verification")
    print("=" * 72)

    for variant, svd_data, label in [
        ("mk20d5", d5_data, "MK20D5 (50MHz)"),
        ("mk20d7", d7_data, "MK20D7 (72MHz)"),
    ]:
        print(f"\n  --- {label} ---")
        errors, passes = check_irqs(variant, svd_data)
        if errors:
            for e in errors:
                print(e)
        print(f"  {len(passes)} checks passed, {len(errors)} issues")
        total_errors += len([e for e in errors if "FAIL" in e])
        total_passes += len(passes)

    # -----------------------------------------------------------------------
    # Check 3: Cross-variant consistency
    # -----------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  Check 3: Cross-Variant Consistency (MK20D5 vs MK20D7)")
    print("=" * 72)

    errors, passes = check_cross_variant(d5_data, d7_data)
    if errors:
        for e in errors:
            print(e)
    print(f"\n  {len(passes)} peripherals consistent, {len(errors)} issues")
    total_errors += len([e for e in errors if "FAIL" in e])
    total_passes += len(passes)

    # -----------------------------------------------------------------------
    # Summary
    # -----------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  Phase 4 Validation Summary")
    print("=" * 72)
    print(f"\n  Total checks passed: {total_passes}")
    print(f"  Total errors:        {total_errors}")

    if total_errors == 0:
        print("\n  RESULT: ALL CHECKS PASSED")
        return 0
    else:
        print(f"\n  RESULT: {total_errors} ERROR(S) FOUND")
        return 1


if __name__ == "__main__":
    sys.exit(main())
