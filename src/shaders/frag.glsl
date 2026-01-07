#version 330 core
uniform vec3 obj_color;
uniform sampler2D texture0;

in vec2 coord;

out vec4 FragColor;

void main() {
  FragColor = texture(texture0, coord) * vec4(obj_color, 1.0);
}
