import json

class RustEnumClassGenerator:
    def __init__(self) -> None:
        pass

    def startGenerate(self, fileDir, outputDir):
        with open(fileDir, "r") as file:
            json_data = json.load(file)
        rust_code = """pub enum MessageType {\n"""

        for message_name, message_info in json_data["Message"].items():
            rust_code += f"    {message_name} = {message_info['ServerBindFunctionUnique']},\n"

        rust_code += "}"

        # Rust 코드를 generated.rs 파일에 저장합니다.
        with open(outputDir, "w") as rust_file:
            rust_file.write(rust_code)
        pass

    def testFunc(self):
        self.startGenerate("example.json", "generated.rs")
        pass


# TEST
if __name__ == "__main__":
    rustGen = RustEnumClassGenerator()
    rustGen.testFunc()
