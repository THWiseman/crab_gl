attribute vec3 position;
attribute vec2 offset;

void main(void) {
    gl_Position = vec4(position, 1.0);
}