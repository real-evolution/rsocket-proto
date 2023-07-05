#[macro_export]
macro_rules! extract {
    ($obj:ident.$name:ident) => {
        $obj.$name.unwrap_or_default()
    };

    (required $obj:ident.$name:ident) => {
        match $obj.$name {
            | Some($name) => $name,
            | None => {
                return Err($crate::Error::MissingFieldValue {
                    frame_type: Self::FRAME_TYPE,
                    field: stringify!($name),
                })
            }
        }
    };

    (critical $obj:ident.$name:ident) => {
        $obj.$name.unwrap_or_else(|| {
            panic!(concat!("field `", stringify!($name), "` is required"))
        })
    };
}

#[macro_export]
macro_rules! define_builder {
    (
        $( #[$($top_meta:meta)*] )*
        builder $top_name:ident {$(
            $( #[$($builder_meta:meta)*] )*
            $name:ident for $($variant:ident)::+: $ftype:ident =>
            {
                // terminals
                $(
                    .$term:ident $( => sets [ $($term_flag:ident),* ] )? ;
                )+

                // setters
                $(
                    $(
                        $field_vis:vis $field:ident

                        $( $(($mod:ident))? : $field_ty:ty )?

                        $( =>
                           $( sets [ $($flag:ident),* ] )?
                        )?
                    );+
                    ;
                )?
            };
        )*}
    ) => {
        $(
            $( #[$($builder_meta)*] )*
            pub struct $name {
                __stream_id: $crate::frame::StreamId,
                __flags: $crate::frame::Flags,
                $(
                    $( $($field: Option<$field_ty>,)? )+
                 )?
            }

            impl $name {
                pub const FRAME_TYPE: $crate::frame::FrameType =
                    $crate::frame::FrameType::$ftype;

                $($(
                    #[inline]
                    $field_vis fn $field(mut self $(, $field: $field_ty)?) -> Self {
                        $( self.$field = Option::<$field_ty>::Some($field); )?

                        $(
                            $($(
                                self.__flags.insert($crate::frame::Flags::$flag);
                            )*)?
                         )?

                        self
                    }
                 )+)?

                fn __build(self) -> $crate::Result<$crate::frame::Frame> {
                    let header = $crate::frame::FrameHeader::new(
                        self.__stream_id,
                        Self::FRAME_TYPE,
                        self.__flags,
                    );

                    let variant = $($variant)::+ $({
                        $($(
                            $field: $crate::extract!($($mod)? self.$field),
                        )?)*
                    })?.into();

                    Ok($crate::frame::Frame { header, variant })
                }

                $(
                    #[inline(always)]
                    #[allow(unused_mut)]
                    pub fn $term(mut self) -> $crate::Result<$crate::frame::Frame> {
                        $($(
                            self.__flags.insert($crate::frame::Flags::$term_flag);
                        )*)?

                        self.__build()
                    }
                 )+
            }
        )*

        $( #[$($top_meta)*] )*
        pub struct $top_name(pub(super) $crate::frame::StreamId);

        impl $top_name {
            $(
                paste::paste! {
                    #[inline]
                    pub fn [<$ftype:snake>](self) -> $name {
                        $name {
                            __stream_id: self.0,
                            __flags: $crate::frame::Flags::empty(),
                            $($(
                                $( $field: Option::<$field_ty>::None, )?
                            )+)?
                        }
                    }
                }
             )*
        }
    };
}
