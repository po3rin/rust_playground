const udf = require('./udf.js')

udf.wasm.then(() => {
    console.log(wasm_bindgen.add(1, 2));
});
