// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_source_build_information(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::SourceBuildInformation,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SourceType");
    if let Some(var_2) = &input.source_type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SourceRepository");
    if let Some(var_4) = &input.source_repository {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SourceLocation");
    if let Some(var_6) = &input.source_location {
        scope_5.string(var_6);
    }
    Ok(())
}

pub fn de_source_build_information(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SourceBuildInformation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SourceBuildInformation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SourceType") /* SourceType com.amazonaws.elasticbeanstalk#SourceBuildInformation$SourceType */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::SourceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::SourceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_source_type(var_7);
            }
            ,
            s if s.matches("SourceRepository") /* SourceRepository com.amazonaws.elasticbeanstalk#SourceBuildInformation$SourceRepository */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::SourceRepository, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::SourceRepository::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_source_repository(var_8);
            }
            ,
            s if s.matches("SourceLocation") /* SourceLocation com.amazonaws.elasticbeanstalk#SourceBuildInformation$SourceLocation */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_location(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
