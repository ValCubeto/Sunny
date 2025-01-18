import { IncomingMessage, ServerResponse, createServer } from "http"
import { createWriteStream, readFileSync, writeFileSync } from "fs"
import { spawn } from "child_process"
import { AddressInfo } from "net"

const SUNNY_PATH = "../../target/debug/v13.exe"
const INPUT_PATH = "./cache/input.sny"
const STDOUT_PATH = "./cache/stdout.log"
const STDERR_PATH = "./cache/stderr.log"

const server = createServer((req: IncomingMessage, res: ServerResponse) => {
  switch (req.url) {
    case "/":
      res.writeHead(200, { "Content-Type": "text/html" })
      res.write(readFileSync("../frontend/index.html"))
      res.end()
      break
    case "/index.js":
      res.writeHead(200, { "Content-Type": "text/javascript" })
      res.write(readFileSync("../frontend/index.js"))
      res.end()
      break
    case "/index.css":
      res.writeHead(200, { "Content-Type": "text/css" })
      res.write(readFileSync("../frontend/index.css"))
      res.end()
      break
    case "/analyze":
      let input = ""
      req.on("data", (chunk: Buffer) => {
        input += chunk.toString()
      })
      req.on("end", () => {
        writeFileSync(INPUT_PATH, input)
        const child = spawn(SUNNY_PATH, ["run", INPUT_PATH, "--no-color", "--debug"])
        child.stdout.pipe(createWriteStream(STDOUT_PATH))
        child.stderr.pipe(createWriteStream(STDERR_PATH))
        child.on("close", () => {
          res.writeHead(200, { "Content-Type": "text/plain" })
          res.write("--STDOUT--\n")
          res.write(readFileSync(STDOUT_PATH))
          res.write("--STDERR--\n")
          res.write(readFileSync(STDERR_PATH))
          res.end()
        })
      })
      break
    default:
      res.writeHead(404)
      res.end()
  }
})

server.listen(8080, "127.0.0.1")

const addr = server.address() as AddressInfo
console.info(`Server started on http://${addr.address}:${addr.port}`)
