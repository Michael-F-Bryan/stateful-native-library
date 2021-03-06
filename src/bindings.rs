// automatically generated by rust-bindgen

#![allow(bad_style, dead_code)]

pub const RESULT_OK: _bindgen_ty_1 = 0;
pub const RESULT_BAD_STATE: _bindgen_ty_1 = 1;
pub const RESULT_INVALID_ARGUMENT: _bindgen_ty_1 = 2;
pub type _bindgen_ty_1 = u32;
extern "C" {
    pub fn stateful_open() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_close() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_start_setting_parameters() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_set_bool_var(
        name: *const ::std::os::raw::c_char,
        value: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_set_int_var(
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_end_setting_parameters() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_start_adding_items() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_add_item(
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_start_adding_group(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_add_group_item(
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_end_adding_group() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_end_adding_items() -> ::std::os::raw::c_int;
}
pub type progress_cb = ::std::option::Option<
    unsafe extern "C" fn(
        percent: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type result_cb = ::std::option::Option<
    unsafe extern "C" fn(
        number_of_results: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn stateful_execute(
        progress: progress_cb,
        result: result_cb,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_get_num_outputs(
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stateful_get_output_by_index(
        index: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
