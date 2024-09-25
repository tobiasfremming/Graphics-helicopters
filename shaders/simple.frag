#version 430 core

in layout(location=0) smooth vec4 fragColor;
in layout(location=2) vec3 fragNormal;
uniform float time;

out vec4 color;

vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

void main()
{
    
    
    vec4 colorWithLight = fragColor * max(0, dot(fragNormal, -lightDirection));
    color = vec4(colorWithLight.xyz, fragColor.a);

    
}