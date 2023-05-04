#version 150 core

uniform uvec2 resolution;
uniform float dt;

uniform sampler2D density;
uniform sampler2D velocity;
uniform float diffusion;
uniform float viscosity;

out vec4 o_Color;

void main() {
    // o_Color = vec4(gl_FragCoord.xy / resolution, 0.0, 0.0);
    o_Color = texture2D(density, gl_FragCoord.xy);
}