//! Regex patterns for AAS verification.

#![allow(dead_code)]

use once_cell::sync::Lazy;
use regex::Regex;

// ---- Simple patterns --------------------------------------------------------

pub static RE_ID_SHORT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap());

pub static RE_VERSION_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0|[1-9][0-9]*)$").unwrap());

pub static RE_REVISION_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(0|[1-9][0-9]*)$").unwrap());

pub static RE_XS_BOOLEAN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(true|false|1|0)$").unwrap());

pub static RE_XS_HEX_BINARY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([0-9a-fA-F]{2})*$").unwrap());

pub static RE_XS_DECIMAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\+|-)?([0-9]+\.[0-9]*|\.[0-9]+|[0-9]+)$").unwrap());

pub static RE_XS_DOUBLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^((\+|-)?([0-9]+(\.[0-9]*)?|\.[0-9]+)([Ee](\+|-)?[0-9]+)?|-?INF|NaN)$").unwrap()
});

pub static RE_XS_FLOAT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^((\+|-)?([0-9]+(\.[0-9]*)?|\.[0-9]+)([Ee](\+|-)?[0-9]+)?|-?INF|NaN)$").unwrap()
});

pub static RE_XS_DURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^-?P((([0-9]+Y([0-9]+M)?([0-9]+D)?|([0-9]+M)([0-9]+D)?|([0-9]+D))(T(([0-9]+H)([0-9]+M)?([0-9]+(\.[0-9]+)?S)?|([0-9]+M)([0-9]+(\.[0-9]+)?S)?|([0-9]+(\.[0-9]+)?S)))?)|(T(([0-9]+H)([0-9]+M)?([0-9]+(\.[0-9]+)?S)?|([0-9]+M)([0-9]+(\.[0-9]+)?S)?|([0-9]+(\.[0-9]+)?S))))$",
    )
    .unwrap()
});

pub static RE_XS_G_DAY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^---(0[1-9]|[12][0-9]|3[01])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$")
        .unwrap()
});

pub static RE_XS_G_MONTH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^--(0[1-9]|1[0-2])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$").unwrap()
});

pub static RE_XS_G_MONTH_DAY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^--(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$",
    )
    .unwrap()
});

pub static RE_XS_G_YEAR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-?([1-9][0-9]{3,}|0[0-9]{3})(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$")
        .unwrap()
});

pub static RE_XS_G_YEAR_MONTH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$",
    )
    .unwrap()
});

pub static RE_XS_TIME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]+)?|(24:00:00(\.0+)?))(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?$",
    )
    .unwrap()
});

pub static RE_XS_INTEGER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[-+]?[0-9]+$").unwrap());

pub static RE_XS_LONG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[-+]?0*[0-9]{1,20}$").unwrap());

pub static RE_XS_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[-+]?0*[0-9]{1,10}$").unwrap());

pub static RE_XS_SHORT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[-+]?0*[0-9]{1,5}$").unwrap());

pub static RE_XS_BYTE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[-+]?0*[0-9]{1,3}$").unwrap());

pub static RE_XS_NON_NEGATIVE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?[0-9]+)$").unwrap());

pub static RE_XS_POSITIVE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\+?0*[1-9][0-9]*$").unwrap());

pub static RE_XS_UNSIGNED_LONG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?0*[0-9]{1,20})$").unwrap());

pub static RE_XS_UNSIGNED_INT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?0*[0-9]{1,10})$").unwrap());

pub static RE_XS_UNSIGNED_SHORT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?0*[0-9]{1,5})$").unwrap());

pub static RE_XS_UNSIGNED_BYTE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?0*[0-9]{1,3})$").unwrap());

pub static RE_XS_NON_POSITIVE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\+0|0|-[0-9]+)$").unwrap());

pub static RE_XS_NEGATIVE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0*[1-9][0-9]*)$").unwrap());

pub static RE_XML_SERIALIZABLE_STRING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\u{9}\u{a}\u{d}\u{20}-\u{D7FF}\u{E000}-\u{FFFD}\u{10000}-\u{10FFFF}]*$").unwrap()
});

pub static RE_XS_STRING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\u{9}\u{a}\u{d}\u{20}-\u{D7FF}\u{E000}-\u{FFFD}\u{10000}-\u{10FFFF}]*$").unwrap()
});

pub static RE_DATE_PREFIX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-?[0-9]+)-([0-9]{2})-([0-9]{2})").unwrap());

pub static RE_LONG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([-+])?0*([0-9]{1,20})$").unwrap());

pub static RE_UNSIGNED_LONG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(-0|\+?0*([0-9]{1,20}))$").unwrap());

pub static RE_IS_BCP47_FOR_ENGLISH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(en|EN)(-.*)?$").unwrap());

// ---- Complex patterns -------------------------------------------------------

pub static RE_XS_DATE_TIME_UTC: Lazy<Regex> = Lazy::new(|| {
    let digit = "[0-9]";
    let year_frag = format!("-?(([1-9]{digit}{digit}{digit}+)|(0{digit}{digit}{digit}))");
    let month_frag = "((0[1-9])|(1[0-2]))";
    let day_frag = format!("((0[1-9])|([12]{digit})|(3[01]))");
    let hour_frag = format!("(([01]{digit})|(2[0-3]))");
    let minute_frag = format!("[0-5]{digit}");
    let second_frag = format!("([0-5]{digit})(\\.[0-9]+)?");
    let end_of_day_frag = "24:00:00(\\.0+)?";
    let timezone_frag = "(Z|\\+00:00|-00:00)";
    let date_time =
        format!("{year_frag}-{month_frag}-{day_frag}T(({hour_frag}:{minute_frag}:{second_frag})|{end_of_day_frag}){timezone_frag}");
    Regex::new(&format!("^{date_time}$")).unwrap()
});

pub static RE_XS_DATE: Lazy<Regex> = Lazy::new(|| {
    let digit = "[0-9]";
    let year_frag = format!("-?(([1-9]{digit}{digit}{digit}+)|(0{digit}{digit}{digit}))");
    let month_frag = "((0[1-9])|(1[0-2]))";
    let day_frag = format!("((0[1-9])|([12]{digit})|(3[01]))");
    let minute_frag = format!("[0-5]{digit}");
    let timezone_frag = format!("(Z|(\\+|-)((0{digit}|1[0-3]):{minute_frag}|14:00))");
    let date_lex_rep = format!("{year_frag}-{month_frag}-{day_frag}{timezone_frag}?");
    Regex::new(&format!("^{date_lex_rep}$")).unwrap()
});

pub static RE_XS_DATE_TIME: Lazy<Regex> = Lazy::new(|| {
    let digit = "[0-9]";
    let year_frag = format!("-?(([1-9]{digit}{digit}{digit}+)|(0{digit}{digit}{digit}))");
    let month_frag = "((0[1-9])|(1[0-2]))";
    let day_frag = format!("((0[1-9])|([12]{digit})|(3[01]))");
    let hour_frag = format!("(([01]{digit})|(2[0-3]))");
    let minute_frag = format!("[0-5]{digit}");
    let second_frag = format!("([0-5]{digit})(\\.[0-9]+)?");
    let end_of_day_frag = "24:00:00(\\.0+)?";
    let timezone_frag = format!("(Z|(\\+|-)((0{digit}|1[0-3]):{minute_frag}|14:00))");
    let date_time = format!(
        "{year_frag}-{month_frag}-{day_frag}T(({hour_frag}:{minute_frag}:{second_frag})|{end_of_day_frag}){timezone_frag}?"
    );
    Regex::new(&format!("^{date_time}$")).unwrap()
});

pub static RE_XS_BASE64_BINARY: Lazy<Regex> = Lazy::new(|| {
    let b64_char = "[A-Za-z0-9+/]";
    let b04_char = "[AQgw]";
    let b16_char = "[AEIMQUYcgkosw048]";
    let b64 = format!("{b64_char} ?");
    let b04 = format!("{b04_char} ?");
    let b16 = format!("{b16_char} ?");
    let b64_quad = format!("({b64}{b64}{b64}{b64})");
    let b64_final_quad = format!("({b64}{b64}{b64}{b64_char})");
    let padded8 = format!("{b64}{b04}= ?=");
    let padded16 = format!("{b64}{b64}{b16}=");
    let b64_final = format!("({b64_final_quad}|{padded16}|{padded8})");
    let base64_binary = format!("({b64_quad}*{b64_final})?");
    Regex::new(&format!("^{base64_binary}$")).unwrap()
});

pub static RE_MIME_TYPE: Lazy<Regex> = Lazy::new(|| {
    let tchar = r"[!#$%&'*+\-.^_`|~0-9a-zA-Z]";
    let token = format!("({tchar})+");
    let ows = r"[ \t]*";
    let obs_text = r"[\u{80}-\u{FF}]";
    let qd_text = format!(r"([\t !#-\[\]-~]|{obs_text})");
    let quoted_pair = format!(r"\\([\t !-~]|{obs_text})");
    let quoted_string = format!(r#""({qd_text}|{quoted_pair})*""#);
    let parameter = format!("{token}=({token}|{quoted_string})");
    let media_type = format!("^{token}/{token}({ows};{ows}{parameter})*$");
    Regex::new(&media_type).unwrap()
});

pub static RE_RFC_2396: Lazy<Regex> = Lazy::new(|| {
    let alphanum = "[a-zA-Z0-9]";
    let mark = r"[-_.!~*'()]";
    let unreserved = format!("({alphanum}|{mark})");
    let hex = "([0-9]|[aA]|[bB]|[cC]|[dD]|[eE]|[fF])";
    let escaped = format!("%{hex}{hex}");
    let pchar = format!("({unreserved}|{escaped}|[:@&=+$,])");
    let param = format!("({pchar})*");
    let segment = format!("({pchar})*(;{param})*");
    let path_segments = format!("{segment}(/{segment})*");
    let abs_path = format!("/{path_segments}");
    let scheme = "[a-zA-Z][a-zA-Z0-9+\\-.]*";
    let userinfo = format!("({unreserved}|{escaped}|[;:&=+$,])*");
    let domainlabel = format!("({alphanum}|{alphanum}({alphanum}|-)*{alphanum})");
    let toplabel = format!("([a-zA-Z]|[a-zA-Z]({alphanum}|-)*{alphanum})");
    let hostname = format!("({domainlabel}\\.)*{toplabel}(\\.)?");
    let ipv4address = "[0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+";
    let host = format!("({hostname}|{ipv4address})");
    let port = "[0-9]*";
    let hostport = format!("{host}(:{port})?");
    let server = format!("(({userinfo}@)?{hostport})?");
    let reg_name = format!("({unreserved}|{escaped}|[$,;:@&=+])+");
    let authority = format!("({server}|{reg_name})");
    let net_path = format!("//{authority}({abs_path})?");
    let reserved = "[;/?:@&=+$,]";
    let uric = format!("({reserved}|{unreserved}|{escaped})");
    let query = format!("({uric})*");
    let hier_part = format!("({net_path}|{abs_path})(\\?{query})?");
    let uric_no_slash = format!("({unreserved}|{escaped}|[;?:@&=+$,])");
    let opaque_part = format!("{uric_no_slash}({uric})*");
    let absolute_uri = format!("{scheme}:({hier_part}|{opaque_part})");
    let fragment = format!("({uric})*");
    let rel_segment = format!("({unreserved}|{escaped}|[;@&=+$,])+");
    let rel_path = format!("{rel_segment}({abs_path})?");
    let relative_uri = format!("({net_path}|{abs_path}|{rel_path})(\\?{query})?");
    let uri_reference = format!("^({absolute_uri}|{relative_uri})?(#{fragment})?$");
    Regex::new(&uri_reference).unwrap()
});

pub static RE_BCP_47: Lazy<Regex> = Lazy::new(|| {
    let alphanum = "[a-zA-Z0-9]";
    let singleton = "[0-9A-WY-Za-wy-z]";
    let extension = format!("{singleton}(-({alphanum}){{2,8}})+");
    let extlang = "[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2}";
    let irregular = "(en-GB-oed|i-ami|i-bnn|i-default|i-enochian|i-hak|i-klingon|i-lux|i-mingo|i-navajo|i-pwn|i-tao|i-tay|i-tsu|sgn-BE-FR|sgn-BE-NL|sgn-CH-DE)";
    let regular =
        "(art-lojban|cel-gaulish|no-bok|no-nyn|zh-guoyu|zh-hakka|zh-min|zh-min-nan|zh-xiang)";
    let grandfathered = format!("({irregular}|{regular})");
    let language = format!("([a-zA-Z]{{2,3}}(-{extlang})?|[a-zA-Z]{{4}}|[a-zA-Z]{{5,8}})");
    let script = "[a-zA-Z]{4}";
    let region = "([a-zA-Z]{2}|[0-9]{3})";
    let variant = format!("(({alphanum}){{5,8}}|[0-9]({alphanum}){{3}})");
    let privateuse = format!("[xX](-({alphanum}){{1,8}})+");
    let langtag =
        format!("{language}(-{script})?(-{region})?(-{variant})*(-{extension})*(-{privateuse})?");
    let language_tag = format!("({langtag}|{privateuse}|{grandfathered})");
    Regex::new(&format!("^{language_tag}$")).unwrap()
});

pub static RE_XS_ANY_URI: Lazy<Regex> = Lazy::new(|| {
    // Simplified IRI pattern per RFC 3987
    let scheme = "[a-zA-Z][a-zA-Z0-9+\\-.]*";
    let ucs_char = r"[\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}]";
    let iunreserved = format!(r"([a-zA-Z0-9\-._~]|{ucs_char})");
    let pct_encoded = "%[0-9A-Fa-f][0-9A-Fa-f]";
    let sub_delims = "[!$&'()*+,;=]";
    let iuserinfo = format!("({iunreserved}|{pct_encoded}|{sub_delims}|:)*");
    let h16 = "[0-9A-Fa-f]{1,4}";
    let dec_octet = "([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])";
    let ipv4 = format!("{dec_octet}\\.{dec_octet}\\.{dec_octet}\\.{dec_octet}");
    let ls32 = format!("({h16}:{h16}|{ipv4})");
    let ipv6 = format!("(({h16}:){{6}}{ls32}|::({h16}:){{5}}{ls32}|({h16})?::({h16}:){{4}}{ls32}|(({h16}:)?{h16})?::({h16}:){{3}}{ls32}|(({h16}:){{0,2}}{h16})?::({h16}:){{2}}{ls32}|(({h16}:){{0,3}}{h16})?::{h16}:{ls32}|(({h16}:){{0,4}}{h16})?::{ls32}|(({h16}:){{0,5}}{h16})?::{h16}|(({h16}:){{0,6}}{h16})?::)");
    let unreserved = r"[a-zA-Z0-9\-._~]";
    let ipvfuture = format!("[vV][0-9A-Fa-f]+\\.({unreserved}|{sub_delims}|:)+");
    let ip_literal = format!("\\[({ipv6}|{ipvfuture})\\]");
    let ireg_name = format!("({iunreserved}|{pct_encoded}|{sub_delims})*");
    let ihost = format!("({ip_literal}|{ipv4}|{ireg_name})");
    let port = "[0-9]*";
    let iauthority = format!("({iuserinfo}@)?{ihost}(:{port})?");
    let ipchar = format!("({iunreserved}|{pct_encoded}|{sub_delims}|[:@])");
    let isegment = format!("({ipchar})*");
    let ipath_abempty = format!("(/{isegment})*");
    let isegment_nz = format!("({ipchar})+");
    let ipath_absolute = format!("/(({isegment_nz}(/{isegment})*))?");
    let ipath_rootless = format!("{isegment_nz}(/{isegment})*");
    let ipath_empty = format!("({ipchar}){{0}}");
    let ihier_part =
        format!("(//{iauthority}{ipath_abempty}|{ipath_absolute}|{ipath_rootless}|{ipath_empty})");
    let iprivate = r"[\u{E000}-\u{F8FF}\u{F0000}-\u{FFFFD}\u{100000}-\u{10FFFD}]";
    let iquery = format!("({ipchar}|{iprivate}|[/?])*");
    let ifragment = format!("({ipchar}|[/?])*");
    let isegment_nz_nc = format!("({iunreserved}|{pct_encoded}|{sub_delims}|@)+");
    let ipath_noscheme = format!("{isegment_nz_nc}(/{isegment})*");
    let irelative_part =
        format!("(//{iauthority}{ipath_abempty}|{ipath_absolute}|{ipath_noscheme}|{ipath_empty})");
    let irelative_ref = format!("{irelative_part}(\\?{iquery})?(#{ifragment})?");
    let iri = format!("{scheme}:{ihier_part}(\\?{iquery})?(#{ifragment})?");
    let iri_reference = format!("({iri}|{irelative_ref})");
    Regex::new(&format!("^{iri_reference}$")).unwrap()
});

// ---- Public matching functions ----------------------------------------------

/// Check that `text` is a valid short ID.
pub fn matches_id_short(text: &str) -> bool {
    RE_ID_SHORT.is_match(text)
}

/// Check that `text` is a valid version string.
pub fn matches_version_type(text: &str) -> bool {
    RE_VERSION_TYPE.is_match(text)
}

/// Check that `text` is a valid revision string.
pub fn matches_revision_type(text: &str) -> bool {
    RE_REVISION_TYPE.is_match(text)
}

/// Check that `text` conforms to the pattern of xs:dateTime with UTC timezone.
pub fn matches_xs_date_time_utc(text: &str) -> bool {
    RE_XS_DATE_TIME_UTC.is_match(text)
}

/// Check that `text` conforms to the pattern of a MIME type.
pub fn matches_mime_type(text: &str) -> bool {
    RE_MIME_TYPE.is_match(text)
}

/// Check that `text` matches the URI pattern defined in RFC 2396.
pub fn matches_rfc2396(text: &str) -> bool {
    RE_RFC_2396.is_match(text)
}

/// Check that `text` is a valid BCP 47 language tag.
pub fn matches_bcp47(text: &str) -> bool {
    RE_BCP_47.is_match(text)
}

/// Check that `text` conforms to the pattern of Constraint AASd-130 (XML serializable).
pub fn matches_xml_serializable_string(text: &str) -> bool {
    RE_XML_SERIALIZABLE_STRING.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:anyURI.
pub fn matches_xs_any_uri(text: &str) -> bool {
    RE_XS_ANY_URI.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:base64Binary.
pub fn matches_xs_base64_binary(text: &str) -> bool {
    RE_XS_BASE64_BINARY.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:boolean.
pub fn matches_xs_boolean(text: &str) -> bool {
    RE_XS_BOOLEAN.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:date.
pub fn matches_xs_date(text: &str) -> bool {
    RE_XS_DATE.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:dateTime.
pub fn matches_xs_date_time(text: &str) -> bool {
    RE_XS_DATE_TIME.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:decimal.
pub fn matches_xs_decimal(text: &str) -> bool {
    RE_XS_DECIMAL.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:double.
pub fn matches_xs_double(text: &str) -> bool {
    RE_XS_DOUBLE.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:duration.
pub fn matches_xs_duration(text: &str) -> bool {
    RE_XS_DURATION.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:float.
pub fn matches_xs_float(text: &str) -> bool {
    RE_XS_FLOAT.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:gDay.
pub fn matches_xs_g_day(text: &str) -> bool {
    RE_XS_G_DAY.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:gMonth.
pub fn matches_xs_g_month(text: &str) -> bool {
    RE_XS_G_MONTH.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:gMonthDay.
pub fn matches_xs_g_month_day(text: &str) -> bool {
    RE_XS_G_MONTH_DAY.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:gYear.
pub fn matches_xs_g_year(text: &str) -> bool {
    RE_XS_G_YEAR.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:gYearMonth.
pub fn matches_xs_g_year_month(text: &str) -> bool {
    RE_XS_G_YEAR_MONTH.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:hexBinary.
pub fn matches_xs_hex_binary(text: &str) -> bool {
    RE_XS_HEX_BINARY.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:time.
pub fn matches_xs_time(text: &str) -> bool {
    RE_XS_TIME.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:integer.
pub fn matches_xs_integer(text: &str) -> bool {
    RE_XS_INTEGER.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:long.
pub fn matches_xs_long(text: &str) -> bool {
    RE_XS_LONG.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:int.
pub fn matches_xs_int(text: &str) -> bool {
    RE_XS_INT.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:short.
pub fn matches_xs_short(text: &str) -> bool {
    RE_XS_SHORT.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:byte.
pub fn matches_xs_byte(text: &str) -> bool {
    RE_XS_BYTE.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:nonNegativeInteger.
pub fn matches_xs_non_negative_integer(text: &str) -> bool {
    RE_XS_NON_NEGATIVE_INTEGER.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:positiveInteger.
pub fn matches_xs_positive_integer(text: &str) -> bool {
    RE_XS_POSITIVE_INTEGER.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:unsignedLong.
pub fn matches_xs_unsigned_long(text: &str) -> bool {
    RE_XS_UNSIGNED_LONG.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:unsignedInt.
pub fn matches_xs_unsigned_int(text: &str) -> bool {
    RE_XS_UNSIGNED_INT.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:unsignedShort.
pub fn matches_xs_unsigned_short(text: &str) -> bool {
    RE_XS_UNSIGNED_SHORT.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:unsignedByte.
pub fn matches_xs_unsigned_byte(text: &str) -> bool {
    RE_XS_UNSIGNED_BYTE.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:nonPositiveInteger.
pub fn matches_xs_non_positive_integer(text: &str) -> bool {
    RE_XS_NON_POSITIVE_INTEGER.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:negativeInteger.
pub fn matches_xs_negative_integer(text: &str) -> bool {
    RE_XS_NEGATIVE_INTEGER.is_match(text)
}

/// Check that `text` conforms to the pattern of an xs:string.
pub fn matches_xs_string(text: &str) -> bool {
    RE_XS_STRING.is_match(text)
}

/// Check if the BCP 47 tag corresponds to English.
pub fn is_bcp47_for_english(text: &str) -> bool {
    RE_IS_BCP47_FOR_ENGLISH.is_match(text)
}
