// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[link(name = "gl",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/gl-rs",
       vers = "0.1")];
#[comment = "An OpenGL function loader."];
#[license = "ASL2"];
#[crate_type = "lib"];

#[feature(macro_rules)];
#[feature(globs)];

use std::libc::*;
use self::types::*;

pub mod types {
    use std::libc::*;
    
    // Common types from OpenGL 1.1
    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLclampx = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;
    pub type GLeglImageOES = *c_void;
    pub type GLchar = c_char;
    pub type GLcharARB = c_char;
    
    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = c_uint;
    
    pub type GLhalfARB = c_ushort;
    pub type GLhalf = c_ushort;
    
    // Must be 32 bits
    pub type GLfixed = GLint;
    
    pub type GLintptr = ptrdiff_t;
    pub type GLsizeiptr = ptrdiff_t;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = ptrdiff_t;
    pub type GLsizeiptrARB = ptrdiff_t;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;
    
    pub struct __GLsync;
    pub type GLsync = *__GLsync;
    
    // compatible with OpenCL cl_context
    pub struct _cl_context;
    pub struct _cl_event;
    
    pub type GLDEBUGPROC = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLDEBUGPROCARB = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLDEBUGPROCKHR = extern "C" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    
    // Vendor extension types
    pub type GLDEBUGPROCAMD = extern "C" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *GLchar, userParam: *c_void);
    pub type GLhalfNV = c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;
}

pub static DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub static STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub static COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub static CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub static CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
pub static CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub static CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub static MAP_READ_BIT: GLenum = 0x0001;
pub static MAP_WRITE_BIT: GLenum = 0x0002;
pub static MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub static MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub static MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub static MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
pub static VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
pub static ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
pub static UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
pub static TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
pub static SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
pub static COMMAND_BARRIER_BIT: GLenum = 0x00000040;
pub static PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
pub static TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
pub static BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
pub static FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
pub static TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
pub static ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
pub static SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
pub static ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
pub static SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub static VERTEX_SHADER_BIT: GLenum = 0x00000001;
pub static FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
pub static GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
pub static TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
pub static TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
pub static ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub static FALSE: GLboolean = 0;
pub static NO_ERROR: GLenum = 0;
pub static ZERO: GLenum = 0;
pub static NONE: GLenum = 0;
pub static TRUE: GLboolean = 1;
pub static ONE: GLenum = 1;
pub static INVALID_INDEX: GLenum = 0xFFFFFFFF;
pub static TIMEOUT_IGNORED: GLenum = 0xFFFFFFFFFFFFFFFF;
pub static POINTS: GLenum = 0x0000;
pub static LINES: GLenum = 0x0001;
pub static LINE_LOOP: GLenum = 0x0002;
pub static LINE_STRIP: GLenum = 0x0003;
pub static TRIANGLES: GLenum = 0x0004;
pub static TRIANGLE_STRIP: GLenum = 0x0005;
pub static TRIANGLE_FAN: GLenum = 0x0006;
pub static LINES_ADJACENCY: GLenum = 0x000A;
pub static LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub static TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub static TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub static PATCHES: GLenum = 0x000E;
pub static NEVER: GLenum = 0x0200;
pub static LESS: GLenum = 0x0201;
pub static EQUAL: GLenum = 0x0202;
pub static LEQUAL: GLenum = 0x0203;
pub static GREATER: GLenum = 0x0204;
pub static NOTEQUAL: GLenum = 0x0205;
pub static GEQUAL: GLenum = 0x0206;
pub static ALWAYS: GLenum = 0x0207;
pub static SRC_COLOR: GLenum = 0x0300;
pub static ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub static SRC_ALPHA: GLenum = 0x0302;
pub static ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub static DST_ALPHA: GLenum = 0x0304;
pub static ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub static DST_COLOR: GLenum = 0x0306;
pub static ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub static SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub static FRONT_LEFT: GLenum = 0x0400;
pub static FRONT_RIGHT: GLenum = 0x0401;
pub static BACK_LEFT: GLenum = 0x0402;
pub static BACK_RIGHT: GLenum = 0x0403;
pub static FRONT: GLenum = 0x0404;
pub static BACK: GLenum = 0x0405;
pub static LEFT: GLenum = 0x0406;
pub static RIGHT: GLenum = 0x0407;
pub static FRONT_AND_BACK: GLenum = 0x0408;
pub static INVALID_ENUM: GLenum = 0x0500;
pub static INVALID_VALUE: GLenum = 0x0501;
pub static INVALID_OPERATION: GLenum = 0x0502;
pub static OUT_OF_MEMORY: GLenum = 0x0505;
pub static INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub static CW: GLenum = 0x0900;
pub static CCW: GLenum = 0x0901;
pub static POINT_SIZE: GLenum = 0x0B11;
pub static POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub static POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub static LINE_SMOOTH: GLenum = 0x0B20;
pub static LINE_WIDTH: GLenum = 0x0B21;
pub static LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub static LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub static POLYGON_MODE: GLenum = 0x0B40;
pub static POLYGON_SMOOTH: GLenum = 0x0B41;
pub static CULL_FACE: GLenum = 0x0B44;
pub static CULL_FACE_MODE: GLenum = 0x0B45;
pub static FRONT_FACE: GLenum = 0x0B46;
pub static DEPTH_RANGE: GLenum = 0x0B70;
pub static DEPTH_TEST: GLenum = 0x0B71;
pub static DEPTH_WRITEMASK: GLenum = 0x0B72;
pub static DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub static DEPTH_FUNC: GLenum = 0x0B74;
pub static STENCIL_TEST: GLenum = 0x0B90;
pub static STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub static STENCIL_FUNC: GLenum = 0x0B92;
pub static STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub static STENCIL_FAIL: GLenum = 0x0B94;
pub static STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub static STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub static STENCIL_REF: GLenum = 0x0B97;
pub static STENCIL_WRITEMASK: GLenum = 0x0B98;
pub static VIEWPORT: GLenum = 0x0BA2;
pub static DITHER: GLenum = 0x0BD0;
pub static BLEND_DST: GLenum = 0x0BE0;
pub static BLEND_SRC: GLenum = 0x0BE1;
pub static BLEND: GLenum = 0x0BE2;
pub static LOGIC_OP_MODE: GLenum = 0x0BF0;
pub static COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub static DRAW_BUFFER: GLenum = 0x0C01;
pub static READ_BUFFER: GLenum = 0x0C02;
pub static SCISSOR_BOX: GLenum = 0x0C10;
pub static SCISSOR_TEST: GLenum = 0x0C11;
pub static COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub static COLOR_WRITEMASK: GLenum = 0x0C23;
pub static DOUBLEBUFFER: GLenum = 0x0C32;
pub static STEREO: GLenum = 0x0C33;
pub static LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub static POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub static UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub static UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub static UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub static UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub static UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub static UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub static PACK_SWAP_BYTES: GLenum = 0x0D00;
pub static PACK_LSB_FIRST: GLenum = 0x0D01;
pub static PACK_ROW_LENGTH: GLenum = 0x0D02;
pub static PACK_SKIP_ROWS: GLenum = 0x0D03;
pub static PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub static PACK_ALIGNMENT: GLenum = 0x0D05;
pub static MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub static MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub static MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub static SUBPIXEL_BITS: GLenum = 0x0D50;
pub static TEXTURE_1D: GLenum = 0x0DE0;
pub static TEXTURE_2D: GLenum = 0x0DE1;
pub static TEXTURE_WIDTH: GLenum = 0x1000;
pub static TEXTURE_HEIGHT: GLenum = 0x1001;
pub static TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub static TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub static DONT_CARE: GLenum = 0x1100;
pub static FASTEST: GLenum = 0x1101;
pub static NICEST: GLenum = 0x1102;
pub static BYTE: GLenum = 0x1400;
pub static UNSIGNED_BYTE: GLenum = 0x1401;
pub static SHORT: GLenum = 0x1402;
pub static UNSIGNED_SHORT: GLenum = 0x1403;
pub static INT: GLenum = 0x1404;
pub static UNSIGNED_INT: GLenum = 0x1405;
pub static FLOAT: GLenum = 0x1406;
pub static DOUBLE: GLenum = 0x140A;
pub static HALF_FLOAT: GLenum = 0x140B;
pub static FIXED: GLenum = 0x140C;
pub static CLEAR: GLenum = 0x1500;
pub static AND: GLenum = 0x1501;
pub static AND_REVERSE: GLenum = 0x1502;
pub static COPY: GLenum = 0x1503;
pub static AND_INVERTED: GLenum = 0x1504;
pub static NOOP: GLenum = 0x1505;
pub static XOR: GLenum = 0x1506;
pub static OR: GLenum = 0x1507;
pub static NOR: GLenum = 0x1508;
pub static EQUIV: GLenum = 0x1509;
pub static INVERT: GLenum = 0x150A;
pub static OR_REVERSE: GLenum = 0x150B;
pub static COPY_INVERTED: GLenum = 0x150C;
pub static OR_INVERTED: GLenum = 0x150D;
pub static NAND: GLenum = 0x150E;
pub static SET: GLenum = 0x150F;
pub static TEXTURE: GLenum = 0x1702;
pub static COLOR: GLenum = 0x1800;
pub static DEPTH: GLenum = 0x1801;
pub static STENCIL: GLenum = 0x1802;
pub static STENCIL_INDEX: GLenum = 0x1901;
pub static DEPTH_COMPONENT: GLenum = 0x1902;
pub static RED: GLenum = 0x1903;
pub static GREEN: GLenum = 0x1904;
pub static BLUE: GLenum = 0x1905;
pub static ALPHA: GLenum = 0x1906;
pub static RGB: GLenum = 0x1907;
pub static RGBA: GLenum = 0x1908;
pub static POINT: GLenum = 0x1B00;
pub static LINE: GLenum = 0x1B01;
pub static FILL: GLenum = 0x1B02;
pub static KEEP: GLenum = 0x1E00;
pub static REPLACE: GLenum = 0x1E01;
pub static INCR: GLenum = 0x1E02;
pub static DECR: GLenum = 0x1E03;
pub static VENDOR: GLenum = 0x1F00;
pub static RENDERER: GLenum = 0x1F01;
pub static VERSION: GLenum = 0x1F02;
pub static EXTENSIONS: GLenum = 0x1F03;
pub static NEAREST: GLenum = 0x2600;
pub static LINEAR: GLenum = 0x2601;
pub static NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub static LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub static NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub static LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub static TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub static TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub static TEXTURE_WRAP_S: GLenum = 0x2802;
pub static TEXTURE_WRAP_T: GLenum = 0x2803;
pub static REPEAT: GLenum = 0x2901;
pub static POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub static POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub static POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub static R3_G3_B2: GLenum = 0x2A10;
pub static CLIP_DISTANCE0: GLenum = 0x3000;
pub static CLIP_DISTANCE1: GLenum = 0x3001;
pub static CLIP_DISTANCE2: GLenum = 0x3002;
pub static CLIP_DISTANCE3: GLenum = 0x3003;
pub static CLIP_DISTANCE4: GLenum = 0x3004;
pub static CLIP_DISTANCE5: GLenum = 0x3005;
pub static CLIP_DISTANCE6: GLenum = 0x3006;
pub static CLIP_DISTANCE7: GLenum = 0x3007;
pub static CONSTANT_COLOR: GLenum = 0x8001;
pub static ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub static CONSTANT_ALPHA: GLenum = 0x8003;
pub static ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub static FUNC_ADD: GLenum = 0x8006;
pub static MIN: GLenum = 0x8007;
pub static MAX: GLenum = 0x8008;
pub static BLEND_EQUATION_RGB: GLenum = 0x8009;
pub static FUNC_SUBTRACT: GLenum = 0x800A;
pub static FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub static UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub static UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub static UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub static UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub static UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub static POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub static POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub static RGB4: GLenum = 0x804F;
pub static RGB5: GLenum = 0x8050;
pub static RGB8: GLenum = 0x8051;
pub static RGB10: GLenum = 0x8052;
pub static RGB12: GLenum = 0x8053;
pub static RGB16: GLenum = 0x8054;
pub static RGBA2: GLenum = 0x8055;
pub static RGBA4: GLenum = 0x8056;
pub static RGB5_A1: GLenum = 0x8057;
pub static RGBA8: GLenum = 0x8058;
pub static RGB10_A2: GLenum = 0x8059;
pub static RGBA12: GLenum = 0x805A;
pub static RGBA16: GLenum = 0x805B;
pub static TEXTURE_RED_SIZE: GLenum = 0x805C;
pub static TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub static TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub static TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub static PROXY_TEXTURE_1D: GLenum = 0x8063;
pub static PROXY_TEXTURE_2D: GLenum = 0x8064;
pub static TEXTURE_BINDING_1D: GLenum = 0x8068;
pub static TEXTURE_BINDING_2D: GLenum = 0x8069;
pub static TEXTURE_BINDING_3D: GLenum = 0x806A;
pub static PACK_SKIP_IMAGES: GLenum = 0x806B;
pub static PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub static UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub static UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub static TEXTURE_3D: GLenum = 0x806F;
pub static PROXY_TEXTURE_3D: GLenum = 0x8070;
pub static TEXTURE_DEPTH: GLenum = 0x8071;
pub static TEXTURE_WRAP_R: GLenum = 0x8072;
pub static MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub static MULTISAMPLE: GLenum = 0x809D;
pub static SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub static SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub static SAMPLE_COVERAGE: GLenum = 0x80A0;
pub static SAMPLE_BUFFERS: GLenum = 0x80A8;
pub static SAMPLES: GLenum = 0x80A9;
pub static SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub static SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub static BLEND_DST_RGB: GLenum = 0x80C8;
pub static BLEND_SRC_RGB: GLenum = 0x80C9;
pub static BLEND_DST_ALPHA: GLenum = 0x80CA;
pub static BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub static BGR: GLenum = 0x80E0;
pub static BGRA: GLenum = 0x80E1;
pub static MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub static MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub static POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub static CLAMP_TO_BORDER: GLenum = 0x812D;
pub static CLAMP_TO_EDGE: GLenum = 0x812F;
pub static TEXTURE_MIN_LOD: GLenum = 0x813A;
pub static TEXTURE_MAX_LOD: GLenum = 0x813B;
pub static TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub static TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub static DEPTH_COMPONENT16: GLenum = 0x81A5;
pub static DEPTH_COMPONENT24: GLenum = 0x81A6;
pub static DEPTH_COMPONENT32: GLenum = 0x81A7;
pub static FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub static FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub static FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub static FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub static FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub static FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub static FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub static FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub static FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub static FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub static DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub static MAJOR_VERSION: GLenum = 0x821B;
pub static MINOR_VERSION: GLenum = 0x821C;
pub static NUM_EXTENSIONS: GLenum = 0x821D;
pub static CONTEXT_FLAGS: GLenum = 0x821E;
pub static INDEX: GLenum = 0x8222;
pub static COMPRESSED_RED: GLenum = 0x8225;
pub static COMPRESSED_RG: GLenum = 0x8226;
pub static RG: GLenum = 0x8227;
pub static RG_INTEGER: GLenum = 0x8228;
pub static R8: GLenum = 0x8229;
pub static R16: GLenum = 0x822A;
pub static RG8: GLenum = 0x822B;
pub static RG16: GLenum = 0x822C;
pub static R16F: GLenum = 0x822D;
pub static R32F: GLenum = 0x822E;
pub static RG16F: GLenum = 0x822F;
pub static RG32F: GLenum = 0x8230;
pub static R8I: GLenum = 0x8231;
pub static R8UI: GLenum = 0x8232;
pub static R16I: GLenum = 0x8233;
pub static R16UI: GLenum = 0x8234;
pub static R32I: GLenum = 0x8235;
pub static R32UI: GLenum = 0x8236;
pub static RG8I: GLenum = 0x8237;
pub static RG8UI: GLenum = 0x8238;
pub static RG16I: GLenum = 0x8239;
pub static RG16UI: GLenum = 0x823A;
pub static RG32I: GLenum = 0x823B;
pub static RG32UI: GLenum = 0x823C;
pub static DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub static DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub static DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub static DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub static DEBUG_SOURCE_API: GLenum = 0x8246;
pub static DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub static DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub static DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub static DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub static DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub static DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub static DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub static DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub static DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub static DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub static DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub static PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub static PROGRAM_SEPARABLE: GLenum = 0x8258;
pub static ACTIVE_PROGRAM: GLenum = 0x8259;
pub static PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
pub static MAX_VIEWPORTS: GLenum = 0x825B;
pub static VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub static VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub static LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub static VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub static UNDEFINED_VERTEX: GLenum = 0x8260;
pub static MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub static MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub static MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub static MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub static MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub static COMPUTE_LOCAL_WORK_SIZE: GLenum = 0x8267;
pub static DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub static DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub static DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub static DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub static MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub static DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub static MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
pub static INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub static INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub static INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub static INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub static INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub static INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub static INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub static INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub static INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub static INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub static INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub static INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub static INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub static INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub static INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub static MAX_WIDTH: GLenum = 0x827E;
pub static MAX_HEIGHT: GLenum = 0x827F;
pub static MAX_DEPTH: GLenum = 0x8280;
pub static MAX_LAYERS: GLenum = 0x8281;
pub static MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub static COLOR_COMPONENTS: GLenum = 0x8283;
pub static DEPTH_COMPONENTS: GLenum = 0x8284;
pub static STENCIL_COMPONENTS: GLenum = 0x8285;
pub static COLOR_RENDERABLE: GLenum = 0x8286;
pub static DEPTH_RENDERABLE: GLenum = 0x8287;
pub static STENCIL_RENDERABLE: GLenum = 0x8288;
pub static FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub static FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub static FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub static READ_PIXELS: GLenum = 0x828C;
pub static READ_PIXELS_FORMAT: GLenum = 0x828D;
pub static READ_PIXELS_TYPE: GLenum = 0x828E;
pub static TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub static TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub static GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub static GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub static MIPMAP: GLenum = 0x8293;
pub static MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub static AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub static COLOR_ENCODING: GLenum = 0x8296;
pub static SRGB_READ: GLenum = 0x8297;
pub static SRGB_WRITE: GLenum = 0x8298;
pub static FILTER: GLenum = 0x829A;
pub static VERTEX_TEXTURE: GLenum = 0x829B;
pub static TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub static TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub static GEOMETRY_TEXTURE: GLenum = 0x829E;
pub static FRAGMENT_TEXTURE: GLenum = 0x829F;
pub static COMPUTE_TEXTURE: GLenum = 0x82A0;
pub static TEXTURE_SHADOW: GLenum = 0x82A1;
pub static TEXTURE_GATHER: GLenum = 0x82A2;
pub static TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub static SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub static SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub static SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub static IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub static IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub static IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub static IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub static SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub static SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub static SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub static SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub static TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub static TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub static TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub static CLEAR_BUFFER: GLenum = 0x82B4;
pub static TEXTURE_VIEW: GLenum = 0x82B5;
pub static VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub static FULL_SUPPORT: GLenum = 0x82B7;
pub static CAVEAT_SUPPORT: GLenum = 0x82B8;
pub static IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub static IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub static IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub static IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub static IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub static IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub static IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub static IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub static IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub static IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub static IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub static VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub static VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub static VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub static VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub static VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub static VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub static VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub static VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub static VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub static VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub static VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub static VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub static VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub static VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub static VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub static VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub static VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub static VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub static VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub static VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub static VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub static MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub static MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub static TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub static TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub static TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub static TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub static TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
pub static BUFFER: GLenum = 0x82E0;
pub static SHADER: GLenum = 0x82E1;
pub static PROGRAM: GLenum = 0x82E2;
pub static QUERY: GLenum = 0x82E3;
pub static PROGRAM_PIPELINE: GLenum = 0x82E4;
pub static SAMPLER: GLenum = 0x82E6;
pub static DISPLAY_LIST: GLenum = 0x82E7;
pub static MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub static NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub static UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub static UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub static UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub static UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub static UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub static UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub static UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub static MIRRORED_REPEAT: GLenum = 0x8370;
pub static ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub static TEXTURE0: GLenum = 0x84C0;
pub static TEXTURE1: GLenum = 0x84C1;
pub static TEXTURE2: GLenum = 0x84C2;
pub static TEXTURE3: GLenum = 0x84C3;
pub static TEXTURE4: GLenum = 0x84C4;
pub static TEXTURE5: GLenum = 0x84C5;
pub static TEXTURE6: GLenum = 0x84C6;
pub static TEXTURE7: GLenum = 0x84C7;
pub static TEXTURE8: GLenum = 0x84C8;
pub static TEXTURE9: GLenum = 0x84C9;
pub static TEXTURE10: GLenum = 0x84CA;
pub static TEXTURE11: GLenum = 0x84CB;
pub static TEXTURE12: GLenum = 0x84CC;
pub static TEXTURE13: GLenum = 0x84CD;
pub static TEXTURE14: GLenum = 0x84CE;
pub static TEXTURE15: GLenum = 0x84CF;
pub static TEXTURE16: GLenum = 0x84D0;
pub static TEXTURE17: GLenum = 0x84D1;
pub static TEXTURE18: GLenum = 0x84D2;
pub static TEXTURE19: GLenum = 0x84D3;
pub static TEXTURE20: GLenum = 0x84D4;
pub static TEXTURE21: GLenum = 0x84D5;
pub static TEXTURE22: GLenum = 0x84D6;
pub static TEXTURE23: GLenum = 0x84D7;
pub static TEXTURE24: GLenum = 0x84D8;
pub static TEXTURE25: GLenum = 0x84D9;
pub static TEXTURE26: GLenum = 0x84DA;
pub static TEXTURE27: GLenum = 0x84DB;
pub static TEXTURE28: GLenum = 0x84DC;
pub static TEXTURE29: GLenum = 0x84DD;
pub static TEXTURE30: GLenum = 0x84DE;
pub static TEXTURE31: GLenum = 0x84DF;
pub static ACTIVE_TEXTURE: GLenum = 0x84E0;
pub static MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub static COMPRESSED_RGB: GLenum = 0x84ED;
pub static COMPRESSED_RGBA: GLenum = 0x84EE;
pub static TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub static UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub static UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
pub static TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub static TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub static PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub static MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub static DEPTH_STENCIL: GLenum = 0x84F9;
pub static UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub static MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub static TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub static INCR_WRAP: GLenum = 0x8507;
pub static DECR_WRAP: GLenum = 0x8508;
pub static TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub static TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub static TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub static TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub static TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub static TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub static TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub static PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub static MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub static SRC1_ALPHA: GLenum = 0x8589;
pub static VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub static VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub static VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub static VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub static VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub static CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub static VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub static VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub static DEPTH_CLAMP: GLenum = 0x864F;
pub static TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub static TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub static NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub static COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub static PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub static VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
pub static BUFFER_SIZE: GLenum = 0x8764;
pub static BUFFER_USAGE: GLenum = 0x8765;
pub static NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub static PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
pub static STENCIL_BACK_FUNC: GLenum = 0x8800;
pub static STENCIL_BACK_FAIL: GLenum = 0x8801;
pub static STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub static STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub static RGBA32F: GLenum = 0x8814;
pub static RGB32F: GLenum = 0x8815;
pub static RGBA16F: GLenum = 0x881A;
pub static RGB16F: GLenum = 0x881B;
pub static MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub static DRAW_BUFFER0: GLenum = 0x8825;
pub static DRAW_BUFFER1: GLenum = 0x8826;
pub static DRAW_BUFFER2: GLenum = 0x8827;
pub static DRAW_BUFFER3: GLenum = 0x8828;
pub static DRAW_BUFFER4: GLenum = 0x8829;
pub static DRAW_BUFFER5: GLenum = 0x882A;
pub static DRAW_BUFFER6: GLenum = 0x882B;
pub static DRAW_BUFFER7: GLenum = 0x882C;
pub static DRAW_BUFFER8: GLenum = 0x882D;
pub static DRAW_BUFFER9: GLenum = 0x882E;
pub static DRAW_BUFFER10: GLenum = 0x882F;
pub static DRAW_BUFFER11: GLenum = 0x8830;
pub static DRAW_BUFFER12: GLenum = 0x8831;
pub static DRAW_BUFFER13: GLenum = 0x8832;
pub static DRAW_BUFFER14: GLenum = 0x8833;
pub static DRAW_BUFFER15: GLenum = 0x8834;
pub static BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub static TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub static TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub static TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub static COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub static TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub static QUERY_COUNTER_BITS: GLenum = 0x8864;
pub static CURRENT_QUERY: GLenum = 0x8865;
pub static QUERY_RESULT: GLenum = 0x8866;
pub static QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub static MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub static VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub static MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub static MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub static MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub static GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub static ARRAY_BUFFER: GLenum = 0x8892;
pub static ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub static ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub static ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub static VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub static READ_ONLY: GLenum = 0x88B8;
pub static WRITE_ONLY: GLenum = 0x88B9;
pub static READ_WRITE: GLenum = 0x88BA;
pub static BUFFER_ACCESS: GLenum = 0x88BB;
pub static BUFFER_MAPPED: GLenum = 0x88BC;
pub static BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub static TIME_ELAPSED: GLenum = 0x88BF;
pub static STREAM_DRAW: GLenum = 0x88E0;
pub static STREAM_READ: GLenum = 0x88E1;
pub static STREAM_COPY: GLenum = 0x88E2;
pub static STATIC_DRAW: GLenum = 0x88E4;
pub static STATIC_READ: GLenum = 0x88E5;
pub static STATIC_COPY: GLenum = 0x88E6;
pub static DYNAMIC_DRAW: GLenum = 0x88E8;
pub static DYNAMIC_READ: GLenum = 0x88E9;
pub static DYNAMIC_COPY: GLenum = 0x88EA;
pub static PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub static PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub static PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub static PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub static DEPTH24_STENCIL8: GLenum = 0x88F0;
pub static TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub static SRC1_COLOR: GLenum = 0x88F9;
pub static ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub static ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub static MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub static VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub static VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub static MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub static MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub static MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub static SAMPLES_PASSED: GLenum = 0x8914;
pub static GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub static GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub static GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub static SAMPLER_BINDING: GLenum = 0x8919;
pub static CLAMP_READ_COLOR: GLenum = 0x891C;
pub static FIXED_ONLY: GLenum = 0x891D;
pub static UNIFORM_BUFFER: GLenum = 0x8A11;
pub static UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub static UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub static UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub static MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub static MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub static MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub static MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub static MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub static MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub static MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub static UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub static ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub static ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub static UNIFORM_TYPE: GLenum = 0x8A37;
pub static UNIFORM_SIZE: GLenum = 0x8A38;
pub static UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub static UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub static UNIFORM_OFFSET: GLenum = 0x8A3B;
pub static UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub static UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub static UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub static UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub static UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub static UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub static UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub static UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub static UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub static UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub static FRAGMENT_SHADER: GLenum = 0x8B30;
pub static VERTEX_SHADER: GLenum = 0x8B31;
pub static MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub static MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub static MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub static MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub static MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub static MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub static SHADER_TYPE: GLenum = 0x8B4F;
pub static FLOAT_VEC2: GLenum = 0x8B50;
pub static FLOAT_VEC3: GLenum = 0x8B51;
pub static FLOAT_VEC4: GLenum = 0x8B52;
pub static INT_VEC2: GLenum = 0x8B53;
pub static INT_VEC3: GLenum = 0x8B54;
pub static INT_VEC4: GLenum = 0x8B55;
pub static BOOL: GLenum = 0x8B56;
pub static BOOL_VEC2: GLenum = 0x8B57;
pub static BOOL_VEC3: GLenum = 0x8B58;
pub static BOOL_VEC4: GLenum = 0x8B59;
pub static FLOAT_MAT2: GLenum = 0x8B5A;
pub static FLOAT_MAT3: GLenum = 0x8B5B;
pub static FLOAT_MAT4: GLenum = 0x8B5C;
pub static SAMPLER_1D: GLenum = 0x8B5D;
pub static SAMPLER_2D: GLenum = 0x8B5E;
pub static SAMPLER_3D: GLenum = 0x8B5F;
pub static SAMPLER_CUBE: GLenum = 0x8B60;
pub static SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub static SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub static SAMPLER_2D_RECT: GLenum = 0x8B63;
pub static SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub static FLOAT_MAT2x3: GLenum = 0x8B65;
pub static FLOAT_MAT2x4: GLenum = 0x8B66;
pub static FLOAT_MAT3x2: GLenum = 0x8B67;
pub static FLOAT_MAT3x4: GLenum = 0x8B68;
pub static FLOAT_MAT4x2: GLenum = 0x8B69;
pub static FLOAT_MAT4x3: GLenum = 0x8B6A;
pub static DELETE_STATUS: GLenum = 0x8B80;
pub static COMPILE_STATUS: GLenum = 0x8B81;
pub static LINK_STATUS: GLenum = 0x8B82;
pub static VALIDATE_STATUS: GLenum = 0x8B83;
pub static INFO_LOG_LENGTH: GLenum = 0x8B84;
pub static ATTACHED_SHADERS: GLenum = 0x8B85;
pub static ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub static ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub static SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub static ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub static ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub static FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub static SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub static CURRENT_PROGRAM: GLenum = 0x8B8D;
pub static IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub static IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub static TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub static TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub static TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub static TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub static TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub static UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub static TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub static PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub static TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub static PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub static TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub static TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub static MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub static TEXTURE_BUFFER: GLenum = 0x8C2A;
pub static MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub static TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub static TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub static ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub static SAMPLE_SHADING: GLenum = 0x8C36;
pub static MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
pub static R11F_G11F_B10F: GLenum = 0x8C3A;
pub static UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub static RGB9_E5: GLenum = 0x8C3D;
pub static UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub static TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub static SRGB: GLenum = 0x8C40;
pub static SRGB8: GLenum = 0x8C41;
pub static SRGB_ALPHA: GLenum = 0x8C42;
pub static SRGB8_ALPHA8: GLenum = 0x8C43;
pub static COMPRESSED_SRGB: GLenum = 0x8C48;
pub static COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub static TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub static TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub static MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub static TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub static TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub static TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub static PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub static TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub static RASTERIZER_DISCARD: GLenum = 0x8C89;
pub static MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub static MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub static INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub static SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub static TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub static TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub static POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub static LOWER_LEFT: GLenum = 0x8CA1;
pub static UPPER_LEFT: GLenum = 0x8CA2;
pub static STENCIL_BACK_REF: GLenum = 0x8CA3;
pub static STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub static STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub static DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub static RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub static READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub static DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub static READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub static RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub static DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub static DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub static FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub static FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub static FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub static FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub static FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub static FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub static FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub static FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub static FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub static MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub static COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub static COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub static COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub static COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub static COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub static COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub static COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub static COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub static COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub static COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub static COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub static COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub static COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub static COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub static COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub static COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub static DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub static STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub static FRAMEBUFFER: GLenum = 0x8D40;
pub static RENDERBUFFER: GLenum = 0x8D41;
pub static RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub static RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub static RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub static STENCIL_INDEX1: GLenum = 0x8D46;
pub static STENCIL_INDEX4: GLenum = 0x8D47;
pub static STENCIL_INDEX8: GLenum = 0x8D48;
pub static STENCIL_INDEX16: GLenum = 0x8D49;
pub static RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub static RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub static RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub static RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub static RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub static RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub static FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub static MAX_SAMPLES: GLenum = 0x8D57;
pub static RGB565: GLenum = 0x8D62;
pub static PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub static ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub static MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
pub static RGBA32UI: GLenum = 0x8D70;
pub static RGB32UI: GLenum = 0x8D71;
pub static RGBA16UI: GLenum = 0x8D76;
pub static RGB16UI: GLenum = 0x8D77;
pub static RGBA8UI: GLenum = 0x8D7C;
pub static RGB8UI: GLenum = 0x8D7D;
pub static RGBA32I: GLenum = 0x8D82;
pub static RGB32I: GLenum = 0x8D83;
pub static RGBA16I: GLenum = 0x8D88;
pub static RGB16I: GLenum = 0x8D89;
pub static RGBA8I: GLenum = 0x8D8E;
pub static RGB8I: GLenum = 0x8D8F;
pub static RED_INTEGER: GLenum = 0x8D94;
pub static GREEN_INTEGER: GLenum = 0x8D95;
pub static BLUE_INTEGER: GLenum = 0x8D96;
pub static RGB_INTEGER: GLenum = 0x8D98;
pub static RGBA_INTEGER: GLenum = 0x8D99;
pub static BGR_INTEGER: GLenum = 0x8D9A;
pub static BGRA_INTEGER: GLenum = 0x8D9B;
pub static INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub static FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub static FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub static FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub static FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub static COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub static COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub static COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub static COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub static SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub static SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub static SAMPLER_BUFFER: GLenum = 0x8DC2;
pub static SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub static SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub static SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub static UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub static UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub static UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub static INT_SAMPLER_1D: GLenum = 0x8DC9;
pub static INT_SAMPLER_2D: GLenum = 0x8DCA;
pub static INT_SAMPLER_3D: GLenum = 0x8DCB;
pub static INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub static INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub static INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub static INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub static INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub static UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub static UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub static UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub static UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub static UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub static UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub static UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub static UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub static GEOMETRY_SHADER: GLenum = 0x8DD9;
pub static MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub static MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub static MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub static ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub static ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub static MAX_SUBROUTINES: GLenum = 0x8DE7;
pub static MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub static LOW_FLOAT: GLenum = 0x8DF0;
pub static MEDIUM_FLOAT: GLenum = 0x8DF1;
pub static HIGH_FLOAT: GLenum = 0x8DF2;
pub static LOW_INT: GLenum = 0x8DF3;
pub static MEDIUM_INT: GLenum = 0x8DF4;
pub static HIGH_INT: GLenum = 0x8DF5;
pub static SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub static NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub static SHADER_COMPILER: GLenum = 0x8DFA;
pub static MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub static MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub static MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub static QUERY_WAIT: GLenum = 0x8E13;
pub static QUERY_NO_WAIT: GLenum = 0x8E14;
pub static QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub static QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub static MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub static MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub static TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub static TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub static TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub static TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
pub static TIMESTAMP: GLenum = 0x8E28;
pub static TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub static TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub static TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub static TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub static TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub static ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub static ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub static ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub static NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub static COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
pub static QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub static FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub static LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub static PROVOKING_VERTEX: GLenum = 0x8E4F;
pub static SAMPLE_POSITION: GLenum = 0x8E50;
pub static SAMPLE_MASK: GLenum = 0x8E51;
pub static SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub static MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub static MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
pub static MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
pub static MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
pub static FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub static MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub static MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
pub static MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
pub static MAX_VERTEX_STREAMS: GLenum = 0x8E71;
pub static PATCH_VERTICES: GLenum = 0x8E72;
pub static PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub static PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub static TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub static TESS_GEN_MODE: GLenum = 0x8E76;
pub static TESS_GEN_SPACING: GLenum = 0x8E77;
pub static TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub static TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub static ISOLINES: GLenum = 0x8E7A;
pub static FRACTIONAL_ODD: GLenum = 0x8E7B;
pub static FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub static MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub static MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub static MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub static MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub static MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub static MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub static MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub static MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub static MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub static MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub static TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub static TESS_CONTROL_SHADER: GLenum = 0x8E88;
pub static MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub static MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub static COPY_READ_BUFFER: GLenum = 0x8F36;
pub static COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub static MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub static MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub static MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
pub static IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub static IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub static IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub static IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub static IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub static DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub static DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
pub static DOUBLE_MAT2: GLenum = 0x8F46;
pub static DOUBLE_MAT3: GLenum = 0x8F47;
pub static DOUBLE_MAT4: GLenum = 0x8F48;
pub static DOUBLE_MAT2x3: GLenum = 0x8F49;
pub static DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub static DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub static DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub static DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub static DOUBLE_MAT4x3: GLenum = 0x8F4E;
pub static R8_SNORM: GLenum = 0x8F94;
pub static RG8_SNORM: GLenum = 0x8F95;
pub static RGB8_SNORM: GLenum = 0x8F96;
pub static RGBA8_SNORM: GLenum = 0x8F97;
pub static R16_SNORM: GLenum = 0x8F98;
pub static RG16_SNORM: GLenum = 0x8F99;
pub static RGB16_SNORM: GLenum = 0x8F9A;
pub static RGBA16_SNORM: GLenum = 0x8F9B;
pub static SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub static PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub static PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub static DOUBLE_VEC2: GLenum = 0x8FFC;
pub static DOUBLE_VEC3: GLenum = 0x8FFD;
pub static DOUBLE_VEC4: GLenum = 0x8FFE;
pub static TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub static TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub static PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub static SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub static SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub static INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub static UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
pub static IMAGE_1D: GLenum = 0x904C;
pub static IMAGE_2D: GLenum = 0x904D;
pub static IMAGE_3D: GLenum = 0x904E;
pub static IMAGE_2D_RECT: GLenum = 0x904F;
pub static IMAGE_CUBE: GLenum = 0x9050;
pub static IMAGE_BUFFER: GLenum = 0x9051;
pub static IMAGE_1D_ARRAY: GLenum = 0x9052;
pub static IMAGE_2D_ARRAY: GLenum = 0x9053;
pub static IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub static IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub static IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub static INT_IMAGE_1D: GLenum = 0x9057;
pub static INT_IMAGE_2D: GLenum = 0x9058;
pub static INT_IMAGE_3D: GLenum = 0x9059;
pub static INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub static INT_IMAGE_CUBE: GLenum = 0x905B;
pub static INT_IMAGE_BUFFER: GLenum = 0x905C;
pub static INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub static INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub static INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub static INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub static INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub static UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub static UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub static UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub static UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub static UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub static UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub static UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub static UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub static UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub static UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub static UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub static MAX_IMAGE_SAMPLES: GLenum = 0x906D;
pub static IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub static RGB10_A2UI: GLenum = 0x906F;
pub static MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
pub static IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub static IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub static IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub static MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub static MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub static MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub static MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub static MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub static MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
pub static SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub static SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub static SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub static SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub static MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub static MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub static MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub static MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub static MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub static MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub static MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub static MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub static MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub static SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub static DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
pub static MAX_COMPUTE_LOCAL_INVOCATIONS: GLenum = 0x90EB;
pub static UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub static DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub static DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub static TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub static PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub static TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub static PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub static TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub static TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub static TEXTURE_SAMPLES: GLenum = 0x9106;
pub static TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub static SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub static INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub static UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub static SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub static INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub static UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub static MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub static MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub static MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub static MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub static OBJECT_TYPE: GLenum = 0x9112;
pub static SYNC_CONDITION: GLenum = 0x9113;
pub static SYNC_STATUS: GLenum = 0x9114;
pub static SYNC_FLAGS: GLenum = 0x9115;
pub static SYNC_FENCE: GLenum = 0x9116;
pub static SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub static UNSIGNALED: GLenum = 0x9118;
pub static SIGNALED: GLenum = 0x9119;
pub static ALREADY_SIGNALED: GLenum = 0x911A;
pub static TIMEOUT_EXPIRED: GLenum = 0x911B;
pub static CONDITION_SATISFIED: GLenum = 0x911C;
pub static WAIT_FAILED: GLenum = 0x911D;
pub static BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub static BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub static BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub static MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub static MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub static MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub static MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub static CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub static UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub static UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub static UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub static UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub static PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub static PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub static PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub static PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
pub static TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
pub static MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub static MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub static DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub static DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub static DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub static DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub static TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub static TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub static TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
pub static COMPUTE_SHADER: GLenum = 0x91B9;
pub static MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub static MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub static MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub static MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub static MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub static COMPRESSED_R11_EAC: GLenum = 0x9270;
pub static COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub static COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub static COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub static COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub static COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub static COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub static COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub static COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub static COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub static ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub static ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub static ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub static ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub static ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub static ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub static ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub static ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub static MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub static MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub static MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub static MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub static MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub static MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub static MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub static MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub static MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub static MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub static MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub static MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub static MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub static ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub static UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub static UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
pub static MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub static DEBUG_OUTPUT: GLenum = 0x92E0;
pub static UNIFORM: GLenum = 0x92E1;
pub static UNIFORM_BLOCK: GLenum = 0x92E2;
pub static PROGRAM_INPUT: GLenum = 0x92E3;
pub static PROGRAM_OUTPUT: GLenum = 0x92E4;
pub static BUFFER_VARIABLE: GLenum = 0x92E5;
pub static SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub static IS_PER_PATCH: GLenum = 0x92E7;
pub static VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub static TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub static TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub static GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub static FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub static COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub static VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub static TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub static TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub static GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub static FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub static COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub static TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub static ACTIVE_RESOURCES: GLenum = 0x92F5;
pub static MAX_NAME_LENGTH: GLenum = 0x92F6;
pub static MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub static MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub static NAME_LENGTH: GLenum = 0x92F9;
pub static TYPE: GLenum = 0x92FA;
pub static ARRAY_SIZE: GLenum = 0x92FB;
pub static OFFSET: GLenum = 0x92FC;
pub static BLOCK_INDEX: GLenum = 0x92FD;
pub static ARRAY_STRIDE: GLenum = 0x92FE;
pub static MATRIX_STRIDE: GLenum = 0x92FF;
pub static IS_ROW_MAJOR: GLenum = 0x9300;
pub static ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub static BUFFER_BINDING: GLenum = 0x9302;
pub static BUFFER_DATA_SIZE: GLenum = 0x9303;
pub static NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub static ACTIVE_VARIABLES: GLenum = 0x9305;
pub static REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub static REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub static REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub static REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub static REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub static REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub static TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub static TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub static LOCATION: GLenum = 0x930E;
pub static LOCATION_INDEX: GLenum = 0x930F;
pub static FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub static FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub static FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub static FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub static FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub static MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub static MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub static MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub static MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
pub static NUM_SAMPLE_COUNTS: GLenum = 0x9380;

#[fixed_stack_segment] #[inline] pub fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) { unsafe { (storage::ActiveShaderProgram.f)(pipeline, program) } }
#[fixed_stack_segment] #[inline] pub fn ActiveTexture(texture: GLenum) { unsafe { (storage::ActiveTexture.f)(texture) } }
#[fixed_stack_segment] #[inline] pub fn AttachShader(program: GLuint, shader: GLuint) { unsafe { (storage::AttachShader.f)(program, shader) } }
#[fixed_stack_segment] #[inline] pub fn BeginConditionalRender(id: GLuint, mode: GLenum) { unsafe { (storage::BeginConditionalRender.f)(id, mode) } }
#[fixed_stack_segment] #[inline] pub fn BeginQuery(target: GLenum, id: GLuint) { unsafe { (storage::BeginQuery.f)(target, id) } }
#[fixed_stack_segment] #[inline] pub fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) { unsafe { (storage::BeginQueryIndexed.f)(target, index, id) } }
#[fixed_stack_segment] #[inline] pub fn BeginTransformFeedback(primitiveMode: GLenum) { unsafe { (storage::BeginTransformFeedback.f)(primitiveMode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar) { (storage::BindAttribLocation.f)(program, index, name) }
#[fixed_stack_segment] #[inline] pub fn BindBuffer(target: GLenum, buffer: GLuint) { unsafe { (storage::BindBuffer.f)(target, buffer) } }
#[fixed_stack_segment] #[inline] pub fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) { unsafe { (storage::BindBufferBase.f)(target, index, buffer) } }
#[fixed_stack_segment] #[inline] pub fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { unsafe { (storage::BindBufferRange.f)(target, index, buffer, offset, size) } }
#[fixed_stack_segment] #[inline] pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *GLchar) { (storage::BindFragDataLocation.f)(program, color, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *GLchar) { (storage::BindFragDataLocationIndexed.f)(program, colorNumber, index, name) }
#[fixed_stack_segment] #[inline] pub fn BindFramebuffer(target: GLenum, framebuffer: GLuint) { unsafe { (storage::BindFramebuffer.f)(target, framebuffer) } }
#[fixed_stack_segment] #[inline] pub fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) { unsafe { (storage::BindImageTexture.f)(unit, texture, level, layered, layer, access, format) } }
#[fixed_stack_segment] #[inline] pub fn BindProgramPipeline(pipeline: GLuint) { unsafe { (storage::BindProgramPipeline.f)(pipeline) } }
#[fixed_stack_segment] #[inline] pub fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) { unsafe { (storage::BindRenderbuffer.f)(target, renderbuffer) } }
#[fixed_stack_segment] #[inline] pub fn BindSampler(unit: GLuint, sampler: GLuint) { unsafe { (storage::BindSampler.f)(unit, sampler) } }
#[fixed_stack_segment] #[inline] pub fn BindTexture(target: GLenum, texture: GLuint) { unsafe { (storage::BindTexture.f)(target, texture) } }
#[fixed_stack_segment] #[inline] pub fn BindTransformFeedback(target: GLenum, id: GLuint) { unsafe { (storage::BindTransformFeedback.f)(target, id) } }
#[fixed_stack_segment] #[inline] pub fn BindVertexArray(array: GLuint) { unsafe { (storage::BindVertexArray.f)(array) } }
#[fixed_stack_segment] #[inline] pub fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) { unsafe { (storage::BindVertexBuffer.f)(bindingindex, buffer, offset, stride) } }
#[fixed_stack_segment] #[inline] pub fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::BlendColor.f)(red, green, blue, alpha) } }
#[fixed_stack_segment] #[inline] pub fn BlendEquation(mode: GLenum) { unsafe { (storage::BlendEquation.f)(mode) } }
#[fixed_stack_segment] #[inline] pub fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) { unsafe { (storage::BlendEquationSeparate.f)(modeRGB, modeAlpha) } }
#[fixed_stack_segment] #[inline] pub fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) { unsafe { (storage::BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) } }
#[fixed_stack_segment] #[inline] pub fn BlendEquationi(buf: GLuint, mode: GLenum) { unsafe { (storage::BlendEquationi.f)(buf, mode) } }
#[fixed_stack_segment] #[inline] pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum) { unsafe { (storage::BlendFunc.f)(sfactor, dfactor) } }
#[fixed_stack_segment] #[inline] pub fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { unsafe { (storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) } }
#[fixed_stack_segment] #[inline] pub fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) { unsafe { (storage::BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) } }
#[fixed_stack_segment] #[inline] pub fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) { unsafe { (storage::BlendFunci.f)(buf, src, dst) } }
#[fixed_stack_segment] #[inline] pub fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { unsafe { (storage::BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } }
#[fixed_stack_segment] #[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *GLvoid, usage: GLenum) { (storage::BufferData.f)(target, size, data, usage) }
#[fixed_stack_segment] #[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid) { (storage::BufferSubData.f)(target, offset, size, data) }
#[fixed_stack_segment] #[inline] pub fn CheckFramebufferStatus(target: GLenum) -> GLenum { unsafe { (storage::CheckFramebufferStatus.f)(target) } }
#[fixed_stack_segment] #[inline] pub fn ClampColor(target: GLenum, clamp: GLenum) { unsafe { (storage::ClampColor.f)(target, clamp) } }
#[fixed_stack_segment] #[inline] pub fn Clear(mask: GLbitfield) { unsafe { (storage::Clear.f)(mask) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *c_void) { (storage::ClearBufferData.f)(target, internalformat, format, type_, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *c_void) { (storage::ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) }
#[fixed_stack_segment] #[inline] pub fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { unsafe { (storage::ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *GLfloat) { (storage::ClearBufferfv.f)(buffer, drawbuffer, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *GLint) { (storage::ClearBufferiv.f)(buffer, drawbuffer, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *GLuint) { (storage::ClearBufferuiv.f)(buffer, drawbuffer, value) }
#[fixed_stack_segment] #[inline] pub fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { unsafe { (storage::ClearColor.f)(red, green, blue, alpha) } }
#[fixed_stack_segment] #[inline] pub fn ClearDepth(depth: GLdouble) { unsafe { (storage::ClearDepth.f)(depth) } }
#[fixed_stack_segment] #[inline] pub fn ClearDepthf(d: GLfloat) { unsafe { (storage::ClearDepthf.f)(d) } }
#[fixed_stack_segment] #[inline] pub fn ClearStencil(s: GLint) { unsafe { (storage::ClearStencil.f)(s) } }
#[fixed_stack_segment] #[inline] pub fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { unsafe { (storage::ClientWaitSync.f)(sync, flags, timeout) } }
#[fixed_stack_segment] #[inline] pub fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { unsafe { (storage::ColorMask.f)(red, green, blue, alpha) } }
#[fixed_stack_segment] #[inline] pub fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) { unsafe { (storage::ColorMaski.f)(index, r, g, b, a) } }
#[fixed_stack_segment] #[inline] pub fn ColorP3ui(type_: GLenum, color: GLuint) { unsafe { (storage::ColorP3ui.f)(type_, color) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ColorP3uiv(type_: GLenum, color: *GLuint) { (storage::ColorP3uiv.f)(type_, color) }
#[fixed_stack_segment] #[inline] pub fn ColorP4ui(type_: GLenum, color: GLuint) { unsafe { (storage::ColorP4ui.f)(type_, color) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ColorP4uiv(type_: GLenum, color: *GLuint) { (storage::ColorP4uiv.f)(type_, color) }
#[fixed_stack_segment] #[inline] pub fn CompileShader(shader: GLuint) { unsafe { (storage::CompileShader.f)(shader) } }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid) { (storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[fixed_stack_segment] #[inline] pub fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { unsafe { (storage::CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) } }
#[fixed_stack_segment] #[inline] pub fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) { unsafe { (storage::CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) } }
#[fixed_stack_segment] #[inline] pub fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) } }
#[fixed_stack_segment] #[inline] pub fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { unsafe { (storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) } }
#[fixed_stack_segment] #[inline] pub fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { unsafe { (storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) } }
#[fixed_stack_segment] #[inline] pub fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) } }
#[fixed_stack_segment] #[inline] pub fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) } }
#[fixed_stack_segment] #[inline] pub fn CreateProgram() -> GLuint { unsafe { (storage::CreateProgram.f)() } }
#[fixed_stack_segment] #[inline] pub fn CreateShader(type_: GLenum) -> GLuint { unsafe { (storage::CreateShader.f)(type_) } }
#[fixed_stack_segment] #[inline] pub unsafe fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: **GLchar) -> GLuint { (storage::CreateShaderProgramv.f)(type_, count, strings) }
#[fixed_stack_segment] #[inline] pub fn CullFace(mode: GLenum) { unsafe { (storage::CullFace.f)(mode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *c_void) { (storage::DebugMessageCallback.f)(callback, userParam) }
#[fixed_stack_segment] #[inline] pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *GLuint, enabled: GLboolean) { (storage::DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
#[fixed_stack_segment] #[inline] pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *GLchar) { (storage::DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *GLuint) { (storage::DeleteBuffers.f)(n, buffers) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *GLuint) { (storage::DeleteFramebuffers.f)(n, framebuffers) }
#[fixed_stack_segment] #[inline] pub fn DeleteProgram(program: GLuint) { unsafe { (storage::DeleteProgram.f)(program) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *GLuint) { (storage::DeleteProgramPipelines.f)(n, pipelines) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *GLuint) { (storage::DeleteQueries.f)(n, ids) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *GLuint) { (storage::DeleteRenderbuffers.f)(n, renderbuffers) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *GLuint) { (storage::DeleteSamplers.f)(count, samplers) }
#[fixed_stack_segment] #[inline] pub fn DeleteShader(shader: GLuint) { unsafe { (storage::DeleteShader.f)(shader) } }
#[fixed_stack_segment] #[inline] pub fn DeleteSync(sync: GLsync) { unsafe { (storage::DeleteSync.f)(sync) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *GLuint) { (storage::DeleteTextures.f)(n, textures) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *GLuint) { (storage::DeleteTransformFeedbacks.f)(n, ids) }
#[fixed_stack_segment] #[inline] pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *GLuint) { (storage::DeleteVertexArrays.f)(n, arrays) }
#[fixed_stack_segment] #[inline] pub fn DepthFunc(func: GLenum) { unsafe { (storage::DepthFunc.f)(func) } }
#[fixed_stack_segment] #[inline] pub fn DepthMask(flag: GLboolean) { unsafe { (storage::DepthMask.f)(flag) } }
#[fixed_stack_segment] #[inline] pub fn DepthRange(near: GLdouble, far: GLdouble) { unsafe { (storage::DepthRange.f)(near, far) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *GLdouble) { (storage::DepthRangeArrayv.f)(first, count, v) }
#[fixed_stack_segment] #[inline] pub fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) { unsafe { (storage::DepthRangeIndexed.f)(index, n, f) } }
#[fixed_stack_segment] #[inline] pub fn DepthRangef(n: GLfloat, f: GLfloat) { unsafe { (storage::DepthRangef.f)(n, f) } }
#[fixed_stack_segment] #[inline] pub fn DetachShader(program: GLuint, shader: GLuint) { unsafe { (storage::DetachShader.f)(program, shader) } }
#[fixed_stack_segment] #[inline] pub fn Disable(cap: GLenum) { unsafe { (storage::Disable.f)(cap) } }
#[fixed_stack_segment] #[inline] pub fn DisableVertexAttribArray(index: GLuint) { unsafe { (storage::DisableVertexAttribArray.f)(index) } }
#[fixed_stack_segment] #[inline] pub fn Disablei(target: GLenum, index: GLuint) { unsafe { (storage::Disablei.f)(target, index) } }
#[fixed_stack_segment] #[inline] pub fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) { unsafe { (storage::DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) } }
#[fixed_stack_segment] #[inline] pub fn DispatchComputeIndirect(indirect: GLintptr) { unsafe { (storage::DispatchComputeIndirect.f)(indirect) } }
#[fixed_stack_segment] #[inline] pub fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) { unsafe { (storage::DrawArrays.f)(mode, first, count) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *GLvoid) { (storage::DrawArraysIndirect.f)(mode, indirect) }
#[fixed_stack_segment] #[inline] pub fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) { unsafe { (storage::DrawArraysInstanced.f)(mode, first, count, instancecount) } }
#[fixed_stack_segment] #[inline] pub fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) { unsafe { (storage::DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) } }
#[fixed_stack_segment] #[inline] pub fn DrawBuffer(mode: GLenum) { unsafe { (storage::DrawBuffer.f)(mode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *GLenum) { (storage::DrawBuffers.f)(n, bufs) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid) { (storage::DrawElements.f)(mode, count, type_, indices) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) { (storage::DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *GLvoid) { (storage::DrawElementsIndirect.f)(mode, type_, indirect) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei) { (storage::DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, baseinstance: GLuint) { (storage::DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, basevertex: GLint) { (storage::DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) { (storage::DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid) { (storage::DrawRangeElements.f)(mode, start, end, count, type_, indices) }
#[fixed_stack_segment] #[inline] pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint) { (storage::DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
#[fixed_stack_segment] #[inline] pub fn DrawTransformFeedback(mode: GLenum, id: GLuint) { unsafe { (storage::DrawTransformFeedback.f)(mode, id) } }
#[fixed_stack_segment] #[inline] pub fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) { unsafe { (storage::DrawTransformFeedbackInstanced.f)(mode, id, instancecount) } }
#[fixed_stack_segment] #[inline] pub fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) { unsafe { (storage::DrawTransformFeedbackStream.f)(mode, id, stream) } }
#[fixed_stack_segment] #[inline] pub fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) { unsafe { (storage::DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) } }
#[fixed_stack_segment] #[inline] pub fn Enable(cap: GLenum) { unsafe { (storage::Enable.f)(cap) } }
#[fixed_stack_segment] #[inline] pub fn EnableVertexAttribArray(index: GLuint) { unsafe { (storage::EnableVertexAttribArray.f)(index) } }
#[fixed_stack_segment] #[inline] pub fn Enablei(target: GLenum, index: GLuint) { unsafe { (storage::Enablei.f)(target, index) } }
#[fixed_stack_segment] #[inline] pub fn EndConditionalRender() { unsafe { (storage::EndConditionalRender.f)() } }
#[fixed_stack_segment] #[inline] pub fn EndQuery(target: GLenum) { unsafe { (storage::EndQuery.f)(target) } }
#[fixed_stack_segment] #[inline] pub fn EndQueryIndexed(target: GLenum, index: GLuint) { unsafe { (storage::EndQueryIndexed.f)(target, index) } }
#[fixed_stack_segment] #[inline] pub fn EndTransformFeedback() { unsafe { (storage::EndTransformFeedback.f)() } }
#[fixed_stack_segment] #[inline] pub fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { unsafe { (storage::FenceSync.f)(condition, flags) } }
#[fixed_stack_segment] #[inline] pub fn Finish() { unsafe { (storage::Finish.f)() } }
#[fixed_stack_segment] #[inline] pub fn Flush() { unsafe { (storage::Flush.f)() } }
#[fixed_stack_segment] #[inline] pub fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) { unsafe { (storage::FlushMappedBufferRange.f)(target, offset, length) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::FramebufferParameteri.f)(target, pname, param) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { unsafe { (storage::FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture.f)(target, attachment, texture, level) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture1D.f)(target, attachment, textarget, texture, level) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { unsafe { (storage::FramebufferTexture2D.f)(target, attachment, textarget, texture, level) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) { unsafe { (storage::FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) } }
#[fixed_stack_segment] #[inline] pub fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { unsafe { (storage::FramebufferTextureLayer.f)(target, attachment, texture, level, layer) } }
#[fixed_stack_segment] #[inline] pub fn FrontFace(mode: GLenum) { unsafe { (storage::FrontFace.f)(mode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) { (storage::GenBuffers.f)(n, buffers) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) { (storage::GenFramebuffers.f)(n, framebuffers) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) { (storage::GenProgramPipelines.f)(n, pipelines) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) { (storage::GenQueries.f)(n, ids) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) { (storage::GenRenderbuffers.f)(n, renderbuffers) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) { (storage::GenSamplers.f)(count, samplers) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) { (storage::GenTextures.f)(n, textures) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) { (storage::GenTransformFeedbacks.f)(n, ids) }
#[fixed_stack_segment] #[inline] pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) { (storage::GenVertexArrays.f)(n, arrays) }
#[fixed_stack_segment] #[inline] pub fn GenerateMipmap(target: GLenum) { unsafe { (storage::GenerateMipmap.f)(target) } }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { (storage::GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { (storage::GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) { (storage::GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { (storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) { (storage::GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) { (storage::GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *mut GLint) { (storage::GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) { (storage::GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetAttribLocation.f)(program, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) { (storage::GetBooleani_v.f)(target, index, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBooleanv(pname: GLenum, params: *mut GLboolean) { (storage::GetBooleanv.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) { (storage::GetBufferParameteri64v.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetBufferParameteriv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut GLvoid) { (storage::GetBufferPointerv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid) { (storage::GetBufferSubData.f)(target, offset, size, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut GLvoid) { (storage::GetCompressedTexImage.f)(target, level, img) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { (storage::GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) { (storage::GetDoublei_v.f)(target, index, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetDoublev(pname: GLenum, params: *mut GLdouble) { (storage::GetDoublev.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub fn GetError() -> GLenum { unsafe { (storage::GetError.f)() } }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) { (storage::GetFloati_v.f)(target, index, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFloatv(pname: GLenum, params: *mut GLfloat) { (storage::GetFloatv.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFragDataIndex(program: GLuint, name: *GLchar) -> GLint { (storage::GetFragDataIndex.f)(program, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFragDataLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetFragDataLocation.f)(program, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetFramebufferParameteriv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) { (storage::GetInteger64i_v.f)(target, index, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetInteger64v(pname: GLenum, params: *mut GLint64) { (storage::GetInteger64v.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) { (storage::GetIntegeri_v.f)(target, index, data) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetIntegerv(pname: GLenum, params: *mut GLint) { (storage::GetIntegerv.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) { (storage::GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) { (storage::GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) { (storage::GetMultisamplefv.f)(pname, index, val) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) { (storage::GetObjectLabel.f)(identifier, name, bufSize, length, label) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetObjectPtrLabel(ptr: *c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) { (storage::GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut GLvoid) { (storage::GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetProgramPipelineiv.f)(pipeline, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLuint { (storage::GetProgramResourceIndex.f)(program, programInterface, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint { (storage::GetProgramResourceLocation.f)(program, programInterface, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint { (storage::GetProgramResourceLocationIndex.f)(program, programInterface, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { (storage::GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) { (storage::GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) { (storage::GetProgramStageiv.f)(program, shadertype, pname, values) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetProgramiv.f)(program, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetQueryIndexediv.f)(target, index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) { (storage::GetQueryObjecti64v.f)(id, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetQueryObjectiv.f)(id, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) { (storage::GetQueryObjectui64v.f)(id, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetQueryObjectuiv.f)(id, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetQueryiv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetRenderbufferParameteriv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetSamplerParameterIiv.f)(sampler, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetSamplerParameterIuiv.f)(sampler, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) { (storage::GetSamplerParameterfv.f)(sampler, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetSamplerParameteriv.f)(sampler, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) { (storage::GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) { (storage::GetShaderSource.f)(shader, bufSize, length, source) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetShaderiv.f)(shader, pname, params) }
#[fixed_stack_segment] #[inline] pub fn GetString(name: GLenum) -> *GLubyte { unsafe { (storage::GetString.f)(name) } }
#[fixed_stack_segment] #[inline] pub fn GetStringi(name: GLenum, index: GLuint) -> *GLubyte { unsafe { (storage::GetStringi.f)(name, index) } }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLuint { (storage::GetSubroutineIndex.f)(program, shadertype, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLint { (storage::GetSubroutineUniformLocation.f)(program, shadertype, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) { (storage::GetSynciv.f)(sync, pname, bufSize, length, values) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut GLvoid) { (storage::GetTexImage.f)(target, level, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) { (storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) { (storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexParameterIiv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) { (storage::GetTexParameterIuiv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) { (storage::GetTexParameterfv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { (storage::GetTexParameteriv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) { (storage::GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *GLchar) -> GLuint { (storage::GetUniformBlockIndex.f)(program, uniformBlockName) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: **GLchar, uniformIndices: *mut GLuint) { (storage::GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint { (storage::GetUniformLocation.f)(program, name) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) { (storage::GetUniformSubroutineuiv.f)(shadertype, location, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) { (storage::GetUniformdv.f)(program, location, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) { (storage::GetUniformfv.f)(program, location, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) { (storage::GetUniformiv.f)(program, location, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) { (storage::GetUniformuiv.f)(program, location, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetVertexAttribIiv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) { (storage::GetVertexAttribIuiv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { (storage::GetVertexAttribLdv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut GLvoid) { (storage::GetVertexAttribPointerv.f)(index, pname, pointer) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { (storage::GetVertexAttribdv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) { (storage::GetVertexAttribfv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) { (storage::GetVertexAttribiv.f)(index, pname, params) }
#[fixed_stack_segment] #[inline] pub fn Hint(target: GLenum, mode: GLenum) { unsafe { (storage::Hint.f)(target, mode) } }
#[fixed_stack_segment] #[inline] pub fn InvalidateBufferData(buffer: GLuint) { unsafe { (storage::InvalidateBufferData.f)(buffer) } }
#[fixed_stack_segment] #[inline] pub fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) { unsafe { (storage::InvalidateBufferSubData.f)(buffer, offset, length) } }
#[fixed_stack_segment] #[inline] pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *GLenum) { (storage::InvalidateFramebuffer.f)(target, numAttachments, attachments) }
#[fixed_stack_segment] #[inline] pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (storage::InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
#[fixed_stack_segment] #[inline] pub fn InvalidateTexImage(texture: GLuint, level: GLint) { unsafe { (storage::InvalidateTexImage.f)(texture, level) } }
#[fixed_stack_segment] #[inline] pub fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) { unsafe { (storage::InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) } }
#[fixed_stack_segment] #[inline] pub fn IsBuffer(buffer: GLuint) -> GLboolean { unsafe { (storage::IsBuffer.f)(buffer) } }
#[fixed_stack_segment] #[inline] pub fn IsEnabled(cap: GLenum) -> GLboolean { unsafe { (storage::IsEnabled.f)(cap) } }
#[fixed_stack_segment] #[inline] pub fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { unsafe { (storage::IsEnabledi.f)(target, index) } }
#[fixed_stack_segment] #[inline] pub fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { unsafe { (storage::IsFramebuffer.f)(framebuffer) } }
#[fixed_stack_segment] #[inline] pub fn IsProgram(program: GLuint) -> GLboolean { unsafe { (storage::IsProgram.f)(program) } }
#[fixed_stack_segment] #[inline] pub fn IsProgramPipeline(pipeline: GLuint) -> GLboolean { unsafe { (storage::IsProgramPipeline.f)(pipeline) } }
#[fixed_stack_segment] #[inline] pub fn IsQuery(id: GLuint) -> GLboolean { unsafe { (storage::IsQuery.f)(id) } }
#[fixed_stack_segment] #[inline] pub fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { unsafe { (storage::IsRenderbuffer.f)(renderbuffer) } }
#[fixed_stack_segment] #[inline] pub fn IsSampler(sampler: GLuint) -> GLboolean { unsafe { (storage::IsSampler.f)(sampler) } }
#[fixed_stack_segment] #[inline] pub fn IsShader(shader: GLuint) -> GLboolean { unsafe { (storage::IsShader.f)(shader) } }
#[fixed_stack_segment] #[inline] pub fn IsSync(sync: GLsync) -> GLboolean { unsafe { (storage::IsSync.f)(sync) } }
#[fixed_stack_segment] #[inline] pub fn IsTexture(texture: GLuint) -> GLboolean { unsafe { (storage::IsTexture.f)(texture) } }
#[fixed_stack_segment] #[inline] pub fn IsTransformFeedback(id: GLuint) -> GLboolean { unsafe { (storage::IsTransformFeedback.f)(id) } }
#[fixed_stack_segment] #[inline] pub fn IsVertexArray(array: GLuint) -> GLboolean { unsafe { (storage::IsVertexArray.f)(array) } }
#[fixed_stack_segment] #[inline] pub fn LineWidth(width: GLfloat) { unsafe { (storage::LineWidth.f)(width) } }
#[fixed_stack_segment] #[inline] pub fn LinkProgram(program: GLuint) { unsafe { (storage::LinkProgram.f)(program) } }
#[fixed_stack_segment] #[inline] pub fn LogicOp(opcode: GLenum) { unsafe { (storage::LogicOp.f)(opcode) } }
#[fixed_stack_segment] #[inline] pub fn MapBuffer(target: GLenum, access: GLenum) -> *c_void { unsafe { (storage::MapBuffer.f)(target, access) } }
#[fixed_stack_segment] #[inline] pub fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *c_void { unsafe { (storage::MapBufferRange.f)(target, offset, length, access) } }
#[fixed_stack_segment] #[inline] pub fn MemoryBarrier(barriers: GLbitfield) { unsafe { (storage::MemoryBarrier.f)(barriers) } }
#[fixed_stack_segment] #[inline] pub fn MinSampleShading(value: GLfloat) { unsafe { (storage::MinSampleShading.f)(value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei) { (storage::MultiDrawArrays.f)(mode, first, count, drawcount) }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiDrawArraysIndirect(mode: GLenum, indirect: *c_void, drawcount: GLsizei, stride: GLsizei) { (storage::MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei) { (storage::MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei, basevertex: *GLint) { (storage::MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *c_void, drawcount: GLsizei, stride: GLsizei) { (storage::MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) }
#[fixed_stack_segment] #[inline] pub fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) { unsafe { (storage::MultiTexCoordP1ui.f)(texture, type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *GLuint) { (storage::MultiTexCoordP1uiv.f)(texture, type_, coords) }
#[fixed_stack_segment] #[inline] pub fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) { unsafe { (storage::MultiTexCoordP2ui.f)(texture, type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *GLuint) { (storage::MultiTexCoordP2uiv.f)(texture, type_, coords) }
#[fixed_stack_segment] #[inline] pub fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) { unsafe { (storage::MultiTexCoordP3ui.f)(texture, type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *GLuint) { (storage::MultiTexCoordP3uiv.f)(texture, type_, coords) }
#[fixed_stack_segment] #[inline] pub fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) { unsafe { (storage::MultiTexCoordP4ui.f)(texture, type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *GLuint) { (storage::MultiTexCoordP4uiv.f)(texture, type_, coords) }
#[fixed_stack_segment] #[inline] pub fn NormalP3ui(type_: GLenum, coords: GLuint) { unsafe { (storage::NormalP3ui.f)(type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn NormalP3uiv(type_: GLenum, coords: *GLuint) { (storage::NormalP3uiv.f)(type_, coords) }
#[fixed_stack_segment] #[inline] pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *GLchar) { (storage::ObjectLabel.f)(identifier, name, length, label) }
#[fixed_stack_segment] #[inline] pub unsafe fn ObjectPtrLabel(ptr: *c_void, length: GLsizei, label: *GLchar) { (storage::ObjectPtrLabel.f)(ptr, length, label) }
#[fixed_stack_segment] #[inline] pub unsafe fn PatchParameterfv(pname: GLenum, values: *GLfloat) { (storage::PatchParameterfv.f)(pname, values) }
#[fixed_stack_segment] #[inline] pub fn PatchParameteri(pname: GLenum, value: GLint) { unsafe { (storage::PatchParameteri.f)(pname, value) } }
#[fixed_stack_segment] #[inline] pub fn PauseTransformFeedback() { unsafe { (storage::PauseTransformFeedback.f)() } }
#[fixed_stack_segment] #[inline] pub fn PixelStoref(pname: GLenum, param: GLfloat) { unsafe { (storage::PixelStoref.f)(pname, param) } }
#[fixed_stack_segment] #[inline] pub fn PixelStorei(pname: GLenum, param: GLint) { unsafe { (storage::PixelStorei.f)(pname, param) } }
#[fixed_stack_segment] #[inline] pub fn PointParameterf(pname: GLenum, param: GLfloat) { unsafe { (storage::PointParameterf.f)(pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *GLfloat) { (storage::PointParameterfv.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub fn PointParameteri(pname: GLenum, param: GLint) { unsafe { (storage::PointParameteri.f)(pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *GLint) { (storage::PointParameteriv.f)(pname, params) }
#[fixed_stack_segment] #[inline] pub fn PointSize(size: GLfloat) { unsafe { (storage::PointSize.f)(size) } }
#[fixed_stack_segment] #[inline] pub fn PolygonMode(face: GLenum, mode: GLenum) { unsafe { (storage::PolygonMode.f)(face, mode) } }
#[fixed_stack_segment] #[inline] pub fn PolygonOffset(factor: GLfloat, units: GLfloat) { unsafe { (storage::PolygonOffset.f)(factor, units) } }
#[fixed_stack_segment] #[inline] pub fn PopDebugGroup() { unsafe { (storage::PopDebugGroup.f)() } }
#[fixed_stack_segment] #[inline] pub fn PrimitiveRestartIndex(index: GLuint) { unsafe { (storage::PrimitiveRestartIndex.f)(index) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *GLvoid, length: GLsizei) { (storage::ProgramBinary.f)(program, binaryFormat, binary, length) }
#[fixed_stack_segment] #[inline] pub fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) { unsafe { (storage::ProgramParameteri.f)(program, pname, value) } }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) { unsafe { (storage::ProgramUniform1d.f)(program, location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (storage::ProgramUniform1dv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) { unsafe { (storage::ProgramUniform1f.f)(program, location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (storage::ProgramUniform1fv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) { unsafe { (storage::ProgramUniform1i.f)(program, location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (storage::ProgramUniform1iv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) { unsafe { (storage::ProgramUniform1ui.f)(program, location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (storage::ProgramUniform1uiv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) { unsafe { (storage::ProgramUniform2d.f)(program, location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (storage::ProgramUniform2dv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) { unsafe { (storage::ProgramUniform2f.f)(program, location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (storage::ProgramUniform2fv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) { unsafe { (storage::ProgramUniform2i.f)(program, location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (storage::ProgramUniform2iv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) { unsafe { (storage::ProgramUniform2ui.f)(program, location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (storage::ProgramUniform2uiv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) { unsafe { (storage::ProgramUniform3d.f)(program, location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (storage::ProgramUniform3dv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { unsafe { (storage::ProgramUniform3f.f)(program, location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (storage::ProgramUniform3fv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) { unsafe { (storage::ProgramUniform3i.f)(program, location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (storage::ProgramUniform3iv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { unsafe { (storage::ProgramUniform3ui.f)(program, location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (storage::ProgramUniform3uiv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) { unsafe { (storage::ProgramUniform4d.f)(program, location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble) { (storage::ProgramUniform4dv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { unsafe { (storage::ProgramUniform4f.f)(program, location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat) { (storage::ProgramUniform4fv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { unsafe { (storage::ProgramUniform4i.f)(program, location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint) { (storage::ProgramUniform4iv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { unsafe { (storage::ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint) { (storage::ProgramUniform4uiv.f)(program, location, count, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub fn ProvokingVertex(mode: GLenum) { unsafe { (storage::ProvokingVertex.f)(mode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *GLchar) { (storage::PushDebugGroup.f)(source, id, length, message) }
#[fixed_stack_segment] #[inline] pub fn QueryCounter(id: GLuint, target: GLenum) { unsafe { (storage::QueryCounter.f)(id, target) } }
#[fixed_stack_segment] #[inline] pub fn ReadBuffer(mode: GLenum) { unsafe { (storage::ReadBuffer.f)(mode) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut GLvoid) { (storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub fn ReleaseShaderCompiler() { unsafe { (storage::ReleaseShaderCompiler.f)() } }
#[fixed_stack_segment] #[inline] pub fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) { unsafe { (storage::RenderbufferStorage.f)(target, internalformat, width, height) } }
#[fixed_stack_segment] #[inline] pub fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { unsafe { (storage::RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) } }
#[fixed_stack_segment] #[inline] pub fn ResumeTransformFeedback() { unsafe { (storage::ResumeTransformFeedback.f)() } }
#[fixed_stack_segment] #[inline] pub fn SampleCoverage(value: GLfloat, invert: GLboolean) { unsafe { (storage::SampleCoverage.f)(value, invert) } }
#[fixed_stack_segment] #[inline] pub fn SampleMaski(index: GLuint, mask: GLbitfield) { unsafe { (storage::SampleMaski.f)(index, mask) } }
#[fixed_stack_segment] #[inline] pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *GLint) { (storage::SamplerParameterIiv.f)(sampler, pname, param) }
#[fixed_stack_segment] #[inline] pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *GLuint) { (storage::SamplerParameterIuiv.f)(sampler, pname, param) }
#[fixed_stack_segment] #[inline] pub fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) { unsafe { (storage::SamplerParameterf.f)(sampler, pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *GLfloat) { (storage::SamplerParameterfv.f)(sampler, pname, param) }
#[fixed_stack_segment] #[inline] pub fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) { unsafe { (storage::SamplerParameteri.f)(sampler, pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *GLint) { (storage::SamplerParameteriv.f)(sampler, pname, param) }
#[fixed_stack_segment] #[inline] pub fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Scissor.f)(x, y, width, height) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *GLint) { (storage::ScissorArrayv.f)(first, count, v) }
#[fixed_stack_segment] #[inline] pub fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::ScissorIndexed.f)(index, left, bottom, width, height) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ScissorIndexedv(index: GLuint, v: *GLint) { (storage::ScissorIndexedv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn SecondaryColorP3ui(type_: GLenum, color: GLuint) { unsafe { (storage::SecondaryColorP3ui.f)(type_, color) } }
#[fixed_stack_segment] #[inline] pub unsafe fn SecondaryColorP3uiv(type_: GLenum, color: *GLuint) { (storage::SecondaryColorP3uiv.f)(type_, color) }
#[fixed_stack_segment] #[inline] pub unsafe fn ShaderBinary(count: GLsizei, shaders: *GLuint, binaryformat: GLenum, binary: *GLvoid, length: GLsizei) { (storage::ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
#[fixed_stack_segment] #[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint) { (storage::ShaderSource.f)(shader, count, string, length) }
#[fixed_stack_segment] #[inline] pub fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) { unsafe { (storage::ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) } }
#[fixed_stack_segment] #[inline] pub fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFunc.f)(func, ref_, mask) } }
#[fixed_stack_segment] #[inline] pub fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) { unsafe { (storage::StencilFuncSeparate.f)(face, func, ref_, mask) } }
#[fixed_stack_segment] #[inline] pub fn StencilMask(mask: GLuint) { unsafe { (storage::StencilMask.f)(mask) } }
#[fixed_stack_segment] #[inline] pub fn StencilMaskSeparate(face: GLenum, mask: GLuint) { unsafe { (storage::StencilMaskSeparate.f)(face, mask) } }
#[fixed_stack_segment] #[inline] pub fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) { unsafe { (storage::StencilOp.f)(fail, zfail, zpass) } }
#[fixed_stack_segment] #[inline] pub fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { unsafe { (storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass) } }
#[fixed_stack_segment] #[inline] pub fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) { unsafe { (storage::TexBuffer.f)(target, internalformat, buffer) } }
#[fixed_stack_segment] #[inline] pub fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { unsafe { (storage::TexBufferRange.f)(target, internalformat, buffer, offset, size) } }
#[fixed_stack_segment] #[inline] pub fn TexCoordP1ui(type_: GLenum, coords: GLuint) { unsafe { (storage::TexCoordP1ui.f)(type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexCoordP1uiv(type_: GLenum, coords: *GLuint) { (storage::TexCoordP1uiv.f)(type_, coords) }
#[fixed_stack_segment] #[inline] pub fn TexCoordP2ui(type_: GLenum, coords: GLuint) { unsafe { (storage::TexCoordP2ui.f)(type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexCoordP2uiv(type_: GLenum, coords: *GLuint) { (storage::TexCoordP2uiv.f)(type_, coords) }
#[fixed_stack_segment] #[inline] pub fn TexCoordP3ui(type_: GLenum, coords: GLuint) { unsafe { (storage::TexCoordP3ui.f)(type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexCoordP3uiv(type_: GLenum, coords: *GLuint) { (storage::TexCoordP3uiv.f)(type_, coords) }
#[fixed_stack_segment] #[inline] pub fn TexCoordP4ui(type_: GLenum, coords: GLuint) { unsafe { (storage::TexCoordP4ui.f)(type_, coords) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexCoordP4uiv(type_: GLenum, coords: *GLuint) { (storage::TexCoordP4uiv.f)(type_, coords) }
#[fixed_stack_segment] #[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexParameterIiv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *GLuint) { (storage::TexParameterIuiv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) { unsafe { (storage::TexParameterf.f)(target, pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat) { (storage::TexParameterfv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) { unsafe { (storage::TexParameteri.f)(target, pname, param) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint) { (storage::TexParameteriv.f)(target, pname, params) }
#[fixed_stack_segment] #[inline] pub fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) { unsafe { (storage::TexStorage1D.f)(target, levels, internalformat, width) } }
#[fixed_stack_segment] #[inline] pub fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { unsafe { (storage::TexStorage2D.f)(target, levels, internalformat, width, height) } }
#[fixed_stack_segment] #[inline] pub fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) } }
#[fixed_stack_segment] #[inline] pub fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) { unsafe { (storage::TexStorage3D.f)(target, levels, internalformat, width, height, depth) } }
#[fixed_stack_segment] #[inline] pub fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { unsafe { (storage::TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid) { (storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[fixed_stack_segment] #[inline] pub fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) { unsafe { (storage::TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) } }
#[fixed_stack_segment] #[inline] pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: **GLchar, bufferMode: GLenum) { (storage::TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
#[fixed_stack_segment] #[inline] pub fn Uniform1d(location: GLint, x: GLdouble) { unsafe { (storage::Uniform1d.f)(location, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *GLdouble) { (storage::Uniform1dv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform1f(location: GLint, v0: GLfloat) { unsafe { (storage::Uniform1f.f)(location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform1fv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform1i(location: GLint, v0: GLint) { unsafe { (storage::Uniform1i.f)(location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform1iv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform1ui(location: GLint, v0: GLuint) { unsafe { (storage::Uniform1ui.f)(location, v0) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform1uiv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) { unsafe { (storage::Uniform2d.f)(location, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *GLdouble) { (storage::Uniform2dv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) { unsafe { (storage::Uniform2f.f)(location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform2fv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform2i(location: GLint, v0: GLint, v1: GLint) { unsafe { (storage::Uniform2i.f)(location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform2iv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) { unsafe { (storage::Uniform2ui.f)(location, v0, v1) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform2uiv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::Uniform3d.f)(location, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *GLdouble) { (storage::Uniform3dv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { unsafe { (storage::Uniform3f.f)(location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform3fv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) { unsafe { (storage::Uniform3i.f)(location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform3iv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { unsafe { (storage::Uniform3ui.f)(location, v0, v1, v2) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform3uiv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::Uniform4d.f)(location, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *GLdouble) { (storage::Uniform4dv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { unsafe { (storage::Uniform4f.f)(location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat) { (storage::Uniform4fv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { unsafe { (storage::Uniform4i.f)(location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint) { (storage::Uniform4iv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { unsafe { (storage::Uniform4ui.f)(location, v0, v1, v2, v3) } }
#[fixed_stack_segment] #[inline] pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *GLuint) { (storage::Uniform4uiv.f)(location, count, value) }
#[fixed_stack_segment] #[inline] pub fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) { unsafe { (storage::UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) } }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix2dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix2x3dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2x3fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix2x4dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix2x4fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix3dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix3x2dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3x2fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix3x4dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix3x4fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix4dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix4x2dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4x2fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble) { (storage::UniformMatrix4x3dv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat) { (storage::UniformMatrix4x3fv.f)(location, count, transpose, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *GLuint) { (storage::UniformSubroutinesuiv.f)(shadertype, count, indices) }
#[fixed_stack_segment] #[inline] pub fn UnmapBuffer(target: GLenum) -> GLboolean { unsafe { (storage::UnmapBuffer.f)(target) } }
#[fixed_stack_segment] #[inline] pub fn UseProgram(program: GLuint) { unsafe { (storage::UseProgram.f)(program) } }
#[fixed_stack_segment] #[inline] pub fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) { unsafe { (storage::UseProgramStages.f)(pipeline, stages, program) } }
#[fixed_stack_segment] #[inline] pub fn ValidateProgram(program: GLuint) { unsafe { (storage::ValidateProgram.f)(program) } }
#[fixed_stack_segment] #[inline] pub fn ValidateProgramPipeline(pipeline: GLuint) { unsafe { (storage::ValidateProgramPipeline.f)(pipeline) } }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib1d(index: GLuint, x: GLdouble) { unsafe { (storage::VertexAttrib1d.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib1dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib1dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib1f(index: GLuint, x: GLfloat) { unsafe { (storage::VertexAttrib1f.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib1fv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib1s(index: GLuint, x: GLshort) { unsafe { (storage::VertexAttrib1s.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib1sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib1sv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) { unsafe { (storage::VertexAttrib2d.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib2dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib2dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) { unsafe { (storage::VertexAttrib2f.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib2fv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) { unsafe { (storage::VertexAttrib2s.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib2sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib2sv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::VertexAttrib3d.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib3dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib3dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) { unsafe { (storage::VertexAttrib3f.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib3fv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) { unsafe { (storage::VertexAttrib3s.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib3sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib3sv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *GLbyte) { (storage::VertexAttrib4Nbv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *GLint) { (storage::VertexAttrib4Niv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *GLshort) { (storage::VertexAttrib4Nsv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) { unsafe { (storage::VertexAttrib4Nub.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *GLubyte) { (storage::VertexAttrib4Nubv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *GLuint) { (storage::VertexAttrib4Nuiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *GLushort) { (storage::VertexAttrib4Nusv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4bv(index: GLuint, v: *GLbyte) { (storage::VertexAttrib4bv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::VertexAttrib4d.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4dv(index: GLuint, v: *GLdouble) { (storage::VertexAttrib4dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { unsafe { (storage::VertexAttrib4f.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *GLfloat) { (storage::VertexAttrib4fv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4iv(index: GLuint, v: *GLint) { (storage::VertexAttrib4iv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { unsafe { (storage::VertexAttrib4s.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4sv(index: GLuint, v: *GLshort) { (storage::VertexAttrib4sv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *GLubyte) { (storage::VertexAttrib4ubv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *GLuint) { (storage::VertexAttrib4uiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttrib4usv(index: GLuint, v: *GLushort) { (storage::VertexAttrib4usv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) { unsafe { (storage::VertexAttribBinding.f)(attribindex, bindingindex) } }
#[fixed_stack_segment] #[inline] pub fn VertexAttribDivisor(index: GLuint, divisor: GLuint) { unsafe { (storage::VertexAttribDivisor.f)(index, divisor) } }
#[fixed_stack_segment] #[inline] pub fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) { unsafe { (storage::VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) } }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI1i(index: GLuint, x: GLint) { unsafe { (storage::VertexAttribI1i.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI1iv(index: GLuint, v: *GLint) { (storage::VertexAttribI1iv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI1ui(index: GLuint, x: GLuint) { unsafe { (storage::VertexAttribI1ui.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI1uiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) { unsafe { (storage::VertexAttribI2i.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI2iv(index: GLuint, v: *GLint) { (storage::VertexAttribI2iv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) { unsafe { (storage::VertexAttribI2ui.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI2uiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) { unsafe { (storage::VertexAttribI3i.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI3iv(index: GLuint, v: *GLint) { (storage::VertexAttribI3iv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) { unsafe { (storage::VertexAttribI3ui.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI3uiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4bv(index: GLuint, v: *GLbyte) { (storage::VertexAttribI4bv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) { unsafe { (storage::VertexAttribI4i.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4iv(index: GLuint, v: *GLint) { (storage::VertexAttribI4iv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4sv(index: GLuint, v: *GLshort) { (storage::VertexAttribI4sv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *GLubyte) { (storage::VertexAttribI4ubv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) { unsafe { (storage::VertexAttribI4ui.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *GLuint) { (storage::VertexAttribI4uiv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribI4usv(index: GLuint, v: *GLushort) { (storage::VertexAttribI4usv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { unsafe { (storage::VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) { (storage::VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribL1d(index: GLuint, x: GLdouble) { unsafe { (storage::VertexAttribL1d.f)(index, x) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribL1dv(index: GLuint, v: *GLdouble) { (storage::VertexAttribL1dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) { unsafe { (storage::VertexAttribL2d.f)(index, x, y) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribL2dv(index: GLuint, v: *GLdouble) { (storage::VertexAttribL2dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { unsafe { (storage::VertexAttribL3d.f)(index, x, y, z) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribL3dv(index: GLuint, v: *GLdouble) { (storage::VertexAttribL3dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { unsafe { (storage::VertexAttribL4d.f)(index, x, y, z, w) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribL4dv(index: GLuint, v: *GLdouble) { (storage::VertexAttribL4dv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { unsafe { (storage::VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid) { (storage::VertexAttribLPointer.f)(index, size, type_, stride, pointer) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { unsafe { (storage::VertexAttribP1ui.f)(index, type_, normalized, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (storage::VertexAttribP1uiv.f)(index, type_, normalized, value) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { unsafe { (storage::VertexAttribP2ui.f)(index, type_, normalized, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (storage::VertexAttribP2uiv.f)(index, type_, normalized, value) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { unsafe { (storage::VertexAttribP3ui.f)(index, type_, normalized, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (storage::VertexAttribP3uiv.f)(index, type_, normalized, value) }
#[fixed_stack_segment] #[inline] pub fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { unsafe { (storage::VertexAttribP4ui.f)(index, type_, normalized, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint) { (storage::VertexAttribP4uiv.f)(index, type_, normalized, value) }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *GLvoid) { (storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[fixed_stack_segment] #[inline] pub fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) { unsafe { (storage::VertexBindingDivisor.f)(bindingindex, divisor) } }
#[fixed_stack_segment] #[inline] pub fn VertexP2ui(type_: GLenum, value: GLuint) { unsafe { (storage::VertexP2ui.f)(type_, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexP2uiv(type_: GLenum, value: *GLuint) { (storage::VertexP2uiv.f)(type_, value) }
#[fixed_stack_segment] #[inline] pub fn VertexP3ui(type_: GLenum, value: GLuint) { unsafe { (storage::VertexP3ui.f)(type_, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexP3uiv(type_: GLenum, value: *GLuint) { (storage::VertexP3uiv.f)(type_, value) }
#[fixed_stack_segment] #[inline] pub fn VertexP4ui(type_: GLenum, value: GLuint) { unsafe { (storage::VertexP4ui.f)(type_, value) } }
#[fixed_stack_segment] #[inline] pub unsafe fn VertexP4uiv(type_: GLenum, value: *GLuint) { (storage::VertexP4uiv.f)(type_, value) }
#[fixed_stack_segment] #[inline] pub fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { unsafe { (storage::Viewport.f)(x, y, width, height) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *GLfloat) { (storage::ViewportArrayv.f)(first, count, v) }
#[fixed_stack_segment] #[inline] pub fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) { unsafe { (storage::ViewportIndexedf.f)(index, x, y, w, h) } }
#[fixed_stack_segment] #[inline] pub unsafe fn ViewportIndexedfv(index: GLuint, v: *GLfloat) { (storage::ViewportIndexedfv.f)(index, v) }
#[fixed_stack_segment] #[inline] pub fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) { unsafe { (storage::WaitSync.f)(sync, flags, timeout) } }

pub struct FnPtr<F> { f: F, is_loaded: bool }

impl<F> FnPtr<F> {
    pub fn new(ptr: Option<extern "C" fn()>, failing_fn: F) -> FnPtr<F> {
        use std::cast::transmute;
        match ptr {
            Some(p) => FnPtr { f: unsafe { transmute(p) }, is_loaded: true },
            None => FnPtr { f: failing_fn, is_loaded: false },
        }
    }
}

mod storage {
    use std::libc::*;
    use super::types::*;
    
    macro_rules! fn_ptr(
        (fn $name:ident()) => (
            pub static mut $name: ::FnPtr<extern "C" fn()> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident() -> $ret_ty:ty) => (
            pub static mut $name: ::FnPtr<extern "C" fn() -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident($($arg:ident : $arg_ty:ty),*)) => (
            pub static mut $name: ::FnPtr<extern "C" fn($($arg: $arg_ty),*)> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
        (fn $name:ident($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty) => (
            pub static mut $name: ::FnPtr<extern "C" fn($($arg: $arg_ty),*) -> $ret_ty> = ::FnPtr { f: ::failing::$name, is_loaded: false };
        );
    )
    
    fn_ptr!(fn ActiveShaderProgram(pipeline: GLuint, program: GLuint))
    fn_ptr!(fn ActiveTexture(texture: GLenum))
    fn_ptr!(fn AttachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn BeginConditionalRender(id: GLuint, mode: GLenum))
    fn_ptr!(fn BeginQuery(target: GLenum, id: GLuint))
    fn_ptr!(fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint))
    fn_ptr!(fn BeginTransformFeedback(primitiveMode: GLenum))
    fn_ptr!(fn BindAttribLocation(program: GLuint, index: GLuint, name: *GLchar))
    fn_ptr!(fn BindBuffer(target: GLenum, buffer: GLuint))
    fn_ptr!(fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint))
    fn_ptr!(fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr))
    fn_ptr!(fn BindFragDataLocation(program: GLuint, color: GLuint, name: *GLchar))
    fn_ptr!(fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *GLchar))
    fn_ptr!(fn BindFramebuffer(target: GLenum, framebuffer: GLuint))
    fn_ptr!(fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum))
    fn_ptr!(fn BindProgramPipeline(pipeline: GLuint))
    fn_ptr!(fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint))
    fn_ptr!(fn BindSampler(unit: GLuint, sampler: GLuint))
    fn_ptr!(fn BindTexture(target: GLenum, texture: GLuint))
    fn_ptr!(fn BindTransformFeedback(target: GLenum, id: GLuint))
    fn_ptr!(fn BindVertexArray(array: GLuint))
    fn_ptr!(fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei))
    fn_ptr!(fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn BlendEquation(mode: GLenum))
    fn_ptr!(fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum))
    fn_ptr!(fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum))
    fn_ptr!(fn BlendEquationi(buf: GLuint, mode: GLenum))
    fn_ptr!(fn BlendFunc(sfactor: GLenum, dfactor: GLenum))
    fn_ptr!(fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum))
    fn_ptr!(fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum))
    fn_ptr!(fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum))
    fn_ptr!(fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum))
    fn_ptr!(fn BufferData(target: GLenum, size: GLsizeiptr, data: *GLvoid, usage: GLenum))
    fn_ptr!(fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *GLvoid))
    fn_ptr!(fn CheckFramebufferStatus(target: GLenum) -> GLenum)
    fn_ptr!(fn ClampColor(target: GLenum, clamp: GLenum))
    fn_ptr!(fn Clear(mask: GLbitfield))
    fn_ptr!(fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *c_void))
    fn_ptr!(fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *c_void))
    fn_ptr!(fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint))
    fn_ptr!(fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *GLfloat))
    fn_ptr!(fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *GLint))
    fn_ptr!(fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *GLuint))
    fn_ptr!(fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat))
    fn_ptr!(fn ClearDepth(depth: GLdouble))
    fn_ptr!(fn ClearDepthf(d: GLfloat))
    fn_ptr!(fn ClearStencil(s: GLint))
    fn_ptr!(fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum)
    fn_ptr!(fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean))
    fn_ptr!(fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean))
    fn_ptr!(fn ColorP3ui(type_: GLenum, color: GLuint))
    fn_ptr!(fn ColorP3uiv(type_: GLenum, color: *GLuint))
    fn_ptr!(fn ColorP4ui(type_: GLenum, color: GLuint))
    fn_ptr!(fn ColorP4uiv(type_: GLenum, color: *GLuint))
    fn_ptr!(fn CompileShader(shader: GLuint))
    fn_ptr!(fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *GLvoid))
    fn_ptr!(fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr))
    fn_ptr!(fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei))
    fn_ptr!(fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint))
    fn_ptr!(fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei))
    fn_ptr!(fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn CreateProgram() -> GLuint)
    fn_ptr!(fn CreateShader(type_: GLenum) -> GLuint)
    fn_ptr!(fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: **GLchar) -> GLuint)
    fn_ptr!(fn CullFace(mode: GLenum))
    fn_ptr!(fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *c_void))
    fn_ptr!(fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *GLuint, enabled: GLboolean))
    fn_ptr!(fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *GLchar))
    fn_ptr!(fn DeleteBuffers(n: GLsizei, buffers: *GLuint))
    fn_ptr!(fn DeleteFramebuffers(n: GLsizei, framebuffers: *GLuint))
    fn_ptr!(fn DeleteProgram(program: GLuint))
    fn_ptr!(fn DeleteProgramPipelines(n: GLsizei, pipelines: *GLuint))
    fn_ptr!(fn DeleteQueries(n: GLsizei, ids: *GLuint))
    fn_ptr!(fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *GLuint))
    fn_ptr!(fn DeleteSamplers(count: GLsizei, samplers: *GLuint))
    fn_ptr!(fn DeleteShader(shader: GLuint))
    fn_ptr!(fn DeleteSync(sync: GLsync))
    fn_ptr!(fn DeleteTextures(n: GLsizei, textures: *GLuint))
    fn_ptr!(fn DeleteTransformFeedbacks(n: GLsizei, ids: *GLuint))
    fn_ptr!(fn DeleteVertexArrays(n: GLsizei, arrays: *GLuint))
    fn_ptr!(fn DepthFunc(func: GLenum))
    fn_ptr!(fn DepthMask(flag: GLboolean))
    fn_ptr!(fn DepthRange(near: GLdouble, far: GLdouble))
    fn_ptr!(fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *GLdouble))
    fn_ptr!(fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble))
    fn_ptr!(fn DepthRangef(n: GLfloat, f: GLfloat))
    fn_ptr!(fn DetachShader(program: GLuint, shader: GLuint))
    fn_ptr!(fn Disable(cap: GLenum))
    fn_ptr!(fn DisableVertexAttribArray(index: GLuint))
    fn_ptr!(fn Disablei(target: GLenum, index: GLuint))
    fn_ptr!(fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint))
    fn_ptr!(fn DispatchComputeIndirect(indirect: GLintptr))
    fn_ptr!(fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei))
    fn_ptr!(fn DrawArraysIndirect(mode: GLenum, indirect: *GLvoid))
    fn_ptr!(fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei))
    fn_ptr!(fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint))
    fn_ptr!(fn DrawBuffer(mode: GLenum))
    fn_ptr!(fn DrawBuffers(n: GLsizei, bufs: *GLenum))
    fn_ptr!(fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid))
    fn_ptr!(fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint))
    fn_ptr!(fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *GLvoid))
    fn_ptr!(fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei))
    fn_ptr!(fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, baseinstance: GLuint))
    fn_ptr!(fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *GLvoid, instancecount: GLsizei, basevertex: GLint))
    fn_ptr!(fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint))
    fn_ptr!(fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid))
    fn_ptr!(fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *GLvoid, basevertex: GLint))
    fn_ptr!(fn DrawTransformFeedback(mode: GLenum, id: GLuint))
    fn_ptr!(fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei))
    fn_ptr!(fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint))
    fn_ptr!(fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei))
    fn_ptr!(fn Enable(cap: GLenum))
    fn_ptr!(fn EnableVertexAttribArray(index: GLuint))
    fn_ptr!(fn Enablei(target: GLenum, index: GLuint))
    fn_ptr!(fn EndConditionalRender())
    fn_ptr!(fn EndQuery(target: GLenum))
    fn_ptr!(fn EndQueryIndexed(target: GLenum, index: GLuint))
    fn_ptr!(fn EndTransformFeedback())
    fn_ptr!(fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync)
    fn_ptr!(fn Finish())
    fn_ptr!(fn Flush())
    fn_ptr!(fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr))
    fn_ptr!(fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint))
    fn_ptr!(fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint))
    fn_ptr!(fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint))
    fn_ptr!(fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint))
    fn_ptr!(fn FrontFace(mode: GLenum))
    fn_ptr!(fn GenBuffers(n: GLsizei, buffers: *mut GLuint))
    fn_ptr!(fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint))
    fn_ptr!(fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint))
    fn_ptr!(fn GenQueries(n: GLsizei, ids: *mut GLuint))
    fn_ptr!(fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint))
    fn_ptr!(fn GenSamplers(count: GLsizei, samplers: *mut GLuint))
    fn_ptr!(fn GenTextures(n: GLsizei, textures: *mut GLuint))
    fn_ptr!(fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint))
    fn_ptr!(fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint))
    fn_ptr!(fn GenerateMipmap(target: GLenum))
    fn_ptr!(fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar))
    fn_ptr!(fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar))
    fn_ptr!(fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint))
    fn_ptr!(fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar))
    fn_ptr!(fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar))
    fn_ptr!(fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint))
    fn_ptr!(fn GetAttribLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean))
    fn_ptr!(fn GetBooleanv(pname: GLenum, params: *mut GLboolean))
    fn_ptr!(fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64))
    fn_ptr!(fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetBufferPointerv(target: GLenum, pname: GLenum, params: **mut GLvoid))
    fn_ptr!(fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid))
    fn_ptr!(fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut GLvoid))
    fn_ptr!(fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint)
    fn_ptr!(fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble))
    fn_ptr!(fn GetDoublev(pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetError() -> GLenum)
    fn_ptr!(fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat))
    fn_ptr!(fn GetFloatv(pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetFragDataIndex(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetFragDataLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64))
    fn_ptr!(fn GetInteger64v(pname: GLenum, params: *mut GLint64))
    fn_ptr!(fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint))
    fn_ptr!(fn GetIntegerv(pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64))
    fn_ptr!(fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint))
    fn_ptr!(fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat))
    fn_ptr!(fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar))
    fn_ptr!(fn GetObjectPtrLabel(ptr: *c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar))
    fn_ptr!(fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut GLvoid))
    fn_ptr!(fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLuint)
    fn_ptr!(fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint)
    fn_ptr!(fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *GLchar) -> GLint)
    fn_ptr!(fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar))
    fn_ptr!(fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint))
    fn_ptr!(fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint))
    fn_ptr!(fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64))
    fn_ptr!(fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64))
    fn_ptr!(fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar))
    fn_ptr!(fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint))
    fn_ptr!(fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar))
    fn_ptr!(fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetString(name: GLenum) -> *GLubyte)
    fn_ptr!(fn GetStringi(name: GLenum, index: GLuint) -> *GLubyte)
    fn_ptr!(fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLuint)
    fn_ptr!(fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *GLchar) -> GLint)
    fn_ptr!(fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint))
    fn_ptr!(fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut GLvoid))
    fn_ptr!(fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar))
    fn_ptr!(fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *GLchar) -> GLuint)
    fn_ptr!(fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: **GLchar, uniformIndices: *mut GLuint))
    fn_ptr!(fn GetUniformLocation(program: GLuint, name: *GLchar) -> GLint)
    fn_ptr!(fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint))
    fn_ptr!(fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble))
    fn_ptr!(fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat))
    fn_ptr!(fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint))
    fn_ptr!(fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint))
    fn_ptr!(fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint))
    fn_ptr!(fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: **mut GLvoid))
    fn_ptr!(fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble))
    fn_ptr!(fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat))
    fn_ptr!(fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint))
    fn_ptr!(fn Hint(target: GLenum, mode: GLenum))
    fn_ptr!(fn InvalidateBufferData(buffer: GLuint))
    fn_ptr!(fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr))
    fn_ptr!(fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *GLenum))
    fn_ptr!(fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn InvalidateTexImage(texture: GLuint, level: GLint))
    fn_ptr!(fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei))
    fn_ptr!(fn IsBuffer(buffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsEnabled(cap: GLenum) -> GLboolean)
    fn_ptr!(fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean)
    fn_ptr!(fn IsFramebuffer(framebuffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsProgram(program: GLuint) -> GLboolean)
    fn_ptr!(fn IsProgramPipeline(pipeline: GLuint) -> GLboolean)
    fn_ptr!(fn IsQuery(id: GLuint) -> GLboolean)
    fn_ptr!(fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean)
    fn_ptr!(fn IsSampler(sampler: GLuint) -> GLboolean)
    fn_ptr!(fn IsShader(shader: GLuint) -> GLboolean)
    fn_ptr!(fn IsSync(sync: GLsync) -> GLboolean)
    fn_ptr!(fn IsTexture(texture: GLuint) -> GLboolean)
    fn_ptr!(fn IsTransformFeedback(id: GLuint) -> GLboolean)
    fn_ptr!(fn IsVertexArray(array: GLuint) -> GLboolean)
    fn_ptr!(fn LineWidth(width: GLfloat))
    fn_ptr!(fn LinkProgram(program: GLuint))
    fn_ptr!(fn LogicOp(opcode: GLenum))
    fn_ptr!(fn MapBuffer(target: GLenum, access: GLenum) -> *c_void)
    fn_ptr!(fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *c_void)
    fn_ptr!(fn MemoryBarrier(barriers: GLbitfield))
    fn_ptr!(fn MinSampleShading(value: GLfloat))
    fn_ptr!(fn MultiDrawArrays(mode: GLenum, first: *GLint, count: *GLsizei, drawcount: GLsizei))
    fn_ptr!(fn MultiDrawArraysIndirect(mode: GLenum, indirect: *c_void, drawcount: GLsizei, stride: GLsizei))
    fn_ptr!(fn MultiDrawElements(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei))
    fn_ptr!(fn MultiDrawElementsBaseVertex(mode: GLenum, count: *GLsizei, type_: GLenum, indices: **GLvoid, drawcount: GLsizei, basevertex: *GLint))
    fn_ptr!(fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *c_void, drawcount: GLsizei, stride: GLsizei))
    fn_ptr!(fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint))
    fn_ptr!(fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *GLuint))
    fn_ptr!(fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint))
    fn_ptr!(fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *GLuint))
    fn_ptr!(fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint))
    fn_ptr!(fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *GLuint))
    fn_ptr!(fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint))
    fn_ptr!(fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *GLuint))
    fn_ptr!(fn NormalP3ui(type_: GLenum, coords: GLuint))
    fn_ptr!(fn NormalP3uiv(type_: GLenum, coords: *GLuint))
    fn_ptr!(fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *GLchar))
    fn_ptr!(fn ObjectPtrLabel(ptr: *c_void, length: GLsizei, label: *GLchar))
    fn_ptr!(fn PatchParameterfv(pname: GLenum, values: *GLfloat))
    fn_ptr!(fn PatchParameteri(pname: GLenum, value: GLint))
    fn_ptr!(fn PauseTransformFeedback())
    fn_ptr!(fn PixelStoref(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PixelStorei(pname: GLenum, param: GLint))
    fn_ptr!(fn PointParameterf(pname: GLenum, param: GLfloat))
    fn_ptr!(fn PointParameterfv(pname: GLenum, params: *GLfloat))
    fn_ptr!(fn PointParameteri(pname: GLenum, param: GLint))
    fn_ptr!(fn PointParameteriv(pname: GLenum, params: *GLint))
    fn_ptr!(fn PointSize(size: GLfloat))
    fn_ptr!(fn PolygonMode(face: GLenum, mode: GLenum))
    fn_ptr!(fn PolygonOffset(factor: GLfloat, units: GLfloat))
    fn_ptr!(fn PopDebugGroup())
    fn_ptr!(fn PrimitiveRestartIndex(index: GLuint))
    fn_ptr!(fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *GLvoid, length: GLsizei))
    fn_ptr!(fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint))
    fn_ptr!(fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble))
    fn_ptr!(fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat))
    fn_ptr!(fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint))
    fn_ptr!(fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint))
    fn_ptr!(fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble))
    fn_ptr!(fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat))
    fn_ptr!(fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint))
    fn_ptr!(fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint))
    fn_ptr!(fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble))
    fn_ptr!(fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat))
    fn_ptr!(fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint))
    fn_ptr!(fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint))
    fn_ptr!(fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble))
    fn_ptr!(fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat))
    fn_ptr!(fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint))
    fn_ptr!(fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint))
    fn_ptr!(fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn ProvokingVertex(mode: GLenum))
    fn_ptr!(fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *GLchar))
    fn_ptr!(fn QueryCounter(id: GLuint, target: GLenum))
    fn_ptr!(fn ReadBuffer(mode: GLenum))
    fn_ptr!(fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut GLvoid))
    fn_ptr!(fn ReleaseShaderCompiler())
    fn_ptr!(fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei))
    fn_ptr!(fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei))
    fn_ptr!(fn ResumeTransformFeedback())
    fn_ptr!(fn SampleCoverage(value: GLfloat, invert: GLboolean))
    fn_ptr!(fn SampleMaski(index: GLuint, mask: GLbitfield))
    fn_ptr!(fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *GLint))
    fn_ptr!(fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *GLuint))
    fn_ptr!(fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat))
    fn_ptr!(fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *GLfloat))
    fn_ptr!(fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint))
    fn_ptr!(fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *GLint))
    fn_ptr!(fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn ScissorArrayv(first: GLuint, count: GLsizei, v: *GLint))
    fn_ptr!(fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn ScissorIndexedv(index: GLuint, v: *GLint))
    fn_ptr!(fn SecondaryColorP3ui(type_: GLenum, color: GLuint))
    fn_ptr!(fn SecondaryColorP3uiv(type_: GLenum, color: *GLuint))
    fn_ptr!(fn ShaderBinary(count: GLsizei, shaders: *GLuint, binaryformat: GLenum, binary: *GLvoid, length: GLsizei))
    fn_ptr!(fn ShaderSource(shader: GLuint, count: GLsizei, string: **GLchar, length: *GLint))
    fn_ptr!(fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint))
    fn_ptr!(fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint))
    fn_ptr!(fn StencilMask(mask: GLuint))
    fn_ptr!(fn StencilMaskSeparate(face: GLenum, mask: GLuint))
    fn_ptr!(fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum))
    fn_ptr!(fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum))
    fn_ptr!(fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint))
    fn_ptr!(fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr))
    fn_ptr!(fn TexCoordP1ui(type_: GLenum, coords: GLuint))
    fn_ptr!(fn TexCoordP1uiv(type_: GLenum, coords: *GLuint))
    fn_ptr!(fn TexCoordP2ui(type_: GLenum, coords: GLuint))
    fn_ptr!(fn TexCoordP2uiv(type_: GLenum, coords: *GLuint))
    fn_ptr!(fn TexCoordP3ui(type_: GLenum, coords: GLuint))
    fn_ptr!(fn TexCoordP3uiv(type_: GLenum, coords: *GLuint))
    fn_ptr!(fn TexCoordP4ui(type_: GLenum, coords: GLuint))
    fn_ptr!(fn TexCoordP4uiv(type_: GLenum, coords: *GLuint))
    fn_ptr!(fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexParameterIiv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *GLuint))
    fn_ptr!(fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat))
    fn_ptr!(fn TexParameterfv(target: GLenum, pname: GLenum, params: *GLfloat))
    fn_ptr!(fn TexParameteri(target: GLenum, pname: GLenum, param: GLint))
    fn_ptr!(fn TexParameteriv(target: GLenum, pname: GLenum, params: *GLint))
    fn_ptr!(fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei))
    fn_ptr!(fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei))
    fn_ptr!(fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei))
    fn_ptr!(fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean))
    fn_ptr!(fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *GLvoid))
    fn_ptr!(fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint))
    fn_ptr!(fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: **GLchar, bufferMode: GLenum))
    fn_ptr!(fn Uniform1d(location: GLint, x: GLdouble))
    fn_ptr!(fn Uniform1dv(location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn Uniform1f(location: GLint, v0: GLfloat))
    fn_ptr!(fn Uniform1fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform1i(location: GLint, v0: GLint))
    fn_ptr!(fn Uniform1iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform1ui(location: GLint, v0: GLuint))
    fn_ptr!(fn Uniform1uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble))
    fn_ptr!(fn Uniform2dv(location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat))
    fn_ptr!(fn Uniform2fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform2i(location: GLint, v0: GLint, v1: GLint))
    fn_ptr!(fn Uniform2iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint))
    fn_ptr!(fn Uniform2uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn Uniform3dv(location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat))
    fn_ptr!(fn Uniform3fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint))
    fn_ptr!(fn Uniform3iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint))
    fn_ptr!(fn Uniform3uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn Uniform4dv(location: GLint, count: GLsizei, value: *GLdouble))
    fn_ptr!(fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat))
    fn_ptr!(fn Uniform4fv(location: GLint, count: GLsizei, value: *GLfloat))
    fn_ptr!(fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint))
    fn_ptr!(fn Uniform4iv(location: GLint, count: GLsizei, value: *GLint))
    fn_ptr!(fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint))
    fn_ptr!(fn Uniform4uiv(location: GLint, count: GLsizei, value: *GLuint))
    fn_ptr!(fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint))
    fn_ptr!(fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLdouble))
    fn_ptr!(fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *GLfloat))
    fn_ptr!(fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *GLuint))
    fn_ptr!(fn UnmapBuffer(target: GLenum) -> GLboolean)
    fn_ptr!(fn UseProgram(program: GLuint))
    fn_ptr!(fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint))
    fn_ptr!(fn ValidateProgram(program: GLuint))
    fn_ptr!(fn ValidateProgramPipeline(pipeline: GLuint))
    fn_ptr!(fn VertexAttrib1d(index: GLuint, x: GLdouble))
    fn_ptr!(fn VertexAttrib1dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib1f(index: GLuint, x: GLfloat))
    fn_ptr!(fn VertexAttrib1fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib1s(index: GLuint, x: GLshort))
    fn_ptr!(fn VertexAttrib1sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble))
    fn_ptr!(fn VertexAttrib2dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat))
    fn_ptr!(fn VertexAttrib2fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort))
    fn_ptr!(fn VertexAttrib2sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn VertexAttrib3dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat))
    fn_ptr!(fn VertexAttrib3fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort))
    fn_ptr!(fn VertexAttrib3sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4Nbv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttrib4Niv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttrib4Nsv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte))
    fn_ptr!(fn VertexAttrib4Nubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttrib4Nuiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttrib4Nusv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttrib4bv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn VertexAttrib4dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat))
    fn_ptr!(fn VertexAttrib4fv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn VertexAttrib4iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort))
    fn_ptr!(fn VertexAttrib4sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttrib4ubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttrib4uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttrib4usv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint))
    fn_ptr!(fn VertexAttribDivisor(index: GLuint, divisor: GLuint))
    fn_ptr!(fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint))
    fn_ptr!(fn VertexAttribI1i(index: GLuint, x: GLint))
    fn_ptr!(fn VertexAttribI1iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI1ui(index: GLuint, x: GLuint))
    fn_ptr!(fn VertexAttribI1uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint))
    fn_ptr!(fn VertexAttribI2iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint))
    fn_ptr!(fn VertexAttribI2uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint))
    fn_ptr!(fn VertexAttribI3iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint))
    fn_ptr!(fn VertexAttribI3uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI4bv(index: GLuint, v: *GLbyte))
    fn_ptr!(fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint))
    fn_ptr!(fn VertexAttribI4iv(index: GLuint, v: *GLint))
    fn_ptr!(fn VertexAttribI4sv(index: GLuint, v: *GLshort))
    fn_ptr!(fn VertexAttribI4ubv(index: GLuint, v: *GLubyte))
    fn_ptr!(fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint))
    fn_ptr!(fn VertexAttribI4uiv(index: GLuint, v: *GLuint))
    fn_ptr!(fn VertexAttribI4usv(index: GLuint, v: *GLushort))
    fn_ptr!(fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint))
    fn_ptr!(fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid))
    fn_ptr!(fn VertexAttribL1d(index: GLuint, x: GLdouble))
    fn_ptr!(fn VertexAttribL1dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble))
    fn_ptr!(fn VertexAttribL2dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble))
    fn_ptr!(fn VertexAttribL3dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble))
    fn_ptr!(fn VertexAttribL4dv(index: GLuint, v: *GLdouble))
    fn_ptr!(fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint))
    fn_ptr!(fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *GLvoid))
    fn_ptr!(fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint))
    fn_ptr!(fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint))
    fn_ptr!(fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint))
    fn_ptr!(fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint))
    fn_ptr!(fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint))
    fn_ptr!(fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint))
    fn_ptr!(fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint))
    fn_ptr!(fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *GLuint))
    fn_ptr!(fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *GLvoid))
    fn_ptr!(fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint))
    fn_ptr!(fn VertexP2ui(type_: GLenum, value: GLuint))
    fn_ptr!(fn VertexP2uiv(type_: GLenum, value: *GLuint))
    fn_ptr!(fn VertexP3ui(type_: GLenum, value: GLuint))
    fn_ptr!(fn VertexP3uiv(type_: GLenum, value: *GLuint))
    fn_ptr!(fn VertexP4ui(type_: GLenum, value: GLuint))
    fn_ptr!(fn VertexP4uiv(type_: GLenum, value: *GLuint))
    fn_ptr!(fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei))
    fn_ptr!(fn ViewportArrayv(first: GLuint, count: GLsizei, v: *GLfloat))
    fn_ptr!(fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat))
    fn_ptr!(fn ViewportIndexedfv(index: GLuint, v: *GLfloat))
    fn_ptr!(fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64))
}

macro_rules! fn_mod(
    ($name:ident, $sym:expr) => (
        pub mod $name {
            #[inline]
            pub fn is_loaded() -> bool { unsafe { ::storage::$name.is_loaded } }
            
            #[inline]
            pub fn load_with(loadfn: &fn(symbol: &str) -> Option<extern "C" fn()>) {
                unsafe { ::storage::$name = ::FnPtr::new(loadfn($sym), ::failing::$name) }
            }
        }
    )
)

fn_mod!(ActiveShaderProgram, "glActiveShaderProgram")
fn_mod!(ActiveTexture, "glActiveTexture")
fn_mod!(AttachShader, "glAttachShader")
fn_mod!(BeginConditionalRender, "glBeginConditionalRender")
fn_mod!(BeginQuery, "glBeginQuery")
fn_mod!(BeginQueryIndexed, "glBeginQueryIndexed")
fn_mod!(BeginTransformFeedback, "glBeginTransformFeedback")
fn_mod!(BindAttribLocation, "glBindAttribLocation")
fn_mod!(BindBuffer, "glBindBuffer")
fn_mod!(BindBufferBase, "glBindBufferBase")
fn_mod!(BindBufferRange, "glBindBufferRange")
fn_mod!(BindFragDataLocation, "glBindFragDataLocation")
fn_mod!(BindFragDataLocationIndexed, "glBindFragDataLocationIndexed")
fn_mod!(BindFramebuffer, "glBindFramebuffer")
fn_mod!(BindImageTexture, "glBindImageTexture")
fn_mod!(BindProgramPipeline, "glBindProgramPipeline")
fn_mod!(BindRenderbuffer, "glBindRenderbuffer")
fn_mod!(BindSampler, "glBindSampler")
fn_mod!(BindTexture, "glBindTexture")
fn_mod!(BindTransformFeedback, "glBindTransformFeedback")
fn_mod!(BindVertexArray, "glBindVertexArray")
fn_mod!(BindVertexBuffer, "glBindVertexBuffer")
fn_mod!(BlendColor, "glBlendColor")
fn_mod!(BlendEquation, "glBlendEquation")
fn_mod!(BlendEquationSeparate, "glBlendEquationSeparate")
fn_mod!(BlendEquationSeparatei, "glBlendEquationSeparatei")
fn_mod!(BlendEquationi, "glBlendEquationi")
fn_mod!(BlendFunc, "glBlendFunc")
fn_mod!(BlendFuncSeparate, "glBlendFuncSeparate")
fn_mod!(BlendFuncSeparatei, "glBlendFuncSeparatei")
fn_mod!(BlendFunci, "glBlendFunci")
fn_mod!(BlitFramebuffer, "glBlitFramebuffer")
fn_mod!(BufferData, "glBufferData")
fn_mod!(BufferSubData, "glBufferSubData")
fn_mod!(CheckFramebufferStatus, "glCheckFramebufferStatus")
fn_mod!(ClampColor, "glClampColor")
fn_mod!(Clear, "glClear")
fn_mod!(ClearBufferData, "glClearBufferData")
fn_mod!(ClearBufferSubData, "glClearBufferSubData")
fn_mod!(ClearBufferfi, "glClearBufferfi")
fn_mod!(ClearBufferfv, "glClearBufferfv")
fn_mod!(ClearBufferiv, "glClearBufferiv")
fn_mod!(ClearBufferuiv, "glClearBufferuiv")
fn_mod!(ClearColor, "glClearColor")
fn_mod!(ClearDepth, "glClearDepth")
fn_mod!(ClearDepthf, "glClearDepthf")
fn_mod!(ClearStencil, "glClearStencil")
fn_mod!(ClientWaitSync, "glClientWaitSync")
fn_mod!(ColorMask, "glColorMask")
fn_mod!(ColorMaski, "glColorMaski")
fn_mod!(ColorP3ui, "glColorP3ui")
fn_mod!(ColorP3uiv, "glColorP3uiv")
fn_mod!(ColorP4ui, "glColorP4ui")
fn_mod!(ColorP4uiv, "glColorP4uiv")
fn_mod!(CompileShader, "glCompileShader")
fn_mod!(CompressedTexImage1D, "glCompressedTexImage1D")
fn_mod!(CompressedTexImage2D, "glCompressedTexImage2D")
fn_mod!(CompressedTexImage3D, "glCompressedTexImage3D")
fn_mod!(CompressedTexSubImage1D, "glCompressedTexSubImage1D")
fn_mod!(CompressedTexSubImage2D, "glCompressedTexSubImage2D")
fn_mod!(CompressedTexSubImage3D, "glCompressedTexSubImage3D")
fn_mod!(CopyBufferSubData, "glCopyBufferSubData")
fn_mod!(CopyImageSubData, "glCopyImageSubData")
fn_mod!(CopyTexImage1D, "glCopyTexImage1D")
fn_mod!(CopyTexImage2D, "glCopyTexImage2D")
fn_mod!(CopyTexSubImage1D, "glCopyTexSubImage1D")
fn_mod!(CopyTexSubImage2D, "glCopyTexSubImage2D")
fn_mod!(CopyTexSubImage3D, "glCopyTexSubImage3D")
fn_mod!(CreateProgram, "glCreateProgram")
fn_mod!(CreateShader, "glCreateShader")
fn_mod!(CreateShaderProgramv, "glCreateShaderProgramv")
fn_mod!(CullFace, "glCullFace")
fn_mod!(DebugMessageCallback, "glDebugMessageCallback")
fn_mod!(DebugMessageControl, "glDebugMessageControl")
fn_mod!(DebugMessageInsert, "glDebugMessageInsert")
fn_mod!(DeleteBuffers, "glDeleteBuffers")
fn_mod!(DeleteFramebuffers, "glDeleteFramebuffers")
fn_mod!(DeleteProgram, "glDeleteProgram")
fn_mod!(DeleteProgramPipelines, "glDeleteProgramPipelines")
fn_mod!(DeleteQueries, "glDeleteQueries")
fn_mod!(DeleteRenderbuffers, "glDeleteRenderbuffers")
fn_mod!(DeleteSamplers, "glDeleteSamplers")
fn_mod!(DeleteShader, "glDeleteShader")
fn_mod!(DeleteSync, "glDeleteSync")
fn_mod!(DeleteTextures, "glDeleteTextures")
fn_mod!(DeleteTransformFeedbacks, "glDeleteTransformFeedbacks")
fn_mod!(DeleteVertexArrays, "glDeleteVertexArrays")
fn_mod!(DepthFunc, "glDepthFunc")
fn_mod!(DepthMask, "glDepthMask")
fn_mod!(DepthRange, "glDepthRange")
fn_mod!(DepthRangeArrayv, "glDepthRangeArrayv")
fn_mod!(DepthRangeIndexed, "glDepthRangeIndexed")
fn_mod!(DepthRangef, "glDepthRangef")
fn_mod!(DetachShader, "glDetachShader")
fn_mod!(Disable, "glDisable")
fn_mod!(DisableVertexAttribArray, "glDisableVertexAttribArray")
fn_mod!(Disablei, "glDisablei")
fn_mod!(DispatchCompute, "glDispatchCompute")
fn_mod!(DispatchComputeIndirect, "glDispatchComputeIndirect")
fn_mod!(DrawArrays, "glDrawArrays")
fn_mod!(DrawArraysIndirect, "glDrawArraysIndirect")
fn_mod!(DrawArraysInstanced, "glDrawArraysInstanced")
fn_mod!(DrawArraysInstancedBaseInstance, "glDrawArraysInstancedBaseInstance")
fn_mod!(DrawBuffer, "glDrawBuffer")
fn_mod!(DrawBuffers, "glDrawBuffers")
fn_mod!(DrawElements, "glDrawElements")
fn_mod!(DrawElementsBaseVertex, "glDrawElementsBaseVertex")
fn_mod!(DrawElementsIndirect, "glDrawElementsIndirect")
fn_mod!(DrawElementsInstanced, "glDrawElementsInstanced")
fn_mod!(DrawElementsInstancedBaseInstance, "glDrawElementsInstancedBaseInstance")
fn_mod!(DrawElementsInstancedBaseVertex, "glDrawElementsInstancedBaseVertex")
fn_mod!(DrawElementsInstancedBaseVertexBaseInstance, "glDrawElementsInstancedBaseVertexBaseInstance")
fn_mod!(DrawRangeElements, "glDrawRangeElements")
fn_mod!(DrawRangeElementsBaseVertex, "glDrawRangeElementsBaseVertex")
fn_mod!(DrawTransformFeedback, "glDrawTransformFeedback")
fn_mod!(DrawTransformFeedbackInstanced, "glDrawTransformFeedbackInstanced")
fn_mod!(DrawTransformFeedbackStream, "glDrawTransformFeedbackStream")
fn_mod!(DrawTransformFeedbackStreamInstanced, "glDrawTransformFeedbackStreamInstanced")
fn_mod!(Enable, "glEnable")
fn_mod!(EnableVertexAttribArray, "glEnableVertexAttribArray")
fn_mod!(Enablei, "glEnablei")
fn_mod!(EndConditionalRender, "glEndConditionalRender")
fn_mod!(EndQuery, "glEndQuery")
fn_mod!(EndQueryIndexed, "glEndQueryIndexed")
fn_mod!(EndTransformFeedback, "glEndTransformFeedback")
fn_mod!(FenceSync, "glFenceSync")
fn_mod!(Finish, "glFinish")
fn_mod!(Flush, "glFlush")
fn_mod!(FlushMappedBufferRange, "glFlushMappedBufferRange")
fn_mod!(FramebufferParameteri, "glFramebufferParameteri")
fn_mod!(FramebufferRenderbuffer, "glFramebufferRenderbuffer")
fn_mod!(FramebufferTexture, "glFramebufferTexture")
fn_mod!(FramebufferTexture1D, "glFramebufferTexture1D")
fn_mod!(FramebufferTexture2D, "glFramebufferTexture2D")
fn_mod!(FramebufferTexture3D, "glFramebufferTexture3D")
fn_mod!(FramebufferTextureLayer, "glFramebufferTextureLayer")
fn_mod!(FrontFace, "glFrontFace")
fn_mod!(GenBuffers, "glGenBuffers")
fn_mod!(GenFramebuffers, "glGenFramebuffers")
fn_mod!(GenProgramPipelines, "glGenProgramPipelines")
fn_mod!(GenQueries, "glGenQueries")
fn_mod!(GenRenderbuffers, "glGenRenderbuffers")
fn_mod!(GenSamplers, "glGenSamplers")
fn_mod!(GenTextures, "glGenTextures")
fn_mod!(GenTransformFeedbacks, "glGenTransformFeedbacks")
fn_mod!(GenVertexArrays, "glGenVertexArrays")
fn_mod!(GenerateMipmap, "glGenerateMipmap")
fn_mod!(GetActiveAtomicCounterBufferiv, "glGetActiveAtomicCounterBufferiv")
fn_mod!(GetActiveAttrib, "glGetActiveAttrib")
fn_mod!(GetActiveSubroutineName, "glGetActiveSubroutineName")
fn_mod!(GetActiveSubroutineUniformName, "glGetActiveSubroutineUniformName")
fn_mod!(GetActiveSubroutineUniformiv, "glGetActiveSubroutineUniformiv")
fn_mod!(GetActiveUniform, "glGetActiveUniform")
fn_mod!(GetActiveUniformBlockName, "glGetActiveUniformBlockName")
fn_mod!(GetActiveUniformBlockiv, "glGetActiveUniformBlockiv")
fn_mod!(GetActiveUniformName, "glGetActiveUniformName")
fn_mod!(GetActiveUniformsiv, "glGetActiveUniformsiv")
fn_mod!(GetAttachedShaders, "glGetAttachedShaders")
fn_mod!(GetAttribLocation, "glGetAttribLocation")
fn_mod!(GetBooleani_v, "glGetBooleani_v")
fn_mod!(GetBooleanv, "glGetBooleanv")
fn_mod!(GetBufferParameteri64v, "glGetBufferParameteri64v")
fn_mod!(GetBufferParameteriv, "glGetBufferParameteriv")
fn_mod!(GetBufferPointerv, "glGetBufferPointerv")
fn_mod!(GetBufferSubData, "glGetBufferSubData")
fn_mod!(GetCompressedTexImage, "glGetCompressedTexImage")
fn_mod!(GetDebugMessageLog, "glGetDebugMessageLog")
fn_mod!(GetDoublei_v, "glGetDoublei_v")
fn_mod!(GetDoublev, "glGetDoublev")
fn_mod!(GetError, "glGetError")
fn_mod!(GetFloati_v, "glGetFloati_v")
fn_mod!(GetFloatv, "glGetFloatv")
fn_mod!(GetFragDataIndex, "glGetFragDataIndex")
fn_mod!(GetFragDataLocation, "glGetFragDataLocation")
fn_mod!(GetFramebufferAttachmentParameteriv, "glGetFramebufferAttachmentParameteriv")
fn_mod!(GetFramebufferParameteriv, "glGetFramebufferParameteriv")
fn_mod!(GetInteger64i_v, "glGetInteger64i_v")
fn_mod!(GetInteger64v, "glGetInteger64v")
fn_mod!(GetIntegeri_v, "glGetIntegeri_v")
fn_mod!(GetIntegerv, "glGetIntegerv")
fn_mod!(GetInternalformati64v, "glGetInternalformati64v")
fn_mod!(GetInternalformativ, "glGetInternalformativ")
fn_mod!(GetMultisamplefv, "glGetMultisamplefv")
fn_mod!(GetObjectLabel, "glGetObjectLabel")
fn_mod!(GetObjectPtrLabel, "glGetObjectPtrLabel")
fn_mod!(GetProgramBinary, "glGetProgramBinary")
fn_mod!(GetProgramInfoLog, "glGetProgramInfoLog")
fn_mod!(GetProgramInterfaceiv, "glGetProgramInterfaceiv")
fn_mod!(GetProgramPipelineInfoLog, "glGetProgramPipelineInfoLog")
fn_mod!(GetProgramPipelineiv, "glGetProgramPipelineiv")
fn_mod!(GetProgramResourceIndex, "glGetProgramResourceIndex")
fn_mod!(GetProgramResourceLocation, "glGetProgramResourceLocation")
fn_mod!(GetProgramResourceLocationIndex, "glGetProgramResourceLocationIndex")
fn_mod!(GetProgramResourceName, "glGetProgramResourceName")
fn_mod!(GetProgramResourceiv, "glGetProgramResourceiv")
fn_mod!(GetProgramStageiv, "glGetProgramStageiv")
fn_mod!(GetProgramiv, "glGetProgramiv")
fn_mod!(GetQueryIndexediv, "glGetQueryIndexediv")
fn_mod!(GetQueryObjecti64v, "glGetQueryObjecti64v")
fn_mod!(GetQueryObjectiv, "glGetQueryObjectiv")
fn_mod!(GetQueryObjectui64v, "glGetQueryObjectui64v")
fn_mod!(GetQueryObjectuiv, "glGetQueryObjectuiv")
fn_mod!(GetQueryiv, "glGetQueryiv")
fn_mod!(GetRenderbufferParameteriv, "glGetRenderbufferParameteriv")
fn_mod!(GetSamplerParameterIiv, "glGetSamplerParameterIiv")
fn_mod!(GetSamplerParameterIuiv, "glGetSamplerParameterIuiv")
fn_mod!(GetSamplerParameterfv, "glGetSamplerParameterfv")
fn_mod!(GetSamplerParameteriv, "glGetSamplerParameteriv")
fn_mod!(GetShaderInfoLog, "glGetShaderInfoLog")
fn_mod!(GetShaderPrecisionFormat, "glGetShaderPrecisionFormat")
fn_mod!(GetShaderSource, "glGetShaderSource")
fn_mod!(GetShaderiv, "glGetShaderiv")
fn_mod!(GetString, "glGetString")
fn_mod!(GetStringi, "glGetStringi")
fn_mod!(GetSubroutineIndex, "glGetSubroutineIndex")
fn_mod!(GetSubroutineUniformLocation, "glGetSubroutineUniformLocation")
fn_mod!(GetSynciv, "glGetSynciv")
fn_mod!(GetTexImage, "glGetTexImage")
fn_mod!(GetTexLevelParameterfv, "glGetTexLevelParameterfv")
fn_mod!(GetTexLevelParameteriv, "glGetTexLevelParameteriv")
fn_mod!(GetTexParameterIiv, "glGetTexParameterIiv")
fn_mod!(GetTexParameterIuiv, "glGetTexParameterIuiv")
fn_mod!(GetTexParameterfv, "glGetTexParameterfv")
fn_mod!(GetTexParameteriv, "glGetTexParameteriv")
fn_mod!(GetTransformFeedbackVarying, "glGetTransformFeedbackVarying")
fn_mod!(GetUniformBlockIndex, "glGetUniformBlockIndex")
fn_mod!(GetUniformIndices, "glGetUniformIndices")
fn_mod!(GetUniformLocation, "glGetUniformLocation")
fn_mod!(GetUniformSubroutineuiv, "glGetUniformSubroutineuiv")
fn_mod!(GetUniformdv, "glGetUniformdv")
fn_mod!(GetUniformfv, "glGetUniformfv")
fn_mod!(GetUniformiv, "glGetUniformiv")
fn_mod!(GetUniformuiv, "glGetUniformuiv")
fn_mod!(GetVertexAttribIiv, "glGetVertexAttribIiv")
fn_mod!(GetVertexAttribIuiv, "glGetVertexAttribIuiv")
fn_mod!(GetVertexAttribLdv, "glGetVertexAttribLdv")
fn_mod!(GetVertexAttribPointerv, "glGetVertexAttribPointerv")
fn_mod!(GetVertexAttribdv, "glGetVertexAttribdv")
fn_mod!(GetVertexAttribfv, "glGetVertexAttribfv")
fn_mod!(GetVertexAttribiv, "glGetVertexAttribiv")
fn_mod!(Hint, "glHint")
fn_mod!(InvalidateBufferData, "glInvalidateBufferData")
fn_mod!(InvalidateBufferSubData, "glInvalidateBufferSubData")
fn_mod!(InvalidateFramebuffer, "glInvalidateFramebuffer")
fn_mod!(InvalidateSubFramebuffer, "glInvalidateSubFramebuffer")
fn_mod!(InvalidateTexImage, "glInvalidateTexImage")
fn_mod!(InvalidateTexSubImage, "glInvalidateTexSubImage")
fn_mod!(IsBuffer, "glIsBuffer")
fn_mod!(IsEnabled, "glIsEnabled")
fn_mod!(IsEnabledi, "glIsEnabledi")
fn_mod!(IsFramebuffer, "glIsFramebuffer")
fn_mod!(IsProgram, "glIsProgram")
fn_mod!(IsProgramPipeline, "glIsProgramPipeline")
fn_mod!(IsQuery, "glIsQuery")
fn_mod!(IsRenderbuffer, "glIsRenderbuffer")
fn_mod!(IsSampler, "glIsSampler")
fn_mod!(IsShader, "glIsShader")
fn_mod!(IsSync, "glIsSync")
fn_mod!(IsTexture, "glIsTexture")
fn_mod!(IsTransformFeedback, "glIsTransformFeedback")
fn_mod!(IsVertexArray, "glIsVertexArray")
fn_mod!(LineWidth, "glLineWidth")
fn_mod!(LinkProgram, "glLinkProgram")
fn_mod!(LogicOp, "glLogicOp")
fn_mod!(MapBuffer, "glMapBuffer")
fn_mod!(MapBufferRange, "glMapBufferRange")
fn_mod!(MemoryBarrier, "glMemoryBarrier")
fn_mod!(MinSampleShading, "glMinSampleShading")
fn_mod!(MultiDrawArrays, "glMultiDrawArrays")
fn_mod!(MultiDrawArraysIndirect, "glMultiDrawArraysIndirect")
fn_mod!(MultiDrawElements, "glMultiDrawElements")
fn_mod!(MultiDrawElementsBaseVertex, "glMultiDrawElementsBaseVertex")
fn_mod!(MultiDrawElementsIndirect, "glMultiDrawElementsIndirect")
fn_mod!(MultiTexCoordP1ui, "glMultiTexCoordP1ui")
fn_mod!(MultiTexCoordP1uiv, "glMultiTexCoordP1uiv")
fn_mod!(MultiTexCoordP2ui, "glMultiTexCoordP2ui")
fn_mod!(MultiTexCoordP2uiv, "glMultiTexCoordP2uiv")
fn_mod!(MultiTexCoordP3ui, "glMultiTexCoordP3ui")
fn_mod!(MultiTexCoordP3uiv, "glMultiTexCoordP3uiv")
fn_mod!(MultiTexCoordP4ui, "glMultiTexCoordP4ui")
fn_mod!(MultiTexCoordP4uiv, "glMultiTexCoordP4uiv")
fn_mod!(NormalP3ui, "glNormalP3ui")
fn_mod!(NormalP3uiv, "glNormalP3uiv")
fn_mod!(ObjectLabel, "glObjectLabel")
fn_mod!(ObjectPtrLabel, "glObjectPtrLabel")
fn_mod!(PatchParameterfv, "glPatchParameterfv")
fn_mod!(PatchParameteri, "glPatchParameteri")
fn_mod!(PauseTransformFeedback, "glPauseTransformFeedback")
fn_mod!(PixelStoref, "glPixelStoref")
fn_mod!(PixelStorei, "glPixelStorei")
fn_mod!(PointParameterf, "glPointParameterf")
fn_mod!(PointParameterfv, "glPointParameterfv")
fn_mod!(PointParameteri, "glPointParameteri")
fn_mod!(PointParameteriv, "glPointParameteriv")
fn_mod!(PointSize, "glPointSize")
fn_mod!(PolygonMode, "glPolygonMode")
fn_mod!(PolygonOffset, "glPolygonOffset")
fn_mod!(PopDebugGroup, "glPopDebugGroup")
fn_mod!(PrimitiveRestartIndex, "glPrimitiveRestartIndex")
fn_mod!(ProgramBinary, "glProgramBinary")
fn_mod!(ProgramParameteri, "glProgramParameteri")
fn_mod!(ProgramUniform1d, "glProgramUniform1d")
fn_mod!(ProgramUniform1dv, "glProgramUniform1dv")
fn_mod!(ProgramUniform1f, "glProgramUniform1f")
fn_mod!(ProgramUniform1fv, "glProgramUniform1fv")
fn_mod!(ProgramUniform1i, "glProgramUniform1i")
fn_mod!(ProgramUniform1iv, "glProgramUniform1iv")
fn_mod!(ProgramUniform1ui, "glProgramUniform1ui")
fn_mod!(ProgramUniform1uiv, "glProgramUniform1uiv")
fn_mod!(ProgramUniform2d, "glProgramUniform2d")
fn_mod!(ProgramUniform2dv, "glProgramUniform2dv")
fn_mod!(ProgramUniform2f, "glProgramUniform2f")
fn_mod!(ProgramUniform2fv, "glProgramUniform2fv")
fn_mod!(ProgramUniform2i, "glProgramUniform2i")
fn_mod!(ProgramUniform2iv, "glProgramUniform2iv")
fn_mod!(ProgramUniform2ui, "glProgramUniform2ui")
fn_mod!(ProgramUniform2uiv, "glProgramUniform2uiv")
fn_mod!(ProgramUniform3d, "glProgramUniform3d")
fn_mod!(ProgramUniform3dv, "glProgramUniform3dv")
fn_mod!(ProgramUniform3f, "glProgramUniform3f")
fn_mod!(ProgramUniform3fv, "glProgramUniform3fv")
fn_mod!(ProgramUniform3i, "glProgramUniform3i")
fn_mod!(ProgramUniform3iv, "glProgramUniform3iv")
fn_mod!(ProgramUniform3ui, "glProgramUniform3ui")
fn_mod!(ProgramUniform3uiv, "glProgramUniform3uiv")
fn_mod!(ProgramUniform4d, "glProgramUniform4d")
fn_mod!(ProgramUniform4dv, "glProgramUniform4dv")
fn_mod!(ProgramUniform4f, "glProgramUniform4f")
fn_mod!(ProgramUniform4fv, "glProgramUniform4fv")
fn_mod!(ProgramUniform4i, "glProgramUniform4i")
fn_mod!(ProgramUniform4iv, "glProgramUniform4iv")
fn_mod!(ProgramUniform4ui, "glProgramUniform4ui")
fn_mod!(ProgramUniform4uiv, "glProgramUniform4uiv")
fn_mod!(ProgramUniformMatrix2dv, "glProgramUniformMatrix2dv")
fn_mod!(ProgramUniformMatrix2fv, "glProgramUniformMatrix2fv")
fn_mod!(ProgramUniformMatrix2x3dv, "glProgramUniformMatrix2x3dv")
fn_mod!(ProgramUniformMatrix2x3fv, "glProgramUniformMatrix2x3fv")
fn_mod!(ProgramUniformMatrix2x4dv, "glProgramUniformMatrix2x4dv")
fn_mod!(ProgramUniformMatrix2x4fv, "glProgramUniformMatrix2x4fv")
fn_mod!(ProgramUniformMatrix3dv, "glProgramUniformMatrix3dv")
fn_mod!(ProgramUniformMatrix3fv, "glProgramUniformMatrix3fv")
fn_mod!(ProgramUniformMatrix3x2dv, "glProgramUniformMatrix3x2dv")
fn_mod!(ProgramUniformMatrix3x2fv, "glProgramUniformMatrix3x2fv")
fn_mod!(ProgramUniformMatrix3x4dv, "glProgramUniformMatrix3x4dv")
fn_mod!(ProgramUniformMatrix3x4fv, "glProgramUniformMatrix3x4fv")
fn_mod!(ProgramUniformMatrix4dv, "glProgramUniformMatrix4dv")
fn_mod!(ProgramUniformMatrix4fv, "glProgramUniformMatrix4fv")
fn_mod!(ProgramUniformMatrix4x2dv, "glProgramUniformMatrix4x2dv")
fn_mod!(ProgramUniformMatrix4x2fv, "glProgramUniformMatrix4x2fv")
fn_mod!(ProgramUniformMatrix4x3dv, "glProgramUniformMatrix4x3dv")
fn_mod!(ProgramUniformMatrix4x3fv, "glProgramUniformMatrix4x3fv")
fn_mod!(ProvokingVertex, "glProvokingVertex")
fn_mod!(PushDebugGroup, "glPushDebugGroup")
fn_mod!(QueryCounter, "glQueryCounter")
fn_mod!(ReadBuffer, "glReadBuffer")
fn_mod!(ReadPixels, "glReadPixels")
fn_mod!(ReleaseShaderCompiler, "glReleaseShaderCompiler")
fn_mod!(RenderbufferStorage, "glRenderbufferStorage")
fn_mod!(RenderbufferStorageMultisample, "glRenderbufferStorageMultisample")
fn_mod!(ResumeTransformFeedback, "glResumeTransformFeedback")
fn_mod!(SampleCoverage, "glSampleCoverage")
fn_mod!(SampleMaski, "glSampleMaski")
fn_mod!(SamplerParameterIiv, "glSamplerParameterIiv")
fn_mod!(SamplerParameterIuiv, "glSamplerParameterIuiv")
fn_mod!(SamplerParameterf, "glSamplerParameterf")
fn_mod!(SamplerParameterfv, "glSamplerParameterfv")
fn_mod!(SamplerParameteri, "glSamplerParameteri")
fn_mod!(SamplerParameteriv, "glSamplerParameteriv")
fn_mod!(Scissor, "glScissor")
fn_mod!(ScissorArrayv, "glScissorArrayv")
fn_mod!(ScissorIndexed, "glScissorIndexed")
fn_mod!(ScissorIndexedv, "glScissorIndexedv")
fn_mod!(SecondaryColorP3ui, "glSecondaryColorP3ui")
fn_mod!(SecondaryColorP3uiv, "glSecondaryColorP3uiv")
fn_mod!(ShaderBinary, "glShaderBinary")
fn_mod!(ShaderSource, "glShaderSource")
fn_mod!(ShaderStorageBlockBinding, "glShaderStorageBlockBinding")
fn_mod!(StencilFunc, "glStencilFunc")
fn_mod!(StencilFuncSeparate, "glStencilFuncSeparate")
fn_mod!(StencilMask, "glStencilMask")
fn_mod!(StencilMaskSeparate, "glStencilMaskSeparate")
fn_mod!(StencilOp, "glStencilOp")
fn_mod!(StencilOpSeparate, "glStencilOpSeparate")
fn_mod!(TexBuffer, "glTexBuffer")
fn_mod!(TexBufferRange, "glTexBufferRange")
fn_mod!(TexCoordP1ui, "glTexCoordP1ui")
fn_mod!(TexCoordP1uiv, "glTexCoordP1uiv")
fn_mod!(TexCoordP2ui, "glTexCoordP2ui")
fn_mod!(TexCoordP2uiv, "glTexCoordP2uiv")
fn_mod!(TexCoordP3ui, "glTexCoordP3ui")
fn_mod!(TexCoordP3uiv, "glTexCoordP3uiv")
fn_mod!(TexCoordP4ui, "glTexCoordP4ui")
fn_mod!(TexCoordP4uiv, "glTexCoordP4uiv")
fn_mod!(TexImage1D, "glTexImage1D")
fn_mod!(TexImage2D, "glTexImage2D")
fn_mod!(TexImage2DMultisample, "glTexImage2DMultisample")
fn_mod!(TexImage3D, "glTexImage3D")
fn_mod!(TexImage3DMultisample, "glTexImage3DMultisample")
fn_mod!(TexParameterIiv, "glTexParameterIiv")
fn_mod!(TexParameterIuiv, "glTexParameterIuiv")
fn_mod!(TexParameterf, "glTexParameterf")
fn_mod!(TexParameterfv, "glTexParameterfv")
fn_mod!(TexParameteri, "glTexParameteri")
fn_mod!(TexParameteriv, "glTexParameteriv")
fn_mod!(TexStorage1D, "glTexStorage1D")
fn_mod!(TexStorage2D, "glTexStorage2D")
fn_mod!(TexStorage2DMultisample, "glTexStorage2DMultisample")
fn_mod!(TexStorage3D, "glTexStorage3D")
fn_mod!(TexStorage3DMultisample, "glTexStorage3DMultisample")
fn_mod!(TexSubImage1D, "glTexSubImage1D")
fn_mod!(TexSubImage2D, "glTexSubImage2D")
fn_mod!(TexSubImage3D, "glTexSubImage3D")
fn_mod!(TextureView, "glTextureView")
fn_mod!(TransformFeedbackVaryings, "glTransformFeedbackVaryings")
fn_mod!(Uniform1d, "glUniform1d")
fn_mod!(Uniform1dv, "glUniform1dv")
fn_mod!(Uniform1f, "glUniform1f")
fn_mod!(Uniform1fv, "glUniform1fv")
fn_mod!(Uniform1i, "glUniform1i")
fn_mod!(Uniform1iv, "glUniform1iv")
fn_mod!(Uniform1ui, "glUniform1ui")
fn_mod!(Uniform1uiv, "glUniform1uiv")
fn_mod!(Uniform2d, "glUniform2d")
fn_mod!(Uniform2dv, "glUniform2dv")
fn_mod!(Uniform2f, "glUniform2f")
fn_mod!(Uniform2fv, "glUniform2fv")
fn_mod!(Uniform2i, "glUniform2i")
fn_mod!(Uniform2iv, "glUniform2iv")
fn_mod!(Uniform2ui, "glUniform2ui")
fn_mod!(Uniform2uiv, "glUniform2uiv")
fn_mod!(Uniform3d, "glUniform3d")
fn_mod!(Uniform3dv, "glUniform3dv")
fn_mod!(Uniform3f, "glUniform3f")
fn_mod!(Uniform3fv, "glUniform3fv")
fn_mod!(Uniform3i, "glUniform3i")
fn_mod!(Uniform3iv, "glUniform3iv")
fn_mod!(Uniform3ui, "glUniform3ui")
fn_mod!(Uniform3uiv, "glUniform3uiv")
fn_mod!(Uniform4d, "glUniform4d")
fn_mod!(Uniform4dv, "glUniform4dv")
fn_mod!(Uniform4f, "glUniform4f")
fn_mod!(Uniform4fv, "glUniform4fv")
fn_mod!(Uniform4i, "glUniform4i")
fn_mod!(Uniform4iv, "glUniform4iv")
fn_mod!(Uniform4ui, "glUniform4ui")
fn_mod!(Uniform4uiv, "glUniform4uiv")
fn_mod!(UniformBlockBinding, "glUniformBlockBinding")
fn_mod!(UniformMatrix2dv, "glUniformMatrix2dv")
fn_mod!(UniformMatrix2fv, "glUniformMatrix2fv")
fn_mod!(UniformMatrix2x3dv, "glUniformMatrix2x3dv")
fn_mod!(UniformMatrix2x3fv, "glUniformMatrix2x3fv")
fn_mod!(UniformMatrix2x4dv, "glUniformMatrix2x4dv")
fn_mod!(UniformMatrix2x4fv, "glUniformMatrix2x4fv")
fn_mod!(UniformMatrix3dv, "glUniformMatrix3dv")
fn_mod!(UniformMatrix3fv, "glUniformMatrix3fv")
fn_mod!(UniformMatrix3x2dv, "glUniformMatrix3x2dv")
fn_mod!(UniformMatrix3x2fv, "glUniformMatrix3x2fv")
fn_mod!(UniformMatrix3x4dv, "glUniformMatrix3x4dv")
fn_mod!(UniformMatrix3x4fv, "glUniformMatrix3x4fv")
fn_mod!(UniformMatrix4dv, "glUniformMatrix4dv")
fn_mod!(UniformMatrix4fv, "glUniformMatrix4fv")
fn_mod!(UniformMatrix4x2dv, "glUniformMatrix4x2dv")
fn_mod!(UniformMatrix4x2fv, "glUniformMatrix4x2fv")
fn_mod!(UniformMatrix4x3dv, "glUniformMatrix4x3dv")
fn_mod!(UniformMatrix4x3fv, "glUniformMatrix4x3fv")
fn_mod!(UniformSubroutinesuiv, "glUniformSubroutinesuiv")
fn_mod!(UnmapBuffer, "glUnmapBuffer")
fn_mod!(UseProgram, "glUseProgram")
fn_mod!(UseProgramStages, "glUseProgramStages")
fn_mod!(ValidateProgram, "glValidateProgram")
fn_mod!(ValidateProgramPipeline, "glValidateProgramPipeline")
fn_mod!(VertexAttrib1d, "glVertexAttrib1d")
fn_mod!(VertexAttrib1dv, "glVertexAttrib1dv")
fn_mod!(VertexAttrib1f, "glVertexAttrib1f")
fn_mod!(VertexAttrib1fv, "glVertexAttrib1fv")
fn_mod!(VertexAttrib1s, "glVertexAttrib1s")
fn_mod!(VertexAttrib1sv, "glVertexAttrib1sv")
fn_mod!(VertexAttrib2d, "glVertexAttrib2d")
fn_mod!(VertexAttrib2dv, "glVertexAttrib2dv")
fn_mod!(VertexAttrib2f, "glVertexAttrib2f")
fn_mod!(VertexAttrib2fv, "glVertexAttrib2fv")
fn_mod!(VertexAttrib2s, "glVertexAttrib2s")
fn_mod!(VertexAttrib2sv, "glVertexAttrib2sv")
fn_mod!(VertexAttrib3d, "glVertexAttrib3d")
fn_mod!(VertexAttrib3dv, "glVertexAttrib3dv")
fn_mod!(VertexAttrib3f, "glVertexAttrib3f")
fn_mod!(VertexAttrib3fv, "glVertexAttrib3fv")
fn_mod!(VertexAttrib3s, "glVertexAttrib3s")
fn_mod!(VertexAttrib3sv, "glVertexAttrib3sv")
fn_mod!(VertexAttrib4Nbv, "glVertexAttrib4Nbv")
fn_mod!(VertexAttrib4Niv, "glVertexAttrib4Niv")
fn_mod!(VertexAttrib4Nsv, "glVertexAttrib4Nsv")
fn_mod!(VertexAttrib4Nub, "glVertexAttrib4Nub")
fn_mod!(VertexAttrib4Nubv, "glVertexAttrib4Nubv")
fn_mod!(VertexAttrib4Nuiv, "glVertexAttrib4Nuiv")
fn_mod!(VertexAttrib4Nusv, "glVertexAttrib4Nusv")
fn_mod!(VertexAttrib4bv, "glVertexAttrib4bv")
fn_mod!(VertexAttrib4d, "glVertexAttrib4d")
fn_mod!(VertexAttrib4dv, "glVertexAttrib4dv")
fn_mod!(VertexAttrib4f, "glVertexAttrib4f")
fn_mod!(VertexAttrib4fv, "glVertexAttrib4fv")
fn_mod!(VertexAttrib4iv, "glVertexAttrib4iv")
fn_mod!(VertexAttrib4s, "glVertexAttrib4s")
fn_mod!(VertexAttrib4sv, "glVertexAttrib4sv")
fn_mod!(VertexAttrib4ubv, "glVertexAttrib4ubv")
fn_mod!(VertexAttrib4uiv, "glVertexAttrib4uiv")
fn_mod!(VertexAttrib4usv, "glVertexAttrib4usv")
fn_mod!(VertexAttribBinding, "glVertexAttribBinding")
fn_mod!(VertexAttribDivisor, "glVertexAttribDivisor")
fn_mod!(VertexAttribFormat, "glVertexAttribFormat")
fn_mod!(VertexAttribI1i, "glVertexAttribI1i")
fn_mod!(VertexAttribI1iv, "glVertexAttribI1iv")
fn_mod!(VertexAttribI1ui, "glVertexAttribI1ui")
fn_mod!(VertexAttribI1uiv, "glVertexAttribI1uiv")
fn_mod!(VertexAttribI2i, "glVertexAttribI2i")
fn_mod!(VertexAttribI2iv, "glVertexAttribI2iv")
fn_mod!(VertexAttribI2ui, "glVertexAttribI2ui")
fn_mod!(VertexAttribI2uiv, "glVertexAttribI2uiv")
fn_mod!(VertexAttribI3i, "glVertexAttribI3i")
fn_mod!(VertexAttribI3iv, "glVertexAttribI3iv")
fn_mod!(VertexAttribI3ui, "glVertexAttribI3ui")
fn_mod!(VertexAttribI3uiv, "glVertexAttribI3uiv")
fn_mod!(VertexAttribI4bv, "glVertexAttribI4bv")
fn_mod!(VertexAttribI4i, "glVertexAttribI4i")
fn_mod!(VertexAttribI4iv, "glVertexAttribI4iv")
fn_mod!(VertexAttribI4sv, "glVertexAttribI4sv")
fn_mod!(VertexAttribI4ubv, "glVertexAttribI4ubv")
fn_mod!(VertexAttribI4ui, "glVertexAttribI4ui")
fn_mod!(VertexAttribI4uiv, "glVertexAttribI4uiv")
fn_mod!(VertexAttribI4usv, "glVertexAttribI4usv")
fn_mod!(VertexAttribIFormat, "glVertexAttribIFormat")
fn_mod!(VertexAttribIPointer, "glVertexAttribIPointer")
fn_mod!(VertexAttribL1d, "glVertexAttribL1d")
fn_mod!(VertexAttribL1dv, "glVertexAttribL1dv")
fn_mod!(VertexAttribL2d, "glVertexAttribL2d")
fn_mod!(VertexAttribL2dv, "glVertexAttribL2dv")
fn_mod!(VertexAttribL3d, "glVertexAttribL3d")
fn_mod!(VertexAttribL3dv, "glVertexAttribL3dv")
fn_mod!(VertexAttribL4d, "glVertexAttribL4d")
fn_mod!(VertexAttribL4dv, "glVertexAttribL4dv")
fn_mod!(VertexAttribLFormat, "glVertexAttribLFormat")
fn_mod!(VertexAttribLPointer, "glVertexAttribLPointer")
fn_mod!(VertexAttribP1ui, "glVertexAttribP1ui")
fn_mod!(VertexAttribP1uiv, "glVertexAttribP1uiv")
fn_mod!(VertexAttribP2ui, "glVertexAttribP2ui")
fn_mod!(VertexAttribP2uiv, "glVertexAttribP2uiv")
fn_mod!(VertexAttribP3ui, "glVertexAttribP3ui")
fn_mod!(VertexAttribP3uiv, "glVertexAttribP3uiv")
fn_mod!(VertexAttribP4ui, "glVertexAttribP4ui")
fn_mod!(VertexAttribP4uiv, "glVertexAttribP4uiv")
fn_mod!(VertexAttribPointer, "glVertexAttribPointer")
fn_mod!(VertexBindingDivisor, "glVertexBindingDivisor")
fn_mod!(VertexP2ui, "glVertexP2ui")
fn_mod!(VertexP2uiv, "glVertexP2uiv")
fn_mod!(VertexP3ui, "glVertexP3ui")
fn_mod!(VertexP3uiv, "glVertexP3uiv")
fn_mod!(VertexP4ui, "glVertexP4ui")
fn_mod!(VertexP4uiv, "glVertexP4uiv")
fn_mod!(Viewport, "glViewport")
fn_mod!(ViewportArrayv, "glViewportArrayv")
fn_mod!(ViewportIndexedf, "glViewportIndexedf")
fn_mod!(ViewportIndexedfv, "glViewportIndexedfv")
fn_mod!(WaitSync, "glWaitSync")

mod failing {
    use std::libc::*;
    use super::types::*;
    
    macro_rules! failing(
        (fn $name:ident()) => (pub extern "C" fn $name() { fail!(stringify!($name was not loaded)) });
        (fn $name:ident() -> $ret_ty:ty) => (pub extern "C" fn $name() -> $ret_ty { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*)) => (pub extern "C" fn $name($(_: $arg_ty),*) { fail!(stringify!($name was not loaded)) });
        (fn $name:ident($($arg_ty:ty),*) -> $ret_ty:ty) => (pub extern "C" fn $name($(_: $arg_ty),*) -> $ret_ty { fail!(stringify!($name was not loaded)) });
    )
    
    failing!(fn ActiveShaderProgram(GLuint, GLuint))
    failing!(fn ActiveTexture(GLenum))
    failing!(fn AttachShader(GLuint, GLuint))
    failing!(fn BeginConditionalRender(GLuint, GLenum))
    failing!(fn BeginQuery(GLenum, GLuint))
    failing!(fn BeginQueryIndexed(GLenum, GLuint, GLuint))
    failing!(fn BeginTransformFeedback(GLenum))
    failing!(fn BindAttribLocation(GLuint, GLuint, *GLchar))
    failing!(fn BindBuffer(GLenum, GLuint))
    failing!(fn BindBufferBase(GLenum, GLuint, GLuint))
    failing!(fn BindBufferRange(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr))
    failing!(fn BindFragDataLocation(GLuint, GLuint, *GLchar))
    failing!(fn BindFragDataLocationIndexed(GLuint, GLuint, GLuint, *GLchar))
    failing!(fn BindFramebuffer(GLenum, GLuint))
    failing!(fn BindImageTexture(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum))
    failing!(fn BindProgramPipeline(GLuint))
    failing!(fn BindRenderbuffer(GLenum, GLuint))
    failing!(fn BindSampler(GLuint, GLuint))
    failing!(fn BindTexture(GLenum, GLuint))
    failing!(fn BindTransformFeedback(GLenum, GLuint))
    failing!(fn BindVertexArray(GLuint))
    failing!(fn BindVertexBuffer(GLuint, GLuint, GLintptr, GLsizei))
    failing!(fn BlendColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn BlendEquation(GLenum))
    failing!(fn BlendEquationSeparate(GLenum, GLenum))
    failing!(fn BlendEquationSeparatei(GLuint, GLenum, GLenum))
    failing!(fn BlendEquationi(GLuint, GLenum))
    failing!(fn BlendFunc(GLenum, GLenum))
    failing!(fn BlendFuncSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn BlendFuncSeparatei(GLuint, GLenum, GLenum, GLenum, GLenum))
    failing!(fn BlendFunci(GLuint, GLenum, GLenum))
    failing!(fn BlitFramebuffer(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum))
    failing!(fn BufferData(GLenum, GLsizeiptr, *GLvoid, GLenum))
    failing!(fn BufferSubData(GLenum, GLintptr, GLsizeiptr, *GLvoid))
    failing!(fn CheckFramebufferStatus(GLenum) -> GLenum)
    failing!(fn ClampColor(GLenum, GLenum))
    failing!(fn Clear(GLbitfield))
    failing!(fn ClearBufferData(GLenum, GLenum, GLenum, GLenum, *c_void))
    failing!(fn ClearBufferSubData(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *c_void))
    failing!(fn ClearBufferfi(GLenum, GLint, GLfloat, GLint))
    failing!(fn ClearBufferfv(GLenum, GLint, *GLfloat))
    failing!(fn ClearBufferiv(GLenum, GLint, *GLint))
    failing!(fn ClearBufferuiv(GLenum, GLint, *GLuint))
    failing!(fn ClearColor(GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ClearDepth(GLdouble))
    failing!(fn ClearDepthf(GLfloat))
    failing!(fn ClearStencil(GLint))
    failing!(fn ClientWaitSync(GLsync, GLbitfield, GLuint64) -> GLenum)
    failing!(fn ColorMask(GLboolean, GLboolean, GLboolean, GLboolean))
    failing!(fn ColorMaski(GLuint, GLboolean, GLboolean, GLboolean, GLboolean))
    failing!(fn ColorP3ui(GLenum, GLuint))
    failing!(fn ColorP3uiv(GLenum, *GLuint))
    failing!(fn ColorP4ui(GLenum, GLuint))
    failing!(fn ColorP4uiv(GLenum, *GLuint))
    failing!(fn CompileShader(GLuint))
    failing!(fn CompressedTexImage1D(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *GLvoid))
    failing!(fn CompressedTexImage2D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *GLvoid))
    failing!(fn CompressedTexImage3D(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *GLvoid))
    failing!(fn CompressedTexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *GLvoid))
    failing!(fn CompressedTexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *GLvoid))
    failing!(fn CompressedTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *GLvoid))
    failing!(fn CopyBufferSubData(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr))
    failing!(fn CopyImageSubData(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei))
    failing!(fn CopyTexImage1D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint))
    failing!(fn CopyTexImage2D(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint))
    failing!(fn CopyTexSubImage1D(GLenum, GLint, GLint, GLint, GLint, GLsizei))
    failing!(fn CopyTexSubImage2D(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CopyTexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn CreateProgram() -> GLuint)
    failing!(fn CreateShader(GLenum) -> GLuint)
    failing!(fn CreateShaderProgramv(GLenum, GLsizei, **GLchar) -> GLuint)
    failing!(fn CullFace(GLenum))
    failing!(fn DebugMessageCallback(GLDEBUGPROC, *c_void))
    failing!(fn DebugMessageControl(GLenum, GLenum, GLenum, GLsizei, *GLuint, GLboolean))
    failing!(fn DebugMessageInsert(GLenum, GLenum, GLuint, GLenum, GLsizei, *GLchar))
    failing!(fn DeleteBuffers(GLsizei, *GLuint))
    failing!(fn DeleteFramebuffers(GLsizei, *GLuint))
    failing!(fn DeleteProgram(GLuint))
    failing!(fn DeleteProgramPipelines(GLsizei, *GLuint))
    failing!(fn DeleteQueries(GLsizei, *GLuint))
    failing!(fn DeleteRenderbuffers(GLsizei, *GLuint))
    failing!(fn DeleteSamplers(GLsizei, *GLuint))
    failing!(fn DeleteShader(GLuint))
    failing!(fn DeleteSync(GLsync))
    failing!(fn DeleteTextures(GLsizei, *GLuint))
    failing!(fn DeleteTransformFeedbacks(GLsizei, *GLuint))
    failing!(fn DeleteVertexArrays(GLsizei, *GLuint))
    failing!(fn DepthFunc(GLenum))
    failing!(fn DepthMask(GLboolean))
    failing!(fn DepthRange(GLdouble, GLdouble))
    failing!(fn DepthRangeArrayv(GLuint, GLsizei, *GLdouble))
    failing!(fn DepthRangeIndexed(GLuint, GLdouble, GLdouble))
    failing!(fn DepthRangef(GLfloat, GLfloat))
    failing!(fn DetachShader(GLuint, GLuint))
    failing!(fn Disable(GLenum))
    failing!(fn DisableVertexAttribArray(GLuint))
    failing!(fn Disablei(GLenum, GLuint))
    failing!(fn DispatchCompute(GLuint, GLuint, GLuint))
    failing!(fn DispatchComputeIndirect(GLintptr))
    failing!(fn DrawArrays(GLenum, GLint, GLsizei))
    failing!(fn DrawArraysIndirect(GLenum, *GLvoid))
    failing!(fn DrawArraysInstanced(GLenum, GLint, GLsizei, GLsizei))
    failing!(fn DrawArraysInstancedBaseInstance(GLenum, GLint, GLsizei, GLsizei, GLuint))
    failing!(fn DrawBuffer(GLenum))
    failing!(fn DrawBuffers(GLsizei, *GLenum))
    failing!(fn DrawElements(GLenum, GLsizei, GLenum, *GLvoid))
    failing!(fn DrawElementsBaseVertex(GLenum, GLsizei, GLenum, *GLvoid, GLint))
    failing!(fn DrawElementsIndirect(GLenum, GLenum, *GLvoid))
    failing!(fn DrawElementsInstanced(GLenum, GLsizei, GLenum, *GLvoid, GLsizei))
    failing!(fn DrawElementsInstancedBaseInstance(GLenum, GLsizei, GLenum, *c_void, GLsizei, GLuint))
    failing!(fn DrawElementsInstancedBaseVertex(GLenum, GLsizei, GLenum, *GLvoid, GLsizei, GLint))
    failing!(fn DrawElementsInstancedBaseVertexBaseInstance(GLenum, GLsizei, GLenum, *c_void, GLsizei, GLint, GLuint))
    failing!(fn DrawRangeElements(GLenum, GLuint, GLuint, GLsizei, GLenum, *GLvoid))
    failing!(fn DrawRangeElementsBaseVertex(GLenum, GLuint, GLuint, GLsizei, GLenum, *GLvoid, GLint))
    failing!(fn DrawTransformFeedback(GLenum, GLuint))
    failing!(fn DrawTransformFeedbackInstanced(GLenum, GLuint, GLsizei))
    failing!(fn DrawTransformFeedbackStream(GLenum, GLuint, GLuint))
    failing!(fn DrawTransformFeedbackStreamInstanced(GLenum, GLuint, GLuint, GLsizei))
    failing!(fn Enable(GLenum))
    failing!(fn EnableVertexAttribArray(GLuint))
    failing!(fn Enablei(GLenum, GLuint))
    failing!(fn EndConditionalRender())
    failing!(fn EndQuery(GLenum))
    failing!(fn EndQueryIndexed(GLenum, GLuint))
    failing!(fn EndTransformFeedback())
    failing!(fn FenceSync(GLenum, GLbitfield) -> GLsync)
    failing!(fn Finish())
    failing!(fn Flush())
    failing!(fn FlushMappedBufferRange(GLenum, GLintptr, GLsizeiptr))
    failing!(fn FramebufferParameteri(GLenum, GLenum, GLint))
    failing!(fn FramebufferRenderbuffer(GLenum, GLenum, GLenum, GLuint))
    failing!(fn FramebufferTexture(GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture1D(GLenum, GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture2D(GLenum, GLenum, GLenum, GLuint, GLint))
    failing!(fn FramebufferTexture3D(GLenum, GLenum, GLenum, GLuint, GLint, GLint))
    failing!(fn FramebufferTextureLayer(GLenum, GLenum, GLuint, GLint, GLint))
    failing!(fn FrontFace(GLenum))
    failing!(fn GenBuffers(GLsizei, *mut GLuint))
    failing!(fn GenFramebuffers(GLsizei, *mut GLuint))
    failing!(fn GenProgramPipelines(GLsizei, *mut GLuint))
    failing!(fn GenQueries(GLsizei, *mut GLuint))
    failing!(fn GenRenderbuffers(GLsizei, *mut GLuint))
    failing!(fn GenSamplers(GLsizei, *mut GLuint))
    failing!(fn GenTextures(GLsizei, *mut GLuint))
    failing!(fn GenTransformFeedbacks(GLsizei, *mut GLuint))
    failing!(fn GenVertexArrays(GLsizei, *mut GLuint))
    failing!(fn GenerateMipmap(GLenum))
    failing!(fn GetActiveAtomicCounterBufferiv(GLuint, GLuint, GLenum, *mut GLint))
    failing!(fn GetActiveAttrib(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetActiveSubroutineName(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveSubroutineUniformName(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveSubroutineUniformiv(GLuint, GLenum, GLuint, GLenum, *mut GLint))
    failing!(fn GetActiveUniform(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar))
    failing!(fn GetActiveUniformBlockName(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveUniformBlockiv(GLuint, GLuint, GLenum, *mut GLint))
    failing!(fn GetActiveUniformName(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetActiveUniformsiv(GLuint, GLsizei, *GLuint, GLenum, *mut GLint))
    failing!(fn GetAttachedShaders(GLuint, GLsizei, *mut GLsizei, *mut GLuint))
    failing!(fn GetAttribLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetBooleani_v(GLenum, GLuint, *mut GLboolean))
    failing!(fn GetBooleanv(GLenum, *mut GLboolean))
    failing!(fn GetBufferParameteri64v(GLenum, GLenum, *mut GLint64))
    failing!(fn GetBufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetBufferPointerv(GLenum, GLenum, **mut GLvoid))
    failing!(fn GetBufferSubData(GLenum, GLintptr, GLsizeiptr, *mut GLvoid))
    failing!(fn GetCompressedTexImage(GLenum, GLint, *mut GLvoid))
    failing!(fn GetDebugMessageLog(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint)
    failing!(fn GetDoublei_v(GLenum, GLuint, *mut GLdouble))
    failing!(fn GetDoublev(GLenum, *mut GLdouble))
    failing!(fn GetError() -> GLenum)
    failing!(fn GetFloati_v(GLenum, GLuint, *mut GLfloat))
    failing!(fn GetFloatv(GLenum, *mut GLfloat))
    failing!(fn GetFragDataIndex(GLuint, *GLchar) -> GLint)
    failing!(fn GetFragDataLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetFramebufferAttachmentParameteriv(GLenum, GLenum, GLenum, *mut GLint))
    failing!(fn GetFramebufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetInteger64i_v(GLenum, GLuint, *mut GLint64))
    failing!(fn GetInteger64v(GLenum, *mut GLint64))
    failing!(fn GetIntegeri_v(GLenum, GLuint, *mut GLint))
    failing!(fn GetIntegerv(GLenum, *mut GLint))
    failing!(fn GetInternalformati64v(GLenum, GLenum, GLenum, GLsizei, *mut GLint64))
    failing!(fn GetInternalformativ(GLenum, GLenum, GLenum, GLsizei, *mut GLint))
    failing!(fn GetMultisamplefv(GLenum, GLuint, *mut GLfloat))
    failing!(fn GetObjectLabel(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetObjectPtrLabel(*c_void, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramBinary(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut GLvoid))
    failing!(fn GetProgramInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramInterfaceiv(GLuint, GLenum, GLenum, *mut GLint))
    failing!(fn GetProgramPipelineInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramPipelineiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetProgramResourceIndex(GLuint, GLenum, *GLchar) -> GLuint)
    failing!(fn GetProgramResourceLocation(GLuint, GLenum, *GLchar) -> GLint)
    failing!(fn GetProgramResourceLocationIndex(GLuint, GLenum, *GLchar) -> GLint)
    failing!(fn GetProgramResourceName(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetProgramResourceiv(GLuint, GLenum, GLuint, GLsizei, *GLenum, GLsizei, *mut GLsizei, *mut GLint))
    failing!(fn GetProgramStageiv(GLuint, GLenum, GLenum, *mut GLint))
    failing!(fn GetProgramiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryIndexediv(GLenum, GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjecti64v(GLuint, GLenum, *mut GLint64))
    failing!(fn GetQueryObjectiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetQueryObjectui64v(GLuint, GLenum, *mut GLuint64))
    failing!(fn GetQueryObjectuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetQueryiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetRenderbufferParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetSamplerParameterIiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetSamplerParameterIuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetSamplerParameterfv(GLuint, GLenum, *mut GLfloat))
    failing!(fn GetSamplerParameteriv(GLuint, GLenum, *mut GLint))
    failing!(fn GetShaderInfoLog(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderPrecisionFormat(GLenum, GLenum, *mut GLint, *mut GLint))
    failing!(fn GetShaderSource(GLuint, GLsizei, *mut GLsizei, *mut GLchar))
    failing!(fn GetShaderiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetString(GLenum) -> *GLubyte)
    failing!(fn GetStringi(GLenum, GLuint) -> *GLubyte)
    failing!(fn GetSubroutineIndex(GLuint, GLenum, *GLchar) -> GLuint)
    failing!(fn GetSubroutineUniformLocation(GLuint, GLenum, *GLchar) -> GLint)
    failing!(fn GetSynciv(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint))
    failing!(fn GetTexImage(GLenum, GLint, GLenum, GLenum, *mut GLvoid))
    failing!(fn GetTexLevelParameterfv(GLenum, GLint, GLenum, *mut GLfloat))
    failing!(fn GetTexLevelParameteriv(GLenum, GLint, GLenum, *mut GLint))
    failing!(fn GetTexParameterIiv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTexParameterIuiv(GLenum, GLenum, *mut GLuint))
    failing!(fn GetTexParameterfv(GLenum, GLenum, *mut GLfloat))
    failing!(fn GetTexParameteriv(GLenum, GLenum, *mut GLint))
    failing!(fn GetTransformFeedbackVarying(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar))
    failing!(fn GetUniformBlockIndex(GLuint, *GLchar) -> GLuint)
    failing!(fn GetUniformIndices(GLuint, GLsizei, **GLchar, *mut GLuint))
    failing!(fn GetUniformLocation(GLuint, *GLchar) -> GLint)
    failing!(fn GetUniformSubroutineuiv(GLenum, GLint, *mut GLuint))
    failing!(fn GetUniformdv(GLuint, GLint, *mut GLdouble))
    failing!(fn GetUniformfv(GLuint, GLint, *mut GLfloat))
    failing!(fn GetUniformiv(GLuint, GLint, *mut GLint))
    failing!(fn GetUniformuiv(GLuint, GLint, *mut GLuint))
    failing!(fn GetVertexAttribIiv(GLuint, GLenum, *mut GLint))
    failing!(fn GetVertexAttribIuiv(GLuint, GLenum, *mut GLuint))
    failing!(fn GetVertexAttribLdv(GLuint, GLenum, *mut GLdouble))
    failing!(fn GetVertexAttribPointerv(GLuint, GLenum, **mut GLvoid))
    failing!(fn GetVertexAttribdv(GLuint, GLenum, *mut GLdouble))
    failing!(fn GetVertexAttribfv(GLuint, GLenum, *mut GLfloat))
    failing!(fn GetVertexAttribiv(GLuint, GLenum, *mut GLint))
    failing!(fn Hint(GLenum, GLenum))
    failing!(fn InvalidateBufferData(GLuint))
    failing!(fn InvalidateBufferSubData(GLuint, GLintptr, GLsizeiptr))
    failing!(fn InvalidateFramebuffer(GLenum, GLsizei, *GLenum))
    failing!(fn InvalidateSubFramebuffer(GLenum, GLsizei, *GLenum, GLint, GLint, GLsizei, GLsizei))
    failing!(fn InvalidateTexImage(GLuint, GLint))
    failing!(fn InvalidateTexSubImage(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei))
    failing!(fn IsBuffer(GLuint) -> GLboolean)
    failing!(fn IsEnabled(GLenum) -> GLboolean)
    failing!(fn IsEnabledi(GLenum, GLuint) -> GLboolean)
    failing!(fn IsFramebuffer(GLuint) -> GLboolean)
    failing!(fn IsProgram(GLuint) -> GLboolean)
    failing!(fn IsProgramPipeline(GLuint) -> GLboolean)
    failing!(fn IsQuery(GLuint) -> GLboolean)
    failing!(fn IsRenderbuffer(GLuint) -> GLboolean)
    failing!(fn IsSampler(GLuint) -> GLboolean)
    failing!(fn IsShader(GLuint) -> GLboolean)
    failing!(fn IsSync(GLsync) -> GLboolean)
    failing!(fn IsTexture(GLuint) -> GLboolean)
    failing!(fn IsTransformFeedback(GLuint) -> GLboolean)
    failing!(fn IsVertexArray(GLuint) -> GLboolean)
    failing!(fn LineWidth(GLfloat))
    failing!(fn LinkProgram(GLuint))
    failing!(fn LogicOp(GLenum))
    failing!(fn MapBuffer(GLenum, GLenum) -> *c_void)
    failing!(fn MapBufferRange(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *c_void)
    failing!(fn MemoryBarrier(GLbitfield))
    failing!(fn MinSampleShading(GLfloat))
    failing!(fn MultiDrawArrays(GLenum, *GLint, *GLsizei, GLsizei))
    failing!(fn MultiDrawArraysIndirect(GLenum, *c_void, GLsizei, GLsizei))
    failing!(fn MultiDrawElements(GLenum, *GLsizei, GLenum, **GLvoid, GLsizei))
    failing!(fn MultiDrawElementsBaseVertex(GLenum, *GLsizei, GLenum, **GLvoid, GLsizei, *GLint))
    failing!(fn MultiDrawElementsIndirect(GLenum, GLenum, *c_void, GLsizei, GLsizei))
    failing!(fn MultiTexCoordP1ui(GLenum, GLenum, GLuint))
    failing!(fn MultiTexCoordP1uiv(GLenum, GLenum, *GLuint))
    failing!(fn MultiTexCoordP2ui(GLenum, GLenum, GLuint))
    failing!(fn MultiTexCoordP2uiv(GLenum, GLenum, *GLuint))
    failing!(fn MultiTexCoordP3ui(GLenum, GLenum, GLuint))
    failing!(fn MultiTexCoordP3uiv(GLenum, GLenum, *GLuint))
    failing!(fn MultiTexCoordP4ui(GLenum, GLenum, GLuint))
    failing!(fn MultiTexCoordP4uiv(GLenum, GLenum, *GLuint))
    failing!(fn NormalP3ui(GLenum, GLuint))
    failing!(fn NormalP3uiv(GLenum, *GLuint))
    failing!(fn ObjectLabel(GLenum, GLuint, GLsizei, *GLchar))
    failing!(fn ObjectPtrLabel(*c_void, GLsizei, *GLchar))
    failing!(fn PatchParameterfv(GLenum, *GLfloat))
    failing!(fn PatchParameteri(GLenum, GLint))
    failing!(fn PauseTransformFeedback())
    failing!(fn PixelStoref(GLenum, GLfloat))
    failing!(fn PixelStorei(GLenum, GLint))
    failing!(fn PointParameterf(GLenum, GLfloat))
    failing!(fn PointParameterfv(GLenum, *GLfloat))
    failing!(fn PointParameteri(GLenum, GLint))
    failing!(fn PointParameteriv(GLenum, *GLint))
    failing!(fn PointSize(GLfloat))
    failing!(fn PolygonMode(GLenum, GLenum))
    failing!(fn PolygonOffset(GLfloat, GLfloat))
    failing!(fn PopDebugGroup())
    failing!(fn PrimitiveRestartIndex(GLuint))
    failing!(fn ProgramBinary(GLuint, GLenum, *GLvoid, GLsizei))
    failing!(fn ProgramParameteri(GLuint, GLenum, GLint))
    failing!(fn ProgramUniform1d(GLuint, GLint, GLdouble))
    failing!(fn ProgramUniform1dv(GLuint, GLint, GLsizei, *GLdouble))
    failing!(fn ProgramUniform1f(GLuint, GLint, GLfloat))
    failing!(fn ProgramUniform1fv(GLuint, GLint, GLsizei, *GLfloat))
    failing!(fn ProgramUniform1i(GLuint, GLint, GLint))
    failing!(fn ProgramUniform1iv(GLuint, GLint, GLsizei, *GLint))
    failing!(fn ProgramUniform1ui(GLuint, GLint, GLuint))
    failing!(fn ProgramUniform1uiv(GLuint, GLint, GLsizei, *GLuint))
    failing!(fn ProgramUniform2d(GLuint, GLint, GLdouble, GLdouble))
    failing!(fn ProgramUniform2dv(GLuint, GLint, GLsizei, *GLdouble))
    failing!(fn ProgramUniform2f(GLuint, GLint, GLfloat, GLfloat))
    failing!(fn ProgramUniform2fv(GLuint, GLint, GLsizei, *GLfloat))
    failing!(fn ProgramUniform2i(GLuint, GLint, GLint, GLint))
    failing!(fn ProgramUniform2iv(GLuint, GLint, GLsizei, *GLint))
    failing!(fn ProgramUniform2ui(GLuint, GLint, GLuint, GLuint))
    failing!(fn ProgramUniform2uiv(GLuint, GLint, GLsizei, *GLuint))
    failing!(fn ProgramUniform3d(GLuint, GLint, GLdouble, GLdouble, GLdouble))
    failing!(fn ProgramUniform3dv(GLuint, GLint, GLsizei, *GLdouble))
    failing!(fn ProgramUniform3f(GLuint, GLint, GLfloat, GLfloat, GLfloat))
    failing!(fn ProgramUniform3fv(GLuint, GLint, GLsizei, *GLfloat))
    failing!(fn ProgramUniform3i(GLuint, GLint, GLint, GLint, GLint))
    failing!(fn ProgramUniform3iv(GLuint, GLint, GLsizei, *GLint))
    failing!(fn ProgramUniform3ui(GLuint, GLint, GLuint, GLuint, GLuint))
    failing!(fn ProgramUniform3uiv(GLuint, GLint, GLsizei, *GLuint))
    failing!(fn ProgramUniform4d(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn ProgramUniform4dv(GLuint, GLint, GLsizei, *GLdouble))
    failing!(fn ProgramUniform4f(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ProgramUniform4fv(GLuint, GLint, GLsizei, *GLfloat))
    failing!(fn ProgramUniform4i(GLuint, GLint, GLint, GLint, GLint, GLint))
    failing!(fn ProgramUniform4iv(GLuint, GLint, GLsizei, *GLint))
    failing!(fn ProgramUniform4ui(GLuint, GLint, GLuint, GLuint, GLuint, GLuint))
    failing!(fn ProgramUniform4uiv(GLuint, GLint, GLsizei, *GLuint))
    failing!(fn ProgramUniformMatrix2dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix2fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix2x3dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix2x3fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix2x4dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix2x4fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix3dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix3fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix3x2dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix3x2fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix3x4dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix3x4fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix4dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix4fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix4x2dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix4x2fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProgramUniformMatrix4x3dv(GLuint, GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn ProgramUniformMatrix4x3fv(GLuint, GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn ProvokingVertex(GLenum))
    failing!(fn PushDebugGroup(GLenum, GLuint, GLsizei, *GLchar))
    failing!(fn QueryCounter(GLuint, GLenum))
    failing!(fn ReadBuffer(GLenum))
    failing!(fn ReadPixels(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut GLvoid))
    failing!(fn ReleaseShaderCompiler())
    failing!(fn RenderbufferStorage(GLenum, GLenum, GLsizei, GLsizei))
    failing!(fn RenderbufferStorageMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei))
    failing!(fn ResumeTransformFeedback())
    failing!(fn SampleCoverage(GLfloat, GLboolean))
    failing!(fn SampleMaski(GLuint, GLbitfield))
    failing!(fn SamplerParameterIiv(GLuint, GLenum, *GLint))
    failing!(fn SamplerParameterIuiv(GLuint, GLenum, *GLuint))
    failing!(fn SamplerParameterf(GLuint, GLenum, GLfloat))
    failing!(fn SamplerParameterfv(GLuint, GLenum, *GLfloat))
    failing!(fn SamplerParameteri(GLuint, GLenum, GLint))
    failing!(fn SamplerParameteriv(GLuint, GLenum, *GLint))
    failing!(fn Scissor(GLint, GLint, GLsizei, GLsizei))
    failing!(fn ScissorArrayv(GLuint, GLsizei, *GLint))
    failing!(fn ScissorIndexed(GLuint, GLint, GLint, GLsizei, GLsizei))
    failing!(fn ScissorIndexedv(GLuint, *GLint))
    failing!(fn SecondaryColorP3ui(GLenum, GLuint))
    failing!(fn SecondaryColorP3uiv(GLenum, *GLuint))
    failing!(fn ShaderBinary(GLsizei, *GLuint, GLenum, *GLvoid, GLsizei))
    failing!(fn ShaderSource(GLuint, GLsizei, **GLchar, *GLint))
    failing!(fn ShaderStorageBlockBinding(GLuint, GLuint, GLuint))
    failing!(fn StencilFunc(GLenum, GLint, GLuint))
    failing!(fn StencilFuncSeparate(GLenum, GLenum, GLint, GLuint))
    failing!(fn StencilMask(GLuint))
    failing!(fn StencilMaskSeparate(GLenum, GLuint))
    failing!(fn StencilOp(GLenum, GLenum, GLenum))
    failing!(fn StencilOpSeparate(GLenum, GLenum, GLenum, GLenum))
    failing!(fn TexBuffer(GLenum, GLenum, GLuint))
    failing!(fn TexBufferRange(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr))
    failing!(fn TexCoordP1ui(GLenum, GLuint))
    failing!(fn TexCoordP1uiv(GLenum, *GLuint))
    failing!(fn TexCoordP2ui(GLenum, GLuint))
    failing!(fn TexCoordP2uiv(GLenum, *GLuint))
    failing!(fn TexCoordP3ui(GLenum, GLuint))
    failing!(fn TexCoordP3uiv(GLenum, *GLuint))
    failing!(fn TexCoordP4ui(GLenum, GLuint))
    failing!(fn TexCoordP4uiv(GLenum, *GLuint))
    failing!(fn TexImage1D(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *GLvoid))
    failing!(fn TexImage2D(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *GLvoid))
    failing!(fn TexImage2DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean))
    failing!(fn TexImage3D(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *GLvoid))
    failing!(fn TexImage3DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean))
    failing!(fn TexParameterIiv(GLenum, GLenum, *GLint))
    failing!(fn TexParameterIuiv(GLenum, GLenum, *GLuint))
    failing!(fn TexParameterf(GLenum, GLenum, GLfloat))
    failing!(fn TexParameterfv(GLenum, GLenum, *GLfloat))
    failing!(fn TexParameteri(GLenum, GLenum, GLint))
    failing!(fn TexParameteriv(GLenum, GLenum, *GLint))
    failing!(fn TexStorage1D(GLenum, GLsizei, GLenum, GLsizei))
    failing!(fn TexStorage2D(GLenum, GLsizei, GLenum, GLsizei, GLsizei))
    failing!(fn TexStorage2DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean))
    failing!(fn TexStorage3D(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei))
    failing!(fn TexStorage3DMultisample(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean))
    failing!(fn TexSubImage1D(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *GLvoid))
    failing!(fn TexSubImage2D(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *GLvoid))
    failing!(fn TexSubImage3D(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *GLvoid))
    failing!(fn TextureView(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint))
    failing!(fn TransformFeedbackVaryings(GLuint, GLsizei, **GLchar, GLenum))
    failing!(fn Uniform1d(GLint, GLdouble))
    failing!(fn Uniform1dv(GLint, GLsizei, *GLdouble))
    failing!(fn Uniform1f(GLint, GLfloat))
    failing!(fn Uniform1fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform1i(GLint, GLint))
    failing!(fn Uniform1iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform1ui(GLint, GLuint))
    failing!(fn Uniform1uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform2d(GLint, GLdouble, GLdouble))
    failing!(fn Uniform2dv(GLint, GLsizei, *GLdouble))
    failing!(fn Uniform2f(GLint, GLfloat, GLfloat))
    failing!(fn Uniform2fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform2i(GLint, GLint, GLint))
    failing!(fn Uniform2iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform2ui(GLint, GLuint, GLuint))
    failing!(fn Uniform2uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform3d(GLint, GLdouble, GLdouble, GLdouble))
    failing!(fn Uniform3dv(GLint, GLsizei, *GLdouble))
    failing!(fn Uniform3f(GLint, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform3fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform3i(GLint, GLint, GLint, GLint))
    failing!(fn Uniform3iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform3ui(GLint, GLuint, GLuint, GLuint))
    failing!(fn Uniform3uiv(GLint, GLsizei, *GLuint))
    failing!(fn Uniform4d(GLint, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn Uniform4dv(GLint, GLsizei, *GLdouble))
    failing!(fn Uniform4f(GLint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn Uniform4fv(GLint, GLsizei, *GLfloat))
    failing!(fn Uniform4i(GLint, GLint, GLint, GLint, GLint))
    failing!(fn Uniform4iv(GLint, GLsizei, *GLint))
    failing!(fn Uniform4ui(GLint, GLuint, GLuint, GLuint, GLuint))
    failing!(fn Uniform4uiv(GLint, GLsizei, *GLuint))
    failing!(fn UniformBlockBinding(GLuint, GLuint, GLuint))
    failing!(fn UniformMatrix2dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix2x3dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix2x3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix2x4dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix2x4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3x2dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix3x2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix3x4dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix3x4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix4fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4x2dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix4x2fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformMatrix4x3dv(GLint, GLsizei, GLboolean, *GLdouble))
    failing!(fn UniformMatrix4x3fv(GLint, GLsizei, GLboolean, *GLfloat))
    failing!(fn UniformSubroutinesuiv(GLenum, GLsizei, *GLuint))
    failing!(fn UnmapBuffer(GLenum) -> GLboolean)
    failing!(fn UseProgram(GLuint))
    failing!(fn UseProgramStages(GLuint, GLbitfield, GLuint))
    failing!(fn ValidateProgram(GLuint))
    failing!(fn ValidateProgramPipeline(GLuint))
    failing!(fn VertexAttrib1d(GLuint, GLdouble))
    failing!(fn VertexAttrib1dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib1f(GLuint, GLfloat))
    failing!(fn VertexAttrib1fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib1s(GLuint, GLshort))
    failing!(fn VertexAttrib1sv(GLuint, *GLshort))
    failing!(fn VertexAttrib2d(GLuint, GLdouble, GLdouble))
    failing!(fn VertexAttrib2dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib2f(GLuint, GLfloat, GLfloat))
    failing!(fn VertexAttrib2fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib2s(GLuint, GLshort, GLshort))
    failing!(fn VertexAttrib2sv(GLuint, *GLshort))
    failing!(fn VertexAttrib3d(GLuint, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttrib3dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib3f(GLuint, GLfloat, GLfloat, GLfloat))
    failing!(fn VertexAttrib3fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib3s(GLuint, GLshort, GLshort, GLshort))
    failing!(fn VertexAttrib3sv(GLuint, *GLshort))
    failing!(fn VertexAttrib4Nbv(GLuint, *GLbyte))
    failing!(fn VertexAttrib4Niv(GLuint, *GLint))
    failing!(fn VertexAttrib4Nsv(GLuint, *GLshort))
    failing!(fn VertexAttrib4Nub(GLuint, GLubyte, GLubyte, GLubyte, GLubyte))
    failing!(fn VertexAttrib4Nubv(GLuint, *GLubyte))
    failing!(fn VertexAttrib4Nuiv(GLuint, *GLuint))
    failing!(fn VertexAttrib4Nusv(GLuint, *GLushort))
    failing!(fn VertexAttrib4bv(GLuint, *GLbyte))
    failing!(fn VertexAttrib4d(GLuint, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttrib4dv(GLuint, *GLdouble))
    failing!(fn VertexAttrib4f(GLuint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn VertexAttrib4fv(GLuint, *GLfloat))
    failing!(fn VertexAttrib4iv(GLuint, *GLint))
    failing!(fn VertexAttrib4s(GLuint, GLshort, GLshort, GLshort, GLshort))
    failing!(fn VertexAttrib4sv(GLuint, *GLshort))
    failing!(fn VertexAttrib4ubv(GLuint, *GLubyte))
    failing!(fn VertexAttrib4uiv(GLuint, *GLuint))
    failing!(fn VertexAttrib4usv(GLuint, *GLushort))
    failing!(fn VertexAttribBinding(GLuint, GLuint))
    failing!(fn VertexAttribDivisor(GLuint, GLuint))
    failing!(fn VertexAttribFormat(GLuint, GLint, GLenum, GLboolean, GLuint))
    failing!(fn VertexAttribI1i(GLuint, GLint))
    failing!(fn VertexAttribI1iv(GLuint, *GLint))
    failing!(fn VertexAttribI1ui(GLuint, GLuint))
    failing!(fn VertexAttribI1uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI2i(GLuint, GLint, GLint))
    failing!(fn VertexAttribI2iv(GLuint, *GLint))
    failing!(fn VertexAttribI2ui(GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI2uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI3i(GLuint, GLint, GLint, GLint))
    failing!(fn VertexAttribI3iv(GLuint, *GLint))
    failing!(fn VertexAttribI3ui(GLuint, GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI3uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI4bv(GLuint, *GLbyte))
    failing!(fn VertexAttribI4i(GLuint, GLint, GLint, GLint, GLint))
    failing!(fn VertexAttribI4iv(GLuint, *GLint))
    failing!(fn VertexAttribI4sv(GLuint, *GLshort))
    failing!(fn VertexAttribI4ubv(GLuint, *GLubyte))
    failing!(fn VertexAttribI4ui(GLuint, GLuint, GLuint, GLuint, GLuint))
    failing!(fn VertexAttribI4uiv(GLuint, *GLuint))
    failing!(fn VertexAttribI4usv(GLuint, *GLushort))
    failing!(fn VertexAttribIFormat(GLuint, GLint, GLenum, GLuint))
    failing!(fn VertexAttribIPointer(GLuint, GLint, GLenum, GLsizei, *GLvoid))
    failing!(fn VertexAttribL1d(GLuint, GLdouble))
    failing!(fn VertexAttribL1dv(GLuint, *GLdouble))
    failing!(fn VertexAttribL2d(GLuint, GLdouble, GLdouble))
    failing!(fn VertexAttribL2dv(GLuint, *GLdouble))
    failing!(fn VertexAttribL3d(GLuint, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttribL3dv(GLuint, *GLdouble))
    failing!(fn VertexAttribL4d(GLuint, GLdouble, GLdouble, GLdouble, GLdouble))
    failing!(fn VertexAttribL4dv(GLuint, *GLdouble))
    failing!(fn VertexAttribLFormat(GLuint, GLint, GLenum, GLuint))
    failing!(fn VertexAttribLPointer(GLuint, GLint, GLenum, GLsizei, *GLvoid))
    failing!(fn VertexAttribP1ui(GLuint, GLenum, GLboolean, GLuint))
    failing!(fn VertexAttribP1uiv(GLuint, GLenum, GLboolean, *GLuint))
    failing!(fn VertexAttribP2ui(GLuint, GLenum, GLboolean, GLuint))
    failing!(fn VertexAttribP2uiv(GLuint, GLenum, GLboolean, *GLuint))
    failing!(fn VertexAttribP3ui(GLuint, GLenum, GLboolean, GLuint))
    failing!(fn VertexAttribP3uiv(GLuint, GLenum, GLboolean, *GLuint))
    failing!(fn VertexAttribP4ui(GLuint, GLenum, GLboolean, GLuint))
    failing!(fn VertexAttribP4uiv(GLuint, GLenum, GLboolean, *GLuint))
    failing!(fn VertexAttribPointer(GLuint, GLint, GLenum, GLboolean, GLsizei, *GLvoid))
    failing!(fn VertexBindingDivisor(GLuint, GLuint))
    failing!(fn VertexP2ui(GLenum, GLuint))
    failing!(fn VertexP2uiv(GLenum, *GLuint))
    failing!(fn VertexP3ui(GLenum, GLuint))
    failing!(fn VertexP3uiv(GLenum, *GLuint))
    failing!(fn VertexP4ui(GLenum, GLuint))
    failing!(fn VertexP4uiv(GLenum, *GLuint))
    failing!(fn Viewport(GLint, GLint, GLsizei, GLsizei))
    failing!(fn ViewportArrayv(GLuint, GLsizei, *GLfloat))
    failing!(fn ViewportIndexedf(GLuint, GLfloat, GLfloat, GLfloat, GLfloat))
    failing!(fn ViewportIndexedfv(GLuint, *GLfloat))
    failing!(fn WaitSync(GLsync, GLbitfield, GLuint64))
}

/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
///
/// ~~~
/// let gl = gl::load_with(glfw::get_proc_address);
/// ~~~
pub fn load_with(loadfn: &fn(symbol: &str) -> Option<extern "C" fn()>) {
    ActiveShaderProgram::load_with(|s| loadfn(s));
    ActiveTexture::load_with(|s| loadfn(s));
    AttachShader::load_with(|s| loadfn(s));
    BeginConditionalRender::load_with(|s| loadfn(s));
    BeginQuery::load_with(|s| loadfn(s));
    BeginQueryIndexed::load_with(|s| loadfn(s));
    BeginTransformFeedback::load_with(|s| loadfn(s));
    BindAttribLocation::load_with(|s| loadfn(s));
    BindBuffer::load_with(|s| loadfn(s));
    BindBufferBase::load_with(|s| loadfn(s));
    BindBufferRange::load_with(|s| loadfn(s));
    BindFragDataLocation::load_with(|s| loadfn(s));
    BindFragDataLocationIndexed::load_with(|s| loadfn(s));
    BindFramebuffer::load_with(|s| loadfn(s));
    BindImageTexture::load_with(|s| loadfn(s));
    BindProgramPipeline::load_with(|s| loadfn(s));
    BindRenderbuffer::load_with(|s| loadfn(s));
    BindSampler::load_with(|s| loadfn(s));
    BindTexture::load_with(|s| loadfn(s));
    BindTransformFeedback::load_with(|s| loadfn(s));
    BindVertexArray::load_with(|s| loadfn(s));
    BindVertexBuffer::load_with(|s| loadfn(s));
    BlendColor::load_with(|s| loadfn(s));
    BlendEquation::load_with(|s| loadfn(s));
    BlendEquationSeparate::load_with(|s| loadfn(s));
    BlendEquationSeparatei::load_with(|s| loadfn(s));
    BlendEquationi::load_with(|s| loadfn(s));
    BlendFunc::load_with(|s| loadfn(s));
    BlendFuncSeparate::load_with(|s| loadfn(s));
    BlendFuncSeparatei::load_with(|s| loadfn(s));
    BlendFunci::load_with(|s| loadfn(s));
    BlitFramebuffer::load_with(|s| loadfn(s));
    BufferData::load_with(|s| loadfn(s));
    BufferSubData::load_with(|s| loadfn(s));
    CheckFramebufferStatus::load_with(|s| loadfn(s));
    ClampColor::load_with(|s| loadfn(s));
    Clear::load_with(|s| loadfn(s));
    ClearBufferData::load_with(|s| loadfn(s));
    ClearBufferSubData::load_with(|s| loadfn(s));
    ClearBufferfi::load_with(|s| loadfn(s));
    ClearBufferfv::load_with(|s| loadfn(s));
    ClearBufferiv::load_with(|s| loadfn(s));
    ClearBufferuiv::load_with(|s| loadfn(s));
    ClearColor::load_with(|s| loadfn(s));
    ClearDepth::load_with(|s| loadfn(s));
    ClearDepthf::load_with(|s| loadfn(s));
    ClearStencil::load_with(|s| loadfn(s));
    ClientWaitSync::load_with(|s| loadfn(s));
    ColorMask::load_with(|s| loadfn(s));
    ColorMaski::load_with(|s| loadfn(s));
    ColorP3ui::load_with(|s| loadfn(s));
    ColorP3uiv::load_with(|s| loadfn(s));
    ColorP4ui::load_with(|s| loadfn(s));
    ColorP4uiv::load_with(|s| loadfn(s));
    CompileShader::load_with(|s| loadfn(s));
    CompressedTexImage1D::load_with(|s| loadfn(s));
    CompressedTexImage2D::load_with(|s| loadfn(s));
    CompressedTexImage3D::load_with(|s| loadfn(s));
    CompressedTexSubImage1D::load_with(|s| loadfn(s));
    CompressedTexSubImage2D::load_with(|s| loadfn(s));
    CompressedTexSubImage3D::load_with(|s| loadfn(s));
    CopyBufferSubData::load_with(|s| loadfn(s));
    CopyImageSubData::load_with(|s| loadfn(s));
    CopyTexImage1D::load_with(|s| loadfn(s));
    CopyTexImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage1D::load_with(|s| loadfn(s));
    CopyTexSubImage2D::load_with(|s| loadfn(s));
    CopyTexSubImage3D::load_with(|s| loadfn(s));
    CreateProgram::load_with(|s| loadfn(s));
    CreateShader::load_with(|s| loadfn(s));
    CreateShaderProgramv::load_with(|s| loadfn(s));
    CullFace::load_with(|s| loadfn(s));
    DebugMessageCallback::load_with(|s| loadfn(s));
    DebugMessageControl::load_with(|s| loadfn(s));
    DebugMessageInsert::load_with(|s| loadfn(s));
    DeleteBuffers::load_with(|s| loadfn(s));
    DeleteFramebuffers::load_with(|s| loadfn(s));
    DeleteProgram::load_with(|s| loadfn(s));
    DeleteProgramPipelines::load_with(|s| loadfn(s));
    DeleteQueries::load_with(|s| loadfn(s));
    DeleteRenderbuffers::load_with(|s| loadfn(s));
    DeleteSamplers::load_with(|s| loadfn(s));
    DeleteShader::load_with(|s| loadfn(s));
    DeleteSync::load_with(|s| loadfn(s));
    DeleteTextures::load_with(|s| loadfn(s));
    DeleteTransformFeedbacks::load_with(|s| loadfn(s));
    DeleteVertexArrays::load_with(|s| loadfn(s));
    DepthFunc::load_with(|s| loadfn(s));
    DepthMask::load_with(|s| loadfn(s));
    DepthRange::load_with(|s| loadfn(s));
    DepthRangeArrayv::load_with(|s| loadfn(s));
    DepthRangeIndexed::load_with(|s| loadfn(s));
    DepthRangef::load_with(|s| loadfn(s));
    DetachShader::load_with(|s| loadfn(s));
    Disable::load_with(|s| loadfn(s));
    DisableVertexAttribArray::load_with(|s| loadfn(s));
    Disablei::load_with(|s| loadfn(s));
    DispatchCompute::load_with(|s| loadfn(s));
    DispatchComputeIndirect::load_with(|s| loadfn(s));
    DrawArrays::load_with(|s| loadfn(s));
    DrawArraysIndirect::load_with(|s| loadfn(s));
    DrawArraysInstanced::load_with(|s| loadfn(s));
    DrawArraysInstancedBaseInstance::load_with(|s| loadfn(s));
    DrawBuffer::load_with(|s| loadfn(s));
    DrawBuffers::load_with(|s| loadfn(s));
    DrawElements::load_with(|s| loadfn(s));
    DrawElementsBaseVertex::load_with(|s| loadfn(s));
    DrawElementsIndirect::load_with(|s| loadfn(s));
    DrawElementsInstanced::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseInstance::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseVertex::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseVertexBaseInstance::load_with(|s| loadfn(s));
    DrawRangeElements::load_with(|s| loadfn(s));
    DrawRangeElementsBaseVertex::load_with(|s| loadfn(s));
    DrawTransformFeedback::load_with(|s| loadfn(s));
    DrawTransformFeedbackInstanced::load_with(|s| loadfn(s));
    DrawTransformFeedbackStream::load_with(|s| loadfn(s));
    DrawTransformFeedbackStreamInstanced::load_with(|s| loadfn(s));
    Enable::load_with(|s| loadfn(s));
    EnableVertexAttribArray::load_with(|s| loadfn(s));
    Enablei::load_with(|s| loadfn(s));
    EndConditionalRender::load_with(|s| loadfn(s));
    EndQuery::load_with(|s| loadfn(s));
    EndQueryIndexed::load_with(|s| loadfn(s));
    EndTransformFeedback::load_with(|s| loadfn(s));
    FenceSync::load_with(|s| loadfn(s));
    Finish::load_with(|s| loadfn(s));
    Flush::load_with(|s| loadfn(s));
    FlushMappedBufferRange::load_with(|s| loadfn(s));
    FramebufferParameteri::load_with(|s| loadfn(s));
    FramebufferRenderbuffer::load_with(|s| loadfn(s));
    FramebufferTexture::load_with(|s| loadfn(s));
    FramebufferTexture1D::load_with(|s| loadfn(s));
    FramebufferTexture2D::load_with(|s| loadfn(s));
    FramebufferTexture3D::load_with(|s| loadfn(s));
    FramebufferTextureLayer::load_with(|s| loadfn(s));
    FrontFace::load_with(|s| loadfn(s));
    GenBuffers::load_with(|s| loadfn(s));
    GenFramebuffers::load_with(|s| loadfn(s));
    GenProgramPipelines::load_with(|s| loadfn(s));
    GenQueries::load_with(|s| loadfn(s));
    GenRenderbuffers::load_with(|s| loadfn(s));
    GenSamplers::load_with(|s| loadfn(s));
    GenTextures::load_with(|s| loadfn(s));
    GenTransformFeedbacks::load_with(|s| loadfn(s));
    GenVertexArrays::load_with(|s| loadfn(s));
    GenerateMipmap::load_with(|s| loadfn(s));
    GetActiveAtomicCounterBufferiv::load_with(|s| loadfn(s));
    GetActiveAttrib::load_with(|s| loadfn(s));
    GetActiveSubroutineName::load_with(|s| loadfn(s));
    GetActiveSubroutineUniformName::load_with(|s| loadfn(s));
    GetActiveSubroutineUniformiv::load_with(|s| loadfn(s));
    GetActiveUniform::load_with(|s| loadfn(s));
    GetActiveUniformBlockName::load_with(|s| loadfn(s));
    GetActiveUniformBlockiv::load_with(|s| loadfn(s));
    GetActiveUniformName::load_with(|s| loadfn(s));
    GetActiveUniformsiv::load_with(|s| loadfn(s));
    GetAttachedShaders::load_with(|s| loadfn(s));
    GetAttribLocation::load_with(|s| loadfn(s));
    GetBooleani_v::load_with(|s| loadfn(s));
    GetBooleanv::load_with(|s| loadfn(s));
    GetBufferParameteri64v::load_with(|s| loadfn(s));
    GetBufferParameteriv::load_with(|s| loadfn(s));
    GetBufferPointerv::load_with(|s| loadfn(s));
    GetBufferSubData::load_with(|s| loadfn(s));
    GetCompressedTexImage::load_with(|s| loadfn(s));
    GetDebugMessageLog::load_with(|s| loadfn(s));
    GetDoublei_v::load_with(|s| loadfn(s));
    GetDoublev::load_with(|s| loadfn(s));
    GetError::load_with(|s| loadfn(s));
    GetFloati_v::load_with(|s| loadfn(s));
    GetFloatv::load_with(|s| loadfn(s));
    GetFragDataIndex::load_with(|s| loadfn(s));
    GetFragDataLocation::load_with(|s| loadfn(s));
    GetFramebufferAttachmentParameteriv::load_with(|s| loadfn(s));
    GetFramebufferParameteriv::load_with(|s| loadfn(s));
    GetInteger64i_v::load_with(|s| loadfn(s));
    GetInteger64v::load_with(|s| loadfn(s));
    GetIntegeri_v::load_with(|s| loadfn(s));
    GetIntegerv::load_with(|s| loadfn(s));
    GetInternalformati64v::load_with(|s| loadfn(s));
    GetInternalformativ::load_with(|s| loadfn(s));
    GetMultisamplefv::load_with(|s| loadfn(s));
    GetObjectLabel::load_with(|s| loadfn(s));
    GetObjectPtrLabel::load_with(|s| loadfn(s));
    GetProgramBinary::load_with(|s| loadfn(s));
    GetProgramInfoLog::load_with(|s| loadfn(s));
    GetProgramInterfaceiv::load_with(|s| loadfn(s));
    GetProgramPipelineInfoLog::load_with(|s| loadfn(s));
    GetProgramPipelineiv::load_with(|s| loadfn(s));
    GetProgramResourceIndex::load_with(|s| loadfn(s));
    GetProgramResourceLocation::load_with(|s| loadfn(s));
    GetProgramResourceLocationIndex::load_with(|s| loadfn(s));
    GetProgramResourceName::load_with(|s| loadfn(s));
    GetProgramResourceiv::load_with(|s| loadfn(s));
    GetProgramStageiv::load_with(|s| loadfn(s));
    GetProgramiv::load_with(|s| loadfn(s));
    GetQueryIndexediv::load_with(|s| loadfn(s));
    GetQueryObjecti64v::load_with(|s| loadfn(s));
    GetQueryObjectiv::load_with(|s| loadfn(s));
    GetQueryObjectui64v::load_with(|s| loadfn(s));
    GetQueryObjectuiv::load_with(|s| loadfn(s));
    GetQueryiv::load_with(|s| loadfn(s));
    GetRenderbufferParameteriv::load_with(|s| loadfn(s));
    GetSamplerParameterIiv::load_with(|s| loadfn(s));
    GetSamplerParameterIuiv::load_with(|s| loadfn(s));
    GetSamplerParameterfv::load_with(|s| loadfn(s));
    GetSamplerParameteriv::load_with(|s| loadfn(s));
    GetShaderInfoLog::load_with(|s| loadfn(s));
    GetShaderPrecisionFormat::load_with(|s| loadfn(s));
    GetShaderSource::load_with(|s| loadfn(s));
    GetShaderiv::load_with(|s| loadfn(s));
    GetString::load_with(|s| loadfn(s));
    GetStringi::load_with(|s| loadfn(s));
    GetSubroutineIndex::load_with(|s| loadfn(s));
    GetSubroutineUniformLocation::load_with(|s| loadfn(s));
    GetSynciv::load_with(|s| loadfn(s));
    GetTexImage::load_with(|s| loadfn(s));
    GetTexLevelParameterfv::load_with(|s| loadfn(s));
    GetTexLevelParameteriv::load_with(|s| loadfn(s));
    GetTexParameterIiv::load_with(|s| loadfn(s));
    GetTexParameterIuiv::load_with(|s| loadfn(s));
    GetTexParameterfv::load_with(|s| loadfn(s));
    GetTexParameteriv::load_with(|s| loadfn(s));
    GetTransformFeedbackVarying::load_with(|s| loadfn(s));
    GetUniformBlockIndex::load_with(|s| loadfn(s));
    GetUniformIndices::load_with(|s| loadfn(s));
    GetUniformLocation::load_with(|s| loadfn(s));
    GetUniformSubroutineuiv::load_with(|s| loadfn(s));
    GetUniformdv::load_with(|s| loadfn(s));
    GetUniformfv::load_with(|s| loadfn(s));
    GetUniformiv::load_with(|s| loadfn(s));
    GetUniformuiv::load_with(|s| loadfn(s));
    GetVertexAttribIiv::load_with(|s| loadfn(s));
    GetVertexAttribIuiv::load_with(|s| loadfn(s));
    GetVertexAttribLdv::load_with(|s| loadfn(s));
    GetVertexAttribPointerv::load_with(|s| loadfn(s));
    GetVertexAttribdv::load_with(|s| loadfn(s));
    GetVertexAttribfv::load_with(|s| loadfn(s));
    GetVertexAttribiv::load_with(|s| loadfn(s));
    Hint::load_with(|s| loadfn(s));
    InvalidateBufferData::load_with(|s| loadfn(s));
    InvalidateBufferSubData::load_with(|s| loadfn(s));
    InvalidateFramebuffer::load_with(|s| loadfn(s));
    InvalidateSubFramebuffer::load_with(|s| loadfn(s));
    InvalidateTexImage::load_with(|s| loadfn(s));
    InvalidateTexSubImage::load_with(|s| loadfn(s));
    IsBuffer::load_with(|s| loadfn(s));
    IsEnabled::load_with(|s| loadfn(s));
    IsEnabledi::load_with(|s| loadfn(s));
    IsFramebuffer::load_with(|s| loadfn(s));
    IsProgram::load_with(|s| loadfn(s));
    IsProgramPipeline::load_with(|s| loadfn(s));
    IsQuery::load_with(|s| loadfn(s));
    IsRenderbuffer::load_with(|s| loadfn(s));
    IsSampler::load_with(|s| loadfn(s));
    IsShader::load_with(|s| loadfn(s));
    IsSync::load_with(|s| loadfn(s));
    IsTexture::load_with(|s| loadfn(s));
    IsTransformFeedback::load_with(|s| loadfn(s));
    IsVertexArray::load_with(|s| loadfn(s));
    LineWidth::load_with(|s| loadfn(s));
    LinkProgram::load_with(|s| loadfn(s));
    LogicOp::load_with(|s| loadfn(s));
    MapBuffer::load_with(|s| loadfn(s));
    MapBufferRange::load_with(|s| loadfn(s));
    MemoryBarrier::load_with(|s| loadfn(s));
    MinSampleShading::load_with(|s| loadfn(s));
    MultiDrawArrays::load_with(|s| loadfn(s));
    MultiDrawArraysIndirect::load_with(|s| loadfn(s));
    MultiDrawElements::load_with(|s| loadfn(s));
    MultiDrawElementsBaseVertex::load_with(|s| loadfn(s));
    MultiDrawElementsIndirect::load_with(|s| loadfn(s));
    MultiTexCoordP1ui::load_with(|s| loadfn(s));
    MultiTexCoordP1uiv::load_with(|s| loadfn(s));
    MultiTexCoordP2ui::load_with(|s| loadfn(s));
    MultiTexCoordP2uiv::load_with(|s| loadfn(s));
    MultiTexCoordP3ui::load_with(|s| loadfn(s));
    MultiTexCoordP3uiv::load_with(|s| loadfn(s));
    MultiTexCoordP4ui::load_with(|s| loadfn(s));
    MultiTexCoordP4uiv::load_with(|s| loadfn(s));
    NormalP3ui::load_with(|s| loadfn(s));
    NormalP3uiv::load_with(|s| loadfn(s));
    ObjectLabel::load_with(|s| loadfn(s));
    ObjectPtrLabel::load_with(|s| loadfn(s));
    PatchParameterfv::load_with(|s| loadfn(s));
    PatchParameteri::load_with(|s| loadfn(s));
    PauseTransformFeedback::load_with(|s| loadfn(s));
    PixelStoref::load_with(|s| loadfn(s));
    PixelStorei::load_with(|s| loadfn(s));
    PointParameterf::load_with(|s| loadfn(s));
    PointParameterfv::load_with(|s| loadfn(s));
    PointParameteri::load_with(|s| loadfn(s));
    PointParameteriv::load_with(|s| loadfn(s));
    PointSize::load_with(|s| loadfn(s));
    PolygonMode::load_with(|s| loadfn(s));
    PolygonOffset::load_with(|s| loadfn(s));
    PopDebugGroup::load_with(|s| loadfn(s));
    PrimitiveRestartIndex::load_with(|s| loadfn(s));
    ProgramBinary::load_with(|s| loadfn(s));
    ProgramParameteri::load_with(|s| loadfn(s));
    ProgramUniform1d::load_with(|s| loadfn(s));
    ProgramUniform1dv::load_with(|s| loadfn(s));
    ProgramUniform1f::load_with(|s| loadfn(s));
    ProgramUniform1fv::load_with(|s| loadfn(s));
    ProgramUniform1i::load_with(|s| loadfn(s));
    ProgramUniform1iv::load_with(|s| loadfn(s));
    ProgramUniform1ui::load_with(|s| loadfn(s));
    ProgramUniform1uiv::load_with(|s| loadfn(s));
    ProgramUniform2d::load_with(|s| loadfn(s));
    ProgramUniform2dv::load_with(|s| loadfn(s));
    ProgramUniform2f::load_with(|s| loadfn(s));
    ProgramUniform2fv::load_with(|s| loadfn(s));
    ProgramUniform2i::load_with(|s| loadfn(s));
    ProgramUniform2iv::load_with(|s| loadfn(s));
    ProgramUniform2ui::load_with(|s| loadfn(s));
    ProgramUniform2uiv::load_with(|s| loadfn(s));
    ProgramUniform3d::load_with(|s| loadfn(s));
    ProgramUniform3dv::load_with(|s| loadfn(s));
    ProgramUniform3f::load_with(|s| loadfn(s));
    ProgramUniform3fv::load_with(|s| loadfn(s));
    ProgramUniform3i::load_with(|s| loadfn(s));
    ProgramUniform3iv::load_with(|s| loadfn(s));
    ProgramUniform3ui::load_with(|s| loadfn(s));
    ProgramUniform3uiv::load_with(|s| loadfn(s));
    ProgramUniform4d::load_with(|s| loadfn(s));
    ProgramUniform4dv::load_with(|s| loadfn(s));
    ProgramUniform4f::load_with(|s| loadfn(s));
    ProgramUniform4fv::load_with(|s| loadfn(s));
    ProgramUniform4i::load_with(|s| loadfn(s));
    ProgramUniform4iv::load_with(|s| loadfn(s));
    ProgramUniform4ui::load_with(|s| loadfn(s));
    ProgramUniform4uiv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x3dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x3fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x4dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x4fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x2dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x2fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x4dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x4fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x2dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x2fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x3dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x3fv::load_with(|s| loadfn(s));
    ProvokingVertex::load_with(|s| loadfn(s));
    PushDebugGroup::load_with(|s| loadfn(s));
    QueryCounter::load_with(|s| loadfn(s));
    ReadBuffer::load_with(|s| loadfn(s));
    ReadPixels::load_with(|s| loadfn(s));
    ReleaseShaderCompiler::load_with(|s| loadfn(s));
    RenderbufferStorage::load_with(|s| loadfn(s));
    RenderbufferStorageMultisample::load_with(|s| loadfn(s));
    ResumeTransformFeedback::load_with(|s| loadfn(s));
    SampleCoverage::load_with(|s| loadfn(s));
    SampleMaski::load_with(|s| loadfn(s));
    SamplerParameterIiv::load_with(|s| loadfn(s));
    SamplerParameterIuiv::load_with(|s| loadfn(s));
    SamplerParameterf::load_with(|s| loadfn(s));
    SamplerParameterfv::load_with(|s| loadfn(s));
    SamplerParameteri::load_with(|s| loadfn(s));
    SamplerParameteriv::load_with(|s| loadfn(s));
    Scissor::load_with(|s| loadfn(s));
    ScissorArrayv::load_with(|s| loadfn(s));
    ScissorIndexed::load_with(|s| loadfn(s));
    ScissorIndexedv::load_with(|s| loadfn(s));
    SecondaryColorP3ui::load_with(|s| loadfn(s));
    SecondaryColorP3uiv::load_with(|s| loadfn(s));
    ShaderBinary::load_with(|s| loadfn(s));
    ShaderSource::load_with(|s| loadfn(s));
    ShaderStorageBlockBinding::load_with(|s| loadfn(s));
    StencilFunc::load_with(|s| loadfn(s));
    StencilFuncSeparate::load_with(|s| loadfn(s));
    StencilMask::load_with(|s| loadfn(s));
    StencilMaskSeparate::load_with(|s| loadfn(s));
    StencilOp::load_with(|s| loadfn(s));
    StencilOpSeparate::load_with(|s| loadfn(s));
    TexBuffer::load_with(|s| loadfn(s));
    TexBufferRange::load_with(|s| loadfn(s));
    TexCoordP1ui::load_with(|s| loadfn(s));
    TexCoordP1uiv::load_with(|s| loadfn(s));
    TexCoordP2ui::load_with(|s| loadfn(s));
    TexCoordP2uiv::load_with(|s| loadfn(s));
    TexCoordP3ui::load_with(|s| loadfn(s));
    TexCoordP3uiv::load_with(|s| loadfn(s));
    TexCoordP4ui::load_with(|s| loadfn(s));
    TexCoordP4uiv::load_with(|s| loadfn(s));
    TexImage1D::load_with(|s| loadfn(s));
    TexImage2D::load_with(|s| loadfn(s));
    TexImage2DMultisample::load_with(|s| loadfn(s));
    TexImage3D::load_with(|s| loadfn(s));
    TexImage3DMultisample::load_with(|s| loadfn(s));
    TexParameterIiv::load_with(|s| loadfn(s));
    TexParameterIuiv::load_with(|s| loadfn(s));
    TexParameterf::load_with(|s| loadfn(s));
    TexParameterfv::load_with(|s| loadfn(s));
    TexParameteri::load_with(|s| loadfn(s));
    TexParameteriv::load_with(|s| loadfn(s));
    TexStorage1D::load_with(|s| loadfn(s));
    TexStorage2D::load_with(|s| loadfn(s));
    TexStorage2DMultisample::load_with(|s| loadfn(s));
    TexStorage3D::load_with(|s| loadfn(s));
    TexStorage3DMultisample::load_with(|s| loadfn(s));
    TexSubImage1D::load_with(|s| loadfn(s));
    TexSubImage2D::load_with(|s| loadfn(s));
    TexSubImage3D::load_with(|s| loadfn(s));
    TextureView::load_with(|s| loadfn(s));
    TransformFeedbackVaryings::load_with(|s| loadfn(s));
    Uniform1d::load_with(|s| loadfn(s));
    Uniform1dv::load_with(|s| loadfn(s));
    Uniform1f::load_with(|s| loadfn(s));
    Uniform1fv::load_with(|s| loadfn(s));
    Uniform1i::load_with(|s| loadfn(s));
    Uniform1iv::load_with(|s| loadfn(s));
    Uniform1ui::load_with(|s| loadfn(s));
    Uniform1uiv::load_with(|s| loadfn(s));
    Uniform2d::load_with(|s| loadfn(s));
    Uniform2dv::load_with(|s| loadfn(s));
    Uniform2f::load_with(|s| loadfn(s));
    Uniform2fv::load_with(|s| loadfn(s));
    Uniform2i::load_with(|s| loadfn(s));
    Uniform2iv::load_with(|s| loadfn(s));
    Uniform2ui::load_with(|s| loadfn(s));
    Uniform2uiv::load_with(|s| loadfn(s));
    Uniform3d::load_with(|s| loadfn(s));
    Uniform3dv::load_with(|s| loadfn(s));
    Uniform3f::load_with(|s| loadfn(s));
    Uniform3fv::load_with(|s| loadfn(s));
    Uniform3i::load_with(|s| loadfn(s));
    Uniform3iv::load_with(|s| loadfn(s));
    Uniform3ui::load_with(|s| loadfn(s));
    Uniform3uiv::load_with(|s| loadfn(s));
    Uniform4d::load_with(|s| loadfn(s));
    Uniform4dv::load_with(|s| loadfn(s));
    Uniform4f::load_with(|s| loadfn(s));
    Uniform4fv::load_with(|s| loadfn(s));
    Uniform4i::load_with(|s| loadfn(s));
    Uniform4iv::load_with(|s| loadfn(s));
    Uniform4ui::load_with(|s| loadfn(s));
    Uniform4uiv::load_with(|s| loadfn(s));
    UniformBlockBinding::load_with(|s| loadfn(s));
    UniformMatrix2dv::load_with(|s| loadfn(s));
    UniformMatrix2fv::load_with(|s| loadfn(s));
    UniformMatrix2x3dv::load_with(|s| loadfn(s));
    UniformMatrix2x3fv::load_with(|s| loadfn(s));
    UniformMatrix2x4dv::load_with(|s| loadfn(s));
    UniformMatrix2x4fv::load_with(|s| loadfn(s));
    UniformMatrix3dv::load_with(|s| loadfn(s));
    UniformMatrix3fv::load_with(|s| loadfn(s));
    UniformMatrix3x2dv::load_with(|s| loadfn(s));
    UniformMatrix3x2fv::load_with(|s| loadfn(s));
    UniformMatrix3x4dv::load_with(|s| loadfn(s));
    UniformMatrix3x4fv::load_with(|s| loadfn(s));
    UniformMatrix4dv::load_with(|s| loadfn(s));
    UniformMatrix4fv::load_with(|s| loadfn(s));
    UniformMatrix4x2dv::load_with(|s| loadfn(s));
    UniformMatrix4x2fv::load_with(|s| loadfn(s));
    UniformMatrix4x3dv::load_with(|s| loadfn(s));
    UniformMatrix4x3fv::load_with(|s| loadfn(s));
    UniformSubroutinesuiv::load_with(|s| loadfn(s));
    UnmapBuffer::load_with(|s| loadfn(s));
    UseProgram::load_with(|s| loadfn(s));
    UseProgramStages::load_with(|s| loadfn(s));
    ValidateProgram::load_with(|s| loadfn(s));
    ValidateProgramPipeline::load_with(|s| loadfn(s));
    VertexAttrib1d::load_with(|s| loadfn(s));
    VertexAttrib1dv::load_with(|s| loadfn(s));
    VertexAttrib1f::load_with(|s| loadfn(s));
    VertexAttrib1fv::load_with(|s| loadfn(s));
    VertexAttrib1s::load_with(|s| loadfn(s));
    VertexAttrib1sv::load_with(|s| loadfn(s));
    VertexAttrib2d::load_with(|s| loadfn(s));
    VertexAttrib2dv::load_with(|s| loadfn(s));
    VertexAttrib2f::load_with(|s| loadfn(s));
    VertexAttrib2fv::load_with(|s| loadfn(s));
    VertexAttrib2s::load_with(|s| loadfn(s));
    VertexAttrib2sv::load_with(|s| loadfn(s));
    VertexAttrib3d::load_with(|s| loadfn(s));
    VertexAttrib3dv::load_with(|s| loadfn(s));
    VertexAttrib3f::load_with(|s| loadfn(s));
    VertexAttrib3fv::load_with(|s| loadfn(s));
    VertexAttrib3s::load_with(|s| loadfn(s));
    VertexAttrib3sv::load_with(|s| loadfn(s));
    VertexAttrib4Nbv::load_with(|s| loadfn(s));
    VertexAttrib4Niv::load_with(|s| loadfn(s));
    VertexAttrib4Nsv::load_with(|s| loadfn(s));
    VertexAttrib4Nub::load_with(|s| loadfn(s));
    VertexAttrib4Nubv::load_with(|s| loadfn(s));
    VertexAttrib4Nuiv::load_with(|s| loadfn(s));
    VertexAttrib4Nusv::load_with(|s| loadfn(s));
    VertexAttrib4bv::load_with(|s| loadfn(s));
    VertexAttrib4d::load_with(|s| loadfn(s));
    VertexAttrib4dv::load_with(|s| loadfn(s));
    VertexAttrib4f::load_with(|s| loadfn(s));
    VertexAttrib4fv::load_with(|s| loadfn(s));
    VertexAttrib4iv::load_with(|s| loadfn(s));
    VertexAttrib4s::load_with(|s| loadfn(s));
    VertexAttrib4sv::load_with(|s| loadfn(s));
    VertexAttrib4ubv::load_with(|s| loadfn(s));
    VertexAttrib4uiv::load_with(|s| loadfn(s));
    VertexAttrib4usv::load_with(|s| loadfn(s));
    VertexAttribBinding::load_with(|s| loadfn(s));
    VertexAttribDivisor::load_with(|s| loadfn(s));
    VertexAttribFormat::load_with(|s| loadfn(s));
    VertexAttribI1i::load_with(|s| loadfn(s));
    VertexAttribI1iv::load_with(|s| loadfn(s));
    VertexAttribI1ui::load_with(|s| loadfn(s));
    VertexAttribI1uiv::load_with(|s| loadfn(s));
    VertexAttribI2i::load_with(|s| loadfn(s));
    VertexAttribI2iv::load_with(|s| loadfn(s));
    VertexAttribI2ui::load_with(|s| loadfn(s));
    VertexAttribI2uiv::load_with(|s| loadfn(s));
    VertexAttribI3i::load_with(|s| loadfn(s));
    VertexAttribI3iv::load_with(|s| loadfn(s));
    VertexAttribI3ui::load_with(|s| loadfn(s));
    VertexAttribI3uiv::load_with(|s| loadfn(s));
    VertexAttribI4bv::load_with(|s| loadfn(s));
    VertexAttribI4i::load_with(|s| loadfn(s));
    VertexAttribI4iv::load_with(|s| loadfn(s));
    VertexAttribI4sv::load_with(|s| loadfn(s));
    VertexAttribI4ubv::load_with(|s| loadfn(s));
    VertexAttribI4ui::load_with(|s| loadfn(s));
    VertexAttribI4uiv::load_with(|s| loadfn(s));
    VertexAttribI4usv::load_with(|s| loadfn(s));
    VertexAttribIFormat::load_with(|s| loadfn(s));
    VertexAttribIPointer::load_with(|s| loadfn(s));
    VertexAttribL1d::load_with(|s| loadfn(s));
    VertexAttribL1dv::load_with(|s| loadfn(s));
    VertexAttribL2d::load_with(|s| loadfn(s));
    VertexAttribL2dv::load_with(|s| loadfn(s));
    VertexAttribL3d::load_with(|s| loadfn(s));
    VertexAttribL3dv::load_with(|s| loadfn(s));
    VertexAttribL4d::load_with(|s| loadfn(s));
    VertexAttribL4dv::load_with(|s| loadfn(s));
    VertexAttribLFormat::load_with(|s| loadfn(s));
    VertexAttribLPointer::load_with(|s| loadfn(s));
    VertexAttribP1ui::load_with(|s| loadfn(s));
    VertexAttribP1uiv::load_with(|s| loadfn(s));
    VertexAttribP2ui::load_with(|s| loadfn(s));
    VertexAttribP2uiv::load_with(|s| loadfn(s));
    VertexAttribP3ui::load_with(|s| loadfn(s));
    VertexAttribP3uiv::load_with(|s| loadfn(s));
    VertexAttribP4ui::load_with(|s| loadfn(s));
    VertexAttribP4uiv::load_with(|s| loadfn(s));
    VertexAttribPointer::load_with(|s| loadfn(s));
    VertexBindingDivisor::load_with(|s| loadfn(s));
    VertexP2ui::load_with(|s| loadfn(s));
    VertexP2uiv::load_with(|s| loadfn(s));
    VertexP3ui::load_with(|s| loadfn(s));
    VertexP3uiv::load_with(|s| loadfn(s));
    VertexP4ui::load_with(|s| loadfn(s));
    VertexP4uiv::load_with(|s| loadfn(s));
    Viewport::load_with(|s| loadfn(s));
    ViewportArrayv::load_with(|s| loadfn(s));
    ViewportIndexedf::load_with(|s| loadfn(s));
    ViewportIndexedfv::load_with(|s| loadfn(s));
    WaitSync::load_with(|s| loadfn(s));
}

