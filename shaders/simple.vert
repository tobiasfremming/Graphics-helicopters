#version 430 core

layout(location = 0) in vec3 position;
out vec2 fragCoord; // Pass fragment coordinates to the fragment shader


void main()
{
    
    gl_Position = vec4(position, 1.0f);

    fragCoord = position.xy;
    
}




















