#version 410

uniform float iGlobalTime;
uniform vec3 iResolution;
uniform vec4 iMouse;
out vec4 color;

// ----------

float sdSphere(vec3 p, float s) {
	return length(p) - s;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
	vec2 uv = fragCoord.xy / iResolution.xy;
	fragColor = vec4(uv, 0.5 + 0.5, 1.0); //* sin(iGlobalTime), 1.0);
    //fragColor = vec4(uv.x, uv.y + sin(fragCoord.y), 1.0, 1.0);

//	fragColor = vec4(sin(fragCoord.x / 60), cos(fragCoord.y / 60), 0.5, 1.0);
}

// ----------

void main() {
	mainImage(color, gl_FragCoord.xy);
}
