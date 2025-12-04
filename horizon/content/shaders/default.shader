[version]
330 core

[vertex]
in vec3 loc;
in vec2 vertTexCoords;

out vec2 texCoord;

void main() {
    gl_Position = vec4(loc, 1.0);
    texCoord = vertTexCoords;
}

[fragment]
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D texture0;

void main() {
    FragColor = texture(texture0, texCoord);
}