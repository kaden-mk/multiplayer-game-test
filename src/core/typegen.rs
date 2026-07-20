pub struct FnSig {
    pub module: &'static str,
    pub name: &'static str,
    pub args: &'static [(&'static str, &'static str)],
    pub ret: &'static str,
}

inventory::collect!(FnSig);
