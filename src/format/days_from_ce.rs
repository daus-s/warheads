use chrono::{Datelike, NaiveDate};
use std::mem::MaybeUninit;
use wincode::{
    error::{ReadError, ReadResult, WriteResult},
    io::{Reader, Writer},
    SchemaRead, SchemaWrite,
};

pub struct DaysFromCE;

impl SchemaWrite for DaysFromCE {
    type Src = NaiveDate;

    fn size_of(src: &NaiveDate) -> WriteResult<usize> {
        <i32 as SchemaWrite>::size_of(&src.num_days_from_ce())
    }

    fn write(writer: &mut impl Writer, src: &NaiveDate) -> WriteResult<()> {
        <i32 as SchemaWrite>::write(writer, &src.num_days_from_ce())
    }
}

impl<'de> SchemaRead<'de> for DaysFromCE {
    type Dst = NaiveDate;

    fn read(reader: &mut impl Reader<'de>, dst: &mut MaybeUninit<NaiveDate>) -> ReadResult<()> {
        let n = <i32 as SchemaRead>::get(reader)?;
        let d = NaiveDate::from_num_days_from_ce_opt(n)
            .ok_or(ReadError::Custom("invalid NaiveDate"))?;
        dst.write(d);
        Ok(())
    }
}
