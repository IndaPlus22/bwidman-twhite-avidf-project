#version 150 core

uniform sampler2D t_density;
uniform sampler2D t_velocity;
uniform float t_diffusion;
uniform float t_viscosity;
out vec4 o_Color;

void main() {
    o_Color = vec4(gl_FragCoord.xy / 512, 0.0, 0.0);
    // o_Color = texture2D(t_density, gl_FragCoord.xy);
}