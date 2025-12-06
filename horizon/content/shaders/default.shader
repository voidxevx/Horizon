[version]
330 core

[vertex]
in vec3 loc;
in vec2 vertTexCoords;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

out vec2 texCoord;

void main() {
    gl_Position = vec4(loc, 1.0) * projectionMatrix * viewMatrix;
    texCoord = vertTexCoords;
}

[fragment]
in vec2 texCoord;
uniform sampler2D texture0;

out vec4 FragColor;

void main() {
    FragColor = texture(texture0, texCoord);
}