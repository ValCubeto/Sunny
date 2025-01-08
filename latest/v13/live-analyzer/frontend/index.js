let input = document.getElementById("input")
let output = document.getElementById("output")

input.addEventListener("input", (_event) => {
  console.info(`POST /analyze { ${input.value} }`)
  fetch("/analyze", {
    method: "POST",
    headers: {
      "Content-Type": "text/plain"
    },
    body: input.value
  })
    .then((response) => response.text())
    .then((text) => {
      console.info(`output[${text.length}] = ${text}`)
      output.innerText = text
    })
})
