#version 150 core
in vec3 a_pos;
in vec3 a_color;
in ivec2 a_tex_coord;
out vec3 v_color;
out vec2 v_TexCoord;
uniform mat4 u_model_view_proj;
void main() {
    v_TexCoord = a_tex_coord;
    v_color = a_color;
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}