import json

class CPPEnumClassGenerator:
    def __init__(self) -> None:
        pass

    def startGenerate(self, fileDir, outputDir):
        with open(fileDir, "r") as file:
            json_data = json.load(file)
        cpp_code = """enum class MessageType {\n"""

        for message_name, message_info in json_data["Message"].items():
            cpp_code += f"{message_name} = {message_info['ServerBindFunctionUnique']},\n"

        cpp_code += "};"
        
        with open(outputDir, "w") as cpp_file:
            cpp_file.write(cpp_code)
        pass

    def testFunc(self):
        self.startGenerate("example.json", "generated.cpp")
        pass


# TEST
if __name__ == "__main__":
    cppGen = CPPEnumClassGenerator()
    cppGen.testFunc()
