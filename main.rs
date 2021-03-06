use rusty_v8 as v8;

mod core_functions;
mod modules;

#[macro_use(lazy_static)]
extern crate lazy_static;

pub fn main() {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  let mut isolate = v8::Isolate::new(Default::default());
  isolate.set_host_import_module_dynamically_callback(modules::dynamic_import_cb);
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();

  let (object_templ, scope) = core_functions::init(scope);
  let context = v8::Context::new_from_template(scope, object_templ);
  let mut cs = v8::ContextScope::new(scope, context);
  let scope = cs.enter();

  modules::custom_module_loader::init_module_loader(scope, context);

  let file = get_bootstrap_file();
  let mut module = modules::compile::compile_file(scope, file.clone().as_ref()).unwrap();

  let _result = module.instantiate_module(context, modules::resolver);
  let _result = module.evaluate(scope, context);
}

fn get_bootstrap_file() -> std::string::String {
  match std::env::var("V8IO_BOOSTRAP") {
    Ok(val) => return val,
    Err(_) => {
      let file = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("bootstrap.js");
      return file.into_os_string().into_string().unwrap();
    }
  };
}
