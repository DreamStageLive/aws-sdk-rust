// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_workspace_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkspaceInput,
) {
    if let Some(var_1) = &input.alias {
        object.key("alias").string(var_1);
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2);
    }
}

pub fn serialize_structure_update_workspace_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkspaceAliasInput,
) {
    if let Some(var_3) = &input.alias {
        object.key("alias").string(var_3);
    }
    if let Some(var_4) = &input.client_token {
        object.key("clientToken").string(var_4);
    }
}
