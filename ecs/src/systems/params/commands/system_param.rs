use crate::{Commands, SystemParam};

impl SystemParam for Commands {
    type Item<'world, 'state> = Commands;
}
