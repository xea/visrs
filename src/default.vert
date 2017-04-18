#version 410

in vec3 position;

uniform float counter;

void main() {
  //gl_Position = vec4(position.x + (counter / 10), position.y, 0.0, 1.0);
  gl_Position = vec4(position, 1.0);
}
