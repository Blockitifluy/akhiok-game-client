#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aCoord;

out vec4 color;
out vec2 coord;

void main() {
    gl_Position = vec4(aPos, 1.0);
    color = vec4(aColor, 1.0);
    coord = aCoord;
}

