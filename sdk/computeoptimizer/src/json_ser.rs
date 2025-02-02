// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_describe_recommendation_export_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRecommendationExportJobsInput,
) {
    if let Some(var_1) = &input.job_ids {
        let mut array_2 = object.key("jobIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.filters {
        let mut array_5 = object.key("filters").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_job_filter(&mut object_7, item_6);
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.next_token {
        object.key("nextToken").string(var_8);
    }
    if let Some(var_9) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_9).into()),
        );
    }
}

pub fn serialize_structure_export_auto_scaling_group_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportAutoScalingGroupRecommendationsInput,
) {
    if let Some(var_10) = &input.account_ids {
        let mut array_11 = object.key("accountIds").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12);
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.filters {
        let mut array_14 = object.key("filters").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_16, item_15);
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.fields_to_export {
        let mut array_18 = object.key("fieldsToExport").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19.as_str());
            }
        }
        array_18.finish();
    }
    if let Some(var_20) = &input.s3_destination_config {
        let mut object_21 = object.key("s3DestinationConfig").start_object();
        crate::json_ser::serialize_structure_s3_destination_config(&mut object_21, var_20);
        object_21.finish();
    }
    if let Some(var_22) = &input.file_format {
        object.key("fileFormat").string(var_22.as_str());
    }
    if input.include_member_accounts {
        object
            .key("includeMemberAccounts")
            .boolean(input.include_member_accounts);
    }
}

pub fn serialize_structure_export_ebs_volume_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportEbsVolumeRecommendationsInput,
) {
    if let Some(var_23) = &input.account_ids {
        let mut array_24 = object.key("accountIds").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25);
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.filters {
        let mut array_27 = object.key("filters").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_ebs_filter(&mut object_29, item_28);
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.fields_to_export {
        let mut array_31 = object.key("fieldsToExport").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    if let Some(var_33) = &input.s3_destination_config {
        let mut object_34 = object.key("s3DestinationConfig").start_object();
        crate::json_ser::serialize_structure_s3_destination_config(&mut object_34, var_33);
        object_34.finish();
    }
    if let Some(var_35) = &input.file_format {
        object.key("fileFormat").string(var_35.as_str());
    }
    if input.include_member_accounts {
        object
            .key("includeMemberAccounts")
            .boolean(input.include_member_accounts);
    }
}

pub fn serialize_structure_export_ec2_instance_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportEc2InstanceRecommendationsInput,
) {
    if let Some(var_36) = &input.account_ids {
        let mut array_37 = object.key("accountIds").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.filters {
        let mut array_40 = object.key("filters").start_array();
        for item_41 in var_39 {
            {
                let mut object_42 = array_40.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_42, item_41);
                object_42.finish();
            }
        }
        array_40.finish();
    }
    if let Some(var_43) = &input.fields_to_export {
        let mut array_44 = object.key("fieldsToExport").start_array();
        for item_45 in var_43 {
            {
                array_44.value().string(item_45.as_str());
            }
        }
        array_44.finish();
    }
    if let Some(var_46) = &input.s3_destination_config {
        let mut object_47 = object.key("s3DestinationConfig").start_object();
        crate::json_ser::serialize_structure_s3_destination_config(&mut object_47, var_46);
        object_47.finish();
    }
    if let Some(var_48) = &input.file_format {
        object.key("fileFormat").string(var_48.as_str());
    }
    if input.include_member_accounts {
        object
            .key("includeMemberAccounts")
            .boolean(input.include_member_accounts);
    }
}

pub fn serialize_structure_export_lambda_function_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportLambdaFunctionRecommendationsInput,
) {
    if let Some(var_49) = &input.account_ids {
        let mut array_50 = object.key("accountIds").start_array();
        for item_51 in var_49 {
            {
                array_50.value().string(item_51);
            }
        }
        array_50.finish();
    }
    if let Some(var_52) = &input.filters {
        let mut array_53 = object.key("filters").start_array();
        for item_54 in var_52 {
            {
                let mut object_55 = array_53.value().start_object();
                crate::json_ser::serialize_structure_lambda_function_recommendation_filter(
                    &mut object_55,
                    item_54,
                );
                object_55.finish();
            }
        }
        array_53.finish();
    }
    if let Some(var_56) = &input.fields_to_export {
        let mut array_57 = object.key("fieldsToExport").start_array();
        for item_58 in var_56 {
            {
                array_57.value().string(item_58.as_str());
            }
        }
        array_57.finish();
    }
    if let Some(var_59) = &input.s3_destination_config {
        let mut object_60 = object.key("s3DestinationConfig").start_object();
        crate::json_ser::serialize_structure_s3_destination_config(&mut object_60, var_59);
        object_60.finish();
    }
    if let Some(var_61) = &input.file_format {
        object.key("fileFormat").string(var_61.as_str());
    }
    if input.include_member_accounts {
        object
            .key("includeMemberAccounts")
            .boolean(input.include_member_accounts);
    }
}

pub fn serialize_structure_get_auto_scaling_group_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAutoScalingGroupRecommendationsInput,
) {
    if let Some(var_62) = &input.account_ids {
        let mut array_63 = object.key("accountIds").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.auto_scaling_group_arns {
        let mut array_66 = object.key("autoScalingGroupArns").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67);
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.next_token {
        object.key("nextToken").string(var_68);
    }
    if let Some(var_69) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_69).into()),
        );
    }
    if let Some(var_70) = &input.filters {
        let mut array_71 = object.key("filters").start_array();
        for item_72 in var_70 {
            {
                let mut object_73 = array_71.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_73, item_72);
                object_73.finish();
            }
        }
        array_71.finish();
    }
}

pub fn serialize_structure_get_ebs_volume_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEbsVolumeRecommendationsInput,
) {
    if let Some(var_74) = &input.volume_arns {
        let mut array_75 = object.key("volumeArns").start_array();
        for item_76 in var_74 {
            {
                array_75.value().string(item_76);
            }
        }
        array_75.finish();
    }
    if let Some(var_77) = &input.next_token {
        object.key("nextToken").string(var_77);
    }
    if let Some(var_78) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_78).into()),
        );
    }
    if let Some(var_79) = &input.filters {
        let mut array_80 = object.key("filters").start_array();
        for item_81 in var_79 {
            {
                let mut object_82 = array_80.value().start_object();
                crate::json_ser::serialize_structure_ebs_filter(&mut object_82, item_81);
                object_82.finish();
            }
        }
        array_80.finish();
    }
    if let Some(var_83) = &input.account_ids {
        let mut array_84 = object.key("accountIds").start_array();
        for item_85 in var_83 {
            {
                array_84.value().string(item_85);
            }
        }
        array_84.finish();
    }
}

pub fn serialize_structure_get_ec2_instance_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEc2InstanceRecommendationsInput,
) {
    if let Some(var_86) = &input.instance_arns {
        let mut array_87 = object.key("instanceArns").start_array();
        for item_88 in var_86 {
            {
                array_87.value().string(item_88);
            }
        }
        array_87.finish();
    }
    if let Some(var_89) = &input.next_token {
        object.key("nextToken").string(var_89);
    }
    if let Some(var_90) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    if let Some(var_91) = &input.filters {
        let mut array_92 = object.key("filters").start_array();
        for item_93 in var_91 {
            {
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_94, item_93);
                object_94.finish();
            }
        }
        array_92.finish();
    }
    if let Some(var_95) = &input.account_ids {
        let mut array_96 = object.key("accountIds").start_array();
        for item_97 in var_95 {
            {
                array_96.value().string(item_97);
            }
        }
        array_96.finish();
    }
}

pub fn serialize_structure_get_ec2_recommendation_projected_metrics_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEc2RecommendationProjectedMetricsInput,
) {
    if let Some(var_98) = &input.instance_arn {
        object.key("instanceArn").string(var_98);
    }
    if let Some(var_99) = &input.stat {
        object.key("stat").string(var_99.as_str());
    }
    {
        object.key("period").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.period).into()),
        );
    }
    if let Some(var_100) = &input.start_time {
        object
            .key("startTime")
            .instant(var_100, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_101) = &input.end_time {
        object
            .key("endTime")
            .instant(var_101, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_get_lambda_function_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLambdaFunctionRecommendationsInput,
) {
    if let Some(var_102) = &input.function_arns {
        let mut array_103 = object.key("functionArns").start_array();
        for item_104 in var_102 {
            {
                array_103.value().string(item_104);
            }
        }
        array_103.finish();
    }
    if let Some(var_105) = &input.account_ids {
        let mut array_106 = object.key("accountIds").start_array();
        for item_107 in var_105 {
            {
                array_106.value().string(item_107);
            }
        }
        array_106.finish();
    }
    if let Some(var_108) = &input.filters {
        let mut array_109 = object.key("filters").start_array();
        for item_110 in var_108 {
            {
                let mut object_111 = array_109.value().start_object();
                crate::json_ser::serialize_structure_lambda_function_recommendation_filter(
                    &mut object_111,
                    item_110,
                );
                object_111.finish();
            }
        }
        array_109.finish();
    }
    if let Some(var_112) = &input.next_token {
        object.key("nextToken").string(var_112);
    }
    if let Some(var_113) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_113).into()),
        );
    }
}

pub fn serialize_structure_get_recommendation_summaries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecommendationSummariesInput,
) {
    if let Some(var_114) = &input.account_ids {
        let mut array_115 = object.key("accountIds").start_array();
        for item_116 in var_114 {
            {
                array_115.value().string(item_116);
            }
        }
        array_115.finish();
    }
    if let Some(var_117) = &input.next_token {
        object.key("nextToken").string(var_117);
    }
    if let Some(var_118) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_118).into()),
        );
    }
}

pub fn serialize_structure_update_enrollment_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnrollmentStatusInput,
) {
    if let Some(var_119) = &input.status {
        object.key("status").string(var_119.as_str());
    }
    if input.include_member_accounts {
        object
            .key("includeMemberAccounts")
            .boolean(input.include_member_accounts);
    }
}

pub fn serialize_structure_job_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobFilter,
) {
    if let Some(var_120) = &input.name {
        object.key("name").string(var_120.as_str());
    }
    if let Some(var_121) = &input.values {
        let mut array_122 = object.key("values").start_array();
        for item_123 in var_121 {
            {
                array_122.value().string(item_123);
            }
        }
        array_122.finish();
    }
}

pub fn serialize_structure_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) {
    if let Some(var_124) = &input.name {
        object.key("name").string(var_124.as_str());
    }
    if let Some(var_125) = &input.values {
        let mut array_126 = object.key("values").start_array();
        for item_127 in var_125 {
            {
                array_126.value().string(item_127);
            }
        }
        array_126.finish();
    }
}

pub fn serialize_structure_s3_destination_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DestinationConfig,
) {
    if let Some(var_128) = &input.bucket {
        object.key("bucket").string(var_128);
    }
    if let Some(var_129) = &input.key_prefix {
        object.key("keyPrefix").string(var_129);
    }
}

pub fn serialize_structure_ebs_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EbsFilter,
) {
    if let Some(var_130) = &input.name {
        object.key("name").string(var_130.as_str());
    }
    if let Some(var_131) = &input.values {
        let mut array_132 = object.key("values").start_array();
        for item_133 in var_131 {
            {
                array_132.value().string(item_133);
            }
        }
        array_132.finish();
    }
}

pub fn serialize_structure_lambda_function_recommendation_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaFunctionRecommendationFilter,
) {
    if let Some(var_134) = &input.name {
        object.key("name").string(var_134.as_str());
    }
    if let Some(var_135) = &input.values {
        let mut array_136 = object.key("values").start_array();
        for item_137 in var_135 {
            {
                array_136.value().string(item_137);
            }
        }
        array_136.finish();
    }
}
