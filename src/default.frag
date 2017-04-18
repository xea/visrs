#version 410

out vec4 color;

uniform float counter;
uniform uint milliseconds;

void main() {
  color = vec4(sin(milliseconds), cos(milliseconds), sin(milliseconds), 1.0);
}
