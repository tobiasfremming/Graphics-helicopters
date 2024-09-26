#version 430 core

in layout(location=0) smooth vec4 fragColor;
in layout(location=1) vec2 fragCoord;
in layout(location=2) vec3 fragNormal;
uniform float time;

out vec4 color;

vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));







void main()
{   
    

    
    
    vec4 colorvec = vec4(1.0, 1,0.933, 1.0);
    //float red   = abs(sin(time * 0.5));  
    //float green = abs(sin(time * 0.7));  
    //float blue  = abs(sin(time * 0.9));
    //vec4 colorvec = vec4(red, green, blue, 1.0);

    
    

    vec4 colorWithLight = fragColor * max(0, dot(fragNormal, -lightDirection));
    color = vec4(colorWithLight.xyz, fragColor.a);
    

    
}