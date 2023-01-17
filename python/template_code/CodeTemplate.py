import json
from copy import deepcopy

class CodeTemplate:
    file_path = ""
    variables = {}
    template_code = ""
    output_code = ""

    def __init__(self, file_path):
        self.file_path = file_path
        self.read_file()

    def read_file(self):
        with open(self.file_path + ".json", 'r') as source:
            vars = json.load(source)
            self.variables = vars["variables"]

        with open(self.file_path + ".txt", 'r') as source:
            self.template_code = source.read()

    def set_variables(self, **kwargs):
        for arg in kwargs:
            if self.variables.has_key(arg):
                self.variables[arg] = kwargs[arg]
            else:
                print("{} is not a variable in the template code".format(arg))

    def insert_variables(self):
        self.output_code = deepcopy(self.template_code)
        for var in self.variables:
            placeholder = "${" + var + "}"
            self.output_code = self.output_code.replace(placeholder, str(self.variables[var]))


    def __str__(self):
        print(self.template_code)
        return "\n"

        
if __name__ == "__main__":
    myTemplate = CodeTemplate("templates/verilog_mult")
    myTemplate.set_variables(IN1_WIDTH = 32, IN2_WIDTH = 32, OUT_WIDTH = 64)
    myTemplate.insert_variables()
    print(myTemplate)
    with open("/home/kc319/Documents/FYP_EGraphs_For_FPGAs/vivado/src/mult.v", 'w') as os:
        os.write(myTemplate.output_code)

    