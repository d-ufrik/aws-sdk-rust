// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration that determines the elements and canvas size options of sheet control.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SheetControlLayoutConfiguration {
    /// <p>The configuration that determines the elements and canvas size options of sheet control.</p>
    #[doc(hidden)]
    pub grid_layout: ::std::option::Option<crate::types::GridLayoutConfiguration>,
}
impl SheetControlLayoutConfiguration {
    /// <p>The configuration that determines the elements and canvas size options of sheet control.</p>
    pub fn grid_layout(&self) -> ::std::option::Option<&crate::types::GridLayoutConfiguration> {
        self.grid_layout.as_ref()
    }
}
impl SheetControlLayoutConfiguration {
    /// Creates a new builder-style object to manufacture [`SheetControlLayoutConfiguration`](crate::types::SheetControlLayoutConfiguration).
    pub fn builder() -> crate::types::builders::SheetControlLayoutConfigurationBuilder {
        crate::types::builders::SheetControlLayoutConfigurationBuilder::default()
    }
}

/// A builder for [`SheetControlLayoutConfiguration`](crate::types::SheetControlLayoutConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SheetControlLayoutConfigurationBuilder {
    pub(crate) grid_layout: ::std::option::Option<crate::types::GridLayoutConfiguration>,
}
impl SheetControlLayoutConfigurationBuilder {
    /// <p>The configuration that determines the elements and canvas size options of sheet control.</p>
    pub fn grid_layout(mut self, input: crate::types::GridLayoutConfiguration) -> Self {
        self.grid_layout = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration that determines the elements and canvas size options of sheet control.</p>
    pub fn set_grid_layout(
        mut self,
        input: ::std::option::Option<crate::types::GridLayoutConfiguration>,
    ) -> Self {
        self.grid_layout = input;
        self
    }
    /// Consumes the builder and constructs a [`SheetControlLayoutConfiguration`](crate::types::SheetControlLayoutConfiguration).
    pub fn build(self) -> crate::types::SheetControlLayoutConfiguration {
        crate::types::SheetControlLayoutConfiguration {
            grid_layout: self.grid_layout,
        }
    }
}
