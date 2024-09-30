#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor; // Vertex color (RGBA)
layout(location = 2) in vec3 normal; 

uniform layout(location = 1) float time;
uniform layout(location = 2) bool is_helicopter;
uniform mat3 transformation_so_far;
uniform mat4 view_projection_matrix;


out layout(location = 0) vec4 fragColor;
out layout(location = 1) vec3 fragCoord;
out layout(location = 2) vec3 fragNormal;




void main()
{
    //fragNormal = transformation_so_far * vec4(position, 1.0f);
    gl_Position = view_projection_matrix * vec4(position, 1.0f);
    
    fragColor = vertexColor;
    fragCoord = position.xyz;
    //fragNormal = normal;
    //fragNormal = transformation_so_far * vec4(normal, 0.0);

    fragNormal = transformation_so_far * normal;    

    
}






















