use extendr_api::prelude::*;
use yrs::{
    Any as YAny, ArrayPrelim as YArrayPrelim, In as YIn, MapPrelim as YMapPrelim,
    TextPrelim as YTextPrelim,
};

use crate::type_conversion::FromExtendr;
use crate::utils;

#[derive(Clone)]
pub enum PrelimType {
    Map(List),
    Array(List),
    Text(Strings),
    Any(Robj),
}

impl IntoRobj for PrelimType {
    fn into_robj(self) -> Robj {
        match self {
            Self::Map(list) => list.into_robj(),
            Self::Array(list) => list.into_robj(),
            Self::Text(str) => str.into_robj(),
            Self::Any(obj) => obj,
        }
    }
}

utils::extendr_struct!(
    #[extendr]
    #[derive(Clone)]
    pub Prelim(PrelimType)
);

impl Prelim {
    fn in_from_text(str: Strings) -> Result<YIn, Error> {
        String::from_extendr(&str)
            .map(YTextPrelim::new)
            .map(Into::into)
    }

    fn in_from_array(list: List) -> Result<YIn, Error> {
        let mut out = YArrayPrelim::default();
        out.reserve(list.len());
        for obj in list.as_slice().iter() {
            // Not recursive, user must explicitly use Prelim internally
            // for nested CRDTs
            out.push(YAny::from_extendr(obj.clone())?.into());
        }
        Ok(out.into())
    }

    fn in_from_map(map: List) -> Result<YIn, Error> {
        let mut out = YMapPrelim::default();
        out.reserve(map.len());
        for (name, obj) in map.iter() {
            // Not recursive, user must explicitly use Prelim internally
            // for nested CRDTs
            out.insert(name.into(), YAny::from_extendr(obj.clone())?.into());
        }
        Ok(out.into())
    }

    fn in_from_any(obj: Robj) -> Result<YIn, Error> {
        YAny::from_extendr(obj).map(Into::into)
    }

    pub fn to_in(&self) -> Result<YIn, Error> {
        match self.as_ref() {
            PrelimType::Text(str) => Self::in_from_text(str.clone()),
            PrelimType::Map(list) => Self::in_from_map(list.clone()),
            PrelimType::Array(list) => Self::in_from_array(list.clone()),
            PrelimType::Any(obj) => Self::in_from_any(obj.clone()),
        }
    }
}

#[extendr]
impl Prelim {
    fn detect(obj: Robj) -> Self {
        if let Ok(prelim) = TryInto::<&Prelim>::try_into(&obj) {
            prelim.as_ref().clone().into()
        } else if let Ok(str) = TryInto::<Strings>::try_into(obj.clone()) {
            Self::text(str)
        } else {
            if let Some(list) = obj.as_list() {
                if list.has_names() {
                    Self::map(list)
                } else {
                    Self::array(list)
                }
            } else {
                Self::any(obj)
            }
        }
    }

    fn text(obj: Strings) -> Self {
        PrelimType::Text(obj).into()
    }

    fn array(obj: List) -> Self {
        PrelimType::Array(obj).into()
    }

    fn map(obj: List) -> Self {
        PrelimType::Map(obj).into()
    }

    fn any(obj: Robj) -> Self {
        PrelimType::Any(obj).into()
    }

    fn inner(&self) -> Robj {
        self.clone().into_robj()
    }
}

extendr_module! {
    mod prelim;
    impl Prelim;
}
