var js_ffi = {
  run: function(cfg) {
    //allocator
    let allocations = [undefined, null, console, window, document];
    let empty = [];
    function allocate(value) {
      const i = empty.length > 0 ? empty.pop() : allocations.length;
      allocations[i] = value;
      return i;
    }
    function allocator_release(handle) {
      if (handle > 4) {
        delete allocations[handle];
        empty.push(handle);
      }
    }
    function allocator_get(handle) {
      if (handle < 0) {
        return;
      }
      const ret = allocations[handle];
      if (handle !== 0 && !ret) {
        console.error(`Asked for ${handle} after it was released.`);
      }
      return ret;
    }

    let functions = [
      // get property
      (o,p) => {
        return o[p];
      },
      // set property
      (o,p,v) => {
        o[p] = v;
      }
    ];
    let mod = null;

    const TYPE_NOTHING = 0;
    const TYPE_NUM = 1;
    const TYPE_STRING = 2;
    const TYPE_BOOL = 3;
    const TYPE_FUNCTION = 4;
    const TYPE_OBJ = 5;
    const TYPE_UINT8_ARRAY = 6;
    const TYPE_INT8_ARRAY = 7;
    const TYPE_UINT8CLAMPED_ARRAY = 8;
    const TYPE_INT16_ARRAY = 9;
    const TYPE_UINT16_ARRAY = 10;
    const TYPE_INT32_ARRAY = 11;
    const TYPE_UINT32_ARRAY = 12;
    const TYPE_F32_ARRAY = 13;
    const TYPE_F64_ARRAY = 14;
    const TYPE_BI64_ARRAY = 15;
    const TYPE_BUI64_ARRAY = 16;
    const TYPE_MEMORY = 17;

    const utf8dec = new TextDecoder("utf-8");
    const utf8enc = new TextEncoder("utf-8");

    function createString(str) {
      let bytes = utf8enc.encode(str + String.fromCharCode(0));
      let len = bytes.length;
      let start = mod.instance.exports.jsffimalloc(len);
      const memory = new Uint8Array(mod.instance.exports.memory.buffer);
      memory.set(bytes, start);
      return start;
    }

    function createTypedArray(r,t,size){
      let start = mod.instance.exports.jsffimalloc(size*r.length+4);
      let memory = new Uint32Array(mod.instance.exports.memory.buffer);
      memory[start/4] = r.length;
      let data_start = (start+4)/size;
      memory = new t(mod.instance.exports.memory.buffer);
      for(let i=0;i<r.length;i++){
        memory[data_start+i] = r[i];
      }
      return start;
    }

    function getStringFromMemory(mem, start) {
      const data = new Uint8Array(mem);
      const str = [];
      let i = start;
      while (data[i] !== 0) {
        str.push(data[i]);
        i++;
      }
      return utf8dec.decode(new Uint8Array(str));
    }

    function getTypedArrayFromMemory(t, mem, start, size) {
      const data32 = new Uint32Array(mem);
      const ptr = data32[start / 4];
      const offset = data32[ptr / 4];
      const length = data32[ptr / 4 + 1];
      let b = mem.slice(offset, offset + length * size);
      let a = new t(b);
      return a;
    }

    function convertArgument(val_type, val) {
      if (val_type == TYPE_NUM) {
      } else if (val_type === TYPE_NOTHING) {
        val = undefined;
      } else if (val_type === TYPE_STRING) {
        val = getStringFromMemory(mod.instance.exports.memory.buffer, val);
      } else if (val_type === TYPE_BOOL) {
        val = val != 0;
      } else if (val_type === TYPE_OBJ) {
        val = allocator_get(val);
      } else if (val_type === TYPE_FUNCTION) {
        let id = val;
        val = function(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) {
          let l = arguments.length;
          if (l === 0) {
            mod.instance.exports.jsfficallback(
              id,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 1) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 2) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 3) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              0,
              0,
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 4) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              0,
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 5) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              0,
              0,
              0,
              0,
              0
            );
          } else if (l === 6) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              convertResponse(a6),
              0,
              0,
              0,
              0
            );
          } else if (l === 7) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              convertResponse(a6),
              convertResponse(a7),
              0,
              0,
              0
            );
          } else if (l === 8) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              convertResponse(a6),
              convertResponse(a7),
              convertResponse(a8),
              0,
              0
            );
          } else if (l === 9) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              convertResponse(a6),
              convertResponse(a7),
              convertResponse(a8),
              convertResponse(a9),
              0
            );
          } else if (l === 10) {
            mod.instance.exports.jsfficallback(
              id,
              convertResponse(a1),
              convertResponse(a2),
              convertResponse(a3),
              convertResponse(a4),
              convertResponse(a5),
              convertResponse(a6),
              convertResponse(a7),
              convertResponse(a8),
              convertResponse(a9),
              convertResponse(a10)
            );
          }
        };
      } else if (val_type === TYPE_UINT8_ARRAY) {
        val = getTypedArrayFromMemory(
          Uint8Array,
          mod.instance.exports.memory.buffer,
          val,
          1
        );
      } else if (val_type === TYPE_INT8_ARRAY) {
        val = getTypedArrayFromMemory(
          Int8Array,
          mod.instance.exports.memory.buffer,
          val,
          1
        );
      } else if (val_type === TYPE_F32_ARRAY) {
        val = getTypedArrayFromMemory(
          Float32Array,
          mod.instance.exports.memory.buffer,
          val,
          4
        );
      } else if (val_type === TYPE_F64_ARRAY) {
        val = getTypedArrayFromMemory(
          Float64Array,
          mod.instance.exports.memory.buffer,
          val,
          8
        );
      } else if (val_type === TYPE_INT32_ARRAY) {
        val = getTypedArrayFromMemory(
          Int32Array,
          mod.instance.exports.memory.buffer,
          val,
          4
        );
      } else if (val_type === TYPE_UINT32_ARRAY) {
        val = getTypedArrayFromMemory(
          Uint32Array,
          mod.instance.exports.memory.buffer,
          val,
          4
        );
      } else if (val_type === TYPE_INT16_ARRAY) {
        val = getTypedArrayFromMemory(
          Int16Array,
          mod.instance.exports.memory.buffer,
          val,
          2
        );
      } else if (val_type === TYPE_UINT16_ARRAY) {
        val = getTypedArrayFromMemory(
          Uint16Array,
          mod.instance.exports.memory.buffer,
          val,
          2
        );
      } else if (val_type === TYPE_MEMORY) {
        val = mod.instance.exports.memory.buffer;
      }else {
        throw error("Unknown data type");
      }
      return val;
    }

    function convertResponse(r) {
      const type = typeof r;
      if (type === "string") {
        return createString(r);
      } else if (type === "number") {
        return r;
      } else if (r === undefined) {
        return 0;
      } else if (r === null) {
        return 1;
      } else if (r === true) {
        return 1;
      } else if (r === false) {
        return 0;
      } else if (r.constructor === Float32Array) {
        return createTypedArray(r,Float32Array,4)
      } else if (r.constructor === Uint8Array) {
        return createTypedArray(r,Uint8Array,1)
      } else if (r.constructor === Int8Array) {
        return createTypedArray(r,Int8Array,1)
      } else if (r.constructor === Float64Array) {
        return createTypedArray(r,Float64Array,8)
      } else if (r.constructor === Int32Array) {
        return createTypedArray(r,Int32Array,4)
      } else if (r.constructor === Uint32Array) {
        return createTypedArray(r,Uint32Array,4)
      } else if (r.constructor === Int16Array) {
        return createTypedArray(r,Int16Array,2)
      } else if (r.constructor === Uint16Array) {
        return createTypedArray(r,Uint16Array,2)
      }
      return allocate(r);
    }
    if (typeof cfg === "string") {
      cfg = { path: cfg };
    }
    fetch(cfg.path)
      .then(response => response.arrayBuffer())
      .then(bytes =>
        WebAssembly.instantiate(bytes, {
          env: {
            jsffithrowerror: function(e) {
              let err = getStringFromMemory(
                mod.instance.exports.memory.buffer,
                e
              );
              console.error(err);
              throw new Error("Web assembly module exited unexpectedly.");
            },
            jsffirelease: function(obj) {
              allocator_release(obj);
            },
            jsffiregister: function(code) {
              code = getStringFromMemory(
                mod.instance.exports.memory.buffer,
                code
              );
              let id = functions.length;
              functions.push(eval("("+code+")"));
              return id;
            },
            jsfficall0: function(obj, f) {
              return convertResponse(functions[f].call(allocator_get(obj)));
            },
            jsfficall1: function(obj, f, a1_type, a1) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1)
                )
              );
            },
            jsfficall2: function(obj, f, a1_type, a1, a2_type, a2) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2)
                )
              );
            },
            jsfficall3: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3)
                )
              );
            },
            jsfficall4: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4)
                )
              );
            },
            jsfficall5: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5)
                )
              );
            },
            jsfficall6: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5,
              a6_type,
              a6
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5),
                  convertArgument(a6_type, a6)
                )
              );
            },
            jsfficall7: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5,
              a6_type,
              a6,
              a7_type,
              a7
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5),
                  convertArgument(a6_type, a6),
                  convertArgument(a7_type, a7)
                )
              );
            },
            jsfficall8: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5,
              a6_type,
              a6,
              a7_type,
              a7,
              a8_type,
              a8
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5),
                  convertArgument(a6_type, a6),
                  convertArgument(a7_type, a7),
                  convertArgument(a8_type, a8)
                )
              );
            },
            jsfficall9: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5,
              a6_type,
              a6,
              a7_type,
              a7,
              a8_type,
              a8,
              a9_type,
              a9
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5),
                  convertArgument(a6_type, a6),
                  convertArgument(a7_type, a7),
                  convertArgument(a8_type, a8),
                  convertArgument(a9_type, a9)
                )
              );
            },
            jsfficall10: function(
              obj,
              f,
              a1_type,
              a1,
              a2_type,
              a2,
              a3_type,
              a3,
              a4_type,
              a4,
              a5_type,
              a5,
              a6_type,
              a6,
              a7_type,
              a7,
              a8_type,
              a8,
              a9_type,
              a9,
              a10_type,
              a10
            ) {
              return convertResponse(
                functions[f].call(
                  allocator_get(obj),
                  convertArgument(a1_type, a1),
                  convertArgument(a2_type, a2),
                  convertArgument(a3_type, a3),
                  convertArgument(a4_type, a4),
                  convertArgument(a5_type, a5),
                  convertArgument(a6_type, a6),
                  convertArgument(a7_type, a7),
                  convertArgument(a8_type, a8),
                  convertArgument(a9_type, a9),
                  convertArgument(a10_type, a10)
                )
              );
            }
          }
        }).then(module => {
          mod = module;
          if (cfg.onLoad) {
            cfg.onLoad(module);
          }
          module.instance.exports.main();
        })
      );
  }
};
