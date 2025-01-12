// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_upgrade_target(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::UpgradeTarget, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::UpgradeTarget::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Engine") /* Engine com.amazonaws.rds#UpgradeTarget$Engine */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine(var_1);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.rds#UpgradeTarget$EngineVersion */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_2);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.rds#UpgradeTarget$Description */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_3);
            }
            ,
            s if s.matches("AutoUpgrade") /* AutoUpgrade com.amazonaws.rds#UpgradeTarget$AutoUpgrade */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_auto_upgrade(var_4);
            }
            ,
            s if s.matches("IsMajorVersionUpgrade") /* IsMajorVersionUpgrade com.amazonaws.rds#UpgradeTarget$IsMajorVersionUpgrade */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_major_version_upgrade(var_5);
            }
            ,
            s if s.matches("SupportedEngineModes") /* SupportedEngineModes com.amazonaws.rds#UpgradeTarget$SupportedEngineModes */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_engine_mode_list::de_engine_mode_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_supported_engine_modes(var_6);
            }
            ,
            s if s.matches("SupportsParallelQuery") /* SupportsParallelQuery com.amazonaws.rds#UpgradeTarget$SupportsParallelQuery */ =>  {
                let var_7 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_supports_parallel_query(var_7);
            }
            ,
            s if s.matches("SupportsGlobalDatabases") /* SupportsGlobalDatabases com.amazonaws.rds#UpgradeTarget$SupportsGlobalDatabases */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_supports_global_databases(var_8);
            }
            ,
            s if s.matches("SupportsBabelfish") /* SupportsBabelfish com.amazonaws.rds#UpgradeTarget$SupportsBabelfish */ =>  {
                let var_9 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_supports_babelfish(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
