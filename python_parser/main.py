data = "/home/cafalchio/projects/cdr_decoder/data/CDR_Nokia-MapeamentoClaro.txt"

class Parser:
    data = None
    def __init__(self, file):
        self.file = file
        self.get_txt()

    def get_txt(self):
        with open(self.file, "r") as f:
            self.data = f.readlines()

    def process_file(self):
        with open("test.txt", "w") as f:
            for line in self.data:
                line = line.replace("DW(  1)",  "HDWord::new(&bytes[5..]).value;")
                line = line.replace("W(  1)",   "Word::new(&bytes[0..2]).value as u32;")
                line = line.replace("BCD(  1)", "BCDWord::new(&bytes[5]).value;")
                line = line.replace("C(  1)",   "HByte::new(&bytes[5]).value;")
                line = line.replace("BCD(  2)", "BCDWord::new(&bytes[5]).value;")
                f.write(line)
        

if __name__ == "__main__":
    p = Parser(data)
    p.process_file()