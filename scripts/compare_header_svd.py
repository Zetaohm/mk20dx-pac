#!/usr/bin/env python3
"""Compare kinetis.h register definitions against SVD file contents.

Cross-references Teensyduino's kinetis.h (ground truth) with NXP vendor SVD
files to identify discrepancies that need patching.

Usage:
    uv run python3 scripts/compare_header_svd.py --variant mk20d7 \
        --svd svd/MK20D7.svd --header reference/kinetis.h
"""

import argparse
import json
import re
import sys
from dataclasses import dataclass, field, asdict
from enum import Enum
from pathlib import Path
from typing import Optional
from xml.etree import ElementTree


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------

@dataclass
class Register:
    name: str
    address: int
    size: int  # bits: 8, 16, or 32
    peripheral: str = ""

@dataclass
class BitField:
    name: str
    mask: int
    shift: int
    width: int

@dataclass
class ParamField:
    name: str
    mask: int
    shift: int
    width: int

@dataclass
class IRQ:
    name: str
    number: int

@dataclass
class StructRegister:
    name: str
    offset: int
    size: int  # bits

@dataclass
class StructDef:
    type_name: str
    base_address: int
    instance_name: str
    registers: list


class Mismatch(str, Enum):
    ADDRESS_MISMATCH = "ADDRESS_MISMATCH"
    SIZE_MISMATCH = "SIZE_MISMATCH"
    FIELD_WIDTH_MISMATCH = "FIELD_WIDTH_MISMATCH"
    FIELD_OFFSET_MISMATCH = "FIELD_OFFSET_MISMATCH"
    MISSING_IN_SVD = "MISSING_IN_SVD"
    MISSING_IN_HEADER = "MISSING_IN_HEADER"
    IRQ_MISMATCH = "IRQ_MISMATCH"


@dataclass
class Discrepancy:
    category: str
    peripheral: str
    register: str
    field: str
    header_value: Optional[str]
    svd_value: Optional[str]
    detail: str


# ---------------------------------------------------------------------------
# Preprocessor-aware line filter
# ---------------------------------------------------------------------------

# Maps variant names to the set of preprocessor symbols that should be defined.
# Includes HAS_KINETIS_* feature flags from the chip-specific sections of kinetis.h.
VARIANT_SYMBOLS = {
    "mk20d5": {
        "__MK20DX128__", "KINETISK",
        "HAS_KINETISK_UART0", "HAS_KINETISK_UART0_FIFO",
        "HAS_KINETISK_UART1", "HAS_KINETISK_UART2",
        "HAS_KINETIS_I2C0",
        "HAS_KINETIS_LLWU_16CH",
        "HAS_KINETIS_ADC0",
        "HAS_KINETIS_TSI",
        "HAS_KINETIS_FLASH_FTFL",
    },
    "mk20d7": {
        "__MK20DX256__", "KINETISK",
        "HAS_KINETISK_UART0", "HAS_KINETISK_UART0_FIFO",
        "HAS_KINETISK_UART1", "HAS_KINETISK_UART1_FIFO",
        "HAS_KINETISK_UART2",
        "HAS_KINETIS_I2C0", "HAS_KINETIS_I2C1",
        "HAS_KINETIS_LLWU_16CH",
        "HAS_KINETIS_ADC0", "HAS_KINETIS_ADC1",
        "HAS_KINETIS_TSI",
        "HAS_KINETIS_FLASH_FTFL",
    },
}

# Chip-specific symbols that, when encountered, mean "not our chip"
OTHER_CHIP_SYMBOLS = {
    "__MK20DX128__", "__MK20DX256__", "__MKL26Z64__",
    "__MK64FX512__", "__MK66FX1M0__",
}


def evaluate_condition(expr: str, defined_symbols: set) -> Optional[bool]:
    """Evaluate a simple preprocessor condition.

    Handles: defined(SYM), !defined(SYM), and combinations with && and ||.
    Returns None if the expression is too complex to evaluate.
    """
    expr = expr.strip()

    # Single defined(SYM)
    m = re.fullmatch(r"defined\(\s*(\w+)\s*\)", expr)
    if m:
        return m.group(1) in defined_symbols

    # !defined(SYM)
    m = re.fullmatch(r"!\s*defined\(\s*(\w+)\s*\)", expr)
    if m:
        return m.group(1) not in defined_symbols

    # SYM alone (used in #ifdef)
    m = re.fullmatch(r"(\w+)", expr)
    if m:
        return m.group(1) in defined_symbols

    # A && B
    if "&&" in expr and "||" not in expr:
        parts = expr.split("&&")
        results = [evaluate_condition(p.strip(), defined_symbols) for p in parts]
        if None in results:
            return None
        return all(results)

    # A || B
    if "||" in expr and "&&" not in expr:
        parts = expr.split("||")
        results = [evaluate_condition(p.strip(), defined_symbols) for p in parts]
        if None in results:
            return None
        return any(results)

    return None  # Too complex


def filter_lines_for_variant(lines: list[str], variant: str) -> list[str]:
    """Filter kinetis.h lines through a minimal preprocessor for the given variant.

    Tracks #if/#ifdef/#elif/#else/#endif nesting and only emits lines that
    are "active" for the specified chip variant.
    """
    defined_symbols = VARIANT_SYMBOLS.get(variant, set())
    output = []

    # Stack of (branch_taken, currently_active) tuples
    # branch_taken: has any branch in this #if group been taken?
    # currently_active: is the current branch active?
    stack = []

    def is_active() -> bool:
        """Are we in an active code section?"""
        if not stack:
            return True
        return all(active for _, active in stack)

    for line in lines:
        stripped = line.strip()

        # #if defined(X) or #if !defined(X) etc.
        m = re.match(r"^#\s*if\s+(.+)", stripped)
        if m and not stripped.startswith("#ifdef") and not stripped.startswith("#ifndef"):
            cond = evaluate_condition(m.group(1), defined_symbols)
            if cond is None:
                # Can't evaluate — assume active (conservative)
                stack.append((True, is_active()))
            else:
                stack.append((cond, cond and is_active()))
            continue

        # #ifdef SYM
        m = re.match(r"^#\s*ifdef\s+(\w+)", stripped)
        if m:
            sym = m.group(1)
            active = sym in defined_symbols
            stack.append((active, active and is_active()))
            continue

        # #ifndef SYM
        m = re.match(r"^#\s*ifndef\s+(\w+)", stripped)
        if m:
            sym = m.group(1)
            active = sym not in defined_symbols
            stack.append((active, active and is_active()))
            continue

        # #elif
        m = re.match(r"^#\s*elif\s+(.+)", stripped)
        if m and stack:
            branch_taken, _ = stack[-1]
            if branch_taken:
                # A previous branch was taken, so this one is inactive
                stack[-1] = (True, False)
            else:
                cond = evaluate_condition(m.group(1), defined_symbols)
                if cond is None:
                    stack[-1] = (False, False)  # Conservative: skip unknown
                elif cond:
                    # Check parent is active
                    parent_active = all(active for _, active in stack[:-1]) if len(stack) > 1 else True
                    stack[-1] = (True, parent_active)
                else:
                    stack[-1] = (False, False)
            continue

        # #else
        if re.match(r"^#\s*else\b", stripped):
            if stack:
                branch_taken, _ = stack[-1]
                if branch_taken:
                    stack[-1] = (True, False)
                else:
                    parent_active = all(active for _, active in stack[:-1]) if len(stack) > 1 else True
                    stack[-1] = (True, parent_active)
            continue

        # #endif
        if re.match(r"^#\s*endif\b", stripped):
            if stack:
                stack.pop()
            continue

        # Regular line — emit if active
        if is_active():
            output.append(line)

    return output


# ---------------------------------------------------------------------------
# kinetis.h parsers
# ---------------------------------------------------------------------------

# Register address: #define NAME (*(volatile uint32_t *)0xADDRESS)
RE_REG_ADDR = re.compile(
    r"^#define\s+(\w+)\s+\(\*\(volatile\s+(uint8_t|uint16_t|uint32_t)\s*\*\)\s*(0x[0-9A-Fa-f]+)\)"
)

# Bitmask constant: #define NAME ((uint32_t)0xMASK)
RE_BITMASK_CAST = re.compile(
    r"^#define\s+(\w+)\s+\(\(uint32_t\)\s*(0x[0-9A-Fa-f]+)\)"
)

# Simple hex bitmask: #define NAME 0xMASK (no cast)
RE_BITMASK_HEX = re.compile(
    r"^#define\s+(\w+)\s+(0x[0-9A-Fa-f]+)\s*(?://|$)"
)

# Parameterized field: match the core pattern ((n) & MASK) << SHIFT anywhere
# Handles all wrapper variations:
#   ((uint32_t)(((n) & 15) << 16))
#   (uint32_t)(((n) & 3) << 28)
#   (((n) & 3) << 6)
RE_PARAM_FIELD = re.compile(
    r"^#define\s+(\w+)\s*\(\s*n\s*\)\s+.*"
    r"\(\s*\(\s*n\s*\)\s*&\s*(0x[0-9A-Fa-f]+|\d+)\s*\)\s*<<\s*(\d+)"
)

# IRQ enum entry: IRQ_NAME = N,
RE_IRQ = re.compile(r"^\s*(IRQ_\w+)\s*=\s*(\d+)")

# Struct typedef start
RE_STRUCT_START = re.compile(r"^typedef\s+struct")

# Struct member: volatile uintN_t NAME;
RE_STRUCT_MEMBER = re.compile(
    r"^\s*volatile\s+(uint8_t|uint16_t|uint32_t)\s+(\w+)\s*;"
)

# Union members inside struct (e.g., UART WP7816T0/WP7816T1)
RE_UNION_START = re.compile(r"^\s*union\s*\{")
RE_UNION_MEMBER = re.compile(
    r"volatile\s+(uint8_t|uint16_t|uint32_t)\s+(\w+)\s*;"
)

# Struct typedef end: } TYPE_NAME;
RE_STRUCT_END = re.compile(r"^\s*}\s*(\w+)\s*;")

# Struct instance: #define INSTANCE (*(TYPE *)0xBASE)
RE_STRUCT_INSTANCE = re.compile(
    r"^#define\s+(\w+)\s+\(\*\(\s*(\w+)\s*\*\)\s*(0x[0-9A-Fa-f]+)\)"
)
# Struct instance with named address: #define INSTANCE (*(TYPE *)NAME_ADDR)
RE_STRUCT_INSTANCE_NAMED = re.compile(
    r"^#define\s+(\w+)\s+\(\*\(\s*(\w+)\s*\*\)\s*(\w+)\)"
)
# Named address constant: #define NAME_ADDR (0xBASE) or #define NAME_ADDRESS (0xBASE)
RE_NAMED_ADDR = re.compile(
    r"^#define\s+(\w+(?:_ADDR(?:ESS)?))\s+\(?(0x[0-9A-Fa-f]+)\)?"
)

SIZE_MAP = {"uint8_t": 8, "uint16_t": 16, "uint32_t": 32}


def mask_to_width(mask: int) -> int:
    """Compute the number of set bits in a contiguous mask."""
    if mask == 0:
        return 0
    # Count contiguous 1-bits from LSB of the mask value
    return bin(mask).count("1")


def mask_to_shift(mask: int) -> int:
    """Compute the bit position of the lowest set bit."""
    if mask == 0:
        return 0
    shift = 0
    while (mask >> shift) & 1 == 0:
        shift += 1
    return shift


def parse_int(s: str) -> int:
    """Parse an integer from hex or decimal string."""
    s = s.strip()
    if s.startswith("0x") or s.startswith("0X"):
        return int(s, 16)
    return int(s)


def infer_peripheral(name: str) -> str:
    """Infer peripheral name from a register/field name.

    E.g., 'PORTA_PCR0' -> 'PORTA', 'SIM_SCGC4' -> 'SIM',
    'UART0_BDH' -> 'UART0', 'DMA_TCD0_SADDR' -> 'DMA'.
    """
    # Special cases for multi-word peripheral prefixes
    for prefix in ("FTFL_FLASH_CONFIG", "FTFL_"):
        if name.startswith(prefix):
            return "FTFL"

    parts = name.split("_")
    if len(parts) >= 2:
        return parts[0]
    return name


def parse_header(lines: list[str]) -> dict:
    """Parse filtered kinetis.h lines into structured data."""
    registers = {}
    bitfields = {}
    param_fields = {}
    irqs = {}
    structs = {}
    named_addrs = {}  # NAME_ADDR -> address (for struct instances with named addresses)

    # State for struct parsing
    in_struct = False
    struct_members = []
    struct_offset = 0
    in_union = False
    union_max_size = 0

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        i += 1

        # Skip comments and blank lines
        if not stripped or stripped.startswith("//") or stripped.startswith("/*"):
            continue

        # --- Struct parsing ---
        if RE_STRUCT_START.match(stripped):
            in_struct = True
            struct_members = []
            struct_offset = 0
            in_union = False
            continue

        if in_struct:
            # Union start
            if RE_UNION_START.match(stripped):
                in_union = True
                union_max_size = 0
                # Parse union members on same line if present
                for um in RE_UNION_MEMBER.finditer(stripped):
                    size = SIZE_MAP[um.group(1)]
                    name = um.group(2)
                    struct_members.append(StructRegister(
                        name=name, offset=struct_offset, size=size
                    ))
                    union_max_size = max(union_max_size, size // 8)
                # Check if union closes on same line
                if "};" in stripped or "}" in stripped:
                    in_union = False
                    struct_offset += union_max_size
                continue

            if in_union:
                for um in RE_UNION_MEMBER.finditer(stripped):
                    size = SIZE_MAP[um.group(1)]
                    name = um.group(2)
                    struct_members.append(StructRegister(
                        name=name, offset=struct_offset, size=size
                    ))
                    union_max_size = max(union_max_size, size // 8)
                if "};" in stripped or ("}" in stripped and "union" not in stripped):
                    in_union = False
                    struct_offset += union_max_size
                continue

            # Regular struct member
            mm = RE_STRUCT_MEMBER.match(stripped)
            if mm:
                size = SIZE_MAP[mm.group(1)]
                name = mm.group(2)
                if name.startswith("unused") or name.startswith("pad"):
                    struct_offset += size // 8
                else:
                    struct_members.append(StructRegister(
                        name=name, offset=struct_offset, size=size
                    ))
                    struct_offset += size // 8
                continue

            # Struct end
            me = RE_STRUCT_END.match(stripped)
            if me:
                type_name = me.group(1)
                structs[type_name] = {
                    "type_name": type_name,
                    "members": struct_members,
                    "total_size": struct_offset,
                }
                in_struct = False
                continue

            continue

        # --- Named address constants (for struct instances) ---
        ma = RE_NAMED_ADDR.match(stripped)
        if ma:
            named_addrs[ma.group(1)] = int(ma.group(2), 16)
            # Don't continue — this line might also match other patterns

        # --- Struct instance (direct hex address) ---
        mi = RE_STRUCT_INSTANCE.match(stripped)
        if mi:
            inst_name = mi.group(1)
            type_name = mi.group(2)
            base_addr = int(mi.group(3), 16)
            if type_name in structs:
                st = structs[type_name]
                if "instances" not in st:
                    st["instances"] = []
                st["instances"].append({"name": inst_name, "base_address": base_addr})
            continue

        # --- Struct instance (named address) ---
        mi = RE_STRUCT_INSTANCE_NAMED.match(stripped)
        if mi:
            inst_name = mi.group(1)
            type_name = mi.group(2)
            addr_name = mi.group(3)
            if type_name in structs and addr_name in named_addrs:
                st = structs[type_name]
                if "instances" not in st:
                    st["instances"] = []
                st["instances"].append({"name": inst_name, "base_address": named_addrs[addr_name]})
            continue

        # --- IRQ enum entries ---
        mi = RE_IRQ.match(stripped)
        if mi:
            irqs[mi.group(1)] = IRQ(name=mi.group(1), number=int(mi.group(2)))
            continue

        # --- Register addresses ---
        mr = RE_REG_ADDR.match(stripped)
        if mr:
            name = mr.group(1)
            size = SIZE_MAP[mr.group(2)]
            addr = int(mr.group(3), 16)
            registers[name] = Register(
                name=name, address=addr, size=size,
                peripheral=infer_peripheral(name)
            )
            continue

        # --- Bitmask constants (with cast) ---
        mb = RE_BITMASK_CAST.match(stripped)
        if mb:
            name = mb.group(1)
            mask = int(mb.group(2), 16)
            bitfields[name] = BitField(
                name=name, mask=mask,
                shift=mask_to_shift(mask),
                width=mask_to_width(mask),
            )
            continue

        # --- Bitmask constants (hex, no cast) — only if looks like a bitmask ---
        mb = RE_BITMASK_HEX.match(stripped)
        if mb:
            name = mb.group(1)
            # Skip things that are clearly not bitmasks (e.g., NUM_CHANNELS)
            if any(kw in name for kw in ("NUM_", "SOURCE_", "ADDRESS")):
                continue
            mask = int(mb.group(2), 16)
            if mask > 0:
                bitfields[name] = BitField(
                    name=name, mask=mask,
                    shift=mask_to_shift(mask),
                    width=mask_to_width(mask),
                )
            continue

        # --- Parameterized fields ---
        mp = RE_PARAM_FIELD.match(stripped)
        if mp:
            name = mp.group(1)
            mask = parse_int(mp.group(2))
            shift = int(mp.group(3))
            param_fields[name] = ParamField(
                name=name, mask=mask, shift=shift,
                width=mask_to_width(mask),
            )
            continue

    return {
        "registers": registers,
        "bitfields": bitfields,
        "param_fields": param_fields,
        "irqs": irqs,
        "structs": structs,
    }


# ---------------------------------------------------------------------------
# SVD parser
# ---------------------------------------------------------------------------

def parse_svd(svd_path: str) -> dict:
    """Parse an SVD file into structured data for comparison."""
    tree = ElementTree.parse(svd_path)
    root = tree.getroot()

    peripherals = {}
    registers = {}
    fields = {}
    interrupts = {}

    for periph_el in root.findall(".//peripheral"):
        periph_name = periph_el.findtext("name", "")
        base_addr_text = periph_el.findtext("baseAddress", "0")
        base_addr = int(base_addr_text, 0)

        # Handle derivedFrom
        derived_from = periph_el.get("derivedFrom")

        peripherals[periph_name] = {
            "name": periph_name,
            "base_address": base_addr,
            "derived_from": derived_from,
        }

        # Parse interrupts
        for intr_el in periph_el.findall("interrupt"):
            intr_name = intr_el.findtext("name", "")
            intr_value = int(intr_el.findtext("value", "0"))
            interrupts[intr_name] = {"name": intr_name, "value": intr_value}

        # Parse registers
        # If derivedFrom, we'll need to look up the base peripheral's registers
        # For now, parse what's directly available
        regs_el = periph_el.find("registers")
        if regs_el is None:
            continue

        default_size = 32
        size_el = periph_el.findtext("size")
        if size_el:
            default_size = int(size_el, 0)

        for reg_el in regs_el.findall("register"):
            reg_name = reg_el.findtext("name", "")
            offset_text = reg_el.findtext("addressOffset", "0")
            offset = int(offset_text, 0)
            reg_addr = base_addr + offset

            reg_size_text = reg_el.findtext("size")
            if reg_size_text:
                reg_size = int(reg_size_text, 0)
            else:
                reg_size = default_size

            # Handle dimensioned registers (dim arrays)
            dim_text = reg_el.findtext("dim")
            dim_increment_text = reg_el.findtext("dimIncrement")
            dim_index_text = reg_el.findtext("dimIndex")

            if dim_text:
                dim = int(dim_text, 0)
                dim_increment = int(dim_increment_text, 0) if dim_increment_text else reg_size // 8

                # Parse dim index
                if dim_index_text and "," in dim_index_text:
                    indices = dim_index_text.split(",")
                elif dim_index_text and "-" in dim_index_text:
                    parts = dim_index_text.split("-")
                    indices = [str(i) for i in range(int(parts[0]), int(parts[1]) + 1)]
                else:
                    indices = [str(i) for i in range(dim)]

                for idx_i, idx in enumerate(indices):
                    expanded_name = reg_name.replace("%s", idx)
                    expanded_addr = reg_addr + idx_i * dim_increment
                    full_name = f"{periph_name}_{expanded_name}"

                    registers[full_name] = {
                        "peripheral": periph_name,
                        "name": expanded_name,
                        "address": expanded_addr,
                        "size": reg_size,
                    }

                    # Parse fields for this register
                    for field_el in reg_el.findall(".//field"):
                        field_name = field_el.findtext("name", "")
                        bit_offset = int(field_el.findtext("bitOffset", "0"))
                        bit_width = int(field_el.findtext("bitWidth", "1"))
                        field_key = f"{full_name}_{field_name}"
                        fields[field_key] = {
                            "register": full_name,
                            "name": field_name,
                            "bit_offset": bit_offset,
                            "bit_width": bit_width,
                        }
            else:
                full_name = f"{periph_name}_{reg_name}"
                registers[full_name] = {
                    "peripheral": periph_name,
                    "name": reg_name,
                    "address": reg_addr,
                    "size": reg_size,
                }

                # Parse fields
                for field_el in reg_el.findall(".//field"):
                    field_name = field_el.findtext("name", "")
                    bit_offset = int(field_el.findtext("bitOffset", "0"))
                    bit_width = int(field_el.findtext("bitWidth", "1"))
                    field_key = f"{full_name}_{field_name}"
                    fields[field_key] = {
                        "register": full_name,
                        "name": field_name,
                        "bit_offset": bit_offset,
                        "bit_width": bit_width,
                    }

    # Build address -> register lookup for matching
    addr_to_regs = {}
    for rname, rdata in registers.items():
        addr = rdata["address"]
        if addr not in addr_to_regs:
            addr_to_regs[addr] = []
        addr_to_regs[addr].append(rdata)

    return {
        "peripherals": peripherals,
        "registers": registers,
        "fields": fields,
        "interrupts": interrupts,
        "addr_to_regs": addr_to_regs,
    }


# ---------------------------------------------------------------------------
# SVD name normalization
# ---------------------------------------------------------------------------

# Mapping from kinetis.h peripheral prefix to SVD peripheral name
PERIPH_NAME_MAP = {
    "PORTA": "PORTA", "PORTB": "PORTB", "PORTC": "PORTC",
    "PORTD": "PORTD", "PORTE": "PORTE",
    "PTA": "PTA", "PTB": "PTB", "PTC": "PTC", "PTD": "PTD", "PTE": "PTE",
    "SIM": "SIM", "MCG": "MCG", "OSC0": "OSC0", "OSC": "OSC",
    "UART0": "UART0", "UART1": "UART1", "UART2": "UART2",
    "UART3": "UART3", "UART4": "UART4",
    "SPI0": "SPI0", "SPI1": "SPI1",
    "I2C0": "I2C0", "I2C1": "I2C1",
    "I2S0": "I2S0",
    "FTM0": "FTM0", "FTM1": "FTM1", "FTM2": "FTM2",
    "ADC0": "ADC0", "ADC1": "ADC1",
    "DAC0": "DAC0", "DAC1": "DAC1",
    "DMA": "DMA", "DMAMUX": "DMAMUX",
    "PIT": "PIT", "PDB0": "PDB0", "PDB": "PDB0",
    "RTC": "RTC", "LPTMR0": "LPTMR0", "LPTMR": "LPTMR0",
    "CMP0": "CMP0", "CMP1": "CMP1", "CMP2": "CMP2",
    "VREF": "VREF",
    "LLWU": "LLWU",
    "PMC": "PMC", "SMC": "SMC", "RCM": "RCM",
    "WDOG": "WDOG", "EWM": "EWM",
    "USB0": "USB0", "USBDCD": "USBDCD",
    "TSI0": "TSI0", "TSI": "TSI0",
    "CRC": "CRC", "CMT": "CMT",
    "FTFL": "FTFL", "FMC": "FMC",
    "CAN0": "CAN0",
    "FB": "FB",
    "MCM": "MCM",
}


def header_name_to_svd_name(header_reg_name: str, svd_registers: dict) -> Optional[str]:
    """Try to find the SVD register name that corresponds to a kinetis.h register name.

    The SVD uses PERIPHERAL_REGISTER format. kinetis.h often uses
    the same convention but with some differences.
    """
    # Direct match
    if header_reg_name in svd_registers:
        return header_reg_name

    # Try common transformations
    # kinetis.h: PORTA_PCR0 -> SVD: PORTA_PCR0 (usually matches)
    # kinetis.h: FTM0_SC -> SVD: FTM0_SC (usually matches)

    return None


# ---------------------------------------------------------------------------
# Cross-reference engine
# ---------------------------------------------------------------------------

# Peripherals in kinetis.h that do NOT exist on MK20DX128/MK20DX256.
# These appear because kinetis.h covers K20/K64/K66 and the shared KINETISK
# section includes registers for peripherals only on larger chips.
K64_K66_ONLY_PERIPHERALS = {
    "ENET", "SDHC", "USBHS", "USBHSDCD", "USBPHY",
    "CAN1", "FTM3", "DAC1", "CMP3",
    "RNG", "RNGA",
    "LMEM", "CAU",
    "TPM0", "TPM1", "TPM2",
    "LPUART", "LPUART0",
    "I2C2", "I2C3",
    "UART3", "UART4", "UART5",
    "SPI2",
}

# Register prefixes that indicate K64/K66-only content even within shared peripherals
K64_K66_ONLY_PREFIXES = {
    "ENET_", "SDHC_", "USBHS_", "USBHSDCD_", "USBPHY_",
    "CAN1_", "FTM3_", "DAC1_", "CMP3_",
    "RNG_", "RNGA_",
    "LMEM_", "CAU_",
    "TPM0_", "TPM1_", "TPM2_",
    "LPUART0_", "LPUART_",
    "I2C2_", "I2C3_",
    "UART3_", "UART4_", "UART5_",
    "SPI2_",
}


def is_k64_k66_only(reg_name: str) -> bool:
    """Check if a register name belongs to a K64/K66-only peripheral."""
    for prefix in K64_K66_ONLY_PREFIXES:
        if reg_name.startswith(prefix):
            return True
    return False


def compare_registers(header_data: dict, svd_data: dict) -> list[Discrepancy]:
    """Compare header register definitions against SVD."""
    discrepancies = []
    matched = 0
    unmatched_header = 0
    skipped_k64_k66 = 0

    for hdr_name, hdr_reg in header_data["registers"].items():
        # Skip registers that belong to K64/K66-only peripherals
        if is_k64_k66_only(hdr_name):
            skipped_k64_k66 += 1
            continue

        # Try to find by address
        svd_at_addr = svd_data["addr_to_regs"].get(hdr_reg.address, [])

        if not svd_at_addr:
            # Try direct name match
            svd_reg = svd_data["registers"].get(hdr_name)
            if svd_reg:
                if svd_reg["address"] != hdr_reg.address:
                    discrepancies.append(Discrepancy(
                        category=Mismatch.ADDRESS_MISMATCH,
                        peripheral=hdr_reg.peripheral,
                        register=hdr_name,
                        field="",
                        header_value=f"0x{hdr_reg.address:08X}",
                        svd_value=f"0x{svd_reg['address']:08X}",
                        detail=f"Register address mismatch: header=0x{hdr_reg.address:08X}, SVD=0x{svd_reg['address']:08X}",
                    ))
                else:
                    matched += 1
            else:
                unmatched_header += 1
                discrepancies.append(Discrepancy(
                    category=Mismatch.MISSING_IN_SVD,
                    peripheral=hdr_reg.peripheral,
                    register=hdr_name,
                    field="",
                    header_value=f"0x{hdr_reg.address:08X} ({hdr_reg.size}-bit)",
                    svd_value=None,
                    detail=f"Register {hdr_name} at 0x{hdr_reg.address:08X} not found in SVD",
                ))
            continue

        # Found register(s) at this address — check size
        # Pick the best name match
        best_match = None
        for svd_reg in svd_at_addr:
            svd_full = f"{svd_reg['peripheral']}_{svd_reg['name']}"
            if svd_full == hdr_name or svd_reg["name"] == hdr_name:
                best_match = svd_reg
                break
        if best_match is None:
            # Use first one at that address
            best_match = svd_at_addr[0]

        matched += 1

        # Check register size
        if best_match["size"] != hdr_reg.size:
            svd_full = f"{best_match['peripheral']}_{best_match['name']}"
            discrepancies.append(Discrepancy(
                category=Mismatch.SIZE_MISMATCH,
                peripheral=hdr_reg.peripheral,
                register=hdr_name,
                field="",
                header_value=f"{hdr_reg.size}-bit",
                svd_value=f"{best_match['size']}-bit",
                detail=f"Register size: header={hdr_reg.size}-bit, SVD={best_match['size']}-bit (SVD name: {svd_full})",
            ))

    return discrepancies, matched, unmatched_header, skipped_k64_k66


def compare_irqs(header_data: dict, svd_data: dict) -> list[Discrepancy]:
    """Compare header IRQ numbers against SVD interrupt definitions."""
    discrepancies = []
    matched = 0

    # Build SVD IRQ lookup by normalized name
    svd_irq_by_name = {}
    for name, data in svd_data["interrupts"].items():
        svd_irq_by_name[name.upper()] = data

    # Build SVD IRQ lookup by number
    svd_irq_by_number = {}
    for name, data in svd_data["interrupts"].items():
        svd_irq_by_number[data["value"]] = data

    for hdr_name, hdr_irq in header_data["irqs"].items():
        # Try to find SVD interrupt with matching name
        # kinetis.h uses IRQ_DMA_CH0, SVD might use DMA0, DMA_CH0, etc.
        # Strip IRQ_ prefix for matching
        search_name = hdr_name.replace("IRQ_", "")

        found = False
        # Try exact match
        if search_name in svd_irq_by_name:
            svd_irq = svd_irq_by_name[search_name]
            found = True
        else:
            # Try to find by number
            if hdr_irq.number in svd_irq_by_number:
                svd_irq = svd_irq_by_number[hdr_irq.number]
                found = True

        if not found:
            # Check if the SVD has this IRQ at number+16 (known offset bug)
            offset_number = hdr_irq.number + 16
            if offset_number in svd_irq_by_number:
                svd_irq = svd_irq_by_number[offset_number]
                discrepancies.append(Discrepancy(
                    category=Mismatch.IRQ_MISMATCH,
                    peripheral="",
                    register="",
                    field=hdr_name,
                    header_value=str(hdr_irq.number),
                    svd_value=f"{svd_irq['value']} (SVD name: {svd_irq['name']})",
                    detail=f"IRQ {hdr_name}: header={hdr_irq.number}, SVD={svd_irq['value']} — likely +16 offset bug",
                ))
                continue
            else:
                discrepancies.append(Discrepancy(
                    category=Mismatch.MISSING_IN_SVD,
                    peripheral="",
                    register="",
                    field=hdr_name,
                    header_value=str(hdr_irq.number),
                    svd_value=None,
                    detail=f"IRQ {hdr_name}={hdr_irq.number} not found in SVD (also checked +16 offset)",
                ))
                continue

        if svd_irq["value"] != hdr_irq.number:
            discrepancies.append(Discrepancy(
                category=Mismatch.IRQ_MISMATCH,
                peripheral="",
                register="",
                field=hdr_name,
                header_value=str(hdr_irq.number),
                svd_value=f"{svd_irq['value']} (SVD name: {svd_irq['name']})",
                detail=f"IRQ {hdr_name}: header={hdr_irq.number}, SVD={svd_irq['value']}",
            ))
        else:
            matched += 1

    return discrepancies, matched


def compare_bitfields(header_data: dict, svd_data: dict) -> list[Discrepancy]:
    """Compare header bitfield definitions against SVD fields.

    Uses peripheral-aware matching to avoid false positives from fields
    with the same name in different peripherals.
    """
    discrepancies = []
    matched = 0
    checked = 0
    not_found = 0

    # Build index: (peripheral, field_name) -> list of SVD fields
    # Also (peripheral, register_suffix, field_name) for precise matching
    svd_field_by_periph = {}  # (periph, field_name) -> [svd_field]
    svd_field_by_reg = {}     # (periph, reg_suffix, field_name) -> svd_field
    for svd_key, svd_field in svd_data["fields"].items():
        reg_name = svd_field["register"]  # e.g., "UART0_BDH"
        parts = reg_name.split("_", 1)
        if len(parts) == 2:
            periph_name, reg_suffix = parts
        else:
            periph_name = parts[0]
            reg_suffix = ""

        fname = svd_field["name"].upper()
        key = (periph_name, fname)
        if key not in svd_field_by_periph:
            svd_field_by_periph[key] = []
        svd_field_by_periph[key].append(svd_field)

        precise_key = (periph_name, reg_suffix.upper(), fname)
        svd_field_by_reg[precise_key] = svd_field

    for hdr_name, hdr_field in header_data["param_fields"].items():
        parts = hdr_name.split("_")
        if len(parts) < 3:
            continue

        hdr_periph = parts[0]  # e.g., "PORT", "FTM", "MCG", "DMA"
        field_name = parts[-1].upper()

        # Determine the register suffix (everything between peripheral and field)
        # E.g., MCG_C1_CLKS -> reg_suffix=C1, field=CLKS
        # E.g., DMA_TCD_BITER_ELINKYES_BITER -> complex, just use field name
        reg_suffix_parts = parts[1:-1]  # middle parts

        # Try precise match first: look for field in the exact peripheral
        # For peripherals like "PORT" that map to "PORTA", "PORTB", etc., try numbered variants
        candidate_periphs = [hdr_periph]
        # Add numbered variants (FTM -> FTM0, FTM1; PORT -> PORTA, PORTB; etc.)
        for suffix in ["0", "1", "2", "3", "A", "B", "C", "D", "E"]:
            candidate_periphs.append(f"{hdr_periph}{suffix}")

        found = False
        for cand_periph in candidate_periphs:
            # Try precise register+field match
            if reg_suffix_parts:
                reg_suffix = "_".join(reg_suffix_parts).upper()
                precise_key = (cand_periph, reg_suffix, field_name)
                if precise_key in svd_field_by_reg:
                    svd_field = svd_field_by_reg[precise_key]
                    found = True
                    checked += 1
                    _check_field(hdr_name, hdr_field, hdr_periph, svd_field,
                                f"{cand_periph}_{reg_suffix}_{field_name}",
                                discrepancies, matched)
                    if svd_field["bit_width"] == hdr_field.width and svd_field["bit_offset"] == hdr_field.shift:
                        matched += 1
                    break

            # Try peripheral+field match (any register in that peripheral)
            key = (cand_periph, field_name)
            if key in svd_field_by_periph:
                svd_field = svd_field_by_periph[key][0]
                found = True
                checked += 1
                _check_field(hdr_name, hdr_field, hdr_periph, svd_field,
                            f"{cand_periph}_*_{field_name}",
                            discrepancies, matched)
                if svd_field["bit_width"] == hdr_field.width and svd_field["bit_offset"] == hdr_field.shift:
                    matched += 1
                break

        if not found:
            not_found += 1

    return discrepancies, matched, checked


def _check_field(hdr_name, hdr_field, hdr_periph, svd_field, svd_key, discrepancies, matched):
    """Check a single header field against an SVD field, appending any discrepancy."""
    if svd_field["bit_width"] != hdr_field.width:
        discrepancies.append(Discrepancy(
            category=Mismatch.FIELD_WIDTH_MISMATCH,
            peripheral=hdr_periph,
            register=svd_field["register"],
            field=hdr_name,
            header_value=f"width={hdr_field.width} (mask=0x{hdr_field.mask:X})",
            svd_value=f"width={svd_field['bit_width']}",
            detail=f"Field width: header {hdr_name} width={hdr_field.width}, SVD {svd_field['register']}.{svd_field['name']} width={svd_field['bit_width']}",
        ))
    elif svd_field["bit_offset"] != hdr_field.shift:
        discrepancies.append(Discrepancy(
            category=Mismatch.FIELD_OFFSET_MISMATCH,
            peripheral=hdr_periph,
            register=svd_field["register"],
            field=hdr_name,
            header_value=f"offset={hdr_field.shift}",
            svd_value=f"offset={svd_field['bit_offset']}",
            detail=f"Field offset: header {hdr_name} shift={hdr_field.shift}, SVD {svd_field['register']}.{svd_field['name']} offset={svd_field['bit_offset']}",
        ))


def compare_struct_sizes(header_data: dict, svd_data: dict) -> list[Discrepancy]:
    """Use struct typedefs to verify register sizes in SVD.

    Struct typedefs in kinetis.h reveal exact register sizes (uint8_t vs uint32_t).
    This catches the common bug where SVD says 32-bit but register is actually 8-bit.
    """
    discrepancies = []

    seen = set()  # Deduplicate across instances of same struct type

    for type_name, struct_info in header_data["structs"].items():
        instances = struct_info.get("instances", [])
        if not instances:
            continue

        for inst in instances:
            base_addr = inst["base_address"]
            inst_name = inst["name"]

            for member in struct_info["members"]:
                reg_addr = base_addr + member.offset
                svd_at_addr = svd_data["addr_to_regs"].get(reg_addr, [])

                for svd_reg in svd_at_addr:
                    if svd_reg["size"] != member.size:
                        svd_full = f"{svd_reg['peripheral']}_{svd_reg['name']}"
                        dedup_key = (svd_reg["peripheral"], svd_reg["name"], member.name)
                        if dedup_key in seen:
                            continue
                        seen.add(dedup_key)
                        discrepancies.append(Discrepancy(
                            category=Mismatch.SIZE_MISMATCH,
                            peripheral=svd_reg["peripheral"],
                            register=svd_full,
                            field="",
                            header_value=f"{member.size}-bit (from {type_name}.{member.name})",
                            svd_value=f"{svd_reg['size']}-bit",
                            detail=f"Struct {type_name}.{member.name} is {member.size}-bit but SVD {svd_full} is {svd_reg['size']}-bit",
                        ))

    return discrepancies


# ---------------------------------------------------------------------------
# Report generation
# ---------------------------------------------------------------------------

def generate_report(
    discrepancies: list[Discrepancy],
    reg_matched: int,
    reg_unmatched: int,
    reg_skipped: int,
    irq_matched: int,
    field_matched: int,
    field_checked: int,
    header_data: dict,
    svd_data: dict,
) -> dict:
    """Generate a structured report."""
    # Group by category
    by_category = {}
    for d in discrepancies:
        cat = d.category
        if cat not in by_category:
            by_category[cat] = []
        by_category[cat].append(d)

    # Group by peripheral
    by_peripheral = {}
    for d in discrepancies:
        periph = d.peripheral or "(global)"
        if periph not in by_peripheral:
            by_peripheral[periph] = []
        by_peripheral[periph].append(d)

    return {
        "summary": {
            "total_discrepancies": len(discrepancies),
            "registers_matched": reg_matched,
            "registers_unmatched_in_svd": reg_unmatched,
            "registers_skipped_k64_k66": reg_skipped,
            "irqs_matched": irq_matched,
            "irqs_in_header": len(header_data["irqs"]),
            "fields_matched": field_matched,
            "fields_checked": field_checked,
            "header_registers": len(header_data["registers"]),
            "header_bitfields": len(header_data["bitfields"]),
            "header_param_fields": len(header_data["param_fields"]),
            "svd_registers": len(svd_data["registers"]),
            "svd_fields": len(svd_data["fields"]),
            "svd_interrupts": len(svd_data["interrupts"]),
        },
        "by_category": {
            cat: [asdict(d) for d in items]
            for cat, items in sorted(by_category.items())
        },
        "by_peripheral": {
            periph: [asdict(d) for d in items]
            for periph, items in sorted(by_peripheral.items())
        },
    }


def print_report(report: dict, variant: str):
    """Print a human-readable summary."""
    s = report["summary"]
    print(f"\n{'='*72}")
    print(f"  SVD vs kinetis.h Comparison Report — {variant.upper()}")
    print(f"{'='*72}")

    print(f"\n  Header: {s['header_registers']} registers, "
          f"{s['header_bitfields']} bitmasks, "
          f"{s['header_param_fields']} parameterized fields, "
          f"{s['irqs_in_header']} IRQs")
    print(f"  SVD:    {s['svd_registers']} registers, "
          f"{s['svd_fields']} fields, "
          f"{s['svd_interrupts']} interrupts")

    print(f"\n  Register matching: {s['registers_matched']} matched, "
          f"{s['registers_unmatched_in_svd']} missing from SVD, "
          f"{s['registers_skipped_k64_k66']} skipped (K64/K66-only)")
    print(f"  IRQ matching:      {s['irqs_matched']}/{s['irqs_in_header']} matched")
    print(f"  Field matching:    {s['fields_matched']}/{s['fields_checked']} checked OK")

    print(f"\n  Total discrepancies: {s['total_discrepancies']}")
    print(f"{'='*72}")

    # By category
    for cat, items in sorted(report["by_category"].items()):
        print(f"\n  [{cat}] ({len(items)} issues)")
        print(f"  {'-'*68}")
        for item in items[:50]:  # Limit output
            detail = item["detail"]
            print(f"    {detail}")
        if len(items) > 50:
            print(f"    ... and {len(items) - 50} more")

    # Summary by peripheral
    print(f"\n  {'='*72}")
    print(f"  Issues by peripheral:")
    print(f"  {'-'*68}")
    for periph, items in sorted(report["by_peripheral"].items()):
        cats = {}
        for item in items:
            c = item["category"]
            cats[c] = cats.get(c, 0) + 1
        cat_str = ", ".join(f"{c}: {n}" for c, n in sorted(cats.items()))
        print(f"    {periph:20s} {len(items):4d} issues  ({cat_str})")

    print()


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="Compare kinetis.h register definitions against SVD file"
    )
    parser.add_argument("--variant", required=True, choices=["mk20d5", "mk20d7"],
                        help="Chip variant to compare")
    parser.add_argument("--svd", required=True, help="Path to SVD file")
    parser.add_argument("--header", required=True, help="Path to kinetis.h")
    parser.add_argument("--json", help="Output JSON report to file")
    parser.add_argument("--verbose", "-v", action="store_true",
                        help="Verbose output")
    args = parser.parse_args()

    # Read and filter header
    header_path = Path(args.header)
    if not header_path.exists():
        print(f"Error: header file not found: {args.header}", file=sys.stderr)
        sys.exit(1)

    svd_path = Path(args.svd)
    if not svd_path.exists():
        print(f"Error: SVD file not found: {args.svd}", file=sys.stderr)
        sys.exit(1)

    print(f"Reading {args.header}...")
    raw_lines = header_path.read_text().splitlines()

    print(f"Filtering for variant {args.variant}...")
    filtered_lines = filter_lines_for_variant(raw_lines, args.variant)
    print(f"  {len(raw_lines)} raw lines -> {len(filtered_lines)} active lines")

    print(f"Parsing header definitions...")
    header_data = parse_header(filtered_lines)
    print(f"  {len(header_data['registers'])} registers, "
          f"{len(header_data['bitfields'])} bitmasks, "
          f"{len(header_data['param_fields'])} param fields, "
          f"{len(header_data['irqs'])} IRQs, "
          f"{len(header_data['structs'])} structs")

    print(f"Parsing {args.svd}...")
    svd_data = parse_svd(args.svd)
    print(f"  {len(svd_data['peripherals'])} peripherals, "
          f"{len(svd_data['registers'])} registers, "
          f"{len(svd_data['fields'])} fields, "
          f"{len(svd_data['interrupts'])} interrupts")

    # Run comparisons
    print(f"\nComparing...")
    reg_discrep, reg_matched, reg_unmatched, reg_skipped = compare_registers(header_data, svd_data)
    irq_discrep, irq_matched = compare_irqs(header_data, svd_data)
    field_discrep, field_matched, field_checked = compare_bitfields(header_data, svd_data)
    struct_discrep = compare_struct_sizes(header_data, svd_data)

    all_discrepancies = reg_discrep + irq_discrep + field_discrep + struct_discrep

    # Generate report
    report = generate_report(
        all_discrepancies,
        reg_matched, reg_unmatched, reg_skipped,
        irq_matched,
        field_matched, field_checked,
        header_data, svd_data,
    )

    print_report(report, args.variant)

    # JSON output
    if args.json:
        with open(args.json, "w") as f:
            json.dump(report, f, indent=2)
        print(f"JSON report written to {args.json}")


if __name__ == "__main__":
    main()
