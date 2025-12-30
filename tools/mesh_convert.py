#!/bin/python3
"""
This script parses .obj file to a mesh file for use in the engine.

NOTE: only works for meshes that are trianglualated
"""
import sys, os
from pathlib import Path

VALID_NAMES = ["v", "f", "vt"]

def convert_obj_to_mesh(txt: str) -> str:
    tex_coords: str = ""
    indices: str = ""
    vertices: str = ""
    for l in txt.splitlines():
        data = l.strip().split()
        if len(data) == 0:
            continue
        name = data[0]
        props = data[1:]
        match name:
            case "v": # vertices
                vertices += f"{props[0]} {props[1]} {props[2]}\n"
            case "vt": # texture coordinate
                tex_coords += f"{props[0]} {props[1]}\n"
            case "f": # indices
                for p in props:
                    indices += f"{int(p.split("/")[0]) - 1} "

    b = f""":Vertices
{vertices}
:Indices
{indices}
:TexCoord
{tex_coords}
:Color
"""
    return b
        

def convert_obj_to_mesh_file(path: str) -> str:
    cont = ""
    with open(path, "r") as f:
        cont = f.read()
    return convert_obj_to_mesh(cont)

if __name__ == "__main__":
    args = sys.argv[1:]
    input_args: list[str] = []
    output_args: list[str] = []
    i = 0
    output_flag_enabled = False
    while i < len(args):
        arg = args[i]
        if output_flag_enabled or arg == "-o": 
            if arg == "-o":
                output_flag_enabled = True
                i += 1
            next = args[i]
            output_args.append(next)
            i += 1
            continue
        input_args.append(arg)
        i += 1
    is_single = len(output_args) == 1
    if not is_single and len(output_args) != len(input_args):
        raise Exception("invalid output: output should be equal to inputs or be one")
    is_directory = is_single and len(output_args) == 1 # if False, then every output has a coreposonding input
    
    for i, input_path in enumerate(input_args):
        data = convert_obj_to_mesh_file(input_path)
        if is_single:
            with open(output_args[0], "w") as f:
                f.write(data)
        elif is_directory:
            path = os.path.join(output_args[0], Path(input_path).stem + ".mesh");
            with open(path, "w") as f:
                f.write(data)
        else:
            with open(output_args[i], "w") as f:
                f.write(data)

            

