import re
import sys

def parse_val(val_str):
    if not val_str:
        return None
    val_str = val_str.replace('**', '').strip()
    if val_str == 'na' or val_str == '':
        return None
    # Handle artifacts like -4<br>6 -> -46
    val_str = val_str.replace('<br>', '')
    # Handle unicode minus
    val_str = val_str.replace('−', '-')
    try:
        return int(val_str)
    except ValueError:
        return None

def parse_markdown_tables(content):
    tables = {}
    lines = content.split('\n')
    current_table = None
    table_lines = []

    for line in lines:
        if line.strip().startswith('**Table 9-'):
            if current_table:
                tables[current_table] = table_lines
            # Extract table number
            match = re.search(r'Table 9-(\d+)', line)
            if match:
                current_table = int(match.group(1))
            table_lines = []
        elif current_table and line.strip().startswith('|'):
            # Check if it's a separator line
            if set(line.strip()) <= {'|', '-', ' '}:
                continue
            # Remove Markdown bold formatting from cells
            row = [cell.strip() for cell in line.strip().strip('|').split('|')]
            table_lines.append(row)
        elif current_table and not line.strip().startswith('|') and line.strip():
            # End of table
            tables[current_table] = table_lines
            current_table = None
            table_lines = []

    if current_table:
        tables[current_table] = table_lines
    return tables

def extract_ground_truth(tables):
    # ground_truth[type][ctxIdx] = (m, n)
    # Types: 'I', 'PB0', 'PB1', 'PB2'
    ground_truth = {
        'I': {},
        'PB0': {},
        'PB1': {},
        'PB2': {},
    }

    # Helper to set value
    def set_gt(type_key, ctx, m, n, table_source=""):
        if m is not None and n is not None:
            ground_truth[type_key][ctx] = (m, n)

    # Process Tables 9-12 to 9-17 (Column-based ctxIdx)
    # Table 9-12: ctxIdx 0-10
    # Rows: Header, Header, m, n
    t12 = tables.get(12)
    if t12:
        header_row_idx = -1
        for i, row in enumerate(t12):
            clean_row = [re.sub(r'\*+', '', c).strip() for c in row]
            if '0' in clean_row and '1' in clean_row:
                header_row_idx = i
                break

        if header_row_idx != -1:
            ctx_indices = []
            for cell in t12[header_row_idx]:
                val = parse_val(cell)
                if val is not None:
                    ctx_indices.append(val)

            ms = []
            ns = []
            for row in t12[header_row_idx+1:]:
                clean_first = re.sub(r'\*+', '', row[0]).strip()
                if 'm' in clean_first:
                    ms = [parse_val(c) for c in row[1:]]
                elif 'n' in clean_first:
                    ns = [parse_val(c) for c in row[1:]]

            for i, ctx in enumerate(ctx_indices):
                if i < len(ms) and i < len(ns):
                    for t in ['I', 'PB0', 'PB1', 'PB2']:
                        set_gt(t, ctx, ms[i], ns[i], "Table 9-12")

    # Tables 9-13, 9-14, 9-15
    for t_num in [13, 14, 15]:
        t = tables.get(t_num)
        if not t: continue

        header_row_idx = -1
        for i, row in enumerate(t):
            clean_row = [re.sub(r'\*+', '', c).strip() for c in row]
            nums = [parse_val(c) for c in row if parse_val(c) is not None]
            if len(nums) > 5:
                header_row_idx = i
                break

        if header_row_idx == -1:
            continue

        ctx_indices = []
        start_col = 0
        for j, cell in enumerate(t[header_row_idx]):
            if parse_val(cell) is not None:
                start_col = j
                break

        for cell in t[header_row_idx][start_col:]:
            ctx_indices.append(parse_val(cell))

        for row in t[header_row_idx+1:]:
            c0 = re.sub(r'\*+', '', row[0]).strip() if len(row) > 0 else ""
            current_idc = None
            if '0' in c0: current_idc = 0
            elif '1' in c0: current_idc = 1
            elif '2' in c0: current_idc = 2

            c1 = re.sub(r'\*+', '', row[1]).strip() if len(row) > 1 else ""
            var_type = None
            if 'm' in c0 or 'm' in c1: var_type = 'm'
            if 'n' in c0 or 'n' in c1: var_type = 'n'

            if current_idc is not None and var_type:
                values = [parse_val(c) for c in row[start_col:]]
                target_map = None
                target_key = ''
                if current_idc == 0:
                    target_map = ground_truth['PB0']
                    target_key = 'PB0'
                elif current_idc == 1:
                    target_map = ground_truth['PB1']
                    target_key = 'PB1'
                elif current_idc == 2:
                    target_map = ground_truth['PB2']
                    target_key = 'PB2'

                if target_map is not None:
                    for i, ctx in enumerate(ctx_indices):
                        if i < len(values):
                            m_or_n = values[i]
                            if m_or_n is None: continue

                            curr = target_map.get(ctx, (None, None))
                            if var_type == 'm':
                                target_map[ctx] = (m_or_n, curr[1])
                            else:
                                target_map[ctx] = (curr[0], m_or_n)

                            pass

    # Table 9-16
    t16 = tables.get(16)
    if t16:
        header_row_idx = -1
        for i, row in enumerate(t16):
            nums = [parse_val(c) for c in row if parse_val(c) is not None]
            if len(nums) > 3:
                header_row_idx = i
                break

        ctx_indices = []
        start_col = 0
        for j, cell in enumerate(t16[header_row_idx]):
            if parse_val(cell) is not None:
                start_col = j
                break
        for cell in t16[header_row_idx][start_col:]:
            ctx_indices.append(parse_val(cell))

        for row in t16[header_row_idx+1:]:
            c0 = re.sub(r'\*+', '', row[0]).strip()
            current_idc = None
            if 'I slices' in c0: current_idc = 'I'
            elif '0' in c0: current_idc = 0
            elif '1' in c0: current_idc = 1
            elif '2' in c0: current_idc = 2

            c1 = re.sub(r'\*+', '', row[1]).strip()
            var_type = None
            if 'm' in c0 or 'm' in c1: var_type = 'm'
            if 'n' in c0 or 'n' in c1: var_type = 'n'

            if current_idc is not None and var_type:
                target_map = None
                target_key = ''
                if current_idc == 'I':
                    target_map = ground_truth['I']
                    target_key = 'I'
                elif current_idc == 0:
                    target_map = ground_truth['PB0']
                    target_key = 'PB0'
                elif current_idc == 1:
                    target_map = ground_truth['PB1']
                    target_key = 'PB1'
                elif current_idc == 2:
                    target_map = ground_truth['PB2']
                    target_key = 'PB2'

                if target_map is not None:
                    values = [parse_val(c) for c in row[start_col:]]
                    for i, ctx in enumerate(ctx_indices):
                        if i < len(values):
                            m_or_n = values[i]
                            if m_or_n is None: continue
                            curr = target_map.get(ctx, (None, None))
                            if var_type == 'm':
                                target_map[ctx] = (m_or_n, curr[1])
                            else:
                                target_map[ctx] = (curr[0], m_or_n)
                            pass

    # Table 9-17
    t17 = tables.get(17)
    if t17:
        header_row_idx = -1
        for i, row in enumerate(t17):
            nums = [parse_val(c) for c in row if parse_val(c) is not None]
            if len(nums) > 5:
                header_row_idx = i
                break

        ctx_indices = []
        start_col = 0
        for j, cell in enumerate(t17[header_row_idx]):
            if parse_val(cell) is not None:
                start_col = j
                break
        for cell in t17[header_row_idx][start_col:]:
            ctx_indices.append(parse_val(cell))

        for row in t17[header_row_idx+1:]:
            c0 = re.sub(r'\*+', '', row[0]).strip()
            var_type = None
            if 'm' in c0: var_type = 'm'
            elif 'n' in c0: var_type = 'n'

            if var_type:
                values = [parse_val(c) for c in row[start_col:]]
                for i, ctx in enumerate(ctx_indices):
                    if i < len(values):
                        m_or_n = values[i]
                        for t in ['I', 'PB0', 'PB1', 'PB2']:
                            target_map = ground_truth[t]
                            curr = target_map.get(ctx, (None, None))
                            if var_type == 'm':
                                target_map[ctx] = (m_or_n, curr[1])
                            else:
                                target_map[ctx] = (curr[0], m_or_n)
                            pass

    # Tables 9-18 to 9-33
    for t_num in range(18, 34):
        t = tables.get(t_num)
        if not t: continue

        for row in t:
            vals = [parse_val(c) for c in row]

            # Skip header rows where the first column is not a number (ctxIdx)
            if vals[0] is None:
                continue

            chunks = []
            if len(vals) >= 9:
                chunks.append(vals[0:9])
            if len(vals) >= 18:
                chunks.append(vals[9:18])

            for chunk in chunks:
                if len(chunk) < 9: continue
                ctx = chunk[0]
                if ctx is None: continue

                set_gt('I', ctx, chunk[1], chunk[2], f"Table 9-{t_num}")
                set_gt('PB0', ctx, chunk[3], chunk[4], f"Table 9-{t_num}")
                set_gt('PB1', ctx, chunk[5], chunk[6], f"Table 9-{t_num}")
                set_gt('PB2', ctx, chunk[7], chunk[8], f"Table 9-{t_num}")

    return ground_truth

def parse_rust_table(content):
    content = re.sub(r'//.*', '', content)
    matches = re.findall(r'\(\s*(-?\d+),\s*(-?\d+)\s*\)', content)
    return [(int(m), int(n)) for m, n in matches]

def main():
    print("Reading spec...")
    with open('spec/sections/9.3_CABAC_parsing_process_for_slice_data.md', 'r') as f:
        spec_content = f.read()

    tables = parse_markdown_tables(spec_content)
    ground_truth = extract_ground_truth(tables)

    print("Reading rust files...")
    rust_files = {
        'I': 'src/h264/cabac_init_tables_i.rs',
        'PB0': 'src/h264/cabac_init_tables_pb0.rs',
        'PB1': 'src/h264/cabac_init_tables_pb1.rs',
        'PB2': 'src/h264/cabac_init_tables_pb2.rs',
    }

    errors = 0
    checked_count = 0

    for type_key, filepath in rust_files.items():
        with open(filepath, 'r') as f:
            rust_content = f.read()

        rust_vals = parse_rust_table(rust_content)
        if len(rust_vals) != 1024:
            print(f"Error: {filepath} has {len(rust_vals)} entries, expected 1024")
            errors += 1
            continue

        gt_map = ground_truth[type_key]

        print(f"Verifying {type_key} against {filepath}...")

        for ctx, (expected_m, expected_n) in gt_map.items():
            # If expected is None, we skip
            if expected_m is None or expected_n is None:
                continue

            actual_m, actual_n = rust_vals[ctx]

            if (actual_m, actual_n) != (expected_m, expected_n):
                print(f"Mismatch in {type_key} at ctxIdx {ctx}: Spec ({expected_m}, {expected_n}) != Code ({actual_m}, {actual_n})")
                errors += 1

            checked_count += 1

    if errors == 0:
        print(f"Validation PASSED. Checked {checked_count} entries.")
    else:
        print(f"Validation FAILED with {errors} mismatches.")
        sys.exit(1)

if __name__ == "__main__":
    main()
