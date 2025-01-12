// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about how traffic will be distributed between multiple target groups in a forward rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TargetGroupTuple {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    #[doc(hidden)]
    pub target_group_arn: ::std::option::Option<::std::string::String>,
    /// <p>The weight. The range is 0 to 999.</p>
    #[doc(hidden)]
    pub weight: ::std::option::Option<i32>,
}
impl TargetGroupTuple {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub fn target_group_arn(&self) -> ::std::option::Option<&str> {
        self.target_group_arn.as_deref()
    }
    /// <p>The weight. The range is 0 to 999.</p>
    pub fn weight(&self) -> ::std::option::Option<i32> {
        self.weight
    }
}
impl TargetGroupTuple {
    /// Creates a new builder-style object to manufacture [`TargetGroupTuple`](crate::types::TargetGroupTuple).
    pub fn builder() -> crate::types::builders::TargetGroupTupleBuilder {
        crate::types::builders::TargetGroupTupleBuilder::default()
    }
}

/// A builder for [`TargetGroupTuple`](crate::types::TargetGroupTuple).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TargetGroupTupleBuilder {
    pub(crate) target_group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) weight: ::std::option::Option<i32>,
}
impl TargetGroupTupleBuilder {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub fn target_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub fn set_target_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_group_arn = input;
        self
    }
    /// <p>The weight. The range is 0 to 999.</p>
    pub fn weight(mut self, input: i32) -> Self {
        self.weight = ::std::option::Option::Some(input);
        self
    }
    /// <p>The weight. The range is 0 to 999.</p>
    pub fn set_weight(mut self, input: ::std::option::Option<i32>) -> Self {
        self.weight = input;
        self
    }
    /// Consumes the builder and constructs a [`TargetGroupTuple`](crate::types::TargetGroupTuple).
    pub fn build(self) -> crate::types::TargetGroupTuple {
        crate::types::TargetGroupTuple {
            target_group_arn: self.target_group_arn,
            weight: self.weight,
        }
    }
}
