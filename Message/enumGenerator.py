from cppGenerator import CPPEnumClassGenerator 
from rustGenerator import RustEnumClassGenerator

from enum import Enum

class GenerateMode(Enum):
    NONE = 'none'
    CPP = 'cpp'
    RUST = 'rust'


class EnumGenerator:
    def __init__(self) -> None:
        self.generateMode = GenerateMode.NONE
        self.generator = None
        self.fileDir = ''
        self.outputDir = ''
        pass

    def setGenerateMode(self, mode):
        self.generateMode = mode
        pass

    def startGenerate(self):

        if self.generateMode == GenerateMode.CPP:
            self.generator = CPPEnumClassGenerator()
            self.generator.startGenerate(self.fileDir, self.outputDir)
        elif self.generateMode == GenerateMode.RUST:
            self.generator = RustEnumClassGenerator()
            self.generator.startGenerate(self.fileDir, self.outputDir)
        else:
            print("Start Generate Failed ! ! !")

        pass

    





