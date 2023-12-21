
from dataclasses import dataclass
from typing import Dict, List

#167409079868000
#256000000000000



@dataclass
class Part:
    x: int
    m: int
    a: int
    s: int

    def get_sum(self) -> int:
        return self.x + self.m + self.a + self.s 

@dataclass
class Condition:
    part_name: str
    value: int
    op: str
    goto: str



def go(ranges, wf_name) -> bool:

    if wf_name == "R":
        return 0
    
    if wf_name == "A":
        product = 1
        for start, end in ranges.values():
            product *= end - start + 1
        return product

    rules = workflows[wf_name]
    default = rules[-1].goto
    rules = rules[:-1]

    total = 0
    is_false = False
    for rule in rules:
        start, end = ranges[rule.part_name]
        if rule.op == "<":
            rule_t = (start, rule.value - 1)
            rule_f = (rule.value, end)
        else:
            rule_t = (rule.value + 1, end)
            rule_f = (start, rule.value)
        
        if rule_t[0] <= rule_t[1]:
            ranges_c = ranges.copy()
            ranges_c[rule.part_name] = rule_t
            total += go(ranges_c, rule.goto)
        
        if rule_f[0] <= rule_f[1]:
            ranges = ranges.copy()
            ranges[rule.part_name] = rule_f
        else:
            is_false = True
            break

    if not is_false:
        total += go(ranges, default)

    return total

def main():
    global workflows
    workflows = {}
    with open("input2.txt") as f:
        lines = f.read().split("\n")

    parts_idx = 0
    for i, line in enumerate(lines):
        if line == "": # end of workflows
            parts_idx = i
            break

        name = line[:line.find("{")]
        commands = line[line.find("{")+1:-1]
        cmds = commands.split(",")
        p_cmds = []
        for cmd in cmds[:-1]:
            cmp, goto = cmd.split(":")
            if ">" in cmp:
                a, v = cmp.split(">")
            else:
                a, v = cmp.split("<")

            c = Condition(
                part_name=a,
                value=int(v),
                op=(">" if ">" in cmp else "<"),
                goto=goto
            )
            p_cmds.append(c)
        p_cmds.append(Condition(
            part_name=None,
            value=None,
            op=None,
            goto=cmds[-1]
        ))

        workflows[name] = p_cmds


    parts = []
    for part in lines[parts_idx+1:]:
        values = part[1:-1].split(",")
        parts.append(
            Part(
                x=int(values[0].split("=")[-1]),
                m=int(values[1].split("=")[-1]),
                a=int(values[2].split("=")[-1]),
                s=int(values[3].split("=")[-1])
            )
        )

    # for k, v in workflows.items():
    #     print(f"{k=} {v}")
    #print(parts)
    ranges = {
        'x': (1, 4000),
        'm': (1, 4000),
        'a': (1, 4000),
        's': (1, 4000)
    }

    ans = go(ranges, "in")

    print(f"{ans=}")

main()