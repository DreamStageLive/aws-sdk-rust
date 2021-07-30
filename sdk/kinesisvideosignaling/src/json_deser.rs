// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response)
}

pub fn deser_structure_client_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::client_limit_exceeded_exception::Builder,
) -> Result<crate::error::client_limit_exceeded_exception::Builder, smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_argument_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_argument_exception::Builder,
) -> Result<crate::error::invalid_argument_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_client_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_client_exception::Builder,
) -> Result<crate::error::invalid_client_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_not_authorized_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::not_authorized_exception::Builder,
) -> Result<crate::error::not_authorized_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_resource_not_found_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::resource_not_found_exception::Builder,
) -> Result<crate::error::resource_not_found_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_session_expired_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::session_expired_exception::Builder,
) -> Result<crate::error::session_expired_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_get_ice_server_config(
    input: &[u8],
    mut builder: crate::output::get_ice_server_config_output::Builder,
) -> Result<crate::output::get_ice_server_config_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "IceServerList" => {
                        builder = builder.set_ice_server_list(
                            crate::json_deser::deser_list_ice_server_list(tokens)?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_send_alexa_offer_to_master(
    input: &[u8],
    mut builder: crate::output::send_alexa_offer_to_master_output::Builder,
) -> Result<
    crate::output::send_alexa_offer_to_master_output::Builder,
    smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Answer" => {
                        builder = builder.set_answer(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_ice_server_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::IceServer>>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value = crate::json_deser::deser_structure_ice_server(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_ice_server<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::IceServer>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::IceServer::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Uris" => {
                                builder =
                                    builder.set_uris(crate::json_deser::deser_list_uris(tokens)?);
                            }
                            "Username" => {
                                builder = builder.set_username(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Password" => {
                                builder = builder.set_password(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Ttl" => {
                                builder = builder.set_ttl(
                                    smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_uris<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<std::string::String>>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}