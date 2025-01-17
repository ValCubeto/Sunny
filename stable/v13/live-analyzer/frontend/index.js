let input = document.getElementById("input")
let output = document.getElementById("output")

input.addEventListener("input", () => {
  const options = {
    method: "POST",
    headers: {
      "Content-Type": "text/plain"
    },
    body: input.value
  }
  fetch("/analyze", options)
    .then((response) => response.text())
    .then((text) => {
      output.innerHTML = text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/ /g, "&nbsp;")
        .split("\n")
        .map((line) => `<p>${line}</p>`)
        .join("")
    })
})
