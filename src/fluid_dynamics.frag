#version 330 core

uniform uvec2 resolution;
uniform float dt;

uniform sampler2D density;
uniform sampler2D velocity;
uniform float diffusion;
uniform float viscosity;

layout(location = 0) out vec4 o_Color;
layout(location = 1) out vec4 o_density;
layout(location = 2) out vec4 o_velocity;

void main() {
    o_Color = vec4(gl_FragCoord.xy / resolution, 0.0, 1.0);
    // vec4 current_density = texture(density, gl_FragCoord.xy / resolution);
    // o_Color = vec4(current_density.xyz, 1.0);
    o_density = vec4(1.0);
}