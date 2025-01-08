let input = document.getElementById("input")
let output = document.getElementById("output")

input.addEventListener("input", (_event) => {
  fetch("/analyze", {
    method: "POST",
    headers: {
      "Content-Type": "text/plain"
    },
    body: input.value
  })
    .then((response) => response.text())
    .then((text) => {
      output.innerHTML = text
        .replace(/ /g, "&nbsp;")
        .replace(/\n/g, "<br>")
    })
})
