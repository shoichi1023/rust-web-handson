use std::marker::PhantomData;

use derive_new::new;

use crate::client::mysql::Rds;

// 継承元になるクラス
#[derive(new)]
pub struct RdsRepositoryImpl<T> {
    pool: Rds,
    _marker: PhantomData<T>,
}

pub mod todo;
