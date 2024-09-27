#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor; // Vertex color (RGBA)
layout(location = 2) in vec3 normal; 

uniform layout(location = 1) float time;
uniform mat4 transformation_so_far;
uniform mat4 view_projection_matrix;


out layout(location = 0) vec4 fragColor;
out layout(location = 1) vec2 fragCoord;
out layout(location = 2) vec3 fragNormal;




void main()
{
    // gl_Position = model_transformation_matrix * vec4(position, 1.0f);
    gl_Position = view_projection_matrix * vec4(position, 1.0f);

    
    fragColor = vertexColor;
    fragCoord = position.xy;
    fragNormal = normal;
    // fragNormal = model_transformation_matrix * vec4(normal, 1.0f);
    

    
}






















