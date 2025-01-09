// GET / -> index.html
// GET /analyze -> passes the code and returns the output of ../../target/debug/v13.exe

import { createServer } from "http"
import { createWriteStream, readFileSync, writeFileSync } from "fs"
import { spawn } from "child_process"

const server = createServer((req, res) => {
  if (req.url === "/") {
    console.info(`GET ${req.url}`)
    res.writeHead(200, { "Content-Type": "text/html" })
    res.write(readFileSync("../frontend/index.html"))
    res.end()
  } else if (req.url === "/index.js") {
    console.info(`GET ${req.url}`)
    res.writeHead(200, { "Content-Type": "text/javascript" })
    res.write(readFileSync("../frontend/index.js"))
    res.end()
  } else if (req.url === "/analyze") {
    let input = ""
    req.on("data", (chunk) => {
      input += chunk.toString()
    })
    req.on("end", () => {
      writeFileSync("./input.sny", input)
      const child = spawn("../../target/debug/v13.exe", ["run", "input.sny", "--no-color", "--debug"])
      res.writeHead(200, { "Content-Type": "text/plain" })
      child.stdout.pipe(createWriteStream("./stdout.log"))
      child.stderr.pipe(createWriteStream("./stderr.log"))
      child.on("close", () => {
        let stdout = readFileSync("./stdout.log")
        let stderr = readFileSync("./stderr.log")
        res.write("--STDOUT--\n")
        res.write(stdout)
        res.write("--STDERR--\n")
        res.write(stderr)
        res.end()
      })
    })
  }
})

server.listen(8080, "127.0.0.1")
console.info("Server started on http://127.0.0.1:8080")
console.info("Listening...")
