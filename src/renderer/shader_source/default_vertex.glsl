attribute vec3 position;
attribute vec2 offset;

varying vec2 v_instance_position;

void main(void) {
    vec2 instance_position = vec2(position.x, position.y) + offset;
    v_instance_position = instance_position;
    gl_Position = vec4(instance_position.x, instance_position.y, position.z, 1.0);
}