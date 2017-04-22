#version 410

in vec3 position;
out vec3 currentPosition;

uniform float counter;
uniform mat4 matrix;

void main() {
  //gl_Position = vec4(position.x + (counter / 10), position.y, 0.0, 1.0);
  currentPosition = position;
  gl_Position = matrix * vec4(position, 1.0);
}
