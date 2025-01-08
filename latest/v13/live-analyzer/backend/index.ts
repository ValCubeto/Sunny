// GET / -> index.html
// GET /analyze -> passes the code and returns the output of ../../target/debug/v13.exe

import { createServer } from "http"
import { readFileSync, writeFileSync } from "fs"
import { spawn } from "child_process"

const server = createServer((req, res) => {
  console.info(`GET ${req.url}`)
  if (req.url === "/") {
    res.writeHead(200, { "Content-Type": "text/html" })
    res.write(readFileSync("../frontend/index.html"))
    res.end()
  } else if (req.url === "/index.js") {
    res.writeHead(200, { "Content-Type": "text/javascript" })
    res.write(readFileSync("../frontend/index.js"))
    res.end()
  } else if (req.url === "/analyze") {
    let input = ""
    req.on("data", (chunk) => {
      input += chunk.toString()
    })
    req.on("end", () => {
      console.info(`input[${input.length}] = ${input}`)
      writeFileSync("./input.sny", input)
      const child = spawn("../../target/debug/v13.exe", ["run", "--no-color", "input.sny"])
      res.writeHead(200, { "Content-Type": "text/plain" })
      let buffer = Buffer.alloc(2048)
      child.stdout.on("data", (chunk) => {
        buffer.write(chunk.toString())
      })
      child.stderr.on("data", (chunk) => {
        buffer.write(chunk.toString())
      })
      child.on("close", (code) => {
        console.info(`child exited with code ${code}`)
        let output = buffer.toString().slice(0, buffer.length)
        console.info(`output[${output.length}] = ${output}`)
        res.write(output)
        res.end()
        console.info("Request ended")
      })
    })
  }
})

server.listen(8080, "127.0.0.1")
console.info("Server started on http://127.0.0.1:8080")
console.info("Listening...")
