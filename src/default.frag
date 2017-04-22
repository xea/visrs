#version 410

in vec3 currentPosition;
out vec4 color;

uniform float counter;
uniform uint milliseconds;

vec4 gradientColor(vec3 pos) {
  return vec4(currentPosition.x, currentPosition.y, 0.0, 1.0);
}

vec4 depthColor(vec3 pos) {
  return vec4(pos.x + 0.5, pos.y + 0.5, pos.z + 0.5, 1.0);
}

void main() {
  color = depthColor(currentPosition);
  //  color = vec4(sin(counter), sin(counter), sin(counter / 2), 1.0);
}

