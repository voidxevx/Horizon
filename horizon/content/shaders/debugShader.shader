[version]
330 core

[vertex]
in vec2 location;

uniform mat4 viewMatrix;
uniform mat4 widgetScaleMatrix;
uniform vec3 widgetColor;

out vec3 backgroundColor;

void main() {
    gl_Position = vec4(location, 0.0, 1.0) * widgetScaleMatrix * viewMatrix;
    backgroundColor = widgetColor;
}

[fragment]
in vec3 backgroundColor;

out vec4 FragColor;

void main() {
    FragColor = vec4(backgroundColor, 1.0);
}
