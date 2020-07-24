import { tokenize } from "tinysegmenter-wasm";

const verbose = true;

document.getElementById("submit-text").onclick = function() {
  const elm = document.getElementById("tokenize-result");
  const text = document.getElementById("input-text").value
  if (verbose) {
    console.log("will tokenize text: " + text)
  }
  elm.textContent = tokenize(text)
};