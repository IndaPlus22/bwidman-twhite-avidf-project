#version 430 core

layout(location = 0) uniform uvec2 resolution;
layout(location = 1) uniform float dt;

layout(location = 2) uniform sampler2D density;
layout(location = 3) uniform sampler2D velocity;
layout(location = 4) uniform float diffusion;
layout(location = 5) uniform float viscosity;

layout(location = 0) out vec4 o_Color;
layout(location = 1) out vec4 o_density;
layout(location = 2) out vec4 o_velocity;

void main() {
    o_Color = vec4(gl_FragCoord.xy / resolution, 0.0, 1.0);
    // vec4 current_density = texture(density, gl_FragCoord.xy / resolution);
    // o_Color = vec4(current_density.xyz, 1.0);
    o_density = vec4(1.0, 0.0, 0.0, 1.0);
    // o_velocity = vec4(0.0, 1.0, 0.0, 1.0);
}