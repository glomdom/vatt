@vs vs
layout(set = 0, binding = 0) uniform UBO {
    vec2 resolution;
} ubo;

in vec2 position;
in vec2 texcoord;

out vec2 uv;

void main() {
    vec2 pixel_pos = floor(position * ubo.resolution);
    vec2 clip_pos = (pixel_pos * 2.0 - ubo.resolution) / ubo.resolution;
    gl_Position = vec4(clip_pos, 0.0, 1.0);
    uv = texcoord;
}
@end

@fs fs
layout(set = 0, binding = 1) uniform texture2D texImage;
layout(set = 0, binding = 2) uniform sampler texSampler;

in vec2 uv;
out vec4 frag_color;

void main() {
    frag_color = texture(sampler2D(texImage, texSampler), uv);
    if(frag_color.a < 0.5) discard;
}
@end


@program simple vs fs