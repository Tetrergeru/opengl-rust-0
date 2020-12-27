#version 330 core

in VS_OUTPUT {
    vec3 Position;
    vec3 Color;
    vec2 TextureCoords;
    vec3 Normal;
} IN;

out vec4 Color;

uniform sampler2D Texture;

struct LightData {
    vec3 location;
    float ambient;
    float diffuse;
    float specular;
};

#define LIGHT_COUNT 3

uniform struct LightData[LIGHT_COUNT] Light;

float calculate_brightness() {
    vec3 normal = normalize(IN.Normal);
    float brightness = 0.0;
    for (int i = 0; i < LIGHT_COUNT; ++i) {
        vec3 light_vec = normalize(Light[i].location - IN.Position);
        float new_brightness = 
              Light[i].ambient
            + Light[i].diffuse * max(dot(normal, light_vec), 0.0) 
            + Light[i].specular * pow(dot(normal, ((normal * (2.0 * dot(normal, light_vec))) - light_vec)), 80.0)
        ;
        if (new_brightness > brightness) {
            brightness = new_brightness;
        }
    }
    return brightness;
}

void main() {
    float cl_coeff = 0.001;
    float tx_coeff = 0.999;
    float brightness = calculate_brightness();
    Color = (vec4(IN.Color, 1.0f) * cl_coeff + texture(Texture, IN.TextureCoords) * tx_coeff) * brightness;
}