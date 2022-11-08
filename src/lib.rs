use postgres_types::{FromSql, ToSql};

mod from;
mod into;

/// A timezone type that can be converted to and from a custom Postgres type.
///
/// Like [`chrono_tz::Tz`](chrono_tz::Tz), but is [`ToSql`](postgres_types::ToSql) and [`FromSql`](postgres_types::FromSql).
///
/// The custom Postgres type `tz` can be found [here](https://github.com/allan2/chrono-tz-postgres/blob/main/tz.sql).
#[allow(non_camel_case_types)]
#[derive(ToSql, FromSql, Debug)]
#[postgres(name = "tz")]
pub enum TzPg {
    /// Africa/Abidjan
    #[postgres(name = "Africa/Abidjan")]
    Africa__Abidjan,
    /// Africa/Accra
    #[postgres(name = "Africa/Accra")]
    Africa__Accra,
    /// Africa/Addis_Ababa
    #[postgres(name = "Africa/Addis_Ababa")]
    Africa__Addis_Ababa,
    /// Africa/Algiers
    #[postgres(name = "Africa/Algiers")]
    Africa__Algiers,
    /// Africa/Asmara
    #[postgres(name = "Africa/Asmara")]
    Africa__Asmara,
    /// Africa/Asmera
    #[postgres(name = "Africa/Asmera")]
    Africa__Asmera,
    /// Africa/Bamako
    #[postgres(name = "Africa/Bamako")]
    Africa__Bamako,
    /// Africa/Bangui
    #[postgres(name = "Africa/Bangui")]
    Africa__Bangui,
    /// Africa/Banjul
    #[postgres(name = "Africa/Banjul")]
    Africa__Banjul,
    /// Africa/Bissau
    #[postgres(name = "Africa/Bissau")]
    Africa__Bissau,
    /// Africa/Blantyre
    #[postgres(name = "Africa/Blantyre")]
    Africa__Blantyre,
    /// Africa/Brazzaville
    #[postgres(name = "Africa/Brazzaville")]
    Africa__Brazzaville,
    /// Africa/Bujumbura
    #[postgres(name = "Africa/Bujumbura")]
    Africa__Bujumbura,
    /// Africa/Cairo
    #[postgres(name = "Africa/Cairo")]
    Africa__Cairo,
    /// Africa/Casablanca
    #[postgres(name = "Africa/Casablanca")]
    Africa__Casablanca,
    /// Africa/Ceuta
    #[postgres(name = "Africa/Ceuta")]
    Africa__Ceuta,
    /// Africa/Conakry
    #[postgres(name = "Africa/Conakry")]
    Africa__Conakry,
    /// Africa/Dakar
    #[postgres(name = "Africa/Dakar")]
    Africa__Dakar,
    /// Africa/Dar_es_Salaam
    #[postgres(name = "Africa/Dar_es_Salaam")]
    Africa__Dar_es_Salaam,
    /// Africa/Djibouti
    #[postgres(name = "Africa/Djibouti")]
    Africa__Djibouti,
    /// Africa/Douala
    #[postgres(name = "Africa/Douala")]
    Africa__Douala,
    /// Africa/El_Aaiun
    #[postgres(name = "Africa/El_Aaiun")]
    Africa__El_Aaiun,
    /// Africa/Freetown
    #[postgres(name = "Africa/Freetown")]
    Africa__Freetown,
    /// Africa/Gaborone
    #[postgres(name = "Africa/Gaborone")]
    Africa__Gaborone,
    /// Africa/Harare
    #[postgres(name = "Africa/Harare")]
    Africa__Harare,
    /// Africa/Johannesburg
    #[postgres(name = "Africa/Johannesburg")]
    Africa__Johannesburg,
    /// Africa/Juba
    #[postgres(name = "Africa/Juba")]
    Africa__Juba,
    /// Africa/Kampala
    #[postgres(name = "Africa/Kampala")]
    Africa__Kampala,
    /// Africa/Khartoum
    #[postgres(name = "Africa/Khartoum")]
    Africa__Khartoum,
    /// Africa/Kigali
    #[postgres(name = "Africa/Kigali")]
    Africa__Kigali,
    /// Africa/Kinshasa
    #[postgres(name = "Africa/Kinshasa")]
    Africa__Kinshasa,
    /// Africa/Lagos
    #[postgres(name = "Africa/Lagos")]
    Africa__Lagos,
    /// Africa/Libreville
    #[postgres(name = "Africa/Libreville")]
    Africa__Libreville,
    /// Africa/Lome
    #[postgres(name = "Africa/Lome")]
    Africa__Lome,
    /// Africa/Luanda
    #[postgres(name = "Africa/Luanda")]
    Africa__Luanda,
    /// Africa/Lubumbashi
    #[postgres(name = "Africa/Lubumbashi")]
    Africa__Lubumbashi,
    /// Africa/Lusaka
    #[postgres(name = "Africa/Lusaka")]
    Africa__Lusaka,
    /// Africa/Malabo
    #[postgres(name = "Africa/Malabo")]
    Africa__Malabo,
    /// Africa/Maputo
    #[postgres(name = "Africa/Maputo")]
    Africa__Maputo,
    /// Africa/Maseru
    #[postgres(name = "Africa/Maseru")]
    Africa__Maseru,
    /// Africa/Mbabane
    #[postgres(name = "Africa/Mbabane")]
    Africa__Mbabane,
    /// Africa/Mogadishu
    #[postgres(name = "Africa/Mogadishu")]
    Africa__Mogadishu,
    /// Africa/Monrovia
    #[postgres(name = "Africa/Monrovia")]
    Africa__Monrovia,
    /// Africa/Nairobi
    #[postgres(name = "Africa/Nairobi")]
    Africa__Nairobi,
    /// Africa/Ndjamena
    #[postgres(name = "Africa/Ndjamena")]
    Africa__Ndjamena,
    /// Africa/Niamey
    #[postgres(name = "Africa/Niamey")]
    Africa__Niamey,
    /// Africa/Nouakchott
    #[postgres(name = "Africa/Nouakchott")]
    Africa__Nouakchott,
    /// Africa/Ouagadougou
    #[postgres(name = "Africa/Ouagadougou")]
    Africa__Ouagadougou,
    /// Africa/Porto-Novo
    #[postgres(name = "Africa/Porto-Novo")]
    Africa__PortoNovo,
    /// Africa/Sao_Tome
    #[postgres(name = "Africa/Sao_Tome")]
    Africa__Sao_Tome,
    /// Africa/Timbuktu
    #[postgres(name = "Africa/Timbuktu")]
    Africa__Timbuktu,
    /// Africa/Tripoli
    #[postgres(name = "Africa/Tripoli")]
    Africa__Tripoli,
    /// Africa/Tunis
    #[postgres(name = "Africa/Tunis")]
    Africa__Tunis,
    /// Africa/Windhoek
    #[postgres(name = "Africa/Windhoek")]
    Africa__Windhoek,
    /// America/Adak
    #[postgres(name = "America/Adak")]
    America__Adak,
    /// America/Anchorage
    #[postgres(name = "America/Anchorage")]
    America__Anchorage,
    /// America/Anguilla
    #[postgres(name = "America/Anguilla")]
    America__Anguilla,
    /// America/Antigua
    #[postgres(name = "America/Antigua")]
    America__Antigua,
    /// America/Araguaina
    #[postgres(name = "America/Araguaina")]
    America__Araguaina,
    /// America/Argentina/Buenos_Aires
    #[postgres(name = "America/Argentina/Buenos_Aires")]
    America__Argentina__Buenos_Aires,
    /// America/Argentina/Catamarca
    #[postgres(name = "America/Argentina/Catamarca")]
    America__Argentina__Catamarca,
    /// America/Argentina/ComodRivadavia
    #[postgres(name = "America/Argentina/ComodRivadavia")]
    America__Argentina__ComodRivadavia,
    /// America/Argentina/Cordoba
    #[postgres(name = "America/Argentina/Cordoba")]
    America__Argentina__Cordoba,
    /// America/Argentina/Jujuy
    #[postgres(name = "America/Argentina/Jujuy")]
    America__Argentina__Jujuy,
    /// America/Argentina/La_Rioja
    #[postgres(name = "America/Argentina/La_Rioja")]
    America__Argentina__La_Rioja,
    /// America/Argentina/Mendoza
    #[postgres(name = "America/Argentina/Mendoza")]
    America__Argentina__Mendoza,
    /// America/Argentina/Rio_Gallegos
    #[postgres(name = "America/Argentina/Rio_Gallegos")]
    America__Argentina__Rio_Gallegos,
    /// America/Argentina/Salta
    #[postgres(name = "America/Argentina/Salta")]
    America__Argentina__Salta,
    /// America/Argentina/San_Juan
    #[postgres(name = "America/Argentina/San_Juan")]
    America__Argentina__San_Juan,
    /// America/Argentina/San_Luis
    #[postgres(name = "America/Argentina/San_Luis")]
    America__Argentina__San_Luis,
    /// America/Argentina/Tucuman
    #[postgres(name = "America/Argentina/Tucuman")]
    America__Argentina__Tucuman,
    /// America/Argentina/Ushuaia
    #[postgres(name = "America/Argentina/Ushuaia")]
    America__Argentina__Ushuaia,
    /// America/Aruba
    #[postgres(name = "America/Aruba")]
    America__Aruba,
    /// America/Asuncion
    #[postgres(name = "America/Asuncion")]
    America__Asuncion,
    /// America/Atikokan
    #[postgres(name = "America/Atikokan")]
    America__Atikokan,
    /// America/Atka
    #[postgres(name = "America/Atka")]
    America__Atka,
    /// America/Bahia
    #[postgres(name = "America/Bahia")]
    America__Bahia,
    /// America/Bahia_Banderas
    #[postgres(name = "America/Bahia_Banderas")]
    America__Bahia_Banderas,
    /// America/Barbados
    #[postgres(name = "America/Barbados")]
    America__Barbados,
    /// America/Belem
    #[postgres(name = "America/Belem")]
    America__Belem,
    /// America/Belize
    #[postgres(name = "America/Belize")]
    America__Belize,
    /// America/Blanc-Sablon
    #[postgres(name = "America/Blanc-Sablon")]
    America__BlancSablon,
    /// America/Boa_Vista
    #[postgres(name = "America/Boa_Vista")]
    America__Boa_Vista,
    /// America/Bogota
    #[postgres(name = "America/Bogota")]
    America__Bogota,
    /// America/Boise
    #[postgres(name = "America/Boise")]
    America__Boise,
    /// America/Buenos_Aires
    #[postgres(name = "America/Buenos_Aires")]
    America__Buenos_Aires,
    /// America/Cambridge_Bay
    #[postgres(name = "America/Cambridge_Bay")]
    America__Cambridge_Bay,
    /// America/Campo_Grande
    #[postgres(name = "America/Campo_Grande")]
    America__Campo_Grande,
    /// America/Cancun
    #[postgres(name = "America/Cancun")]
    America__Cancun,
    /// America/Caracas
    #[postgres(name = "America/Caracas")]
    America__Caracas,
    /// America/Catamarca
    #[postgres(name = "America/Catamarca")]
    America__Catamarca,
    /// America/Cayenne
    #[postgres(name = "America/Cayenne")]
    America__Cayenne,
    /// America/Cayman
    #[postgres(name = "America/Cayman")]
    America__Cayman,
    /// America/Chicago
    #[postgres(name = "America/Chicago")]
    America__Chicago,
    /// America/Chihuahua
    #[postgres(name = "America/Chihuahua")]
    America__Chihuahua,
    /// America/Coral_Harbour
    #[postgres(name = "America/Coral_Harbour")]
    America__Coral_Harbour,
    /// America/Cordoba
    #[postgres(name = "America/Cordoba")]
    America__Cordoba,
    /// America/Costa_Rica
    #[postgres(name = "America/Costa_Rica")]
    America__Costa_Rica,
    /// America/Creston
    #[postgres(name = "America/Creston")]
    America__Creston,
    /// America/Cuiaba
    #[postgres(name = "America/Cuiaba")]
    America__Cuiaba,
    /// America/Curacao
    #[postgres(name = "America/Curacao")]
    America__Curacao,
    /// America/Danmarkshavn
    #[postgres(name = "America/Danmarkshavn")]
    America__Danmarkshavn,
    /// America/Dawson
    #[postgres(name = "America/Dawson")]
    America__Dawson,
    /// America/Dawson_Creek
    #[postgres(name = "America/Dawson_Creek")]
    America__Dawson_Creek,
    /// America/Denver
    #[postgres(name = "America/Denver")]
    America__Denver,
    /// America/Detroit
    #[postgres(name = "America/Detroit")]
    America__Detroit,
    /// America/Dominica
    #[postgres(name = "America/Dominica")]
    America__Dominica,
    /// America/Edmonton
    #[postgres(name = "America/Edmonton")]
    America__Edmonton,
    /// America/Eirunepe
    #[postgres(name = "America/Eirunepe")]
    America__Eirunepe,
    /// America/El_Salvador
    #[postgres(name = "America/El_Salvador")]
    America__El_Salvador,
    /// America/Ensenada
    #[postgres(name = "America/Ensenada")]
    America__Ensenada,
    /// America/Fort_Nelson
    #[postgres(name = "America/Fort_Nelson")]
    America__Fort_Nelson,
    /// America/Fort_Wayne
    #[postgres(name = "America/Fort_Wayne")]
    America__Fort_Wayne,
    /// America/Fortaleza
    #[postgres(name = "America/Fortaleza")]
    America__Fortaleza,
    /// America/Glace_Bay
    #[postgres(name = "America/Glace_Bay")]
    America__Glace_Bay,
    /// America/Godthab
    #[postgres(name = "America/Godthab")]
    America__Godthab,
    /// America/Goose_Bay
    #[postgres(name = "America/Goose_Bay")]
    America__Goose_Bay,
    /// America/Grand_Turk
    #[postgres(name = "America/Grand_Turk")]
    America__Grand_Turk,
    /// America/Grenada
    #[postgres(name = "America/Grenada")]
    America__Grenada,
    /// America/Guadeloupe
    #[postgres(name = "America/Guadeloupe")]
    America__Guadeloupe,
    /// America/Guatemala
    #[postgres(name = "America/Guatemala")]
    America__Guatemala,
    /// America/Guayaquil
    #[postgres(name = "America/Guayaquil")]
    America__Guayaquil,
    /// America/Guyana
    #[postgres(name = "America/Guyana")]
    America__Guyana,
    /// America/Halifax
    #[postgres(name = "America/Halifax")]
    America__Halifax,
    /// America/Havana
    #[postgres(name = "America/Havana")]
    America__Havana,
    /// America/Hermosillo
    #[postgres(name = "America/Hermosillo")]
    America__Hermosillo,
    /// America/Indiana/Indianapolis
    #[postgres(name = "America/Indiana/Indianapolis")]
    America__Indiana__Indianapolis,
    /// America/Indiana/Knox
    #[postgres(name = "America/Indiana/Knox")]
    America__Indiana__Knox,
    /// America/Indiana/Marengo
    #[postgres(name = "America/Indiana/Marengo")]
    America__Indiana__Marengo,
    /// America/Indiana/Petersburg
    #[postgres(name = "America/Indiana/Petersburg")]
    America__Indiana__Petersburg,
    /// America/Indiana/Tell_City
    #[postgres(name = "America/Indiana/Tell_City")]
    America__Indiana__Tell_City,
    /// America/Indiana/Vevay
    #[postgres(name = "America/Indiana/Vevay")]
    America__Indiana__Vevay,
    /// America/Indiana/Vincennes
    #[postgres(name = "America/Indiana/Vincennes")]
    America__Indiana__Vincennes,
    /// America/Indiana/Winamac
    #[postgres(name = "America/Indiana/Winamac")]
    America__Indiana__Winamac,
    /// America/Indianapolis
    #[postgres(name = "America/Indianapolis")]
    America__Indianapolis,
    /// America/Inuvik
    #[postgres(name = "America/Inuvik")]
    America__Inuvik,
    /// America/Iqaluit
    #[postgres(name = "America/Iqaluit")]
    America__Iqaluit,
    /// America/Jamaica
    #[postgres(name = "America/Jamaica")]
    America__Jamaica,
    /// America/Jujuy
    #[postgres(name = "America/Jujuy")]
    America__Jujuy,
    /// America/Juneau
    #[postgres(name = "America/Juneau")]
    America__Juneau,
    /// America/Kentucky/Louisville
    #[postgres(name = "America/Kentucky/Louisville")]
    America__Kentucky__Louisville,
    /// America/Kentucky/Monticello
    #[postgres(name = "America/Kentucky/Monticello")]
    America__Kentucky__Monticello,
    /// America/Knox_IN
    #[postgres(name = "America/Knox_IN")]
    America__Knox_IN,
    /// America/Kralendijk
    #[postgres(name = "America/Kralendijk")]
    America__Kralendijk,
    /// America/La_Paz
    #[postgres(name = "America/La_Paz")]
    America__La_Paz,
    /// America/Lima
    #[postgres(name = "America/Lima")]
    America__Lima,
    /// America/Los_Angeles
    #[postgres(name = "America/Los_Angeles")]
    America__Los_Angeles,
    /// America/Louisville
    #[postgres(name = "America/Louisville")]
    America__Louisville,
    /// America/Lower_Princes
    #[postgres(name = "America/Lower_Princes")]
    America__Lower_Princes,
    /// America/Maceio
    #[postgres(name = "America/Maceio")]
    America__Maceio,
    /// America/Managua
    #[postgres(name = "America/Managua")]
    America__Managua,
    /// America/Manaus
    #[postgres(name = "America/Manaus")]
    America__Manaus,
    /// America/Marigot
    #[postgres(name = "America/Marigot")]
    America__Marigot,
    /// America/Martinique
    #[postgres(name = "America/Martinique")]
    America__Martinique,
    /// America/Matamoros
    #[postgres(name = "America/Matamoros")]
    America__Matamoros,
    /// America/Mazatlan
    #[postgres(name = "America/Mazatlan")]
    America__Mazatlan,
    /// America/Mendoza
    #[postgres(name = "America/Mendoza")]
    America__Mendoza,
    /// America/Menominee
    #[postgres(name = "America/Menominee")]
    America__Menominee,
    /// America/Merida
    #[postgres(name = "America/Merida")]
    America__Merida,
    /// America/Metlakatla
    #[postgres(name = "America/Metlakatla")]
    America__Metlakatla,
    /// America/Mexico_City
    #[postgres(name = "America/Mexico_City")]
    America__Mexico_City,
    /// America/Miquelon
    #[postgres(name = "America/Miquelon")]
    America__Miquelon,
    /// America/Moncton
    #[postgres(name = "America/Moncton")]
    America__Moncton,
    /// America/Monterrey
    #[postgres(name = "America/Monterrey")]
    America__Monterrey,
    /// America/Montevideo
    #[postgres(name = "America/Montevideo")]
    America__Montevideo,
    /// America/Montreal
    #[postgres(name = "America/Montreal")]
    America__Montreal,
    /// America/Montserrat
    #[postgres(name = "America/Montserrat")]
    America__Montserrat,
    /// America/Nassau
    #[postgres(name = "America/Nassau")]
    America__Nassau,
    /// America/New_York
    #[postgres(name = "America/New_York")]
    America__New_York,
    /// America/Nipigon
    #[postgres(name = "America/Nipigon")]
    America__Nipigon,
    /// America/Nome
    #[postgres(name = "America/Nome")]
    America__Nome,
    /// America/Noronha
    #[postgres(name = "America/Noronha")]
    America__Noronha,
    /// America/North_Dakota/Beulah
    #[postgres(name = "America/North_Dakota/Beulah")]
    America__North_Dakota__Beulah,
    /// America/North_Dakota/Center
    #[postgres(name = "America/North_Dakota/Center")]
    America__North_Dakota__Center,
    /// America/North_Dakota/New_Salem
    #[postgres(name = "America/North_Dakota/New_Salem")]
    America__North_Dakota__New_Salem,
    /// America/Nuuk
    #[postgres(name = "America/Nuuk")]
    America__Nuuk,
    /// America/Ojinaga
    #[postgres(name = "America/Ojinaga")]
    America__Ojinaga,
    /// America/Panama
    #[postgres(name = "America/Panama")]
    America__Panama,
    /// America/Pangnirtung
    #[postgres(name = "America/Pangnirtung")]
    America__Pangnirtung,
    /// America/Paramaribo
    #[postgres(name = "America/Paramaribo")]
    America__Paramaribo,
    /// America/Phoenix
    #[postgres(name = "America/Phoenix")]
    America__Phoenix,
    /// America/Port-au-Prince
    #[postgres(name = "America/Port-au-Prince")]
    America__PortauPrince,
    /// America/Port_of_Spain
    #[postgres(name = "America/Port_of_Spain")]
    America__Port_of_Spain,
    /// America/Porto_Acre
    #[postgres(name = "America/Porto_Acre")]
    America__Porto_Acre,
    /// America/Porto_Velho
    #[postgres(name = "America/Porto_Velho")]
    America__Porto_Velho,
    /// America/Puerto_Rico
    #[postgres(name = "America/Puerto_Rico")]
    America__Puerto_Rico,
    /// America/Punta_Arenas
    #[postgres(name = "America/Punta_Arenas")]
    America__Punta_Arenas,
    /// America/Rainy_River
    #[postgres(name = "America/Rainy_River")]
    America__Rainy_River,
    /// America/Rankin_Inlet
    #[postgres(name = "America/Rankin_Inlet")]
    America__Rankin_Inlet,
    /// America/Recife
    #[postgres(name = "America/Recife")]
    America__Recife,
    /// America/Regina
    #[postgres(name = "America/Regina")]
    America__Regina,
    /// America/Resolute
    #[postgres(name = "America/Resolute")]
    America__Resolute,
    /// America/Rio_Branco
    #[postgres(name = "America/Rio_Branco")]
    America__Rio_Branco,
    /// America/Rosario
    #[postgres(name = "America/Rosario")]
    America__Rosario,
    /// America/Santa_Isabel
    #[postgres(name = "America/Santa_Isabel")]
    America__Santa_Isabel,
    /// America/Santarem
    #[postgres(name = "America/Santarem")]
    America__Santarem,
    /// America/Santiago
    #[postgres(name = "America/Santiago")]
    America__Santiago,
    /// America/Santo_Domingo
    #[postgres(name = "America/Santo_Domingo")]
    America__Santo_Domingo,
    /// America/Sao_Paulo
    #[postgres(name = "America/Sao_Paulo")]
    America__Sao_Paulo,
    /// America/Scoresbysund
    #[postgres(name = "America/Scoresbysund")]
    America__Scoresbysund,
    /// America/Shiprock
    #[postgres(name = "America/Shiprock")]
    America__Shiprock,
    /// America/Sitka
    #[postgres(name = "America/Sitka")]
    America__Sitka,
    /// America/St_Barthelemy
    #[postgres(name = "America/St_Barthelemy")]
    America__St_Barthelemy,
    /// America/St_Johns
    #[postgres(name = "America/St_Johns")]
    America__St_Johns,
    /// America/St_Kitts
    #[postgres(name = "America/St_Kitts")]
    America__St_Kitts,
    /// America/St_Lucia
    #[postgres(name = "America/St_Lucia")]
    America__St_Lucia,
    /// America/St_Thomas
    #[postgres(name = "America/St_Thomas")]
    America__St_Thomas,
    /// America/St_Vincent
    #[postgres(name = "America/St_Vincent")]
    America__St_Vincent,
    /// America/Swift_Current
    #[postgres(name = "America/Swift_Current")]
    America__Swift_Current,
    /// America/Tegucigalpa
    #[postgres(name = "America/Tegucigalpa")]
    America__Tegucigalpa,
    /// America/Thule
    #[postgres(name = "America/Thule")]
    America__Thule,
    /// America/Thunder_Bay
    #[postgres(name = "America/Thunder_Bay")]
    America__Thunder_Bay,
    /// America/Tijuana
    #[postgres(name = "America/Tijuana")]
    America__Tijuana,
    /// America/Toronto
    #[postgres(name = "America/Toronto")]
    America__Toronto,
    /// America/Tortola
    #[postgres(name = "America/Tortola")]
    America__Tortola,
    /// America/Vancouver
    #[postgres(name = "America/Vancouver")]
    America__Vancouver,
    /// America/Virgin
    #[postgres(name = "America/Virgin")]
    America__Virgin,
    /// America/Whitehorse
    #[postgres(name = "America/Whitehorse")]
    America__Whitehorse,
    /// America/Winnipeg
    #[postgres(name = "America/Winnipeg")]
    America__Winnipeg,
    /// America/Yakutat
    #[postgres(name = "America/Yakutat")]
    America__Yakutat,
    /// America/Yellowknife
    #[postgres(name = "America/Yellowknife")]
    America__Yellowknife,
    /// Antarctica/Casey
    #[postgres(name = "Antarctica/Casey")]
    Antarctica__Casey,
    /// Antarctica/Davis
    #[postgres(name = "Antarctica/Davis")]
    Antarctica__Davis,
    /// Antarctica/DumontDUrville
    #[postgres(name = "Antarctica/DumontDUrville")]
    Antarctica__DumontDUrville,
    /// Antarctica/Macquarie
    #[postgres(name = "Antarctica/Macquarie")]
    Antarctica__Macquarie,
    /// Antarctica/Mawson
    #[postgres(name = "Antarctica/Mawson")]
    Antarctica__Mawson,
    /// Antarctica/McMurdo
    #[postgres(name = "Antarctica/McMurdo")]
    Antarctica__McMurdo,
    /// Antarctica/Palmer
    #[postgres(name = "Antarctica/Palmer")]
    Antarctica__Palmer,
    /// Antarctica/Rothera
    #[postgres(name = "Antarctica/Rothera")]
    Antarctica__Rothera,
    /// Antarctica/South_Pole
    #[postgres(name = "Antarctica/South_Pole")]
    Antarctica__South_Pole,
    /// Antarctica/Syowa
    #[postgres(name = "Antarctica/Syowa")]
    Antarctica__Syowa,
    /// Antarctica/Troll
    #[postgres(name = "Antarctica/Troll")]
    Antarctica__Troll,
    /// Antarctica/Vostok
    #[postgres(name = "Antarctica/Vostok")]
    Antarctica__Vostok,
    /// Arctic/Longyearbyen
    #[postgres(name = "Arctic/Longyearbyen")]
    Arctic__Longyearbyen,
    /// Asia/Aden
    #[postgres(name = "Asia/Aden")]
    Asia__Aden,
    /// Asia/Almaty
    #[postgres(name = "Asia/Almaty")]
    Asia__Almaty,
    /// Asia/Amman
    #[postgres(name = "Asia/Amman")]
    Asia__Amman,
    /// Asia/Anadyr
    #[postgres(name = "Asia/Anadyr")]
    Asia__Anadyr,
    /// Asia/Aqtau
    #[postgres(name = "Asia/Aqtau")]
    Asia__Aqtau,
    /// Asia/Aqtobe
    #[postgres(name = "Asia/Aqtobe")]
    Asia__Aqtobe,
    /// Asia/Ashgabat
    #[postgres(name = "Asia/Ashgabat")]
    Asia__Ashgabat,
    /// Asia/Ashkhabad
    #[postgres(name = "Asia/Ashkhabad")]
    Asia__Ashkhabad,
    /// Asia/Atyrau
    #[postgres(name = "Asia/Atyrau")]
    Asia__Atyrau,
    /// Asia/Baghdad
    #[postgres(name = "Asia/Baghdad")]
    Asia__Baghdad,
    /// Asia/Bahrain
    #[postgres(name = "Asia/Bahrain")]
    Asia__Bahrain,
    /// Asia/Baku
    #[postgres(name = "Asia/Baku")]
    Asia__Baku,
    /// Asia/Bangkok
    #[postgres(name = "Asia/Bangkok")]
    Asia__Bangkok,
    /// Asia/Barnaul
    #[postgres(name = "Asia/Barnaul")]
    Asia__Barnaul,
    /// Asia/Beirut
    #[postgres(name = "Asia/Beirut")]
    Asia__Beirut,
    /// Asia/Bishkek
    #[postgres(name = "Asia/Bishkek")]
    Asia__Bishkek,
    /// Asia/Brunei
    #[postgres(name = "Asia/Brunei")]
    Asia__Brunei,
    /// Asia/Calcutta
    #[postgres(name = "Asia/Calcutta")]
    Asia__Calcutta,
    /// Asia/Chita
    #[postgres(name = "Asia/Chita")]
    Asia__Chita,
    /// Asia/Choibalsan
    #[postgres(name = "Asia/Choibalsan")]
    Asia__Choibalsan,
    /// Asia/Chongqing
    #[postgres(name = "Asia/Chongqing")]
    Asia__Chongqing,
    /// Asia/Chungking
    #[postgres(name = "Asia/Chungking")]
    Asia__Chungking,
    /// Asia/Colombo
    #[postgres(name = "Asia/Colombo")]
    Asia__Colombo,
    /// Asia/Dacca
    #[postgres(name = "Asia/Dacca")]
    Asia__Dacca,
    /// Asia/Damascus
    #[postgres(name = "Asia/Damascus")]
    Asia__Damascus,
    /// Asia/Dhaka
    #[postgres(name = "Asia/Dhaka")]
    Asia__Dhaka,
    /// Asia/Dili
    #[postgres(name = "Asia/Dili")]
    Asia__Dili,
    /// Asia/Dubai
    #[postgres(name = "Asia/Dubai")]
    Asia__Dubai,
    /// Asia/Dushanbe
    #[postgres(name = "Asia/Dushanbe")]
    Asia__Dushanbe,
    /// Asia/Famagusta
    #[postgres(name = "Asia/Famagusta")]
    Asia__Famagusta,
    /// Asia/Gaza
    #[postgres(name = "Asia/Gaza")]
    Asia__Gaza,
    /// Asia/Harbin
    #[postgres(name = "Asia/Harbin")]
    Asia__Harbin,
    /// Asia/Hebron
    #[postgres(name = "Asia/Hebron")]
    Asia__Hebron,
    /// Asia/Ho_Chi_Minh
    #[postgres(name = "Asia/Ho_Chi_Minh")]
    Asia__Ho_Chi_Minh,
    /// Asia/Hong_Kong
    #[postgres(name = "Asia/Hong_Kong")]
    Asia__Hong_Kong,
    /// Asia/Hovd
    #[postgres(name = "Asia/Hovd")]
    Asia__Hovd,
    /// Asia/Irkutsk
    #[postgres(name = "Asia/Irkutsk")]
    Asia__Irkutsk,
    /// Asia/Istanbul
    #[postgres(name = "Asia/Istanbul")]
    Asia__Istanbul,
    /// Asia/Jakarta
    #[postgres(name = "Asia/Jakarta")]
    Asia__Jakarta,
    /// Asia/Jayapura
    #[postgres(name = "Asia/Jayapura")]
    Asia__Jayapura,
    /// Asia/Jerusalem
    #[postgres(name = "Asia/Jerusalem")]
    Asia__Jerusalem,
    /// Asia/Kabul
    #[postgres(name = "Asia/Kabul")]
    Asia__Kabul,
    /// Asia/Kamchatka
    #[postgres(name = "Asia/Kamchatka")]
    Asia__Kamchatka,
    /// Asia/Karachi
    #[postgres(name = "Asia/Karachi")]
    Asia__Karachi,
    /// Asia/Kashgar
    #[postgres(name = "Asia/Kashgar")]
    Asia__Kashgar,
    /// Asia/Kathmandu
    #[postgres(name = "Asia/Kathmandu")]
    Asia__Kathmandu,
    /// Asia/Katmandu
    #[postgres(name = "Asia/Katmandu")]
    Asia__Katmandu,
    /// Asia/Khandyga
    #[postgres(name = "Asia/Khandyga")]
    Asia__Khandyga,
    /// Asia/Kolkata
    #[postgres(name = "Asia/Kolkata")]
    Asia__Kolkata,
    /// Asia/Krasnoyarsk
    #[postgres(name = "Asia/Krasnoyarsk")]
    Asia__Krasnoyarsk,
    /// Asia/Kuala_Lumpur
    #[postgres(name = "Asia/Kuala_Lumpur")]
    Asia__Kuala_Lumpur,
    /// Asia/Kuching
    #[postgres(name = "Asia/Kuching")]
    Asia__Kuching,
    /// Asia/Kuwait
    #[postgres(name = "Asia/Kuwait")]
    Asia__Kuwait,
    /// Asia/Macao
    #[postgres(name = "Asia/Macao")]
    Asia__Macao,
    /// Asia/Macau
    #[postgres(name = "Asia/Macau")]
    Asia__Macau,
    /// Asia/Magadan
    #[postgres(name = "Asia/Magadan")]
    Asia__Magadan,
    /// Asia/Makassar
    #[postgres(name = "Asia/Makassar")]
    Asia__Makassar,
    /// Asia/Manila
    #[postgres(name = "Asia/Manila")]
    Asia__Manila,
    /// Asia/Muscat
    #[postgres(name = "Asia/Muscat")]
    Asia__Muscat,
    /// Asia/Nicosia
    #[postgres(name = "Asia/Nicosia")]
    Asia__Nicosia,
    /// Asia/Novokuznetsk
    #[postgres(name = "Asia/Novokuznetsk")]
    Asia__Novokuznetsk,
    /// Asia/Novosibirsk
    #[postgres(name = "Asia/Novosibirsk")]
    Asia__Novosibirsk,
    /// Asia/Omsk
    #[postgres(name = "Asia/Omsk")]
    Asia__Omsk,
    /// Asia/Oral
    #[postgres(name = "Asia/Oral")]
    Asia__Oral,
    /// Asia/Phnom_Penh
    #[postgres(name = "Asia/Phnom_Penh")]
    Asia__Phnom_Penh,
    /// Asia/Pontianak
    #[postgres(name = "Asia/Pontianak")]
    Asia__Pontianak,
    /// Asia/Pyongyang
    #[postgres(name = "Asia/Pyongyang")]
    Asia__Pyongyang,
    /// Asia/Qatar
    #[postgres(name = "Asia/Qatar")]
    Asia__Qatar,
    /// Asia/Qostanay
    #[postgres(name = "Asia/Qostanay")]
    Asia__Qostanay,
    /// Asia/Qyzylorda
    #[postgres(name = "Asia/Qyzylorda")]
    Asia__Qyzylorda,
    /// Asia/Rangoon
    #[postgres(name = "Asia/Rangoon")]
    Asia__Rangoon,
    /// Asia/Riyadh
    #[postgres(name = "Asia/Riyadh")]
    Asia__Riyadh,
    /// Asia/Saigon
    #[postgres(name = "Asia/Saigon")]
    Asia__Saigon,
    /// Asia/Sakhalin
    #[postgres(name = "Asia/Sakhalin")]
    Asia__Sakhalin,
    /// Asia/Samarkand
    #[postgres(name = "Asia/Samarkand")]
    Asia__Samarkand,
    /// Asia/Seoul
    #[postgres(name = "Asia/Seoul")]
    Asia__Seoul,
    /// Asia/Shanghai
    #[postgres(name = "Asia/Shanghai")]
    Asia__Shanghai,
    /// Asia/Singapore
    #[postgres(name = "Asia/Singapore")]
    Asia__Singapore,
    /// Asia/Srednekolymsk
    #[postgres(name = "Asia/Srednekolymsk")]
    Asia__Srednekolymsk,
    /// Asia/Taipei
    #[postgres(name = "Asia/Taipei")]
    Asia__Taipei,
    /// Asia/Tashkent
    #[postgres(name = "Asia/Tashkent")]
    Asia__Tashkent,
    /// Asia/Tbilisi
    #[postgres(name = "Asia/Tbilisi")]
    Asia__Tbilisi,
    /// Asia/Tehran
    #[postgres(name = "Asia/Tehran")]
    Asia__Tehran,
    /// Asia/Tel_Aviv
    #[postgres(name = "Asia/Tel_Aviv")]
    Asia__Tel_Aviv,
    /// Asia/Thimbu
    #[postgres(name = "Asia/Thimbu")]
    Asia__Thimbu,
    /// Asia/Thimphu
    #[postgres(name = "Asia/Thimphu")]
    Asia__Thimphu,
    /// Asia/Tokyo
    #[postgres(name = "Asia/Tokyo")]
    Asia__Tokyo,
    /// Asia/Tomsk
    #[postgres(name = "Asia/Tomsk")]
    Asia__Tomsk,
    /// Asia/Ujung_Pandang
    #[postgres(name = "Asia/Ujung_Pandang")]
    Asia__Ujung_Pandang,
    /// Asia/Ulaanbaatar
    #[postgres(name = "Asia/Ulaanbaatar")]
    Asia__Ulaanbaatar,
    /// Asia/Ulan_Bator
    #[postgres(name = "Asia/Ulan_Bator")]
    Asia__Ulan_Bator,
    /// Asia/Urumqi
    #[postgres(name = "Asia/Urumqi")]
    Asia__Urumqi,
    /// Asia/Ust-Nera
    #[postgres(name = "Asia/Ust-Nera")]
    Asia__UstNera,
    /// Asia/Vientiane
    #[postgres(name = "Asia/Vientiane")]
    Asia__Vientiane,
    /// Asia/Vladivostok
    #[postgres(name = "Asia/Vladivostok")]
    Asia__Vladivostok,
    /// Asia/Yakutsk
    #[postgres(name = "Asia/Yakutsk")]
    Asia__Yakutsk,
    /// Asia/Yangon
    #[postgres(name = "Asia/Yangon")]
    Asia__Yangon,
    /// Asia/Yekaterinburg
    #[postgres(name = "Asia/Yekaterinburg")]
    Asia__Yekaterinburg,
    /// Asia/Yerevan
    #[postgres(name = "Asia/Yerevan")]
    Asia__Yerevan,
    /// Atlantic/Azores
    #[postgres(name = "Atlantic/Azores")]
    Atlantic__Azores,
    /// Atlantic/Bermuda
    #[postgres(name = "Atlantic/Bermuda")]
    Atlantic__Bermuda,
    /// Atlantic/Canary
    #[postgres(name = "Atlantic/Canary")]
    Atlantic__Canary,
    /// Atlantic/Cape_Verde
    #[postgres(name = "Atlantic/Cape_Verde")]
    Atlantic__Cape_Verde,
    /// Atlantic/Faeroe
    #[postgres(name = "Atlantic/Faeroe")]
    Atlantic__Faeroe,
    /// Atlantic/Faroe
    #[postgres(name = "Atlantic/Faroe")]
    Atlantic__Faroe,
    /// Atlantic/Jan_Mayen
    #[postgres(name = "Atlantic/Jan_Mayen")]
    Atlantic__Jan_Mayen,
    /// Atlantic/Madeira
    #[postgres(name = "Atlantic/Madeira")]
    Atlantic__Madeira,
    /// Atlantic/Reykjavik
    #[postgres(name = "Atlantic/Reykjavik")]
    Atlantic__Reykjavik,
    /// Atlantic/South_Georgia
    #[postgres(name = "Atlantic/South_Georgia")]
    Atlantic__South_Georgia,
    /// Atlantic/St_Helena
    #[postgres(name = "Atlantic/St_Helena")]
    Atlantic__St_Helena,
    /// Atlantic/Stanley
    #[postgres(name = "Atlantic/Stanley")]
    Atlantic__Stanley,
    /// Australia/ACT
    #[postgres(name = "Australia/ACT")]
    Australia__ACT,
    /// Australia/Adelaide
    #[postgres(name = "Australia/Adelaide")]
    Australia__Adelaide,
    /// Australia/Brisbane
    #[postgres(name = "Australia/Brisbane")]
    Australia__Brisbane,
    /// Australia/Broken_Hill
    #[postgres(name = "Australia/Broken_Hill")]
    Australia__Broken_Hill,
    /// Australia/Canberra
    #[postgres(name = "Australia/Canberra")]
    Australia__Canberra,
    /// Australia/Currie
    #[postgres(name = "Australia/Currie")]
    Australia__Currie,
    /// Australia/Darwin
    #[postgres(name = "Australia/Darwin")]
    Australia__Darwin,
    /// Australia/Eucla
    #[postgres(name = "Australia/Eucla")]
    Australia__Eucla,
    /// Australia/Hobart
    #[postgres(name = "Australia/Hobart")]
    Australia__Hobart,
    /// Australia/LHI
    #[postgres(name = "Australia/LHI")]
    Australia__LHI,
    /// Australia/Lindeman
    #[postgres(name = "Australia/Lindeman")]
    Australia__Lindeman,
    /// Australia/Lord_Howe
    #[postgres(name = "Australia/Lord_Howe")]
    Australia__Lord_Howe,
    /// Australia/Melbourne
    #[postgres(name = "Australia/Melbourne")]
    Australia__Melbourne,
    /// Australia/NSW
    #[postgres(name = "Australia/NSW")]
    Australia__NSW,
    /// Australia/North
    #[postgres(name = "Australia/North")]
    Australia__North,
    /// Australia/Perth
    #[postgres(name = "Australia/Perth")]
    Australia__Perth,
    /// Australia/Queensland
    #[postgres(name = "Australia/Queensland")]
    Australia__Queensland,
    /// Australia/South
    #[postgres(name = "Australia/South")]
    Australia__South,
    /// Australia/Sydney
    #[postgres(name = "Australia/Sydney")]
    Australia__Sydney,
    /// Australia/Tasmania
    #[postgres(name = "Australia/Tasmania")]
    Australia__Tasmania,
    /// Australia/Victoria
    #[postgres(name = "Australia/Victoria")]
    Australia__Victoria,
    /// Australia/West
    #[postgres(name = "Australia/West")]
    Australia__West,
    /// Australia/Yancowinna
    #[postgres(name = "Australia/Yancowinna")]
    Australia__Yancowinna,
    /// Brazil/Acre
    #[postgres(name = "Brazil/Acre")]
    Brazil__Acre,
    /// Brazil/DeNoronha
    #[postgres(name = "Brazil/DeNoronha")]
    Brazil__DeNoronha,
    /// Brazil/East
    #[postgres(name = "Brazil/East")]
    Brazil__East,
    /// Brazil/West
    #[postgres(name = "Brazil/West")]
    Brazil__West,
    /// CET
    CET,
    /// CST6CDT
    CST6CDT,
    /// Canada/Atlantic
    #[postgres(name = "Canada/Atlantic")]
    Canada__Atlantic,
    /// Canada/Central
    #[postgres(name = "Canada/Central")]
    Canada__Central,
    /// Canada/Eastern
    #[postgres(name = "Canada/Eastern")]
    Canada__Eastern,
    /// Canada/Mountain
    #[postgres(name = "Canada/Mountain")]
    Canada__Mountain,
    /// Canada/Newfoundland
    #[postgres(name = "Canada/Newfoundland")]
    Canada__Newfoundland,
    /// Canada/Pacific
    #[postgres(name = "Canada/Pacific")]
    Canada__Pacific,
    /// Canada/Saskatchewan
    #[postgres(name = "Canada/Saskatchewan")]
    Canada__Saskatchewan,
    /// Canada/Yukon
    #[postgres(name = "Canada/Yukon")]
    Canada__Yukon,
    /// Chile/Continental
    #[postgres(name = "Chile/Continental")]
    Chile__Continental,
    /// Chile/EasterIsland
    #[postgres(name = "Chile/EasterIsland")]
    Chile__EasterIsland,
    /// Cuba
    Cuba,
    /// EET
    EET,
    /// EST
    EST,
    /// EST5EDT
    EST5EDT,
    /// Egypt
    Egypt,
    /// Eire
    Eire,
    /// Etc/GMT
    #[postgres(name = "Etc/GMT")]
    Etc__GMT,
    /// Etc/GMT+0
    #[postgres(name = "Etc/GMT+0")]
    Etc__GMTPlus0,
    /// Etc/GMT+1
    #[postgres(name = "Etc/GMT+1")]
    Etc__GMTPlus1,
    /// Etc/GMT+10
    #[postgres(name = "Etc/GMT+10")]
    Etc__GMTPlus10,
    /// Etc/GMT+11
    #[postgres(name = "Etc/GMT+11")]
    Etc__GMTPlus11,
    /// Etc/GMT+12
    #[postgres(name = "Etc/GMT+12")]
    Etc__GMTPlus12,
    /// Etc/GMT+2
    #[postgres(name = "Etc/GMT+2")]
    Etc__GMTPlus2,
    /// Etc/GMT+3
    #[postgres(name = "Etc/GMT+3")]
    Etc__GMTPlus3,
    /// Etc/GMT+4
    #[postgres(name = "Etc/GMT+4")]
    Etc__GMTPlus4,
    /// Etc/GMT+5
    #[postgres(name = "Etc/GMT+5")]
    Etc__GMTPlus5,
    /// Etc/GMT+6
    #[postgres(name = "Etc/GMT+6")]
    Etc__GMTPlus6,
    /// Etc/GMT+7
    #[postgres(name = "Etc/GMT+7")]
    Etc__GMTPlus7,
    /// Etc/GMT+8
    #[postgres(name = "Etc/GMT+8")]
    Etc__GMTPlus8,
    /// Etc/GMT+9
    #[postgres(name = "Etc/GMT+9")]
    Etc__GMTPlus9,
    /// Etc/GMT-0
    #[postgres(name = "Etc/GMT-0")]
    Etc__GMTMinus0,
    /// Etc/GMT-1
    #[postgres(name = "Etc/GMT-1")]
    Etc__GMTMinus1,
    /// Etc/GMT-10
    #[postgres(name = "Etc/GMT-10")]
    Etc__GMTMinus10,
    /// Etc/GMT-11
    #[postgres(name = "Etc/GMT-11")]
    Etc__GMTMinus11,
    /// Etc/GMT-12
    #[postgres(name = "Etc/GMT-12")]
    Etc__GMTMinus12,
    /// Etc/GMT-13
    #[postgres(name = "Etc/GMT-13")]
    Etc__GMTMinus13,
    /// Etc/GMT-14
    #[postgres(name = "Etc/GMT-14")]
    Etc__GMTMinus14,
    /// Etc/GMT-2
    #[postgres(name = "Etc/GMT-2")]
    Etc__GMTMinus2,
    /// Etc/GMT-3
    #[postgres(name = "Etc/GMT-3")]
    Etc__GMTMinus3,
    /// Etc/GMT-4
    #[postgres(name = "Etc/GMT-4")]
    Etc__GMTMinus4,
    /// Etc/GMT-5
    #[postgres(name = "Etc/GMT-5")]
    Etc__GMTMinus5,
    /// Etc/GMT-6
    #[postgres(name = "Etc/GMT-6")]
    Etc__GMTMinus6,
    /// Etc/GMT-7
    #[postgres(name = "Etc/GMT-7")]
    Etc__GMTMinus7,
    /// Etc/GMT-8
    #[postgres(name = "Etc/GMT-8")]
    Etc__GMTMinus8,
    /// Etc/GMT-9
    #[postgres(name = "Etc/GMT-9")]
    Etc__GMTMinus9,
    /// Etc/GMT0
    #[postgres(name = "Etc/GMT0")]
    Etc__GMT0,
    /// Etc/Greenwich
    #[postgres(name = "Etc/Greenwich")]
    Etc__Greenwich,
    /// Etc/UCT
    #[postgres(name = "Etc/UCT")]
    Etc__UCT,
    /// Etc/UTC
    #[postgres(name = "Etc/UTC")]
    Etc__UTC,
    /// Etc/Universal
    #[postgres(name = "Etc/Universal")]
    Etc__Universal,
    /// Etc/Zulu
    #[postgres(name = "Etc/Zulu")]
    Etc__Zulu,
    /// Europe/Amsterdam
    #[postgres(name = "Europe/Amsterdam")]
    Europe__Amsterdam,
    /// Europe/Andorra
    #[postgres(name = "Europe/Andorra")]
    Europe__Andorra,
    /// Europe/Astrakhan
    #[postgres(name = "Europe/Astrakhan")]
    Europe__Astrakhan,
    /// Europe/Athens
    #[postgres(name = "Europe/Athens")]
    Europe__Athens,
    /// Europe/Belfast
    #[postgres(name = "Europe/Belfast")]
    Europe__Belfast,
    /// Europe/Belgrade
    #[postgres(name = "Europe/Belgrade")]
    Europe__Belgrade,
    /// Europe/Berlin
    #[postgres(name = "Europe/Berlin")]
    Europe__Berlin,
    /// Europe/Bratislava
    #[postgres(name = "Europe/Bratislava")]
    Europe__Bratislava,
    /// Europe/Brussels
    #[postgres(name = "Europe/Brussels")]
    Europe__Brussels,
    /// Europe/Bucharest
    #[postgres(name = "Europe/Bucharest")]
    Europe__Bucharest,
    /// Europe/Budapest
    #[postgres(name = "Europe/Budapest")]
    Europe__Budapest,
    /// Europe/Busingen
    #[postgres(name = "Europe/Busingen")]
    Europe__Busingen,
    /// Europe/Chisinau
    #[postgres(name = "Europe/Chisinau")]
    Europe__Chisinau,
    /// Europe/Copenhagen
    #[postgres(name = "Europe/Copenhagen")]
    Europe__Copenhagen,
    /// Europe/Dublin
    #[postgres(name = "Europe/Dublin")]
    Europe__Dublin,
    /// Europe/Gibraltar
    #[postgres(name = "Europe/Gibraltar")]
    Europe__Gibraltar,
    /// Europe/Guernsey
    #[postgres(name = "Europe/Guernsey")]
    Europe__Guernsey,
    /// Europe/Helsinki
    #[postgres(name = "Europe/Helsinki")]
    Europe__Helsinki,
    /// Europe/Isle_of_Man
    #[postgres(name = "Europe/Isle_of_Man")]
    Europe__Isle_of_Man,
    /// Europe/Istanbul
    #[postgres(name = "Europe/Istanbul")]
    Europe__Istanbul,
    /// Europe/Jersey
    #[postgres(name = "Europe/Jersey")]
    Europe__Jersey,
    /// Europe/Kaliningrad
    #[postgres(name = "Europe/Kaliningrad")]
    Europe__Kaliningrad,
    /// Europe/Kiev
    #[postgres(name = "Europe/Kiev")]
    Europe__Kiev,
    /// Europe/Kirov
    #[postgres(name = "Europe/Kirov")]
    Europe__Kirov,
    /// Europe/Kyiv
    #[postgres(name = "Europe/Kyiv")]
    Europe__Kyiv,
    /// Europe/Lisbon
    #[postgres(name = "Europe/Lisbon")]
    Europe__Lisbon,
    /// Europe/Ljubljana
    #[postgres(name = "Europe/Ljubljana")]
    Europe__Ljubljana,
    /// Europe/London
    #[postgres(name = "Europe/London")]
    Europe__London,
    /// Europe/Luxembourg
    #[postgres(name = "Europe/Luxembourg")]
    Europe__Luxembourg,
    /// Europe/Madrid
    #[postgres(name = "Europe/Madrid")]
    Europe__Madrid,
    /// Europe/Malta
    #[postgres(name = "Europe/Malta")]
    Europe__Malta,
    /// Europe/Mariehamn
    #[postgres(name = "Europe/Mariehamn")]
    Europe__Mariehamn,
    /// Europe/Minsk
    #[postgres(name = "Europe/Minsk")]
    Europe__Minsk,
    /// Europe/Monaco
    #[postgres(name = "Europe/Monaco")]
    Europe__Monaco,
    /// Europe/Moscow
    #[postgres(name = "Europe/Moscow")]
    Europe__Moscow,
    /// Europe/Nicosia
    #[postgres(name = "Europe/Nicosia")]
    Europe__Nicosia,
    /// Europe/Oslo
    #[postgres(name = "Europe/Oslo")]
    Europe__Oslo,
    /// Europe/Paris
    #[postgres(name = "Europe/Paris")]
    Europe__Paris,
    /// Europe/Podgorica
    #[postgres(name = "Europe/Podgorica")]
    Europe__Podgorica,
    /// Europe/Prague
    #[postgres(name = "Europe/Prague")]
    Europe__Prague,
    /// Europe/Riga
    #[postgres(name = "Europe/Riga")]
    Europe__Riga,
    /// Europe/Rome
    #[postgres(name = "Europe/Rome")]
    Europe__Rome,
    /// Europe/Samara
    #[postgres(name = "Europe/Samara")]
    Europe__Samara,
    /// Europe/San_Marino
    #[postgres(name = "Europe/San_Marino")]
    Europe__San_Marino,
    /// Europe/Sarajevo
    #[postgres(name = "Europe/Sarajevo")]
    Europe__Sarajevo,
    /// Europe/Saratov
    #[postgres(name = "Europe/Saratov")]
    Europe__Saratov,
    /// Europe/Simferopol
    #[postgres(name = "Europe/Simferopol")]
    Europe__Simferopol,
    /// Europe/Skopje
    #[postgres(name = "Europe/Skopje")]
    Europe__Skopje,
    /// Europe/Sofia
    #[postgres(name = "Europe/Sofia")]
    Europe__Sofia,
    /// Europe/Stockholm
    #[postgres(name = "Europe/Stockholm")]
    Europe__Stockholm,
    /// Europe/Tallinn
    #[postgres(name = "Europe/Tallinn")]
    Europe__Tallinn,
    /// Europe/Tirane
    #[postgres(name = "Europe/Tirane")]
    Europe__Tirane,
    /// Europe/Tiraspol
    #[postgres(name = "Europe/Tiraspol")]
    Europe__Tiraspol,
    /// Europe/Ulyanovsk
    #[postgres(name = "Europe/Ulyanovsk")]
    Europe__Ulyanovsk,
    /// Europe/Uzhgorod
    #[postgres(name = "Europe/Uzhgorod")]
    Europe__Uzhgorod,
    /// Europe/Vaduz
    #[postgres(name = "Europe/Vaduz")]
    Europe__Vaduz,
    /// Europe/Vatican
    #[postgres(name = "Europe/Vatican")]
    Europe__Vatican,
    /// Europe/Vienna
    #[postgres(name = "Europe/Vienna")]
    Europe__Vienna,
    /// Europe/Vilnius
    #[postgres(name = "Europe/Vilnius")]
    Europe__Vilnius,
    /// Europe/Volgograd
    #[postgres(name = "Europe/Volgograd")]
    Europe__Volgograd,
    /// Europe/Warsaw
    #[postgres(name = "Europe/Warsaw")]
    Europe__Warsaw,
    /// Europe/Zagreb
    #[postgres(name = "Europe/Zagreb")]
    Europe__Zagreb,
    /// Europe/Zaporozhye
    #[postgres(name = "Europe/Zaporozhye")]
    Europe__Zaporozhye,
    /// Europe/Zurich
    #[postgres(name = "Europe/Zurich")]
    Europe__Zurich,
    /// GB
    GB,
    /// GB-Eire
    #[postgres(name = "GB-Eire")]
    GBEire,
    /// GMT
    GMT,
    /// GMT+0
    #[postgres(name = "GMT+0")]
    GMTPlus0,
    /// GMT-0
    #[postgres(name = "GMT-0")]
    GMTMinus0,
    /// GMT0
    GMT0,
    /// Greenwich
    Greenwich,
    /// HST
    HST,
    /// Hongkong
    Hongkong,
    /// Iceland
    Iceland,
    /// Indian/Antananarivo
    #[postgres(name = "Indian/Antananarivo")]
    Indian__Antananarivo,
    /// Indian/Chagos
    #[postgres(name = "Indian/Chagos")]
    Indian__Chagos,
    /// Indian/Christmas
    #[postgres(name = "Indian/Christmas")]
    Indian__Christmas,
    /// Indian/Cocos
    #[postgres(name = "Indian/Cocos")]
    Indian__Cocos,
    /// Indian/Comoro
    #[postgres(name = "Indian/Comoro")]
    Indian__Comoro,
    /// Indian/Kerguelen
    #[postgres(name = "Indian/Kerguelen")]
    Indian__Kerguelen,
    /// Indian/Mahe
    #[postgres(name = "Indian/Mahe")]
    Indian__Mahe,
    /// Indian/Maldives
    #[postgres(name = "Indian/Maldives")]
    Indian__Maldives,
    /// Indian/Mauritius
    #[postgres(name = "Indian/Mauritius")]
    Indian__Mauritius,
    /// Indian/Mayotte
    #[postgres(name = "Indian/Mayotte")]
    Indian__Mayotte,
    /// Indian/Reunion
    #[postgres(name = "Indian/Reunion")]
    Indian__Reunion,
    /// Iran
    Iran,
    /// Israel
    Israel,
    /// Jamaica
    Jamaica,
    /// Japan
    Japan,
    /// Kwajalein
    Kwajalein,
    /// Libya
    Libya,
    /// MET
    MET,
    /// MST
    MST,
    /// MST7MDT
    MST7MDT,
    /// Mexico/BajaNorte
    #[postgres(name = "Mexico/BajaNorte")]
    Mexico__BajaNorte,
    /// Mexico/BajaSur
    #[postgres(name = "Mexico/BajaSur")]
    Mexico__BajaSur,
    /// Mexico/General
    #[postgres(name = "Mexico/General")]
    Mexico__General,
    /// NZ
    NZ,
    /// NZ-CHAT
    #[postgres(name = "NZ-CHAT")]
    NZCHAT,
    /// Navajo
    Navajo,
    /// PRC
    PRC,
    /// PST8PDT
    PST8PDT,
    /// Pacific/Apia
    #[postgres(name = "Pacific/Apia")]
    Pacific__Apia,
    /// Pacific/Auckland
    #[postgres(name = "Pacific/Auckland")]
    Pacific__Auckland,
    /// Pacific/Bougainville
    #[postgres(name = "Pacific/Bougainville")]
    Pacific__Bougainville,
    /// Pacific/Chatham
    #[postgres(name = "Pacific/Chatham")]
    Pacific__Chatham,
    /// Pacific/Chuuk
    #[postgres(name = "Pacific/Chuuk")]
    Pacific__Chuuk,
    /// Pacific/Easter
    #[postgres(name = "Pacific/Easter")]
    Pacific__Easter,
    /// Pacific/Efate
    #[postgres(name = "Pacific/Efate")]
    Pacific__Efate,
    /// Pacific/Enderbury
    #[postgres(name = "Pacific/Enderbury")]
    Pacific__Enderbury,
    /// Pacific/Fakaofo
    #[postgres(name = "Pacific/Fakaofo")]
    Pacific__Fakaofo,
    /// Pacific/Fiji
    #[postgres(name = "Pacific/Fiji")]
    Pacific__Fiji,
    /// Pacific/Funafuti
    #[postgres(name = "Pacific/Funafuti")]
    Pacific__Funafuti,
    /// Pacific/Galapagos
    #[postgres(name = "Pacific/Galapagos")]
    Pacific__Galapagos,
    /// Pacific/Gambier
    #[postgres(name = "Pacific/Gambier")]
    Pacific__Gambier,
    /// Pacific/Guadalcanal
    #[postgres(name = "Pacific/Guadalcanal")]
    Pacific__Guadalcanal,
    /// Pacific/Guam
    #[postgres(name = "Pacific/Guam")]
    Pacific__Guam,
    /// Pacific/Honolulu
    #[postgres(name = "Pacific/Honolulu")]
    Pacific__Honolulu,
    /// Pacific/Johnston
    #[postgres(name = "Pacific/Johnston")]
    Pacific__Johnston,
    /// Pacific/Kanton
    #[postgres(name = "Pacific/Kanton")]
    Pacific__Kanton,
    /// Pacific/Kiritimati
    #[postgres(name = "Pacific/Kiritimati")]
    Pacific__Kiritimati,
    /// Pacific/Kosrae
    #[postgres(name = "Pacific/Kosrae")]
    Pacific__Kosrae,
    /// Pacific/Kwajalein
    #[postgres(name = "Pacific/Kwajalein")]
    Pacific__Kwajalein,
    /// Pacific/Majuro
    #[postgres(name = "Pacific/Majuro")]
    Pacific__Majuro,
    /// Pacific/Marquesas
    #[postgres(name = "Pacific/Marquesas")]
    Pacific__Marquesas,
    /// Pacific/Midway
    #[postgres(name = "Pacific/Midway")]
    Pacific__Midway,
    /// Pacific/Nauru
    #[postgres(name = "Pacific/Nauru")]
    Pacific__Nauru,
    /// Pacific/Niue
    #[postgres(name = "Pacific/Niue")]
    Pacific__Niue,
    /// Pacific/Norfolk
    #[postgres(name = "Pacific/Norfolk")]
    Pacific__Norfolk,
    /// Pacific/Noumea
    #[postgres(name = "Pacific/Noumea")]
    Pacific__Noumea,
    /// Pacific/Pago_Pago
    #[postgres(name = "Pacific/Pago_Pago")]
    Pacific__Pago_Pago,
    /// Pacific/Palau
    #[postgres(name = "Pacific/Palau")]
    Pacific__Palau,
    /// Pacific/Pitcairn
    #[postgres(name = "Pacific/Pitcairn")]
    Pacific__Pitcairn,
    /// Pacific/Pohnpei
    #[postgres(name = "Pacific/Pohnpei")]
    Pacific__Pohnpei,
    /// Pacific/Ponape
    #[postgres(name = "Pacific/Ponape")]
    Pacific__Ponape,
    /// Pacific/Port_Moresby
    #[postgres(name = "Pacific/Port_Moresby")]
    Pacific__Port_Moresby,
    /// Pacific/Rarotonga
    #[postgres(name = "Pacific/Rarotonga")]
    Pacific__Rarotonga,
    /// Pacific/Saipan
    #[postgres(name = "Pacific/Saipan")]
    Pacific__Saipan,
    /// Pacific/Samoa
    #[postgres(name = "Pacific/Samoa")]
    Pacific__Samoa,
    /// Pacific/Tahiti
    #[postgres(name = "Pacific/Tahiti")]
    Pacific__Tahiti,
    /// Pacific/Tarawa
    #[postgres(name = "Pacific/Tarawa")]
    Pacific__Tarawa,
    /// Pacific/Tongatapu
    #[postgres(name = "Pacific/Tongatapu")]
    Pacific__Tongatapu,
    /// Pacific/Truk
    #[postgres(name = "Pacific/Truk")]
    Pacific__Truk,
    /// Pacific/Wake
    #[postgres(name = "Pacific/Wake")]
    Pacific__Wake,
    /// Pacific/Wallis
    #[postgres(name = "Pacific/Wallis")]
    Pacific__Wallis,
    /// Pacific/Yap
    #[postgres(name = "Pacific/Yap")]
    Pacific__Yap,
    /// Poland
    Poland,
    /// Portugal
    Portugal,
    /// ROC
    ROC,
    /// ROK
    ROK,
    /// Singapore
    Singapore,
    /// Turkey
    Turkey,
    /// UCT
    UCT,
    /// US/Alaska
    #[postgres(name = "US/Alaska")]
    US__Alaska,
    /// US/Aleutian
    #[postgres(name = "US/Aleutian")]
    US__Aleutian,
    /// US/Arizona
    #[postgres(name = "US/Arizona")]
    US__Arizona,
    /// US/Central
    #[postgres(name = "US/Central")]
    US__Central,
    /// US/East-Indiana
    #[postgres(name = "US/East-Indiana")]
    US__EastIndiana,
    /// US/Eastern
    #[postgres(name = "US/Eastern")]
    US__Eastern,
    /// US/Hawaii
    #[postgres(name = "US/Hawaii")]
    US__Hawaii,
    /// US/Indiana-Starke
    #[postgres(name = "US/Indiana-Starke")]
    US__IndianaStarke,
    /// US/Michigan
    #[postgres(name = "US/Michigan")]
    US__Michigan,
    /// US/Mountain
    #[postgres(name = "US/Mountain")]
    US__Mountain,
    /// US/Pacific
    #[postgres(name = "US/Pacific")]
    US__Pacific,
    /// US/Samoa
    #[postgres(name = "US/Samoa")]
    US__Samoa,
    /// UTC
    UTC,
    /// Universal
    Universal,
    /// W-SU
    #[postgres(name = "W-SU")]
    WSU,
    /// WET
    WET,
    /// Zulu
    Zulu,
}
