#version 430 core

in layout(location=0) smooth vec4 fragColor;
in layout(location=1) vec2 fragCoord;
in layout(location=2) vec4 fragNormal;
uniform layout(location=1)float time;
uniform layout(location=2) bool is_helicopter;

out vec4 color;




vec4 lightDirection = normalize(vec4(0.8, -0.5, 0.6, 0.0));




void main()
{   
    
    float red   = abs(sin(time * 0.5));  
    float green = abs(sin(time * 0.7));  
    float blue  = abs(sin(time * 0.9));
    
    
    vec4 colorvec = fragColor;
    
  
    if (is_helicopter){
        colorvec = vec4(red, green, blue, 1.0);
    }


    vec4 colorWithLight = colorvec * max(0, dot(normalize(fragNormal), -lightDirection));

    color = vec4(colorWithLight.xyz, fragColor.a);
    

    
}