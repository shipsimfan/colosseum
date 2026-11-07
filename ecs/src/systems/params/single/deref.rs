use crate::{QueryData, QueryFilter, Single};
use std::ops::{Deref, DerefMut};

impl<'world, 'state, Data: QueryData, Filter: QueryFilter> Deref
    for Single<'world, 'state, Data, Filter>
{
    type Target = Data::Item<'world, 'state>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'world, 'state, Data: QueryData, Filter: QueryFilter> DerefMut
    for Single<'world, 'state, Data, Filter>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'world, 'state, Data: QueryData, Filter: QueryFilter> AsRef<Data::Item<'world, 'state>>
    for Single<'world, 'state, Data, Filter>
{
    fn as_ref(&self) -> &Data::Item<'world, 'state> {
        &self.value
    }
}

impl<'world, 'state, Data: QueryData, Filter: QueryFilter> AsMut<Data::Item<'world, 'state>>
    for Single<'world, 'state, Data, Filter>
{
    fn as_mut(&mut self) -> &mut Data::Item<'world, 'state> {
        &mut self.value
    }
}
