let name = variable::get("crate_name");
variable::set("camel_name", name.to_upper_camel_case());
