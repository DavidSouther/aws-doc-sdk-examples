
def instructions(lang_config, customs: dict[str, str], hello: list):
    instruction_blocks = [h2("Instructions")]

    if lang_config['name'] == 'JavaScript' and lang_config['sdk_ver'] == 3:
        instruction_blocks.append(js_instructions())
    if lang_config['name'] == '.NET' and lang_config['sdk_ver'] == 3:
        instruction_blocks.append(dotnet_3_instructions())
    if lang_config['name'] == 'C++':
        instruction_blocks.append(cpp_instructions())
    if lang_config['name'] == 'swfit':
        instruction_blocks.append(swift_instructions())

    for hello_ex in hello:
        instruction_blocks.append(hello_ex(hello))


    instruction_blocks.append(comment_block("custom.instructions", customs["instructions"]))

def js_instructions():
    return block(
        para(
        strong("Note"),
    ": All code examples are written in ECMAscript 6 (ES6). For guidelines on converting to CommonJS, see",
    link(title="JavaScript ES6/CommonJS syntax", href="https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/sdk-examples-javascript-syntax.html"), "."
        ),
        block(
        para(strong("Run a single action")),
        code_block(lang="bash", "node ./actions/<fileName>"),
        ),
        block(
        para(strong("Run a scenario")),
        code_block(lang="bash", "node ./scenarios/<fileName>"),
        )
    )

def dotnet_3_instructions():
    return block(
        para(
        "For general instructions to run the examples, see the ",
        link(title="README", href="../README.md#building-and-running-the-code-examples")
        " in the `dotnetv3` folder."
        ),
        para("Some projects might include a settings.json file. Before compiling the project, you can change these values to match your own account and resources. Alternatively, add a settings.local.json file with your local settings, which will be loaded automatically when the application runs."),
        para("After the example compiles, you can run it from the command line. To do so, navigate to the folder that contains the .csproj file and run the following command:"),
        code_block(language="sh", "dotnet_run"),
        para("Alternatively, you can run the example from within your IDE.")
    )


def cpp_instructions():
    return block(
        para("""An executable is built for each source file in this folder. These executables are located in the build folder and have "run_" prepended to the source file name, minus the suffix. See the "main" function in the source file for further instructions."""),
        para("""For example, to run the action in the source file "my_action.cpp", execute the following command from within the build folder. The command will display any required arguments."""),
        code_block(language="bash", "./run_my_action"),
        para("""If the source file is in a different folder, instructions can be found in the README in that folder."""),
    )


def swift_instructions():
    return block(
        para("To build any of these examples from a terminal window, navigate into its directory, then use the following command:"),
        code_block(lanuage="bash", "$ swift build"),
        para("To build one of these examples in Xcode, navigate to the example's directory (such as the `ListUsers` directory, to build that example). Then type `xed.` to open the example directory in Xcode. You can then use standard Xcode build and run commands."),
    )



def hello_instructions(lang_config, example):
    language_instructions = None
    if lang_config['name'] == 'Go' and lang_config['sdk_ver'] == 2:
        language_instructions = "go run ./hello"
    if lang_config['name'] == 'Python' and lang_config['sdk_ver'] == 3:
        language_instructions = f"python {example['run_file']}"
    if lang_config['name'] == 'JavaScript' and lang_config['sdk_ver'] == 3:
        language_instructions = f"node ./hello.js"
    if lang_config['name'] == 'Ruby' and lang_config['sdk_ver'] == 3:
        language_instructions = f"ruby {example['run_file']}"
    
    language_instructions = code_block(language_instructions) if language_instructions is not None

    return block(
        h4(example['title_abbrev']),
        para(f"This example shows you how to {example['synopsis']}"),
        language_instructions
    )


def scenarios(lang_config, scenario_examples, customs):
    scenario_blocks = []

    if lang_config['name'] == 'Go' and lang_config['sdk_ver'] == 2:
        scenario_blocks.append(block(
            h4("Run a scenario"),
            para("All scenarios can be run with the `cmd` runner. To get a list of scenarios and to get help for running a scenario, use the following command:"),
            code_block("go run ./cmd -h")
        ))

    for scenario in scenarios_examples:
        scenario_blocks.append(h4(scenario['title_abbrev']))

        if scenario['synopsis']:
            scenario_blocks.append(para(f"This example shows you how to {scenario['synopsis']}"))
        else:
            scenario_blocks.append(para("This example shows you how to do the following:"))
        
        scenario_blocks.append(unordered_list([item(syn) for syn in scenario['synopsis_list']]))
        scenario_custom = customs.get('scenario_prereqs', {}).get(scenario['id'], "")
        scenario_blocks.append(f"cusom.scenario_prereqs.{scenario['id']}", scenario_custom)

        if lang_config['name'] == 'Python' and lang_config['sdk_ver'] == 3 and scenario["file"].endswith(".py"):
            scenario_blocks.append(para("Start the example by running the following at a command prompt:"))
            scenario_blocks.append(code_block(f"python {scenario['file']}"))
        elif lang_config['name'] == 'Ruby' and lang_config['sdk_ver'] == 3:
            scenario_blocks.append(para("Start the example by running the following at a command prompt:"))
            scenario_blocks.append(code_block(f"ruby {scenario['file']}"))
        customs_scenarios = customs.get('scenarios', {}).get(scenario['id'], "")
        scenario_blocks.append(customs_scenarios)
