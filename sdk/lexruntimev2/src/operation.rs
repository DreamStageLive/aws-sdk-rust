// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Removes session information for a specified bot, alias, and user ID. </p>
/// <p>You can use this operation to restart a conversation with a bot.
/// When you remove a session, the entire history of the session is removed
/// so that you can start again.</p>
/// <p>You don't need to delete a session. Sessions have a time limit and
/// will expire. Set the session time limit when you create the bot. The
/// default is 5 minutes, but you can specify anything between 1 minute and
/// 24 hours.</p>
/// <p>If you specify a bot or alias ID that doesn't exist, you receive a
/// <code>BadRequestException.</code>
/// </p>
/// <p>If the locale doesn't exist in the bot, or if the locale hasn't been
/// enables for the alias, you receive a
/// <code>BadRequestException</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSession {
    _private: (),
}
impl DeleteSession {
    /// Creates a new builder-style object to manufacture [`DeleteSessionInput`](crate::input::DeleteSessionInput)
    pub fn builder() -> crate::input::delete_session_input::Builder {
        crate::input::delete_session_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSession {
    type Output =
        std::result::Result<crate::output::DeleteSessionOutput, crate::error::DeleteSessionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_session_error(response)
        } else {
            crate::operation_deser::parse_delete_session_response(response)
        }
    }
}

/// <p>Returns session information for a specified bot, alias, and
/// user.</p>
/// <p>For example, you can use this operation to retrieve session
/// information for a user that has left a long-running session in
/// use.</p>
/// <p>If the bot, alias, or session identifier doesn't exist, Amazon Lex V2
/// returns a <code>BadRequestException</code>. If the locale doesn't exist
/// or is not enabled for the alias, you receive a
/// <code>BadRequestException</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSession {
    _private: (),
}
impl GetSession {
    /// Creates a new builder-style object to manufacture [`GetSessionInput`](crate::input::GetSessionInput)
    pub fn builder() -> crate::input::get_session_input::Builder {
        crate::input::get_session_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetSession {
    type Output =
        std::result::Result<crate::output::GetSessionOutput, crate::error::GetSessionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_session_error(response)
        } else {
            crate::operation_deser::parse_get_session_response(response)
        }
    }
}

/// <p>Creates a new session or modifies an existing session with an Amazon Lex V2
/// bot. Use this operation to enable your application to set the state of
/// the bot.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutSession {
    _private: (),
}
impl PutSession {
    /// Creates a new builder-style object to manufacture [`PutSessionInput`](crate::input::PutSessionInput)
    pub fn builder() -> crate::input::put_session_input::Builder {
        crate::input::put_session_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for PutSession {
    type Output =
        std::result::Result<crate::output::PutSessionOutput, crate::error::PutSessionError>;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_put_session(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_put_session_error(response)
    }
}

/// <p>Sends user input to Amazon Lex V2. Client applications use this API to send
/// requests to Amazon Lex V2 at runtime. Amazon Lex V2 then interprets the user input
/// using the machine learning model that it build for the bot.</p>
/// <p>In response, Amazon Lex V2 returns the next message to convey to the user
/// and an optional response card to display.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RecognizeText {
    _private: (),
}
impl RecognizeText {
    /// Creates a new builder-style object to manufacture [`RecognizeTextInput`](crate::input::RecognizeTextInput)
    pub fn builder() -> crate::input::recognize_text_input::Builder {
        crate::input::recognize_text_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for RecognizeText {
    type Output =
        std::result::Result<crate::output::RecognizeTextOutput, crate::error::RecognizeTextError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_recognize_text_error(response)
        } else {
            crate::operation_deser::parse_recognize_text_response(response)
        }
    }
}

/// <p>Sends user input to Amazon Lex V2. You can send text or speech. Clients use
/// this API to send text and audio requests to Amazon Lex V2 at runtime. Amazon Lex V2
/// interprets the user input using the machine learning model built for
/// the bot.</p>
/// <p>The following request fields must be compressed with gzip and then
/// base64 encoded before you send them to Amazon Lex V2. </p>
/// <ul>
/// <li>
/// <p>requestAttributes</p>
/// </li>
/// <li>
/// <p>sessionState</p>
/// </li>
/// </ul>
/// <p>The following response fields are compressed using gzip and then
/// base64 encoded by Amazon Lex V2. Before you can use these fields, you must
/// decode and decompress them. </p>
/// <ul>
/// <li>
/// <p>inputTranscript</p>
/// </li>
/// <li>
/// <p>interpretations</p>
/// </li>
/// <li>
/// <p>messages</p>
/// </li>
/// <li>
/// <p>requestAttributes</p>
/// </li>
/// <li>
/// <p>sessionState</p>
/// </li>
/// </ul>
/// <p>The example contains a Java application that compresses and encodes
/// a Java object to send to Amazon Lex V2, and a second that decodes and
/// decompresses a response from Amazon Lex V2.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RecognizeUtterance {
    _private: (),
}
impl RecognizeUtterance {
    /// Creates a new builder-style object to manufacture [`RecognizeUtteranceInput`](crate::input::RecognizeUtteranceInput)
    pub fn builder() -> crate::input::recognize_utterance_input::Builder {
        crate::input::recognize_utterance_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for RecognizeUtterance {
    type Output = std::result::Result<
        crate::output::RecognizeUtteranceOutput,
        crate::error::RecognizeUtteranceError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_recognize_utterance(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_recognize_utterance_error(response)
    }
}
