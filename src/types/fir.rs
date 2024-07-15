#[derive(Debug, strum::Display)]
/// List of the world's Flight Information Regions
/// Source: <https://en.wikipedia.org/wiki/Flight_information_region>
///
/// Extraction code
/// ```javascript
/// let res = "";
///
/// document.querySelectorAll(".jquery-tablesorter tbody tr").forEach(line => {
///     let childs = line.querySelectorAll("td");
///
///     if (childs[1].innerText === "FIR") {
///         res += `
///         /// ${childs[2].innerText.trim()}, ${childs[4].innerText.trim().replaceAll("\u00a0", "")}
///         ${childs[0].innerText.trim()},`;
///     }
/// });
///
/// copy(JSON.stringify(res))
/// ```
pub enum Fir {
    /// Honiara ACC, Solomon Islands
    AGGG,
    /// Nauru ACC, Nauru
    ANAU,
    /// Port Moresby ACC, Papua New Guinea
    AYPM,
    /// Nuuk ACC, Greenland (Denmark)
    BGGL,
    /// Reykjavík ACC, Iceland
    BIRD,
    /// Edmonton ACC, Canada
    CZEG,
    /// Moncton Southern ACC, Canada
    CZQM,
    /// Gander Domestic ACC, Canada
    CZQX,
    /// Montreal ACC, Canada
    CZUL,
    /// Vancouver ACC, Canada
    CZVR,
    /// Winnipeg ACC, Canada
    CZWG,
    /// Toronto ACC, Canada
    CZYZ,
    /// Alger ACC, Algeria
    DAAA,
    /// Accra ACC, Ghana
    DGAC,
    /// Abidjan ACC, Ivory Coast
    DIII,
    /// Kano ACC, Nigeria
    DNKK,
    /// Niamey ACC, Niger
    DRRR,
    /// Tunis ACC, Tunisia
    DTTC,
    /// EUROCONTROL, Belgium
    EZZZ,
    /// Brussels ACC, Belgium/Luxembourg
    EBBU,
    /// Langen ACC, Germany
    EDGG,
    /// Munich ACC, Germany
    EDMM,
    /// Bremen ACC, Germany
    EDWW,
    /// Tallinn ACC, Estonia
    EETT,
    /// Helsinki ACC, Finland
    EFIN,
    /// Shanwick Oceanic OCA, United Kingdom
    EGGX,
    /// Scottish ACC, United Kingdom
    EGPX,
    /// Scottish ACC (Mil), United Kingdom
    EGQQ,
    /// London ACC, United Kingdom
    EGTT,
    /// Amsterdam ACC, Netherlands
    EHAA,
    /// Shannon ACC, Ireland
    EISN,
    /// Copenhagen ACC, Denmark
    EKDK,
    /// Bodo Oceanic OCA, Norway
    ENOB,
    /// Polaris ACC, Norway
    ENOR,
    /// Warszawa ACC, Poland
    EPWW,
    /// Sweden ACC, Sweden
    ESAA,
    /// Malmo ACC, Sweden
    ESMM,
    /// Stockholm ACC, Sweden
    ESOS,
    /// Riga ACC, Latvia
    EVRR,
    /// Vilnius ACC, Lithuania
    EYVL,
    /// Bloemfontein ACC, South Africa
    FABL,
    /// Cape Town ACC, South Africa
    FACA,
    /// Cape Town ACC, South Africa
    FACT,
    /// Durban ACC, South Africa
    FADN,
    /// Johannesburg Oceanic ACC, South Africa
    FAJO,
    /// Johannesburg ACC, South Africa
    FAJX,
    /// Port Elizabeth ACC, South Africa
    FAPX,
    /// Gaborone ACC, Botswana
    FBGR,
    /// Brazzaville ACC, Congo, Republic of the
    FCCC,
    /// Mauritius ACC, Mauritius
    FIMM,
    /// Douala ACC, Cameroon
    FKKK,
    /// Lusaka ACC, Zambia
    FLFI,
    /// Comoros ACC, Comoros
    FMCX,
    /// Antananarivo ACC, Madagascar
    FMMM,
    /// Luanda ACC, Angola
    FNAN,
    /// Libreville ACC, Gabon
    FOOO,
    /// Beira ACC, Mozambique
    FQBE,
    /// Seychelles ACC, Seychelles
    FSSS,
    /// N'Djamena ACC, Chad
    FTTT,
    /// Harare ACC, Zimbabwe
    FVHF,
    /// Lilongwe ACC, Malawi
    FWLL,
    /// Windhoek ACC, Namibia
    FYWF,
    /// Kinshasa ACC, Congo, Democratic Republic of the
    FZZA,
    /// Canarias ACC, Canary Islands (Spain)
    GCCC,
    /// Roberts ACC, Liberia
    GLRB,
    /// Agadir ACC, Morocco
    GMAC,
    /// Casablanca ACC, Morocco
    GMMM,
    /// Dakar Oceanic ACC, Senegal
    GOOO,
    /// Sal Oceanic ACC, Cape Verde
    GVSC,
    /// Addis Ababa ACC, Ethiopia
    HAAA,
    /// Bujumbura ACC, Burundi
    HBBA,
    /// Mogadishu ACC, Somalia
    HCSM,
    /// Cairo ACC, Egypt
    HECC,
    /// Asmara ACC, Eritrea
    HHAA,
    /// Nairobi ACC, Kenya
    HKNA,
    /// Tripoli ACC, Libya
    HLLL,
    /// Kigali ACC, Rwanda
    HRYR,
    /// Khartoum ACC, Sudan
    HSSS,
    /// Dar Es Salaam ACC, Tanzania
    HTDC,
    /// Entebbe ACC, Uganda
    HUEC,
    /// Albuquerque ARTCC, United States
    KZAB,
    /// Oakland Oceanic ARTCC, United States
    KZAK,
    /// Chicago ARTCC, United States
    KZAU,
    /// Boston ARTCC, United States
    KZBW,
    /// Washington ARTCC, United States
    KZDC,
    /// Denver ARTCC, United States
    KZDV,
    /// Ft Worth ARTCC, United States
    KZFW,
    /// Houston ARTCC, United States
    KZHU,
    /// Indianapolis ARTCC, United States
    KZID,
    /// Jacksonville ARTCC, United States
    KZJX,
    /// Kansas City ARTCC, United States
    KZKC,
    /// Los Angeles ARTCC, United States
    KZLA,
    /// Salt Lake ARTCC, United States
    KZLC,
    /// Miami ARTCC, United States
    KZMA,
    /// Memphis ARTCC, United States
    KZME,
    /// Minneapolis ARTCC, United States
    KZMP,
    /// New York ARTCC, United States
    KZNY,
    /// Oakland ARTCC, United States
    KZOA,
    /// Cleveland ARTCC, United States
    KZOB,
    /// Seattle ARTCC, United States
    KZSE,
    /// Atlanta ARTCC, United States
    KZTL,
    /// New York Oceanic ARTCC, United States
    KZWY,
    /// Tirana ACC, Albania
    LAAA,
    /// Sofia ACC, Bulgaria
    LBSR,
    /// Varna ACC, Bulgaria
    LBWR,
    /// Nicosia ACC, Cyprus
    LCCC,
    /// Zagreb ACC, Croatia
    LDZO,
    /// Barcelona ACC, Spain
    LECB,
    /// Madrid ACC, Spain
    LECM,
    /// Sevilla ACC, Spain
    LECS,
    /// Bordeaux ACC, France
    LFBB,
    /// Reims ACC, France
    LFEE,
    /// Paris ACC, France
    LFFF,
    /// Marseille ACC, France
    LFMM,
    /// Brest ACC, France
    LFRR,
    /// Athens ACC, Greece
    LGGG,
    /// Budapest ACC, Hungary
    LHCC,
    /// Brindisi ACC, Italy
    LIBB,
    /// Milano ACC, Italy
    LIMM,
    /// Roma ACC, Italy
    LIRR,
    /// Ljubljana ACC, Slovenia
    LJLA,
    /// Praha ACC, Czech Republic
    LKAA,
    /// Tel-Aviv ACC, Israel
    LLLL,
    /// Malta ACC, Malta
    LMMM,
    /// Wien ACC, Austria
    LOVV,
    /// Lisboa ACC, Portugal
    LPPC,
    /// Santa Maria Oceanic ACC, Azores (Portugal)
    LPPO,
    /// Sarajevo ACC, Bosnia and Herzegovina
    LQSB,
    /// Bucuresti ACC, Romania
    LRBB,
    /// Geneve ACC, Switzerland
    LSAG,
    /// Switzerland ACC, Switzerland
    LSAS,
    /// Zurich ACC, Switzerland
    LSAZ,
    /// Ankara ACC, Turkey
    LTAA,
    /// Istanbul ACC, Turkey
    LTBB,
    /// Chisinau ACC, Moldova
    LUUU,
    /// Skopje ACC, North Macedonia
    LWSS,
    /// Beograd ACC, Serbia
    LYBA,
    /// Bratislava ACC, Slovakia
    LZBB,
    /// Santo Domingo ACC, Dominican Republic
    MDCS,
    /// Central American ACC, Honduras
    MHTG,
    /// Kingston ACC, Jamaica
    MKJK,
    /// Mazatlan Oceanic ACC, Mexico
    MMFO,
    /// Mexico ACC, Mexico
    MMFR,
    /// Panama ACC, Panama
    MPZL,
    /// Port-Au-Prince ACC, Haiti
    MTEG,
    /// Habana ACC, Cuba
    MUFH,
    /// Nassau ACC, Bahamas
    MYNA,
    /// Nadi ACC, Fiji
    NFFF,
    /// Tahiti ACC, French Polynesia (France)
    NTTT,
    /// Noumea ACC, New Caledonia (France)
    NWWX,
    /// New Zealand ACC, New Zealand
    NZZC,
    /// Auckland Oceanic ACC, New Zealand
    NZZO,
    /// Kabul ACC, Afghanistan
    OAKX,
    /// Bahrain ACC, Bahrain
    OBBB,
    /// Jeddah ACC, Saudi Arabia
    OEJD,
    /// Tehran ACC, Iran
    OIIX,
    /// Amman ACC, Jordan
    OJAC,
    /// Kuwait ACC, Kuwait
    OKKK,
    /// Beirut ACC, Lebanon
    OLBB,
    /// Emirates ACC, United Arab Emirates
    OMAE,
    /// Muscat ACC, Oman
    OOMM,
    /// Karachi ACC, Pakistan
    OPKR,
    /// Lahore ACC, Pakistan
    OPLR,
    /// Baghdad ACC, Iraq
    ORBB,
    /// ORMM FIR, Iraq
    ORMM,
    /// Damascus ACC, Syria
    OSTT,
    /// Sanaa ACC, Yemen
    OYSC,
    /// Anchorage ARTCC, United States
    PAZA,
    /// Anchorage Oceanic ACC, United States
    PAZN,
    /// Honolulu ACC, United States
    PHZH,
    /// Taipei ACC, Taiwan
    RCAA,
    /// Fukuoka ACC, Japan
    RJJJ,
    /// Incheon ACC, South Korea
    RKRR,
    /// Manila ACC, Philippines
    RPHI,
    /// Cordoba ACC, Argentina
    SACF,
    /// Ezeiza ACC, Argentina
    SAEF,
    /// Mendoza ACC, Argentina
    SAMF,
    /// Resistencia ACC, Argentina
    SARR,
    /// Comodoro Rivadavia ACC, Argentina
    SAVF,
    /// Atlantico ACC, Brazil
    SBAO,
    /// Amazonica ACC, Brazil
    SBAZ,
    /// Brasilia ACC, Brazil
    SBBS,
    /// Curitiba ACC, Brazil
    SBCW,
    /// Recife ACC, Brazil
    SBRE,
    /// Punta Arenas ACC, Chile
    SCCZ,
    /// Santiago ACC, Chile
    SCEZ,
    /// Antofagasta ACC, Chile
    SCFZ,
    /// Easter Island ACC, Easter Island (Chile)
    SCIZ,
    /// Puerto Montt ACC, Chile
    SCTZ,
    /// Guayaquil ACC, Ecuador
    SEFG,
    /// Asuncion ACC, Paraguay
    SGFA,
    /// Barranquilla ACC, Colombia
    SKEC,
    /// Bogota ACC, Colombia
    SKED,
    /// La Paz ACC, Bolivia
    SLLF,
    /// Paramaribo ACC, Suriname
    SMPM,
    /// Rochambeau ACC, French Guiana (France)
    SOOO,
    /// Lima ACC, Peru
    SPIM,
    /// Montevideo ACC, Uruguay
    SUEO,
    /// Maiquetia ACC, Venezuela
    SVZM,
    /// Georgetown ACC, Guyana
    SYGC,
    /// San Juan ACC, Puerto Rico (United States)
    TJZS,
    /// Curacao ACC, Curaçao (Netherlands)
    TNCF,
    /// Piarco ACC, Trinidad and Tobago
    TTZP,
    /// Almaty ACC, Kazakhstan
    UAAX,
    /// Astana ACC, Kazakhstan
    UACX,
    /// Bishkek ACC, Kyrgyzstan
    UAFX,
    /// Semipalatinsk ACC, Kazakhstan
    UASS,
    /// Yerevan ACC, Armenia
    UDDD,
    /// Tyoply Klyuch ACC, Russia
    UEMH,
    /// Nyurba ACC, Russia
    UENN,
    /// Chersky ACC, Russia
    UESS,
    /// Zyryanka ACC, Russia
    UESU,
    /// Gigansk ACC, Russia
    UEVV,
    /// Yerevan/Zvartnots ACC, Russia
    UGEE,
    /// Tbilisi ACC, Georgia
    UGGG,
    /// Magdagachi ACC, Russia
    UHBI,
    /// Khabarovsk/Novy, Russia
    UHHH,
    /// Mys Shmidta ACC, Russia
    UHMI,
    /// Magadan Oceanic, Russia
    UHMM,
    /// Pevek ACC, Russia
    UHMP,
    /// Nikolayevsk-na-Amure ACC, Russia
    UHNN,
    /// Tilichiki ACC, Russia
    UHPT,
    /// Ust-Khairyozovo ACC, Russia
    UHPU,
    /// Okha ACC, Russia
    UHSH,
    /// Bodaybo ACC, Russia
    UIKB,
    /// Kirensk ACC, Russia
    UIKK,
    /// Kyiv ACC, Ukraine
    UKBV,
    /// Dnipro ACC, Ukraine
    UKDV,
    /// Dnipro ACC, Odesa ACC[7], Ukraine
    UKFV,
    /// Lviv ACC, Ukraine
    UKLV,
    /// Odesa ACC, Ukraine
    UKOV,
    /// Sankt Peterburg ACC, Russia
    ULLL,
    /// Velikiye Luki ACC, Russia
    ULOL,
    /// Kazan ACC, Russia
    UMKD,
    /// Minsk ACC, Belarus
    UMMV,
    /// Kolpashevo ACC, Russia
    UNLL,
    /// Turukhansk ACC, Russia
    UOTT,
    /// Rostov-Na-Donu ACC, Russia
    URRV,
    /// Mys Kamenny ACC, Russia
    USDK,
    /// Beryozovo ACC, Russia
    USHB,
    /// Khanty-Mansiysk ACC, Russia
    USHH,
    /// Ashgabat ACC, Turkmenistan
    UTAA,
    /// Turkmenbashi ACC, Turkmenistan
    UTAK,
    /// Turkmenabat ACC, Turkmenistan
    UTAV,
    /// Nukus ACC, Uzbekistan
    UTNR,
    /// Samarkand ACC, Uzbekistan
    UTSD,
    /// Tashkent ACC, Uzbekistan
    UTTR,
    /// Moscow ACC, Russia
    UUWV,
    /// Vorkuta ACC, Russia
    UUYW,
    /// Syktyvkar ACC, Russia
    UUYY,
    /// Orenburg/Tsentralny ACC, Russia
    UWOO,
    /// Mumbai ACC, India
    VABF,
    /// Colombo ACC, Sri Lanka
    VCCF,
    /// Phnom Penh ACC, Cambodia
    VDPF,
    /// Kolkata ACC, India
    VECF,
    /// Dhaka ACC, Bangladesh
    VGFR,
    /// Hong Kong ACC, Hong Kong ( China)
    VHHK,
    /// Delhi ACC, India
    VIDF,
    /// Vientiane ACC, Laos
    VLIV,
    /// Vientiane ACC, Laos
    VLVT,
    /// Kathmandu ACC, Nepal
    VNSM,
    /// Chennai ACC, India
    VOMF,
    /// Male ACC, Maldives
    VRMF,
    /// Bangkok ACC, Thailand
    VTBB,
    /// Ho Chi Minh ACC, Vietnam
    VVHM,
    /// Hanoi ACC, Vietnam
    VVHN,
    /// , Myanmar
    VYMD,
    /// Yangon ACC, Myanmar
    VYYF,
    /// Ujung Pandang, Indonesia
    WAAF,
    /// Ujung Pandang ACC, Indonesia
    WAAZ,
    /// Biak Sector, Indonesia
    WABZ,
    /// Bali Sector, Indonesia
    WADZ,
    /// , Indonesia
    WAJZ,
    /// Merauke Sector, Indonesia
    WAKZ,
    /// Balikpapan Sector, Indonesia
    WALZ,
    /// Manado Sector, Indonesia
    WAMZ,
    /// Banjarmasin Sector, Indonesia
    WAOZ,
    /// Ambon Sector, Indonesia
    WAPZ,
    /// Kupang Sector, Indonesia
    WATZ,
    /// Kota Kinabalu ACC, Brunei/Malaysia
    WBFC,
    /// Jakarta, Indonesia
    WIIF,
    /// Jakarta ACC, Indonesia
    WIIZ,
    /// Medan Sector, Indonesia
    WIMZ,
    /// Pontianak Sector, Indonesia
    WIOZ,
    /// Palembang Sector, Indonesia
    WIPZ,
    /// Kuala Lumpur ACC, Malaysia
    WMFC,
    /// Singapore ACC, Singapore
    WSJC,
    /// Brisbane ACC, Australia
    YBBB,
    /// Melbourne ACC, Australia
    YMMM,
    /// Beijing ACC, China
    ZBPE,
    /// Guangzhou ACC, China
    ZGZU,
    /// Wuhan ACC, China
    ZHWH,
    /// Sanya ACC, China
    ZJSA,
    /// Pyongyang ACC, North Korea
    ZKKP,
    /// Lanzhou ACC, China
    ZLHW,
    /// Ulan Bator ACC, Mongolia
    ZMUB,
    /// Kunming ACC, China
    ZPKM,
    /// Shanghai ACC, China
    ZSHA,
    /// Urumqi ACC, China
    ZWUQ,
    /// Shenyang ACC, China
    ZYSH,
}
