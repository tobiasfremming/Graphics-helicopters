#version 430 core

out vec4 color;
in vec2 fragCoord; // The fragment's position, to be passed from the vertex shader
uniform float time;

void main()
{
    float red   = abs(sin(time * 0.5));  
    float green = abs(sin(time * 0.7));  
    float blue  = abs(sin(time * 0.9));

    
    float dist = length(fragCoord - vec2(0.5, 0.5));    
    float scale = 1.0;  
    
     // angle (theta) of the fragment (in radians)
    float angle = atan(fragCoord.y - 0.5, fragCoord.x - 0.5); 

    // The scale factor gives the number of spirals
    float spiralFactor = 10.0; 

    
    float spiral = sin(spiralFactor * angle + dist * 20.0); 

    //the y value for the sine function
    float sineValue = sin(fragCoord.x * 3.14159);
    float threshold = 0.02; // line width

    // Use the spiral effect to set the color, making a repeating pattern
    if (spiral > 0.0) {
        color = vec4(red, green, blue, 1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 1.0); 
    }
    
}