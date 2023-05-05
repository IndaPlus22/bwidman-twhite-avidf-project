#version 150 core

uniform uvec2 resolution;
uniform float dt;

uniform sampler2D density;
uniform sampler2D velocity;
uniform float diffusion;
uniform float viscosity;
uniform float dt;

out vec4 o_Color;


// Helper function to get velocity 
vec2 get_velocity(sampler2D velocity_texture, vec2 uv) {
    return (texture(velocity_texture, uv).rg * 2.0 - 1.0) * resolution;
}

// Advection step
vec4 advect(sampler2D field, sampler2D velocity, vec2 uv, float dt) {
    vec2 velocity_sample = get_velocity(velocity, uv);
    vec2 back_trace = uv - dt * velocity_sample;
    return texture(field, back_trace);
}

void main() {

    // o_Color = vec4(gl_FragCoord.xy / resolution, 0.0, 0.0);
    o_Color = texture2D(density, gl_FragCoord.xy);

    vec2 uv = gl_FragCoord.xy / resolution;

    // Load density and velocity values from textures
    vec4 density = texture(density, uv);
    vec2 velocity = get_velocity(velocity, uv);

    // Update density and velocity values
    vec4 new_density = advect(density, velocity, uv, dt);
    vec2 new_velocity = get_velocity(velocity, uv); // update velocity

    // Output updated density and velocity values
    o_Color = vec4(new_density.rgb, 1.0); // Replace with updated velocity 
}




