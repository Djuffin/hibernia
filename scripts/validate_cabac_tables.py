import re
import sys
import os

def parse_signed_int(s):
    s = s.strip()
    s = s.replace('−', '-') # Replace unicode minus
    try:
        return int(s)
    except ValueError:
        return None

def parse_spec_tables(filepath):
    if not os.path.exists(filepath):
        print(f"Spec file not found: {filepath}")
        return {}

    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.readlines()

    tables = {}
    current_table = None
    table_header_regex = re.compile(r'\*\*Table 9-([0-9]+) – .*\*\*')

    # Store table lines to process after collecting them
    table_lines = []

    for i, line in enumerate(content):
        match = table_header_regex.search(line)
        if match:
            if current_table:
                process_table(current_table, table_lines, tables)
            current_table = int(match.group(1))
            table_lines = []
        elif current_table:
            # Check if we hit the next section or table (heuristic)
            if line.startswith('**') or line.startswith('Table 9-'):
                if not line.strip() == "":
                     # Might be start of next table or section, but regex handles table start
                     pass
            table_lines.append(line)

    if current_table:
        process_table(current_table, table_lines, tables)

    return tables

def process_table(table_id, lines, tables):
    # This function needs to handle various table formats
    # Clean up markdown table syntax
    clean_lines = []
    for line in lines:
        if line.strip().startswith('|'):
            clean_lines.append(line.strip())

    if not clean_lines:
        return

    rows = []
    for line in clean_lines:
        parts = [p.strip() for p in line.strip('|').split('|')]
        rows.append(parts)

    # Process based on Table ID
    if table_id == 12: # 0 to 10
        process_table_9_12(rows, tables)
    elif table_id == 13: # 11 to 23
        process_table_9_13(rows, tables)
    elif table_id == 14: # 24 to 39
        process_table_9_14(rows, tables)
    elif table_id == 15: # 40 to 53
        process_table_9_15(rows, tables)
    elif table_id == 16: # 54 to 59, 399 to 401
        process_table_9_16(rows, tables)
    elif table_id == 17: # 60 to 69
        process_table_9_17(rows, tables)
    elif table_id == 18: # 70 to 104
        process_table_9_18(rows, tables)
    elif table_id in [19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33]:
        process_table_generic(table_id, rows, tables)

def get_mn_from_rows(rows, start_col, count):
    # returns list of (m, n)
    m_row = None
    n_row = None
    for row in rows:
        if len(row) > 0 and 'm' in row[0]:
            m_row = row
        if len(row) > 0 and 'n' in row[0]:
            n_row = row

    if not m_row or not n_row:
        return []

    res = []
    for i in range(count):
        idx = start_col + i
        if idx < len(m_row) and idx < len(n_row):
            m = parse_signed_int(m_row[idx])
            n = parse_signed_int(n_row[idx])
            if m is not None and n is not None:
                res.append((m, n))
            else:
                res.append(None)
        else:
            res.append(None)
    return res

def add_to_tables(tables, ctx_idxs, mn_list, slice_types=['I', 'PB_0', 'PB_1', 'PB_2']):
    for i, ctx_idx in enumerate(ctx_idxs):
        if i < len(mn_list) and mn_list[i] is not None:
            if ctx_idx not in tables:
                tables[ctx_idx] = {}
            for st in slice_types:
                tables[ctx_idx][st] = mn_list[i]

def process_table_9_12(rows, tables):
    ctx_idxs = list(range(0, 11))
    mn_list = get_mn_from_rows(rows, 1, 11)
    add_to_tables(tables, ctx_idxs, mn_list)

def process_table_9_13(rows, tables):
    ctx_idxs = list(range(11, 24))
    process_init_idc_table(rows, tables, ctx_idxs)

def process_table_9_14(rows, tables):
    ctx_idxs = list(range(24, 40))
    process_init_idc_table(rows, tables, ctx_idxs)

def process_table_9_15(rows, tables):
    ctx_idxs = list(range(40, 54))
    process_init_idc_table(rows, tables, ctx_idxs)

def process_init_idc_table(rows, tables, ctx_idxs):
    for row in rows:
        if len(row) < 3: continue
        idc_val = row[0]
        var_name = row[1]

        current_slice_types = []
        if '0' in idc_val:
            current_slice_types = ['PB_0']
        elif '1' in idc_val:
            current_slice_types = ['PB_1']
        elif '2' in idc_val:
            current_slice_types = ['PB_2']
        else:
            continue

        if 'm' in var_name:
            m_values = row[2:]
            # Find n row
            idx = rows.index(row)
            if idx + 1 < len(rows):
                n_row = rows[idx+1]
                if 'n' in n_row[1] and n_row[0] == idc_val:
                    n_values = n_row[2:]
                    mn_list = []
                    for k in range(len(ctx_idxs)):
                        if k < len(m_values) and k < len(n_values):
                            m_str = m_values[k].split('<br>')[0]
                            n_str = n_values[k].split('<br>')[0]
                            m = parse_signed_int(m_str)
                            n = parse_signed_int(n_str)
                            if m is not None and n is not None:
                                mn_list.append((m, n))
                            else:
                                mn_list.append(None)
                        else:
                            mn_list.append(None)
                    add_to_tables(tables, ctx_idxs, mn_list, current_slice_types)

def process_table_9_16(rows, tables):
    ctx_idxs = list(range(54, 60)) + list(range(399, 402))

    for row in rows:
        if len(row) < 3: continue
        cond = row[0] # "I slices" or "0", "1", "2"
        var_name = row[1]

        slice_types = []
        if 'I' in cond:
            slice_types = ['I']
        elif '0' in cond:
            slice_types = ['PB_0']
        elif '1' in cond:
            slice_types = ['PB_1']
        elif '2' in cond:
            slice_types = ['PB_2']
        else:
            continue

        if 'm' in var_name:
            m_values = row[2:]
            # Find n row
            idx = rows.index(row)
            if idx + 1 < len(rows):
                n_row = rows[idx+1]
                if 'n' in n_row[1] and n_row[0] == cond:
                    n_values = n_row[2:]
                    mn_list = []
                    for k in range(len(ctx_idxs)):
                        if k < len(m_values) and k < len(n_values):
                            if 'na' in m_values[k] or 'na' in n_values[k]:
                                mn_list.append(None)
                            else:
                                m = parse_signed_int(m_values[k])
                                n = parse_signed_int(n_values[k])
                                if m is not None and n is not None:
                                    mn_list.append((m, n))
                                else:
                                    mn_list.append(None)
                        else:
                            mn_list.append(None)
                    add_to_tables(tables, ctx_idxs, mn_list, slice_types)

def process_table_9_17(rows, tables):
    ctx_idxs = list(range(60, 70))
    mn_list = get_mn_from_rows(rows, 1, 10)
    add_to_tables(tables, ctx_idxs, mn_list)

def process_table_9_18(rows, tables):
    start_row = 0
    for i, row in enumerate(rows):
        if len(row) > 0 and row[0].isdigit():
            start_row = i
            break

    for row in rows[start_row:]:
        if len(row) < 9: continue

        # Left side
        try:
            ctx_idx = int(row[0])
            add_mn(tables, ctx_idx, row, 1, 2, ['I'])
            add_mn(tables, ctx_idx, row, 3, 4, ['PB_0'])
            add_mn(tables, ctx_idx, row, 5, 6, ['PB_1'])
            add_mn(tables, ctx_idx, row, 7, 8, ['PB_2'])
        except ValueError:
            pass

        # Right side
        if len(row) > 9 and row[9].isdigit():
            try:
                ctx_idx = int(row[9])
                add_mn(tables, ctx_idx, row, 10, 11, ['I'])
                add_mn(tables, ctx_idx, row, 12, 13, ['PB_0'])
                add_mn(tables, ctx_idx, row, 14, 15, ['PB_1'])
                add_mn(tables, ctx_idx, row, 16, 17, ['PB_2'])
            except ValueError:
                pass

def process_table_generic(table_id, rows, tables):
    process_table_9_18(rows, tables)

def add_mn(tables, ctx_idx, row, col_m, col_n, slice_types):
    if col_m < len(row) and col_n < len(row):
        m = parse_signed_int(row[col_m].split('<br>')[0])
        n = parse_signed_int(row[col_n].split('<br>')[0])
        if m is not None and n is not None:
            if ctx_idx not in tables:
                tables[ctx_idx] = {}
            for st in slice_types:
                tables[ctx_idx][st] = (m, n)

def parse_rust_table(filepath):
    if not os.path.exists(filepath):
        print(f"Code file not found: {filepath}")
        return []
    with open(filepath, 'r') as f:
        content = f.read()
    # Extract tuples
    content = re.sub(r'//.*', '', content)
    content = content.replace('[', '').replace(']', '').replace('\n', '').strip()

    tuples = []
    items = content.split('),')
    for item in items:
        item = item.strip(' (),')
        if not item: continue
        parts = item.split(',')
        if len(parts) == 2:
            try:
                m = int(parts[0].strip())
                n = int(parts[1].strip())
                tuples.append((m, n))
            except ValueError:
                pass
    return tuples

def main():
    spec_path = 'spec/sections/9.3_CABAC_parsing_process_for_slice_data.md'
    tables = parse_spec_tables(spec_path)

    code_files = {
        'I': 'src/h264/cabac_init_tables_i.rs',
        'PB_0': 'src/h264/cabac_init_tables_pb0.rs',
        'PB_1': 'src/h264/cabac_init_tables_pb1.rs',
        'PB_2': 'src/h264/cabac_init_tables_pb2.rs',
    }

    code_arrays = {}
    for st, path in code_files.items():
        code_arrays[st] = parse_rust_table(path)
        print(f"Parsed {st} array size: {len(code_arrays[st])}")

    mismatches = 0
    for ctx_idx in sorted(tables.keys()):
        spec_vals = tables[ctx_idx]

        for st in ['I', 'PB_0', 'PB_1', 'PB_2']:
            if st in spec_vals:
                spec_m, spec_n = spec_vals[st]

                if ctx_idx < len(code_arrays[st]):
                    code_m, code_n = code_arrays[st][ctx_idx]

                    if (code_m, code_n) != (spec_m, spec_n):
                        print(f"Mismatch for ctxIdx {ctx_idx}, slice {st}: Spec ({spec_m}, {spec_n}) != Code ({code_m}, {code_n})")
                        mismatches += 1
                else:
                    print(f"Index {ctx_idx} out of bounds for {st}")

    if mismatches == 0:
        print("All checks passed!")
        sys.exit(0)
    else:
        print(f"Found {mismatches} mismatches.")
        sys.exit(1)

if __name__ == '__main__':
    main()
