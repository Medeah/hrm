from fileinput import input
from sys import stdout


class Assembler:
    current_line = 0
    bytes_ = bytearray()

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
    need_replacement = {}

    def error(self, mess):
        exit("Error: {0}, line: {1}".format(mess, self.current_line))

    def out(self, b):
        self.bytes_ += b

    def print_num(self, numstr):
        numstr = numstr.strip("[]")
        tal = int(numstr)
        try:
            b = bytearray([tal])
            self.out(b)
        except ValueError as e:
            self.error(e)

    def parse_file(self):
        for line in input():
            self.current_line += 1
            if line.strip().startswith("--"):
                continue
            if line.strip().startswith("DEFINE"):
                break

            if line.find(":") >= 0:
                lab, _ = line.split(":")
                if lab in self.labels:
                    self.error("label " + lab + " defined x2")
                else:
                    self.labels[lab] = len(self.bytes_)
                    if lab in self.need_replacement:
                        for index in self.need_replacement[lab]:
                            self.bytes_[index] = len(self.bytes_)
                        del self.need_replacement[lab]
            else:
                commds = line.split()
                if len(commds) == 0:
                    pass
                elif len(commds) == 1:
                    if commds[0] in self.single:
                        self.out(self.single[commds[0]])
                    else:
                        self.error("Unknown command: " + commds[0])
                elif len(commds) == 2:
                    if commds[0] in self.dint:
                        indirect = commds[1].find("[") >= 0
                        if indirect:
                            self.out(self.dint[commds[0]][1])
                        else:
                            self.out(self.dint[commds[0]][0])
                            self.print_num(commds[1])
                    elif commds[0] in self.djump:
                        self.out(self.djump[commds[0]])
                        lab = commds[1]
                        if lab in self.labels:
                            self.out(bytearray([self.labels[commds[1]]]))
                        else:
                            if lab in self.need_replacement:
                                self.need_replacement[lab].append(len(self.bytes_))
                            else:
                                self.need_replacement[lab] = [len(self.bytes_)]
                            self.out(b"0")
                    elif commds[0] == "COMMENT":
                        pass
                    else:
                        self.error("Unknown command: " + commds[0] + " " + commds[1])
                else:
                    self.error("Too many in one line")

        if not len(self.need_replacement) == 0:
            self.error("labels " + str(list(self.need_replacement.keys())) + " used, but not defined")
        stdout.buffer.write(self.bytes_)
        return


if __name__ == "__main__":
    x = Assembler()
    x.parse_file()
