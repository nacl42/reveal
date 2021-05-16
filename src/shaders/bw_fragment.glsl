#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;
    
uniform sampler2D Texture;

void main() {
  vec3 res = texture2D(Texture, uv).rgb * color.rgb;
  float luminance = 0.299 * res.r + 0.587 * res.g + 0.114 * res.b;
  gl_FragColor = vec4(luminance, luminance, luminance, 1.0);
}
