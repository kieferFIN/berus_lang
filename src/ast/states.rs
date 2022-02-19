pub trait AstState {}

pub struct Unverified;

pub struct Verified;

impl AstState for Unverified {}

impl AstState for Verified {}