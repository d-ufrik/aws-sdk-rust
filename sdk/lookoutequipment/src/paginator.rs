// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListDataIngestionJobs`](crate::operation::ListDataIngestionJobs)
pub struct ListDataIngestionJobsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_data_ingestion_jobs_input::Builder,
}

impl ListDataIngestionJobsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_data_ingestion_jobs_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListDataIngestionJobsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListDataIngestionJobsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_data_ingestion_jobs_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListDatasets`](crate::operation::ListDatasets)
pub struct ListDatasetsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_datasets_input::Builder,
}

impl ListDatasetsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_datasets_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListDatasetsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListDatasetsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_datasets_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListInferenceExecutions`](crate::operation::ListInferenceExecutions)
pub struct ListInferenceExecutionsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_inference_executions_input::Builder,
}

impl ListInferenceExecutionsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_inference_executions_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListInferenceExecutionsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListInferenceExecutionsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_inference_executions_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListInferenceSchedulers`](crate::operation::ListInferenceSchedulers)
pub struct ListInferenceSchedulersPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_inference_schedulers_input::Builder,
}

impl ListInferenceSchedulersPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_inference_schedulers_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListInferenceSchedulersOutput,
            aws_smithy_http::result::SdkError<crate::error::ListInferenceSchedulersError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_inference_schedulers_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListModels`](crate::operation::ListModels)
pub struct ListModelsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_models_input::Builder,
}

impl ListModelsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_models_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListModelsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListModelsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_models_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListSensorStatistics`](crate::operation::ListSensorStatistics)
pub struct ListSensorStatisticsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::input::list_sensor_statistics_input::Builder,
}

impl ListSensorStatisticsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::input::list_sensor_statistics_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListSensorStatisticsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListSensorStatisticsError>,
        >,
    > + Unpin {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            let new_token = crate::lens::reflens_structure_crate_output_list_sensor_statistics_output_next_token(resp);
                            let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                            if !is_empty && new_token == input.next_token.as_ref() {
                                let _ = tx.send(Err(aws_smithy_http::result::SdkError::ConstructionFailure("next token did not change, aborting paginator. This indicates an SDK or AWS service bug.".into()))).await;
                                return;
                            }
                            input.next_token = new_token.cloned();
                            is_empty
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}
