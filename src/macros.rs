/// A macro to define structs with automatically public or private fields.
///
/// By default, all fields in a struct defined with `pub_struct!` are made `pub`.
/// To keep a field private, use the `#[private]` attribute. It also supports 
/// other attributes like `#[derive(...)]`.
///
/// # Examples
///
/// ```rust
/// use tamashii::pub_struct;
///
/// pub_struct! {
///     #[derive(Debug)]
///     pub struct MyStruct {
///         public_field: String,
///         #[private]
///         private_field: i32,
///     }
/// }
/// ```
///
/// # Visual Flow
///
/// ```text
/// pub_struct! { struct S { f: T } }
///      ↓
/// parse_fields! { @build ... }
///      ↓
/// struct S { pub f: T }
/// ```
#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($fields:tt)*
        }
    ) => {
        $crate::parse_fields! {
            @build
            $(#[$struct_meta])*
            $vis struct $name {}
            []
            $($fields)*
        }
    };
}

/// Helper macro for `pub_struct!` to recursively parse and transform fields.
///
/// This macro handles:
/// 1.  **Termination**: Emitting the final struct.
/// 2.  **Privacy**: Specifically looking for `#[private]`.
/// 3.  **Publicity**: Defaulting to `pub` for other fields.
/// 4.  **Attributes**: Collecting and applying other meta attributes.
#[macro_export]
macro_rules! parse_fields {
    // Done - output the final struct
    (
        @build
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($built:tt)*
        }
        []
    ) => {
        $(#[$struct_meta])*
        $vis struct $name {
            $($built)*
        }
    };

    // FIRST: Check for #[private] - emit field WITHOUT pub
    (
        @build
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($built:tt)*
        }
        [$(#[$acc:meta])*]
        #[private]
        $($rest:tt)*
    ) => {
        $crate::parse_fields! {
            @build_private
            $(#[$struct_meta])*
            $vis struct $name {
                $($built)*
            }
            [$(#[$acc])*]
            $($rest)*
        }
    };

    // SECOND: Hit a field identifier - emit WITH pub
    (
        @build
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($built:tt)*
        }
        [$(#[$acc:meta])*]
        $field:ident : $ty:ty,
        $($rest:tt)*
    ) => {
        $crate::parse_fields! {
            @build
            $(#[$struct_meta])*
            $vis struct $name {
                $($built)*
                $(#[$acc])*
                pub $field: $ty,
            }
            []
            $($rest)*
        }
    };

    // THIRD: Collect any other attribute
    (
        @build
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($built:tt)*
        }
        [$(#[$acc:meta])*]
        #[$attr:meta]
        $($rest:tt)*
    ) => {
        $crate::parse_fields! {
            @build
            $(#[$struct_meta])*
            $vis struct $name {
                $($built)*
            }
            [$(#[$acc])* #[$attr]]
            $($rest)*
        }
    };

    // Handle private field (after #[private] was matched)
    (
        @build_private
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($built:tt)*
        }
        [$(#[$acc:meta])*]
        $field:ident : $ty:ty,
        $($rest:tt)*
    ) => {
        $crate::parse_fields! {
            @build
            $(#[$struct_meta])*
            $vis struct $name {
                $($built)*
                $(#[$acc])*
                $field: $ty,
            }
            []
            $($rest)*
        }
    };
}
