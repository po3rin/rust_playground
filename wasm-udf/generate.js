const fs = require('fs');

const glue = fs.readFileSync("./pkg/wasm_udf.js", { encoding: "utf8" });
const buffer = fs.readFileSync("./pkg/wasm_udf_bg.wasm");

const bytes = Array.from(new Uint8Array(buffer.buffer));

fs.writeFileSync("udf.js", `\
${glue}
self.wasm = wasm_bindgen(new Uint8Array(${JSON.stringify(bytes)}));
`);

