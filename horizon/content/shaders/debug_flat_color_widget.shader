[version]
330 core

[vertex]
in vec2 vertPos;
in vec2 vertTexCoord; // consumed for widgets that actually have a texture binding.
uniform mat4 viewMatrix;
uniform vec4 wigetColor;

out vec4 backgroundColor;

void main()
{
    gl_Position = vec4(vertPos, 1.0, 1.0) * viewMatrix;
    backgroundColor = widgetColor;
}

[fragment]
in vec4 backgroundColor;

out vec4 FragColor;

void main()
{
    FragColor = backgroundColor;
}