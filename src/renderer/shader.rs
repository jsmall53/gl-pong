use glow::{ HasContext };
use std::collections::HashMap;
use std::rc::Rc;


pub struct GLShader {
    gl: Rc<glow::Context>,
    name: String,
    sources: HashMap<u32, String>,
    program: glow::NativeProgram,
}



impl GLShader {
    pub fn new(gl: Rc<glow::Context>, name: &str, vertex_src: &str, fragment_src: &str) -> Self {
        let mut sources = HashMap::new();
        sources.insert(glow::VERTEX_SHADER, vertex_src.into());
        sources.insert(glow::FRAGMENT_SHADER, fragment_src.into());

        unsafe {
            let program = init_program(&gl, vertex_src, fragment_src);
            Self {
                gl,
                name: name.into(),
                sources,
                program,
            }

        }
    }

    pub fn bind(&mut self) { 
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            self.gl.use_program(None);
        }
    }
}



unsafe fn init_program(gl: &glow::Context, vshader_src: &str, fshader_src: &str) -> glow::NativeProgram {
    let program = gl.create_program().expect("Failed to create gl program");

    let vertex_shader = init_shader(&gl, glow::VERTEX_SHADER, vshader_src);
    let frag_shader = init_shader(&gl, glow::FRAGMENT_SHADER, fshader_src);

    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, frag_shader);
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }
    gl.delete_shader(vertex_shader);
    gl.delete_shader(frag_shader);

    program
}



unsafe fn init_shader(gl: &glow::Context, shader_type: u32, shader_source: &str) -> glow::Shader {
    let shader: glow::Shader = gl.create_shader(shader_type)
        .expect(&format!("Failed to create shader: {}", shader_type));

    gl.shader_source(shader, shader_source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        panic!("{}\n{}", gl.get_shader_info_log(shader), shader_source);
    } else {
        shader
    }
}

pub const VERTEX_SRC: &str = "
#version 450 core

layout(location = 0) in vec3 a_Position;
layout(location = 1) in vec4 a_Color;
layout(location = 2) in vec2 a_TexCoord;
layout(location = 3) in float a_TexIndex;
layout(location = 4) in float a_TilingFactor;
layout(location = 5) in int a_EntityID;

layout(std140, binding = 0) uniform Camera
{
	mat4 u_ViewProjection;
};

struct VertexOutput
{
	vec4 Color;
	vec2 TexCoord;
	float TilingFactor;
};

layout (location = 0) out VertexOutput Output;
layout (location = 3) out flat float v_TexIndex;
layout (location = 4) out flat int v_EntityID;

void main()
{
	Output.Color = a_Color;
	Output.TexCoord = a_TexCoord;
	Output.TilingFactor = a_TilingFactor;
	v_TexIndex = a_TexIndex;
	v_EntityID = a_EntityID;

	gl_Position = u_ViewProjection * vec4(a_Position, 1.0);
}
\0";



pub const FRAGMENT_SRC: &str = "
#version 450 core

layout(location = 0) out vec4 o_Color;
layout(location = 1) out int o_EntityID;

struct VertexOutput
{
	vec4 Color;
	vec2 TexCoord;
	float TilingFactor;
};

layout (location = 0) in VertexOutput Input;
layout (location = 3) in flat float v_TexIndex;
layout (location = 4) in flat int v_EntityID;

// layout (binding = 0) uniform sampler2D u_Textures[32];

void main()
{
	vec4 texColor = Input.Color;

	// switch(int(v_TexIndex))
	// {
	// 	case  0: texColor *= texture(u_Textures[ 0], Input.TexCoord * Input.TilingFactor); break;
	// 	case  1: texColor *= texture(u_Textures[ 1], Input.TexCoord * Input.TilingFactor); break;
	// 	case  2: texColor *= texture(u_Textures[ 2], Input.TexCoord * Input.TilingFactor); break;
	// 	case  3: texColor *= texture(u_Textures[ 3], Input.TexCoord * Input.TilingFactor); break;
	// 	case  4: texColor *= texture(u_Textures[ 4], Input.TexCoord * Input.TilingFactor); break;
	// 	case  5: texColor *= texture(u_Textures[ 5], Input.TexCoord * Input.TilingFactor); break;
	// 	case  6: texColor *= texture(u_Textures[ 6], Input.TexCoord * Input.TilingFactor); break;
	// 	case  7: texColor *= texture(u_Textures[ 7], Input.TexCoord * Input.TilingFactor); break;
	// 	case  8: texColor *= texture(u_Textures[ 8], Input.TexCoord * Input.TilingFactor); break;
	// 	case  9: texColor *= texture(u_Textures[ 9], Input.TexCoord * Input.TilingFactor); break;
	// 	case 10: texColor *= texture(u_Textures[10], Input.TexCoord * Input.TilingFactor); break;
	// 	case 11: texColor *= texture(u_Textures[11], Input.TexCoord * Input.TilingFactor); break;
	// 	case 12: texColor *= texture(u_Textures[12], Input.TexCoord * Input.TilingFactor); break;
	// 	case 13: texColor *= texture(u_Textures[13], Input.TexCoord * Input.TilingFactor); break;
	// 	case 14: texColor *= texture(u_Textures[14], Input.TexCoord * Input.TilingFactor); break;
	// 	case 15: texColor *= texture(u_Textures[15], Input.TexCoord * Input.TilingFactor); break;
	// 	case 16: texColor *= texture(u_Textures[16], Input.TexCoord * Input.TilingFactor); break;
	// 	case 17: texColor *= texture(u_Textures[17], Input.TexCoord * Input.TilingFactor); break;
	// 	case 18: texColor *= texture(u_Textures[18], Input.TexCoord * Input.TilingFactor); break;
	// 	case 19: texColor *= texture(u_Textures[19], Input.TexCoord * Input.TilingFactor); break;
	// 	case 20: texColor *= texture(u_Textures[20], Input.TexCoord * Input.TilingFactor); break;
	// 	case 21: texColor *= texture(u_Textures[21], Input.TexCoord * Input.TilingFactor); break;
	// 	case 22: texColor *= texture(u_Textures[22], Input.TexCoord * Input.TilingFactor); break;
	// 	case 23: texColor *= texture(u_Textures[23], Input.TexCoord * Input.TilingFactor); break;
	// 	case 24: texColor *= texture(u_Textures[24], Input.TexCoord * Input.TilingFactor); break;
	// 	case 25: texColor *= texture(u_Textures[25], Input.TexCoord * Input.TilingFactor); break;
	// 	case 26: texColor *= texture(u_Textures[26], Input.TexCoord * Input.TilingFactor); break;
	// 	case 27: texColor *= texture(u_Textures[27], Input.TexCoord * Input.TilingFactor); break;
	// 	case 28: texColor *= texture(u_Textures[28], Input.TexCoord * Input.TilingFactor); break;
	// 	case 29: texColor *= texture(u_Textures[29], Input.TexCoord * Input.TilingFactor); break;
	// 	case 30: texColor *= texture(u_Textures[30], Input.TexCoord * Input.TilingFactor); break;
	// 	case 31: texColor *= texture(u_Textures[31], Input.TexCoord * Input.TilingFactor); break;
	// }

	if (texColor.a == 0.0)
		discard;

	o_Color = texColor;
	o_EntityID = v_EntityID;
}
\0";
