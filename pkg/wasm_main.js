import init, { run, run_files } from "./wasm_aes.js";
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

let raw_output_bytes;
document.querySelector('#execute').addEventListener("click", () => {
  raw_output_bytes = run_files(bytes, 
    document.getElementById('password2').value, 
    document.getElementById('salt2').value, 
    document.getElementById('operation2').value,
    document.getElementById('mode2').value, 
    document.getElementById('iterations2').value);
    var datetime = new Date().toLocaleString();
    if (raw_output_bytes[0] == 0) {
      document.getElementById('output2').textContent = datetime + " :::  Failed the operation";
    } else {
      document.getElementById('output2').textContent = datetime + " :::  Operation success. Please download the file now";
    }  

});

document.querySelector('#download').addEventListener("click", () => {
  var bytes_to_blob = new Uint8Array(raw_output_bytes);
  var operation = document.getElementById('operation2').value;
  if (operation == "Encrypt") {
    var file_name = filename + ".enom";
  }

  if (operation == "Decrypt") {
    var file_name = filename.replace(/.enom[\.a-zA-Z]+/, "")
  }
  downloadRawBytes(file_name, bytes_to_blob);
});


function downloadRawBytes(file, bytes) {
  // Create a Blob from the raw bytes
  var blob = new Blob([bytes]);

  // Create an invisible element
  var element = document.createElement('a');
  
  var url = URL.createObjectURL(blob);
  
  var dataUrl = 'data:attachment/octet-stream,' + encodeURIComponent(bytes);

  // Set the href attribute to a URL representing the Blob
  element.setAttribute('href', dataUrl);

  // Set the download attribute to the desired filename
  element.setAttribute('download', file);

  // Append the element to the document body
  document.body.appendChild(element);

  // Simulate a click on the element to trigger the download
  element.click();

  // Clean up: remove the element and the temporary URL
  document.body.removeChild(element);
  URL.revokeObjectURL(element.href);
}
