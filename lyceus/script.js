(() => {
  const canvas = document.getElementById('field');
  const gl = canvas.getContext('webgl', { alpha: true, antialias: true });
  if (!gl) return; // The CSS field and orbital marks remain visible without WebGL.

  const compile = (type, source) => {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) return null;
    return shader;
  };
  const vertex = compile(gl.VERTEX_SHADER, 'attribute vec2 p; void main(){gl_Position=vec4(p,0.,1.);}');
  const fragment = compile(gl.FRAGMENT_SHADER, `
    precision mediump float;
    uniform vec2 resolution;
    uniform float time;
    void main() {
      vec2 p = (gl_FragCoord.xy - resolution * .5) / min(resolution.x, resolution.y);
      float r = length(p);
      float angle = atan(p.y, p.x);
      float core = exp(-r * 9.0);
      float halo = exp(-abs(r - .25) * 10.0) * .25;
      float threads = 0.0;
      for (float i = 0.0; i < 12.0; i++) {
        float a = i * .5236 + time * .045;
        float wave = sin(angle * 3.0 + r * 27.0 - time * .28 + i) * .018;
        float line = abs(sin(angle - a + wave));
        threads += .0013 / (line + .012) * smoothstep(.48, .04, r) * smoothstep(.035, .16, r);
      }
      float rings = pow(max(0.0, cos(r * 95.0 - time * .25)), 12.0) * exp(-abs(r - .23) * 13.0) * .10;
      float light = core * .65 + halo + threads * .34 + rings;
      gl_FragColor = vec4(vec3(.68, .84, .48) * light, clamp(light, 0.0, .9));
    }
  `);
  if (!vertex || !fragment) return;
  const program = gl.createProgram();
  gl.attachShader(program, vertex); gl.attachShader(program, fragment); gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) return;
  gl.useProgram(program);
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1, 1,-1, -1,1, -1,1, 1,-1, 1,1]), gl.STATIC_DRAW);
  const position = gl.getAttribLocation(program, 'p');
  gl.enableVertexAttribArray(position); gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
  const resolution = gl.getUniformLocation(program, 'resolution');
  const time = gl.getUniformLocation(program, 'time');
  const motion = matchMedia('(prefers-reduced-motion: reduce)');
  let frame;
  function draw(now = 0) {
    const scale = Math.min(devicePixelRatio || 1, 2);
    const width = Math.round(canvas.clientWidth * scale), height = Math.round(canvas.clientHeight * scale);
    if (canvas.width !== width || canvas.height !== height) { canvas.width = width; canvas.height = height; }
    gl.viewport(0, 0, width, height);
    gl.uniform2f(resolution, width, height); gl.uniform1f(time, motion.matches ? 0 : now / 1000);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
    if (!motion.matches) frame = requestAnimationFrame(draw);
  }
  function restart() { cancelAnimationFrame(frame); draw(); }
  motion.addEventListener('change', restart);
  window.addEventListener('resize', restart);
  draw();
})();
