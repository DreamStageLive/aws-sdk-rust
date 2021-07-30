// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_accept_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptEnvironmentAccountConnectionInput,
) {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1);
    }
}

pub fn serialize_structure_cancel_environment_deployment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelEnvironmentDeploymentInput,
) {
    if let Some(var_2) = &input.environment_name {
        object.key("environmentName").string(var_2);
    }
}

pub fn serialize_structure_cancel_service_instance_deployment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelServiceInstanceDeploymentInput,
) {
    if let Some(var_3) = &input.service_instance_name {
        object.key("serviceInstanceName").string(var_3);
    }
    if let Some(var_4) = &input.service_name {
        object.key("serviceName").string(var_4);
    }
}

pub fn serialize_structure_cancel_service_pipeline_deployment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelServicePipelineDeploymentInput,
) {
    if let Some(var_5) = &input.service_name {
        object.key("serviceName").string(var_5);
    }
}

pub fn serialize_structure_create_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) {
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6);
    }
    if let Some(var_7) = &input.template_name {
        object.key("templateName").string(var_7);
    }
    if let Some(var_8) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_8);
    }
    if let Some(var_9) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_9);
    }
    if let Some(var_10) = &input.description {
        object.key("description").string(var_10);
    }
    if let Some(var_11) = &input.spec {
        object.key("spec").string(var_11);
    }
    if let Some(var_12) = &input.proton_service_role_arn {
        object.key("protonServiceRoleArn").string(var_12);
    }
    if let Some(var_13) = &input.environment_account_connection_id {
        object.key("environmentAccountConnectionId").string(var_13);
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("tags").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_17, item_16);
                object_17.finish();
            }
        }
        array_15.finish();
    }
}

pub fn serialize_structure_create_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentAccountConnectionInput,
) {
    if let Some(var_18) = &input.client_token {
        object.key("clientToken").string(var_18);
    }
    if let Some(var_19) = &input.management_account_id {
        object.key("managementAccountId").string(var_19);
    }
    if let Some(var_20) = &input.role_arn {
        object.key("roleArn").string(var_20);
    }
    if let Some(var_21) = &input.environment_name {
        object.key("environmentName").string(var_21);
    }
}

pub fn serialize_structure_create_environment_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentTemplateInput,
) {
    if let Some(var_22) = &input.name {
        object.key("name").string(var_22);
    }
    if let Some(var_23) = &input.display_name {
        object.key("displayName").string(var_23);
    }
    if let Some(var_24) = &input.description {
        object.key("description").string(var_24);
    }
    if let Some(var_25) = &input.encryption_key {
        object.key("encryptionKey").string(var_25);
    }
    if let Some(var_26) = &input.provisioning {
        object.key("provisioning").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("tags").start_array();
        for item_29 in var_27 {
            {
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_30, item_29);
                object_30.finish();
            }
        }
        array_28.finish();
    }
}

pub fn serialize_structure_create_environment_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentTemplateVersionInput,
) {
    if let Some(var_31) = &input.client_token {
        object.key("clientToken").string(var_31);
    }
    if let Some(var_32) = &input.template_name {
        object.key("templateName").string(var_32);
    }
    if let Some(var_33) = &input.description {
        object.key("description").string(var_33);
    }
    if let Some(var_34) = &input.major_version {
        object.key("majorVersion").string(var_34);
    }
    if let Some(var_35) = &input.source {
        let mut object_36 = object.key("source").start_object();
        crate::json_ser::serialize_union_template_version_source_input(&mut object_36, var_35);
        object_36.finish();
    }
    if let Some(var_37) = &input.tags {
        let mut array_38 = object.key("tags").start_array();
        for item_39 in var_37 {
            {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_40, item_39);
                object_40.finish();
            }
        }
        array_38.finish();
    }
}

pub fn serialize_structure_create_service_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceInput,
) {
    if let Some(var_41) = &input.name {
        object.key("name").string(var_41);
    }
    if let Some(var_42) = &input.description {
        object.key("description").string(var_42);
    }
    if let Some(var_43) = &input.template_name {
        object.key("templateName").string(var_43);
    }
    if let Some(var_44) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_44);
    }
    if let Some(var_45) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_45);
    }
    if let Some(var_46) = &input.spec {
        object.key("spec").string(var_46);
    }
    if let Some(var_47) = &input.repository_connection_arn {
        object.key("repositoryConnectionArn").string(var_47);
    }
    if let Some(var_48) = &input.repository_id {
        object.key("repositoryId").string(var_48);
    }
    if let Some(var_49) = &input.branch_name {
        object.key("branchName").string(var_49);
    }
    if let Some(var_50) = &input.tags {
        let mut array_51 = object.key("tags").start_array();
        for item_52 in var_50 {
            {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_53, item_52);
                object_53.finish();
            }
        }
        array_51.finish();
    }
}

pub fn serialize_structure_create_service_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceTemplateInput,
) {
    if let Some(var_54) = &input.name {
        object.key("name").string(var_54);
    }
    if let Some(var_55) = &input.display_name {
        object.key("displayName").string(var_55);
    }
    if let Some(var_56) = &input.description {
        object.key("description").string(var_56);
    }
    if let Some(var_57) = &input.encryption_key {
        object.key("encryptionKey").string(var_57);
    }
    if let Some(var_58) = &input.pipeline_provisioning {
        object.key("pipelineProvisioning").string(var_58.as_str());
    }
    if let Some(var_59) = &input.tags {
        let mut array_60 = object.key("tags").start_array();
        for item_61 in var_59 {
            {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_62, item_61);
                object_62.finish();
            }
        }
        array_60.finish();
    }
}

pub fn serialize_structure_create_service_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceTemplateVersionInput,
) {
    if let Some(var_63) = &input.client_token {
        object.key("clientToken").string(var_63);
    }
    if let Some(var_64) = &input.template_name {
        object.key("templateName").string(var_64);
    }
    if let Some(var_65) = &input.description {
        object.key("description").string(var_65);
    }
    if let Some(var_66) = &input.major_version {
        object.key("majorVersion").string(var_66);
    }
    if let Some(var_67) = &input.source {
        let mut object_68 = object.key("source").start_object();
        crate::json_ser::serialize_union_template_version_source_input(&mut object_68, var_67);
        object_68.finish();
    }
    if let Some(var_69) = &input.compatible_environment_templates {
        let mut array_70 = object.key("compatibleEnvironmentTemplates").start_array();
        for item_71 in var_69 {
            {
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_compatible_environment_template_input(
                    &mut object_72,
                    item_71,
                );
                object_72.finish();
            }
        }
        array_70.finish();
    }
    if let Some(var_73) = &input.tags {
        let mut array_74 = object.key("tags").start_array();
        for item_75 in var_73 {
            {
                let mut object_76 = array_74.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_76, item_75);
                object_76.finish();
            }
        }
        array_74.finish();
    }
}

pub fn serialize_structure_delete_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentInput,
) {
    if let Some(var_77) = &input.name {
        object.key("name").string(var_77);
    }
}

pub fn serialize_structure_delete_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentAccountConnectionInput,
) {
    if let Some(var_78) = &input.id {
        object.key("id").string(var_78);
    }
}

pub fn serialize_structure_delete_environment_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentTemplateInput,
) {
    if let Some(var_79) = &input.name {
        object.key("name").string(var_79);
    }
}

pub fn serialize_structure_delete_environment_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEnvironmentTemplateVersionInput,
) {
    if let Some(var_80) = &input.template_name {
        object.key("templateName").string(var_80);
    }
    if let Some(var_81) = &input.major_version {
        object.key("majorVersion").string(var_81);
    }
    if let Some(var_82) = &input.minor_version {
        object.key("minorVersion").string(var_82);
    }
}

pub fn serialize_structure_delete_service_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteServiceInput,
) {
    if let Some(var_83) = &input.name {
        object.key("name").string(var_83);
    }
}

pub fn serialize_structure_delete_service_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteServiceTemplateInput,
) {
    if let Some(var_84) = &input.name {
        object.key("name").string(var_84);
    }
}

pub fn serialize_structure_delete_service_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteServiceTemplateVersionInput,
) {
    if let Some(var_85) = &input.template_name {
        object.key("templateName").string(var_85);
    }
    if let Some(var_86) = &input.major_version {
        object.key("majorVersion").string(var_86);
    }
    if let Some(var_87) = &input.minor_version {
        object.key("minorVersion").string(var_87);
    }
}

pub fn serialize_structure_get_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEnvironmentInput,
) {
    if let Some(var_88) = &input.name {
        object.key("name").string(var_88);
    }
}

pub fn serialize_structure_get_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEnvironmentAccountConnectionInput,
) {
    if let Some(var_89) = &input.id {
        object.key("id").string(var_89);
    }
}

pub fn serialize_structure_get_environment_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEnvironmentTemplateInput,
) {
    if let Some(var_90) = &input.name {
        object.key("name").string(var_90);
    }
}

pub fn serialize_structure_get_environment_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEnvironmentTemplateVersionInput,
) {
    if let Some(var_91) = &input.template_name {
        object.key("templateName").string(var_91);
    }
    if let Some(var_92) = &input.major_version {
        object.key("majorVersion").string(var_92);
    }
    if let Some(var_93) = &input.minor_version {
        object.key("minorVersion").string(var_93);
    }
}

pub fn serialize_structure_get_service_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceInput,
) {
    if let Some(var_94) = &input.name {
        object.key("name").string(var_94);
    }
}

pub fn serialize_structure_get_service_instance_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceInstanceInput,
) {
    if let Some(var_95) = &input.name {
        object.key("name").string(var_95);
    }
    if let Some(var_96) = &input.service_name {
        object.key("serviceName").string(var_96);
    }
}

pub fn serialize_structure_get_service_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceTemplateInput,
) {
    if let Some(var_97) = &input.name {
        object.key("name").string(var_97);
    }
}

pub fn serialize_structure_get_service_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetServiceTemplateVersionInput,
) {
    if let Some(var_98) = &input.template_name {
        object.key("templateName").string(var_98);
    }
    if let Some(var_99) = &input.major_version {
        object.key("majorVersion").string(var_99);
    }
    if let Some(var_100) = &input.minor_version {
        object.key("minorVersion").string(var_100);
    }
}

pub fn serialize_structure_list_environment_account_connections_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEnvironmentAccountConnectionsInput,
) {
    if let Some(var_101) = &input.requested_by {
        object.key("requestedBy").string(var_101.as_str());
    }
    if let Some(var_102) = &input.environment_name {
        object.key("environmentName").string(var_102);
    }
    if let Some(var_103) = &input.statuses {
        let mut array_104 = object.key("statuses").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    if let Some(var_106) = &input.next_token {
        object.key("nextToken").string(var_106);
    }
    if let Some(var_107) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_107).into()),
        );
    }
}

pub fn serialize_structure_list_environments_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEnvironmentsInput,
) {
    if let Some(var_108) = &input.next_token {
        object.key("nextToken").string(var_108);
    }
    if let Some(var_109) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_109).into()),
        );
    }
    if let Some(var_110) = &input.environment_templates {
        let mut array_111 = object.key("environmentTemplates").start_array();
        for item_112 in var_110 {
            {
                let mut object_113 = array_111.value().start_object();
                crate::json_ser::serialize_structure_environment_template_filter(
                    &mut object_113,
                    item_112,
                );
                object_113.finish();
            }
        }
        array_111.finish();
    }
}

pub fn serialize_structure_list_environment_templates_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEnvironmentTemplatesInput,
) {
    if let Some(var_114) = &input.next_token {
        object.key("nextToken").string(var_114);
    }
    if let Some(var_115) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_115).into()),
        );
    }
}

pub fn serialize_structure_list_environment_template_versions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEnvironmentTemplateVersionsInput,
) {
    if let Some(var_116) = &input.next_token {
        object.key("nextToken").string(var_116);
    }
    if let Some(var_117) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    if let Some(var_118) = &input.template_name {
        object.key("templateName").string(var_118);
    }
    if let Some(var_119) = &input.major_version {
        object.key("majorVersion").string(var_119);
    }
}

pub fn serialize_structure_list_service_instances_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServiceInstancesInput,
) {
    if let Some(var_120) = &input.service_name {
        object.key("serviceName").string(var_120);
    }
    if let Some(var_121) = &input.next_token {
        object.key("nextToken").string(var_121);
    }
    if let Some(var_122) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_122).into()),
        );
    }
}

pub fn serialize_structure_list_services_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServicesInput,
) {
    if let Some(var_123) = &input.next_token {
        object.key("nextToken").string(var_123);
    }
    if let Some(var_124) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_124).into()),
        );
    }
}

pub fn serialize_structure_list_service_templates_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServiceTemplatesInput,
) {
    if let Some(var_125) = &input.next_token {
        object.key("nextToken").string(var_125);
    }
    if let Some(var_126) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_126).into()),
        );
    }
}

pub fn serialize_structure_list_service_template_versions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServiceTemplateVersionsInput,
) {
    if let Some(var_127) = &input.next_token {
        object.key("nextToken").string(var_127);
    }
    if let Some(var_128) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_128).into()),
        );
    }
    if let Some(var_129) = &input.template_name {
        object.key("templateName").string(var_129);
    }
    if let Some(var_130) = &input.major_version {
        object.key("majorVersion").string(var_130);
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_131) = &input.resource_arn {
        object.key("resourceArn").string(var_131);
    }
    if let Some(var_132) = &input.next_token {
        object.key("nextToken").string(var_132);
    }
    if let Some(var_133) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_133).into()),
        );
    }
}

pub fn serialize_structure_reject_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectEnvironmentAccountConnectionInput,
) {
    if let Some(var_134) = &input.id {
        object.key("id").string(var_134);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_135) = &input.resource_arn {
        object.key("resourceArn").string(var_135);
    }
    if let Some(var_136) = &input.tags {
        let mut array_137 = object.key("tags").start_array();
        for item_138 in var_136 {
            {
                let mut object_139 = array_137.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_139, item_138);
                object_139.finish();
            }
        }
        array_137.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_140) = &input.resource_arn {
        object.key("resourceArn").string(var_140);
    }
    if let Some(var_141) = &input.tag_keys {
        let mut array_142 = object.key("tagKeys").start_array();
        for item_143 in var_141 {
            {
                array_142.value().string(item_143);
            }
        }
        array_142.finish();
    }
}

pub fn serialize_structure_update_account_settings_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAccountSettingsInput,
) {
    if let Some(var_144) = &input.pipeline_service_role_arn {
        object.key("pipelineServiceRoleArn").string(var_144);
    }
}

pub fn serialize_structure_update_environment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentInput,
) {
    if let Some(var_145) = &input.name {
        object.key("name").string(var_145);
    }
    if let Some(var_146) = &input.description {
        object.key("description").string(var_146);
    }
    if let Some(var_147) = &input.spec {
        object.key("spec").string(var_147);
    }
    if let Some(var_148) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_148);
    }
    if let Some(var_149) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_149);
    }
    if let Some(var_150) = &input.proton_service_role_arn {
        object.key("protonServiceRoleArn").string(var_150);
    }
    if let Some(var_151) = &input.deployment_type {
        object.key("deploymentType").string(var_151.as_str());
    }
    if let Some(var_152) = &input.environment_account_connection_id {
        object.key("environmentAccountConnectionId").string(var_152);
    }
}

pub fn serialize_structure_update_environment_account_connection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentAccountConnectionInput,
) {
    if let Some(var_153) = &input.id {
        object.key("id").string(var_153);
    }
    if let Some(var_154) = &input.role_arn {
        object.key("roleArn").string(var_154);
    }
}

pub fn serialize_structure_update_environment_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentTemplateInput,
) {
    if let Some(var_155) = &input.name {
        object.key("name").string(var_155);
    }
    if let Some(var_156) = &input.display_name {
        object.key("displayName").string(var_156);
    }
    if let Some(var_157) = &input.description {
        object.key("description").string(var_157);
    }
}

pub fn serialize_structure_update_environment_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentTemplateVersionInput,
) {
    if let Some(var_158) = &input.template_name {
        object.key("templateName").string(var_158);
    }
    if let Some(var_159) = &input.major_version {
        object.key("majorVersion").string(var_159);
    }
    if let Some(var_160) = &input.minor_version {
        object.key("minorVersion").string(var_160);
    }
    if let Some(var_161) = &input.description {
        object.key("description").string(var_161);
    }
    if let Some(var_162) = &input.status {
        object.key("status").string(var_162.as_str());
    }
}

pub fn serialize_structure_update_service_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServiceInput,
) {
    if let Some(var_163) = &input.name {
        object.key("name").string(var_163);
    }
    if let Some(var_164) = &input.description {
        object.key("description").string(var_164);
    }
    if let Some(var_165) = &input.spec {
        object.key("spec").string(var_165);
    }
}

pub fn serialize_structure_update_service_instance_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServiceInstanceInput,
) {
    if let Some(var_166) = &input.name {
        object.key("name").string(var_166);
    }
    if let Some(var_167) = &input.service_name {
        object.key("serviceName").string(var_167);
    }
    if let Some(var_168) = &input.deployment_type {
        object.key("deploymentType").string(var_168.as_str());
    }
    if let Some(var_169) = &input.spec {
        object.key("spec").string(var_169);
    }
    if let Some(var_170) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_170);
    }
    if let Some(var_171) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_171);
    }
}

pub fn serialize_structure_update_service_pipeline_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServicePipelineInput,
) {
    if let Some(var_172) = &input.service_name {
        object.key("serviceName").string(var_172);
    }
    if let Some(var_173) = &input.spec {
        object.key("spec").string(var_173);
    }
    if let Some(var_174) = &input.deployment_type {
        object.key("deploymentType").string(var_174.as_str());
    }
    if let Some(var_175) = &input.template_major_version {
        object.key("templateMajorVersion").string(var_175);
    }
    if let Some(var_176) = &input.template_minor_version {
        object.key("templateMinorVersion").string(var_176);
    }
}

pub fn serialize_structure_update_service_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServiceTemplateInput,
) {
    if let Some(var_177) = &input.name {
        object.key("name").string(var_177);
    }
    if let Some(var_178) = &input.display_name {
        object.key("displayName").string(var_178);
    }
    if let Some(var_179) = &input.description {
        object.key("description").string(var_179);
    }
}

pub fn serialize_structure_update_service_template_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServiceTemplateVersionInput,
) {
    if let Some(var_180) = &input.template_name {
        object.key("templateName").string(var_180);
    }
    if let Some(var_181) = &input.major_version {
        object.key("majorVersion").string(var_181);
    }
    if let Some(var_182) = &input.minor_version {
        object.key("minorVersion").string(var_182);
    }
    if let Some(var_183) = &input.description {
        object.key("description").string(var_183);
    }
    if let Some(var_184) = &input.status {
        object.key("status").string(var_184.as_str());
    }
    if let Some(var_185) = &input.compatible_environment_templates {
        let mut array_186 = object.key("compatibleEnvironmentTemplates").start_array();
        for item_187 in var_185 {
            {
                let mut object_188 = array_186.value().start_object();
                crate::json_ser::serialize_structure_compatible_environment_template_input(
                    &mut object_188,
                    item_187,
                );
                object_188.finish();
            }
        }
        array_186.finish();
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_189) = &input.key {
        object.key("key").string(var_189);
    }
    if let Some(var_190) = &input.value {
        object.key("value").string(var_190);
    }
}

pub fn serialize_union_template_version_source_input(
    object_36: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TemplateVersionSourceInput,
) {
    match input {
        crate::model::TemplateVersionSourceInput::S3(inner) => {
            let mut object_191 = object_36.key("s3").start_object();
            crate::json_ser::serialize_structure_s3_object_source(&mut object_191, inner);
            object_191.finish();
        }
    }
}

pub fn serialize_structure_compatible_environment_template_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CompatibleEnvironmentTemplateInput,
) {
    if let Some(var_192) = &input.template_name {
        object.key("templateName").string(var_192);
    }
    if let Some(var_193) = &input.major_version {
        object.key("majorVersion").string(var_193);
    }
}

pub fn serialize_structure_environment_template_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EnvironmentTemplateFilter,
) {
    if let Some(var_194) = &input.template_name {
        object.key("templateName").string(var_194);
    }
    if let Some(var_195) = &input.major_version {
        object.key("majorVersion").string(var_195);
    }
}

pub fn serialize_structure_s3_object_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ObjectSource,
) {
    if let Some(var_196) = &input.bucket {
        object.key("bucket").string(var_196);
    }
    if let Some(var_197) = &input.key {
        object.key("key").string(var_197);
    }
}