use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;
use std::path::Path;


const GO_LSP: &str = "gopls";
const PLUGIN_PERFIX: &str = "[lapce-go-plugins]";

fn get_go_path() -> String {
	let output = Command::new("go")
		.args(&["env", "GOPATH"])
		.output()
		.expect("failed to execute process");
	let out = String::from_utf8(output.stdout).unwrap();
	
	return out;
}

fn install_gopls() {
	Command::new("go")
		.args(&["env", "-w", "GOPROXY=https://goproxy.io,direct"])
		.output()
		.expect("failed to set GOPROXY");
	Command::new("go")
		.args(&["get", "-u", "golang.org/x/tools/gopls"])
		.output()
		.expect("failed to install gopls");
}

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
	arch: String,
	os: String,
	configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
	language_id: String,
	system_lsp: bool,
	options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
	fn initialize(&mut self, info: serde_json::Value) {
		eprintln!("{} starting plugin", PLUGIN_PERFIX);
		let info = serde_json::from_value::<PluginInfo>(info).unwrap();
		
		let exec_path = if !info.configuration.system_lsp {
			let go_path = get_go_path();
			let exec_suffix = if info.os == "windows" {
				".exe"
			} else {
				""
			};
			let path = Path::new(&go_path);
			let dir = path.join("bin");
			let files = dir.as_path().read_dir().unwrap();
			let mut is_exist = false;
			for x in files {
				if let Ok(path) = x {
					// 是否存在某个文件
					if path.file_name().eq(format!("{}{}", GO_LSP, exec_suffix).as_str()) {
						eprintln!("{} found gopls", PLUGIN_PERFIX);
						is_exist = true;
						break;
					}
				}
			}
			if is_exist {
				format!("{}/bin/{}{}", go_path, GO_LSP, exec_suffix)
			} else {
				install_gopls();
				format!("{}/bin/{}{}", go_path, GO_LSP, exec_suffix)
			}
		} else {
			GO_LSP.to_string()
		};
		eprintln!("{} exec path: {}", PLUGIN_PERFIX, exec_path);
		start_lsp(
			&exec_path,
			info.configuration.language_id.as_str(),
			info.configuration.options,
			info.configuration.system_lsp,
		)
	}
}
