class Dir:
    def __init__(self, name, parent=None):
        self.name = name
        self.parent = parent
        self.dirs = []
        self.files = []

    @staticmethod
    def root():
        res = Dir("/")
        res.parent = res
        return res

    def get_subdir(self, name):
        for dir in self.dirs:
            if dir.name == name:
                return dir

    def add_subdir(self, name):
        self.dirs.append(Dir(name, self))
    
    def add_subfile(self, name, size):
        self.files.append(File(name, size))

    def get_size(self):
        sum = 0
        for dir in self.dirs:
            sum += dir.get_size()
        for file in self.files:
            sum += file.size
        return sum

    def all_dirs(self):
        for dir in self.dirs:
            yield dir
            yield from dir.all_dirs()

class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

ROOT = Dir.root()

def main(file):
    current = ROOT
    for line in file:
        tokens = line.strip().split(' ')
        if tokens[0] == '$':
            if tokens[1] == 'ls':
                pass
            elif tokens[1] == 'cd':
                if tokens[2] == '/':
                    current = ROOT
                elif tokens[2] == '..':
                    current = current.parent
                elif tokens[2] == '.':
                    current = current
                else:
                    current = current.get_subdir(tokens[2])
        elif tokens[0] == 'dir':
            current.add_subdir(tokens[1])
        else:
            size = int(tokens[0])
            current.add_subfile(tokens[1], size)
    
    print("total size: ", ROOT.get_size())

    # now, part 1:
    sum = 0
    for dir in ROOT.all_dirs():
        if dir.get_size() < 100000:
            sum += dir.get_size()
    print("part 1: ", sum)
                

if __name__ == "__main__":
    import sys
    main(sys.stdin)