#version 330 core

in VS_OUTPUT {
    vec3 Position;
    vec3 Color;
    vec2 TextureCoords;
    vec3 Normal;
} IN;

out vec4 Color;

uniform sampler2D Texture;
uniform sampler2D Texture_2;

uniform float color_coeff;
uniform float texture_coeff;

struct LightData {
    int mode; // 0 - off, 1 - point, 2 - projector
    vec3 location;
    vec3 direction; // for projector only
    float ambient;
    float diffuse;
    float specular;
};

#define LIGHT_COUNT 3

uniform struct LightData[LIGHT_COUNT] Light;

float calculate_brightness() {
    vec3 normal = normalize(IN.Normal);

    float brightness = 0.0;
    int active_lights = 0;
    for (int i = 0; i < LIGHT_COUNT; ++i)
    {
        vec3 light_vec = normalize(Light[i].location - IN.Position);
        float new_brightness = 0.0;
        
        switch (Light[i].mode)
        {
            case 1:
                active_lights += 1;
                new_brightness = Light[i].ambient
                    + Light[i].diffuse * max(dot(normal, light_vec), 0.0) 
                    + Light[i].specular * max(pow(dot(normal, ((normal * (2.0 * dot(normal, light_vec))) - light_vec)), 80.0), 0.0);
                break;
            case 2:
                active_lights += 1;
                float ambient = Light[i].ambient;

                float diff = max(dot(normal, light_vec),0.0);
                float diffuse = Light[i].diffuse * diff;

                vec3 halfwayDir = normalize(light_vec/* + viewDir */);
                float spec = pow(max(dot(halfwayDir,normal), 0.0),32);
                float specular = Light[i].specular * spec;

                float theta = dot(light_vec, normalize(-Light[i].direction));
                float epsilon = cos(0.3) - cos(0.6);
                float intensity = clamp((theta - cos(0.6)) / epsilon, 0.0, 1.0);
                
                new_brightness = intensity * (ambient + diffuse + specular);
                break;
            default:
                continue;
        } 

        brightness += new_brightness;
    }
    return brightness / float(active_lights);
}

void main() {
    float brightness = calculate_brightness();
    Color = mix(vec4(IN.Color, 1.0f), mix(texture(Texture, IN.TextureCoords), texture(Texture_2, IN.TextureCoords), texture_coeff) * brightness, color_coeff);
}