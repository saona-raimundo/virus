(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/wasm_app.js":
/*!**************************!*\
  !*** ../pkg/wasm_app.js ***!
  \**************************/
/*! exports provided: Input, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_self_86b4b13392c7af56, __wbg_require_f5521a5b85ad2542, __wbg_crypto_b8c92eaac23d0d80, __wbg_msCrypto_9ad6677321a08dd8, __wbindgen_is_undefined, __wbg_getRandomValues_dd27e6b0652b3236, __wbg_getRandomValues_e57c9b75ddead065, __wbg_randomFillSync_d2ba53160aec6aba, __wbg_static_accessor_MODULE_452b4680e8614c81, __wbg_buffer_3f12a1c608c6d04e, __wbg_new_c6c0228e6d22a2f9, __wbg_set_b91afac9fd216d99, __wbg_length_c645e7c02233b440, __wbg_newwithlength_a429e08f8a8fe4b3, __wbg_subarray_02e2fcfa6b285cb2, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_app_bg.wasm */ \"../pkg/wasm_app_bg.wasm\");\n/* harmony import */ var _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_app_bg.js */ \"../pkg/wasm_app_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Input\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Input\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_86b4b13392c7af56\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_86b4b13392c7af56\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_f5521a5b85ad2542\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_f5521a5b85ad2542\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_b8c92eaac23d0d80\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_b8c92eaac23d0d80\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_9ad6677321a08dd8\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_msCrypto_9ad6677321a08dd8\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_dd27e6b0652b3236\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_dd27e6b0652b3236\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_e57c9b75ddead065\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_e57c9b75ddead065\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d2ba53160aec6aba\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_d2ba53160aec6aba\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_452b4680e8614c81\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_static_accessor_MODULE_452b4680e8614c81\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_3f12a1c608c6d04e\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_buffer_3f12a1c608c6d04e\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_c6c0228e6d22a2f9\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_c6c0228e6d22a2f9\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_b91afac9fd216d99\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_b91afac9fd216d99\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_c645e7c02233b440\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_length_c645e7c02233b440\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_a429e08f8a8fe4b3\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithlength_a429e08f8a8fe4b3\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_02e2fcfa6b285cb2\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_subarray_02e2fcfa6b285cb2\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return _wasm_app_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_memory\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_app.js?");

/***/ }),

/***/ "../pkg/wasm_app_bg.js":
/*!*****************************!*\
  !*** ../pkg/wasm_app_bg.js ***!
  \*****************************/
/*! exports provided: Input, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_self_86b4b13392c7af56, __wbg_require_f5521a5b85ad2542, __wbg_crypto_b8c92eaac23d0d80, __wbg_msCrypto_9ad6677321a08dd8, __wbindgen_is_undefined, __wbg_getRandomValues_dd27e6b0652b3236, __wbg_getRandomValues_e57c9b75ddead065, __wbg_randomFillSync_d2ba53160aec6aba, __wbg_static_accessor_MODULE_452b4680e8614c81, __wbg_buffer_3f12a1c608c6d04e, __wbg_new_c6c0228e6d22a2f9, __wbg_set_b91afac9fd216d99, __wbg_length_c645e7c02233b440, __wbg_newwithlength_a429e08f8a8fe4b3, __wbg_subarray_02e2fcfa6b285cb2, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Input\", function() { return Input; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_86b4b13392c7af56\", function() { return __wbg_self_86b4b13392c7af56; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_f5521a5b85ad2542\", function() { return __wbg_require_f5521a5b85ad2542; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_b8c92eaac23d0d80\", function() { return __wbg_crypto_b8c92eaac23d0d80; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_9ad6677321a08dd8\", function() { return __wbg_msCrypto_9ad6677321a08dd8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_dd27e6b0652b3236\", function() { return __wbg_getRandomValues_dd27e6b0652b3236; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_e57c9b75ddead065\", function() { return __wbg_getRandomValues_e57c9b75ddead065; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d2ba53160aec6aba\", function() { return __wbg_randomFillSync_d2ba53160aec6aba; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_452b4680e8614c81\", function() { return __wbg_static_accessor_MODULE_452b4680e8614c81; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_3f12a1c608c6d04e\", function() { return __wbg_buffer_3f12a1c608c6d04e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_c6c0228e6d22a2f9\", function() { return __wbg_new_c6c0228e6d22a2f9; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_b91afac9fd216d99\", function() { return __wbg_set_b91afac9fd216d99; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_c645e7c02233b440\", function() { return __wbg_length_c645e7c02233b440; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_a429e08f8a8fe4b3\", function() { return __wbg_newwithlength_a429e08f8a8fe4b3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_02e2fcfa6b285cb2\", function() { return __wbg_subarray_02e2fcfa6b285cb2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony import */ var _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_app_bg.wasm */ \"../pkg/wasm_app_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n*/\nclass Input {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Input.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_input_free\"](ptr);\n    }\n    /**\n    * @param {number} inmune_init\n    * @param {boolean} concert_hall\n    * @param {boolean} bakery\n    * @param {boolean} school\n    * @param {boolean} pharmacy\n    * @param {boolean} restaurant\n    * @param {boolean} gym\n    * @param {boolean} supermarket\n    * @param {boolean} shopping_center\n    * @returns {Input}\n    */\n    static new(inmune_init, concert_hall, bakery, school, pharmacy, restaurant, gym, supermarket, shopping_center) {\n        var ret = _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"input_new\"](inmune_init, concert_hall, bakery, school, pharmacy, restaurant, gym, supermarket, shopping_center);\n        return Input.__wrap(ret);\n    }\n    /**\n    * @returns {string}\n    */\n    message_js() {\n        try {\n            const retptr = _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"input_message_js\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @returns {string}\n    */\n    message_many_js() {\n        try {\n            const retptr = _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"input_message_many_js\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n            _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n}\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_self_86b4b13392c7af56 = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_require_f5521a5b85ad2542 = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_b8c92eaac23d0d80 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbg_msCrypto_9ad6677321a08dd8 = function(arg0) {\n    var ret = getObject(arg0).msCrypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_dd27e6b0652b3236 = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbg_getRandomValues_e57c9b75ddead065 = function(arg0, arg1) {\n    getObject(arg0).getRandomValues(getObject(arg1));\n};\n\nconst __wbg_randomFillSync_d2ba53160aec6aba = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_static_accessor_MODULE_452b4680e8614c81 = function() {\n    var ret = module;\n    return addHeapObject(ret);\n};\n\nconst __wbg_buffer_3f12a1c608c6d04e = function(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nconst __wbg_new_c6c0228e6d22a2f9 = function(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nconst __wbg_set_b91afac9fd216d99 = function(arg0, arg1, arg2) {\n    getObject(arg0).set(getObject(arg1), arg2 >>> 0);\n};\n\nconst __wbg_length_c645e7c02233b440 = function(arg0) {\n    var ret = getObject(arg0).length;\n    return ret;\n};\n\nconst __wbg_newwithlength_a429e08f8a8fe4b3 = function(arg0) {\n    var ret = new Uint8Array(arg0 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_subarray_02e2fcfa6b285cb2 = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbindgen_memory = function() {\n    var ret = _wasm_app_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/wasm_app_bg.js?");

/***/ }),

/***/ "../pkg/wasm_app_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/wasm_app_bg.wasm ***!
  \*******************************/
/*! exports provided: memory, __wbg_input_free, input_new, input_message_js, input_message_many_js, __wbindgen_export_0, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./wasm_app_bg.js */ \"../pkg/wasm_app_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_app_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_app__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-app */ \"../pkg/wasm_app.js\");\n\r\n\r\n\r\n// Input\r\n\r\n\r\nfunction read_input() {\r\n\tconst inmune_init = parseInt(document.getElementById(\"inmune\").value);\t\t\r\n\tconst concert_hall = document.getElementById(\"concert_hall\").checked;\r\n\tconst bakery = document.getElementById(\"bakery\").checked;\r\n\tconst school = document.getElementById(\"school\").checked;\r\n\tconst pharmacy = document.getElementById(\"pharmacy\").checked;\r\n\tconst restaurant = document.getElementById(\"restaurant\").checked;\r\n\tconst gym = document.getElementById(\"gym\").checked;\r\n\tconst supermarket = document.getElementById(\"supermarket\").checked;\r\n\tconst shopping_center = document.getElementById(\"shopping_center\").checked;\r\n\treturn wasm_app__WEBPACK_IMPORTED_MODULE_0__[\"Input\"].new(\r\n\t\tinmune_init, \r\n\t\tconcert_hall, \r\n\t\tbakery,\r\n\t\tschool,\r\n\t\tpharmacy,\r\n\t\trestaurant,\r\n\t\tgym,\r\n\t\tsupermarket,\r\n\t\tshopping_center,\r\n\t);\r\n}\r\n\r\n// Interactive elements\r\nconst pre = document.getElementById(\"output\");\r\nconst simulateButton = document.getElementById(\"simulate\");\r\nconst simulateManyButton = document.getElementById(\"simulate_many\");\r\n\r\n// Interactions\r\nsimulateButton.addEventListener(\"click\", event => {\r\n\tconsole.time(\"simulation_js\");\r\n\tconst input = read_input();\r\n\tpre.textContent = input.message_js();\r\n\tconsole.timeEnd(\"simulation_js\");\r\n});\r\n\r\nsimulateManyButton.addEventListener(\"click\", event => {\r\n\tconsole.time(\"simulation_many_js\");\r\n\tconst input = read_input();\r\n\tpre.textContent = input.message_many_js();\r\n\tconsole.timeEnd(\"simulation_many_js\");\r\n});\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);