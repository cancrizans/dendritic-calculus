message = "Hello, world!"
ords = list(map(ord,message))
program = []

for i in range(8):
    terms = []
    for e,char in enumerate(ords):
        dig = f"{char:08b}"[i]
        if dig == '1':
            exponent = '+'.join(["[]"]*e) if e > 0 else ""
            terms.append("[" + exponent + "]")
        else:
            assert dig == '0'
    if len(terms) > 0:
        program.append("+="+ '+'.join(terms))
    program.append("/[]>[]+[]")

program.pop()
from os import path
with open(path.join(path.dirname(__file__),"nodigits_hello_world.dc"),'w') as f:
    f.write("".join(program))