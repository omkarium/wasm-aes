import init, { run } from "./wasm_rufendec.js";
init();

document.querySelector('#clickme').addEventListener("click", () => {
  document.getElementById('output').value = 
    run(document.getElementById('data').value, 
        document.getElementById('password').value, 
        document.getElementById('salt').value, 
        document.getElementById('operation').value,
        document.getElementById('mode').value, 
        document.getElementById('iterations').value);
});