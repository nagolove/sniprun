//Interpreter:| Lua_original        | lua         |
//############|_____________________|_____________|________________<- delimiters to help formatting,
//############| Interpretername     | language    | comment
// Keep (but modify the first line after the :) if you wish to have this interpreter listed via SnipInfo
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Lua_original {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    lua_work_dir: String,
    main_file_path: String,
}
impl ReplLikeInterpreter for Lua_original {}
impl Interpreter for Lua_original {
    fn new_with_level(data: DataHolder, level: SupportLevel) -> Box<Lua_original> {
        let bwd = data.work_dir.clone() + "/lua-original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&bwd)
            .expect("Could not create directory for lua-original");
        let mfp = bwd.clone() + "/main.lua";
        Box::new(Lua_original {
            data,
            support_level: level,
            code: String::from(""),
            lua_work_dir: bwd,
            main_file_path: mfp,
        })
    }

    fn get_name() -> String {
        String::from("Lua_original")
    }

    fn get_supported_languages() -> Vec<String> {
        vec![String::from("lua")]
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        SupportLevel::Bloc
    }

    fn fallback(&mut self) -> Option<Result<String, SniprunError>> {
        //do not fallback if one's is explicitely selected
        if self.support_level == SupportLevel::Selected {
            return None;
        }
        self.fetch_code().expect("could not fetch code");
        if self.code.contains("nvim") || self.code.contains("vim") {
            //then this is not pure lua code but  lua-nvim one
            let mut good_interpreter = crate::interpreters::Lua_nvim::new_with_level(
                self.data.clone(),
                self.get_current_level(),
            );
            return Some(good_interpreter.run());
        }
        return None;
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.get_current_level() >= SupportLevel::Bloc
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.get_current_level() >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for lua-original");

        write(&self.main_file_path, &self.code).expect("Unable to write to file for lua-original");
        Ok(())
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        let output = Command::new("lua")
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");
        info!("yay from lua interpreter");
        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}

#[cfg(test)]
mod test_lua_original {
    use super::*;

    #[test]
    fn run_all() { 
        //nececssary to run sequentially 
        //because of file access & shared things
        simple_print();
    }
    fn simple_print() {
        let mut data = DataHolder::new();
        data.current_bloc = String::from("print(\"Hi\")");
        let mut interpreter = Lua_original::new(data);
        let res = interpreter.run();

        // should panic if not an Ok()
        let string_result = res.unwrap();
        assert_eq!(string_result, "Hi\n");
    }

}
