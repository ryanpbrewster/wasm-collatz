function collatz_len(n) {
  let count = 0;
  while (n > 1) {
    n = n % 2 == 0 ? n / 2 : 3 * n + 1;
    count++;
  }
  return count;
}

function collatz_max(hi) {
  let best = 1;
  let best_score = 0;
  for (let n=2; n <= hi; n++) {
    let score = collatz_len(n);
    if (score > best_score) {
      best = n;
      best_score = score;
    }
  }
  return best;
}

window.onload = function() {
  const dom_seed = document.getElementById("seed");
  const dom_js_output = document.getElementById("js-output");
  const dom_wasm_output = document.getElementById("wasm-output");

  import("../crate/pkg").then(module => {
    console.log("loaded module");
    setInterval(function() {
      let n = parseInt(dom_seed.value) || 3;
      dom_js_output.value = collatz_max(n);
      dom_wasm_output.value = module.collatz_max(n);
      dom_seed.value = n + 1;
    }, 0);
  });
};
