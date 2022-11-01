use crate::TzPg;
use chrono_tz::Tz;

impl From<Tz> for TzPg {
    fn from(tz: Tz) -> Self {
        match tz {
            Tz::Africa__Abidjan => Self::Africa__Abidjan,
            Tz::Africa__Accra => Self::Africa__Accra,
            Tz::Africa__Addis_Ababa => Self::Africa__Addis_Ababa,
            Tz::Africa__Algiers => Self::Africa__Algiers,
            Tz::Africa__Asmara => Self::Africa__Asmara,
            Tz::Africa__Asmera => Self::Africa__Asmera,
            Tz::Africa__Bamako => Self::Africa__Bamako,
            Tz::Africa__Bangui => Self::Africa__Bangui,
            Tz::Africa__Banjul => Self::Africa__Banjul,
            Tz::Africa__Bissau => Self::Africa__Bissau,
            Tz::Africa__Blantyre => Self::Africa__Blantyre,
            Tz::Africa__Brazzaville => Self::Africa__Brazzaville,
            Tz::Africa__Bujumbura => Self::Africa__Bujumbura,
            Tz::Africa__Cairo => Self::Africa__Cairo,
            Tz::Africa__Casablanca => Self::Africa__Casablanca,
            Tz::Africa__Ceuta => Self::Africa__Ceuta,
            Tz::Africa__Conakry => Self::Africa__Conakry,
            Tz::Africa__Dakar => Self::Africa__Dakar,
            Tz::Africa__Dar_es_Salaam => Self::Africa__Dar_es_Salaam,
            Tz::Africa__Djibouti => Self::Africa__Djibouti,
            Tz::Africa__Douala => Self::Africa__Douala,
            Tz::Africa__El_Aaiun => Self::Africa__El_Aaiun,
            Tz::Africa__Freetown => Self::Africa__Freetown,
            Tz::Africa__Gaborone => Self::Africa__Gaborone,
            Tz::Africa__Harare => Self::Africa__Harare,
            Tz::Africa__Johannesburg => Self::Africa__Johannesburg,
            Tz::Africa__Juba => Self::Africa__Juba,
            Tz::Africa__Kampala => Self::Africa__Kampala,
            Tz::Africa__Khartoum => Self::Africa__Khartoum,
            Tz::Africa__Kigali => Self::Africa__Kigali,
            Tz::Africa__Kinshasa => Self::Africa__Kinshasa,
            Tz::Africa__Lagos => Self::Africa__Lagos,
            Tz::Africa__Libreville => Self::Africa__Libreville,
            Tz::Africa__Lome => Self::Africa__Lome,
            Tz::Africa__Luanda => Self::Africa__Luanda,
            Tz::Africa__Lubumbashi => Self::Africa__Lubumbashi,
            Tz::Africa__Lusaka => Self::Africa__Lusaka,
            Tz::Africa__Malabo => Self::Africa__Malabo,
            Tz::Africa__Maputo => Self::Africa__Maputo,
            Tz::Africa__Maseru => Self::Africa__Maseru,
            Tz::Africa__Mbabane => Self::Africa__Mbabane,
            Tz::Africa__Mogadishu => Self::Africa__Mogadishu,
            Tz::Africa__Monrovia => Self::Africa__Monrovia,
            Tz::Africa__Nairobi => Self::Africa__Nairobi,
            Tz::Africa__Ndjamena => Self::Africa__Ndjamena,
            Tz::Africa__Niamey => Self::Africa__Niamey,
            Tz::Africa__Nouakchott => Self::Africa__Nouakchott,
            Tz::Africa__Ouagadougou => Self::Africa__Ouagadougou,
            Tz::Africa__PortoNovo => Self::Africa__PortoNovo,
            Tz::Africa__Sao_Tome => Self::Africa__Sao_Tome,
            Tz::Africa__Timbuktu => Self::Africa__Timbuktu,
            Tz::Africa__Tripoli => Self::Africa__Tripoli,
            Tz::Africa__Tunis => Self::Africa__Tunis,
            Tz::Africa__Windhoek => Self::Africa__Windhoek,
            Tz::America__Adak => Self::America__Adak,
            Tz::America__Anchorage => Self::America__Anchorage,
            Tz::America__Anguilla => Self::America__Anguilla,
            Tz::America__Antigua => Self::America__Antigua,
            Tz::America__Araguaina => Self::America__Araguaina,
            Tz::America__Argentina__Buenos_Aires => Self::America__Argentina__Buenos_Aires,
            Tz::America__Argentina__Catamarca => Self::America__Argentina__Catamarca,
            Tz::America__Argentina__ComodRivadavia => Self::America__Argentina__ComodRivadavia,
            Tz::America__Argentina__Cordoba => Self::America__Argentina__Cordoba,
            Tz::America__Argentina__Jujuy => Self::America__Argentina__Jujuy,
            Tz::America__Argentina__La_Rioja => Self::America__Argentina__La_Rioja,
            Tz::America__Argentina__Mendoza => Self::America__Argentina__Mendoza,
            Tz::America__Argentina__Rio_Gallegos => Self::America__Argentina__Rio_Gallegos,
            Tz::America__Argentina__Salta => Self::America__Argentina__Salta,
            Tz::America__Argentina__San_Juan => Self::America__Argentina__San_Juan,
            Tz::America__Argentina__San_Luis => Self::America__Argentina__San_Luis,
            Tz::America__Argentina__Tucuman => Self::America__Argentina__Tucuman,
            Tz::America__Argentina__Ushuaia => Self::America__Argentina__Ushuaia,
            Tz::America__Aruba => Self::America__Aruba,
            Tz::America__Asuncion => Self::America__Asuncion,
            Tz::America__Atikokan => Self::America__Atikokan,
            Tz::America__Atka => Self::America__Atka,
            Tz::America__Bahia => Self::America__Bahia,
            Tz::America__Bahia_Banderas => Self::America__Bahia_Banderas,
            Tz::America__Barbados => Self::America__Barbados,
            Tz::America__Belem => Self::America__Belem,
            Tz::America__Belize => Self::America__Belize,
            Tz::America__BlancSablon => Self::America__BlancSablon,
            Tz::America__Boa_Vista => Self::America__Boa_Vista,
            Tz::America__Bogota => Self::America__Bogota,
            Tz::America__Boise => Self::America__Boise,
            Tz::America__Buenos_Aires => Self::America__Buenos_Aires,
            Tz::America__Cambridge_Bay => Self::America__Cambridge_Bay,
            Tz::America__Campo_Grande => Self::America__Campo_Grande,
            Tz::America__Cancun => Self::America__Cancun,
            Tz::America__Caracas => Self::America__Caracas,
            Tz::America__Catamarca => Self::America__Catamarca,
            Tz::America__Cayenne => Self::America__Cayenne,
            Tz::America__Cayman => Self::America__Cayman,
            Tz::America__Chicago => Self::America__Chicago,
            Tz::America__Chihuahua => Self::America__Chihuahua,
            Tz::America__Coral_Harbour => Self::America__Coral_Harbour,
            Tz::America__Cordoba => Self::America__Cordoba,
            Tz::America__Costa_Rica => Self::America__Costa_Rica,
            Tz::America__Creston => Self::America__Creston,
            Tz::America__Cuiaba => Self::America__Cuiaba,
            Tz::America__Curacao => Self::America__Curacao,
            Tz::America__Danmarkshavn => Self::America__Danmarkshavn,
            Tz::America__Dawson => Self::America__Dawson,
            Tz::America__Dawson_Creek => Self::America__Dawson_Creek,
            Tz::America__Denver => Self::America__Denver,
            Tz::America__Detroit => Self::America__Detroit,
            Tz::America__Dominica => Self::America__Dominica,
            Tz::America__Edmonton => Self::America__Edmonton,
            Tz::America__Eirunepe => Self::America__Eirunepe,
            Tz::America__El_Salvador => Self::America__El_Salvador,
            Tz::America__Ensenada => Self::America__Ensenada,
            Tz::America__Fort_Nelson => Self::America__Fort_Nelson,
            Tz::America__Fort_Wayne => Self::America__Fort_Wayne,
            Tz::America__Fortaleza => Self::America__Fortaleza,
            Tz::America__Glace_Bay => Self::America__Glace_Bay,
            Tz::America__Godthab => Self::America__Godthab,
            Tz::America__Goose_Bay => Self::America__Goose_Bay,
            Tz::America__Grand_Turk => Self::America__Grand_Turk,
            Tz::America__Grenada => Self::America__Grenada,
            Tz::America__Guadeloupe => Self::America__Guadeloupe,
            Tz::America__Guatemala => Self::America__Guatemala,
            Tz::America__Guayaquil => Self::America__Guayaquil,
            Tz::America__Guyana => Self::America__Guyana,
            Tz::America__Halifax => Self::America__Halifax,
            Tz::America__Havana => Self::America__Havana,
            Tz::America__Hermosillo => Self::America__Hermosillo,
            Tz::America__Indiana__Indianapolis => Self::America__Indiana__Indianapolis,
            Tz::America__Indiana__Knox => Self::America__Indiana__Knox,
            Tz::America__Indiana__Marengo => Self::America__Indiana__Marengo,
            Tz::America__Indiana__Petersburg => Self::America__Indiana__Petersburg,
            Tz::America__Indiana__Tell_City => Self::America__Indiana__Tell_City,
            Tz::America__Indiana__Vevay => Self::America__Indiana__Vevay,
            Tz::America__Indiana__Vincennes => Self::America__Indiana__Vincennes,
            Tz::America__Indiana__Winamac => Self::America__Indiana__Winamac,
            Tz::America__Indianapolis => Self::America__Indianapolis,
            Tz::America__Inuvik => Self::America__Inuvik,
            Tz::America__Iqaluit => Self::America__Iqaluit,
            Tz::America__Jamaica => Self::America__Jamaica,
            Tz::America__Jujuy => Self::America__Jujuy,
            Tz::America__Juneau => Self::America__Juneau,
            Tz::America__Kentucky__Louisville => Self::America__Kentucky__Louisville,
            Tz::America__Kentucky__Monticello => Self::America__Kentucky__Monticello,
            Tz::America__Knox_IN => Self::America__Knox_IN,
            Tz::America__Kralendijk => Self::America__Kralendijk,
            Tz::America__La_Paz => Self::America__La_Paz,
            Tz::America__Lima => Self::America__Lima,
            Tz::America__Los_Angeles => Self::America__Los_Angeles,
            Tz::America__Louisville => Self::America__Louisville,
            Tz::America__Lower_Princes => Self::America__Lower_Princes,
            Tz::America__Maceio => Self::America__Maceio,
            Tz::America__Managua => Self::America__Managua,
            Tz::America__Manaus => Self::America__Manaus,
            Tz::America__Marigot => Self::America__Marigot,
            Tz::America__Martinique => Self::America__Martinique,
            Tz::America__Matamoros => Self::America__Matamoros,
            Tz::America__Mazatlan => Self::America__Mazatlan,
            Tz::America__Mendoza => Self::America__Mendoza,
            Tz::America__Menominee => Self::America__Menominee,
            Tz::America__Merida => Self::America__Merida,
            Tz::America__Metlakatla => Self::America__Metlakatla,
            Tz::America__Mexico_City => Self::America__Mexico_City,
            Tz::America__Miquelon => Self::America__Miquelon,
            Tz::America__Moncton => Self::America__Moncton,
            Tz::America__Monterrey => Self::America__Monterrey,
            Tz::America__Montevideo => Self::America__Montevideo,
            Tz::America__Montreal => Self::America__Montreal,
            Tz::America__Montserrat => Self::America__Montserrat,
            Tz::America__Nassau => Self::America__Nassau,
            Tz::America__New_York => Self::America__New_York,
            Tz::America__Nipigon => Self::America__Nipigon,
            Tz::America__Nome => Self::America__Nome,
            Tz::America__Noronha => Self::America__Noronha,
            Tz::America__North_Dakota__Beulah => Self::America__North_Dakota__Beulah,
            Tz::America__North_Dakota__Center => Self::America__North_Dakota__Center,
            Tz::America__North_Dakota__New_Salem => Self::America__North_Dakota__New_Salem,
            Tz::America__Nuuk => Self::America__Nuuk,
            Tz::America__Ojinaga => Self::America__Ojinaga,
            Tz::America__Panama => Self::America__Panama,
            Tz::America__Pangnirtung => Self::America__Pangnirtung,
            Tz::America__Paramaribo => Self::America__Paramaribo,
            Tz::America__Phoenix => Self::America__Phoenix,
            Tz::America__PortauPrince => Self::America__PortauPrince,
            Tz::America__Port_of_Spain => Self::America__Port_of_Spain,
            Tz::America__Porto_Acre => Self::America__Porto_Acre,
            Tz::America__Porto_Velho => Self::America__Porto_Velho,
            Tz::America__Puerto_Rico => Self::America__Puerto_Rico,
            Tz::America__Punta_Arenas => Self::America__Punta_Arenas,
            Tz::America__Rainy_River => Self::America__Rainy_River,
            Tz::America__Rankin_Inlet => Self::America__Rankin_Inlet,
            Tz::America__Recife => Self::America__Recife,
            Tz::America__Regina => Self::America__Regina,
            Tz::America__Resolute => Self::America__Resolute,
            Tz::America__Rio_Branco => Self::America__Rio_Branco,
            Tz::America__Rosario => Self::America__Rosario,
            Tz::America__Santa_Isabel => Self::America__Santa_Isabel,
            Tz::America__Santarem => Self::America__Santarem,
            Tz::America__Santiago => Self::America__Santiago,
            Tz::America__Santo_Domingo => Self::America__Santo_Domingo,
            Tz::America__Sao_Paulo => Self::America__Sao_Paulo,
            Tz::America__Scoresbysund => Self::America__Scoresbysund,
            Tz::America__Shiprock => Self::America__Shiprock,
            Tz::America__Sitka => Self::America__Sitka,
            Tz::America__St_Barthelemy => Self::America__St_Barthelemy,
            Tz::America__St_Johns => Self::America__St_Johns,
            Tz::America__St_Kitts => Self::America__St_Kitts,
            Tz::America__St_Lucia => Self::America__St_Lucia,
            Tz::America__St_Thomas => Self::America__St_Thomas,
            Tz::America__St_Vincent => Self::America__St_Vincent,
            Tz::America__Swift_Current => Self::America__Swift_Current,
            Tz::America__Tegucigalpa => Self::America__Tegucigalpa,
            Tz::America__Thule => Self::America__Thule,
            Tz::America__Thunder_Bay => Self::America__Thunder_Bay,
            Tz::America__Tijuana => Self::America__Tijuana,
            Tz::America__Toronto => Self::America__Toronto,
            Tz::America__Tortola => Self::America__Tortola,
            Tz::America__Vancouver => Self::America__Vancouver,
            Tz::America__Virgin => Self::America__Virgin,
            Tz::America__Whitehorse => Self::America__Whitehorse,
            Tz::America__Winnipeg => Self::America__Winnipeg,
            Tz::America__Yakutat => Self::America__Yakutat,
            Tz::America__Yellowknife => Self::America__Yellowknife,
            Tz::Antarctica__Casey => Self::Antarctica__Casey,
            Tz::Antarctica__Davis => Self::Antarctica__Davis,
            Tz::Antarctica__DumontDUrville => Self::Antarctica__DumontDUrville,
            Tz::Antarctica__Macquarie => Self::Antarctica__Macquarie,
            Tz::Antarctica__Mawson => Self::Antarctica__Mawson,
            Tz::Antarctica__McMurdo => Self::Antarctica__McMurdo,
            Tz::Antarctica__Palmer => Self::Antarctica__Palmer,
            Tz::Antarctica__Rothera => Self::Antarctica__Rothera,
            Tz::Antarctica__South_Pole => Self::Antarctica__South_Pole,
            Tz::Antarctica__Syowa => Self::Antarctica__Syowa,
            Tz::Antarctica__Troll => Self::Antarctica__Troll,
            Tz::Antarctica__Vostok => Self::Antarctica__Vostok,
            Tz::Arctic__Longyearbyen => Self::Arctic__Longyearbyen,
            Tz::Asia__Aden => Self::Asia__Aden,
            Tz::Asia__Almaty => Self::Asia__Almaty,
            Tz::Asia__Amman => Self::Asia__Amman,
            Tz::Asia__Anadyr => Self::Asia__Anadyr,
            Tz::Asia__Aqtau => Self::Asia__Aqtau,
            Tz::Asia__Aqtobe => Self::Asia__Aqtobe,
            Tz::Asia__Ashgabat => Self::Asia__Ashgabat,
            Tz::Asia__Ashkhabad => Self::Asia__Ashkhabad,
            Tz::Asia__Atyrau => Self::Asia__Atyrau,
            Tz::Asia__Baghdad => Self::Asia__Baghdad,
            Tz::Asia__Bahrain => Self::Asia__Bahrain,
            Tz::Asia__Baku => Self::Asia__Baku,
            Tz::Asia__Bangkok => Self::Asia__Bangkok,
            Tz::Asia__Barnaul => Self::Asia__Barnaul,
            Tz::Asia__Beirut => Self::Asia__Beirut,
            Tz::Asia__Bishkek => Self::Asia__Bishkek,
            Tz::Asia__Brunei => Self::Asia__Brunei,
            Tz::Asia__Calcutta => Self::Asia__Calcutta,
            Tz::Asia__Chita => Self::Asia__Chita,
            Tz::Asia__Choibalsan => Self::Asia__Choibalsan,
            Tz::Asia__Chongqing => Self::Asia__Chongqing,
            Tz::Asia__Chungking => Self::Asia__Chungking,
            Tz::Asia__Colombo => Self::Asia__Colombo,
            Tz::Asia__Dacca => Self::Asia__Dacca,
            Tz::Asia__Damascus => Self::Asia__Damascus,
            Tz::Asia__Dhaka => Self::Asia__Dhaka,
            Tz::Asia__Dili => Self::Asia__Dili,
            Tz::Asia__Dubai => Self::Asia__Dubai,
            Tz::Asia__Dushanbe => Self::Asia__Dushanbe,
            Tz::Asia__Famagusta => Self::Asia__Famagusta,
            Tz::Asia__Gaza => Self::Asia__Gaza,
            Tz::Asia__Harbin => Self::Asia__Harbin,
            Tz::Asia__Hebron => Self::Asia__Hebron,
            Tz::Asia__Ho_Chi_Minh => Self::Asia__Ho_Chi_Minh,
            Tz::Asia__Hong_Kong => Self::Asia__Hong_Kong,
            Tz::Asia__Hovd => Self::Asia__Hovd,
            Tz::Asia__Irkutsk => Self::Asia__Irkutsk,
            Tz::Asia__Istanbul => Self::Asia__Istanbul,
            Tz::Asia__Jakarta => Self::Asia__Jakarta,
            Tz::Asia__Jayapura => Self::Asia__Jayapura,
            Tz::Asia__Jerusalem => Self::Asia__Jerusalem,
            Tz::Asia__Kabul => Self::Asia__Kabul,
            Tz::Asia__Kamchatka => Self::Asia__Kamchatka,
            Tz::Asia__Karachi => Self::Asia__Karachi,
            Tz::Asia__Kashgar => Self::Asia__Kashgar,
            Tz::Asia__Kathmandu => Self::Asia__Kathmandu,
            Tz::Asia__Katmandu => Self::Asia__Katmandu,
            Tz::Asia__Khandyga => Self::Asia__Khandyga,
            Tz::Asia__Kolkata => Self::Asia__Kolkata,
            Tz::Asia__Krasnoyarsk => Self::Asia__Krasnoyarsk,
            Tz::Asia__Kuala_Lumpur => Self::Asia__Kuala_Lumpur,
            Tz::Asia__Kuching => Self::Asia__Kuching,
            Tz::Asia__Kuwait => Self::Asia__Kuwait,
            Tz::Asia__Macao => Self::Asia__Macao,
            Tz::Asia__Macau => Self::Asia__Macau,
            Tz::Asia__Magadan => Self::Asia__Magadan,
            Tz::Asia__Makassar => Self::Asia__Makassar,
            Tz::Asia__Manila => Self::Asia__Manila,
            Tz::Asia__Muscat => Self::Asia__Muscat,
            Tz::Asia__Nicosia => Self::Asia__Nicosia,
            Tz::Asia__Novokuznetsk => Self::Asia__Novokuznetsk,
            Tz::Asia__Novosibirsk => Self::Asia__Novosibirsk,
            Tz::Asia__Omsk => Self::Asia__Omsk,
            Tz::Asia__Oral => Self::Asia__Oral,
            Tz::Asia__Phnom_Penh => Self::Asia__Phnom_Penh,
            Tz::Asia__Pontianak => Self::Asia__Pontianak,
            Tz::Asia__Pyongyang => Self::Asia__Pyongyang,
            Tz::Asia__Qatar => Self::Asia__Qatar,
            Tz::Asia__Qostanay => Self::Asia__Qostanay,
            Tz::Asia__Qyzylorda => Self::Asia__Qyzylorda,
            Tz::Asia__Rangoon => Self::Asia__Rangoon,
            Tz::Asia__Riyadh => Self::Asia__Riyadh,
            Tz::Asia__Saigon => Self::Asia__Saigon,
            Tz::Asia__Sakhalin => Self::Asia__Sakhalin,
            Tz::Asia__Samarkand => Self::Asia__Samarkand,
            Tz::Asia__Seoul => Self::Asia__Seoul,
            Tz::Asia__Shanghai => Self::Asia__Shanghai,
            Tz::Asia__Singapore => Self::Asia__Singapore,
            Tz::Asia__Srednekolymsk => Self::Asia__Srednekolymsk,
            Tz::Asia__Taipei => Self::Asia__Taipei,
            Tz::Asia__Tashkent => Self::Asia__Tashkent,
            Tz::Asia__Tbilisi => Self::Asia__Tbilisi,
            Tz::Asia__Tehran => Self::Asia__Tehran,
            Tz::Asia__Tel_Aviv => Self::Asia__Tel_Aviv,
            Tz::Asia__Thimbu => Self::Asia__Thimbu,
            Tz::Asia__Thimphu => Self::Asia__Thimphu,
            Tz::Asia__Tokyo => Self::Asia__Tokyo,
            Tz::Asia__Tomsk => Self::Asia__Tomsk,
            Tz::Asia__Ujung_Pandang => Self::Asia__Ujung_Pandang,
            Tz::Asia__Ulaanbaatar => Self::Asia__Ulaanbaatar,
            Tz::Asia__Ulan_Bator => Self::Asia__Ulan_Bator,
            Tz::Asia__Urumqi => Self::Asia__Urumqi,
            Tz::Asia__UstNera => Self::Asia__UstNera,
            Tz::Asia__Vientiane => Self::Asia__Vientiane,
            Tz::Asia__Vladivostok => Self::Asia__Vladivostok,
            Tz::Asia__Yakutsk => Self::Asia__Yakutsk,
            Tz::Asia__Yangon => Self::Asia__Yangon,
            Tz::Asia__Yekaterinburg => Self::Asia__Yekaterinburg,
            Tz::Asia__Yerevan => Self::Asia__Yerevan,
            Tz::Atlantic__Azores => Self::Atlantic__Azores,
            Tz::Atlantic__Bermuda => Self::Atlantic__Bermuda,
            Tz::Atlantic__Canary => Self::Atlantic__Canary,
            Tz::Atlantic__Cape_Verde => Self::Atlantic__Cape_Verde,
            Tz::Atlantic__Faeroe => Self::Atlantic__Faeroe,
            Tz::Atlantic__Faroe => Self::Atlantic__Faroe,
            Tz::Atlantic__Jan_Mayen => Self::Atlantic__Jan_Mayen,
            Tz::Atlantic__Madeira => Self::Atlantic__Madeira,
            Tz::Atlantic__Reykjavik => Self::Atlantic__Reykjavik,
            Tz::Atlantic__South_Georgia => Self::Atlantic__South_Georgia,
            Tz::Atlantic__St_Helena => Self::Atlantic__St_Helena,
            Tz::Atlantic__Stanley => Self::Atlantic__Stanley,
            Tz::Australia__ACT => Self::Australia__ACT,
            Tz::Australia__Adelaide => Self::Australia__Adelaide,
            Tz::Australia__Brisbane => Self::Australia__Brisbane,
            Tz::Australia__Broken_Hill => Self::Australia__Broken_Hill,
            Tz::Australia__Canberra => Self::Australia__Canberra,
            Tz::Australia__Currie => Self::Australia__Currie,
            Tz::Australia__Darwin => Self::Australia__Darwin,
            Tz::Australia__Eucla => Self::Australia__Eucla,
            Tz::Australia__Hobart => Self::Australia__Hobart,
            Tz::Australia__LHI => Self::Australia__LHI,
            Tz::Australia__Lindeman => Self::Australia__Lindeman,
            Tz::Australia__Lord_Howe => Self::Australia__Lord_Howe,
            Tz::Australia__Melbourne => Self::Australia__Melbourne,
            Tz::Australia__NSW => Self::Australia__NSW,
            Tz::Australia__North => Self::Australia__North,
            Tz::Australia__Perth => Self::Australia__Perth,
            Tz::Australia__Queensland => Self::Australia__Queensland,
            Tz::Australia__South => Self::Australia__South,
            Tz::Australia__Sydney => Self::Australia__Sydney,
            Tz::Australia__Tasmania => Self::Australia__Tasmania,
            Tz::Australia__Victoria => Self::Australia__Victoria,
            Tz::Australia__West => Self::Australia__West,
            Tz::Australia__Yancowinna => Self::Australia__Yancowinna,
            Tz::Brazil__Acre => Self::Brazil__Acre,
            Tz::Brazil__DeNoronha => Self::Brazil__DeNoronha,
            Tz::Brazil__East => Self::Brazil__East,
            Tz::Brazil__West => Self::Brazil__West,
            Tz::CET => Self::CET,
            Tz::CST6CDT => Self::CST6CDT,
            Tz::Canada__Atlantic => Self::Canada__Atlantic,
            Tz::Canada__Central => Self::Canada__Central,
            Tz::Canada__Eastern => Self::Canada__Eastern,
            Tz::Canada__Mountain => Self::Canada__Mountain,
            Tz::Canada__Newfoundland => Self::Canada__Newfoundland,
            Tz::Canada__Pacific => Self::Canada__Pacific,
            Tz::Canada__Saskatchewan => Self::Canada__Saskatchewan,
            Tz::Canada__Yukon => Self::Canada__Yukon,
            Tz::Chile__Continental => Self::Chile__Continental,
            Tz::Chile__EasterIsland => Self::Chile__EasterIsland,
            Tz::Cuba => Self::Cuba,
            Tz::EET => Self::EET,
            Tz::EST => Self::EST,
            Tz::EST5EDT => Self::EST5EDT,
            Tz::Egypt => Self::Egypt,
            Tz::Eire => Self::Eire,
            Tz::Etc__GMT => Self::Etc__GMT,
            Tz::Etc__GMTPlus0 => Self::Etc__GMTPlus0,
            Tz::Etc__GMTPlus1 => Self::Etc__GMTPlus1,
            Tz::Etc__GMTPlus10 => Self::Etc__GMTPlus10,
            Tz::Etc__GMTPlus11 => Self::Etc__GMTPlus11,
            Tz::Etc__GMTPlus12 => Self::Etc__GMTPlus12,
            Tz::Etc__GMTPlus2 => Self::Etc__GMTPlus2,
            Tz::Etc__GMTPlus3 => Self::Etc__GMTPlus3,
            Tz::Etc__GMTPlus4 => Self::Etc__GMTPlus4,
            Tz::Etc__GMTPlus5 => Self::Etc__GMTPlus5,
            Tz::Etc__GMTPlus6 => Self::Etc__GMTPlus6,
            Tz::Etc__GMTPlus7 => Self::Etc__GMTPlus7,
            Tz::Etc__GMTPlus8 => Self::Etc__GMTPlus8,
            Tz::Etc__GMTPlus9 => Self::Etc__GMTPlus9,
            Tz::Etc__GMTMinus0 => Self::Etc__GMTMinus0,
            Tz::Etc__GMTMinus1 => Self::Etc__GMTMinus1,
            Tz::Etc__GMTMinus10 => Self::Etc__GMTMinus10,
            Tz::Etc__GMTMinus11 => Self::Etc__GMTMinus11,
            Tz::Etc__GMTMinus12 => Self::Etc__GMTMinus12,
            Tz::Etc__GMTMinus13 => Self::Etc__GMTMinus13,
            Tz::Etc__GMTMinus14 => Self::Etc__GMTMinus14,
            Tz::Etc__GMTMinus2 => Self::Etc__GMTMinus2,
            Tz::Etc__GMTMinus3 => Self::Etc__GMTMinus3,
            Tz::Etc__GMTMinus4 => Self::Etc__GMTMinus4,
            Tz::Etc__GMTMinus5 => Self::Etc__GMTMinus5,
            Tz::Etc__GMTMinus6 => Self::Etc__GMTMinus6,
            Tz::Etc__GMTMinus7 => Self::Etc__GMTMinus7,
            Tz::Etc__GMTMinus8 => Self::Etc__GMTMinus8,
            Tz::Etc__GMTMinus9 => Self::Etc__GMTMinus9,
            Tz::Etc__GMT0 => Self::Etc__GMT0,
            Tz::Etc__Greenwich => Self::Etc__Greenwich,
            Tz::Etc__UCT => Self::Etc__UCT,
            Tz::Etc__UTC => Self::Etc__UTC,
            Tz::Etc__Universal => Self::Etc__Universal,
            Tz::Etc__Zulu => Self::Etc__Zulu,
            Tz::Europe__Amsterdam => Self::Europe__Amsterdam,
            Tz::Europe__Andorra => Self::Europe__Andorra,
            Tz::Europe__Astrakhan => Self::Europe__Astrakhan,
            Tz::Europe__Athens => Self::Europe__Athens,
            Tz::Europe__Belfast => Self::Europe__Belfast,
            Tz::Europe__Belgrade => Self::Europe__Belgrade,
            Tz::Europe__Berlin => Self::Europe__Berlin,
            Tz::Europe__Bratislava => Self::Europe__Bratislava,
            Tz::Europe__Brussels => Self::Europe__Brussels,
            Tz::Europe__Bucharest => Self::Europe__Bucharest,
            Tz::Europe__Budapest => Self::Europe__Budapest,
            Tz::Europe__Busingen => Self::Europe__Busingen,
            Tz::Europe__Chisinau => Self::Europe__Chisinau,
            Tz::Europe__Copenhagen => Self::Europe__Copenhagen,
            Tz::Europe__Dublin => Self::Europe__Dublin,
            Tz::Europe__Gibraltar => Self::Europe__Gibraltar,
            Tz::Europe__Guernsey => Self::Europe__Guernsey,
            Tz::Europe__Helsinki => Self::Europe__Helsinki,
            Tz::Europe__Isle_of_Man => Self::Europe__Isle_of_Man,
            Tz::Europe__Istanbul => Self::Europe__Istanbul,
            Tz::Europe__Jersey => Self::Europe__Jersey,
            Tz::Europe__Kaliningrad => Self::Europe__Kaliningrad,
            Tz::Europe__Kiev => Self::Europe__Kiev,
            Tz::Europe__Kirov => Self::Europe__Kirov,
            Tz::Europe__Kyiv => Self::Europe__Kyiv,
            Tz::Europe__Lisbon => Self::Europe__Lisbon,
            Tz::Europe__Ljubljana => Self::Europe__Ljubljana,
            Tz::Europe__London => Self::Europe__London,
            Tz::Europe__Luxembourg => Self::Europe__Luxembourg,
            Tz::Europe__Madrid => Self::Europe__Madrid,
            Tz::Europe__Malta => Self::Europe__Malta,
            Tz::Europe__Mariehamn => Self::Europe__Mariehamn,
            Tz::Europe__Minsk => Self::Europe__Minsk,
            Tz::Europe__Monaco => Self::Europe__Monaco,
            Tz::Europe__Moscow => Self::Europe__Moscow,
            Tz::Europe__Nicosia => Self::Europe__Nicosia,
            Tz::Europe__Oslo => Self::Europe__Oslo,
            Tz::Europe__Paris => Self::Europe__Paris,
            Tz::Europe__Podgorica => Self::Europe__Podgorica,
            Tz::Europe__Prague => Self::Europe__Prague,
            Tz::Europe__Riga => Self::Europe__Riga,
            Tz::Europe__Rome => Self::Europe__Rome,
            Tz::Europe__Samara => Self::Europe__Samara,
            Tz::Europe__San_Marino => Self::Europe__San_Marino,
            Tz::Europe__Sarajevo => Self::Europe__Sarajevo,
            Tz::Europe__Saratov => Self::Europe__Saratov,
            Tz::Europe__Simferopol => Self::Europe__Simferopol,
            Tz::Europe__Skopje => Self::Europe__Skopje,
            Tz::Europe__Sofia => Self::Europe__Sofia,
            Tz::Europe__Stockholm => Self::Europe__Stockholm,
            Tz::Europe__Tallinn => Self::Europe__Tallinn,
            Tz::Europe__Tirane => Self::Europe__Tirane,
            Tz::Europe__Tiraspol => Self::Europe__Tiraspol,
            Tz::Europe__Ulyanovsk => Self::Europe__Ulyanovsk,
            Tz::Europe__Uzhgorod => Self::Europe__Uzhgorod,
            Tz::Europe__Vaduz => Self::Europe__Vaduz,
            Tz::Europe__Vatican => Self::Europe__Vatican,
            Tz::Europe__Vienna => Self::Europe__Vienna,
            Tz::Europe__Vilnius => Self::Europe__Vilnius,
            Tz::Europe__Volgograd => Self::Europe__Volgograd,
            Tz::Europe__Warsaw => Self::Europe__Warsaw,
            Tz::Europe__Zagreb => Self::Europe__Zagreb,
            Tz::Europe__Zaporozhye => Self::Europe__Zaporozhye,
            Tz::Europe__Zurich => Self::Europe__Zurich,
            Tz::GB => Self::GB,
            Tz::GBEire => Self::GBEire,
            Tz::GMT => Self::GMT,
            Tz::GMTPlus0 => Self::GMTPlus0,
            Tz::GMTMinus0 => Self::GMTMinus0,
            Tz::GMT0 => Self::GMT0,
            Tz::Greenwich => Self::Greenwich,
            Tz::HST => Self::HST,
            Tz::Hongkong => Self::Hongkong,
            Tz::Iceland => Self::Iceland,
            Tz::Indian__Antananarivo => Self::Indian__Antananarivo,
            Tz::Indian__Chagos => Self::Indian__Chagos,
            Tz::Indian__Christmas => Self::Indian__Christmas,
            Tz::Indian__Cocos => Self::Indian__Cocos,
            Tz::Indian__Comoro => Self::Indian__Comoro,
            Tz::Indian__Kerguelen => Self::Indian__Kerguelen,
            Tz::Indian__Mahe => Self::Indian__Mahe,
            Tz::Indian__Maldives => Self::Indian__Maldives,
            Tz::Indian__Mauritius => Self::Indian__Mauritius,
            Tz::Indian__Mayotte => Self::Indian__Mayotte,
            Tz::Indian__Reunion => Self::Indian__Reunion,
            Tz::Iran => Self::Iran,
            Tz::Israel => Self::Israel,
            Tz::Jamaica => Self::Jamaica,
            Tz::Japan => Self::Japan,
            Tz::Kwajalein => Self::Kwajalein,
            Tz::Libya => Self::Libya,
            Tz::MET => Self::MET,
            Tz::MST => Self::MST,
            Tz::MST7MDT => Self::MST7MDT,
            Tz::Mexico__BajaNorte => Self::Mexico__BajaNorte,
            Tz::Mexico__BajaSur => Self::Mexico__BajaSur,
            Tz::Mexico__General => Self::Mexico__General,
            Tz::NZ => Self::NZ,
            Tz::NZCHAT => Self::NZCHAT,
            Tz::Navajo => Self::Navajo,
            Tz::PRC => Self::PRC,
            Tz::PST8PDT => Self::PST8PDT,
            Tz::Pacific__Apia => Self::Pacific__Apia,
            Tz::Pacific__Auckland => Self::Pacific__Auckland,
            Tz::Pacific__Bougainville => Self::Pacific__Bougainville,
            Tz::Pacific__Chatham => Self::Pacific__Chatham,
            Tz::Pacific__Chuuk => Self::Pacific__Chuuk,
            Tz::Pacific__Easter => Self::Pacific__Easter,
            Tz::Pacific__Efate => Self::Pacific__Efate,
            Tz::Pacific__Enderbury => Self::Pacific__Enderbury,
            Tz::Pacific__Fakaofo => Self::Pacific__Fakaofo,
            Tz::Pacific__Fiji => Self::Pacific__Fiji,
            Tz::Pacific__Funafuti => Self::Pacific__Funafuti,
            Tz::Pacific__Galapagos => Self::Pacific__Galapagos,
            Tz::Pacific__Gambier => Self::Pacific__Gambier,
            Tz::Pacific__Guadalcanal => Self::Pacific__Guadalcanal,
            Tz::Pacific__Guam => Self::Pacific__Guam,
            Tz::Pacific__Honolulu => Self::Pacific__Honolulu,
            Tz::Pacific__Johnston => Self::Pacific__Johnston,
            Tz::Pacific__Kanton => Self::Pacific__Kanton,
            Tz::Pacific__Kiritimati => Self::Pacific__Kiritimati,
            Tz::Pacific__Kosrae => Self::Pacific__Kosrae,
            Tz::Pacific__Kwajalein => Self::Pacific__Kwajalein,
            Tz::Pacific__Majuro => Self::Pacific__Majuro,
            Tz::Pacific__Marquesas => Self::Pacific__Marquesas,
            Tz::Pacific__Midway => Self::Pacific__Midway,
            Tz::Pacific__Nauru => Self::Pacific__Nauru,
            Tz::Pacific__Niue => Self::Pacific__Niue,
            Tz::Pacific__Norfolk => Self::Pacific__Norfolk,
            Tz::Pacific__Noumea => Self::Pacific__Noumea,
            Tz::Pacific__Pago_Pago => Self::Pacific__Pago_Pago,
            Tz::Pacific__Palau => Self::Pacific__Palau,
            Tz::Pacific__Pitcairn => Self::Pacific__Pitcairn,
            Tz::Pacific__Pohnpei => Self::Pacific__Pohnpei,
            Tz::Pacific__Ponape => Self::Pacific__Ponape,
            Tz::Pacific__Port_Moresby => Self::Pacific__Port_Moresby,
            Tz::Pacific__Rarotonga => Self::Pacific__Rarotonga,
            Tz::Pacific__Saipan => Self::Pacific__Saipan,
            Tz::Pacific__Samoa => Self::Pacific__Samoa,
            Tz::Pacific__Tahiti => Self::Pacific__Tahiti,
            Tz::Pacific__Tarawa => Self::Pacific__Tarawa,
            Tz::Pacific__Tongatapu => Self::Pacific__Tongatapu,
            Tz::Pacific__Truk => Self::Pacific__Truk,
            Tz::Pacific__Wake => Self::Pacific__Wake,
            Tz::Pacific__Wallis => Self::Pacific__Wallis,
            Tz::Pacific__Yap => Self::Pacific__Yap,
            Tz::Poland => Self::Poland,
            Tz::Portugal => Self::Portugal,
            Tz::ROC => Self::ROC,
            Tz::ROK => Self::ROK,
            Tz::Singapore => Self::Singapore,
            Tz::Turkey => Self::Turkey,
            Tz::UCT => Self::UCT,
            Tz::US__Alaska => Self::US__Alaska,
            Tz::US__Aleutian => Self::US__Aleutian,
            Tz::US__Arizona => Self::US__Arizona,
            Tz::US__Central => Self::US__Central,
            Tz::US__EastIndiana => Self::US__EastIndiana,
            Tz::US__Eastern => Self::US__Eastern,
            Tz::US__Hawaii => Self::US__Hawaii,
            Tz::US__IndianaStarke => Self::US__IndianaStarke,
            Tz::US__Michigan => Self::US__Michigan,
            Tz::US__Mountain => Self::US__Mountain,
            Tz::US__Pacific => Self::US__Pacific,
            Tz::US__Samoa => Self::US__Samoa,
            Tz::UTC => Self::UTC,
            Tz::Universal => Self::Universal,
            Tz::WSU => Self::WSU,
            Tz::WET => Self::WET,
            Tz::Zulu => Self::Zulu,
        }
    }
}
