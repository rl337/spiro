//! Qualified binding keys (pyiv ``Key`` + ``Named``).

/// Qualifier on a binding, like pyiv ``Named('ci')``.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Named {
    Default,
    Ci,
    Live,
    Serve,
}

impl Named {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Ci => "ci",
            Self::Live => "live",
            Self::Serve => "serve",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Key<T: ?Sized> {
    pub name: Named,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ?Sized> Key<T> {
    pub const DEFAULT: Self = Self {
        name: Named::Default,
        _marker: std::marker::PhantomData,
    };

    pub const fn named(name: Named) -> Self {
        Self {
            name,
            _marker: std::marker::PhantomData,
        }
    }
}
