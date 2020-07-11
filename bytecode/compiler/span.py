SPAN_COL = "\033[38;5;16m"
RESET_COL = "\033[0m"

def lpad(st, wanted_len, pad_ch=" "):
    return pad_ch * (wanted_len - len(st)) + st

class Span:
    def __init__(self, start, end, source):
        self.start = start
        self.end = end
        self.source_lines = source.split("\n")

    def print_aa(self):
        # Find start line
        line_start = 0
        for i, line in enumerate(self.source_lines):
            line += "\n"
            if line_start + len(line) > self.start:
                start_line = i
                break

            line_start += len(line)

        # Find start line
        line_end = 0
        for i, line in enumerate(self.source_lines):
            line += "\n"

            line_end += len(line)
            if line_end > self.end:
                end_line = i
                break


        offset_into_first_line = self.start - line_start
        offset_into_last_line = self.end - line_end + len(self.source_lines[end_line])

        linenr_len = len(str(end_line))

        print(SPAN_COL + f"  At self.source_lines {start_line+1}-{end_line+1}")

        if start_line == end_line:
            print(
                SPAN_COL +
                lpad(str(start_line+1), linenr_len) +
                " │",
                RESET_COL + self.source_lines[start_line]
            )
            print(
                SPAN_COL +
                "   " + " " * (linenr_len + offset_into_first_line) +
                "╰─" +
                "─" * (offset_into_last_line - offset_into_first_line - 2) +
                "╯" +
                RESET_COL
            )
        else:
            print(
                SPAN_COL +
                " " * linenr_len +
                " ┌─" +
                "─" * offset_into_first_line +
                "╮"
            )
            for line_idx in range(start_line, end_line + 1):
                print(
                    SPAN_COL +
                    lpad(str(line_idx+1), linenr_len) +
                    " │",
                    RESET_COL +
                    self.source_lines[line_idx]
                )
            print(
                SPAN_COL +
                " " * linenr_len +
                " └─" +
                "─" * offset_into_last_line +
                "╯" +
                RESET_COL
            )

if __name__ == "__main__":
    file_cont = open(__file__).read()

    span = Span(100, 105, file_cont)
    span.print_aa()
    print(file_cont[span.start], file_cont[span.end])
