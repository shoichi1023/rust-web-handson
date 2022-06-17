use std::{sync::Arc, marker::PhantomData};

use derive_new::new;
use rust_web_handson_domain::repository::RepositoriesModuleExt;

pub mod todo;

#[derive(new)]
pub struct UseCaseImpl<T, R: RepositoriesModuleExt> {
    repositories: Arc<R>,
    _marker: PhantomData<T>
}
