use derive_more::Display;
use serde::{Deserialize, Serialize};

/// ### CountryCode : Enum representing country codes
///
/// Provides a representation for different country codes.
///
/// # Examples
///
/// ```
/// use your_crate_name::CountryCode;
///
/// let usa_code = CountryCode::USA;
/// assert_eq!(usa_code.to_string(), "+1");
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Display)]
pub enum CountryCode {
    #[display(fmt = "+1")]
    USA,
    #[display(fmt = "+44")]
    UK,
    #[display(fmt = "+91")]
    IND,
    #[display(fmt = "+62")]
    INA, // Indonesia
    #[display(fmt = "+86")]
    CHN, // China
    #[display(fmt = "+81")]
    JPN, // Japan
    #[display(fmt = "+82")]
    KOR, // South Korea
    #[display(fmt = "+49")]
    DEU, // Germany
    #[display(fmt = "+7")]
    RUS, // Russia
    #[display(fmt = "+33")]
    FRA, // France
    #[display(fmt = "+39")]
    ITA, // Italy
    #[display(fmt = "+34")]
    ESP, // Spain
    #[display(fmt = "+61")]
    AUS, // Australia
    #[display(fmt = "+65")]
    SGP, // Singapore
    #[display(fmt = "+54")]
    ARG, // Argentina
    #[display(fmt = "+55")]
    BRA, // Brazil
    #[display(fmt = "+56")]
    CHL, // Chile
    #[display(fmt = "+57")]
    COL, // Colombia
    #[display(fmt = "+58")]
    VEN, // Venezuela
    #[display(fmt = "+60")]
    MYS, // Malaysia
    #[display(fmt = "+63")]
    PHL, // Philippines
    #[display(fmt = "+64")]
    NZL, // New Zealand
    #[display(fmt = "+66")]
    THA, // Thailand
    #[display(fmt = "+84")]
    VNM, // Vietnam
    #[display(fmt = "+90")]
    TUR, // Turkey
    #[display(fmt = "+92")]
    PAK, // Pakistan
    #[display(fmt = "+93")]
    AFG, // Afghanistan
    #[display(fmt = "+94")]
    LKA, // Sri Lanka
    #[display(fmt = "+95")]
    MMR, // Myanmar (Burma)
    #[display(fmt = "+98")]
    IRN, // Iran
    #[display(fmt = "+212")]
    MAR, // Morocco
    #[display(fmt = "+213")]
    DZA, // Algeria
    #[display(fmt = "+216")]
    TUN, // Tunisia
    #[display(fmt = "+218")]
    LBY, // Libya
    #[display(fmt = "+220")]
    GMB, // Gambia
    #[display(fmt = "+221")]
    SEN, // Senegal
    #[display(fmt = "+222")]
    MRT, // Mauritania
    #[display(fmt = "+223")]
    MLI, // Mali
    #[display(fmt = "+224")]
    GIN, // Guinea
    #[display(fmt = "+225")]
    CIV, // Ivory Coast (Côte d'Ivoire)
    #[display(fmt = "+226")]
    BFA, // Burkina Faso
    #[display(fmt = "+227")]
    NER, // Niger
    #[display(fmt = "+228")]
    TGO, // Togo
    #[display(fmt = "+229")]
    BEN, // Benin
    #[display(fmt = "+230")]
    MUS, // Mauritius
    #[display(fmt = "+231")]
    LBR, // Liberia
    #[display(fmt = "+232")]
    SLE, // Sierra Leone
    #[display(fmt = "+233")]
    GHA, // Ghana
    #[display(fmt = "+234")]
    NGA, // Nigeria
    #[display(fmt = "+235")]
    TCD, // Chad
    #[display(fmt = "+236")]
    CAF, // Central African Republic
    #[display(fmt = "+237")]
    CMR, // Cameroon
    #[display(fmt = "+238")]
    CPV, // Cape Verde
    #[display(fmt = "+239")]
    STP, // São Tomé and Príncipe
    #[display(fmt = "+240")]
    GNQ, // Equatorial Guinea
    #[display(fmt = "+241")]
    GAB, // Gabon
    #[display(fmt = "+242")]
    COG, // Congo-Brazzaville
    #[display(fmt = "+243")]
    COD, // Democratic Republic of the Congo
    #[display(fmt = "+244")]
    AGO, // Angola
    #[display(fmt = "+245")]
    GNB, // Guinea-Bissau
    #[display(fmt = "+246")]
    IOT, // British Indian Ocean Territory
    #[display(fmt = "+247")]
    SHN, // Saint Helena, Ascension and Tristan da Cunha
    #[display(fmt = "+248")]
    SYC, // Seychelles
    #[display(fmt = "+249")]
    SDN, // Sudan
    #[display(fmt = "+250")]
    RWA, // Rwanda
    #[display(fmt = "+251")]
    ETH, // Ethiopia
    #[display(fmt = "+252")]
    SOM, // Somalia
    #[display(fmt = "+253")]
    DJI, // Djibouti
    #[display(fmt = "+254")]
    KEN, // Kenya
    #[display(fmt = "+255")]
    TZA, // Tanzania
    #[display(fmt = "+256")]
    UGA, // Uganda
    #[display(fmt = "+257")]
    BDI, // Burundi
    #[display(fmt = "+258")]
    MOZ, // Mozambique
    #[display(fmt = "+260")]
    ZMB, // Zambia
    #[display(fmt = "+261")]
    MDG, // Madagascar
    #[display(fmt = "+262")]
    REU, // Réunion
    #[display(fmt = "+263")]
    ZWE, // Zimbabwe
    #[display(fmt = "+264")]
    NAM, // Namibia
    #[display(fmt = "+265")]
    MWI, // Malawi
    #[display(fmt = "+266")]
    LSO, // Lesotho
    #[display(fmt = "+267")]
    BWA, // Botswana
    #[display(fmt = "+268")]
    SWZ, // Eswatini (Swaziland)
    #[display(fmt = "+269")]
    COM, // Comoros
    #[display(fmt = "+291")]
    ERI, // Eritrea
    #[display(fmt = "+297")]
    ABW, // Aruba
    #[display(fmt = "+298")]
    FRO, // Faroe Islands
    #[display(fmt = "+299")]
    GRL, // Greenland
    #[display(fmt = "+350")]
    GIB, // Gibraltar
    #[display(fmt = "+351")]
    PRT, // Portugal
    #[display(fmt = "+352")]
    LUX, // Luxembourg
    #[display(fmt = "+353")]
    IRL, // Ireland
    #[display(fmt = "+354")]
    ISL, // Iceland
    #[display(fmt = "+355")]
    ALB, // Albania
    #[display(fmt = "+356")]
    MLT, // Malta
    #[display(fmt = "+357")]
    CYP, // Cyprus
    #[display(fmt = "+358")]
    FIN, // Finland
    #[display(fmt = "+359")]
    BGR, // Bulgaria
    #[display(fmt = "+370")]
    LTU, // Lithuania
    #[display(fmt = "+371")]
    LVA, // Latvia
    #[display(fmt = "+372")]
    EST, // Estonia
    #[display(fmt = "+373")]
    MDA, // Moldova
    #[display(fmt = "+374")]
    ARM, // Armenia
    #[display(fmt = "+375")]
    BLR, // Belarus
    #[display(fmt = "+376")]
    AND, // Andorra
    #[display(fmt = "+377")]
    MCO, // Monaco
    #[display(fmt = "+378")]
    SMR, // San Marino
    #[display(fmt = "+379")]
    VAT, // Vatican City
    #[display(fmt = "+380")]
    UKR, // Ukraine
    #[display(fmt = "+381")]
    SRB, // Serbia
    #[display(fmt = "+382")]
    MNE, // Montenegro
    #[display(fmt = "+385")]
    HRV, // Croatia
    #[display(fmt = "+386")]
    SVN, // Slovenia
    #[display(fmt = "+387")]
    BIH, // Bosnia and Herzegovina
    #[display(fmt = "+389")]
    MKD, // North Macedonia
    #[display(fmt = "+420")]
    CZE, // Czechia (Czech Republic)
    #[display(fmt = "+421")]
    SVK, // Slovakia
    #[display(fmt = "+423")]
    LIE, // Liechtenstein
    #[display(fmt = "+500")]
    FLK, // Falkland Islands
    #[display(fmt = "+501")]
    BLZ, // Belize
    #[display(fmt = "+502")]
    GTM, // Guatemala
    #[display(fmt = "+503")]
    SLV, // El Salvador
    #[display(fmt = "+504")]
    HND, // Honduras
    #[display(fmt = "+505")]
    NIC, // Nicaragua
    #[display(fmt = "+506")]
    CRI, // Costa Rica
    #[display(fmt = "+507")]
    PAN, // Panama
    #[display(fmt = "+508")]
    SPM, // Saint Pierre and Miquelon
    #[display(fmt = "+509")]
    HTI, // Haiti
    #[display(fmt = "+590")]
    GLP, // Guadeloupe
    #[display(fmt = "+591")]
    BOL, // Bolivia
    #[display(fmt = "+592")]
    GUY, // Guyana
    #[display(fmt = "+593")]
    ECU, // Ecuador
    #[display(fmt = "+594")]
    MYT, // Mayotte
    #[display(fmt = "+595")]
    PRY, // Paraguay
    #[display(fmt = "+596")]
    MTQ, // Martinique
    #[display(fmt = "+597")]
    SUR, // Suriname
    #[display(fmt = "+598")]
    URY, // Uruguay
    #[display(fmt = "+599")]
    ANT, // Caribbean Netherlands
    #[display(fmt = "+670")]
    TLS, // Timor-Leste (East Timor)
    #[display(fmt = "+672")]
    ATA, // Antarctica
    #[display(fmt = "+673")]
    BRN, // Brunei
    #[display(fmt = "+674")]
    NRU, // Nauru
    #[display(fmt = "+675")]
    PNG, // Papua New Guinea
    #[display(fmt = "+676")]
    TON, // Tonga
    #[display(fmt = "+677")]
    SLB, // Solomon Islands
    #[display(fmt = "+678")]
    VUT, // Vanuatu
    #[display(fmt = "+679")]
    FJI, // Fiji
    #[display(fmt = "+680")]
    PLW, // Palau
    #[display(fmt = "+681")]
    WLF, // Wallis and Futuna
    #[display(fmt = "+682")]
    COK, // Cook Islands
    #[display(fmt = "+683")]
    NIU, // Niue
    #[display(fmt = "+685")]
    WSM, // Samoa
    #[display(fmt = "+686")]
    KIR, // Kiribati
    #[display(fmt = "+687")]
    NCL, // New Caledonia
    #[display(fmt = "+688")]
    TUV, // Tuvalu
    #[display(fmt = "+689")]
    PYF, // French Polynesia
    #[display(fmt = "+690")]
    TKL, // Tokelau
    #[display(fmt = "+691")]
    FSM, // Federated States of Micronesia
    #[display(fmt = "+692")]
    MHL, // Marshall Islands
    #[display(fmt = "+850")]
    PRK, // North Korea
    #[display(fmt = "+852")]
    HKG, // Hong Kong
    #[display(fmt = "+853")]
    MAC, // Macao
    #[display(fmt = "+855")]
    KHM, // Cambodia
    #[display(fmt = "+856")]
    LAO, // Laos
    #[display(fmt = "+880")]
    BGD, // Bangladesh
    #[display(fmt = "+886")]
    TWN, // Taiwan
    #[display(fmt = "+960")]
    MDV, // Maldives
    #[display(fmt = "+961")]
    LBN, // Lebanon
    #[display(fmt = "+962")]
    JOR, // Jordan
    #[display(fmt = "+963")]
    SYR, // Syria
    #[display(fmt = "+964")]
    IRQ, // Iraq
    #[display(fmt = "+965")]
    KWT, // Kuwait
    #[display(fmt = "+966")]
    SAU, // Saudi Arabia
    #[display(fmt = "+967")]
    YEM, // Yemen
    #[display(fmt = "+968")]
    OMN, // Oman
    #[display(fmt = "+970")]
    PSE, // Palestine
    #[display(fmt = "+971")]
    ARE, // United Arab Emirates
    #[display(fmt = "+972")]
    ISR, // Israel
    #[display(fmt = "+973")]
    BHR, // Bahrain
    #[display(fmt = "+974")]
    QAT, // Qatar
    #[display(fmt = "+975")]
    BTN, // Bhutan
    #[display(fmt = "+976")]
    MNG, // Mongolia
    #[display(fmt = "+977")]
    NPL, // Nepal
    #[display(fmt = "+992")]
    TJK, // Tajikistan
    #[display(fmt = "+993")]
    TKM, // Turkmenistan
    #[display(fmt = "+994")]
    AZE, // Azerbaijan
    #[display(fmt = "+995")]
    GEO, // Georgia
    #[display(fmt = "+996")]
    KGZ, // Kyrgyzstan
    #[display(fmt = "+998")]
    UZB, // Uzbekistan
}
