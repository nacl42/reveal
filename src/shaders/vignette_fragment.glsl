#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;
    
uniform sampler2D Texture;

void DrawVignette( inout vec3 color, vec2 uv )
{    
  float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
  vignette = clamp( pow( 16.0 * vignette, 0.25 ), 0.0, 1.0 );
  color *= vignette;
}

void main() {
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
	
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        res = vec3(0.0, 0.0, 0.0);
    }

    DrawVignette(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
