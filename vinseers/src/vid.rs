use crate::regexes;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LpnType {
    Fin,
    Fra,
    Hun,
    Ita,
    Ltu,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VidType {
    Vin,
    Lpn(LpnType),
}

impl VidType {
    pub const ALL: &'static [Self; 6] = &[
        Self::Vin,
        Self::Lpn(LpnType::Fin),
        Self::Lpn(LpnType::Fra),
        Self::Lpn(LpnType::Hun),
        Self::Lpn(LpnType::Ita),
        Self::Lpn(LpnType::Ltu),
        ];

    pub fn to_regex(&self) -> String {
        match self {
            Self::Vin => regexes::VIN_DEFAULT.to_string(),
            Self::Lpn(t) => match t {
                LpnType::Fin => regexes::LPN_FIN.to_string(),
                LpnType::Fra => regexes::LPN_FRA.to_string(),
                LpnType::Hun => regexes::LPN_HUN.to_string(),
                LpnType::Ita => regexes::LPN_ITA.to_string(),
                LpnType::Ltu => regexes::LPN_LTU.to_string(),
            },
        }
    }
}

impl ToString for VidType {
    fn to_string(&self) -> String {
        match self {
            VidType::Vin => "VIN".to_string(),
            VidType::Lpn(t) => match t {
                LpnType::Fin => "LPN-FIN".to_string(),
                LpnType::Fra => "LPN-FRA".to_string(),
                LpnType::Hun => "LPN-HUN".to_string(),
                LpnType::Ita => "LPN-ITA".to_string(),
                LpnType::Ltu => "LPN-LTU".to_string(),
            },
        }
    }
}
