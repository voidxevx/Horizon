[version]
330 core

[vertex]
layout (location = 0) in vec3 loc;

void main() {
    gl_Position = vec4(loc, 1.0);
}

[fragment]
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}