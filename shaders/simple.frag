#version 430 core

in layout(location=0) smooth vec4 fragColor;
in layout(location=1) vec3 fragCoord;
in layout(location=2) vec3 fragNormal;
uniform layout(location=1)float time;
uniform layout(location=2) bool is_helicopter;

out vec4 color;




vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));




void main()
{   
    
    float red   = abs(sin(time * 0.5));  
    float green = abs(sin(time * 0.7));  
    float blue  = abs(sin(time * 0.9));
    
    
    
    vec3 colorvec = vec3(red, green, blue);

    if (is_helicopter){
        
        colorvec = fragColor.xyz;
    }
    
    colorvec = fragColor.xyz;

    


    vec3 colorWithLight = colorvec* max(0, dot(normalize(fragNormal), -lightDirection));

    color = vec4(colorWithLight.xyz, fragColor.a);
    

    
}