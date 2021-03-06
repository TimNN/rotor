/// Compose multiple state machines into single type
///
/// Composition requires two types: the state machine itself and Seed which
/// is used to create children state machines.
///
/// Mostly because of limitations of Rust macro system composition only works
/// on concrete context type.
///
/// # Example
/// ```ignore
/// rotor_compose!{
///     pub enum Fsm/Seed<Context> {
///         Http(HttpMachine)
///         Dns(DnsMachine)
///     }
/// }
/// ```
///
/// This creates a an `Fsm` state machine type which is enum with two options.
/// And `Seed` state machine type, which is also enum with same option names
/// but uses `<HttpMachine as rotor::Machine>::Seed` for the wrapped type.
#[macro_export]
macro_rules! rotor_compose {
    /* TODO(tailhook) make and check generic combinators
    (pub enum $name:ident { $($x:ident ($y:ty),)* }) => {
        pub enum $name { $($x ($y),)* }
        rotor_compose!(@machine $name C [] $($x($y),)*);
    };
    (enum $name:ident { $($x:ident ($y:ty),)* }) => {
        enum $name { $($x ($y),)* }
        rotor_compose!(@machine $name $($x($y),)*);
    };
    */
    (pub enum $name:ident/$cname:ident <$context_type:ident>
        { $($x:ident ($y:ty),)* })
    => {
        pub enum $name { $($x ($y),)* }
        pub enum $cname {
            $( $x (<$y as $crate::Machine>::Seed), )*
        }
        rotor_compose!(@machine $name/$cname
            $context_type [] $($x($y),)*);
    };
    (enum $name:ident/$cname:ident <$context_type:ident>
        { $($x:ident ($y:ty),)* })
    => {
        enum $name { $($x ($y),)* }
        enum $cname {
            $( $x (<$y as $crate::Machine>::Seed), )*
        }
        rotor_compose!(@machine $name/$cname
            $context_type [] $($x($y),)*);
    };
    (@machine $name:ident/$cname:ident $ctx_typ:ident
        [ $(<$ctx_name:ident $(: $ctx_bound:ident)*>)* ]
        $($iname:ident ($itype:ty),)*)
    => {
        impl $( <$ctx_name:$ctx_bound> )* $crate::Machine for $name {
            type Context = $ctx_typ;
            type Seed = $cname;
            fn create(seed: $cname, scope: &mut $crate::Scope<$ctx_typ>)
                -> $crate::Response<Self, $crate::Void>
            {
                match seed {
                    $( $cname::$iname (x)
                        => $crate::Machine::create(x, scope)
                            .map($name::$iname,
                                 |x| $crate::void::unreachable(x)),
                    )*
                }
            }
            fn ready(self, events: $crate::EventSet,
                scope: &mut $crate::Scope<$ctx_typ>)
                -> $crate::Response<Self, Self::Seed>
            {
                match self {
                    $(
                        $name::$iname(m) => {
                            m.ready(events, scope)
                                .map($name::$iname, $cname::$iname)
                        }
                    )*
                }
            }
            fn spawned(self, scope: &mut $crate::Scope<$ctx_typ>)
                -> $crate::Response<Self, Self::Seed>
            {
                match self {
                    $(
                        $name::$iname(m) => {
                            m.spawned(scope)
                                .map($name::$iname, $cname::$iname)
                        }
                    )*
                }
            }
            fn timeout(self, scope: &mut $crate::Scope<$ctx_typ>)
                -> $crate::Response<Self, Self::Seed>
            {
                match self {
                    $(
                        $name::$iname(m) => {
                            m.timeout(scope)
                                .map($name::$iname, $cname::$iname)
                        }
                    )*
                }
            }
            fn wakeup(self, scope: &mut $crate::Scope<$ctx_typ>)
                -> $crate::Response<Self, Self::Seed>
            {
                match self {
                    $(
                        $name::$iname(m) => {
                            m.wakeup(scope)
                                .map($name::$iname, $cname::$iname)
                        }
                    )*
                }
            }
        }

    }
}
