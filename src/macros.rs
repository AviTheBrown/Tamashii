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
