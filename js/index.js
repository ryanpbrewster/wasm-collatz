function collatz_len(n) {
  let count = 0;
  while (n > 1) {
    n = n % 2 == 0 ? n / 2 : 3 * n + 1;
    count++;
  }
  return count;
}

window.onload = function() {
  const dom_seed = document.getElementById("seed");
  const dom_compute = document.getElementById("compute");
  const dom_js_output = document.getElementById("js-output");
  const dom_wasm_output = document.getElementById("wasm-output");

  import("../crate/pkg").then(module => {
    console.log("loaded module");
    dom_compute.addEventListener("click", evt => {
      let n = parseInt(dom_seed.value) || 3;
      dom_js_output.value = collatz_len(n);
      dom_wasm_output.value = module.collatz_len(n);
    });
  });
};
