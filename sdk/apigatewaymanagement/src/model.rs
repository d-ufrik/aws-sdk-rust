// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Identity {
    /// <p>The source IP address of the TCP connection making the request to API Gateway.</p>
    pub source_ip: std::option::Option<std::string::String>,
    /// <p>The User Agent of the API caller.</p>
    pub user_agent: std::option::Option<std::string::String>,
}
impl Identity {
    /// <p>The source IP address of the TCP connection making the request to API Gateway.</p>
    pub fn source_ip(&self) -> std::option::Option<&str> {
        self.source_ip.as_deref()
    }
    /// <p>The User Agent of the API caller.</p>
    pub fn user_agent(&self) -> std::option::Option<&str> {
        self.user_agent.as_deref()
    }
}
impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Identity");
        formatter.field("source_ip", &self.source_ip);
        formatter.field("user_agent", &self.user_agent);
        formatter.finish()
    }
}
/// See [`Identity`](crate::model::Identity)
pub mod identity {
    /// A builder for [`Identity`](crate::model::Identity)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) source_ip: std::option::Option<std::string::String>,
        pub(crate) user_agent: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The source IP address of the TCP connection making the request to API Gateway.</p>
        pub fn source_ip(mut self, input: impl Into<std::string::String>) -> Self {
            self.source_ip = Some(input.into());
            self
        }
        /// <p>The source IP address of the TCP connection making the request to API Gateway.</p>
        pub fn set_source_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.source_ip = input;
            self
        }
        /// <p>The User Agent of the API caller.</p>
        pub fn user_agent(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_agent = Some(input.into());
            self
        }
        /// <p>The User Agent of the API caller.</p>
        pub fn set_user_agent(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_agent = input;
            self
        }
        /// Consumes the builder and constructs a [`Identity`](crate::model::Identity)
        pub fn build(self) -> crate::model::Identity {
            crate::model::Identity {
                source_ip: self.source_ip,
                user_agent: self.user_agent,
            }
        }
    }
}
impl Identity {
    /// Creates a new builder-style object to manufacture [`Identity`](crate::model::Identity)
    pub fn builder() -> crate::model::identity::Builder {
        crate::model::identity::Builder::default()
    }
}
