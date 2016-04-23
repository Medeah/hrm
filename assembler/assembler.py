from fileinput import input
from sys import stdout, stderr


def error(mess):
    print("Error: {0}, line: {1}".format(mess, current_line), file=stderr)
    exit()


def print_num(numstr):
    numstr = numstr.strip("[]")
    tal = int(numstr)
    try:
        b = bytes([tal])
        stdout.buffer.write(b)
    except ValueError as e:
        error(e)


current_line = 0


single = {"INBOX": b"i", "OUTBOX": b"o"}
dint = {"COPYFROM": [b"f", b"g"],
        "COPYTO": [b"t", b"c"],
        "ADD": [b"a", b"b"],
        "SUB": [b"s", b"k"],
        "BUMPUP": [b"u", b"h"],
        "BUMPDN": [b"d", b"e"]
        }
djump = {"JUMP": b"j", "JUMPZ": b"z", "JUMPN": b"n"}

labels = {}


def parse_file():
    global current_line
    current_byte = 0
    for line in input():
        current_line += 1
        if line.strip().startswith("--"):
            continue
        if line.strip().startswith("DEFINE"):
            break

        if line.find(":") >= 0:
            lab, _ = line.split(":")
            if lab in labels:
                error("label " + lab + " defined x2")
            else:
                labels[lab] = current_byte

        else:
            commds = line.split()
            if len(commds) == 0:
                pass
            elif len(commds) == 1:
                if commds[0] in single:
                    stdout.buffer.write(single[commds[0]])
                    current_byte += 1
                else:
                    error("Unknown command: " + commds[0])
            elif len(commds) == 2:
                indirect = commds[1].find("[") >= 0
                if commds[0] in dint:
                    if indirect:
                        stdout.buffer.write(dint[commds[0]][0])
                    else:
                        stdout.buffer.write(dint[commds[0]][1])
                    print_num(commds[1])
                    current_byte += 2

                elif commds[0] in djump:
                    stdout.buffer.write(djump[commds[0]])
                    lab = commds[1]
                    if lab in labels:
                        stdout.buffer.write(bytes([labels[commds[1]]]))
                    else:
                        stdout.buffer.write(b"0")
                        #something
                    current_byte += 2
                elif commds[0] == "COMMENT":
                    pass
                else:
                    error("Unknown command: " + commds[0] + " " + commds[1])
            else:
                error("Too many in one line")
    return


if __name__ == "__main__":
    parse_file()
