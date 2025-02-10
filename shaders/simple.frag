#version 430 core

in layout(location=0) smooth vec4 fragColor;
in layout(location=1) vec3 fragCoord;
in layout(location=2) smooth vec3 fragNormal;
uniform layout(location=1)float time;
uniform layout(location=2) bool is_helicopter;
uniform vec3 camera_position;

out vec4 color;




vec3 lightDirection = normalize(vec3(0.8, 0.5, 0.6));
vec3 lightColor = vec3(0.8, 0.8, 0.8); 



void main()
{   
    
    float red   = abs(sin(time * 0.5));  
    float green = abs(sin(time * 0.7));  
    float blue  = abs(sin(time * 0.9));
    
    
    
    vec3 colorvec = vec3(red, green, blue);
    float shininess = 32.0;


    if (is_helicopter){
        
        colorvec = fragColor.xyz;
        shininess = 100.0;
    }
    colorvec = fragColor.xyz;


    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // Diffuse component
    vec3 norm = normalize(fragNormal);  // Normalize the normal vector
    float diff = max(dot(norm, lightDirection), 0.0);
    vec3 diffuse = diff * lightColor;


    float specularStrength = 0.5;
    vec3 viewDir = normalize(camera_position - fragCoord);  // Direction from fragment to camera
    vec3 reflectDir = reflect(-lightDirection, norm);  // Direction of the reflection of the light
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * colorvec;

    color = vec4(result, fragColor.a);

    
    

    


    //vec3 colorWithLight = colorvec* max(0, dot(normalize(fragNormal), -lightDirection));

    //color = vec4(colorWithLight.xyz, fragColor.a);
    

    
}