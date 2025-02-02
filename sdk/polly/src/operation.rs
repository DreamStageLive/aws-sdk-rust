// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Deletes the specified pronunciation lexicon stored in an AWS Region. A lexicon which
/// has been deleted is not available for speech synthesis, nor is it possible to retrieve it
/// using either the <code>GetLexicon</code> or <code>ListLexicon</code> APIs.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing
/// Lexicons</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLexicon {
    _private: (),
}
impl DeleteLexicon {
    /// Creates a new builder-style object to manufacture [`DeleteLexiconInput`](crate::input::DeleteLexiconInput)
    pub fn builder() -> crate::input::delete_lexicon_input::Builder {
        crate::input::delete_lexicon_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteLexicon {
    type Output =
        std::result::Result<crate::output::DeleteLexiconOutput, crate::error::DeleteLexiconError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_lexicon_error(response)
        } else {
            crate::operation_deser::parse_delete_lexicon_response(response)
        }
    }
}

/// <p>Returns the list of voices that are available for use when requesting speech synthesis.
/// Each voice speaks a specified language, is either male or female, and is identified by an ID,
/// which is the ASCII version of the voice name. </p>
/// <p>When synthesizing speech ( <code>SynthesizeSpeech</code> ), you provide the voice ID
/// for the voice you want from the list of voices returned by
/// <code>DescribeVoices</code>.</p>
/// <p>For example, you want your news reader application to read news in a specific language,
/// but giving a user the option to choose the voice. Using the <code>DescribeVoices</code>
/// operation you can provide the user with a list of available voices to select from.</p>
/// <p> You can optionally specify a language code to filter the available voices. For
/// example, if you specify <code>en-US</code>, the operation returns a list of all available US
/// English voices. </p>
/// <p>This operation requires permissions to perform the <code>polly:DescribeVoices</code>
/// action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeVoices {
    _private: (),
}
impl DescribeVoices {
    /// Creates a new builder-style object to manufacture [`DescribeVoicesInput`](crate::input::DescribeVoicesInput)
    pub fn builder() -> crate::input::describe_voices_input::Builder {
        crate::input::describe_voices_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeVoices {
    type Output =
        std::result::Result<crate::output::DescribeVoicesOutput, crate::error::DescribeVoicesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_voices_error(response)
        } else {
            crate::operation_deser::parse_describe_voices_response(response)
        }
    }
}

/// <p>Returns the content of the specified pronunciation lexicon stored in an AWS Region. For
/// more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing
/// Lexicons</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLexicon {
    _private: (),
}
impl GetLexicon {
    /// Creates a new builder-style object to manufacture [`GetLexiconInput`](crate::input::GetLexiconInput)
    pub fn builder() -> crate::input::get_lexicon_input::Builder {
        crate::input::get_lexicon_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetLexicon {
    type Output =
        std::result::Result<crate::output::GetLexiconOutput, crate::error::GetLexiconError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lexicon_error(response)
        } else {
            crate::operation_deser::parse_get_lexicon_response(response)
        }
    }
}

/// <p>Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains
/// information about the given speech synthesis task, including the status of the task, and a
/// link to the S3 bucket containing the output of the task.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSpeechSynthesisTask {
    _private: (),
}
impl GetSpeechSynthesisTask {
    /// Creates a new builder-style object to manufacture [`GetSpeechSynthesisTaskInput`](crate::input::GetSpeechSynthesisTaskInput)
    pub fn builder() -> crate::input::get_speech_synthesis_task_input::Builder {
        crate::input::get_speech_synthesis_task_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetSpeechSynthesisTask {
    type Output = std::result::Result<
        crate::output::GetSpeechSynthesisTaskOutput,
        crate::error::GetSpeechSynthesisTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_speech_synthesis_task_error(response)
        } else {
            crate::operation_deser::parse_get_speech_synthesis_task_response(response)
        }
    }
}

/// <p>Returns a list of pronunciation lexicons stored in an AWS Region. For more information,
/// see <a href="https://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing
/// Lexicons</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLexicons {
    _private: (),
}
impl ListLexicons {
    /// Creates a new builder-style object to manufacture [`ListLexiconsInput`](crate::input::ListLexiconsInput)
    pub fn builder() -> crate::input::list_lexicons_input::Builder {
        crate::input::list_lexicons_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLexicons {
    type Output =
        std::result::Result<crate::output::ListLexiconsOutput, crate::error::ListLexiconsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_lexicons_error(response)
        } else {
            crate::operation_deser::parse_list_lexicons_response(response)
        }
    }
}

/// <p>Returns a list of SpeechSynthesisTask objects ordered by their creation date. This
/// operation can filter the tasks by their status, for example, allowing users to list only tasks
/// that are completed.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSpeechSynthesisTasks {
    _private: (),
}
impl ListSpeechSynthesisTasks {
    /// Creates a new builder-style object to manufacture [`ListSpeechSynthesisTasksInput`](crate::input::ListSpeechSynthesisTasksInput)
    pub fn builder() -> crate::input::list_speech_synthesis_tasks_input::Builder {
        crate::input::list_speech_synthesis_tasks_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSpeechSynthesisTasks {
    type Output = std::result::Result<
        crate::output::ListSpeechSynthesisTasksOutput,
        crate::error::ListSpeechSynthesisTasksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_speech_synthesis_tasks_error(response)
        } else {
            crate::operation_deser::parse_list_speech_synthesis_tasks_response(response)
        }
    }
}

/// <p>Stores a pronunciation lexicon in an AWS Region. If a lexicon with the same name
/// already exists in the region, it is overwritten by the new lexicon. Lexicon operations have
/// eventual consistency, therefore, it might take some time before the lexicon is available to
/// the SynthesizeSpeech operation.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing
/// Lexicons</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutLexicon {
    _private: (),
}
impl PutLexicon {
    /// Creates a new builder-style object to manufacture [`PutLexiconInput`](crate::input::PutLexiconInput)
    pub fn builder() -> crate::input::put_lexicon_input::Builder {
        crate::input::put_lexicon_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutLexicon {
    type Output =
        std::result::Result<crate::output::PutLexiconOutput, crate::error::PutLexiconError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_lexicon_error(response)
        } else {
            crate::operation_deser::parse_put_lexicon_response(response)
        }
    }
}

/// <p>Allows the creation of an asynchronous synthesis task, by starting a new
/// <code>SpeechSynthesisTask</code>. This operation requires all the standard information
/// needed for speech synthesis, plus the name of an Amazon S3 bucket for the service to store the
/// output of the synthesis task and two optional parameters (OutputS3KeyPrefix and SnsTopicArn).
/// Once the synthesis task is created, this operation will return a SpeechSynthesisTask object,
/// which will include an identifier of this task as well as the current status.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartSpeechSynthesisTask {
    _private: (),
}
impl StartSpeechSynthesisTask {
    /// Creates a new builder-style object to manufacture [`StartSpeechSynthesisTaskInput`](crate::input::StartSpeechSynthesisTaskInput)
    pub fn builder() -> crate::input::start_speech_synthesis_task_input::Builder {
        crate::input::start_speech_synthesis_task_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartSpeechSynthesisTask {
    type Output = std::result::Result<
        crate::output::StartSpeechSynthesisTaskOutput,
        crate::error::StartSpeechSynthesisTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_speech_synthesis_task_error(response)
        } else {
            crate::operation_deser::parse_start_speech_synthesis_task_response(response)
        }
    }
}

/// <p>Synthesizes UTF-8 input, plain text or SSML, to a stream of bytes. SSML input must be
/// valid, well-formed SSML. Some alphabets might not be available with all the voices (for
/// example, Cyrillic might not be read at all by English voices) unless phoneme mapping is used.
/// For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/how-text-to-speech-works.html">How it
/// Works</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SynthesizeSpeech {
    _private: (),
}
impl SynthesizeSpeech {
    /// Creates a new builder-style object to manufacture [`SynthesizeSpeechInput`](crate::input::SynthesizeSpeechInput)
    pub fn builder() -> crate::input::synthesize_speech_input::Builder {
        crate::input::synthesize_speech_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for SynthesizeSpeech {
    type Output = std::result::Result<
        crate::output::SynthesizeSpeechOutput,
        crate::error::SynthesizeSpeechError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_synthesize_speech(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_synthesize_speech_error(response)
    }
}
