// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `Ebs`.
///
/// This client allows ergonomic access to a `Ebs`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn complete_snapshot(&self) -> fluent_builders::CompleteSnapshot<C, M, R> {
        fluent_builders::CompleteSnapshot::new(self.handle.clone())
    }
    pub fn get_snapshot_block(&self) -> fluent_builders::GetSnapshotBlock<C, M, R> {
        fluent_builders::GetSnapshotBlock::new(self.handle.clone())
    }
    pub fn list_changed_blocks(&self) -> fluent_builders::ListChangedBlocks<C, M, R> {
        fluent_builders::ListChangedBlocks::new(self.handle.clone())
    }
    pub fn list_snapshot_blocks(&self) -> fluent_builders::ListSnapshotBlocks<C, M, R> {
        fluent_builders::ListSnapshotBlocks::new(self.handle.clone())
    }
    pub fn put_snapshot_block(&self) -> fluent_builders::PutSnapshotBlock<C, M, R> {
        fluent_builders::PutSnapshotBlock::new(self.handle.clone())
    }
    pub fn start_snapshot(&self) -> fluent_builders::StartSnapshot<C, M, R> {
        fluent_builders::StartSnapshot::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CompleteSnapshot<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::complete_snapshot_input::Builder,
    }
    impl<C, M, R> CompleteSnapshot<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CompleteSnapshotOutput,
            smithy_http::result::SdkError<crate::error::CompleteSnapshotError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CompleteSnapshotInputOperationOutputAlias,
                crate::output::CompleteSnapshotOutput,
                crate::error::CompleteSnapshotError,
                crate::input::CompleteSnapshotInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the snapshot.</p>
        pub fn snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(inp);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
        /// <p>The number of blocks that were written to the snapshot.</p>
        pub fn changed_blocks_count(mut self, inp: i32) -> Self {
            self.inner = self.inner.changed_blocks_count(inp);
            self
        }
        pub fn set_changed_blocks_count(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_changed_blocks_count(input);
            self
        }
        /// <p>An aggregated Base-64 SHA256 checksum based on the checksums of each written
        /// block.</p>
        /// <p>To generate the aggregated checksum using the linear aggregation method, arrange the
        /// checksums for each written block in ascending order of their block index, concatenate
        /// them to form a single string, and then generate the checksum on the entire string using
        /// the SHA256 algorithm.</p>
        pub fn checksum(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.checksum(inp);
            self
        }
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_checksum(input);
            self
        }
        /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm
        /// is <code>SHA256</code>.</p>
        pub fn checksum_algorithm(mut self, inp: crate::model::ChecksumAlgorithm) -> Self {
            self.inner = self.inner.checksum_algorithm(inp);
            self
        }
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::ChecksumAlgorithm>,
        ) -> Self {
            self.inner = self.inner.set_checksum_algorithm(input);
            self
        }
        /// <p>The aggregation method used to generate the checksum. Currently, the only supported
        /// aggregation method is <code>LINEAR</code>.</p>
        pub fn checksum_aggregation_method(
            mut self,
            inp: crate::model::ChecksumAggregationMethod,
        ) -> Self {
            self.inner = self.inner.checksum_aggregation_method(inp);
            self
        }
        pub fn set_checksum_aggregation_method(
            mut self,
            input: std::option::Option<crate::model::ChecksumAggregationMethod>,
        ) -> Self {
            self.inner = self.inner.set_checksum_aggregation_method(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetSnapshotBlock<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_snapshot_block_input::Builder,
    }
    impl<C, M, R> GetSnapshotBlock<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetSnapshotBlockOutput,
            smithy_http::result::SdkError<crate::error::GetSnapshotBlockError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetSnapshotBlockInputOperationOutputAlias,
                crate::output::GetSnapshotBlockOutput,
                crate::error::GetSnapshotBlockError,
                crate::input::GetSnapshotBlockInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the snapshot containing the block from which to get data.</p>
        pub fn snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(inp);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
        /// <p>The block index of the block from which to get data.</p>
        /// <p>Obtain the <code>BlockIndex</code> by running the <code>ListChangedBlocks</code> or
        /// <code>ListSnapshotBlocks</code> operations.</p>
        pub fn block_index(mut self, inp: i32) -> Self {
            self.inner = self.inner.block_index(inp);
            self
        }
        pub fn set_block_index(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_block_index(input);
            self
        }
        /// <p>The block token of the block from which to get data.</p>
        /// <p>Obtain the <code>BlockToken</code> by running the <code>ListChangedBlocks</code> or
        /// <code>ListSnapshotBlocks</code> operations.</p>
        pub fn block_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.block_token(inp);
            self
        }
        pub fn set_block_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_block_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListChangedBlocks<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_changed_blocks_input::Builder,
    }
    impl<C, M, R> ListChangedBlocks<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListChangedBlocksOutput,
            smithy_http::result::SdkError<crate::error::ListChangedBlocksError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListChangedBlocksInputOperationOutputAlias,
                crate::output::ListChangedBlocksOutput,
                crate::error::ListChangedBlocksError,
                crate::input::ListChangedBlocksInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the first snapshot to use for the comparison.</p>
        /// <important>
        /// <p>The <code>FirstSnapshotID</code> parameter must be specified with a
        /// <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>
        /// </important>
        pub fn first_snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.first_snapshot_id(inp);
            self
        }
        pub fn set_first_snapshot_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_first_snapshot_id(input);
            self
        }
        /// <p>The ID of the second snapshot to use for the comparison.</p>
        /// <important>
        /// <p>The <code>SecondSnapshotId</code> parameter must be specified with a
        /// <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>
        /// </important>
        pub fn second_snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.second_snapshot_id(inp);
            self
        }
        pub fn set_second_snapshot_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_second_snapshot_id(input);
            self
        }
        /// <p>The token to request the next page of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The number of results to return.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The block index from which the comparison should start.</p>
        /// <p>The list in the response will start from this block index or the next valid block
        /// index in the snapshots.</p>
        pub fn starting_block_index(mut self, inp: i32) -> Self {
            self.inner = self.inner.starting_block_index(inp);
            self
        }
        pub fn set_starting_block_index(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_starting_block_index(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListSnapshotBlocks<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_snapshot_blocks_input::Builder,
    }
    impl<C, M, R> ListSnapshotBlocks<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListSnapshotBlocksOutput,
            smithy_http::result::SdkError<crate::error::ListSnapshotBlocksError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListSnapshotBlocksInputOperationOutputAlias,
                crate::output::ListSnapshotBlocksOutput,
                crate::error::ListSnapshotBlocksError,
                crate::input::ListSnapshotBlocksInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the snapshot from which to get block indexes and block tokens.</p>
        pub fn snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(inp);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
        /// <p>The token to request the next page of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The number of results to return.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The block index from which the list should start. The list in the response will start
        /// from this block index or the next valid block index in the snapshot.</p>
        pub fn starting_block_index(mut self, inp: i32) -> Self {
            self.inner = self.inner.starting_block_index(inp);
            self
        }
        pub fn set_starting_block_index(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_starting_block_index(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutSnapshotBlock<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_snapshot_block_input::Builder,
    }
    impl<C, M, R> PutSnapshotBlock<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutSnapshotBlockOutput,
            smithy_http::result::SdkError<crate::error::PutSnapshotBlockError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutSnapshotBlockInputOperationOutputAlias,
                crate::output::PutSnapshotBlockOutput,
                crate::error::PutSnapshotBlockError,
                crate::input::PutSnapshotBlockInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the snapshot.</p>
        pub fn snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(inp);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
        /// <p>The block index of the block in which to write the data. A block index is a logical
        /// index in units of <code>512</code> KiB blocks. To identify the block index, divide
        /// the logical offset of the data in the logical volume by the block size (logical offset of
        /// data/<code>524288</code>). The logical offset of the data must be <code>512</code>
        /// KiB aligned.</p>
        pub fn block_index(mut self, inp: i32) -> Self {
            self.inner = self.inner.block_index(inp);
            self
        }
        pub fn set_block_index(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_block_index(input);
            self
        }
        /// <p>The data to write to the block.</p>
        /// <p>The block data is not signed as part of the Signature Version 4 signing process. As a
        /// result, you must generate and provide a Base64-encoded SHA256 checksum for the block
        /// data using the <b>x-amz-Checksum</b> header. Also, you
        /// must specify the checksum algorithm using the <b>x-amz-Checksum-Algorithm</b>
        /// header. The checksum that you provide is part of the Signature Version 4 signing process.
        /// It is validated against a checksum generated by Amazon EBS to ensure the validity and authenticity
        /// of the data. If the checksums do not correspond, the request fails. For more information,
        /// see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapis-using-checksums">
        /// Using checksums with the EBS direct APIs</a> in the <i>Amazon Elastic Compute Cloud User
        /// Guide</i>.</p>
        pub fn block_data(mut self, inp: smithy_http::byte_stream::ByteStream) -> Self {
            self.inner = self.inner.block_data(inp);
            self
        }
        pub fn set_block_data(
            mut self,
            input: std::option::Option<smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.inner = self.inner.set_block_data(input);
            self
        }
        /// <p>The size of the data to write to the block, in bytes. Currently, the only supported
        /// size is <code>524288</code> bytes.</p>
        /// <p>Valid values: <code>524288</code>
        /// </p>
        pub fn data_length(mut self, inp: i32) -> Self {
            self.inner = self.inner.data_length(inp);
            self
        }
        pub fn set_data_length(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_data_length(input);
            self
        }
        /// <p>The progress of the write process, as a percentage.</p>
        pub fn progress(mut self, inp: i32) -> Self {
            self.inner = self.inner.progress(inp);
            self
        }
        pub fn set_progress(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_progress(input);
            self
        }
        /// <p>A Base64-encoded SHA256 checksum of the data. Only SHA256 checksums are
        /// supported.</p>
        pub fn checksum(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.checksum(inp);
            self
        }
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_checksum(input);
            self
        }
        /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm
        /// is <code>SHA256</code>.</p>
        pub fn checksum_algorithm(mut self, inp: crate::model::ChecksumAlgorithm) -> Self {
            self.inner = self.inner.checksum_algorithm(inp);
            self
        }
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::ChecksumAlgorithm>,
        ) -> Self {
            self.inner = self.inner.set_checksum_algorithm(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartSnapshot<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_snapshot_input::Builder,
    }
    impl<C, M, R> StartSnapshot<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartSnapshotOutput,
            smithy_http::result::SdkError<crate::error::StartSnapshotError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartSnapshotInputOperationOutputAlias,
                crate::output::StartSnapshotOutput,
                crate::error::StartSnapshotError,
                crate::input::StartSnapshotInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The size of the volume, in GiB. The maximum size is <code>65536</code> GiB (64
        /// TiB).</p>
        pub fn volume_size(mut self, inp: i64) -> Self {
            self.inner = self.inner.volume_size(inp);
            self
        }
        pub fn set_volume_size(mut self, input: std::option::Option<i64>) -> Self {
            self.inner = self.inner.set_volume_size(input);
            self
        }
        /// <p>The ID of the parent snapshot. If there is no parent snapshot, or if you are creating
        /// the first snapshot for an on-premises volume, omit this parameter.</p>
        /// <p>If your account is enabled for encryption by default, you cannot use an unencrypted
        /// snapshot as a parent snapshot. You must first create an encrypted copy of the parent
        /// snapshot using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CopySnapshot.html">CopySnapshot</a>.</p>
        pub fn parent_snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.parent_snapshot_id(inp);
            self
        }
        pub fn set_parent_snapshot_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_parent_snapshot_id(input);
            self
        }
        /// Appends an item to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>The tags to apply to the snapshot.</p>
        pub fn tags(mut self, inp: impl Into<crate::model::Tag>) -> Self {
            self.inner = self.inner.tags(inp);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
        /// <p>A description for the snapshot.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
        /// request. Idempotency ensures that an API request completes only once. With an idempotent
        /// request, if the original request completes successfully. The subsequent retries with the same
        /// client token return the result from the original successful request and they have no additional
        /// effect.</p>
        /// <p>If you do not specify a client token, one is automatically generated by the Amazon Web Services SDK.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-direct-api-idempotency.html">
        /// Idempotency for StartSnapshot API</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
        pub fn client_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_token(inp);
            self
        }
        pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_token(input);
            self
        }
        /// <p>Indicates whether to encrypt the snapshot. To create an encrypted snapshot, specify
        /// <code>true</code>. To create an unencrypted snapshot, omit this parameter.</p>
        /// <p>If you specify a value for <b>ParentSnapshotId</b>, omit
        /// this parameter.</p>
        /// <p>If you specify <code>true</code>, the snapshot is encrypted using the KMS key specified
        /// using the <b>KmsKeyArn</b> parameter. If no value is specified
        /// for <b>KmsKeyArn</b>, the default KMS key for your account is
        /// used. If no default KMS key has been specified for your account, the Amazon Web Services managed KMS key is used.
        /// To set a default KMS key for your account, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyEbsDefaultKmsKeyId.html">
        /// ModifyEbsDefaultKmsKeyId</a>.</p>
        /// <p>If your account is enabled for encryption by default, you cannot set this parameter to
        /// <code>false</code>. In this case, you can omit this parameter.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapis-using-encryption">
        /// Using encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
        pub fn encrypted(mut self, inp: bool) -> Self {
            self.inner = self.inner.encrypted(inp);
            self
        }
        pub fn set_encrypted(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_encrypted(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS)
        /// key to be used to encrypt the snapshot. If you do not specify a
        /// KMS key, the default Amazon Web Services managed KMS key is used.</p>
        /// <p>If you specify a <b>ParentSnapshotId</b>, omit this
        /// parameter; the snapshot will be encrypted using the same KMS key that was used to encrypt
        /// the parent snapshot.</p>
        /// <p>If <b>Encrypted</b> is set to <code>true</code>,
        /// you must specify a KMS key ARN. </p>
        pub fn kms_key_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.kms_key_arn(inp);
            self
        }
        pub fn set_kms_key_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_kms_key_arn(input);
            self
        }
        /// <p>The amount of time (in minutes) after which the snapshot is automatically cancelled
        /// if:</p>
        /// <ul>
        /// <li>
        /// <p>No blocks are written to the snapshot.</p>
        /// </li>
        /// <li>
        /// <p>The snapshot is not completed after writing the last block of data.</p>
        /// </li>
        /// </ul>
        /// <p>If no value is specified, the timeout defaults to <code>60</code> minutes.</p>
        pub fn timeout(mut self, inp: i32) -> Self {
            self.inner = self.inner.timeout(inp);
            self
        }
        pub fn set_timeout(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_timeout(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::new(conn).with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::https().with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
