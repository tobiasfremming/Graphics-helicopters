#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor; // Vertex color (RGBA)
uniform float time;
uniform mat4 transformation_matrix;

out vec4 fragColor;
out vec2 fragCoord;




void main()
{

    gl_Position = transformation_matrix * vec4(position, 1.0f);

    
    fragColor = vertexColor;
    fragCoord = position.xy;
    

    
}






















