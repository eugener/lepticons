
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideGlyph {
    #[cfg(feature = "a_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDIIINhILQ0CIPg8C+A4FheGoGggIhNDEOIODYVg3haGIdhyBIegkbYQDENoRC4MAzDkLQ5C4Ng5GELg1j4IAwkGDggC4OQyDOQRMDEMYkiiGYsiuBYHgkTQzC4Mwwg0MQzGgNpYkeT4qgE",
        categories = "text,design",
        tags = "letter,font size,text,formatting,smaller",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowDown,
    #[cfg(feature = "a_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDEIA0C2DYUCIPg8C+A4FheGoGggIhNDEOIODYVg3haGIdhyBIegkbQyiSEQuDAMw5C0OQuDYORhC4NY9CAMJAg4IAuDkMgzkATIPiSKIZiyK4FgeCRNDMLgzDCDQxDMaA2laRpNiqAQ",
        categories = "text,design",
        tags = "letter,font size,text,formatting,larger,bigger",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowUp,
    #[cfg(feature = "a_large_small")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDYIAyC4NQzDYLQ3C4MwyDgYQxC4MAyg6H4hDEIAwg6Ig5huJhMDKJA2CIPg8C+A4FjKNYGggIhNgwLg2DkN4ODQaA1j4MIwjKNIEGiN5LgeCRti8IA0h8Mw5C0OY+DkYYThOJpfiULoqDOLAxiWD4xjOOJNgWT47DOGQwDSDgzGgNoZiqaZKjaAQA",
        categories = "text,design",
        tags = "letter,font size,text,formatting",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis,vichotech,karsa-mistmere"
    ))]
    ALargeSmall,
    #[cfg(feature = "accessibility")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDENoOGMeYLDQIg+DwL4DgWB4bHAYR0GgIBkgsbQxDgIAxDmLAtDcLQ2iyGociKJIhiOJYnCIbQ1CCKwzC0MwgDULo/kIMguDOMwzkeNQvjcaI5iSJoLE0NAuDINIslkNRhj+PwwCCY5jDYLg4iuFIblGOpUjuVwxk4N4zDEN5HmCRZknsMIymgOIylCUg+gEA",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[cfg(feature = "activity")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIaAtDILg0DgYYNg0MAghoMAtDELg5DODoUDYbISC4Mw1CAOIoDYYQuDINYwiqHIOC2FA4hsTA5jANAghMMYWjOM4bkWHY4huJoTimK4tEGGJGg4IA0hQOYODISAyCIPg8C+A4FD6AQ",
        categories = "medical,account,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "colebemis,jguddas"
    ))]
    Activity,
    #[cfg(feature = "air_vent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDcLg1GEMoRCCFA1CAMIOg4LQ0hYLgwDMVgxDIIg+DwL4DgWJ4qgaCIKDaDgyEgNIThaGY4DELQyjsVoSDKN4ahoMYWjsaAxDaNpAkKG5ADIdo/kGOY7hYaI7iaKItiyBIugkTYxDiR4lieKZcluBYHl4NgujEMQ1hENwyEGTo5jmQw3HYLQ1liZYrgE",
        categories = "home",
        tags = "air conditioner,ac,central air,cooling,climate-control",
        contributors = "karsa-mistmere,jguddas"
    ))]
    AirVent,
    #[cfg(feature = "airplay")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ3EgNBhDIIITDAIIWDELQyhoVg1hKFIXiEMYUhoaAxDaH4ViKIAyHYMQwimIYYhqFBoC0MQiD4PAvgOBY6j2BoICIbQxhMMYMgwNhIDcWo5juQA+gEA",
        categories = "multimedia,connectivity",
        tags = "stream,cast,mirroring,screen,monitor,macos,osx",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Airplay,
    #[cfg(feature = "alarm_clock_check")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwOIQGMeYMDEMwiD4PAvgOBYHhwcBhHQaAgGSDBNDUIAzCAMggDaG4diOJYiiSJooCIbQyi8NgtDOP4yC+NBojaJYnikNguDMOAgDEOAuDcIA0i4MZCkSRo4ikMQ3C4NpUk+XpSDIMJVleN5ZkiOg5k6LYvi8NAtDSZ41gEA",
        categories = "devices,notifications,time",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockCheck,
    #[cfg(feature = "alarm_clock_minus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHCITQ5k0MxojCG5CmeAQ",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockMinus,
    #[cfg(feature = "alarm_clock_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4OA3CCDIOGEOAghQMAgDEIIXDEMQuDINoYh2HwiD4PAvgOBYliiBoIgoMQ5C4OYYDSHg1hOFYajkMAtjAMQ1jwLo+iSJoriqBIsgkbQyDKEAtDOTpDieR5GgWB4JguHogDEOAuDaDw0CAMgxlGRYCkeVgikqYZhhcMgwmSU5mlWLRNmCYJMDacIpgE",
        categories = "devices,notifications,time",
        tags = "morning,turn-off",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    AlarmClockOff,
    #[cfg(feature = "alarm_clock_plus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeILDEMoOGMeYSDMIg+DwL4DgWB4bHAYR0GgIBkgsTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHCITYTk0MB2jCG5CmcPJDmkTQ5k0MxonCMpngE",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockPlus,
    #[cfg(feature = "alarm_clock")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTYOCAOR2DQbAyCCD4bC+IokiGI4licIhNDUIAzjEIA2hqHI1GiN4kiaDBtDKMA2C0M5PkONI4keOYoDYLgzDgIAxDgLg3CANIxDGUpFlWSY7DENwuDaYpdmyYAyDCY5llSAQ",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_smoke")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDEYwwC0MguDWDoThWFw1CIPg8C+A4Fh2IIGgiCgxDaDoQhKFIWDKGItC2G4dh+BBoiKNYHgkbQxDkIA4C0Lg4CAMxhDELgyhaRpICAMJMCAMQtkqTxIDeRZHkmV5Ok2UJKlETIWDiHIeiON4FjmCoPkORZPlqbINDEdgyGEMoOm2UJ0DISA0nOdZNluE4TFae5vnaT5RHqYo0iGAo4iUTYog+EYvheLIaomZIBA",
        categories = "home,devices,travel",
        tags = "fire,alert,warning,detector,carbon monoxide,safety,equipment,amenities",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    AlarmSmoke,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEeYUhEcoTCIMoRGgZRpGcaB0g2D4ZheHAiD4PAvgOBYrHAbxsHkbBpG4ZQgjGNh0HOJQxCAMwgDGP5DkINAgDiQg3kKRJLhWKwvjGM41jcPoBA",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "align_center_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGgMgwCIPg8C+A4FhSF4GgiCgxDCDQ2HYNBhgyDIfh8MQtiUSA2iQIImCCKIqiodgtDSE4VhqGYEhuCRNh4IA4FaI4ljGRgwjMMosi6MIniqL4ijiFo8juBYHj6EYgHYMZMkeDZPg+KpdieXwyjQLQxlKOoCjyV4dDSQRWDcY5IDELp2DmYIqhCY5GDGL5QmmFJThiAQ",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterHorizontal,
    #[cfg(feature = "align_center_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgwCIPg8C+A4FhSF4GgiCg4CAMQwEgNBhg2DQwCCJwxC0MorFYNhjDALQxC6Mw5iuDorGgNIThWGoZgSG4JgsNofDCOokg6KJKieLAyi6SImkuK45C2O4UhaQI/gWB5Ch6ERIDeUJLh+U4PiuMIyjQLo2iWOQxjyWIYgKQJcgoMZEDENBoDGYonimSYPDKfZKiqJRojKcI+gE",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterVertical,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB4g0NIQHmDQyhAaBlGkZxoHSDQxg8IByhIIoYD4PAvgOBYoiuBoWCIOYQgiCoMg6EIkheEIlDGFAghqHIeg2MooiqBB0igcBhgsIBkg0TQyDIIJREiJ4pkqCw+gE",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDSDBygiCoMHcaRkHQaIJDENoMGgZRpGcaB0gmHQ+DwL4DgWJ4qgaEoJguB4JDmHogiKJAih2DYbhAIIWhiGgijSJ4pgQdInHAYYZCAZIJE0MgyCCUBWguRJJhkPoBA",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIByHiDQyhCEwiDSEBoGUaRnGgdINDGGQgHmDQ1CIPg8C+A4FimLIGheIoQhKFIQgiCoMg6EIlCIN4ahyHogCIMQwiiKovikcBhgsIBkg0TQxDcIAyDIdgtieKQvkqC5JksaJNk+UQgDcVoVlmWxol2TJOCITZSlSVgzkaWpemqX5sm4IA1mWc5oD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIBoGUaRnGgdINDENIQHmDQ1hAch4g0MoQiEIoaD4PAvgOBYoiuBogiKEIShSFoYDCG4NDeEIIgqDIOiSGInimLooHAYYLCAZINE0MQwCAMh2DKN4oC+RoLkWRxokmS5Sk+UZTimVhoD6AQA",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENAiCAdxpGQdBog4NoSHmDg1hIch4g4MoSiEIoRD4PAvgOBYoiuBolhCG4ODeH4liOE4VheGYSgiCoMg4MQwCKKIqgQdIoHAYYXCAZIOE0NAgDIdgykKRJJheSJKGiTJOhCUZTlWKZXGgPoBA",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQxDSDB5gkNYMHKCIKgwdxpGQdBogkNgiD4PAvgOBYoiuBoOhCEoUDCDIdDGJgghgIg3huHYLCCIIiiQIomiiKoEHSKBwGGIwgGSCRNDEMggDIdgyjSRpLiMPoBA",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAch4gkMoMHcaRkHQaIJDaDBoGUaRnGgdIJDENIMg8IoRD4PAvgOBYoiuBoThWFwihmB4JDeJIhhEIIbh2H4hDCDIOhAIooiqBB0igcBhhYIBkgkTQyDIIAyHYMpAkWSoWD6AQ",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB5g0NYQHIeINDKEBoGUaRnGgdINDENIQheDgiD4PAvgOBYoiuBobh2H4hDCFYlhmEYNDeEIIgqDImCCJQxg+KIqgQdIoHAYYLCAZINE0MggDIdgyjSRJJgsPoBA",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAdxpGQdBogkNoMHIeIJDKDIXCIOYMGgZRpGcaB0gkMQwCIPg8C+A4FikcBhhEIBkgkTQ0CAMgyFaGYpC+L4Ri6MBojKNAyDCN45juKo+GgPoBA",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENAiCAeYODWEhyHiDgyhIdxpGQdBog4NoShkIgzCIPg8C+A4FimLIGh2H4hCKIwgiUMYWCCGIahKCIKgyDgxDCEoUCIN4oiqL4pHAYYgCAZIOE0MwgDIdgykOKQvkyIJLk0aJPlEMgxlSVpYiqWxoD6AQ",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCg2g2CQiDSDR5hSDRoGUaRnGgdIKDGFQ+DwL4DgWJ4qgaFwxhkIIeiCIoKDmGocg6EISjmCIKgyJ4pgQdInHAYYSCAZIKE0MgyCAMhIkCKJGhIPoBA",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIBoGUaRnGgdIJDaDR3GkZB0GiCQ5g0eIVg0cogCIMgiD4PAvgOBYoiuBoYhqHIKhaB4JgwIIjgmJggiSNIPhGE4hiiKoEHSKBwGGGwgGSCRNDIIAyHYMgwieKZIhsPoBA",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcaAtDMIg+DwL4DgWFYYgaCIKgwIIQC0NYUhaG4agSHIJE0NYODcSAyiSF4oieBYHioN4gi+MYmDwchlGMdAgGgZRpGcaB0gkNgiCAeIJiMIB5gkMQ0ksdxpGSBZSlQIByk0IowhUL4+kCFZjkGUQiluXYJjCQpEkaSAikoIJWlgaJSDCS5eDeMZmD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis,jguddas"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHIeINDKEYUCINYRGgZRpGcaB0g0NoRHmDYPCIPg8C+A4FimLIGiUIoQCCHIeiCIoRgiCoMg4MIXg0N4RhOFYoiqL4pHAYYLCAZINE0MggDIMBolKRQvkmC5IkoaJMk6UAxlOVYpleWw+gE",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHmDYPhEch4g0MoRGgZRpGcaB0g0NoRhgIg1CIPg8C+A4FimLIGhMIoQCCF4ZhGCIKgyDgwiSDQ3huHYfiEIojimK4EHSKRwGGCwgGSDRNDIIIPGgMo8kaS4LkqTBok6UJSDSVZXiqWRoD6AQ",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ2CIIBoGUaRnGgdIJgwIB3GkZB0GiCQxDSDR4gkNYNHKIQiDIIg+DwL4DgWKotgaD4RhOFYNgiJ4NhiGocgoMIggkN4kiaKIqiyBB0iocBhhsIBkgkTQyCAMQyGgMo+kWSobD6AQ",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQyCIIByHiCYMCAdxpGQdBogkMQ0g2EAiDWDRoGUaRnGgdIJDYIg+DwL4DgWKotgaIYjiWJ4NhSFoYgoMIcgkN4Ng+EYNggIoMiqLIEHSKhwGGFwgGSCRNDIIAyDIaAyjuRpLhcPoBA",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ2CIIBoGUaRnGgdIJgwIB4gkNYNHKGAiDKDR3GkZB0GiCQxDQIg+DwL4DgWKotgaHIJh+DoQhKFAihaCI5iCIokiYMINh0N4piuMIqHAYYkCAZIJE0MggDIaAykGKgvkmJA+gE",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDAIggHmDQ5hEaBlGkZxoHSDQ2hEeINDeERyiAIgyCIPg8C+A4FikcBhgsIBkg0TQyDIIAyDASInikL4vguLowGiMo0jYIA0juKIqj8aA+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHIeIODKEYUCINYRHmDgxhoIB3GkZB0GiHQ0CIPg8C+A4FimLIGhOFYRgiCoMg6EAghgN4bg4M4RiGI4lCIMQwiiKovikcBhiQIBkg4TQyCAMgxGgMpFikL5KiSSZLGiTZPlEM5VleKpaGgPoBA",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceBetween,
    #[cfg(feature = "ambulance")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMEgNgiD4PAvgOBYThaBoIgoMQ0g4OBWDYYQyCCJINicLQyikSA0iOJQgieMIpiUdgxDEYQxg6MI7g2OQxGgMoShSGYYgSGoJgsOYfkCOI6jGPQtDEdgtDMLgyDiTY5k8LQuDYOA0lwOQ0DgbJRC4OQyDOXA2DSN4+jyDpcDUNw4nIMAymUMQuDUMw5lQLgwDcNhBm+T4ODYLgzDiJA4EiHZChWRpFgWB5IDgIA4HYNKQkSApGpWCpKDEOBohGE6RhcPBjGkchjGwZQgGMeIJDENwirEea0DitxygmQanquravhOwaurCvQikGuK6resoJrawKssYPoBA",
        categories = "medical,transportation",
        tags = "ambulance,emergency,medical,vehicle,siren,healthcare,transportation,rescue,urgent,first aid",
        contributors = "jordan808,jguddas,colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley"
    ))]
    Ambulance,
    #[cfg(feature = "ampersand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDIaAzCIPg8C+A4FhSF4GgiCgxDcLg1g4MhhDgIIlDAIIoDELYlDgQQ0iAIIwiGKIqCCIQxDWIBjDALYNDgLQ0iaLIgGEMwgkeNo9g0MI8kgIAyjEOIxg+DoShSFoEGgPoBA",
        categories = "text,development",
        tags = "and,typography,operator,join,concatenate,code,&",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Ampersand,
    #[cfg(feature = "ampersands")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDcYwtDULQzC0N4WhgORhDIIIcg2DQxCANAgDAY4NDILoTDWHYphIIA2iSDguDeMgzCCNo4iwOIxhOKIcioIg+DwL4DgWQpFgaCIKDKHIPhGE4VheUgthqHIejGIYjiWJ4tiuKIqi+WIzjWN5ljaKI7g2PQuj+EpBkOSA+gE",
        categories = "text,development",
        tags = "and,operator,then,code,&&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersands,
    #[cfg(feature = "amphora")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYNQuDYMwyGODQuDQMg0C0LgyDcMguDcOQ1hwNg1DMLg5DgMhBDYIIug2MYvCAMQ0GMLgwDCLg0CAMwgDcIA1CAOAiD4PAvgOBZGkmBoIgqDJBEgOBhDKDggjKMg0GiEpEkaSIEGiS5ggeCYLjyD4RhOFYXhkNIdh+IYjhINYgiqLIujCV40jQOI0jaDYbj4NwtkKXZHkyYoFmST48DUaAylSVoyDGepaiWhpfkqApjk6C59DIMhIDaRaHmCiZNmUOYOGio5eoiAQ",
        categories = "food-beverage,gaming",
        tags = "pottery,artifact,artefact,vase,ceramics,clay,archaeology,museum,wine,oil",
        contributors = "karsa-mistmere"
    ))]
    Amphora,
    #[cfg(feature = "anchor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYMQ2CIPg8C+A4FhSF4GggIhtDEOQgDEMwgDILQxGGIIgDAIIrDGJg4iwbINDGE4VhqGYEhuCRNiAMQxGiEoUhaOYUGMaRyGMbBlCAYx4gmDAiCAcoJDKURjHmCQ0jUL5GkiSg+gEA",
        categories = "transportation,text",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Anchor,
    #[cfg(feature = "angry")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDYIIqHMLQxC4NQtDILQ0jONQgjQNI5CKGwviKJIhiOJYnCITQ3jEIA4iwMAgDmPYckAaJCiSJoLG0MY7DmOYxi+UI/kOVJEiiW4NGgLgwDGX5SmKVpGDENZMmeaZrmGAQ",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[cfg(feature = "annoyed")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgDENRoDiGYbiGI4giKJImCKKAgDkaIPhoL4xGiM4jiWJwxDSOY7i+Po0D6AQA",
        categories = "emoji",
        tags = "emoji,nuisance,face,emotion",
        contributors = "karsa-mistmere"
    ))]
    Annoyed,
    #[cfg(feature = "antenna")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMYMDcIAyCIPg8C+A4FhSF4GggIhthCDggDULQxDCE4VhqGYEhuCRtiCIIiiSJoWiqKYFgeLAxh+DIwiWFIzhiAoqjeCg0C4NQgDcaAxDWMookGNocE2Lg2HYNpNjSAQA",
        categories = "devices,multimedia,communication",
        tags = "signal,connection,connectivity,tv,television,broadcast,live,frequency,tune,scan,channels,aerial,receiver,transmission,transducer,terrestrial,satellite,cable",
        contributors = "danielbayley"
    ))]
    Antenna,
    #[cfg(feature = "anvil")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwEgNhhDQIITDAIIWDELQ0hqDYdhaGINC0MRoDQIg+DwL4DgWJ4qgaCIKgwNRhDGHoXh2GYjDEM4zjWII0jSDIMh+IZBEgOI8jSQ44iIeomiiLYsgSLoJE0OYNDIdg1k6KZSlGBYHlQMQ1leWZblCApSmCCpjDIMBhDMIJwkqcQtDMaJHnCco2jSeY3jaGIig2EJIn+IY4maXYBA",
        categories = "buildings,tools,gaming",
        tags = "metal,iron,alloy,materials,heavy,weight,blacksmith,forge,acme",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Anvil,
    #[cfg(feature = "aperture")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDQLgzDEIA4CANQuDcNAgDkLg5DQIobC+IokiGI4licIhNjcNg5i8aAxDELg0DiO4cj4aJAiSJooDeLIwg6MYzDQLY3jmUI9kGVJCgyRQukcIAxDYIAzjgNQgDYLgwDaYpSmWVpEiqLIumwSAyC4OAzneZA8lKeopnMNgymsMgtjKNI2jiOo8niAQ",
        categories = "photography",
        tags = "camera,photo,pictures,shutter,exposure",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window_mac")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAdxpGQdBogkMgwgwch4hODIYCIMoMGgZRpGcaB0gkMQ2CIPg8C+A4FikcBhhEIBkgkTQ2CAOBoC4MAxiiKovhGLowGiMo0DEMI3jmO49C+PxokGMYzCITQxDSSI6jyKZMkIPoBA",
        categories = "layout,design,development,files",
        tags = "application,menu bar,pane,preferences,macos,osx,executable",
        contributors = "danielbayley"
    ))]
    AppWindowMac,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAch4gkMoMGgZRpGcaB0gkMQ2gwdxpGQdBohAMIMg8IoRD4PAvgOBYoHAYYgCAZIJE0MQwCANB2guKAvi6IIti8aIxjMMggDgaAyiOO49GiP4wjIIhNDaN45CKSpAD6AQA",
        categories = "layout,design,development,files",
        tags = "application,menu bar,pane,executable",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    AppWindow,
    #[cfg(feature = "apple")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2C4NQyDgVgzGEMQghcMAghqFwxC0MRoDAIg+DwL4DgWJIngaCIKDEOAuDIMw3CAMgxEEMQ1hiOYajyNINDEMRhDaDobkUMIfkcNAuDQNwyEGQ5Dj2Go/kGOAuh0NZXkaRQzC4Nw2DOGIamGYZSCCXQ2DSL5pDgIJZjmb5bheWQxDebQwEGZJbhyL4xjONYjiWKg+gEA",
        categories = "food-beverage",
        tags = "fruit,food,healthy,snack,nutrition,fresh,produce,grocery,organic,harvest,vitamin,red,green,juicy,sweet,tart,bite,orchard,plant,core,raw,diet",
        contributors = "karsa-mistmere"
    ))]
    Apple,
    #[cfg(feature = "archive_restore")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQ1gwdxpGQdBogkMgwgweYJDODByggIgxCIPg8C+A4FikcBhhoIBkgkTQ0CAOB2DEMRhDIII9DAIJAkCPQyGiC4pC+L4ai6MBojKNIdjeOY7kSQZWDELZEGiWYoiqShokyMYzCIbQ5CAMQ1CAMwtDOapql2SZNmGTpjE0MY9ncdg5nCXw+gE",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIIJCIMoNHmCgzg0dxpGQdBogoMgwg0aBlGkZxoHSCg1CIPg8C+A4FikcBhhoIBkgoTQ0CAOB2DEMRhDIII9DAIJAkCPQyGgMQyjyPpBkuQwtDIVg4iiKovhqLowGiMoKG0OQuDUIAxDcIA1C2J4pC+VBolaMYzCKW5dl+PZemWU5XD6AQ",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDODByggIgxgwdxpGQdBogkMgwgwaBlGkZxoHSCQ1CIPg8C+A4FikcBhhoIBkgkTQ0CAOB2DEMRhDIII9DAIJAkCPQyGgMQyjyPpBkuQwtDIVg4iiKovhqLowGiMo0DGQJHGgNJSC+VBoD6AQA",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Archive,
    #[cfg(feature = "armchair")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA5FYNhhDIIITDAIIWDALQyhoSA3hKFIXiGGYTDIdgzCIPg8C+A4FimLIGgiCgzCAMYRiSIohiQaAxDSH4VjiG4lC0NY+jiGQ0hcdgxC6RJMkyOAxC2Tg1GgLQ5GGU5QlINZbFYMQxkWGIXC2SAwHqKIqi+LoEjCCRNDWNA4HYMpoiubJrgWB5ugycZznWaoBA",
        categories = "home",
        tags = "sofa,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Armchair,
    #[cfg(feature = "arrow_big_down_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA4GEMQghEMAghSEYXHYMoQhKFYdhaEhoDMLgyDkMxhC4NwwDeKIqh6HAuDWEojiobAtDYLolDkII3jmEIzDeMgyi2FIWC0MYsDiFY1jwNI2jgNInimK5Si6EYwkaMw2EiD4Xi6RQxFYOYbhOHYXkYegiD4PAvgOBZqm2BoIgqOg0GgNppmucA+gE",
        categories = "arrows,gaming,files",
        tags = "backwards,reverse,slow,direction,south,download",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigDownDash,
    #[cfg(feature = "arrow_big_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CANRhDEIIQDAIIThAMQtDEaA0g+EYUh6FoRHYNochKHoVhEaAzC4Mg5DMYQuDcMA3jCMomh0Lg1hGK4yGwLYzDAOA2CCP5BiSNoXDELg0DENIUj2RA2j4LpAiONIzjEN5HCCOIYjsNxIDiRoTmOEYYHoIg+DwL4DgUPoBA",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south",
        contributors = "Andreto,mittalyashu,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigDown,
    #[cfg(feature = "arrow_big_left_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA5GEMQghEMAghQMQthcMRWDQLg3DANxhh2H4iDeFYmDCGAuDKHwtC4NRsC0NguDkNAgjKNIQiqH4SjqJYUj+JgxiQbI3jWRYhh4N4kieQY9i0NRWDENoQhKTIRhkaAylSE5WhgdgtDSW5MiiGR6CIPg8C+A4Fmia4GgiCorg4dg2meaZuD6AQ",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,turn,corner",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigLeftDash,
    #[cfg(feature = "arrow_big_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg3DkMwgDEOYOhAYYODAN4YDcIAwh2HwxC4MoZC0Lg1FYMQ2GEMYSh+HosDELQxGiKowi6IIyHYLQ0iuLYej+MoyGgLY1j6OIxDEVg0huFw3hmG43DCMoiiSJhskQLg5DQIA2lmPIhiOHJghmUYghsegiD4PAvgOBQ+gE",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigLeft,
    #[cfg(feature = "arrow_big_right_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA5GGDYNDAIIThMMQtDEVg0C4NwwDcYYch6IQ3hSJYRC4MoeC0Lg1GwNguDkNAgi+MYQiiHggDGN4khWOYmiMbAtjSMpDiCHQ3iOJYWhiO4rDUVgxDaEI+j0MIYhgSIPhGSo5lcMR2C0NJThKJpdDEegiD4PAvgOBZqm2BoIgqMg5HYNppmucA+gE",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,turn,corner",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigRightDash,
    #[cfg(feature = "arrow_big_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLgyDANwgDEOQuDcOQzGGFYQhqEQwCCHgxC0MYOhALQuDUVgxDYYQxhKH4vDCIoiEgNYsi6HogjIMR2C0NI2i2OIuiEMRoiuLZAjCEoiFYNIchkN4blCHYvkeJA3iYNRsDYLg5DQIJbl2LJWhKY5BkiI5SHoIg+DwL4DgUPoBA",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigRight,
    #[cfg(feature = "arrow_big_up_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDYYQxg4IAwhSFgxC0MR2C0MoRhOFYVhKGAxGgMwuDIOQzGELg3DAN4si6FogCALg1hmJ4uGwLQ2C6KQ5juPQzDmEY4DeDpFjKFI3i0OIUjqPA5g2UA0iuLYvlaSYVjWRwyDANhIDiHoSjOIoOHaHZljOIYOHoIg+DwL4DgWb5ygaCIKDkIJdGgNpunCdQ+gEA",
        categories = "arrows,text,development,gaming",
        tags = "caps lock,capitals,keyboard,button,mac,forward,direction,north,faster,speed,boost",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigUpDash,
    #[cfg(feature = "arrow_big_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ5GEMYNCAMIThWEQxGgNIQhKFIdg0LQxHYLQ2huEYeheIBoDMLgyDkMxhC4NwwDeMYzhWHguDWIIsjMbAtjQMA4DaPwukGJIXjeE47DQMQ0hOPpAkIIJRiSNY0jIN5JhSOYNjwNxIDiJZJkgMR6CIPg8C+A4FD6AQ",
        categories = "arrows,text,development,gaming",
        tags = "shift,keyboard,button,mac,capitalize,capitalise,forward,direction,north",
        contributors = "Andreto,mittalyashu,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigUp,
    #[cfg(feature = "arrow_down_01")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiFxyGUYx0CAdxpGSBYJhUIBoGUaRnGgdIJDYIggHIeYJDKQZECKNx4gkMQ1haGIti+HIqgeCRNDGIokHYLQ2GgLZGiiHZTgWVYgk2IwwGiJ4YmKAQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDGIgxiUNBoC0MoWhiHYsgWLogDENYNDAaInjqKoXHIZRjHQIB5gkMYVCAdxpGSBYJlEaBlGkZxoHSCQ2CIIB4k8NZhHKTgijiKJJksPoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDown10,
    #[cfg(feature = "arrow_down_az")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNiQIA4GgLQ1haGIdiyBYuiAMQ1g2JQ2C4NRhDKQ4jkcMAgkoMQgj+JQxDCN4phuAoth8TY+g0NBoDUbI1CANpdlOOYBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDownAZ,
    #[cfg(feature = "arrow_down_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAzEgNQiD4PAvgOBYThaBoIgoMQyCAMgxFYN4ShSGYYgSGoJG0NggDENQgiyMQtDaJIVigPoBA",
        categories = "arrows,files",
        tags = "backwards,reverse,direction,south,download,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownFromLine,
    #[cfg(feature = "arrow_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIINg2DAiD4PAvgOBYThaBoIgqDAggwSA3FYN4ShSGQ+gE",
        categories = "arrows",
        tags = "direction,south-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownLeft,
    #[cfg(feature = "arrow_down_narrow_wide")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDEMYQGiJ4Yh2LIFi6IIxCAOBoDeFo2iqOIei+PAxDIaAxDCQYphuAQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownNarrowWide,
    #[cfg(feature = "arrow_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPCIPg8C+A4FhSF4GggIhNDGDA3HaDxIDeE4VhoPoBA",
        categories = "arrows",
        tags = "direction,south-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownRight,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQ0CIPg8C+A4FhSF4GggIhtDEOQgDkLQ3CAN4jiOE4VhqFBjGkchjGwZQgHKCQxCIIBjHiNQyjcYx5gkMo2hQL4ti+MQ+gEA",
        categories = "arrows",
        tags = "direction,south,waypoint,location,step,into",
        contributors = "danielbayley"
    ))]
    ArrowDownToDot,
    #[cfg(feature = "arrow_down_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcVgzCIPg8C+A4FhSF4GggIhtDaDgxCCH4jC0NoThWGoZgSG4JgsOQgDIMRIDWJ4WisPoBA",
        categories = "arrows,files,development",
        tags = "behind,direction,south,download,save,git,version control,pull,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownToLine,
    #[cfg(feature = "arrow_down_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRtDIMQgDiFI1hGJ4Yh2LIFi6IAxiINB2g6Fo5ioPoB",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,network,traffic,flow,mobile data,internet,sort,reorder,move",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownUp,
    #[cfg(feature = "arrow_down_wide_narrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDEMYQGgMQwhaGIdiyBYuiCMQgDgaA3jeKYbgKLYfjCMgxDIaInjiKg+gEA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowDownWideNarrow,
    #[cfg(feature = "arrow_down_za")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDeEB2g6FoYh2HIEh6CRNDENYQGgNRsC2Lw2jGJoZiqKYFgeLIuCAMgwHYLQzC4NRhDKRpAkoMAgk0MQgi8MBWkGOIogKKo9iCQYNDgaI0laOoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical,reverse",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDownZA,
    #[cfg(feature = "arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1HYMQ0CIPg8C+A4FhSF4GggIhtDEOQggwLQ3CAN4jiOE4VhoPoBA",
        categories = "arrows",
        tags = "backwards,reverse,direction,south",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDown,
    #[cfg(feature = "arrow_left_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CANgtDaDYRDYIg+DwL4DgWFYYgaCAiE0MwgDEMhoDENIUhaG4agSHIJE0MgxiEORWDWJ4XisPoBA",
        categories = "arrows",
        tags = "previous,back,direction,west,expand,fold,horizontal,<-|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftFromLine,
    #[cfg(feature = "arrow_left_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMwgDQIA3GyEA0CIPg8C+A4FheGoGgiCoQDcaAxDaFoYh2HIEh6CRtiQIAyDGDwtDSM4ziaGYqimBYHgkTQyDAIAxDcSIVheOIbgEA",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ArrowLeftRight,
    #[cfg(feature = "arrow_left_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQ5FYNQiD4PAvgOBYThaBoICIbQxgwNgtDYIIiiSEoUhmGIEhqCRNDeDQyGgMQ0iaFYqD6AQ",
        categories = "arrows",
        tags = "previous,back,direction,west,collapse,fold,horizontal,|<-",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftToLine,
    #[cfg(feature = "arrow_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDkLQ3hEIISDcIg+DwL4DgWGIbgaCAiE2D4ODISA1heGYeD6AQ",
        categories = "arrows",
        tags = "previous,back,direction,west,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeft,
    #[cfg(feature = "arrow_right_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANR2DENAiD4PAvgOBYThaBoIgoMgxCAMQyEgN4ShSGYYgSGoJG0MQ1h8OAgDYLYxjOJIVigPoBA",
        categories = "arrows",
        tags = "next,forward,direction,east,export,expand,fold,horizontal,|->",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightFromLine,
    #[cfg(feature = "arrow_right_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAzCANIQC2EQ0CIPg8C+A4FheGoGggIhNDIMAgDcSIVheGYEGiHIqgeCRtDgIAyDGE4ThKJ4Yh2LIFi6IIRDENxogyFo5ioPoB",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "karsa-mistmere"
    ))]
    ArrowRightLeft,
    #[cfg(feature = "arrow_right_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDISAzCIPg8C+A4FhSF4GggIhtDEMYODgIA2C2JImhOFYahmBIbgkTQyiANR2DENIohaLA+gEA",
        categories = "arrows,development",
        tags = "next,forward,direction,east,tab,keyboard,mac,indent,collapse,fold,horizontal,->|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightToLine,
    #[cfg(feature = "arrow_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FhSF4GggIhtg4IIMDcIA3C2Ig3hOFYaD6AQ",
        categories = "arrows",
        tags = "forward,next,direction,east,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRight,
    #[cfg(feature = "arrow_up_01")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyFhyGUYx0CAeIJDENQiCAeYJDSMxyjUIgyjMdxpGSBY2jMaBlGkZxoHSCYlhYL4qiyG4Eh2CRNDGIQyDAdgtDYaAtjyTIogKUYHlOMQglcaI3l+UQ+gEA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUp01,
    #[cfg(feature = "arrow_up_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDGIQxDAVg0GgLQyiaGIqimBYHiwMQ1CCMBoDSN4oDwchlGMdAgHcaRkgWCZDCAch5gmNggGgZRpGcaB0gmJQgHiCY+CIIJUCIMZDhYL5HkkPoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUp10,
    #[cfg(feature = "arrow_up_az")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDIMINGgLQ1iaGIqimBYHiwMQ1CAMQwFYNguDUYQykIIJFjyL4vDEIJJFaPo0iiAoqjmH47j0NBoDUbIyCANpalGNoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUpAZ,
    #[cfg(feature = "arrow_up_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIAxDYLQ0CANIRhEIg+DwL4DgWGIbgaCAiE0MQ3CAMgwFYNIXhmHodgSH4JG0MwgDiE4RhOE4qhqLotgWB4JE2JA0HaD45iyAQ",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,network,mobile data,internet,sort,reorder,move",
        contributors = "it-is-not,karsa-mistmere,ericfennis"
    ))]
    ArrowUpDown,
    #[cfg(feature = "arrow_up_from_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAOQgDcLQ3g+DwiD4PAvgOBYWhmBoICITQxDIIAxDYVgyhWF4chYYxpHIYxsGUIBjHmCQyDEIggHKCY3jIeI7ieFgviyLowD6AQ",
        categories = "arrows",
        tags = "direction,north,step,out",
        contributors = "danielbayley"
    ))]
    ArrowUpFromDot,
    #[cfg(feature = "arrow_up_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIA5C0NoQhAIA2CIPg8C+A4FheGoGggIhNDEMggDMdgxDSFoYh2HIEh6CRNDUIAyDEaInimGYtD6AQ",
        categories = "arrows,files,development",
        tags = "forward,direction,north,upload,git,version control,push,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpFromLine,
    #[cfg(feature = "arrow_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ3FYNxoDEMAiD4PAvgOBYWhmBoIgqDoNgyIoVheHA+gE",
        categories = "arrows",
        tags = "direction,north-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpLeft,
    #[cfg(feature = "arrow_up_narrow_wide")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDEMQgDEMhoDSJoYiqKYFgeLIujANhoDeNYogKKo6h+PAyDAaAxDCQY3gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "lukesmurray,ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowUpNarrowWide,
    #[cfg(feature = "arrow_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CANxoDEMB2hAIg+DwL4DgWFYYgaCIKgwMYfgwN4UhaGw+gE",
        categories = "arrows",
        tags = "direction,north-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpRight,
    #[cfg(feature = "arrow_up_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxoDENAiD4PAvgOBYThaBoICIbQxDgIAxDMLQ2iOIwgDaEoUhmGIEhqCRNDEMggDcdoQimFYtD6AQ",
        categories = "arrows,files",
        tags = "forward,direction,north,upload,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpToLine,
    #[cfg(feature = "arrow_up_wide_narrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDEMQgDEMhoDEMImhiKopgWB4si6MA2GgN42iiAoqjuH49DIMBoDSQo4gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpWideNarrow,
    #[cfg(feature = "arrow_up_za")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDENYPGgNRsC2Lw2jGJoYiqKYFgeLIuCAMgwHYLQzC4NRhDKRpAkoMAgk0MQgi8MBWkGOIogKKo9h+QQgDEOBojSVo6gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical,reverse",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUpZA,
    #[cfg(feature = "arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAMQyCANwtDeEIQCIPg8C+A4FheGoGggIhNg6DQ5FYNYWhiHQ+gE",
        categories = "arrows",
        tags = "forward,direction,north",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUp,
    #[cfg(feature = "arrows_up_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CANggDMLQzg+DwiD4PAvgOBYWhmBoICITQ3CAMQ3FYM4VheHIbgSHYJG0MYMg6EISjOJ4YiuKoFgeCRNiOIokiaFo2hqAorjqH4MDIMRoDENo1imAQ",
        categories = "arrows,transportation,mail",
        tags = "direction,orientation,this way up,vertical,package,box,fragile,postage,shipping",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowsUpFromLine,
    #[cfg(feature = "asterisk")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HaDAiD4PAvgOBYThaBoIgoMQ3C4MQ5DYIA5g4Lg4DANAgDENYShSGYYgSGoJG0NomiiI4qDALgzDmDQ2i2FYxD6AQ",
        categories = "text,math,development",
        tags = "reference,times,multiply,multiplication,operator,code,glob pattern,wildcard,*",
        contributors = "mittalyashu,ericfennis"
    ))]
    Asterisk,
    #[cfg(feature = "at_sign")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeILDEMoOGMeYShQPg8C+A4FgeGhwGEdBoCAZILE0MQ2CAOB2DUYQzCCMAwCCM4zioMB2C0MRhDGM49jQIAxjQLQ0isIoaC+IYjD6AQA",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMYQGMeITg8Pg8C+A4FgeGhwGEdBoCAZIME0MgwC4MggimKxjDILgwDQLYxDAM4yDILQ3C4Mw2C0NAuDULQxDELg5kCQo0kENY6DkLg4DcLQ2kqRJGkiTI1jINItjIMwtjkII8j4IJMCCRZHmWSpqk0IJPlEIJUDWXJoDmbBaCKGgviGI4giKJImCITQxDULg3mehQ3GOTI6kyXJyjSb6HlmdZajelo0mOP45laaZZoybI0pCbpQlKZp1l2N6plymphpWTJ4nqfBoD6AQ",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule,electricity,energy,chemistry",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[cfg(feature = "audio_lines")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQwHYMwiD4PAvgOBYThaBoIgoNggDYdgxDGEoUhmGIEhqCRNg4IAziAOIjhWJ4mgWB4pDENAgDgdg3jCJYCieNYKDEOAgDWIIRhOMYXj+NIbguDIOhCPYygEA",
        categories = "multimedia,communication",
        tags = "graphic equaliser,sound,noise,listen,hearing,hertz,frequency,wavelength,vibrate,sine,synthesizer,synthesiser,levels,track,music,playback,radio,broadcast,airwaves,voice,vocals,singer,song",
        contributors = "danielbayley"
    ))]
    AudioLines,
    #[cfg(feature = "audio_waveform")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQzGGDIMDAIIThMMgtDIVg3hAIIShSDQgDSFB2g6HIehWIYUFYNImh+EwxikMIkg+EYuh+IoyC2LI1iiMIXDIIg+DwL4DgUPoB",
        categories = "multimedia,communication",
        tags = "sound,noise,listen,hearing,hertz,frequency,wavelength,vibrate,sine,synthesizer,synthesiser,levels,track,music,playback,radio,broadcast,airwaves,voice,vocals,singer,song",
        contributors = "danielbayley"
    ))]
    AudioWaveform,
    #[cfg(feature = "award")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDULg0DcNwgDEMguDgOYSC4NYMCAOIZDINhhhmGQgDCJISC2FQxg4NxsC0M4ZDgLYUDYOA3GEMYSiaJQwC2KgxDmEQwi2Lw1DgNggjORohg0NY6ieKYoDQNg5GyKoaDQLYdDWHwiD4PAvgOBZeGMaRyGMbBlCAcoJDYIggGMeYJDibhjHiCYTl2X5kmaaA+gE",
        categories = "account,sports,gaming",
        tags = "achievement,badge,rosette,prize,winner",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Award,
    #[cfg(feature = "axe")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDILQ4C4Mw4DEIIShQYYWhYMAgh0MQtDMLgwDCIAzEwMYWDkIg+DwL4DgWLYwgaCAiE0MQ1g4NQuDUYY8jyHpBh2Pw1EENpAkeOYdksIAyDIIA5jyPo7kqQgtjyVw1GgLQxC4Ng3DIYZPk+TIgl0NIMlkOA2GwLY7DANoQm+cYaC4MgwjmXZ3lWS5cC4Nwwg0MBMlEMw1k+Ow2DQOJ1nuDp2niQqSl2gA0Gyc6IiOcRBmOk46g6Ig4DIOB6iyLozD6AQ",
        categories = "tools,gaming",
        tags = "hatchet,weapon,chop,sharp,equipment,fireman,firefighter,brigade,lumberjack,woodcutter,logger,forestry",
        contributors = "Andreto,ericfennis,csandman,jguddas,karsa-mistmere"
    ))]
    Axe,
    #[cfg(feature = "axis_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg1CAMQwg6EIPDkIg+DwL4DgWGIbgaCIKDQIA0HYMQ1GEMYQCAMIri2KQxGiJoXhmHodgSH4JE0NAuDIOQzhAOQuDcMA3CANoQDiM4ajeNoFgeCRtDmFIQg4LQxg6So1gE",
        categories = "design",
        tags = "gizmo,coordinates",
        contributors = "lscheibel,jguddas"
    ))]
    Axis3D,
    #[cfg(feature = "baby")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDYYwuDULgzg4LgyhIIAyCCEhzDGEgtheGohDUIg+DwL4DgWJ4qgaCIKDENYODIaAuDAMYmiiLYsgSLoJgsOYUDgIA2C4OAxDMQQ5CCS4Ng0MYaDCRoOlIMhhhuG5Og4IINDMLg2kyYZaDELQxDeX5chqapjlwLZeDaSpimmUAxhsMxjlkIJejKH5QnuGodiGS4YDULZYhIY4hkMMJliALg0o2hY4ieKY9jyBYHj+S51jWN45pWK4BA",
        categories = "accessibility,people",
        tags = "child,childproof,children",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Baby,
    #[cfg(feature = "backpack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGGDIMDAIITDEIA0C0NBoDiEIXhSH4WhEdoOGEMggiaE4VC2JgyEgNolieH4qDKKx6CIPg8C+A4FjiO4GgiCg4g0MIbjeOY+j2BI/gkTZCDEOJFjiOpKkmBYHkyQgyDIdgti+LIyg2J4rGgNIwiiIIxlsNpGlOPICkqV4KDkIA2FaZZfimYY0DIaAymaYIWiwdgymySIBA",
        categories = "gaming,photography,travel",
        tags = "bag,hiking,travel,camping,school,childhood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Backpack,
    #[cfg(feature = "badge_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDEMgiCAeZlCKZ5pHgMZmmiapxCIOJYlqX5hl6YJimScguDAMZvnWbp0mYNppmuiJ4C+ehlD6AQ",
        categories = "account,social",
        tags = "check,verified,unverified,security,safety,issue",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeAlert,
    #[cfg(feature = "badge_cent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxDIIA3HYMQwliWpfl6BJggkTQxDULoWmiFY8jePQ0mqW5uD6AQ",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    BadgeCent,
    #[cfg(feature = "badge_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDmNgyCCZpmDSIZYlqXw+gE",
        categories = "social",
        tags = "verified,check",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeCheck,
    #[cfg(feature = "badge_dollar_sign")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxDaEBokYYQyCCaYoksNBoDSaJqlKNo9DQSA4liWpfl6BJggkTQxmkMQ4FYNp5lufQ+gE",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,usd,$",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeDollarSign,
    #[cfg(feature = "badge_euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCokDEMhoDWWJal+XoEmCCRNDGDw5C4NIVjyN49DULgymiW5sD6AQ",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,€",
        contributors = "danielbayley"
    ))]
    BadgeEuro,
    #[cfg(feature = "badge_indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCpCDgaA4liWpfl6BJggkTZCDEMplmeW5rmqBYHgkbQxDONg3C0NQtDEaAxhWPJLDALZmlmdJdgEA",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,inr,₹",
        contributors = "danielbayley"
    ))]
    BadgeIndianRupee,
    #[cfg(feature = "badge_info")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMYJDEMgiCAeQymaaJjmwIpnmkeZlnENpYlqX5hl6YJinSCQ4mkeJwmcLgwDGgp1nKapwoGWQvnoZQ+gE",
        categories = "account,accessibility,social",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeInfo,
    #[cfg(feature = "badge_japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDmEAgDOZh2DeWJal+XoEmCCRtDEMo2hwMwtDOa5bm+boFgeCRNmScxoDaeptgKb5/gqgg2oShp8gEA",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,jpy,¥",
        contributors = "danielbayley"
    ))]
    BadgeJapaneseYen,
    #[cfg(feature = "badge_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMYJDgIggHkMoJDEMpoHiawiDENpoHmZZym6WQvl+YQ+gE",
        categories = "social",
        tags = "verified,unverified,delete,remove,erase",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeMinus,
    #[cfg(feature = "badge_percent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDGDw5kaKZYlqX5egSYIJE0OQgDkaAuDAMZplubZsgWB5vmSNg1nSdp4muAQ",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePercent,
    #[cfg(feature = "badge_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHkMoJDENgiCAeAxmYMppHiZQiDGbpjmwIg4liWpfmGXpgmKcJmmiY5xnOb52nidZtnkL57GUPoB",
        categories = "social",
        tags = "verified,unverified,add,create,new",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePlus,
    #[cfg(feature = "badge_pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCpCDEMhoDSWJal+XoEmCCRNDGGw2FYOQuDUYQynUIJ4kiUggkiaJbmya4FgebpjDYaA3oCaoBA",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,british,gbp,£",
        contributors = "danielbayley"
    ))]
    BadgePoundSterling,
    #[cfg(feature = "badge_question_mark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCg5C4MA5CAORhDMIJqkuHA1g2agxGOGQyC2agznaa5YlqX5ZGwaRuGUIB5DKCQxDcIggHihQiDEMpkDGiR5DGhqIoqlKNDKewvn+gQ+gE",
        categories = "accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeQuestionMark,
    #[cfg(feature = "badge_russian_ruble")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCg5jYNhoDWWJal+XoEmCCRNmMMQymYYQyCCdYoieIRoC0Mx2DmaJbmwPoB",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,rub,₽",
        contributors = "danielbayley"
    ))]
    BadgeRussianRuble,
    #[cfg(feature = "badge_swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxhwMQ3FYOBoDSWJal+XoEmCCRNmONgyGgM5rlub5ugWB5xDmNg2mmeJtgEA",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,chf,₣",
        contributors = "danielbayley"
    ))]
    BadgeSwissFranc,
    #[cfg(feature = "badge_turkish_lira")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA3HYMQwGENQghQMAgheFw1C0NQiD4PAvgOBYfiKBoICIbQxhQOAtDYIAzh6IIliSBImgkTQzC4OIrC4NgyGENAgkGGQgg0NAuDeLJHDcN5Ck6RINDaSJDk6VIXkaSA4kKWZVhiXoNheUg3kGZJelcLZLk2S5amWUItkiFpdm6a5okiapPl+GJvDcNoxiGNQ+gEA",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,try,₺",
        contributors = "danielbayley,jamiemlaw"
    ))]
    BadgeTurkishLira,
    #[cfg(feature = "badge_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHkMoJDENQiCAeJlCIOZpHkMYJm6apxCKZ5YlqX5hl6YJimuZponScpvnWc5koCeAvnoZQ+gEA",
        categories = "social",
        tags = "verified,unverified,lost,delete,remove",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeX,
    #[cfg(feature = "badge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FD6AQ",
        categories = "account,social,shapes",
        tags = "check,verified,unverified",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Badge,
    #[cfg(feature = "baggage_claim")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgSA2GGDYNDAIIWDELQyhoVg3hMIIVheF4ahoIg+DwL4DgWJ4qgaCIKDEN4ODQVg0h+IYWDCJAyGgLQxjeIo5hqIB2DEMImiiLYnHIZRjHQIBoGUaRnGgdIJDgIggHiV5ZHmCQ2lkcpbCIMZZHcaRkgWCQxDOSAvkyTonGMaRyGMbBlCAcoJDKWRjl4IgykcIBjmOD5unOdZ3nKdJ2nieqAn2f6Bn2Yw5oejKKgEA",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "balloon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYdgxGGDYNDAIIWhaFBohKFIXh4MQghSEQiD4PAvgOBYliiBoIgqDAgDaE4hh6FoghSJImiuKoEiyCYLDgIA4GOFg0C0MwuDWQQtDaQRzksLZFDYLQ4GGTJMhiDoOhWOInjwPoB",
        categories = "emoji",
        tags = "party,festival,congratulations,celebration,decoration,colorful,floating,fun,birthday,event,entertainment",
        contributors = "peteruithoven"
    ))]
    Balloon,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQ0C4OQyDkIIqiyLgxDkLgwDcIIzjUNwxCKGwviKJA+gE",
        categories = "account",
        tags = "cancel,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,circle,slash,null,void",
        contributors = "colebemis"
    ))]
    Ban,
    #[cfg(feature = "banana")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQzGMMwuDULQyCAOIUg0MAgDIYQ1hIIIeDUIIahoMYWiAIg+DwL4DgWKotgaCIKh4MYiDENwuDgORjiEMgtDGEoVDgLg2hMNo5DkIA3j8MhDDGQA1iKDJPh+FYOhuEAuDKFYaiKIYgieGpHhMNJag2PgxDALg0kkMZNjSJpbhuG4VnUMBjDCP4fkAMQ0nqUQghGNYUC6TxaimK4wD6AQ",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Banana,
    #[cfg(feature = "bandage")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMC4MAxGiEAxCIPg8C+A4FheGoGgiCoMg4NIUhOEYWhiHYcgSHoJgsNIODCJIUieGYriqBYHi0MYvjuMomheNYbgKK45iAOAgDYdgxDKNIpkOOIfE0NpIkqTJAk4chlGMdAgHiCZMCAdxpGSBZfDAIggGgZRpGcaB0gmS5oHmCQ2mgcpeCKVoYlmWw+gE",
        categories = "medical",
        tags = "plaster,band-aid,first aid,medical,health,wound,injury,care,treatment,healing,protection,emergency,aid,safety,patch",
        contributors = "karsa-mistmere,jamiemlaw,jguddas"
    ))]
    Bandage,
    #[cfg(feature = "banknote_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgSA0GGDYNDAIIWDELQyhoVg4hMIIVheDoghoaAxDaH4hhiIIgHYNQiD4PAvgOBYxjSBoICIbYng4OQgDOP4/C0M4wjKN42gSOIJgsOIODIaAuDAMZFjOSZIgWB5LDGPonHYNpUkeApJlmCg2k6UJSmCVg8GMaRyGMbBlCAcoJDIIggGMeIJgydxjHme52jEL5tm+cQ+gE",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,withdraw,expense,out,payout,refund,debit,spending,decrease",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteArrowDown,
    #[cfg(feature = "banknote_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgSA0GGDYNDAIIWDELQyhoVg4hMIIVheDoghoaAxDaH4hhiIIgHYNQiD4PAvgOBYxjSBoIgqD4ODIaAuDAMYwjKN42gSOIJgsOYgDIdgtDaQozkaRYFgeCRtDKDQxDkLQzlyXAgDOUJEgKRpVgoNo8j6QJilIPBjGkchjGwZQgGMeYJgwIggHKCQynoYx4nifoxC+b5xnMPoBA",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,deposit,earnings,income,in,credit,prepaid,growth,increase",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteArrowUp,
    #[cfg(feature = "banknote_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDgSA0GEMgghQMAghcMQtDKGxWDiE4VhiIgxhWGxoDENoghaI4hDIdg1CIPg8C+A4FjKNYGggIhtDEN4Oj4NQgjCMo0gQaI3kaB4JgsOIODIaAuDAMYxjOOJIgWSo7DKFI9C2QZDlWRpXjmSw2k6UJSlSRY2DwYxpHIYxsGUIBjHiCQxDIIp0Hmd55CAcoJnmRJunCcg+gE",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,error,failed,rejected,canceled,declined,lost,delete,remove",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteX,
    #[cfg(feature = "banknote")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHiDQyhEeYNDaERoGUaRnGgdINDGFQgHKE4OCIPg8C+A4FikYxpHIYxsGWJIUhEY4miKN4XCKOopC+L4xjOKRwGGCwgGSDRNDYIIiGgLgwDETQxDiTQyk+UYoiqRYLD6AQ",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "barcode")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANR2DENAiD4PAvgOBYThaBoIgoOINg+EYThWBBohiI4HgkTQxDKHoQhKFIZiWBYngoMQ3iyIIviOMYaigMgxjeLoiheAQ",
        categories = "shopping",
        tags = "scan,checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer",
        contributors = "danielbayley"
    ))]
    Barcode,
    #[cfg(feature = "barrel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzGENAxCCEQgg2FoVCAMQ4CIPg8C+A4Fh2IIGgiCgxDSDoQhKFIXhKDYahyHojiKBIkgmCw3ikMggjuLYZC4Ng4C4OY7DENQuDINYZkeSYYi+GAxDYLpREGO49lCGY5DIMRIDcYZWk6GQtDGQA4C2Q5FkySpGkiSo+DCY5SlSYI+jkMx6jGH41jSBYHjcMwuDiKAxDcaJRC4MwynmM4CjWfoKoCgggoWh6Joue4BA",
        categories = "food-beverage,navigation",
        tags = "keg,drum,tank,wine,beer,oak,wood,firkin,hogshead,kilderkin,barrique,solera,aging,whiskey,brewery,distillery,winery,vineyard",
        contributors = "karsa-mistmere,jamiemlaw"
    ))]
    Barrel,
    #[cfg(feature = "baseline")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2CIPg8C+A4FhSF4GggIhtDYIIRCANgtDEMoiiAMoThWGoZgSG4JE0OIoGgOIqhaLg+gEA",
        categories = "text",
        tags = "text,format,color",
        contributors = "ericfennis"
    ))]
    Baseline,
    #[cfg(feature = "bath")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA0CAOAgDYIg+DwL4DgWFYYgaCIKDENwgDEOR2DKFIWhuGoEhyCRNDKIQyGgMgwiaF4qimBYHiyIIiiSNIogKKo5goOQgDUIA3C4NgyDEIAzkmSxBDILgxksIJSlSTINlqDpFHaVBhi6Lpbg2YRol+YQgmOVgtDIdgtDWPo2gE",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    Bath,
    #[cfg(feature = "battery_charging")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIA3C0MwgDUaA0GyEISCIPg8C+A4FhqHYGggIhNDENAuDgNQ2CANhIDENhhDIIIxDAII0g2MQyHYOIwjKNY+DELY4GiQQuDkMw1hmG4gh+BIhgkTQyjGJR2C0NJJhyTZMgWB5PDULolCAMQ4EgNI8jOP5BkEVo7jiPo2jKQRoDKRQzDaV5LgEA",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey,jguddas"
    ))]
    BatteryCharging,
    #[cfg(feature = "battery_full")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMHYNAiD4PAvgOBYThaBoIgoMQ0g4MIQhKFIZhiBIagkTQyDKDg0HYLYRhOFYmiWBYHigNofiGMYkDwchlGMdAgHmCQ2CIIBoGUaRnGgdIJDEMpGHcaRkgWTpFCAeIJlAIBylkIpQjGPpAD6AQA",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey,jguddas"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDQdgtDQIg+DwL4DgWFYYgaCIKDaDoQhKFIWhuFRyGUYx0CAaBlGkZxoHSCQxDIIggHcaRkgWMg2jUch4gmNAgj8IpBHmCY8hUL4nikPoBA",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere,jguddas"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDQdgtDQIg+DwL4DgWFYYgaCIKDIMoOhCEoUhaG4agSHIJE0NohhGE4VheKIVHIZRjHQIBoGUaRnGgdIJDEMgiCAeIJkEIB3GkZIFj8NpCHmCZNCAcpECKQYwjSNg+gE",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere,jguddas"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5HYNgiD4PAvgOBYThaBoIgoMQyC4NQ0DMIA2EgMQ2GEMggimDYNDGKoqHYOIoi+LAgDELYpDIaAtDMLg2DANYShSGYYgSGoJE0MopDENB2C0NJChWRpFgWB5IDeNo6hGE5SheApGlaCg3j4MA2jYOBIDSM4rCCLY4jgVoyjmbZ0i4Mo4GiPY/luQ5TgE",
        categories = "devices",
        tags = "power,electricity,energy,accumulator,charge,plus,economy,health,add,new,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis,jguddas,johnletey,Footagesus"
    ))]
    BatteryPlus,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDcaAuDAMQiD4PAvgOBYWhmBoIgqDAgDcdg2hWF4chuBIdgmCw0CANhoDIYQyCCM4Ng0MY0jQdg4jKOY2g4LYzDIaJBiWGIpiiBYHisMozDENB2C0NJGieAopkuCg2g4OBIDSPY1CCN5BkEVo8kKYZojgMpBjCVJIgE",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis,jguddas"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQgDIMggDENAgEyDIODEMAiD4PAvgOBYZHIZRjHQIB4gkMgiCAeYJDaJxoGUaRnGgdIJDGJggHKJAijUdxpGSBYziuGQvh+IQ+gEA",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "colebemis,ericfennis,johnletey,jguddas"
    ))]
    Battery,
    #[cfg(feature = "beaker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4NQgDMaAxDUIg+DwL4DgWFYYgaCIKDaDx2DENhhDIIIlDAIIoiiJQyGgOIkiaKYyisLQyFYM4UhaG4agSHIJE2HwxDSEQyjmF49D6AQ",
        categories = "science,gaming",
        tags = "cup,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis"
    ))]
    Beaker,
    #[cfg(feature = "bean_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAORjC0Lg2DSEQ0C0MQuDUMoXDkNYVDILg0DAMgghcMQ2DUQQ2CCKgwCCLYtDgIAyDIYQxDMLg5iqNo4iyLo+DmOAthMMQiD4PAvgOBZGkmBoIgoMQwC4Nw1CANQuDAOQzimK4+i0MYyiMOBji2Hw0DEMYQDaX4TDYOIWhEOAziuEQ2kWR5MkuBJNgkTZWDOZoklENo0DQIKFi+Pg2iAOA3iQLgyDAOBNlCEYjn6ZhhhMMAxlSmqcl2JIylcM4Mh8MA0naSJ6kYbBpG4ZQgHgMYJDIIggHmswirWtwyrSux4r2uq1kYL6tq8PoBA",
        categories = "food-beverage",
        tags = "soy free,legume,soy,food,seed,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BeanOff,
    #[cfg(feature = "bean")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxDYNQgDYLg1DkOBDDkLg5DUNAgDcLg0DcOAghkNodDgLgzDaJIkGMLQuiaMA0C0MYUDKNYbjMMogDAMggjWDw1EGK4rDAIJGkaIwyDIY4fDcM4+kYMYzhMMg2iOU40DQYZEkeXgwjSNQ4DMNY0jwMhaCIPg8C+A4FmuboGgiCg1ikNAxj+DQ2DIYYdh2UpenUMg3DkLaCDiapsnEPoBA",
        categories = "food-beverage",
        tags = "legume,soy,food,seed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bean,
    #[cfg(feature = "bed_double")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwHYLQ4GGDIMDAIIWDGDQtDIaAxDaE4NheIoZhQdg4CIPg8C+A4FimLIGgiCg0CAMQwFaH4UiKGIahwMQyiCFYjiEMh2DSKIqi+LoEjCCRNj4IA0HYNpHiuS5KgWB5NgwMQ4GiDpUkmAQ",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedDouble,
    #[cfg(feature = "bed_single")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgwHYLQ4GEMoNCAMIWCAMYNC0MhoDENIThWF4XhqFAyHYOAiD4PAvgOBYri6BoIgoNYZDAVg2iGFIjhmG4dDEMI6hiJIVicNIqiyMYwgSMoJguGQ4h6KYri2TA+gE",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedSingle,
    #[cfg(feature = "bed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANB2DENgiD4PAvgOBYThaBoIgqDA4GgMQ4GGDIMDAIIlDEIIjg8MIShSGYYgSGoJgsIAxDcaAyiyE4VjGMIFgeMw2CAOB2DmLY8heAQ",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bed,
    #[cfg(feature = "beef_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg3DcMQgDYLgxDAORhDILg1CCGYbDAIIfhEM4UDIIIjDEMgiD4PAvgOBYri6BoIgoMQ3C4OA1iWKIUjgYYThuP4gkIMAtDkLgwDMNZFkcNIqiyMYwgSMoJgsOJHDEMwgDGVgwlgQwxDWRwyDmHAwC4Mw0mSFY3DOEQyiUN4cDIYZZlmH4hC2GQ2DgLYNDYNhMhkNJaj+TotlKUYFgeCRtluGoShyFJkDSGo+C4NA4hKl6ZneWgthQM6TmcOQzoaUIClKiwiG2Jath8MgwqaiKooqMxNhOSZAmcNxhnGcadkSRw3mEMgzGOfYUiaoKfDemYjDkLYnpmEwwDgQZ1kOQoblsY4jkkNpChMOaksgMg2nGRo4tGFA1DGsovgEA",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak,vegetarian",
        contributors = "kemie,ericfennis,karsa-mistmere,jguddas"
    ))]
    BeefOff,
    #[cfg(feature = "beef")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLg0CAMQzC4NxBg0NQghYIAwhCGoYC4Mg4h4NhjC0MQuDEIIShELYTiGEg5C2Kohg0MA4EEM4ph2G4bhcMQ4GOD4bDiDolC6IQxiYNAtDQLgzCIPg8C+A4FlCU4GggIhtj4LoXDYIAyicOQgkwNRhg0NIzg6IY7hALZgDKYg3h8QwxDWDpfDCH4QigMgyCAN5fDIYY4jibAxm4Lg2DiRQ2DYTJgg+DJck+UZWlAYxpHIYxsGUIBjHmCZDDUIggHKCZgqOnh4gkMaopQL6YpqnA+gEA",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak",
        contributors = "kemie,ericfennis,karsa-mistmere"
    ))]
    Beef,
    #[cfg(feature = "beer_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIIMHYNQiD4PAvgOBYThaBoIgoMQ3g4MQuDQNxWDiEoUhmGIEhqCYLh4MQxGgMRhg2DQwCCNgxCAMguDcNA1CANAuDKL4mhWKopgWB4JG0Mo6jqNgyDCRYogKKpKgqPw4HYMQyGGTZNjaYZODIaA4l6TpilALQyHYLQzlOR5VkmGxNDcLg1DMNggnaeA1EMNo8Daep2DYNA5CCgAxDUNAgDgIA1nejZepGO4/mIMQtiANw2DgLZBDKnJwhecorgoOI8DKHgzkIMA0EMOQuDMMJ6jum4erAOA4j+TYvjoY4gDWepQg6kYNsAc6aDILaUneyw1GGOY5jiDo3o8Y7LDejgwpmzbNpWzozC4MaGCCq7joeabLDgNI7lyohokipRNoe4wuDYVgxiWE5GheAQ",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    BeerOff,
    #[cfg(feature = "beer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDEaAxGEMwghQMAghcMYYCANhoC0MQiD4PAvgOBYiiWBoIgoOYODIdg2iGI4oieBIpgmC4UDGLowiKJI1jSBYHjcMQ0CANwuDUY4fhiHwuDQNJIC2FJIHMLQyC2UQzlgNZNDcMpRl8NZIGGYQgmWF4ZkySQuDcOIbDGSJHDWZpIkgUw5k6RQyg6GgyHOe5wnOOZIg6bJXoSYZbmSiKEmiDobkmWJtkygZblaUZIFqMY+iaAo1kKCpzDgdo6mSZobo6ewyGgOKmnujoXlcMhWDim4zgE",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Beer,
    #[cfg(feature = "bell_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCgxDELoQhILgwDAORBDYII3hmOo4CAOBjhkNIdDkOQti4NItCANQuDkNQ2C0MguDcM4RDcLgzg8QQxCCWo7kCWw3GgMQ2GGWpchqGZRDSRYvlIYwtC4OAymqcA1msNg3Dia5SDGT4OnyHAzDQOIiiSKYjGMaRyGMbBlCAYx4gkMaDo4eYJDUIggHKCQzoQL6IoqjA+gE",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley,jguddas"
    ))]
    BellDot,
    #[cfg(feature = "bell_electric")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg1gwIAxDcLgzDQNxBDcIIZDAIIcDGEQ0hEOQiD4PAvgOBYliiBoIgqDAuDgIA0EEMYfjWHY4h8MociOJYngQaIqkCB4JE0OQgDkaAuDAMYkiaK4lGMaRyGMbBlCAcoJDIIggGMeJaDCXBjHmCQxDaTgvlKVJWlGU5VleXoJiOXZkCKc5ZCIN5omqb4lHIZRjHQIBoGUaRnGgdIJmcIJfCINJcHKjZbCAdxpGSBZlmEIJ1maaJ/oEPoBA",
        categories = "devices,notifications,home",
        tags = "fire alarm,flames,smoke,firefighter,fireman,department,brigade,station,emergency,alert,safety,school bell,period break,recess,doorbell,entrance,entry,ring,reception",
        contributors = "danielbayley,jguddas"
    ))]
    BellElectric,
    #[cfg(feature = "bell_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCgxDUIA4GgNoiiSKYogSKoJgsNoODQMwghwNw1DcQQ2CCRYZkiRovGOGQ0h0OQ5C0MYdDEMQgDULg5DUNgtDILg3DOEQ3C4M4PEGVpWkmTQgDENxoDENhhmiGpJl8NJSC4NpgEEOYdCCfYgkmaA4nmepsDKM4ljcPoBA",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder,delete,remove,erase",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellMinus,
    #[cfg(feature = "bell_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCgxDcIItEgNBhDGL4ahkMQtC4Nw0C0MQuDYNwzEMNAuDUOYvhwOQ1DYIJMDEModDmR5MDgYZMkyGY3CCDg1DiPY6DQMoiiSKYogSKoJG2FprhkMgwmOJZnmaBYHgkTQ4j+EAghwMAxEGV42jUMYRDgY5tj8OA3jqLpEDYNQzi+Ogwi4NguDANZwmWAQ",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    BellOff,
    #[cfg(feature = "bell_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCgxDUIA4GgNoiiSKYogSKoJguEQ1HaMojiWN42gWB45DKDQwDCFgxDSHYfGEOQglCGZTCALg3DMOAuDgNgzEEMQgl+VJfkaYA3EgNBhl+YYaDELZWDQLQxC4NpXEOTA1lAMYcDkNQ2CCfgxDKHQ5lCfg4GGfp+mKL5WDULQ1C4MwzDKM5AieAQ",
        categories = "notifications",
        tags = "notification,silent,reminder,add,create,new",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellPlus,
    #[cfg(feature = "bell_ring")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCgyhYOBjDALQyC4MwtC4OAtDSNIyC0NoiiSKYogSKoJE2HIPhYMQ1jSDxBDEIJPhmUggiAMQ3GgMQ2GGT5RhqGQuDcNAtDELg2DcMxDDEOYdlyHA5DUNpQhEMYzDQOQ5nIIA4EGcZxlOGZxi+GY6nYOZjmyT5Lm8NoymAM4RDeTI+iOJZDkKBYHkWIAyEOM4RhwN4XkuoYuj+lYngEA",
        categories = "notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "ericfennis,danielbayley"
    ))]
    BellRing,
    #[cfg(feature = "bell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDYOAgDIMRhDKEggDCGIaDMLg0DYNIYCIPg8C+A4FiOJoGgiCocg+FgxDULgzg8QQxCCNoZjkIIgDENxoDENhhjaOIahkLg3DQLQxC4Ng3DMQwxDmHZDhwOQ1DaN4RDEModDkOZZCAOBBliWI6hmWA4GOGQ0l0OZKlONoxlYNgtlyToRDeMoPiKJIpD6AQ",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    Bell,
    #[cfg(feature = "between_horizontal_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDcIggHIeIODGER3GkZB0GiFQzhGFAih4IB5g6Hg+DwL4DgWJxwGGGwgGSDhtDIMggDENQtDOOQgjqJooi2G4niqBoYhqHAiDGIogiKE4VhGCIKgyDoQiOFQ0CKJ4pgQdA+gEA",
        categories = "layout,design,tools",
        tags = "insert,add,left,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalEnd,
    #[cfg(feature = "between_horizontal_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDMIggHmDYQCAaBlGkZxoHSDQ3hEeINDiERyiCDgiD4PAvgOBYoHAYYLCAZING0MggDkIAzjgLY5hCKAvi6C4oiuBoliKB4JguDYPiOJQxhGE4ODSEYXhmG4dieKZDD6AQ",
        categories = "layout,design,tools",
        tags = "insert,add,right,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalStart,
    #[cfg(feature = "between_vertical_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIB5goM4NGgZRpGcaB0goMYRCCCQihsdxpGQdBogoNwiD4PAvgOBYoHAYYjCAZIKG0OQgDIMggDMLQzjmOYnimLojiiK4GhOFYXhmH4hiOJYNh0MQ0g2D4eg2CIZj+KoEHQPoBA",
        categories = "layout,design,tools",
        tags = "insert,add,top,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalEnd,
    #[cfg(feature = "between_vertical_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIB5goOINGgZRpGcaB0goMQzg0dxpGQdBogoN4NgkIobD4PAvgOBYoHAYYgCAZIKG0MQ1CAMgtDMIAzjmOQiigL4uiCKIrgaDwihEIIlDENINgiGYch6IIihKFIWhiC4nimRQ+gEA",
        categories = "layout,design,tools",
        tags = "insert,add,bottom,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalStart,
    #[cfg(feature = "biceps_flexed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg0DAOQgDEMwuDAMQ3EENQghoMAgh0MQgDIMoSDUY4dhQOA2DYLQ0CANwthGLw0hUNw3h4LQ4C4MQ1DMLQuDgMgtDEMAuDMNwxC2DQ0DaQYODKK5FDGUA2DOSI/DOTZMkKRZMEODQxDEOISg0NwyDOIQukyNojkOIRhmeZ4dh8IJwiGdpyhKSYhGOW5DhyWw2DSPg0oSSZCCIPg8C+A4FomjIGgiCo7hINBhhqHIejcNwuDUOItDKiKKo+joEpCCRNDkLg5oEIA2j8Mg1EOOYWjGqo1CCqYahMIJijuoaLqUPoBA",
        categories = "emoji",
        tags = "arm,muscle,strong,working out,athletic,toned,muscular,forelimb,curled",
        contributors = "karsa-mistmere"
    ))]
    BicepsFlexed,
    #[cfg(feature = "bike")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4C4NQiCAcoMDOEISGMeYMDEN4XD4PAvgOBYHh+IoGgiFAihaEYJgsIg1heCYag2HYRh+IYEieJY5geE4bhiM4sgqG42iCJokDwcBhHQaAgGSDBNDEMgghyEBWDENBsC0M5bCANJdlMMxoDIIo3kqTA+gEA",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHmDgxDSER3GkZB0GiDoWCAeIUh4cogCIMgiD4PAvgOBYoiuBoYhqHAih6JIQgeCYLg2D4RhOM4RiODomiiKoEHSKBwGGGwgGSDhNDYIAyDAaIWkOSIbkeSRokuTYVCAMZSlSKZWGiWJKkwIpOl4NBoDIdoQlWWZllqZxNl2a5tm+YpxgE",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "binoculars")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMGgNAiD4PAvgOBYThaBoIgoMQ5CANxWDQYQxg4IINicLQxikaAtDKI4lieJopg4dgzhKFIZhiBIagkTQyg0MgxGEMggkSMZAi0dgtDMLg4DUMRjDCKQuDMOYti0Lg5DYMpXDSTQyDkVg4i+JJHimKwtiKJJliaMprHYMZCkSRptkCRR6jeFY7jqBYHj2RZEDENggEyRYODaeY5gKO5+goNJFnKhoxiqXAykqTJOlCUgxlSHqVlmW5FmmX5hmOa51iWKgxhCZKoqcMZwpGdINpSd6Jnui59hsTYeiCaownWZwxEgNqtmab42hOeoXgEA",
        categories = "navigation,nature,photography,science,travel,development",
        tags = "field glasses,lorgnette,pince-nez,observation,sightseeing,nature,wildlife,birdwatching,scouting,surveillance,search,discovery,monitoring,lookout,viewpoint,travel,tourism,research",
        contributors = "karsa-mistmere"
    ))]
    Binoculars,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQxC4OYQHKDIPD4PAvgOBYHhwcBhHQaAgGSDBNDYLg3CAMwuDQYwthYIAyC4NQgDAIA1C4Mo1j0IIrDcQ4rjgOYuiwIA5C4No1CCFJMCKHAviOJYiiSJooCIbQ4jQMQwC4MZPjALg4lKHZVGiV4lieKQxDcLgzkiMY0jaOI6jwMgtjaPpCj8NAthWPp5jOTYrkeFQ1meVJYmuWYMG0MY8mKX5hoGZJmlOaaOm0IhNDGfgymAOIyjYNozoANJMnuq6qDcLY8DOM4+qarJiqoOAtq6O49oumw8mmnafj4MYvDkdoVDavqNsCWLCsWN68jGgaznu1ZyDCy5Ws2bJap+LagjAY4sjO4I9peTaJqyiqasywbejyOLQDm47GmWYw4mOOJ2tqaoBA",
        categories = "science",
        tags = "fallout,waste,biology,chemistry,chemical,element",
        contributors = "karsa-mistmere,danielbayley,ericfennis"
    ))]
    Biohazard,
    #[cfg(feature = "bird")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA3GgLgwDEIg+DwL4DgWFYYgaCIKDMLg0CAMQ4EgMQyGEOAgikMAgiyLA4C0OBWDcYYhiGLotC0NwuDKMAyC4MxMDIIAyDCFIWhuGoEhyCRtkWDpECALg1C2Q5TkeF5LkqBYHgmC4siMdgzliSYCkuXYKDGIQxjsNw1FYMoThWWYZmaXIdE0N4iDgYYNg2OIsh8OA0C0MQwC4NpykiWoBA",
        categories = "animals",
        tags = "peace,freedom,wing,avian,tweet",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Bird,
    #[cfg(feature = "birdhouse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgdg0CIPg8C+A4FhSF4GggIhtDEN4ODiDguDkNQ2C0MQxC4NA2DiE4VhqGYEhuCRtDMIIiDcLg4DILQ1C4NgxDUYYNg0MAgkcMQgDILgzDaSBMDKSouhSFozjKBYHgkTQ0iEaAxDaL5WhiAozlqCogg8II/DANJdDYLg1DMMpijEPBjGkchjGwZQgHKCZ0CAYx4gmDAioIeaFDCYp4nqfA+gEA",
        categories = "nature,animals,navigation,home",
        tags = "birdhouse,bird,garden,home,house,woodwork",
        contributors = "hieu-onefold,karsa-mistmere"
    ))]
    Birdhouse,
    #[cfg(feature = "bitcoin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg3DYNwgDEOQuDAOA5GMNAuDkMoaDgNg4CANguDENAtiMMAyDWEguDIMQ2icLoXDQbQtg2Lg2iKMg5DQTA1jKOQxDiFQ0DcbY/DmFosDANAyC0LgzkWLA5kaDQ1DYMwtkOHw0hmG4dkAOY6iWMYpiuNwxDWMYXDONZoiuI5sC0M4biYLg2jyR4kDWaojDITJDDKYoaDINp6kmIYNkwMpQDQOI2hsNxNDcLqOCAMgwlANg0GydAxoWNqUDcMg3CIPg8C+A4FD6AQA",
        categories = "development,finance",
        tags = "cryptocurrency,digital,blockchain,finance,coin,market,decentralized,investment,crypto,currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Bitcoin,
    #[cfg(feature = "blend")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIOQiCAcoMDeDxjHiDIOD4PAvgOBYHhmHIGgiEQihOCYLCIMQ1hSFooiqGYbgSIQ+gE",
        categories = "design,photography,tools,development",
        tags = "mode,overlay,multiply,screen,opacity,transparency,alpha,filters,lenses,mixed,shades,tints,hues,saturation,brightness,overlap,colors,colours",
        contributors = "danielbayley"
    ))]
    Blend,
    #[cfg(feature = "blinds")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoDEOAiD4PAvgOBYThaBoIgoMgwCANxIhGE4VgQaIYiWB4JE2HQgDEMYhhKFIZieBYpgoMYeDEOYPDCMYkheAoohsTQ4i0NYPDKPozkGNZDDSDR2DENJKiWExjGkchjGwZQgGMeIJlMIBygmSZdHmCY6j6V5ZlsPoBA",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[cfg(feature = "blocks")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDIVg3GEMQghSDYXC0MYZEgNBhDKDggheIQth8Mh2DEMoeiCIoNiUaIoiqH4sg6JB2C0NYThWIY7DCGYbDIIg+DwL4DgWQhyGUYx0CAch4gkMQiCAeYJkAIBoGUaRnGgdIJDiUZOCIMQ0lEdxpGSBZdkGQ5IkoPoBA",
        categories = "development,layout,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning,squares,corner",
        contributors = "danielbayley,jguddas"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPC0NQgDUVgyGyFA1EyDg3CIPg8C+A4FiAbBpG4ZQgHkMYJDEMgiioMoti8IB4iwIgxDiMB4jIIgyDGH4hiaKIlieKY2gkM47j0NowiuM5Oj2LpBC+QxlD6AQA",
        categories = "connectivity,devices",
        tags = "paired",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothConnected,
    #[cfg(feature = "bluetooth_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIIMC0NQgDUVgxDIbIQhIIg+DwL4DgWG4egaCIKDIIIlDIMImDCGociGIIEiKCRNDENAuhEOY2g6DQ3hcNYQHaNQ1iyHYwD6AQ",
        categories = "connectivity,devices",
        tags = "lost",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothOff,
    #[cfg(feature = "bluetooth_searching")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPC0NQgDUVgyGyFA1EyDg3CIPg8C+A4FiCI4GggIhNDIMAuDgM4RDSLQzGENAgjWEI4CAMITC4Ng2h+IYmiWBIngkTQxDiEQyGgLgwDGQIikQPoB",
        categories = "connectivity,devices",
        tags = "pairing",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothSearching,
    #[cfg(feature = "bluetooth")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPC0NQgDUVgyGyFA1EyDg3CIPg8C+A4FD6AQ",
        categories = "connectivity,devices",
        tags = "wireless",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    Bluetooth,
    #[cfg(feature = "bold")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQyGgORhDQIITDAIIWDGFwgDgSA3GGGYZhaGAtDGJBWDWH4NhqGINiQaIehOFYaiGGwiD4PAvgOBQ+gEA",
        categories = "text",
        tags = "text,strong,format",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Bold,
    #[cfg(feature = "bolt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FlAYxpHIYxsGUIBygkNAiCAYx4gkMQymAYx5mSZpQC+VpYloPoBA",
        categories = "tools,home",
        tags = "nut,screw,settings,preferences,configuration,controls,edit,diy,fixed,build,construction,parts",
        contributors = "danielbayley"
    ))]
    Bolt,
    #[cfg(feature = "bomb")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDkIggGMeYLDEM4OGMeISDEIg+DwL4DgWB4bHAYR0GgIBkgsTQxDQLgzDUIIrDaLgxDaLAgDILg3GGNw0DGNgujwIAwkEII9DOP5BGwMQuDaRJLjqR47kOQo9kKRg0GwLZKDmMguluGociKJIhiOJYnCIbQyDKNpZC6XA1l8L5hGgPoBA",
        categories = "security,tools",
        tags = "fatal,error,crash,blockbuster,mine,explosion,explode,explosive",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bomb,
    #[cfg(feature = "bone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDAYwuDcLYSg4Lg2DkIAwCAMguDWGhhh2H4ihqDolDALYfh6Hong6FA1i+HIsiSGwxhqKYaGOGwuDgMYVj4OIliIbAtg0NxjhQN4SC2PoYjeNIhjOLIblSJQ1hEMpBjsNQuDKIoelGI5TiaG4fhCKI8DGSZMjyT4eFoIg+DwL4DgUPoB",
        categories = "animals,medical,gaming",
        tags = "health,skeleton,skull,death,pets,dog",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bone,
    #[cfg(feature = "book_a")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoICIbQ4g0MwgDQLQ3lEIA3kKRJJkiBJKgkTYPisMRoDULpXkORZcD6AQ",
        categories = "text,gaming",
        tags = "dictionary,define,definition,thesaurus,encyclopedia,encyclopaedia,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,alphabetical,a-z,ordered",
        contributors = "danielbayley"
    ))]
    BookA,
    #[cfg(feature = "book_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMaAuDAMQiD4PAvgOBYWhmBoIgqDAgDYdgzhWF4chuBIdgkTQ0g4OQuDUdgtDENRBDKMAgjcNQgDCPIOiGOAyEgMQ5GEMY/j2PZHksdgxDiRpIj4MYzg4SA2jCUJHkmSAtDUSAyDCJYYikPoBA",
        categories = "text,development,gaming",
        tags = "reading,paperback,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,warning,alert,danger,exclamation mark",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,lscheibel,domasmark"
    ))]
    BookAlert,
    #[cfg(feature = "book_audio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNwiD4PAvgOBYThaBoIgoMQ2CAOB2DOEoUhmGIEhqCRNDQIAxDkLg1HYLQxDUQQyi8II2DUIAwjuLIOjcMhIi0YQxj6PI8kWSR2DEOJEkaPQxjKLBIDaL5OkWR5GC0NRIDIMIjhWJ4mgWB4pDiH4hmCJYB",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookAudio,
    #[cfg(feature = "book_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoICIbQ5CCD4bDKGQgDQLQ0kKRJJD6AQ",
        categories = "text,development,gaming",
        tags = "read,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "danielbayley"
    ))]
    BookCheck,
    #[cfg(feature = "book_copy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANxhDIIIQDAIITDALYQDIdgxDEIg+DwL4DgWHohgaCIKDULg4DAMwgDEOBIDWD4RhSNIVCANBoDkLoxjuO41jSOwtjsVgyhyHoggQaIjkmB4JE0OYtDUVg0jKEo0DGEYXjmOxhj2DI2liXoalSXo/DGQooDUSIblWZoUC2OAxDCHYfiQPoB",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,clone,fork,duplicate,multiple,books,library,copies,copied,plagiarism,plagiarised,plagiarized,reading list,information,informed,knowledge,knowledgeable,knowledgable,education,high school,university,college,academy,student,study,learning,research,smart,intelligent,intellectual",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookCopy,
    #[cfg(feature = "book_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcaAxC4NQiD4PAvgOBYWhmBoIgqDAgDIMoRhOFYXhyG4Eh2CYLg2I4ShSFoYiqKYFgeLIPhOIQyEgMQ5GEMYOCAMJDkUMQtDGJozhqAoqjeHw3jqPI+kCQpEkSQZZHaMJKiiTY2h4TQylgNB2DMaAtDKJYyl6HJPmKRA4hMVgxDCXY0l+K4KDSDgwFacoxieeJumGfI+nMMQ0neTKEiyfA0hMQZqDWIY6leQg2lISA4osaI1noTQ4jsSKZDWVZBpeqAtDWm6dD6AQ",
        categories = "development",
        tags = "code,coding,version control,git,repository,template,draft,script,screenplay,writing,writer,author,unwritten,unpublished,untold",
        contributors = "danielbayley,jguddas"
    ))]
    BookDashed,
    #[cfg(feature = "book_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GgiCg0g4OQuDUdgtDENRBDKIQgigNQgDCLYOCANopDISAxDkYQxjCLoujmPR2DEOI4jqLwxiSDhIjINZCjmO46C0NRIDIMIThWGoZgSG4JG0OYOi4Mwgl8MwtDOVIWlgPoBA",
        categories = "development",
        tags = "code,coding,version control,git,repository,pull",
        contributors = "danielbayley"
    ))]
    BookDown,
    #[cfg(feature = "book_headphones")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoIgoOINDKEgyGGDIMjuJ5NDAdgykKRJJkMYxpHIYxsGUIBygkMQiCAYx4mcNZpGMeZnlqQwvl+YZjl6YJimSa4JDmb5xCIMZamWZ5bnWep4gE",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookHeadphones,
    #[cfg(feature = "book_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoIgoOAuDYMggg8OIWC4MobheVoyh8MZQiEOAzDYYZYleVYbiqHwzC4Mw4hkLg5DYNhsC2F5PDabQ4DWYZuDmTQ5nyHoqhOEAwDeHB6kKRJJD6AQ",
        categories = "social,text,gaming",
        tags = "diary,romance,novel,journal,entry,entries,personal,private,secret,crush,like,love,emotion,feminine,girls,teens,teenager,therapy,theraputic,therapist,planner,organizer,organiser,notes,notepad,stationery,sketchbook,writing,written,reading,favorite,favourite,high school",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BookHeart,
    #[cfg(feature = "book_image")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDAIAxDMLg3C0MguDGE4VGEMgghqDYdhcOAgDATA5hGDg3CIPg8C+A4FimLIGggIhNDSDokDUdgtDENRBhQNYbC6Podg4IA2kCGxIDEORhDGQ5CkyTx2DEOJLk2IYOjmDhIkUNZUkyTohC0NRIgyKIqi+KRjGkchjGwZQgGMeYJDgIpvHiCQxDCdBygkMplC+aZrm0PoB",
        categories = "photography,text,multimedia,files,social,shopping,travel",
        tags = "images,pictures,photos,album,collection,event,magazine,catalog,catalogue,brochure,browse,gallery",
        contributors = "danielbayley,jguddas"
    ))]
    BookImage,
    #[cfg(feature = "book_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyEgNguDUQQyhKDoWDAIIZhkNAgDSEh2DENQiD4PAvgOBYliiBoIgoMQ3g4dg2iSJoriqBIsgmC4wDQaAyjSJ44jeBYHjoMoZiILgyFYMgxGEMQglCG5RC2UAxhCEpPlGGpclILQ1EiR5AjYPBjGkchjGwZQgGMeIJi8IggHKCY/mweZvDCQJmmiag+gE",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,public,secret,unlocked,hidden,revealed,knowledge,learning",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2FYNBhDIIITDAIAxCAMAtDSGR2DIIg+DwL4DgWIYkgaCIKDKFgxDUdg2GGGIYhaLAtjISA2C4NYxheGY+jMLQ1EiK4giKJ4mgSKIJE2HAxDmOh2jYNRBDKOoUlaNI9jkNYUEgMQwkWI5JiEchlGMdAgHIeIJDEIggHcaRkgWCQ4m6awiDGHwgGgZRpGcaB0gkNZuHmCQ2mGZZnD6AQ",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,secret,hidden,knowledge",
        contributors = "danielbayley"
    ))]
    BookLock,
    #[cfg(feature = "book_marked")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYOBsDMLQzCCFQzFYMgiD4PAvgOBYch+BoIgoNAgDEOQuDUdgtDENRBDKKoOjKDYNDEIA2jIMhIigYY3jeNYnkIMR2DEOI+kKQQxi2JxIjkNZIkAII2lMLQ1EgMgwhuHYiD6AQ",
        categories = "text,development,gaming",
        tags = "dictionary,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,saved,later,future,reference,index,code,coding,version control,git,repository",
        contributors = "danielbayley"
    ))]
    BookMarked,
    #[cfg(feature = "book_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoIgoOYNDAaA2kKRJJD6AQ",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,censor,cancel,forbid,prohibit,ban,uneducated,re-educate,unlearn,downgrade",
        contributors = "danielbayley"
    ))]
    BookMinus,
    #[cfg(feature = "book_open_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDEVg3CIPg8C+A4FhSF4GggIhtDENgggyDojDQLQ0hOFYahmBIbgkTQyg0NhWDQYQxiEIAwjiOAtDGPBoC0NRhDQIJDjmRomkSSZFjoMImiYSAzjWN5GjuNgxHYMZRlaTI6lYaA2GEMwgmKVI2mKZ5jlyZgtDOX5SjaVI5j2V48C4M4ohaLA+gE",
        categories = "text,development,gaming",
        tags = "read,pages,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "schmidt-oliver,karsa-mistmere,ericfennis"
    ))]
    BookOpenCheck,
    #[cfg(feature = "book_open_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYMQ0CIPg8C+A4FhSF4GgiCgxDYIIMGgMoThWGoZgSG4JguHw4iKJIWiiJ4FgeKgziAOBhDGIAgDCPIgC0MZAFYNI5juPY9jqQQxGgNRhDQIJPkeO5PlSUI+kiUAtDSTJFjqUpJiCEAzl2V4/kkaAtDYYY2jaUgwC2bAgnGbpwnAeoviaAoojSCofiGI4UjCGJ6jOHBNiyLqBiaAQ",
        categories = "text,development",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed",
        contributors = "danielbayley"
    ))]
    BookOpenText,
    #[cfg(feature = "book_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYMQ0CIPg8C+A4FhSF4GgiCgzCAMQ4GEMYfCAMIlh8LQxikVg0iKJImiaI4qDEaA1GENAgjiMIkjiPY5ieMY5C0NI1i6I47jKH4QDORpAiiMhoC0Nhhh6Ho7DALZVCCWpXlmWR6hOFYaD6AQA",
        categories = "text,development,gaming",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,screenplay,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed,blank,plain",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    BookOpen,
    #[cfg(feature = "book_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYNgiD4PAvgOBYThaBoIgoNAgDEOQuDUdgtDENRBDKIQgigNQgDCLYeCANopDISIfGEMYwi6Lo4jwdgxDiN45i8MYkh4SIyDWQY4jqOQtDUSAyDCEoUhmGIEhqCRNDmHgwGiEYThWVw+gE",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,read,write,author,publish,inform,graduate,re-educate,study,learn,research,knowledge,improve,upgrade,level up",
        contributors = "danielbayley"
    ))]
    BookPlus,
    #[cfg(feature = "book_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDISA1C4NRhg2DQwCCGIXC0NRoDSEwwDEIg+DwL4DgWJIngaCAiG0MoNg8LQxC4OA3DmMo0DcOIjiWKopgSK4JE0MwgDEOYTHaMg1EEMoTg6ToYhoIISDWDhIDEOIVkWGZchaRR2juJImkCJBjGkchjGwZQgHKCQzCIIBjHiCQxDecBjHmdJhiWZpomoPoBA",
        categories = "text,development,gaming",
        tags = "reading,library,study,education,research,knowledge,discover,browsing,lookup,finding,scanning",
        contributors = "karsa-mistmere,Muhammad-Aqib-Bashir"
    ))]
    BookSearch,
    #[cfg(feature = "book_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBZDkaBoIgoOINDEaA4kKRJJkiBJKgkTZNDcaA2lKRZWD6AQ",
        categories = "text,gaming",
        tags = "reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "danielbayley"
    ))]
    BookText,
    #[cfg(feature = "book_type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDMaA0CIPg8C+A4FhSF4GgiCgxDIIA2HYN4ThWGoZgSG4JgsNggDgVg2EgOB2DKJIWiiJ4FgeKg0g4OQuDUdgtDENRBDKPwgkYNQgg2DQxiCRwyEgMQ5GGTpOkyDpZDEdgxDiVZZlgMZCg4SA2j+X5XkuYAtDUSAyDCNYmgE",
        categories = "text,design,gaming",
        tags = "thesaurus,synonym,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,typography,fonts,collection",
        contributors = "danielbayley"
    ))]
    BookType,
    #[cfg(feature = "book_up_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GgiCgxDgIAyGgMRhDGDggDCJ4miUMR2h6JImiiKAxC2KxIDYLg1i+JYxjALQ1EgMgwhOFYahmBIbgkTQ0g4OY4HaNA1EEMo4iCVI8iWNw1iCQ4WkeRoFgeCRtDmDooDMLQzCCaQzlyRYCkeYQimMIJamea5qm2XoBA",
        categories = "development",
        tags = "code,coding,version control,git,repository,push,force",
        contributors = "danielbayley"
    ))]
    BookUp2,
    #[cfg(feature = "book_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GgiCg0g4OQuDUdgtDENRBDKIQgigNQgDCLYOCANopDISAxDkYQxjCLoujmPR2DEOI4jqLwxiSDhIjINZCjmO46C0NRIDIMIThWGoZgSG4JG0OYOi4MwtDMIJhDOVIWlgPoBA",
        categories = "development",
        tags = "code,coding,version control,git,repository,push",
        contributors = "danielbayley"
    ))]
    BookUp,
    #[cfg(feature = "book_user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDMYQzCCEgwg4IAwC0NoXCIPg8C+A4Fh2IIGgiCg0g4OQuDUdgtgwQQyioIIwg2FYVDEIA2jEMhIDEORhjeN41haQB2DEOI/haQgxi2DhIjkNZIkGF5JC0NRIDIMIch6I4dGMaRyGMbBlCAYx4gkMQyCIIBygmaZkHmCQ4loL5emCYg+gEA",
        categories = "account,connectivity,communication,social",
        tags = "person,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking",
        contributors = "danielbayley"
    ))]
    BookUser,
    #[cfg(feature = "book_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CANwtg8NQiD4PAvgOBYWhmBoICITQ0CAMQ5g4dgtDENRBDKDggiuDwwCCMAxCANosDISIjGGM4zjCMoiiIdgxDiOo/j2IoniISI1DWRI8jGRYSEgMgwhWF4chuBIdgkbYkg8NwghOVYYlkPoBA",
        categories = "text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,reading,misinformation,disinformation,misinformed,charlatan,sophistry,false,lies,untruth,propaganda,censored,cancelled,forbidden,prohibited,banned,uneducated,re-education,unlearn",
        contributors = "danielbayley"
    ))]
    BookX,
    #[cfg(feature = "book")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIg4YQxh+HYdieKx2DEOImiiHgxhODRIiENYwieKYoC0NRIDIMAiD4PAvgOBQ+gEA",
        categories = "text,development,gaming",
        tags = "reading,paperback,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Book,
    #[cfg(feature = "bookmark_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGEMgghEMAghQMYShIdgxDUYYXheFIWC0MQuDQOQ2C4OA2DgbAtDQLg1DEMgtDKLw3DiEIYiCFYiC4OQ4DSFYsi6MIRjQNY2EGHoVkuFw1hIMBWhyRZLhaEozHoIg+DwL4DgWWpdgaCAiG0OQgDGFJTDSLZZluYA+gE",
        categories = "account",
        tags = "read,finished,complete,clip,marker,tag,task,todo",
        contributors = "danielbayley,jguddas"
    ))]
    BookmarkCheck,
    #[cfg(feature = "bookmark_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDASA5CIPg8C+A4FhSF4GgiCgxDcIAzGEMggiMMAgiYMYkiQdoMGGKYpiaKAtDELg0DkNguDgNg4GwLQ0C4NQxDILQykANw4iKKoxieMwuDkOA0iePY/kGI5FDWRxBi+J5cimDQyDAVg1kmJZdiSRB6hOFYaD6AQ",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis,jguddas"
    ))]
    BookmarkMinus,
    #[cfg(feature = "bookmark_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYNgiD4PAvgOBYThaBoIgoMQ1CAMQwEgOYShSGYYgSGoJgsNwgDMYYNg0MAgjIMQgjAdodGGNY1jKNAtDELg0DkNguDgNg4GwLQ0C4NYMC0MpMDcOIvjaM5WDCPwuDkOA0jOSZLk2MJRDgQY7leHwgh4MgwFYNZUjGVo1DKTx6iSFYoD6AQ",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis,jguddas"
    ))]
    BookmarkPlus,
    #[cfg(feature = "bookmark_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CAN4OC2Dw1CIPg8C+A4FheGoGggIhNDENwgDMYQyCCJwwCCKgxiiKB2DENRhi2LYqiwLQxC4NA5DYLg4DYOBsC2DQ1DEMgtDKDg3DiJoujaK44C4OQ4DSK5CkSRookoOBBjSK5fi2DwyDAVoyieKZgiiSB6haGIdhyBIegkbQ5g6EJ2hSbYZnEPoBA",
        categories = "account",
        tags = "read,clip,marker,tag,cancel,close,delete,remove,clear",
        contributors = "danielbayley,jguddas"
    ))]
    BookmarkX,
    #[cfg(feature = "bookmark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGEMgghEMAghQMYShIdgxDUYYXheFIWC0MQuDQOQ2C4OA2DgbAtDQLg1DEMgtDKLw3DiEIYiCFYiC4OQ4DSFYsi6MIRjQNY2EGHoVkuFw1hIMBWhyRZLhaEozHoIg+DwL4DgUPoB",
        categories = "account",
        tags = "save,favorite,mark,label,attachment,file,stick,pin,read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis,jguddas"
    ))]
    Bookmark,
    #[cfg(feature = "boom_box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAORWDUYQyCCEgwCCFQxhMLQyGgMQyhGE4WiGGISDIdg0CIPg8C+A4FimLIGgiCg4CAOB2DGKIqi+LoEjCCRNh2NI2jiK48juBYHj4MQ2kGN4pkSLQ8HIZRjHQIB3GkZIFgkMgwCIIB4luXh5gkOZeHKYAiDKXhoGUaRnGgdIJh2Q5SlSKRjGkchjGwZQgGOYwiDENZeGOaA4maYZOniep8neeZ7n2hZyDahKAoKiJpkOi6QD6AQ",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = "danielbayley"
    ))]
    BoomBox,
    #[cfg(feature = "bot_message_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2FYMhIDgIg+DwL4DgWFYYgaCIKDENQgDEMR2DKFIWhuGoEhyCRNg2DBoiWFYXiqKYFgeLAyDCIQyjCJozhmAoqjeCo5iENhhg2DY6joMQtkmEguDgMg4kgIJKCCSwtDELg0DENAuDUOA2GyTgujmSZmDAMhBC4N5bm6WJxDEIA0lYMJmmIVpUkmcZMlaThogyVZXn6SR6j6KJBjaHRNDmIYjjGJ40gE",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BotMessageSquare,
    #[cfg(feature = "bot_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg2DcIA4EgMQ4GEMgghcMAghoMYYhgdg0C4MwzCIPg8C+A4FiaKYGgiCoXDENBoDKJYniyK4Ei2CRNDKHIyjSJoojmOIFgeOwyheSIekCNpDgKOZGgoOIREgNoWh6GpZC2SR2hWSYbmCGpJGgMQyleGZhCAMQuDSMQtC4NQ4DaNZCiqT5Fi4TQ5moMx2kydRokSOoKDmDoQDSEwyn6IokkGN4B",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "taichimaeda,ericfennis"
    ))]
    BotOff,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA4FYNBIDgIg+DwL4DgWFRyGUYx0CAeYJhMIByHiCQyCIIIlCINIoHcaRkgWCQxDaKBoGUaRnGgdIyieFQvhuHYVhiBoIgqDQxDQaI9haQ5CgSRIJE0MgwCCSJKhSTJPk6BYHlEMQ1lUMx2kuF5agKT5dgoOZhmOWJlhmAQA",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "ericfennis"
    ))]
    Bot,
    #[cfg(feature = "bottle_wine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzGEMQghGDYNhEMQtDEaAyhCEgghSHYWHaGw2CCJIfhULgyg4Lg2GyLAuDgQYkiaHogDeEgzHYOIchONYXhYSI7haNYVhiGB2C2O4zkSIIpC0M4si4NgtjCMolkyFYNDUegiD4PAvgOBZemGBoIgoMY3DEMxoC0NI8liGISHaD5DieIBoDSXZfmQPoBA",
        categories = "food-beverage",
        tags = "alcohol,drink,glass,goblet,chalice,vineyard,winery,red,white,rose,dry,sparkling,bar,party,nightclub,nightlife,sommelier,restaurant,dinner,meal",
        contributors = "jamiemlaw"
    ))]
    BottleWine,
    #[cfg(feature = "bow_arrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGgNB2DQIg+DwL4DgWFYYgaCIKDEOAuDUNw1CAMQxC4MA4DIYQxDOJYuDAIIxDGJYoDQOAgDmKAyg2J4MjWP4xjMLYnDkMQ0iEOQ3EyR4lDeFIWhuGoEhyCRNj2MQzC4Mg5CANpbDmLAuj+PoNkKJQgC4NpEC6Rovm+Z40joMIuicMA1lCF5UlOBYHlaPQ0mKPQumaMoymwMgwDeIRsC0MpqDQNggo8NqREGIYhoaZ5aiSHxIDWLJonGaAxHaJ6gpiJJnjELg4DWSAzq8TI6p2IKJDcQYnoOhZxjMNx2o6oY0quRJEHqeZSgKVJ+gqOg3oqJZIlyLgyjQM7InuAQ",
        categories = "gaming,tools",
        tags = "archer,archery,game,war,weapon",
        contributors = "jamiemlaw"
    ))]
    BowArrow,
    #[cfg(feature = "box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA4GEMgghEMAghQMAtDGGAuDcMxsC0NwtDSEIShWJYXhOHg3CANBBhGE4mCAM4OHaD4ujCFINDGG4diqIo2haJYoiCLI/jCDAgDENhaCIPg8C+A4Fk2UIGggIhtDMLoyioOIbCANYOhsLQ1kyTpTlKBJUgkTQxi4MhWmyZJPmgPoBA",
        categories = "shapes,gaming,development,math",
        tags = "cube,package,container,storage,geometry,3d,isometric",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Box,
    #[cfg(feature = "boxes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ3CAMYMDkMhBDIIIWDAIIZhmFgxDQLg2DMdgzC4Mg0GGFoYhqK4Ng8MQuDcMRsDOEAuDiKIXiuG4XC4MA2hoTIRhAOR2C0NQuDUbJGC0Mwth8MI0gwNAyFoIg+DwL4DgWV5agaCAiG2Lg2kiTowDQLYMDgNZWliXZcgSXoJmGEJjDUIA1kybJZnCb4FgeCRNmKSB2kcMQ3nqboCnCf4KkIMYkDUVgxDmM4tjwM43imOorgyPoajMLYvpmOY7hmLahjAMZFiSJo4iqGwtqeL4xkGLgwkiSp2DOVZXnuW6Kn6XxtoadJkngM6InywJxmCxAxnUIIfDeZ5pmuvaJl2jBNs6daEC6hrJr+2ZfoGloflOFakpuD5jo8dofDMNxsrqd5MFa7Qzq6mwwrGDqojGSpNqK+qlmiPY/DDAY1DivJtsq46AkIOAgDeJY/oW1sOuKi7CxK0ZmwaarhGifbMtuHaQFYOMjD6AQ",
        categories = "shapes,gaming,development",
        tags = "cubes,packages,parts,group,units,collection,cluster,geometry",
        contributors = "karsa-mistmere"
    ))]
    Boxes,
    #[cfg(feature = "braces")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMxIDcYQyCCEgwCCFQwC2EgyHYNYRhOFogDGGYfhqIIVDGJIcGOJwuDELg5iSExoDEIg+DwL4DgWNo5gaCIKDENoTDGM4ehSJoThkdgtDWKwti6Lg5iMMoZkWR4ilMMhWh2JYXhaGYZGiTo1jePA+gE",
        categories = "development,files",
        tags = "json,code,token,curly brackets,data,{,}",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Braces,
    #[cfg(feature = "brackets")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgMxhDEIITDAIIWhOGR2gyEoUheHwxC2GRoC0MwiD4PAvgOBYoiuBoIgoOAgDIMRIDWHYViCIoiFYNI4h+GIUiKEInimLg+gEA",
        categories = "development,files",
        tags = "code,token,array,list,square,[,]",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Brackets,
    #[cfg(feature = "brain_circuit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GEMwghEMAgDEIAwC0NQuDkOQ3C6DA1CANIiheJYYDILg1DINoOC4Nw3iSI4UjMIIpDWLA2ikOA4EGI4yhWJYMhUOBaCIPg8C+A4FkeSoGgiCg5hUMxhDSKYilaNIUDMLQ0kaSJNkyBJOgkTY5DAMIRhqIBBhGE4mCCOQ0DCFo5DWXpJmKYYFgeZAzC4NIvhUMAuDgOQ2lSJI0haOg1C0Lgzoad5ggKYp8gqLAxDiiI/hQMQtDGGw2Dejg1DENqSnmlJ7k+C4NDEMxol2R54kuqpjgqQqZGih4Ng2iggr0dgxqitZNparQgDgaA4sQaJ6reC4sDgVoPr2b4WDILQys2RxjGkchjGwZQgGMeYJq8IrkHi56nCAcoJimd7euC4rdt+4bjuWCQzum7wivG6rnsys7zvi9r0vm6wiDIMLpvrC7Du68J2wS971DzBbixK/52wHC8NuS5giwOSMZGUPoBA",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,ericfennis"
    ))]
    BrainCircuit,
    #[cfg(feature = "brain_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg4DUMggDENAuDcNwyC0LgzDgMwuDkMgzCIPg8C+A4FiOJoGgiCoMg6EAgDkLgyDIOIZhsM4Zh+IYjiWBBoiiPoHgmC4dhMOIShSFgyhoOJLh8NIiiSKZAgWQosh0NQzDEIA4hoMA1jaHIeiCUY9ieApBiuC5JheEoNg+TogmGO5Sj6VIqkOE4Vm0MZFDSXY6kydJmj+aJVisTQxDcLg1DmRw2owQQzCCkwwhIIKWDGEQ1GGk6VpimAtDULg2jgMQuDQNA2pSrKWq6Ng2kepw1DeWw0CCt6uqGS4QDUIKjhYMZllOhp4CKiaLDmjq/C6mqcreuagluvAyr6wK1sOdrFlaiYxDWX5ujIOQ0GG0KgpkLazCCiw3DANrZmeKbchQMAzhGiqohwQbmrqmanqqtwyDAY7NjWGgxqcOIfjbCISC4MJtDC5a4ueoKLDS9gtrwMbvjyxLyoiFK+i0Mg5DG+8Uv0IKrDEOLwoXIIJE2kAwDCm7NtWnatxWqMODMNw1y+d5WG2MYzrKb4QjmcqC0K25q0aNISn7BqBjfThjGkchjGwZQgHKCYhCAYx5gmmgi2MeNmDKZdZ1vXQ+gEA",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,UsamaKhan"
    ))]
    BrainCog,
    #[cfg(feature = "brain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgVg1CIPg8C+A4FhSF4GgiCgxDWDgzGENAuDENwgiOJQgDCKoOC0MwtDSJ4kiaKImiuKwxi6J4ThWGoZgSG4JgsNwuDUOQ4CANpFEEMwgk2OIsgwIA1GGTZPg6KgtDWRZHg6RY8haQI/gWB5CiULg5DmJpbgyVIxjGN5YDKRQyDaUwuDcN5gj6ApAmWHZIg+IoniycQyC2RA0DYNJ7mKfZkhyCw5mgNommcNA4DMQZvlGWJSoKnJQjmRA5pULZFiWjYYo+QYKnaoKEnGOaHomi6qGiY6tE2SgwDCTZsDKbqxoULZzDWdZ3nmtw+gEA",
        categories = "medical,science",
        tags = "medical,mind,mental,intellect,cerebral,consciousness,genius,artificial intelligence,ai,think,thought,insight,intelligent,smart",
        contributors = "karsa-mistmere,jguddas,it-is-not,jamiemlaw"
    ))]
    Brain,
    #[cfg(feature = "brick_wall_fire")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzHYMguDEMA3CIPg8C+A4FheGoGgiCgxDcIA5GMMYOCCEQ1g4LoqDOLAgDSLBBiqKgwCCNomDIMggiEYY0jeQAxC2E43GMMAtC4M43kgNoSkgORhjuO44kCLgzC0MhDDGSgxDGL4MiOPIiDmFoYh2HIEh6CRNDKJg4C4Mg3DQVg1lGKJAjaRwylgSJ1lKeJLlIdgxDSdpToCUhoi4OQzDiZYZmmaIFgea5cDUaA1nANQzo+Z4CmmlIKkoORom+OqOhekIbp+k4fE0OI8DUdg2p2kasmqCqwg+tKpmeAQ",
        categories = "security,home,connectivity",
        tags = "firewall,security,bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone,campfire,camping,wilderness,outdoors,lit,warmth,wood,twigs,sticks",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWallFire,
    #[cfg(feature = "brick_wall_shield")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA5HYMQuDINQ4CIPg8C+A4FheGoGgiCgxDYIAzHYNQuDQNoWhiHYcgSHoJE0Mgxg4LgxDEOBWDUYYNg0MAgj4MAtDKQhIjqPI/kiQY8hANI7CCPZJk+TxoiYNw1iqGYui2BYHjAMoNDENwuDUY4+DKJw5DkLYRlaIwuDcNJqDMLg4DMIA0iecBhmOY5RDELQuDMMw1oAMAwmSQguDAOKEm+Q5znWhIRDkNwtpCgwtngNA3FYMZNnwNZ+CCY6AnEORjjOZoSo2Ipzgya6BDaew2C6IpACCM5vDahQwDEY50lauqhmeDK4rGbrFhEM6yqCoqgHqWIsgKLpdgqdgxDUaA3tGWrTlyHxNnYORogyNQ0DK3Ibt6L4KDiuA1HaKYXlm6odtUTbuiS8orlqAQ",
        categories = "security,home,connectivity",
        tags = "firewall,security,bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone,cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWallShield,
    #[cfg(feature = "brick_wall")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEeIUhEaBlGkZxoHSDYPhEcoXCIMgiD4PAvgOBYoHAYYLCAZINE0MQyCAOR2DaJ4pi6C4ti8aIxjMMQ2CAMQ1jmOwvj0aI/jCMgijSRQzkmKJLkCTpBlATQzkYNRoiGVpMlmQpRl0OZghCYpYDyTJlE0OJelWPJsm6W5xlSOprj6AQA",
        categories = "buildings,home",
        tags = "bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWall,
    #[cfg(feature = "briefcase_business")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMGgLgwDEIg+DwL4DgWFYYgaCIKDENggDYVg0GGDYNDAIIoDALQyiwaAtiSJopjOK4mHYMoUhaG4agSHIJE0MoNDEMxhDEOAuDENYOkeSY0g6LIqjmF49hUchlGMdAgHIeIJjgIB3GkZIFl0MAiCCXAil4aBlGkZxoHSCQxDSZh5gkNpSlaWA+gE",
        categories = "transportation",
        tags = "work,bag,baggage,folder,portfolio",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    BriefcaseBusiness,
    #[cfg(feature = "briefcase_conveyor_belt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDAdgyCIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIZiWB4JgsOIghGE4VhqJ4FimCgyDGIBIDOL4khiAoohwTQ2i2IowiWMobiqLAxDYVg0GEMoOCCDYNjcMgtDIaJOlCUJTCCVYOHYMZEjyJg8HIZRjHQIBoGUaRnGgdIJgwIggHIeIJhIIJ3CINJ0HcaRkgWcg2nQeYJoSI5nmkPoBA",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase,conveyor,carousel",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BriefcaseConveyorBelt,
    #[cfg(feature = "briefcase_medical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDEdg0CIPg8C+A4FhSF4GgiCgxDSDgzGgLYShSFoEGiGYngeCYLDYIA2FYNBhg2DQwCCNgwC0Mo6iKMo0jeQI5jQdgyhOFYaimBYrh0OIvHaHpGiaGICiqHBNi4NpPiSR4nhQchlGMdAgGgZRpGcaB0gmUAgHmCQ2CIIB3GkZIFgkMgwnAeJ2nAcp6CKRYll+YQ+gEA",
        categories = "medical,transportation",
        tags = "doctor,medicine,first aid",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    BriefcaseMedical,
    #[cfg(feature = "briefcase")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDAVg0GEMoOCAMIWhYLQyhoaAthKFIUheIoag4doMCIPg8C+A4FikchlGMdAgHcaRkgWCYPCIIB4jiOhoGUaRnGgdIJDENI6HKPAiDKOh5gkNooiqL4xD6AQ",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAeYJgsIByggIgygwaBlGkZxoHSD4MHcaRkHQaIdD4PAvgOBYlHAYYiCAZIJE0NAgDEMBhDIII3DAII6DELQyj4Vg0jaOI7kUMY4j4aJCjeOZGkSFYlC+K4iiqLBoi6MAxjIMo1kyRY6jqTJKkOTZgkgMh2C2S5EmYMI+j4IpRlMaA+gEA",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[cfg(feature = "brush_cleaning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAyDILQxC0NAiD4PAvgOBYWhmBoICITQxDkIAxDQYQxiMIAwimK4SDEdoRGEMoOiuKgwC2EAyGgLQziaKI1iOEYRFaJYyjKP42DSKR2DWPYnj+LYjEgNoxjOR43g4dgxk2NIsiOFYXhyG4Eh2CYgiKJBIDUbIRC4OQ3DMIA2C4Nw2DcQYnk6XJJg8aIMluRwgm2b5sDIMwzHqX4YmOYoFgeCRtDiDoyhKFIWoqGoBA",
        categories = "home,tools,design",
        tags = "cleaning,utensil,housekeeping,tool,sweeping,scrubbing,hygiene,maintenance,household,cleaner,chores,equipment,sanitation,bristles,handle,home care,sanitize,purify,wash,disinfect,sterilize,scrub,polish,decontaminate,wipe,spotless,remove,empty,erase,purge,eliminate",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BrushCleaning,
    #[cfg(feature = "brush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDAIAzhEIg+DwL4DgWFYYgaCAiE0NguDUIAyDEQQziGEYohCDYQhIMQ3iEYQyC4NgyiONI2hCKwtC4NwwDiDo9DkMhBg2LAgjqEYjDEeoUhaG4agSHIJE0OQuDkNg5g6MAwDODYkC4Mw3kANY4DQYZGkiagwC2JwwDAMptC6bwyEyIJYDeDg0nOXpOheUg+gEA",
        categories = "text,design,tools",
        tags = "clean,sweep,refactor,remove,draw,paint,color,artist",
        contributors = "ericfennis,chessurisme,jguddas,karsa-mistmere"
    ))]
    Brush,
    #[cfg(feature = "bubbles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4MAwDEIAxDWDQ4DUQQxC4NYRhkIAwh2EQgDmEQ2hkIg+DwL4DgWJxjGkchjGwZQgGMeIJDEOIlCAcoJDOORjHmCY4DWJooi2L4xiyLowjKP42iSQ46gmE5QjSCYMkOJwvkaS5JkeTI1CKVwilEIgyj6QAiDSJZZluSIBA",
        categories = "weather",
        tags = "water,cleaning,soap,bath,hygiene,freshness,wash,foam,cleanliness,shampoo,purity,splash,lightness,airy,relaxation,spa,bubbly,fluid,floating,drop",
        contributors = "vqh2602,jguddas,karsa-mistmere"
    ))]
    Bubbles,
    #[cfg(feature = "bug_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAdgtDgIg+DwL4DgWFYYgaCIKgwLg2DUNggDcSAxDQYQ0CCKgwCCLQxiuKx2DELgzDQNIUhaG4agSHIJgsNAugwIAzC4OA4CAMYjDKOYXj2PIFgePwxDeQgyDOSZVgwMxBiOI4ti8IIjieEQzimMZgkmSQuDcMgtkUMg4DeTY7gKPZSCIbYNnuLYPnST52lGHRNDKMA1meLIukmbwuDUNZEC4OZzhWToZoGPoKDKDQxDMaKMjaOKUnWG54E2WKFoiiphkUOAxC2oY6oCpKDlih4qomaZFo6kKSn+lqzj+YwzEiTKirKd4dG2SKbkaSI0kevholCmBNDmbJDkEMA2EGWJYmmMAxo8NozkIM7RD6AQA",
        categories = "development,animals",
        tags = "issue,fixed,resolved,testing,debug,code,insect,kill,exterminate,pest control",
        contributors = "danielbayley,jamiemlaw,jguddas"
    ))]
    BugOff,
    #[cfg(feature = "bug_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDkLg2DUNRBDYIIWg2DQxheDg0HYLQzGENAgiOGYOiQLQ0GgNIiiQIImhuIwzC4OQ3CIPg8C+A4FjiO4GgiCgxiMMQ1C4MAwiGG4bjCDguDUMQ3C0Lg4DUORsDSNI1CAMpZDcYZKi+YZLk0NwxDgbIpl2W5dl+J4wC0MZOlCUg4DYeo3jmPo9gSP4JguWAxDIIIzDgOIOhYMp4jqfJ7gWB5+DKGw1i2JZiiCTg1oONI2jii48gKfKPgoM5bDGlJhhqmg4DGKaKnqoKOkATakpOI6ViaM4TpqNauoysJ9gqFgxDMSKJp2r4+qIbaGoKcaFk2ha9p+yayDkIA3C6wxWDYYakqSqYbhgdpxsO0hoD6AQA",
        categories = "development,animals",
        tags = "issue,testing,debug,reproduce,code,insect",
        contributors = "danielbayley,jguddas,karsa-mistmere,jamiemlaw"
    ))]
    BugPlay,
    #[cfg(feature = "bug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAdgtDkIg+DwL4DgWFYYgaCIKDENAgDcYYgiAMAgiYMQgiQdgzGENggi+JooC2DInhGLYkieOopDQLQ0HqFIWhuGoEhyCYLDQLo1DMLg4DgIAxi8MpBheRZEgWB5HDKKZbiOKo6jILZMDgMY+lSQ4CkWWYKlsIA1l6JY7mILg1DUIJMDkN5nlaaZYh0TQyg0MQzGiZoVlWGZ9kaCgzg4MZwmCUJ3k2ZQ0nuiYbmsTaNm+OYyjqTJ1pOeaXGiV6LE2L6DEiU6HmimYdG2T6Ck2TwxrWpanpoOYhkoMxWDYYaNo2KKSjEdq3oOuYB",
        categories = "development,animals",
        tags = "issue,error,defect,testing,troubleshoot,problem,report,debug,code,insect,beetle",
        contributors = "danielbayley,jamiemlaw"
    ))]
    Bug,
    #[cfg(feature = "building_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIaA0CIPg8C+A4FhSF4GgiCoMCAOIRhOFYahmBIbgmCw0CAMgxHYLQzGEMorCCDY1C2KgwHYM4ihaJolgWB4oDaDgwEgNIxjONY0C2MgyHYN5IjKSoNk0aAxDaUY0lqVJMFYOZZlOTJMGiTI8iSAomkGCpDiwVg1mCWgxiuYw4nCDZyk0dpXmaPoBA",
        categories = "account,buildings",
        tags = "business,company,enterprise,skyscraper,organisation,organization,city",
        contributors = "maxim-s-barabash,ericfennis,karsa-mistmere,jguddas"
    ))]
    Building2,
    #[cfg(feature = "building")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAaAuDAMQiD4PAvgOBYWhmBoIgqDIODSEYThWF4chuBIdgmC4NDaI4UhaGIpiiBYHisMQ2g6EISjCJozgKKY2h+OQxiKPIljKGpAjWHoLjmLpHjGJ5LiqCg4jqL5IlOHJCE2V5FlmUo/lyTZXlCJJikqZIrDkIAyDIdgtDMYQxg4IAwnedgxC0MRoDSdJ2nieJ1oQdgzlqPxyGUYx0CAeYJDIIggHiCQ0pIdxpGSBYJjikhypQIqRCAaBlGkZxoHSkAwkiiqMD6AQ",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANggDIIA3CIPg8C+A4FhSF4GgiCgxDCDRoDSE4VhqGYEhuCRtDKEA3C0MgtDGI4WieFByGUYx0CAdxpGSBYJDENgiCAaBlGkZxoHSP5BCAeIJiIIByk0IgykIeYJDOMo2jiJoFgeCYLCAMQxGiQIyiWAonl6Cg4mENRoC4MIxhSM4YmiXYcE2QJtm+cZmjSdoogqDgxDkdpUnOZ4amqeZsDIMR2i6foYgE",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANh2DYIg+DwL4DgWEoVgaCIKDENYNg+EYThiF4EhmCRNDIIAxDIaAxDkLoQhKFIkiOBYHiYMYMjgaAzHMLg1C0MQuDcLg4C0MpEGMLgxC0Lg0C4MpMDiT5Ak8IAwkwNJMkuRJMlCQQyGyVJZDUQwyDCSoNkSKYuDGDYpgwNhIDQYYoigMJWlaRggDIdgxDCO4gjKFg8GMaRyGMbBlCAYx5gmOAioseIJDekBygkMqBoWh6JjSJYKDmbxoDWgYioShqIoqlgipikaOhCi6NCKj4xpqqA+gE",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzGgLgwDEIg+DwL4DgWFYYgaCIKDENAgDKEIShSFobhqBIcgkbQyCAOYhDALQ1iWF4piiBYHgmC4tDEMhWDYLozhWNYZDwchlGMdAgHcaRkgWCQxDYIggHiCQ0lMeZQDKUxylUIgzlMaBlGkZxoHSUAwjSR5JjeKoKi+PR2kKJo2gKKY5h4NQgnGc5EGibZ4E2IAxDcaJRjSJ4BA",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[cfg(feature = "cable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDkYQxg4IAwhSDgtDGGB2C0MhhDIIIfhWFYSDKHBoh2H4hhaJIgHaHYShKIoXjAegiD4PAvgOBY3jqBoIgqDIgDGGwyjaOI9jyBI+gmCw5g4NBWDYLg1hGE4yDALYNDAdgxDGVYxiuWYUFYMQwkaOZKkmBYHkwMokkOHJnkiApKmyCgzCANRWDOcppnSa4/E0NIODCHoghaI4chyUZfoiE4ZDEaA0o2Mowg6LqGiqiYpjWN5ojuf5LgqDZ6nynpIgEA",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,energy,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley,jguddas"
    ))]
    Cable,
    #[cfg(feature = "cake_slice")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDMSAzCIPg8C+A4FhSF4GgiCoMg4N4RhOFYahmBIbgkbQ3C4MggioOQtDMLgzDgOAgDILg1EGLIsDAII9j0M4OjcMAxFYMgwGEMYOj6TI9koMRogySZLj+TAxC0MR2C0OAuDYNQ0GMMAtDKYwuDQNAtDYLgwDINppmaaJcmwNpTkqVZiDGaw4kMNQ3EwMQwmYIA1l2IoWiaFBjGkchjGwZQgGMeYJDcIqQHiCQ5pUcoJDKhqKoyjg+gE",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = "danielbayley,jguddas"
    ))]
    CakeSlice,
    #[cfg(feature = "cake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAyDEdgtDgYQyg4IINhgLQyhoSA2hSFoYheGoOHYOAiD4PAvgOBYoiuBoIgoNAgDENhzC4NQtDGDo5g6N4WjKFQyjeIw0iOQg1j+FobjqS4nimLotgSL4JguDgxGiDJOiqUpRgWB5UDcIA4HYM5alCApSl+CgxhWY5liiW4smiXowE0MZhm6ZpcnOU4KmENBoC4MAxnqcoumqdoVoCgqEnCZ6HnWdwgoug6FGgPoBA",
        categories = "food-beverage,social,account",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,fondant,icing sugar,sweet,baking",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Cake,
    #[cfg(feature = "calculator")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINAiCAeYJDKDB3GkZB0GiCQxDaDByggIoQCAaBlGkZxoHSDwwCIPg8C+A4FikbBpG4ZYNDGCYZgeNAiDiDB5DKNYMHiPQihiKIqi+MYujCMo8heOoHkGQ43heNh5jgMYLikL5GGWKRwGGFQgGSCRNhgIAxDAaAuDAMZEC+XYVlyXhomCYgxDKZZnmma5Ym4aJwl+YQiE0OJ3miapsnyfpyoCY52lahZ6iqiA8nyc6BoOjp5oecaJpWjJlDij6am+k5xp2l6gpme6bgE",
        categories = "math,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[cfg(feature = "calendar_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDQaAxHYNAiD4PAvgOBYWhmBoIgoMQ2CAMoThWF4chuBIdgkTQzg4MIRDiJYYimKIFgeKw4iKJIWjOGg8HIZRjHQIByHiCQyCIIB5gmFAgGgZRpGcaB0gkMYxCCRgiDOSR3GkZIFlWMY8kCQg+gE",
        categories = "time",
        tags = "date,month,year,event,single,singular,once,1,first",
        contributors = "colebemis,ericfennis,peteruithoven"
    ))]
    Calendar1,
    #[cfg(feature = "calendar_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDgIINhILQ0CIPg8C+A4FheGoGggIhNDENggDIdoVheGYEGiHIqgeCYhhCDB2DiFoYh2LIFi6IAyDGDgxC4Mw1DQVg2GEMokCAMJJkkLQyk0SA1kaSJKlSTYkHaDJSkeVJLkcMhoDeQA0DONYphuAoth8TQzg4MBog+ZY3miOZqhCJYnjaKg+gE",
        categories = "time",
        tags = "date,month,year,event,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    CalendarArrowDown,
    #[cfg(feature = "calendar_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDgIA0C2DYUCIPg8C+A4FheGoGggIhNDENggDIdg0haGIdhyBIegmIYQDKJQtDiKIZiyK4FgeLgyDGDgxC4Mw0DMVg2GEMokCAMJJkkLQyk0SA1kaSJKlSTYkHaDJSkeVJLkcMhoDmNYqgKLI6iAM4ODAaIPmKN5kjmHxNjCJpthuAQ",
        categories = "time",
        tags = "date,month,year,event,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    CalendarArrowUp,
    #[cfg(feature = "calendar_check_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiF4EhmCRNDIMQgDENBWDYYQyg0IAwjKMgtDKNhIDWL4xjOPY2h6K47jCPY0jAMhoDiIIUiSI4FgeJgzioMBoDGSYSkuFoCiSTwiG2HINjORggDQLYQleIoBA",
        categories = "time",
        tags = "date,day,month,year,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[cfg(feature = "calendar_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAch4gkMgiCAeYJhAIBoGUaRnGgdIJDEOItHcaRkgWOY7CCKgiDOIAviSJoXgSGYJE0MwgDEMBojqR4igKTIHgkbQ5lGHQyg0IA0C2EIShSTA+gEA",
        categories = "time",
        tags = "date,day,month,year,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[cfg(feature = "calendar_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDQdgyC4MhsDELoNDEIg+DwL4DgWG4egaCIKgwIAyHYNIahyIYggSIoJE0MgxCANwuDUVg2GEMomCAMI9j0LQykESA1jqPI+kiQYmHaD5GjuSI/jsMhoDONoqh2LotgWB4wDODgwGgNZXiyAoulyCg4kuKYbliHw8GMaRyGMbBlCAYx5gmDAiCAcoJDaexjHieZ/mycJynQPoBA",
        categories = "time",
        tags = "date,day,month,year,event,clock,hour",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    CalendarClock,
    #[cfg(feature = "calendar_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDULgyDIOAgDENguDgNQyC0Lg5DIM4ZDMOAzCIPg8C+A4FiOJoGgiCoMg6EISDkLgxDQOIZhsMwuh+IYjiWBBoiiPoHgkTYTCAMh2DSIokimQIFkKLIUDQN4SDSOQwg2HwyhqHJKj2J4CkGK4LhSFgykYMAuDcN4YjmIJbkmPJMmCTpiDGMYzhGLYPDibYdluO5Lj6TYqgmC4xDUM5GDELg2DkNoeDibIbnCgZfimTxtDKaJqmaE4Vheb6QoCXo/nOhIKpqaZrjCMo0m+fZdnKl4rE0MgxhKaA1hsVg2GGZpmDAILBDALYYDISA1r6RrCsyxK/HaM7KsCzbLDIaA2rGgqmk8TaJDEMBoDEOLZpaYZDhGR6UqSIxjGkchjGwZQgGMeYJuIIrzHi9rjCAcoJqO7bvvEPoBA",
        categories = "time",
        tags = "date,day,month,year,events,settings,gear,cog",
        contributors = "karsa-mistmere,ericfennis,AlexandrePhilibert,UsamaKhan,jguddas"
    ))]
    CalendarCog,
    #[cfg(feature = "calendar_days")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAdxpGSBYJDEOAiCAeIJDOMB5gmEAgGgZRpGcaB0i2LwgHKMgiDKIAviSJoXgSGYJE0MwgDEMBoi6R4igKTIHk6DAxDQaAuDAMZWkyS4FlqGwylGXpgmKEoUmSWJmhoTYcmqX5hmOFpxk2CpcDid5tiGcIYmedJpi6gJ5GiZZ8nSHaImyig+gE",
        categories = "time",
        tags = "date,month,year,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[cfg(feature = "calendar_fold")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgwGEMoNCAMIThWEQyGgMYPDILg0g2HYVhSFAxC4NwwDYLYlicbAzC4NQ4DgLYti8OBBhyHo3iGFgxCAMQ2FYNoQhKIoTC0MpGEgNZChGRAwkaDR6CIPg8C+A4FlOVoGgiCgxDWDQyHYLZKjyPJEmQLQxGgNZSlSWZYgSWoJE0OINHYNJslWcJvgWB5yj6dZ3lOeZXgKcJ9gqDIahkOJ4m6AQ",
        categories = "time,files",
        tags = "date,month,year,event,birthday,birthdate,ics",
        contributors = "danielbayley"
    ))]
    CalendarFold,
    #[cfg(feature = "calendar_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILoMDcIAyDISA1GEMoRCAMIZCAMQtDKHhWDaFoYhqGgxhGHhoDENIjheJYchgMh2DWDgyDUIg+DwL4DgWOY8gaCIKisLg2hcMQ4C4OBBg2NoRC6TYmjCR4cjQOAziKTA1k6UJSCAMwuDMOJODkNg2GyHpEDINpODgNYiC4OQ5kicZii+HQxC4NQwhAMB6jiOo/j6BJAgmC5rjINJ/jug6CgWB6FDOHAwioOKKoGAqDo+CpioilqMgE",
        categories = "time",
        tags = "date,month,year,event,heart,favourite,subscribe,valentines day",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[cfg(feature = "calendar_minus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAdxpGSBYJDEOAiCAeIJDOMBoGUaRnGgdIti8IByjIIgyjAeYJhCEgviSJoXgSGYJE0MwgDEMBoi6IIUkyS4FgeTpSlENhokaIZYgE",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "danielbayley"
    ))]
    CalendarMinus2,
    #[cfg(feature = "calendar_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDkaA2CIPg8C+A4FhSF4GgiCoMCAMh2DSE4VhqGYEhuCRNDIMYODUVg2GEMofCAMI0jQLQyjgSA1jGM41j+OIfHYMQ0j2Mo/jaMgyGgOAuDWI4WieJoFgeKQzg4MBoDEOJQiWAonlWCg4kKIoUlGGIBA",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarMinus,
    #[cfg(feature = "calendar_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4MgggwMhBg6DgwCCFYVDMIA2HYMQ0GGE4WiGFYTGiHYfCCFIiCAMQuDgMgtiwMQ4CIPg8C+A4FjWOIGgiCgyDGKw1C4NRWDaJ4phcLYvDISA5kONI2juOoEjyCRNDENooHYNJQjeVJTgWB5WhkMQwGgN5dlKApUmKPpAmUaAtkINZpl+a5hj0bYgDKIwwnWOYBA",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[cfg(feature = "calendar_plus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAaBlGkZxoHSCQxDgIggHiCQzjIeYJhAIByjQIgyjIdxpGSBYvjGEgviSJoXgSGYJE0MwgDEMBojCIIUkyS4FgeTpSlENhohCR4igKTJbhsMpRDSH5iliAQ",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "danielbayley"
    ))]
    CalendarPlus2,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDkaA2CIPg8C+A4FhSF4GgiCoMCAMh2DSE4VhqGYEhuCYLDmDg2HaEoUhaJ4mgWB4pDIMYODILg1DkOBWDYYQyh8IAwkSRAtDKSBIDWQZDkWT5Ih8dgxDSTZCk+RpCDIaA4juI4xhiAonjWCgzg4MBoDEOJfiWYo0hwTQ4lKIowiWAQ",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goNINgkIgzg0dxpGQdBogoMQ4g0aBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIKE0MQ2CAMh2hGKAvi6Goti8aIxjMMwgDEMBoh0Io7j0aI/jCMgiE0OI3jmSopkyTpBlCNA3kUNBoC0NpWjyQJZkKUQxkSHRIDeY5YDyTJnE2XQxl8LgwDGbplnCQJyDGdA4Gid55kuZYBA",
        categories = "time",
        tags = "date,day,month,year,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[cfg(feature = "calendar_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyHYNAiD4PAvgOBYThaBoIgoMgxCAMQxC4Nw1FYNhhDKDggDCKoqC0MouEgNYnimK41i6Dh2DENIziiNYsigMhoDcLgyDWEoUhmGIEhqCRtDKQIviEOIjC2UojkeFZLkqBYHgkTQzh8MBoDEOJYkmApLl2Cg4jiEYTlmFw8GMaRyGMbBlCAYx4gmZAiCAcoJDOfhjHmfJlm+c51ncPoBA",
        categories = "time",
        tags = "date,day,month,year,events,search,lens",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarSearch,
    #[cfg(feature = "calendar_sync")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDAdg0GgNAiD4PAvgOBYWhmBoICIbYMg4NIOC4NQzDULQxC4NgwDUYQ1CCMAwCCM4NDiJA1hWF4chuBIdgmCw2CAMoRjqGI+j2BYHgkbQyg0MQ4imJYniSLIujCMo0g4LZRiqOYWkeGoCj6S4Kk6Q5EC2EpqkaPJjkqHhNmcOIlFYNhhDKQ5ajMMAtDKfhIi6eZ5nyNJ+kMdgxDSeJ6oWM6DhMLgzm2SJvj+Cgzg4MITpSYocmUTY3kSFJgjyAQ",
        categories = "arrows,time",
        tags = "repeat,refresh,reconnect,transfer,backup,date,month,year,event,subscribe,recurring,schedule,reminder,automatic,auto",
        contributors = "danielbayley,jguddas,karsa-mistmere,chessurisme"
    ))]
    CalendarSync,
    #[cfg(feature = "calendar_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiF4EhmCRNDIMQgDEMxWDYYQyg0IAwjKMgtDKNhIDWL4xjOPY2h4MQ0juMI9jSMAyGgOIghSJIjgWB4mDOKgwGgMZKhKTIWgKJJQCIbQxDeDYwDULQ1kuIpbk+GpfmGYAgDWb5nk2AQ",
        categories = "time",
        tags = "date,day,month,year,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[cfg(feature = "calendar_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAeIJDMIggHKKQiDKLB5gmEAgGgZRpGcaB0gkMQ4iwdxpGSBY9j+EgviSJoXgSGYJE0MwgDEMBoj6IIUkyS4FgeCRtDENJRDQLZfhCR4igKTJbCKXQwmAIJjlaIoBA",
        categories = "time",
        tags = "date,day,month,year,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAeIJDMIggGgZRpGcaB0gkMQ4iwdxpGSBY0jYIB5gmEAgHKKQiDKIAviSJoXgSGYJE0MwgDEMBojWR4igE",
        categories = "time",
        tags = "date,month,year,event,birthday,birthdate",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "calendars")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgiD4PAvgOBYThaBoIgoMQ1C4NwyDaDgxC4MAxEGDYNDAIIrDEIAxDSDgyEgNBhimLI4DELQyjsVgxDCNoOjiLYOjuEoUhmGIEhqCYLDiDoQkeFZLkqBYHk2DQxDMaIRhOU4XgKS5XgqTw4GiMJSkkPByGUYx0CAeYJDMIggHiCQ4nQaBlGkZxoHSCZoCAcp2CKEQgHcaRkgWgA0lKbJuD6AQ",
        categories = "time",
        tags = "date,month,year,event,dates,months,years,events",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere"
    ))]
    Calendars,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1DYNAggyDg1DgYQzCCGAwhKEgtg0MQyDKHguiAMQiD4PAvgOBYoiuBoICIbQyCCMwyhuNonimLotgSL4JE2No0DASA0GGNQghuGwxC2IgyFYOZGjSSJTDGNJMGgMQuDkOQ3lGM5JlMLg4DEOQtiQNw1jmKo9jyBYHj8OQuDYOQ1CCDQwDINBBkeYJVDEMAuDAMIRDQaAzloOQzl6U5KhILg3DajgwDUbAuDQOA2lqe5SmCSg4oEMIYDcSI2oufZSDIdg3C4Mw0DSao7gEA",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg5DkNwgDQYQyCCFQwCCGAxCAMQuDcNocC4MA1GwLg0DgNoOEGFYXhmLgxDiIgwDMIA3EgMgwhSFouhqOwyHYOY6i2GgtiwSITiyPIckWRRWkGSYYj0MpFGiHYPDeQpKhqHg1DkLYdDCJ4lieXoOmGK47lGHIcDCMg0hIegiD4PAvgOBZzGMaRyGMbBlCAcoJDMIggGMeIJDEMqDGMeaHoKcwvnme59D6AQA",
        categories = "photography,devices,communication",
        tags = "photography,lens,focus,capture,shot,visual,image,device,equipment,photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis,karsa-mistmere"
    ))]
    Camera,
    #[cfg(feature = "candy_cane")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NwgDIMRhDKDwgDCFQgDELQzC4NQtDIbA4C4NgtDENBhDYIIohaFgxhgMAuDSKYUhOLIYhqMA2DSHozheLQwjcNI5h4WgiD4PAvgOBZGkmBoIgoMQ3g0NQgg4MZTDILgxkWR5MkuBJNgkTQxi8OQgDQLg4hgMwgDmW5Il+XoFgeCRtlGZQ5g2FJnDSbpdgKX5zgqZ5liWeZViGbZGm+SoBA",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CandyCane,
    #[cfg(feature = "candy_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMHYNwuDkIg+DwL4DgWFYYgaCIKDEMQuDgMAyCANguDENA1GENQgiyDYNDGJQuDANQzjKNAzhSFobhqBIcgmCw0jIMR2DILgyDSOYVhePo9gWB4JG0MQ1C6LJTlUNwxC2Eg2DSXA0iuLQgi+DgthEMJZmOYoumqMQwmaMw3GyX5bDkNg1jqTIZgKPpQh4NggDcVgzGGMZum2DguDeaJbosNwgkaLKRmqZJGlORpZo+hqUg6iQxDOhonqCkJVqSbJkooMaPpYNQyEGm5kjEMoxDgaAtDSeY8nyT4dG2JK/g0MgwrmTa7j+Cg4g4Nx2mCsJtC2IKOoqaKmtWZJvqwMqNqqnaHjC0KilqIKfrKpaTte2w3C22avp2sQgjYMQ2GiuJLjyAQ",
        categories = "food-beverage",
        tags = "sugar free,food,sweet,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CandyOff,
    #[cfg(feature = "candy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA3HaDAuDkIg+DwL4DgWFYYgaCIKDENAgDYLgxFYMQ3hSFobhqBIcgmCw2g4VgzGEMQgjWDYNjUMQuDcMA3C2PI+CAMguDWQ5FCCOJJkcMQ1kQN4mjaUpKjqIwzlUMZXkeRpEkaSoNjyUZEk0MhBjqS45kONQ4GgLQ0iiF4siuBYHi4MYiDUM4wDcLg0DYNRhkaXpoC2fI+DKSRsC2iAyoEIKDl+DgukKgpokuhg3ogMBsDKi6OpChKYnCKoCiydYKDiNoPDSNJTksMQtjuPZ8rOW62l+i4jk6QJQDeUo3q+sZWrCO5ZjWXa3oSYY/mOTpmq6aQzjYNhom+FZxhmAQ",
        categories = "food-beverage",
        tags = "sugar,food,sweet",
        contributors = "karsa-mistmere"
    ))]
    Candy,
    #[cfg(feature = "cannabis_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDQYwxC4NQghWFwzhYIAzCANodCAMAthkLYWC2Gw1C0MgtDUIg+DwL4DgWMIzgaCIKDGGw5DgOAgDgLgzDINxDjoLg5DCDQ2C4MA1DSGIbDMNoaC4OINgyDhhDmQQgluHgwiKIokC4NA0hcMpHi+MY2jWBI3gmCw3kEN4XDGFQ3DINZFkCPZXDCFoeDIMQgnIOA0oOgofGOK5BDKT4jDWTI+hUMg5DYLZLDYNoNmgNqGmqMpum2BYHgkbacg6YAyDCoJsgKbqlgqgpMo6GKRDMNw4EEMZ1heYK/g6Vw1ouZAypelA3DmjA2DejA1DWzQ0C6eaMkgN6tqKr6kjgTZLDkOYXt4OQyEOkQ3DGT5Lk+0qVh+IQ2GOqoOhyF7pmOvoShyJYah2+ociCg8AveGIng2KYrCCZ4cmC0oqhmH4otiNIB",
        categories = "nature",
        tags = "cannabis,weed,leaf",
        contributors = "nickveles"
    ))]
    CannabisOff,
    #[cfg(feature = "cannabis")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDQIg+DwL4DgWFYYgaCIKDcIIMGMLQxC4NQgDCEoliCJQtiYMwgDOKokiYNogCCNQxiOMosg2MQ1C2DYmDKKgwCANIsjONwti+SZJj6MI3lCKIzC2LI+kCLYnlqQw1ioNojlWP5UlcM4tkuK5ikaEpql+XIni2O4/h+L4olyYZmDILQ3jqQpAkaPAgnuVJUkeP5lnudIOg6KomDSN4UhaGw+gE",
        categories = "nature",
        tags = "cannabis,weed,leaf",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Cannabis,
    #[cfg(feature = "captions_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CANRIDEORhDIIIWDAIIZDGF4XHYOIOCIPg8C+A4FiOJoGgiCgxDcIAxDEaAtiGI4lgQaIojeB4JgsOYvDkSA1hWHYZhsLQykcVg3kOGIai+F5HiKJIpjmBY7CIbYWlqGQyDCUo2ieAo6isTYujAaA0l+VJilaZJmDUaAyjSU43D6AQ",
        categories = "multimedia",
        tags = "closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "DefaultLP"
    ))]
    CaptionsOff,
    #[cfg(feature = "captions")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUDSDR5goNYNgkIgzg0cohgsIg+DwL4DgWLRwGGEggGSChNDcIAxDUaA0E2PI7j0Mo5jsMRokQMQzkaPosi6M4SD6AQ",
        categories = "multimedia",
        tags = "closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "DefaultLP"
    ))]
    Captions,
    #[cfg(feature = "car_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIA4C0MggDILQxC4NQtDMLg3EGEYRDAIIfh8MQ1C4Ng0DYIA1EgOAuDQYYdiCMQwhQLg5DAMwghUMg1DcTA1jmH44DgIg+DwL4DgWRZIgaCAiE0N45DQaAuDAMZEkaS5KgSTIJE0MZQDGUpUlaRZHluRRyGUYx0CAdxpGSBYJDGQwgHiCQzCIIBoGUaRnGgdIJnQeZyDCeRynYIgylcL5pmuWoFgeXY/nMdqKmWWYClukZODEOY5DilaLlmAQ",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyGgNAiD4PAvgOBYThaBoICIbQyDEIA4C0MoOC0MQuDULQzC4NxBiOI4NjAIAxDULg2DQNggDUSA4C4NBhi4IIxDCJQuDkMAzjILgyDUNxMDWMoNkgOIShSGYYgSGoJE0N4yDQaAuDAMZUhWWJXgWB5aDGXAxl6YJihOZIXDwchlGMdAgHmCYMCIIByHiCQynwaBlGkZxoHSCZTCAdxpGSBZ6oqfwiDOY50naZpZgqTwxDgdqBnCVoCliaIKDEOYyp2n5VmWAQA",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDcaAyGMLg2CAMIOC0Lg0hgMR2C0MxjDCGQ5hkNwtDELomigNYnC4ORDDEOIpg4MIUg6FQxhcMY4DAc4tDOLQ0C0MguDKQwuiCGYshqLQxiWLQ4iUSA1GOGYViKKIokKWouGyQQgkQORBDOM5kDeFpohcMoODIdg0iEIIUhqDp0DGEQiD4PAvgOBZ5GMaRyGMbBlCAYx4gkNwioUeYJg+ihygkMp4nqf6BoOeZ8gaCIKg2DxoDak57gQaJ+oCgqEoajaJCCkAipKi6qqGlanD6AQ",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[cfg(feature = "caravan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDkVg5GENAghQMAghcMAtDSGxIDaE4VhiIoahQNB2DgYQyCCKoZiKKgyGgMgiD4PAvgOBY0jeBoIgqKg5GgMxhDGDojkSQwxHYMpCkSLQxC2RxIjKNI2gQaI5lWB4JE0MoqDENx2DGS5Dk2T4OEgMQwHYLYSkeRZHk+MZim6Rh2DmM41jqNBjGkchjGwZQgHKCYyCAYx5gmDwioUeIJDidwvnufZ/D6AQA",
        categories = "transportation,travel,nature",
        tags = "trailer,tow,camping,campsite,mobile home,holiday,nomadic,wilderness,outdoors",
        contributors = "danielbayley,jguddas"
    ))]
    Caravan,
    #[cfg(feature = "card_sim")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDQdg0CIPg8C+A4FhSF4GgiCoPC4MQ3g0MhhiIIAwiaDoOC4NIeDUOA2GwMwuDgMg4CCMo0DgQYlieJwxCAMonDeM41FaQYkkCKI+C2IhIDaSINj2DpMkwVg0lCSopDKTB6hOFYahmBIbgkTY2g8aA4l6FpihQchlGMdAgHmCQxDAIggHIeJ0ncdxpGSBYJmkIBoGUaRnGgdKBneegimmFAvm6cA+gE",
        categories = "connectivity,communication,multimedia,devices",
        tags = "cellphone,smartphone,mobile,network,cellular,service,provider,signal,coverage,disk,data,format,storage,flash,digital,contacts,phone book,contractual,circuit board,chip",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CardSim,
    #[cfg(feature = "carrot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4Mg3CAMgxC4NxzDkLg4DcLQzC4NQgDGDA3DMLQ2C4Mw2GENIcCCKYdDAIIuDCI4lDaMgzDcQw1hODwxhIOA0hCDYPhGE5Ag6EISDcehNDgLg2j8MQ0GwLYMDANZTC4MA0E0MY5DOTw1lKDA0jSYg2CIPg8C+A4Fmia4GgiCgyDIIA5HMLYSDOIgyhqHJTEMMYkDgNggjuHQ5h6hhzngM4QCCG4dDIU5ynSEJzDkepnmmbptgSb4JlukJ2nOi5To6HBTlylKpnWe49j+e6Pn+D4bnmiKNqkMqYmiaqdD6AQ",
        categories = "food-beverage",
        tags = "vegetable,food,eat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Carrot,
    #[cfg(feature = "case_lower")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5HYNwiD4PAvgOBYThaBoIgoMQ0CANh2gyEoUhmExjGkchjGwZQgGMeYJDEMguDUIotHiMA3jONRygkM46hML4nimK4miiKosjwIo+jSNoJDaOoti8Ioxj+FJCkcPoBA",
        categories = "text,development",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,vichotech,karsa-mistmere"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMQ2CANAuDAMw5C0OQuDYORhC4NYcCAMIfg0IAuDkMgzh8TAxDGDQ2CIPg8C+A4Fi+MoGggIhNDKDA5HYN4ujCNY0gSNoJE0MwuDMMA0g0MxoDaSIlj+MZDi8YxpHIYxsGUIBjHiCQxDiHAiCAcoJkcNZjGMeZfDKYovC+VpYloPoBA",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,vichotech,karsa-mistmere"
    ))]
    CaseSensitive,
    #[cfg(feature = "case_upper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDEaA0C4NRhDGDggDCGIXhkNRoC0NBhhOE4ahkMQthOJw1HYLQ5iENYjhmJQgiiExoDOFYbhqFocCIPg8C+A4Fj6QYGggIhtDKDg2CCEgwDMOYsC4NotiKDYxhcLg5DIM4YEyD5Kj2P5EkOBJFgkTQzC4MwwDSDgzGgNpqlqYZAmUPoB",
        categories = "text,development",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,jguddas,vichotech,karsa-mistmere"
    ))]
    CaseUpper,
    #[cfg(feature = "cassette_tape")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAch4g4MoSHmDg0hKFQihcIB3GkZB0GiFgwCIPg8C+A4FikYxpHIYxsGWE4WhIY4Zg+JwgGOHQ4iiKovjGM4pHAYYkCAZIOE0OAgDEMhoj+KQvkaJIujCMo0j2DoQhIco2jyOQxieU5ClmRZHGiSYOG0NggDIMAgC4NwtDILg5EEMQuDSTp7CCcZxDEIA4C6goQGgNwuDgYZ6nyjZ/pChp+DEbJzCAM5AlSaQ+gEA",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = "danielbayley"
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBWDYYYMgwMAghQMQgDILQyGgMYQhKFYgheEh2DEMoRhiIIWhqGBoC0NgiD4PAvgOBYxjSBoIgqDIlGEOQgj6FIWg2DYwjKN42gSOIJgsIIdGENQglCQZNCANJVkWM5JjEbBpG4ZQgHgMoJDILgwDEIggHmYgiDIMJoHkMZjm6YJxmyWJcl4PoBA",
        categories = "devices,connectivity",
        tags = "chromecast,airplay,screen",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Cast,
    #[cfg(feature = "castle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA1FYMwiD4PAvgOBYThaBoIgoMQ0g6EIShSGYYgSGoJgsNQgDIMR2C0MxhDMIIxg2NAtDYIAwHaEYThWJYkgWB4nDEOIyHYOIhj2F4CiWQYckQNRIDaSIjkuQIbE0MgyCAMQxEgMpTj6VYmgqWQgDkdoMGGWpajSWwtmsSA0mqKo4nUMZvm8Vg5mCSoZk0TY3DORp8GgPoBA",
        categories = "buildings,gaming,navigation",
        tags = "fortress,stronghold,palace,chateau,building",
        contributors = "karsa-mistmere"
    ))]
    Castle,
    #[cfg(feature = "cat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GMLg2DcIAwCAMQuDMNQuDAOQgg0LgyDaFguDcOAtg2GgwDOJwuDgNAgDYLg0DKLIhiMNAuDWJoyg0NwtjwIITjmE4XDCRIeiCLwxCAM4yDQQwyksMQ3C6HQxjEOYTlGFoNlEcwtDkLYrmGVA1DYYwwC2FwyhoNYskqYpOhSFJqi2YYxjOP5ukSGJhjkOAgjgNwyiAM4wjmSAyDMQQ5huL6NDCL4VhWUooFoIg+DwL4DgWmacgaCIKoAMQ0HaOaYpqn6egSoIJguIqkqYNaopurKrgWB6uDGaw1haMZsGiFw1EyDIWDcbI/DebokDWl6ZrWnYBA",
        categories = "animals",
        tags = "animal,pet,kitten,feline",
        contributors = "kemie"
    ))]
    Cat,
    #[cfg(feature = "cctv_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILgzDAOQgDYLg2DUMggDQLg3DkNwgg0NAwDEYQxCCJAwCCJ4kC4NA0h0MYODQMRsC0Lg1iGJQuDCIYUDANY8DUaINDcMg1iOJYokiKg4DkNI4iwN4zhoMpNi+LA4CIPg8C+A4FlmXIGgiCgxj4MQ2DaJZkmaNA3DGEZVDMOZGiaSQti8Mw0h+LRMDOFIkgyDhhg0OYXoKF4nimdYOC2fA5DGe4UhGPg2oELoRoKSIpCALg4DcM6JDAMw3liWpfl6BJggkTYXm0aJ8Dek6EpiSabomMpVjELaCDAMqjlup6mgWB4JG2sQyiexq9qWAqnsIIqqh4MR2C0NLJr+y7BmETYdDkaI5DG1ZdgEA",
        categories = "security,devices,communication,connectivity,photography",
        tags = "camera,surveillance,recording,film,videotape,crime,watching",
        contributors = "danielbayley,karsa-mistmere,rrod497"
    ))]
    CctvOff,
    #[cfg(feature = "cctv")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLg3DUIAxDIaAzC4NgzDIYQxhEIAwh2HAuDgOQ0hELg0DQNxsC0MguDAM4kDSLQ2DmGoch6HgxC0MYODAOAuDGL4qiyEg0isLg5DcIg+DwL4DgWS5OgaCIKDEN4/DANggDmLQ1DONYbjeIInDeJYvDGKoVDGWAgg0MgxDGX4fjiOgui+LJjEyFQ2huEp1GGLA5DIIKAoKYY5jsMwthUOQxnmFg5CANYWn+R6CoScocosMp0DMepKkyUZQgSUoJE2ggxDmFIODaf6DpiOIhnSjaQDENafk2o6igWB6lpYMR2C0NK3qGAqjryCpkqmLQxsOuYBA",
        categories = "security,devices,communication,connectivity,photography",
        tags = "camera,surveillance,recording,film,videotape,crime,watching",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cctv,
    #[cfg(feature = "chart_area")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoNwgDEMQuDIMA3GELg1jKF41DEIAuDENA2C0LgzDUMxsDILQyjGMw1jWFo3C4NwwDiFxsDOLQ5hOUgylSRo0hiNZMk6UA0lOQ5gleRYylqNo4DgNZgj8MxWhAYY3jeWwxC2chIDicYqkmKp2nYeoeiCJQ+gEA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,area",
        contributors = "nstokoe"
    ))]
    ChartArea,
    #[cfg(feature = "chart_bar_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfHIZRjHQIBoGUaRnGgdIJDQIggHcaRkgWCQ5jIch4gkMYyHmPQzjKPAiDeHogiWJ4kiaKJACINYyjSNhoj0Mo6kSPggkSRopiuLYvCKMYfC+SR0D6AQ",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartBarBig,
    #[cfg(feature = "chart_bar_decreasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoNwgDEMRoDiHogiWJIEiaCRNimEBoDOL4hjOMoFgeNYpDaHAyjuMYB",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartBarDecreasing,
    #[cfg(feature = "chart_bar_increasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoNwgDEMRoDiHogiWJIEiaCRNimEIcDKL4hjOMoFgeNYpDYaAzjuMYB",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere"
    ))]
    ChartBarIncreasing,
    #[cfg(feature = "chart_bar_stacked")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDMdg0CIPg8C+A4FhSF4GgiCgxDUIA1hGE4VhqGYEhuCRNDMIIQDENhhDIIIxDAII0jSMQyGiLojhaJ4UHIZRjHQIB5gmDwiCAaBlGkZxoHSCYSCAdxpGSBYJDmSByHiRpIlsIg3jyQJCj+QZDkUIg1lmXgxkiSpMk6UJdgmYJSlSVgiDEMphmUPoBA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,multivariate,categorical,comparison",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartBarStacked,
    #[cfg(feature = "chart_bar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoNwghAaA4h6IIliSBImgkTYpDEMYcDKLohjKMYFgeNIpDYaAzjuMIB",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartBar,
    #[cfg(feature = "chart_candlestick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CANR2DQIg+DwL4DgWEhyGUYx0CAdxpGSBYJhAIBoGUaRnGgdIJDYIggHiCQ3iweYJDmLByi4IgxhGE4YhqEoVgaCIKgwMYODKOoUgQaI+kmB4JE0MQ3CAMx2kaEpIhYPI8huMgiDWLIkiaKIJDiLIdh8aIhiyN5EjWa5HlqS4Fk2CpQCAMZTDOR4/nGQJODOUh2DENhhDIIKFDAIKIoihQyGigp6kkPoBA",
        categories = "charts,finance",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,stocks,exchange,hedge fund,statistics,analytics,diagram,graph",
        contributors = "danielbayley"
    ))]
    ChartCandlestick,
    #[cfg(feature = "chart_column_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfHIZRjHQIB4gkMQ1CIIBoGUaRnGgdIrDKLh5gmLQgHcaRkgWCQ0i4coqCIMYeiCJYniSJoojCMo0gkOZDkWRwgjkIg4i6RQ3i6PY/GiQZIC+Sh0D6AQ",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartColumnBig,
    #[cfg(feature = "chart_column_decreasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDcVg5CIPg8C+A4FhSF4GgiCgxDiDg3HYLQzhOFYahmBIbgkTYNDMdgxDYYQyCCMwwCCNo2jMMhojCJYWimKIFgeK4fg8Vg1j6J4BA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartColumnDecreasing,
    #[cfg(feature = "chart_column_increasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDcVg5CIPg8C+A4FhSF4GgiCgxDiDoQDWE4VhqGYEhuCRNg0Mx2DENhhDIIIxDAII0jSMQyGiLojhaJ4mgWB4ph+Dx2C0M48iWAQ",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumnIncreasing,
    #[cfg(feature = "chart_column_stacked")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDMSA3CIPg8C+A4FhSF4GgiCgxDkIA5GgLQ0hOFYahmBIbgkTQzCAMx2DENhhDIII0DAII3jeNAyGiMYlhaKYUHIZRjHQIB4gkMQ1CIIB3GkZIFgmJAgHmCZLCAaBlGkZxoHSSQykwcpICIMY/kORZCkSRpVCIOJMmOEpNk+UQilOYpJkyWZbl2CQ5maag+gEA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,multivariate,categorical,comparison",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumnStacked,
    #[cfg(feature = "chart_column")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoMQ4CAMQ3FYOYeiCJYkgSJoJE0MYMiwVg1jCIY0jOBYHjaKosHYLQzj2MoBA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumn,
    #[cfg(feature = "chart_gantt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA2GgOAiD4PAvgOBYThaBoIgoMQyCAMYPDaEoUhmGIEhqCRNDMIAzHaIBhh6HoNjMIIxGiIIjhWJ4mgWB4pDiHwxGgN45iWAQ",
        categories = "charts",
        tags = "diagram,graph,timeline,planning",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    ChartGantt,
    #[cfg(feature = "chart_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoICIbQxDkIA5C0NQgDULQ0jILYMDOHogiUPoB",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere"
    ))]
    ChartLine,
    #[cfg(feature = "chart_network")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLgxDEIA3C4Ng2DQIAxC4Nw4CAMoTDcMgiD4PAvgOBYiiWBoIgoMQ0g4NgyheHYaDgLYNDMMoWhgNI4iGI4oieBIpgkbQyDAIA0C0NguDANoXC4NQxDWPYkkGQIFgeCRNDMIAzHYMQ2GGMIwkaZIchwaJflOPw8GMaRyGMbBlCAYx4gkMYgnMeYJDYIggHKCYgiIL5tm+cYioScJyn8Ip4GOeginefZ0nafKCoihpsm6iZ5naUp+oCkp1CIOZTpcZQ+gEA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,topology,cluster,web,nodes,connections,edges",
        contributors = "karsa-mistmere"
    ))]
    ChartNetwork,
    #[cfg(feature = "chart_no_axes_column_decreasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxFYMwiD4PAvgOBYThaBoIgoMQyg2Dw5hKFIZhiBIagkTQxDmHx2C0NoihWJg+gE",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartNoAxesColumnDecreasing,
    #[cfg(feature = "chart_no_axes_column_increasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxHYLQ2CIPg8C+A4FhSF4GgiCgxDKDQxFYOYThWGoZgSG4JE0MQ5iAVgziSFooD6AQA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    ChartNoAxesColumnIncreasing,
    #[cfg(feature = "chart_no_axes_column")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxHYLQ2CIPg8C+A4FhSF4GgiCgxDKDQxFYM4ThWGoZgSG4JE0MQ5iAVg5iSFooD6AQA",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    ChartNoAxesColumn,
    #[cfg(feature = "chart_no_axes_combined")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYdg1CIPg8C+A4FhSF4GgiCoPg4NB2DeE4VhqGYEhuCRNDIMIODAdgxDGI4WieJoFgeCRtDKDQzC0OAuDYNA2CCPpADYYQuDWSAgiyLAxC0Lg3DAOJLEwOQuDMNQ0kOV5ZkeSQ1kuYQwk+UQ3lSDQxhKFIzhiAonjeCpaDEOB2DOMolm6NocE2UwxiCIpriWAQ",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere"
    ))]
    ChartNoAxesCombined,
    #[cfg(feature = "chart_no_axes_gantt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CANRoDEMgiD4PAvgOBYThaBoIgoNAghCDwwhKFIZhiBIagkTYQh4ORoDiIoViYPoBA",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartNoAxesGantt,
    #[cfg(feature = "chart_pie")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYwuDUNQyCAMIOC4MAwDULQuDQNA5C4OYciIOQ4GEMYXimFosDALQ4iINQzi+MQxGOHYTh2G4kDmJguDOJodj2MIjHaJ4Ng2F5Kg6Dh6CIPg8C+A4FlCU4GgiCoMC6DIODULg4DkQYriuKpMDgIAyl8M5PlGVg+gE",
        categories = "charts,files",
        tags = "statistics,analytics,diagram,presentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ChartPie,
    #[cfg(feature = "chart_scatter")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGYaRsGwPQiGMdRyHIZRuHQQxvGwbxyCIIByg4Lg1hwYx5g4N4giIeIlicPg8C+A4FgeLIvgaCIjg4NYnh2H4hgmC4Ng+EYThWF4ZhsIBjikIgxDiK4tjOMYCgSNI9gyDoQhKFIWhiGoch4Io5kiDgxDGYIkkqZIhiyLpSlCT4IgqVZAliQ5bkaNpKDaOZemCSYmmmTpsGWMqBkeZgxDSeo7iiYp+hycI/leQpakUIpqm6LBwGEdBoCAZIOE0MwgDMdgxDYYQyCCqAwCCq6rqgMhoqWlYtpmmw+gE",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ChartScatter,
    #[cfg(feature = "chart_spline")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DENhhDIIITDAIIWhaEwyGiEAiD4PAvgOBYfiKBoIgoNwghAYwuDULYTDGLQtikNIzhSF43gwNINhSLY4DSMg1CCLg3h6IIlD6AQ",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,curve,continuous,smooth,polynomial,quadratic,function,interpolation",
        contributors = "karsa-mistmere"
    ))]
    ChartSpline,
    #[cfg(feature = "check_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2CANwgDENxsC0NYWCIPg8C+A4FhqHYGggIhtDIMoSDALQ3C4NYQisTAxDOEg2hmG4gD6AQ",
        categories = "notifications",
        tags = "done,received,double,todo,tick,complete,task",
        contributors = "ericfennis"
    ))]
    CheckCheck,
    #[cfg(feature = "check_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA0EwOQgDENQiD4PAvgOBYWhmBoIgoMgxhIORMDOIoVheHIbgSHYJE2EYTEwNISDCJ4YisPoBA",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,oosawy"
    ))]
    CheckLine,
    #[cfg(feature = "check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA2CAOQgDENxsC0NYWCIPg8C+A4FD6AQ",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis"
    ))]
    Check,
    #[cfg(feature = "chef_hat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAyDEYQxCCEgwCCFYVDELQxHYLQ1C4Mw1GMMAtC4NA1DeHwxDaJA4DQNAuDcMg3hoLgwDSEA0CCOYXhYLQyC4MQzDQLYoDUOA5CANZJhaTIjDmQA4DaTI5juTY+kCQggkUOA4GOJQxDGQA5DiMIyC6RoejGDZhjYMRWDIMIRhOTZMhIMRaCIPg8C+A4FnqfYGgiCpSgwaAxDKeZ7oAPoBA",
        categories = "food-beverage",
        tags = "cooking,food,kitchen,restaurant",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    ChefHat,
    #[cfg(feature = "cherry")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ3GENQghEMAghSFAxhYYwwC0MguDcNocC4NQtiOIwziGIwyiQIAuDINIrDMWgiD4PAvgOBY0jeBoIgoMYMg6EIShWQ4XhmG4dh+KIkkuJ4dimK4ti+EYxjONY6jmBI7gkTQ3g0NBjDOLYqh0OQxCANItDkLQ4h6EYjj6DQuDYNggh0Mw4mcLg5DQIA5kKPpVjaWZYgWB5bDKDA5GMLZoDKfobDcLgxi+donhgLZdDWHpmkWeQ2l2GAgDeMo0oKOIB",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Cherry,
    #[cfg(feature = "chess_bishop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgwGEMoNCAMITCAMYNC0MhoDGD4RhGFIUheHh2DEYYXheIIWC2JxIDaJoWhWIYrisegiD4PAvgOBY3jqBoIgoMYMDEOBjDELg1C0Lg2kEIAzhkLg0kuTQtDQLg5DIMxDkMIA4C4Nw2DmFpVgyVZRhEMYeCAOZHCCZQ2hENpcl4NwgnEMQzC4MA3nSdg1kcM5hDebAxoIMw4gyYZDjaOI9jyBI+gkbQxnEN5PgwMpHouOaPo6BYHgkTZhhoNqao2AQ",
        categories = "gaming,emoji",
        tags = "mitre,miter,piece,board game,religion",
        contributors = "karsa-mistmere"
    ))]
    ChessBishop,
    #[cfg(feature = "chess_king")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGEMoNCAMITCAMYNC0MhoDEMoQhKFIUheEQyHYMRhheF4ghYLYoEgNYnhaFYhiyLB6CIPg8C+A4FjiO4GggIhtDYLg3hYOI0DEQw0C4Mw1hYNQuDYOIRDOFpLDAOQglWHBhk6ToqheSw5DULQ1GMMQuDUOIMhQMpEC4NA1k6V5OmgOAxDgQwxDOTJODecJylacA2DYIJFDGQwwn4QZejKMQyigMhjm0LgwlOLJ9lmfA2DcM4ZkQIA1GyLIWjeOY+j2BI/gkTQxhQNBoDSpo6qqqYFgerIcg0dpDncOKzqiAQ",
        categories = "gaming,emoji",
        tags = "ruler,crown,piece,board game,stalemate",
        contributors = "karsa-mistmere"
    ))]
    ChessKing,
    #[cfg(feature = "chess_knight")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgwGEMoNCAMITCAMYNC0MhoDGD4RhGFIUheHh2DEYYXheIIWC2JxIDaJoWhWIYrisegiD4PAvgOBY3jqBoIgoMQ2C6DAxDgYwxhmDZDC2DAyksORhDcIJSikMAtDeV4tC4NgzDWL4ojELQuDcNg4haWw0EyUg1GyGQuDOEQ1C4OAwDKEISlUIAuDmTZDDINhsk4OJSDELg0DUNo2jiPY8gSPoJG0MYMkShgyDWK6VDWio5o6jYFgekAxlKZqFDUM6Yqam6MgKjqggoOZjDEM4Wk6RYMoQOKqp2AQ",
        categories = "gaming,emoji",
        tags = "piece,horse,board game",
        contributors = "karsa-mistmere"
    ))]
    ChessKnight,
    #[cfg(feature = "chess_pawn")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgwGEMoNCAMITCAMYNC0MhoDGD4RhGFIUheHh2DEYYXheIIWC2JxIDaJoWhWIYrisegiD4PAvgOBY3jqBoICIbQxDQLoMhyFpECAOI2jiPY8gSPoJE0N4WDCGwwkuOZPk6BYHgkbQ4haYAxkQLZKjeWY7DwYxpHIYxsGUIBjHmCQ2CIIBygkNJ2GMeIJDEMpYmubZvD6AQ",
        categories = "gaming,emoji",
        tags = "piece,board game",
        contributors = "karsa-mistmere"
    ))]
    ChessPawn,
    #[cfg(feature = "chess_queen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGEMoNCAMITCAMYNC0MhoDEMoQhKFIUheEQyHYMRhheF4ghYLYoEgNYnhaFYhiyLB6CIPg8C+A4FjiO4GggIhthwLg0DeDA1C4OQ0DOFguDUNg3CCSAzDSMIpjKTQ3lMMg4GwMguDYMQ2C0M5EDAMo3jmPo9gSP4JG2DggDmZJymmOptmyBYHm+SA1koIA4C6Dg5g2YAxDUIJlDQMAzlaWIhC6WgtC4MwyDkbAxk6UAtlMNJ2muAptnuCpRDEOAggwOafngPBjGkchjGwZQgHKCZoCAYx4gmHAirgeYJp6OAvq6sKyjixKxrOtQireua2DCvRjr8Ig3nayLGq2r7Jr6CbVriugip6tK2ta2rGgE",
        categories = "gaming,emoji",
        tags = "ruler,crown,piece,board game,stalemate",
        contributors = "karsa-mistmere"
    ))]
    ChessQueen,
    #[cfg(feature = "chess_rook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgwGEMoNCAMITCAMYNC0MhoDGD4RhGFIUheHh2DEYYXheIIWC2JxIDaJoWhWIYrisegiD4PAvgOBY3jqBoIgqHINHYMo2jiPY8gSPoJE0MQ0kKRI3jmSZIgWB4JG0MQ3hYOIzDmRZSjuApJlaCg2kINYQhKKYUh4aA4mmH4xhgMhWlCRpTmKVY/E2Zg0hudpgGiVJKCIbZaDEOIql6UZHgEA",
        categories = "gaming,emoji",
        tags = "castle,piece,board game",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ChessRook,
    #[cfg(feature = "chevron_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAOQggyEAtDYIg+DwL4DgUPoBA",
        categories = "arrows,gaming",
        tags = "backwards,reverse,slow,dropdown",
        contributors = "colebemis"
    ))]
    ChevronDown,
    #[cfg(feature = "chevron_first")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDgLQ2hEIISDYIg+DwL4DgWGIbgaCAiE2DQ2HYMQyheGYeD6AQ",
        categories = "arrows,multimedia",
        tags = "previous,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronFirst,
    #[cfg(feature = "chevron_last")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQ4CANgthGEwiD4PAvgOBYWhmBoICITQxgwNh2DEMoVheHA+gE",
        categories = "arrows,multimedia",
        tags = "skip,next,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronLast,
    #[cfg(feature = "chevron_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDgLQ2hEIISDYIg+DwL4DgUPoBA",
        categories = "arrows",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "colebemis"
    ))]
    ChevronLeft,
    #[cfg(feature = "chevron_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQ4CANgthGEwiD4PAvgOBQ+gE",
        categories = "arrows,math,development",
        tags = "forward,next,more than,greater,menu,code,coding,command line,terminal,prompt,shell,>",
        contributors = "colebemis"
    ))]
    ChevronRight,
    #[cfg(feature = "chevron_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDULQ2hGEQgDYIg+DwL4DgUPoBA",
        categories = "arrows,math,gaming",
        tags = "caret,keyboard,mac,control,ctrl,superscript,exponential,power,ahead,fast,^,dropdown",
        contributors = "colebemis"
    ))]
    ChevronUp,
    #[cfg(feature = "chevrons_down_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMgwCANQtDWEIQCIPg8C+A4FheGoGgiCoMDSFIThENYWhiHQ+gE",
        categories = "arrows",
        tags = "collapse,fold,vertical",
        contributors = "PeterlitsZo,mittalyashu,ericfennis"
    ))]
    ChevronsDownUp,
    #[cfg(feature = "chevrons_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CANggDWD4PC0NQiD4PAvgOBYWhmBoIgqDAxDOEYQDWE4VheHA+gE",
        categories = "arrows,gaming",
        tags = "backwards,reverse,slower",
        contributors = "colebemis"
    ))]
    ChevronsDown,
    #[cfg(feature = "chevrons_left_right_ellipsis")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMGgLgwDEIg+DwL4DgWFYYgaCIKDENoODKEIShSFobhqBIcgkbQxDcIIuDUIA1C2MQ1iWF4piiBYHiuLg3jSMpBjaFY4hmAopjyCg4iGI4TkSJ4BA",
        categories = "communication,devices,multimedia,gaming",
        tags = "internet,network,connection,cable,lan,port,router,switch,hub,modem,web,online,networking,communication,socket,plug,slot,controller,connector,interface,console,signal,data,input,output",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ChevronsLeftRightEllipsis,
    #[cfg(feature = "chevrons_left_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CANwtDUIIQhIIg+DwL4DgWFYYgaCIKDGEA3hGEYPhGFIWhsPoBA",
        categories = "arrows",
        tags = "expand,horizontal,unfold",
        contributors = "karsa-mistmere"
    ))]
    ChevronsLeftRight,
    #[cfg(feature = "chevrons_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDcLQ1hEIISDUIg+DwL4DgWGIbgaCIKDEOIOhCFYUhGF4Zh4PoBA",
        categories = "arrows,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsLeft,
    #[cfg(feature = "chevrons_right_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDAIAxDcLQ1hEIISDUIg+DwL4DgWGIbgaCIKDSDg3hSEYmhaGIagQaA+gEA",
        categories = "arrows",
        tags = "collapse,fold,horizontal",
        contributors = "karsa-mistmere"
    ))]
    ChevronsRightLeft,
    #[cfg(feature = "chevrons_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ3CANQthGEwiD4PAvgOBYWhmBoIgoMQzg2D4ThKEoVheHA+gE",
        categories = "arrows,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsRight,
    #[cfg(feature = "chevrons_up_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQ1CCD4RC0NQiD4PAvgOBYWhmBoIgqDA5hCE4QhCFYXhwPoBA",
        categories = "arrows",
        tags = "expand,unfold,vertical",
        contributors = "mittalyashu,ericfennis"
    ))]
    ChevronsUpDown,
    #[cfg(feature = "chevrons_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDELQ1hGEQgDUIg+DwL4DgWGIbgaCIKgyDg4hOEg1hWF4Zh4PoBA",
        categories = "arrows,gaming",
        tags = "forward,ahead,faster,speed,boost",
        contributors = "colebemis"
    ))]
    ChevronsUp,
    #[cfg(feature = "church")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5GgNAiD4PAvgOBYThaBoIgoMQyCANx2DWEoUhmGIEhqCYLDQIAyDEdgtDMYYeh6DY1C2KwwHYM4jhWJ4mgWB4JG0MQ4g4IAzC4NYzC4MQ0DcYQxCCUY1lIIAuDQOAuDgNQ0FYMQ5jKLAglQMQtjMSA0mGNJjlKZpmi8NguDmc5QlWZJWlgLZalwTA2g6PIlgKJ5BgqfotFaT5RlObJRC4Mw3Daeg3DgMhsDWMJynSipsg2iguDINA5C4MAwDGlggDQQabneRIfHaTaAj6AQ",
        categories = "buildings,navigation",
        tags = "temple,building",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Church,
    #[cfg(feature = "cigarette_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMEgMxhDGDggDCFYVC2EwxHYMoShSFoghQMRoDEMwiD4PAvgOBYoiuBoIgoMQ4CAOBjDALQyC4NY4jiOo8DWJ4pi6LYEi+CRtg2SYWDIMJBiqRZEgWB4JE0Moah2GoXhaWYbliH4XDELY6C4OA2DaTpDgKRZTgoMoNjWN45jsMo9nMLZAiiT4smqUowE0N4ODIdg0miUIBA",
        categories = "travel,transportation,medical",
        tags = "smoking,no-smoking",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CigaretteOff,
    #[cfg(feature = "cigarette")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDISAzGEMYOCAMIWhYLYUDEdgyhOFYXiGFQxGgMQ0CIPg8C+A4FimLIGgiCgxDgIA4GMMAtDILg1jmOY7j0NYoiqL4ugSMIJE0MobDaH4UiKFwxhodo5k2GIhhqGpCiuRpFgWB5IDIMo1jePo8DKZZAlqRICkaX4Kg2Dx2ieKZbi2AQ",
        categories = "travel,transportation,medical",
        tags = "smoking",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Cigarette,
    #[cfg(feature = "circle_alert")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhsGkboIHgMoTg8IB4DGJ4QHmKwiDiLYmg2D4aC+IYjiCIoki+FIpjODguDAMYtj0NoyhOR42jgZQ+gE",
        categories = "notifications",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    CircleAlert,
    #[cfg(feature = "circle_arrow_down")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDgdg4hmG4hiOIIiiSJgiG0OAgikNAgjsNAtDSLgvjAaA+gEA",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowDown,
    #[cfg(feature = "circle_arrow_left")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbYOCAOAtDQIIujAIobC+IokiGI4licIhNDENggg4SA4jKHI1GgPoB",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,sign,turn,button,<-",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowLeft,
    #[cfg(feature = "circle_arrow_out_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGEMQwg2EoSDGDYThMIg+DwL4DgWG4egaCAiG2DAygyEQthGGociGIIEiKCRNDgIInEgMh2C0Nosh2MA+gE",
        categories = "arrows",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutDownLeft,
    #[cfg(feature = "circle_arrow_out_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYQxDAIISCCEwxhSFAwC2EgiD4PAvgOBYfiKBoIgqD4Og2DIUDKHogiWJIEiaCRNikMQ2HYNhoC0NoviGMw+gE",
        categories = "arrows",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutDownRight,
    #[cfg(feature = "circle_arrow_out_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBWDIaA2CIPg8C+A4FhSF4GggIhtgyDAxDAIIhhOFYahmBIbgkTQxh8QYhiOIoiDGIwgiAMolhaKQ+gEA",
        categories = "arrows,development",
        tags = "outwards,direction,north-west,diagonal,keyboard,button,escape",
        contributors = "danielbayley"
    ))]
    CircleArrowOutUpLeft,
    #[cfg(feature = "circle_arrow_out_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIQQxDCDoThMMYOg6DQyCIPg8C+A4Fh2IIGgiCoMCCDYPhmHIeiOIoEiSCRNDENooGgNh2DaLIfjAPoBA",
        categories = "arrows",
        tags = "outwards,direction,north-east,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutUpRight,
    #[cfg(feature = "circle_arrow_right")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsbYUCAMQ2CANAti+MQihsL4iiSIYjiWJwiE0OIrDIaA4jOHI2GgPoB",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,sign,turn,button,->",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowRight,
    #[cfg(feature = "circle_arrow_up")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDYIIOC0NIui4IA0CKGwviKJIhiOJYnCITYOiwNhWDiNIcjcaA+gEA",
        categories = "arrows,gaming",
        tags = "forward,direction,north,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowUp,
    #[cfg(feature = "circle_check_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg4DAMQgDEMBBhOEgwCCGIRhsNwgDMLgzDMNQiD4PAvgOBYliiBoICIbQ5hKEQzh4TAyDIIA0iSJorD6AQ",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    CircleCheckBig,
    #[cfg(feature = "circle_check")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQ5CCDggDKLQgDQLQ0CKGwviKJA+gEA",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis"
    ))]
    CircleCheck,
    #[cfg(feature = "circle_chevron_down")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQxDYIINC0NAgDSLouCKGwviKJA+gEA",
        categories = "arrows",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    CircleChevronDown,
    #[cfg(feature = "circle_chevron_left")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ0CAMQ2C0NIuCCLw0hmG4hiMPoBA",
        categories = "arrows",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    CircleChevronLeft,
    #[cfg(feature = "circle_chevron_right")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsbYNCAOAgDSLQti4NAihsL4iiQPoBA",
        categories = "arrows",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    CircleChevronRight,
    #[cfg(feature = "circle_chevron_up")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQ4CAMQ0CANAti2MQihsL4iiQPoBA",
        categories = "arrows",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    CircleChevronUp,
    #[cfg(feature = "circle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgMhhgwIIWDAIIZg8MwuDiGgiD4PAvgOBYiiWBoIgoMYdDmEAxh4MQ4hWG4ZjaFwth2HwwiGI4oieBIpgmCw3C4Ngwi6HQ3DIMY0heN4bhCRouhEN49iSQZAgWB5DhGMgyheLZOhiGoXhqOYeleP4CkGXIKDKDQyDeLgxkWRw5mOUI4lWUg2DmapZmyW4qE2TIxh+DIOnmZZRhmOqAiagpCgqSpMCANguDMOZNhaZJRlULYRn6kBolqkxNpimoPnALpyninZ6DGoZTrOVoiliJoBA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = "danielbayley,jguddas"
    ))]
    CircleDashed,
    #[cfg(feature = "circle_divide")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bGwaRuggeQxhmEB4icIg4hAeQyigIB4jCDQ2CKGwviKJIhiOJYrDGNggi+E5BiqMYziiOI6GWPIkjKP4PjKNIOi6NItkKK4tkqPQ+gEA",
        categories = "math",
        tags = "calculate,math,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    CircleDivide,
    #[cfg(feature = "circle_dollar_sign")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDYIA4GgLQ2GEMggjIMAgDEII1jUNBoDSMYzjiNpBjoSA4CKGwviKJIhiOJYnCKKYyDEOBWDaRockkaA+gE",
        categories = "finance",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[cfg(feature = "circle_dot_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgYQ5C4OQzCCFYXCAMIcCCDwzC4OIcCIPg8C+A4FiaKYGgiCgxDcLg2CCIQ3DGFIWDWGY5h6HYPhENg5hALg3iWJ4siuBItgkTQyDGIgyh+DY3hqGJVj2H4eiEOJGiiSpJgWB5MDKDQykKMIyjgOY6hqOodj4LYRDeQ5Bl2SICkqYoviGQpOlCag0jsOaBm+HwtluJIml6Kp4mGLhNDYLqBmQLpmmqbI8oUMZxjIOackWip3iyepNhKH58mqVoWhimocoeIp2l+jZLgqNYPpENKXoKboej+RKcnWoayGMaRyGMbBlCAYx4gkMQyCIIBys20BjHmzbPoqxLGsgPoBA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = "danielbayley"
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bh6BoIheGYQhKDYWguDYahyIoggEA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ3CCDhoC4MAxhmG4hiOIIiiSJgiigMorDKLYvjEL4zGiNYjiWJ4qiyLowhqQY2D6AQ",
        categories = "layout,development",
        tags = "ellipsis,et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0NwgDEMBoiuGYbiGI4giKJImCKKIqDSLYYhoL4xGgPoBA",
        categories = "math",
        tags = "calculate,shape,=",
        contributors = "danielbayley"
    ))]
    CircleEqual,
    #[cfg(feature = "circle_fading_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMQwCCEQghKEgxCANwuDMOITDYLg3DUIg+DwL4DgWJIngaCAiG0MQ2hMMgtDSM4zCANIjiWKopgSK4JguDYvFYOI5iaPY8gWB4/DILg1CAOAuDiIYQheFoVhULZNCAM5FjuAo9kqCpMDgM4elSE5WlaTA0mUMwujiJJGiiX5JiwTQ0C4NgzjANQuDIMw1meFJWhiUQ5DGWQ4DUN5dkedI+gqUA2DQNIODGb4PhSg5XhKGp6oiG5EnGO4BA",
        categories = "arrows,development",
        tags = "north,up,upgrade,improve,circle,button",
        contributors = "jordan808,jguddas,colebemis,ericfennis,mosch"
    ))]
    CircleFadingArrowUp,
    #[cfg(feature = "circle_fading_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMQwCCEQghKEgxCANwuDMOITDYLg3DUIg+DwL4DgWJIngaCIKgwIA4HYOIjiWKopgSK4JgsNoTDISIyiSJo3jaBYHjkMguDWLwuDiIYQheFoVhULZICAM4zkGKICjeRYKkcOAzh6ToTlCUJHDSYAzC4NJXjWWpEiwTQ0C4NgzjsNQuDIMw1mKFJQhiSw5DGUw4DUN5skKbo4goOJzDQNIODGaoPhSfZRhKGp0oKG4/jSQoBA",
        categories = "communication,social",
        tags = "stories,social media,instagram,facebook,meta,snapchat,sharing,content",
        contributors = "jordan808,jguddas"
    ))]
    CircleFadingPlus,
    #[cfg(feature = "circle_gauge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg2CAMguDcYQxDAIIVCCFgxhkIINDeHYSCIPg8C+A4FiMYxpHIYxsGUIBygkMgiCAYx4gkMYyjQeY3jKIwvimK4tiOJoGgiCgxDMLg0hcMIOhcOYdiKJJED6AQ",
        categories = "transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "danielbayley"
    ))]
    CircleGauge,
    #[cfg(feature = "circle_minus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQ4CCDhoDgIobC+IokD6AQA",
        categories = "math",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    CircleMinus,
    #[cfg(feature = "circle_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNDgLgzDWDQuDYORBDGEIoCCEIpg0MYgCAMQ1iQNYThWGoZgSG4JE0MQ5C4MA4jGP5BieKYsjGSQ0C4OYMkuTY2haOg+gEA",
        categories = "shapes",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure",
        contributors = "danielbayley"
    ))]
    CircleOff,
    #[cfg(feature = "circle_parking_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg2DUNggDcSAxDMYQzCCGAwCCGwxCCDQ5DgNIZC4MwwDcIg+DwL4DgWKotgaCIKhUIIVEgOYpiuMIvgSMYJgsOQuieHgxkGQxBkSHJKh4NAuDmGJNk+OYsj2PIFgeCRtDKH4fhsMgwlOO4Cj2WIKDiJQ1DeH4ODgNxhDGHYbnKNY1iCEJ1k6EJhlWY5XjITQ5jUNxWjiKpUi6AQA",
        categories = "transportation,navigation",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    CircleParkingOff,
    #[cfg(feature = "circle_parking")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQ5CAMQ3FYNxoDQYQzCCMwwCCNgxjcIA2EgOQihsL4iiQPoBA",
        categories = "transportation,navigation",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleParking,
    #[cfg(feature = "circle_pause")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bGwaRuggeAyhODggHkMYTDWDx5icIg5g8eIsgyDobC+IokiGI4IjCC4zCCJoTDSL42DGLpDkiRo5jsZQ+gEA",
        categories = "multimedia",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis"
    ))]
    CirclePause,
    #[cfg(feature = "circle_percent")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ1CAOQtDYIA2hmG4hiOIIiiSJgiE0OYrGgLgwDGMQvjMaI1iOJYME2KQgimPY/kGQw+gEA",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    CirclePercent,
    #[cfg(feature = "circle_pile")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ5hAcoMg8Pg8C+A4FgeGodgaCIXCKD4RgwNYQgqE4ZhuIYfgKBIiieDYmiSJorg0NgihqHIyjCL4IjkMgwhaGIqhKDYVj2QYgj+I5HjSFIqgsIg0jyLpPk6HpQiWVIMDiSIsliPpcD6AQ",
        categories = "shapes",
        tags = "off,zero,record,shape,circle-pile,circle,pile,stack,layer,structure,form,group,collection,stock,inventory,materials,warehouse",
        contributors = "colebemis,nathan-de-pachtere"
    ))]
    CirclePile,
    #[cfg(feature = "circle_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAOQuDAMAzGEMQghQMAgheFAxC4NQxDcLQuDgNQ5GwNAuDkOQ3CAMonimE4VhiMYWhULg3DEOBsC2JooiqLI8EGGoxhmDYVjsOQ2HoIg+DwL4DgWSxjGkchjGwZQgGMeIJDEMgilceZalwIByloMJKkyUZTlUPoB",
        categories = "multimedia",
        tags = "music,start,run",
        contributors = "colebemis,karsa-mistmere"
    ))]
    CirclePlay,
    #[cfg(feature = "circle_plus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ4CCDhoDgIobC+IokiGI4licIhNg4IA4HaLowjIaA+gE",
        categories = "math,development,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    CirclePlus,
    #[cfg(feature = "circle_pound_sterling")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTYNCAMQ2FYOQuDUYQxisIAwjWNA1jUIobC+IokiGI4licIhNDiKwyGgNI7hyPhokCJImiiRosGgN5Lj2QQ+gE",
        categories = "finance",
        tags = "monetization,coin,penny,marketing,currency,money,payment,british,gbp,£",
        contributors = "karsa-mistmere,jguddas,danielbayley,LieOnLion"
    ))]
    CirclePoundSterling,
    #[cfg(feature = "circle_power")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANx2DQIobC+IokiGI4licIhNDcLg5DkOAgDkLgwDAMxhDUIJGDAIAxCCSQ4C2QQwDWL4cjIaA+gE",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley,jguddas"
    ))]
    CirclePower,
    #[cfg(feature = "circle_question_mark")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OQuDAOQgDkYQzCCMAwCCMwxCANQuDiMAxGOMwyC2MAzkCMYZhuIYjiCIokiYIhNg4IAxDcaIqDGRQvkcaA+gE",
        categories = "accessibility,text,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    CircleQuestionMark,
    #[cfg(feature = "circle_slash_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MgyCCKosg+GgviGIw+gE",
        categories = "shapes,math,development",
        tags = "diameter,zero,ø,nothing,null,void,ban,math,divide,division,half,split,/,average,avg,mean,median,normal",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bGwaRuggeAxgwOYQHmJ4NDWEB4DKE4uCAeYxCKKYbC+IokD6AQ",
        categories = "development,math",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[cfg(feature = "circle_small")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDYIggGMeILDEMoOGMeYShQPg8C+A4FgcPoBA",
        categories = "shapes,medical",
        tags = "shape,bullet,gender,genderless",
        contributors = "jamiemlaw"
    ))]
    CircleSmall,
    #[cfg(feature = "circle_star")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQxC4MA1DEIA3C4NgxDYYYui4MAgjiNguDkMA5isMg0GwLg3DMNwgioNA1DKNZIjmT44kSRguDUMw1GyKg2DMNAuDINY0jaUJOlQOA4kgLg4DANhsC2KgxDcMpnjMOJNjeUAtl0OAymgNprl0NQ5meMgznWYgxm2VA0jsMZbmySQ2DWeA3DWhY4paeA5g6OaOC4NKQkSlJhpaSKIlUOaIowM5Dl4OKIoOlZ3nkMp4DgNg3pwMZfqiS6wjoIJUnCiJ6DKWIxDMM54l6YJOqOUZFDeeJVDUeoZhuIYjD6AQ",
        categories = "sports,gaming",
        tags = "badge,medal,honour,decoration,order,pin,laurel,trophy,medallion,insignia,bronze,silver,gold",
        contributors = "karsa-mistmere"
    ))]
    CircleStar,
    #[cfg(feature = "circle_stop")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhyGUYx0CCEgiDmEILieEBoGUaRnGgdIMDaFYqDGEB3GkZB0GiM4ZhuIYjD6AQ",
        categories = "multimedia",
        tags = "media,music",
        contributors = "colebemis,ericfennis"
    ))]
    CircleStop,
    #[cfg(feature = "circle_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg5DINQgDIMAuDANQ2GENgghoMAgh0MAtDEMQuDgNYjDAMAxCIPg8C+A4FiwYxpHIYxsGUIBjHmCYiCKOB4jsMo9HKCQ0iuLYyjSNoxjONY3jmQI9GOPwiDGQQgkOVAwkYL5Ik0PoB",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUserRound,
    #[cfg(feature = "circle_user")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGodgaCBjhKDYPgmC4NhgIIWCIM4ZhuIYfDwcBhHQaAgGSDBNDcIAyDALg2DYMhWDEORhDKPggDCSwgDGPgtDIaA2kiSpMkyT5JDIdgxkGQ4wC+NY3D6AQA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUser,
    #[cfg(feature = "circle_x")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDUIA5C0NggDYIobC+IokiGI4licIhtDmLIwjCMocjUaA+gEA",
        categories = "math,development",
        tags = "cancel,close,delete,remove,times,clear,error,incorrect,wrong,mistake,failure,linter,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    CircleX,
    #[cfg(feature = "circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgcPoBA",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[cfg(feature = "circuit_board")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHIeIODKEodCIM4SHmDojD4PAvgOBYoHAYYXCAZIOE0MQxCAORoDQYQyCCPAwCCP4/DILQyFaJ4pi6F4oGMaRyGMbBlCAY4lCIOYbh6EhjiGVooC+TJOlCLYvGiMYzDePQxHYLY6jyPpACCNpDDKOQil2SRokuTZPlEcpYlKIQxDWWZUoGdYpl+ew+gE",
        categories = "science,development",
        tags = "computing,electricity,electronics",
        contributors = "danielbayley,jguddas"
    ))]
    CircuitBoard,
    #[cfg(feature = "citrus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg2DYIAxDeDg3GGDQwDiEQuhgIAwh2EQthsNIaDYQQxDKEYoh6HgxCANAuDcMwgDILgzDiFgui2DYtiuEYkDGIQwDQegiD4PAvgOBZGkmBoIgoMQ5g4NYRDWDolhmGY9i0OI1lOLwzDSRZHkyS4Ek2CRtDGIwxDALZVlOb5ikiZplgWB4JE2aoRhMOA1FabBIDaOQ1nKZIBA",
        categories = "food-beverage",
        tags = "lemon,orange,grapefruit,fruit",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Citrus,
    #[cfg(feature = "clapperboard")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILgyDkNggDMLg0DYNISC4MAyhgOQ1DYIg+DwL4DgWIYkgaCAiE0Mgwg4IIRDMIAxDEbAtC4OQtg0NBjjYMwtDELpAj6DYbkKOQuDUbAxhMNQtjuQAxj0IJEC6MYNDWMpVHqIIiieJoEiiCRNjGMxoDEOB2DgYYbhsMAgm6UZsEgNZrlOb53nGOZbiGI5gl+BYHgkbQ2kEOAgDWDg3jCQYYDgOQ5lyfYlgE",
        categories = "multimedia",
        tags = "movie,film,video,camera,cinema,cut,action,television,tv,show,entertainment",
        contributors = "it-is-not,ericfennis,danielbayley,torfmuer"
    ))]
    Clapperboard,
    #[cfg(feature = "clipboard_check")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgxCIIIJCIMoNHiCg4g0dxpGQdBohSDRoGUaRnGgdIKDSDRyhOCwiD4PAvgOBYrHAYYaCAZIKE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PIqiyMYajCMhojSChtDmVw0kaPw0C2JYrC+ZRoD6AQ",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDQdgyC4MhsDELoNDEIg+DwL4DgWG4egaCIKgwIA0GgMhhDIIIrDAIIuDGLIsHYLg4DMMoahyIYggSIoJE0OImEgNoqjKLpHC2KwyHaD5Fi2L5QkqKI5h2PYbGMaRyGMbBlCAYx4gmDAil4eZhDaYxygmZ4bC+WJalyGxyGUYx0CCYAiDiY5lCKOAgHcaRkgWCZ5CAcp3hkIBoGUaRnGgdIJDSVJynQPoBA",
        categories = "time,text",
        tags = "copy,paste,history,log,clock,time,watch,alarm,hour,minute,reminder,scheduled,deadline,pending,time tracking,timesheets,appointment,logbook",
        contributors = "beanduong,colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    ClipboardClock,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIB5g0MoQHIeINDGEIXg6FYSCKGQgGgZRpGcaB0g0NAiD4PAvgOBYrHAYYLCAZINE0OAgDQSA2GEMggj4MAgkEMAtj4Mh2DENI9j+QpNkGRhoDEMpLkCTo/kUdpFiqLIxguMIyGiNI2DENo5GiU5GlYMZMkeKYrC+XRol+M41CITQymuSRIDGGZvnGc5hnUbQxDUIAxkQNI5oqbpcmAPoB",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAdxpGQdBogkOIMHIeIJDGDIXCKFAgGgZRpGcaB0gkNIVggIoZD4PAvgOBYrHAYYRCAZIJE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PAiisL4xhGMIyGiNI2DGPwxDEaImmSZhomiM41CKN5uDacZjiydJ2mqeBNDiV5wC4MIqn+aaBmueaFjgaKIoqZaMgE",
        categories = "text",
        tags = "copy,paste,tasks",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardList,
    #[cfg(feature = "clipboard_minus")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAdxpGQdBogmCwgHKCAiDGDByHmCYZCAaBlGkZxoHSCQ0gyHAiDIIg+DwL4DgWLRwGGEQgGSCRNDENggDQaAyGEMggkEMAgkQMZCkIdgxDSQJIkSRgtkEMhIDaTZDkUIAxlGURWlWUpYkaQpRj6LIujOEYyjQaI2jgOZZj0NplC+ZxoD6AQ",
        categories = "text,medical",
        tags = "copy,delete,remove,erase,document,medical,report,doctor",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ClipboardMinus,
    #[cfg(feature = "clipboard_paste")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDQaAxDAIg+DwL4DgWFYYgaCIKDENgghAMhhDIIIlDAIIog2JQyHYMQuDMNA0hSFobhqBIcgkbQxDeDg4iELQ0kGQY0heOI3gWB4JE2Pw0EgNokiaKZTDALYsi4NJRieVJSDKEYjiyXIqC4Nw5DMLYvgwM5FjYPByGUYx0CAaBlGkZxoHSCYzCAeYJDIIggHcaRkgWCQ4oAeKGoAcqJCIMZFm+cQ+gE",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "xnousnow,ericfennis,jguddas"
    ))]
    ClipboardPaste,
    #[cfg(feature = "clipboard_pen_line")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAaBlGkZxoHSCQ0gweYJDKDByggIgxgwdxpGQdBogmCw+DwL4DgWJxwGGIwgGSCRNDgIA0EgNhhDIII6DAII9DALY6DIdgxDSOY7j6SY9kIaAxDKR48kqO5BHYLQuDUIonC+LYjiyLhojCMgxDaNRok+QpSDEIAxC4NwzmuWYolwaJei+MQijOaw4k2cZbl+dZgncTQymwMw3jSTguDYMo4mqao/j4LQzC4MAwDSkqUpYbAtDSlJqp0MJOlCUpAlcMA2C4OA1DSm6poaO6pDcYZXlepAgooMq4GyuQ4DeVg4oao6Qj2qarlYNanHqfZzD6AQ",
        categories = "text",
        tags = "paste",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardPenLine,
    #[cfg(feature = "clipboard_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA0GgMhhDIIITDAIIWDGFIUHYMgiD4PAvgOBYfiKBoIgoMgxC4Mw0CAMQ1C4Ng2DQYYZhmGIXC0MwuDAMA0jqPI+GwLYwDCGZFDGEYThWF45C4NQwDYLg4DUNJDlMMw3hSUw3GGT5Pk2FoWjEMpkGyZQ4DcLZYl2S5hk2U5VmuUA2HqHogiWJIEiaCRNDiFAyEgNoShqYouC0MqIFag5uoaGaJDKEJ3iGe4fHIZRjHQIB4gkOAiCAdxpGSBadp8aBlGkZxoHSCQ0p8eYJh0IBypwIgxpOl6ZD6AQ",
        categories = "text",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis,Spleefies"
    ))]
    ClipboardPen,
    #[cfg(feature = "clipboard_plus")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDQIggHcaRkHQaIODiER5g4MoRHKGwiDGHh4g6IggiQIoZD4PAvgOBYrHAYYWCAZIOE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PAiisL4xhaMIyGiNI2DmV47DaY4smYaJojONQijePwxDcdgtnCZJzD6AQ",
        categories = "text,medical",
        tags = "copy,paste,add,create,new,document,medical,report,doctor",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ClipboardPlus,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDSEByHmDQxhCHAiDKGh4h2EIkg4Ig+DwL4DgWKhwGGCwgGSDRNDENggDQaAyGEMggj4MAgkEMY/j8dgxDSPZFkGQwtj4MhIDaSpAkIIAxk6ThWlKT5VkOP5OjuKYrjCC4vjEaIzjUOZWDIdgtDEaA2keYgvmQaJmjKNAijaRAxDeYYqnWZ54mieo2j4MQxHYNp0nYPoB",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAch4gkMYMHKCAihEIIPCIOIMHcaRkHQaIJhoIBoGUaRnGgdIJDQIg+DwL4DgWLRwGGHwgGSCRNDENggDQaAyGEMggkEMAgkQMZCkIdgxDSQJIkSRgtkEMhIDaTZDkUIAxlGURWlWUpYkaQpRj6LIujOH4yjQaI2gkbQxDWWZajsNplC+ZxommNY3CIbQ5nEIJznWdw+gEA",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIB4g2DwgHmDQyhAcoSCIMYZhaHIQGgZRpGcaB0g0NAiD4PAvgOBYrHAYYLCAZINE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PIqiyMYLD6AQ",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbAyC0NAihsL4iiQPoBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsTYUCANh2DYbAtDQLYahyIokD6AQ",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbAtDILQ0CKGwviKJA+gEA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYIobC+IokD6AQA",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDYdg2GwNAtg+GgviGIw+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYaA0CKGwviKJA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock3,
    #[cfg(feature = "clock_4")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDYdg2GwNAgg+GgviGIw+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[cfg(feature = "clock_5")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbAyCANAihsL4iiQPoBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2hQIobC+IokD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbAtDIIA0CKGwviKJA+gEA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbAtDQIIahyIokD6AQ",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[cfg(feature = "clock_9")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDYdg2EgOIZhuIYjD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock9,
    #[cfg(feature = "clock_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNhsDQIAyCIPg8C+A4FheGoGgiCgyDAIIMHYNYWhiHYcgSHoJE2IYUDEaAuDAMYnhmK4qgWB4tDIMQuDINQgDiPxBDGIpGCCR5JiMNowC4MQ2jaKYB",
        categories = "time",
        tags = "time,watch,alarm,warning,wrong",
        contributors = "colebemis,jguddas,jamiemlaw"
    ))]
    ClockAlert,
    #[cfg(feature = "clock_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNhsg0MQiD4PAvgOBYWhmBoIgqDAuDMMw3CAMgxC4OQ5DQYQxDAIItCCLgxi8IA5C4NQ4DgLQ4C4Nw2DeFYXhyG4Eh2CRtDENIvDgIJKk4LQ0kGGJFkSBYHgmC5Mkkdg4lKQ4B",
        categories = "time",
        tags = "time,watch,alarm,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis"
    ))]
    ClockArrowDown,
    #[cfg(feature = "clock_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNhsDELg1DYLg3DgIg+DwL4DgWG4egaCIKDEMwuDIMg3CAMoTDkMg1GEMQwCCMggjMMY0CAOIXDYNwtDmFA4hmG4dgQaIgkaB4JG0MQ0jQOAgDQLZOlSGociGSIFkqJJQigdgtkOV5GD6AQA",
        categories = "time",
        tags = "time,watch,alarm,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis"
    ))]
    ClockArrowUp,
    #[cfg(feature = "clock_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNhsDQIAyCIPg8C+A4FheGoGgiCgyg2DBhDEMAgiUIImDGKQtDGKw5C4OQ1haGIdhyBIegkbYhicNgtDULg1CCQA1EwMYTDEOY0hmOA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis,jguddas,karsa-mistmere"
    ))]
    ClockCheck,
    #[cfg(feature = "clock_fading")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMQwCCEQghKEgxCANwuDMOITDYLg3DUIg+DwL4DgWJIngaCIKgwIA2HYNhsDSDojiWKopgSK4JE0MguDUIA4C4OIhhCF4WhWFQtj8IAzjaJo6jmBYHjyPg4DOHpGhOSJIj4NJYDMLg0k+OICjqVIKDQLg2DMNggDULgyDMNZahSSIYkMOQxksOA1DeZJRmaU4sE2Qg2DSNAyDGYoPhSdpJhKGpsnuGw4oCKIBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis,jguddas"
    ))]
    ClockFading,
    #[cfg(feature = "clock_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYNhsDMLg2DQNAgDELg4DIMgiD4PAvgOBYfiKBoIgoMQ2hgORoDaHogiWJIEiaCYLDmGIPi6H4hjOMoFgeNQyhkOYNDGEwyDYNxhDEMIYk2TQxCAMAtDiFA1DMIJVDaV4vjyI4B",
        categories = "time",
        tags = "time,watch,alarm,add,create,new",
        contributors = "karsa-mistmere,colebemis,gubser"
    ))]
    ClockPlus,
    #[cfg(feature = "clock")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCANh2DYbA0CCGociKJA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "closed_caption")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5C4MQ3GEMwghSDQxCCDYNDULg2DYIg+DwL4DgWIYkgaCIKhGDoQhKFIWCCGIaCCHIeiCIoniEchlGMdAgHcaRkgWCQyDAIggGgZRpGcaB0gkMQ0kceJEkceYJDWRxylMIgyjcL47j0PoBA",
        categories = "accessibility,multimedia",
        tags = "tv,movie,video,closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "ericfennis,UsamaKhan"
    ))]
    ClosedCaption,
    #[cfg(feature = "cloud_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMHYNAiD4PAvgOBYThaBoIgqDAgDIMBoC4MAxhKFIZhiBIagkTQ4C6DA4g4NguDkNA5EENwgjgMIOjwMQ1C4NwxCAOBoDGQA5GGQpCjuO5LCAORoC2Rg2DQMolhWKQ+gE",
        categories = "development",
        tags = "weather,danger,warning,alert,error,sync,network,exclamation",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,lscheibel,jguddas"
    ))]
    CloudAlert,
    #[cfg(feature = "cloud_backup")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDULgyDUMRBDQLg1CCFoYDAIIchwMQ3hcIA4GgLQxC4Nw5EENwgiyH4dCAM4ODMLg2DANwiD4PAvgOBY6j2BoIgqLAxDEdg0GgNI5juQI/gSQYJE0OIODkYYYhuMIcDkLYyhqGYih6HQthqY4ihYOAyl+aJZmKNA0g2J5vEyRA1kuPJPD6AQ",
        categories = "arrows,files",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,synchronize,synchronise,refresh,reconnect,transfer,data,security,upload,save,remote,safety",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    CloudBackup,
    #[cfg(feature = "cloud_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDULQ1C4NQghINRMDmDg4CIPg8C+A4Fh2IIGggIhNhYMQ2g4NguDANxBg2DQwg6NIPC4NwxCAOBoDGNw5GENITCCQYUjOM45DOEwwjkNwuDMMg3hyHojD6AQ",
        categories = "development",
        tags = "sync,network,success,done,completed,saved,persisted",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas,lscheibel"
    ))]
    CloudCheck,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg4DUMggDEOQuDcNwyC0LgzDgMwuDkMg0CIPg8C+A4FiOJoGgiCgxh0MQ0DiEg0C4MgyDiGochmHwziKJIpiiBIqgkTYtC6L4xhOFYXGEMwgk0MISCAMAtDKNA5DYLQ1C4NQ0DQbIZhsM46DKPIjiWQZAgWB4JguHQ1k0MoNDaV5gDiGIeiCTJOlKUQxlSVpYlqXIhmaP4CkGa4sjOFoRDGWoPlWO51mWPpooeaorgui4XhKN5HniHZhj2Z4npeQgiE2M6NloMRhDcIKvlAMZRhQOZihSDw4EGr6xnys6OhWsw4GgMYVDkYYzDUILJnysgglWyo3DKo6GimiRthSNZIo+EJjmKOKUqQaJpqe2I0janZGjC3rgtSaIB",
        categories = "development",
        tags = "computing,ai,cluster,network",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    CloudCog,
    #[cfg(feature = "cloud_download")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg4GwLQ0hQIg+DwL4DgWGIbgaCAiG2DAgDIMQghUNIXhmHodgSH4JE0NAuDMOQzg4NQuDINg5EENwgj4MIOkIMY4DeJg4GgMQuDcORhjINYnC6UJBkGJgyC4NAzDYIA4jkOIphiGouD6AQA",
        categories = "arrows,files",
        tags = "import",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDownload,
    #[cfg(feature = "cloud_drizzle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCg4g0OR2DGN45j6PYEj+CRNkKDpFkeOpLkqBYHk0MQ2kOUY4lOPICkuV4KlmDQ0lySJUmCVpAE0MQyCAMgxmeXholWTJjm+WZzkmAQA",
        categories = "weather",
        tags = "weather,shower",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDrizzle,
    #[cfg(feature = "cloud_fog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCgxDaDQ3EgN43jmPo9gSP4JE0MYVDIMRIDmSI6kwPoBA",
        categories = "weather",
        tags = "weather,mist",
        contributors = "ericfennis,karsa-mistmere,mittalyashu"
    ))]
    CloudFog,
    #[cfg(feature = "cloud_hail")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCgxDaDQ0HaNo4jqBBoj2SoHgkTQ4kSRo3jmPpMgWTpBkMMgwGgLgwDGVJJjyApNkCUAglyXpgmKVplliZwxDKDQ2lOSJuj6WRNnKaQymuYZ3koPoB",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudHail,
    #[cfg(feature = "cloud_lightning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ2C4MwyDYQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYQ0C4NQgieKYYhiH4oiALg5DcMwiD4PAvgOBY3jqBoICIbQxDODQyC2Qw1GgNBskYIA1jaOI9D6AQ",
        categories = "weather",
        tags = "weather,bolt",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudLightning,
    #[cfg(feature = "cloud_moon_rain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDAdgyCIPg8C+A4FhSF4GgiCgxDgLgzDcNggDENAuDUMQyGGI4jDAIIui4MwuDQNgxC2JopDcYwuiUOAtC4NgyDWPw2DUOY/DkNwtDELgyDSPguDeJRhDQIJVjCJAtDWTZGlqTQ2juQpDicOJHC4MJHkwMw5mOUg2kuTZPisIIti+L43icOZVlsMw2hOFYahmBIbgkTQzg4MBhDUIKLi6DYNh+Rw0EgMQzGGh6HliDQyoycZ/hag6CgWB6FDeJA5hGn6BgE",
        categories = "weather",
        tags = "weather,partly,night,rainfall",
        contributors = "it-is-not,karsa-mistmere,jguddas"
    ))]
    CloudMoonRain,
    #[cfg(feature = "cloud_moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDYYYNg0MAghQMYVCANhIDcYQ1CCHoWg4IA0C4OQtDYegiD4PAvgOBYri6BoIgoMQ4C4Mw3DaDokDUMQyGGOo6hSQwgDMLg0DYMQtiSPg3GMLgxDQOAtC4NgyDWVA2DWJolDcLQxC4MpSlQN5RGENIjhiFgtDWYZbmyYQ2k+V5YC4NQ4lwMImmAMw5nWZQ2l+YZSkCGZqhWS52DmaJtDMNoqiyMQ+gE",
        categories = "weather",
        tags = "weather,night",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudMoon,
    #[cfg(feature = "cloud_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg5DQIA1C4Mg3DQQQ3CCGAwCCGwxCAMYSDeHoMGgMQuDcORhDQLg1CCK4thuHYuhMMgyCANguDANQ3CIPg8C+A4Fj6QYGgiCgxDiJw5DaH5JDgMRBi+M4whyH4fDeLIfDkSA5heGZVjKIQ5hGJw4j2P5EkOBJFgkbY2m+GwyDCZ5AmsPoB",
        categories = "connectivity,weather",
        tags = "disconnect",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudOff,
    #[cfg(feature = "cloud_rain_wind")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GggIhtDmMwgDIMggDMLQ3jeOY+j2BI/gmQoNkkM4WkyOpQk+BYHlIMYVDGVZXjiWY8gE",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudRainWind,
    #[cfg(feature = "cloud_rain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCgxDaDQ0HYNo3jmPo9gSP4JE0OJEkaSI6kyS4FgeTgxDKDQ2lKOJUjyAQ",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudRain,
    #[cfg(feature = "cloud_snow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCg4g0NRoC4MAxjeOY+j2BI/gkTZCDEOZFkeSY6k2TIFgeTwxDKDQ3lSSI4lePICk2W4Kl0IAyDGYZWkuZpakATQxDaQ5umOcI+midJ2lKeJKliAQ",
        categories = "weather",
        tags = "weather,blizzard",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSnow,
    #[cfg(feature = "cloud_sun_rain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgiD4PAvgOBYThaBoICIbQ0C4OQzCCHogCAMQuDQMYlicMYShSGYYgSGoJE0MgwiUMhohGE4VjGMIFgeCRtDEOQuDAN4ih8MwtiaKIqiiLY8heAoxkCCgxDWHw0keDAuDYNRhDSIggjaZAtlgOQyDULYegwOJQi+U4/hsTYhjUYQ1CCeI2imKQ4h+axIDEMxhiGIZkiWDp5C4Mg0m+PZxjKVopjWEKOlKGZVE2Ww5pWO4vgE",
        categories = "weather",
        tags = "weather,partly,rainfall",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    CloudSunRain,
    #[cfg(feature = "cloud_sun")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgiD4PAvgOBYThaBoICIbQ0C4OQzCCHogCAMQuDQMYlicMYShSGYYgSGoJE0MgwiUMhohGE4VjGMIFgeCRtDEOQuDAN4ih8MwtiaKIqiiLY8heAoxkCCgxDWHw0keDAuDYNRhDSIggjaZAtlgOQyDULYegwOJQi+U4/huC4hDIMhIDcYQ1CCe42imKYjC0NhIDEMxhiGIZkiWYwgDYWpvj2AQ",
        categories = "weather",
        tags = "weather,partly",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudSun,
    #[cfg(feature = "cloud_sync")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDgLQxC4NQzDWDguDYMA1GGFoWDAIIfDELYQhINQiD4PAvgOBYoiuBoICITYMCAMgyHYLQ0GiOInimLotgSL4JE0MgwC4OQ5DaDg1C4Mg1DEQQ0hMIJRh6IJWgwLg0DmFg4jqEg3DkYYNg2IYghEMguDcMA5CCS4ZDePIqkCP4FgeQoNDEMB2jkNJxj6ApAnaCp4DSF4UDWEYYhqHJtlaZQ4oafpzgE",
        categories = "arrows,files",
        tags = "synchronize,synchronise,refresh,reconnect,transfer,backup,storage,upload,download,connection,network,data",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSync,
    #[cfg(feature = "cloud_upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg4CIPg8C+A4FhSF4GgiCg0g4NAuDgOQ5EENwgiYMIOioMQ1C4NwxCAOBoDGLg5GGIA1CCOAgimKYwDILo5DgLgyDQMoThWGoZgSG4JG0OIOiYNAth6VZIhaTA+gE",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudUpload,
    #[cfg(feature = "cloud")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMQ5EgORhDcIIWDCEIaDYLg3DELQ5GgMYdhQNIOCCJoPhkMYahkORaCIPg8C+A4FD6AQ",
        categories = "weather",
        tags = "weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cloud,
    #[cfg(feature = "cloudy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMQyGEMYQCAMIVhSFw5EgOQuDAMA2GENwgiOF4UhQNguDcMAyC0OR6CIPg8C+A4FjKNYGgiCgyDELg4DMMggDkQQzCCRYXkiEA5iQaAtDILgyDANxhDWDgglWD5JDALQxDCKpPDYMYxjOOA+gE",
        categories = "weather",
        tags = "weather,clouds",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    Cloudy,
    #[cfg(feature = "clover")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLgxDcIA3C4OAzCAMoWDIIg+DwL4DgWG4egaCIKDQLgwhcMQyGEMoTDKEIsDiLggDAIAxjUIAzhMMQtiWDxBjCMpAhCNI2kWF4lieK4ti+S4zjeNo9hCOQ4DGP5NkKTpFjUOQuDkOI1iqWJYkSNQtlOUIODeVoxkybJZk+KJcl6SpumOT48mmZo6EGWpklCYIahyIYggSIoJG2EoUhGE4VDgLgzDQIKOpCgYdoUPoBA",
        categories = "gaming",
        tags = "leaf,luck,plant",
        contributors = "ericfennis,csandman,jguddas"
    ))]
    Clover,
    #[cfg(feature = "club")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLgyDgIA5C4MA1GENQuDUIIXhkMAgDEIAwC0MQwhgNogEGG4ahiIIeiwMQyh6DQ2DaForimHYfh2F4PC0OAuDYWgiD4PAvgOBZDkaBoIgqL4xj8NhMk0MgykKRJJD6AQA",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Club,
    #[cfg(feature = "code_xml")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDYIA0C2EoUCIPg8C+A4FheGoGgiCoQDiE4RiQNIWhiHYcgSHoJgsNAuDWEQtjGD4nhmKw+gEA",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "ericfennis,mittalyashu"
    ))]
    CodeXml,
    #[cfg(feature = "code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAxDgIA2C2EoUCIPg8C+A4FheGoGgiCoQhSEYjDaFoYh0PoBA",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "colebemis"
    ))]
    Code,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYMgiD4PAvgOBYThaBoIgoMQ0g6EIShSGYYgSGoJgsNggDgYQxCCLYNg2LYyHaK4eh6MIuC2NhIDcYY2CCOAxjqOhWDmLIukCSYyC0MRoh2PggjeSIvioaJMiGFYliSBYHieKYPhGE5ZheAQA",
        categories = "food-beverage",
        tags = "drink,cup,mug,tea,cafe,hot,beverage",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    Coffee,
    #[cfg(feature = "cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDALgyDcIITDMLgzDQIg+DwL4DgWG4egaCAiG2DIOhYNwzC0NAgDYLg5DOGociGIIEiKCYLDIIAyDIdgtDKModjaNYFgeOAxjqPZAhuQofgKNpGgoMYskgaA4kGNJPkWI4lhMMoQDYNgtDGYwuimWJDlqN4kDGFYXDSY4OmaMZMlmIZRE2OpVkuM5pneXJfC6YYODeZYpmOaJOn+CRtoGg6FDGc4OokaJEmsbYWhihJyocMaUpaUaYm+FKcDOk51mkYxpHIYxsGUIBygmQAgGMeYJkgIq0Hit58C+qqsq6G6/q2rxjrsIq4rStrIrOsQileTLDsGAQ",
        categories = "account",
        tags = "computing,settings,cog,edit,gear,preferences,controls,configuration,fixed,build,construction,parts",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Cog,
    #[cfg(feature = "coins")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg3DQNAgDEN4ODMNhhDYIIZDCEoSC2FA0DiHwuiEIg+DwL4DgWJ4qgaCIKDENYaGgMR2DSJooi2LIEi6CRtDYLoMhEMQ0g4Ng4C4OA2DYLQujIMggg0NA2jeJ4pjyJxjGkchjGwZQgGMeIJDENgimAeYJDiZhygmZZWlqXJeD6AQ",
        categories = "gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere,jguddas"
    ))]
    Coins,
    #[cfg(feature = "columns_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NGgZRpGcaB0goMQ4g0eYKhEIB3GkZB0GiGYbD4PAvgOBYoHAYYkCAZIKE0MQyCAMx2hoIooC+LokD6AQA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,parallel,series,split,vertical,horizontal,half,center,middle,even,sidebar,drawer,gutter,fold,reflow,typography,pagination,pages",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns2,
    #[cfg(feature = "columns_3_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CAMgxEgNRhDKEAgDCGAgDELQyh0VoUhaFoZhkMYQh0aAxDSFYXiSG4XDIdg1g4Ig+DwL4DgWNo5gaCAiG2KguDOGw5C4NobC0Lg0jWN48juBI9gmC4PDMdg3jSNo4lCT4FgeCZAjOFgxDYLg5kmZpCkyWo6gKUJej+Y5GhAMQulcM5nmqTptl2PpAmQOIbjOd5KC0MZ5lue5RnCRYmDGYZCniWZ6jyb5AkWR4RnWSQ0oWh5spSfQyg0N4bn+SJKp4aJcoobaZqQMZFpyZZJDOqarm8TQ5CCVQxDitg8GMaRyGMbBlCAcoJrUIBjHiCa9CKyx5s6vpZsGw7FD6AQ",
        categories = "layout,design",
        tags = "columns,settings,customize,table,grid,adjust,configuration,panel,layout",
        contributors = "irvineacosta,danielbayley,karsa-mistmere"
    ))]
    Columns3Cog,
    #[cfg(feature = "columns_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHmCYLCAdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJE0OQgDMdodCKKAvi6Goti8aIxjMMQ1jaOInimPRoD6AQA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages",
        contributors = "danielbayley"
    ))]
    Columns3,
    #[cfg(feature = "columns_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NHcaRkHQaIKDEOINGgZRpGcaB0hiGggHmCoRD4PAvgOBYoHAYYWCAZIKE0NwuDUIAzHaGQiigL4uhaLYvGiMYzDEMo4jqGo9j8aJBjCMgiE0MQ2jaSI7kuQg+gE",
        categories = "layout,design,text,security",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages,prison,jail,bars,sentence,police,cops,cell,crime,criminal,justice,law,enforcement,grill",
        contributors = "danielbayley"
    ))]
    Columns4,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAzGEMQghEMAghSEYXHYNYQhKFYdDELYXCIPg8C+A4FiOJoGgiCgxDmDobhOHocDGGYwh2FoghKIokimKIEiqCRtDeEg1g6Do7iWP4+gWB5BkMMoRDMLQzEiGgyCCV4UjgMgtDIdpdkiPQ8HIZRjHQIB3GkZIFgkNwiCAeYJgybx4nINJvGgZRpGcaB0m2bxynUIgxkiZJmiOhpnnEIgzniep8n4IpuCCgZynSCaNmiapspKhZlHQPoBA",
        categories = "development,files",
        tags = "cubes,packages,parts,units,collection,cluster,combine,gather,merge",
        contributors = "danielbayley,ericfennis,jguddas"
    ))]
    Combine,
    #[cfg(feature = "command")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2HYMQyGEMwghQMAgDEIIXDMLQzEgNoThWGoYiOFAzFaIImiOGQwh2FRohGIYWiSLYcDMIg+DwL4DgUPoBA",
        categories = "development",
        tags = "keyboard,key,mac,cmd,button",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Command,
    #[cfg(feature = "compass")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ2C4Mg0CANwuDcNgtDELg4DCLQ1C4NAxDEYQyCCPwwCCQgxjOKw2DUII0DKSBMi+MZKiqLBsjSNg0C2OY7j2P5BkOSpKkcNZGkwNR6hmG4hiMPoB",
        categories = "navigation,travel",
        tags = "direction,north,east,south,west,safari,browser",
        contributors = "colebemis,jguddas"
    ))]
    Compass,
    #[cfg(feature = "component")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg1DMNggDEMQuDIOQzGEMYSCAMIch6HYUDQMQ0GwMguDMN4RiaKA3hmG4djCEguiINIciWJw3DcLYrjmLoajGMAtiGIxsjuOI6jyLYaj+H5CjOI4cHoIg+DwL4DgWVJXgaCIKiaFg3hKFIWhiS4fmaQ4kkkIJJj6Zogk+NQwjeLJGiybZAhyTo0EwNguDAOA4CAOAuDkMQ2neTZolGU5VlqWYEluCRNoOhYRDEN6EDEMqIniQw1nOOZrjih5lp2cI2kmdYppyiZPp+qpIqOrAwnqUAwlKVJWpCj4FgekqUoYIA0C4Ng3DSs4emioJgjypIvmep5yqmzbIrSyqwrCs61nGuKNruAQ",
        categories = "design,development",
        tags = "design,element,group,module,part,symbol",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Component,
    #[cfg(feature = "computer")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAdxpGQdBogkMQ0gwaBlGkZxoHSCQ4gweIJDWDByiCCgiD4PAvgOBYoiuBoXhmG4dgyDoQhKCgwh+CYLCCJI7gyCAihSJ4pi6KBwGGEQgGSCRNDYIAxDgaILigL5IhGR5JGiS5NDEMpQlINpElaWg+gE",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = "danielbayley"
    ))]
    Computer,
    #[cfg(feature = "concierge_bell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgwGEMQghEMAghQMQthcMR2hgYQyg2FYghEMgtDIaAxDaHYfhSFofDIdgxhCEoghaGISFoIg+DwL4DgWOY8gaCIKg6EooDgIJGiwMIYDaFY4jqP4+gSQIJE0MYeDQdg0k6O5SlGBYHlQMYUDQaJajmXI9gEA",
        categories = "travel",
        tags = "reception,bell,porter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ConciergeBell,
    #[cfg(feature = "cone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDALg5CAMQ4C4NQ1C0OAtDENYODgYQxhAIAwiCIIYC4OIgGyFoQhoOQ4CIPg8C+A4Fi8ZRsGwaRwHMZQgGMeYJDEOQiCAch4gmQY8kUIgxDKQhyj4Igzi6MI1jeORlD6AQA",
        categories = "shapes,math",
        tags = "conical,triangle,triangular,geometry,filter,funnel,hopper,spotlight,searchlight",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cone,
    #[cfg(feature = "construction")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggGgZRpGcaB0g0OIRHIeINDGEYdg6ER5g0NgiD4PAvgOBYoHAYYLCAZINE0MQ3CAMQ0HYN4nimLoLi2LxojGM42jiOo8C+PhokCMIyCKNI2DMdgzkiSpMkKThNlGU5VkGV5Dk8MQwjcNAgDILgzCANpol2Pw8kqYBtjiaggDcLo2naO4okmXpvkGcQ4nSgYanuVoBA",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyHYMgiD4PAvgOBYThaBoIgoMQ3C4OQxDWDgyGGDYNDAIIoDALQxDKKYShSGYYgSGoJE0OIOhCMIVjSExjGkchjGwZQgHKCQ0CIIBjHiCYtkkYx5k2EYTC+P5BkOExyGUYx0kWTAihEIJRCKSAgHcaRkgWTQ4kkaBlGkZxoHSa5Jl8M47lqXA+gE",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ContactRound,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyHYMgiD4PAvgOBYThaBoIgoN4Og8LQyGEMoOCAMIlCAMYOiAaA2iKJImiaKYjg+EYThWBBohiOIHgkTQ4g6EIShSGYTGMaRyGMbBlCAYx5gkMQxCKTB4k+EQgHKCQzkIL5GkiSoTHIZRjHQIJOCINJSlQIpaCAaBlGkZxoHSTw4lIcpqlYdxpGSBZ0luYZjD6AQ",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "lscheibel,karsa-mistmere,FPDK,ericfennis,jguddas"
    ))]
    Contact,
    #[cfg(feature = "container")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA3C4NxjDALQuDaFA0C0MQuDKFA4hkLg1GwLQ2C4MwtDMLg5GGGg3g2LINDAIIxhOLIyiIMQwiUIA2GOFA1huFA5C6HopCCGg0HaJI8jGIAuDSRobkOUIhiQMwgiiKovlCLYyl2MY1DAbI4iWIxjiCFJYhkIIph8NRaCIPg8C+A4FnGdIGgiCo4CAMoaDkVgxDQTAyC4MQgkIMZwnKd52gSeIJG2e6Bkafojimipzo6jYFgeCRNpMMZCDgdgtDihaYoyAqOp2eg4kaDw1FaQg0qimoBA",
        categories = "development,transportation,mail",
        tags = "storage,shipping,freight,supply chain,docker,environment,devops,code,coding",
        contributors = "danielbayley"
    ))]
    Container,
    #[cfg(feature = "contrast")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDEOBhDYIIuDAIIxjMLYOHaDh6hmG4hiMPoBA",
        categories = "photography,accessibility,design",
        tags = "display,accessibility",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Contrast,
    #[cfg(feature = "cookie")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMQwCCEQghIMYVhOFoSDQIIchKFgtDWIYdiSH4TiGIQiD4PAvgOBYri6BoIgoOAuDUII1DUdguDAMYqiyMYwgSMoJgsNoTDWNo7j2P4tkOQoFgeRYMhMMpLj6K5Oi+ApDlKCgxhcMQ3leTZBlyUYzE0N4TDSZJZkGAQA",
        categories = "account,food-beverage",
        tags = "biscuit,privacy,legal,food",
        contributors = "it-is-not,ericfennis"
    ))]
    Cookie,
    #[cfg(feature = "cooking_pot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGgMgwCIPg8C+A4FhSF4GgiCoRg0Mh2DgYYMgwMAgiYMQtiQSA2iMIIlieDYqiodgtDiE4VhqGYEhuCRtDQIA4g0NgtDSOIWjyO4FgePg4C4OA2CANguDcOAtC4NA1C0MZPDGLowiiDZYloMpYDMbJcDkNJXDSIokjGYZlDQM5iDQNhsmOYo3hSSIYgE",
        categories = "food-beverage,home",
        tags = "pod,cooking,recipe,food,kitchen,chef,restaurant,dinner,lunch,breakfast,meal,eat",
        contributors = "guillermo-angeles,ericfennis"
    ))]
    CookingPot,
    #[cfg(feature = "copy_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDUIINg0NAtDQIg+DwL4DgWGByGUYx0CAdxpGSBYJDGFggGgZRpGcaB0ieKRyHiCQyCIII0CIOI3HIeY1jePo6heGYeiCGIbgaCAiE0NIODYYwtDELgxCAMAtDILQuDmV5XFYNBjlaUpSluEpXGgMQwGOYpVhEIJam2NoYhqBBoD6AQ",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDUIggHiCYLDiDh4DGCgxDKDh5haC4ND4PAvgOBYfHIZRjHQIB3GkZB0GiFw0g4ch4gqGYPgqEwgGgZRpGcaB0i+Go3jEeY0CKHwviWJ4fHAYYtCAZIKE0NAgDENhjC0MQuDEIAwC0MgtC4OZel4Vg0GOXZZlmYgyCCXwyGgMQwGOaZcm0IJhnaGZHkyLQ+gE",
        categories = "text,math",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDUIggHmCYLDiDh4DGCoMg4eYWgsMgiD4PAvgOBYfiKBoVheHYHhEMYTg+G4Yg+K4Nh+IYEGWHxyGUYx0CAaBlGkZxoHSFw0g4ch5gqKRyHiSYOHcaRkHQaJEhSCotkgIoTjSOY7h8cBhlIIBkgoTQ0CAMQ2GMLQxC4MQgDALQyC0Lg5nGcRWDQY5wmybJ1DIIJyDIaAxDAY58m+gAgnSiYdjSX5SD6AQ",
        categories = "text,math",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIoHDKCgxDiDh5hCC4NCAeYJguEw+DwL4DgWHhyGUYx0geCoTCAdxpGQdBohENIOHIeIKhgch5jaDhoGUaRnGgdIxhSKQih4L4kiaHhwGGLwgGSChNDQIAxDYYwtDELgxCAMAtDILQuDmXZdFYNBjlyWJYmEMggl4MhoDEMBjmiW5sCCYJ1g2RpLi8PoBA",
        categories = "text,development,math",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIggHmCYLg2DwygoMQ4g4eIVguGA+DwL4DgWHohgaCIWhOGoWhiD4RheDh5huDAih6IIEGWHhyGUYx0CAdxpGQdBohYNIOGgZRpGcaB0kOL4Kisch5gqExyHiUoZk6M4fjmO4eHAYZBCAZIKE0NAgDENhjC0MQuDEIAwC0MgtC4OZwnAVg0GOb5rmudAyCCcQyGgMQwGOe5un8IJzoiDY0l6QQ+gEA",
        categories = "notifications,math",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAcoICIMoMGgZRpGcaB0gkMQ0gweYJguDYdhCDB3GkZB0GiGYbD4PAvgOBYrHAYYnCAZIJE0NAgDENhjC0MQuDEIAwC0MgtC4OZDkMVg0GOQo+j6RwyCCRAyGgMQwGOTpBlIIJGluEYrC+MYnD6AQA",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[cfg(feature = "copyleft")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQ5C4MQ3CAMQ0C4OAzGENAgjUMIuCCOAwC0NQuDYNgihsL4iiQPoB",
        categories = "text",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[cfg(feature = "copyright")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQxDQLg4DMIIqiwMxhDQII0DCL44DALQ1C4Ng2CKGwviKJA+gEA",
        categories = "text",
        tags = "licence,license",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyright,
    #[cfg(feature = "corner_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA0HYNxhDSDggg2DQxC2Ew0EgNAiD4PAvgOBYfiKBoICIbQ5CAMQwC0NQgi+MYeiCJQ+gE",
        categories = "arrows",
        tags = "arrow,return",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownLeft,
    #[cfg(feature = "corner_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDAIINDULYSCIPg8C+A4FheGoGggIhNDQIA0HYNxhiKIoQiqI4jGgMQyhaGIdD6AQA",
        categories = "arrows,text,development",
        tags = "arrow,indent,tab",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownRight,
    #[cfg(feature = "corner_left_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDULQ1CCEIUCIPg8C+A4FheGoGggIhNDIMAgDQaAtDcYYNg2I4sC2Kh2DEMoWhiHQ+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftDown,
    #[cfg(feature = "corner_left_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA5g4IINg0OQiD4PAvgOBYWhmBoIgoMgwCCIBoC0NxhhIIIhiEMQtDSLRWDSFYXhwPoBA",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftUp,
    #[cfg(feature = "corner_right_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAxDUIIQhILQ1CIPg8C+A4FheGoGggIhNDQIA0GgNxhiKIoNg0MYjiMdgxDKFoYh0PoBA",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightDown,
    #[cfg(feature = "corner_right_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIA5CANQtDWEIQCIPg8C+A4FheGoGggIhNDQIAyDAaA3GGIoig2KwgDQLQ0FYNIWhiHQ+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightUp,
    #[cfg(feature = "corner_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMHYLQ3GENAghSDYXC0NIZEgNAiD4PAvgOBYfiKBoIgoOQgDGFIUDkbA1C0NYeiCJQ+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpLeft,
    #[cfg(feature = "corner_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDQIA1C2EoUCIPg8C+A4FheGoGggIhNhAMgwHYLQ3GGEIQDAIIrDEIA0C0NBoDEMoWhiHQ+gEA",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpRight,
    #[cfg(feature = "cpu")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAdgyCIPg8C+A4FhSF4GgiCoMg6EYThWGoZgSG4JgsN4OhCEoUhaJYkgWB4nDGKQyiCLYjgKJYygqDYMGiLIii+OoxhwTY+DeQIhi6GJEiaPQgkmQZMGiMJPkcMAgj+U45hqPJYlqUpLl2O5Gg+UZKjiQ5ekaNYrmOa5lieNY3kKTRyGUYx0CAdxpGSBYJDENgiCAaBlGkZxoHSgaDCAch4gmEggpAIg0oQeYJpaLZ4nqFKcnufZ/GiCQ4oSlKloWh6JosIqopiraEo+gZLp8PoBA",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCAOQuDMYQyC4OAgjCMoqjYLQzC4NQgDEII5j2PwgjaQpEjkNIzC4N5IkqQ4qjmOwxhmG4hiOIIiiSJgiiiSosi6NJIjWRAwjiOo8j4LpAmiYprkaS5uk2Z5QlIL5UGgPoB",
        categories = "text",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goNYNHcaRkHQaIKDIMINGgZRpGcaB0goMQ0g2CYLCIPg8C+A4FikbBpG4ZQgHgMoYgyDo1CIMYag4MYijweI+ieKQvi+MQ+gEA",
        categories = "account,finance",
        tags = "bank,purchase,payment,cc",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    CreditCard,
    #[cfg(feature = "croissant")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyCAMQ4EgNAuDcNw0GEMQuDWEIbCAMIfhALYaDMNQyC0Lg5DeEAxiyIYgi0LgxDOJw2C4NA4DcIg+DwL4DgWPI/gaCIKhGEINDIVoUhaGIahyTovh+KIqiMLolg8MYtlmUQwC2No4jaMwyjuPZCkGBJDgmCw4CANRhDQIAzlGLZwnKD4PiCMAtneR59nmUg1jIMw5h0NJjjyPpomeBYHmqTw4GGcpwn+IKSCCfKUpee59gyc6FiegQxDSZKJkCApoo2Cg4hUMKEDKGw1k2MKZl2YQ1hytpPh6UKZC4Ng3DaHQ2DINhsDkLg4DCK6BoYYaYlGrw3hGe4VhEbAtsyJ7HskN4ZruHqUlWww2iivw2qSZoBA",
        categories = "food-beverage",
        tags = "bakery,cooking,food,pastry",
        contributors = "karsa-mistmere"
    ))]
    Croissant,
    #[cfg(feature = "crop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMh2DENBhDKDQgDCFYXhMMhohAIg+DwL4DgWHohgaCIKDEOINDIVg4hKFIWjALQyjISAyh2H4kD6AQ",
        categories = "photography,design",
        tags = "photo,image",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Crop,
    #[cfg(feature = "cross")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAORhDIIIQDAIITDALYQDIdgyg+EYUh6E4YGgNBhDEIIlhWJopDEdojhiH4eiGG4uiiIIXHYLYjiWJ4ejoLQxiKHISi8Mo2heQYvhaRAyGiOIkimKAxj6PhWi2HY0heF5MjKVofheEYsk6O4TlGOh6CIPg8C+A4FD6AQ",
        categories = "shapes",
        tags = "healthcare,first aid",
        contributors = "lscheibel,ericfennis"
    ))]
    Cross,
    #[cfg(feature = "crosshair")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bGwaRuggeQxhmEB5DKKAgHiJwiDKD4tiuDQ4CKGwviKJIhiOJY0g6EB4jSMomiyLoMDaN4cjoZY8iSM4skUIpJCCKoMjKR4NhqS49k6CJZkCVYvjGKY/jaUJakqOZdgEA",
        categories = "photography",
        tags = "aim,target",
        contributors = "colebemis,ericfennis"
    ))]
    Crosshair,
    #[cfg(feature = "crown")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg1DYMggDMLgyDYNhhg6DggDCGwgDEIAuDgNw2hsTAxDULgzDkIA4iENxhh+H4cjOHoODENoUDkNBMDKDQxDgMwgigNYYkOHYch8Lg3DmLQ1DEORsC0MohDMNIeDCFA0heMZHh4LQuDmD5KlUSIoDgMYwh6XQxl+YQ3l8N5VjwLgwhGOJ1kWGo0kmSw4l+TpQDSFIjhILoWDSaYymuNg2l8Mo6HoIg+DwL4DgWk6WgaCIKDUII9GgMQ0pKlKZD6AQ",
        categories = "gaming",
        tags = "diadem,tiara,circlet,corona,king,ruler,winner,favourite",
        contributors = "ahtohbi4,ericfennis,csandman,karsa-mistmere"
    ))]
    Crown,
    #[cfg(feature = "cuboid")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDIdgtDgIg+DwL4DgWFYYgaCIKDILgzDMNggDgLg4DkIIMikNBsDEMQuDcMQ1C0NwuDAMg5hSFobhqBIcgkTYPisYQyg4IINg0MQtC4OYximMIyGwLYqDaRJGkiKQth8MQzDiSwwDWUg2C0NBBkWRZYDGRgxDaEZVmeR5xmqTIxlOUJhgwLZvlecoOC6XIlmAbIjmWcJpg6RQ4HqOoXj4PoB",
        categories = "shapes,math,food-beverage",
        tags = "brick,block,box,3d,solid,volume,container,storage,shipping,carton,geometry,rectangular,hexahedron,butter,tofu,soap,cheese,package,parcel,crate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cuboid,
    #[cfg(feature = "cup_soda")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAOAgDELg3DWDwyC4Mg4GEMgghoMAgh2HYahANwyGgNAuDUNIZhuHosiALYiDITAxg4OAiD4PAvgOBY3jqBoICITYTDgaAxDSNo4j2PIEj6CRNDeDw1GENguDST5TlWLYPCCE4dleVpUk+H4slyR45kuSoFgeCRtDGGoODELQ2GgMplkmAQ",
        categories = "food-beverage",
        tags = "beverage,cup,drink,soda,straw,water",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CupSoda,
    #[cfg(feature = "currency")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeILDEMoOGMeYShQPg8C+A4FgeGhsGkboIHgMoLDaDh5iYIooCAeQxgsM4OHiMAijKGgviGI4giKJI1DIMYzisMYNi6NYyi6K4ojiOhljyI5GguQIzkeQonimQ4NkyPZPgiL5SkGSYSkWJZjlSYAiluO4B",
        categories = "finance",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "cylinder")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx5D0Ig1CIIByHiDQ5hAY4TCIMQyhAcoMCIMwiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUdgxDQYQ5CCMgwCCPI8DEOI9FaD4iC+KosD6AQ",
        categories = "shapes,design,math",
        tags = "shape,elliptical,geometry,container,storage,tin,pot",
        contributors = "danielbayley"
    ))]
    Cylinder,
    #[cfg(feature = "dam")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIIMC4MwxGMMQuDENwuDUNoOhgNIbDYOQgDOGIeiAMojDAIImDULQyCCK4thQMwgiiFA5iOKoOCIPg8C+A4FjuPoGgiCoPDcNYODgY4QDWI4UDSR4NiaRoOimJ5Vi+LoshuMo0C6NpHjgMY6jyQZAgSQoJE2MAwGgNJjj2Z5mgWB5pjANJtm+ZYCmedIKjAOJ4jucI/nuc5DmoIA2oGZJxoWaIKDeIRhg2DYopYLaUHYMQ2pOVKWjOVAxm2naVqCNKYEwMYoDSpKmjOmKYHqeZxgEA",
        categories = "buildings,sustainability,navigation",
        tags = "electricity,energy,water",
        contributors = "AnnaSasDev"
    ))]
    Dam,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx4D0IgxDIIggHIeYNDOERjhQIg1hEcoMCIOQiD4PAvgOBYHGWIhwGEdBoCAZINE0Mwgg8YQ5CCMgwCCOY5DUIAyC4NogiIL4qiyKYri2LwiE0MgxCAOQuDMVobkORRokeLIujCMg1HYMQ0jWN46mOOQ2C4NA3j4Lg4DiIYjlaWJJjCD4zDIdg0GgNJukSSJxlqSwxjIMgwGGPY9juYw5C2Mg0C6PaNoeZAtpCk6OGMLQxlGOAtj8NaQDSi5nk6mQ0DETJ0DENp7nCAQA",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDELg2DkMxWDUIg+DwL4DgWGIbgaCAiG0MgyCCIwthAOA3DWJwuimFoYhqBBoh2MoHgkTQzg4MhhDkII5DAIJAkAOIRDkN4lC4OQ5DiF4Zh6NIFjaCo5DUdgxDSPI+kGW5ADkLgyDiSJKDmTYxhwPBjGkchjGwZQgHKCQzCIIBjHiCQxkydB5neTIwmma5thgZRsGwaRwHObp1ncMpzHKdgimSb57CKcp6gmL4ZoOhaHGUPoBA",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas,Spleefies"
    ))]
    DatabaseSearch,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch5D0IgzCIIBjgwIg1hAch4g0OYQGOGAiDEMgiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUVgxDkQQ5CCMgwCCPI8DENQgDIMQuDgNIhiOKosimK4ti8IhNkOMxWDiSAvkoaJMiyLowlKHxMDEOAgDENxIDIMpgjmZ5WliWpOjCMofjiOo9nWPw0C4NY5DGeA4DebJND6AQ",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch5D0IgzCIIBjHiDQxDKEByhMIg5hAY4MCINQiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUVgxDkQQ5CCMgwCCPI8DIMQgjYVogiIL4qiyKYri2LwijGQgyjiOo9lSP5BhWIYjkgaA+gEA",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "decimals_arrow_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMIAyDELQzhEIISDMIg+DwL4DgWGIbgaCAiE0MgwCAMQ4EgMQwheGYeh2BIfgkTYNDEMRoC4MAxiuGovhgchlGMdAgHiCQ2CIIBoGUaRnGgdIJDiRhykMIgyC4NZGHcaRkgWCZWCAeYJhaGAvj6QA+gE",
        categories = "design,text,arrows,math",
        tags = "numerical,decimal,decrease,less,fewer,precision,rounding,digits,fraction,float,number",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    DecimalsArrowLeft,
    #[cfg(feature = "decimals_arrow_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDgaIMCIPg8C+A4FhSF4GggIhtDENwgDIMQgDMLYlieE4VhqGYEhuCRNDODgxGgLgwDGKYWi2FByGUYx0CAaBlGkZxoHSCQ4CIIByHiCQyC4NZJkwIgxlAIB5gkM5JHcaRkgWCZQhQL48j6O49j+QZDkWR5JlcIpZCCW5dGiX5JkuTZPlGCQ2jiYx0D6AQ",
        categories = "design,text,arrows,math",
        tags = "numerical,decimal,increase,more,precision,rounding,digits,fraction,float,number",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    DecimalsArrowRight,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA1GEMgghGDYUC0MQuDMNA0C4NQxDkbAtDaGAyDiDguDcNBhDEIIrhQIIuhcNA4DEbIiDOJImigMRBhGE4vj+DIsDkaIMhCEo/i4MgtDIVg3kaPoVkoMh6CIPg8C+A4FlaWYGggIhtDGEQ5CANpklWV5cluBJdgmYIlDmIZmlaWJrD6AQ",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxDYMggDODg2DcQYMCCGAwCCG4bhEMQzGGEYRh2HAgDSHB2C0MYiCCJImDGJ4pDSLYviWKAwiqNIjiaG4xjgdosjyJYbDQLQuDAMA2hmPpEhwLQ4hQMQtDkLg4DINgiD4PAvgOBZbl6BoIgoMoNDgMIoDENJWDYORhDkIJwiWUwxDcLg2DAOIclqXJhlsYxpHIYxsGUIBygkMgiCAYx5gkNKKGMeIJDGiZbC+gKCoQPoB",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = "danielbayley,jguddas"
    ))]
    Dessert,
    #[cfg(feature = "diameter")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDEOYOGMeYShQPg8C+A4FgeGodgaCIKgyFYRCINYVheKAihqHIEiKGhwGEdBoCAZILE0NguDQOAgDMLg2DYYQxDAIJFCCRpGDGR5ADgNpNC6T4thuM41jKNI2jgIhtjsNJMl6TAxDELgxj6Y5lDiVAvlYaJYjWN45kCQggl4OJEkuSpJnsMZOlCfZSDaa5tD6AQ",
        categories = "shapes,math,design,tools",
        tags = "shape,circle,geometry,trigonometry,width,height,size,calculate,measure",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Diameter,
    #[cfg(feature = "diamond_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NwgDEMAuDMYYMDQMQghWFwwCCG4dCAMwuhYbA3C4NQ5CCJImhSIYXhmHIvhuIIWhyI4lDkLYpDmK4zi6HodC2MgxEwMYgg6DA3DGO4tiyMIckCTAwHoIg+DwL4DgWVJXgaCIKDiDwyGgOJTlWWg+gE",
        categories = "multimedia,photography,tools,devices",
        tags = "keyframe,subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "chessurisme"
    ))]
    DiamondMinus,
    #[cfg(feature = "diamond_percent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NwgDEMAuDMYYMDQMQghWFwwCCG4dCAMwuhYbA3C4NQ5CCJImhSIYXhmHIvhuIIWhyI4lDkLYpDmK4zi6HodC2MgxEwMYgg6DA3DGO4tiyMIckCTAwFoIg+DwL4DgWVJXgaCIKDkLgyCCXgyGgLgwDGU5VlqWYEluCRtDENIlmGJQtDUIA1miVpsmuBYHgkTZwg2D5xDiZJmnmaoBA",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    DiamondPercent,
    #[cfg(feature = "diamond_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA4HYOAiD4PAvgOBYThaBoIgoMguDcIAxDALgzGGHQ0DEIImigMAgiyLggDMLonGwNwuDUOQgjWN4ljKKIqi2QIsjGJ4tjSNg5C2Og5jyRI/i+LgtkMMRMDGMYfh0NwxkyPo9kGLZRl0MB6hKFIZhiBIagkTQ4iAMhohGE4VmgPoBA",
        categories = "multimedia,photography,tools,devices",
        tags = "keyframe,add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "chessurisme"
    ))]
    DiamondPlus,
    #[cfg(feature = "diamond")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NwgDEMAuDMYYMDQMQghWFwwCCG4dCAMwuhYbA3C4NQ5CCJImhSIYXhmHIvhuIIWhyI4lDkLYpDmK4zi6HodC2MgxGyOI2kSKouj2MJAiyHBaCIPg8C+A4FD6AQ",
        categories = "shapes,gaming",
        tags = "square,rectangle,oblique,rhombus,shape,suit,playing,cards",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Diamond,
    #[cfg(feature = "dice_1")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEch4g0MoRhgIoVCAcoTCKGggGgZRpGcaB0g2DwiD4PAvgOBYuHAYYLCAZINE0MQyCCOxoC4MAxi2L40gsPoB",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGoNggIoRCCD4KCIPg8C+A4FiwcBhhYIBkgkTQxDUIA5GgLgwDGK4tjKFoxjMaI1jcOQgjmPY/kEL5DGgPoB",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEcoTCIMoRGgZRpGcaB0g2D4WHiDYaCCJQihUPg8C+A4FiwcBhgsIBkg0TQxDYIA4GgLgwDEIosC+MoLjGMxojWNwxDIIJLj2P5Bi2RBokaNI2CITQ4k0NpPkCQpTD6AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEhyhQIgyhIdxpGQdBog6EIahiHQiD4PAvgOBYrHAYYjCAZIOE0MQ2CAOBoC4MAxiqLIxiOMIyGiNI2DiOo8j6QAvkIaJEjONQiE2SY4kuP4rk6RZRkaU43jmV49lmQZcgE",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKERyHmFYRGgZRpGcaB0g2D4RhQIgzhGGYmCIPg8C+A4FiwcBhgsIBkg0TQxDYIA4GgLgwDGK4tjKC4xjMaI1jcOI7j2P5BC+QxokWNI2CITZKjmTJAiyT5GlKR5UjiOpYj6WpCl0PJQkiVQxDIIJslmTpQD6AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgHcaRkHQaIKDEOINGgZRpGcaB0hiGgggkIgzg2D4mCIPg8C+A4FiwcBhhYIBkgoTQxDYIA4GgLgwDGK4tjKFoxjMaI1jeOQgDEMo9j+QQvkMaJFjSNgijiOo5k6QIslGRpUkeVhNDiO5blCUpgkiV5kkyZpdmgPJSmqY5LDabpCl+AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMgiCAeIOhEIByhQIoWHmDgxDCEhyhyGoSHcaRkHQaIdhEPg8C+A4FiwcBhigIBkg4bQxDcLg5DIIAxDQIAzC4NQtkINRhDILgykCSZLCAMJPlEMJFGwLZEDSOwykiSpMlyUpPkWTxMh4IA2CKLAvjKKIxjMaI1g4TQ2j4OBoC4MAxmeLZqGibI0jYIhNmSP51neeZpm2fZun+gQ1mWhJ4mie6Jm+gAxDgIA5o+hqSgEA",
        categories = "gaming",
        tags = "dice,random,tabletop,board,game",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dices,
    #[cfg(feature = "diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ0CIPg8C+A4FhSF4GgiCg1CAMQwGiEYThWGoZgSG4JE2HgyDGIoShSFooD6AQA",
        categories = "development,files",
        tags = "patch,difference,compare,plus,minus,plus-minus,math",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Diff,
    #[cfg(feature = "disc_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCIKhODwghYIg0hAY4Sg2D4ahyBIihocBhHQaAgGSDBNg4IIOGgLgwDGGYbjWNw+gEA",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ2CCDhjDALQxC4N4xC0MwuDKKwuDgLQ0jYIobC+IokhuHoGgiF4ZhCEgig+CYLg2GockSIA8kGJYnCITQxDiKwyi2OA3C2MQgjUMovjkII8lCQIjGgPoBA",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc_album")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoSHmDgzhKFQihsIB3GkZB0GiDoQCIPg8C+A4FikYxpHIYxsGUIBjhmD4XjWHQxjkcoODWKIqi+MYzikcBhiQIBkg4TY8CCPBoC4MAxkEL5HiQPoBA",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "danielbayley"
    ))]
    DiscAlbum,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bh6BoIgoIoVhCEoMieF4ZCKG4dgSIw+gE",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDaDhjHiCwxDIIg+DwL4DgWB4bGwaRuggeQyheGQgHgMYLDWDh4ieDA5g4eYsgyGYbC+IokhuHoGgiEIXDiFIWjeDoKgyGocj6IIBA",
        categories = "math,development",
        tags = "calculate,math,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyGMLQxC4M4NhINQtDILgwDkMggDOGAug0NIhEwMQ0CAOAiD4PAvgOBYri6BoICIbQxDcIA2iAOA5DGOo8iqLIxjCBIygkTYdgwYwzhMM4fDOOAuDYNg3C2TwxDCVZAi2RJDgWB4JG2HZiDCDgwlqQoCkSX40DKZA5CALo7hKcpnlyaZejOR4dDmEIWCCEoUlWDoahwLYjg2GQ1GyEYRnWL53kWCpLleb4mC6lAgieDKOGiXaRG0Nohn+iZ/n+nKemsbY3DEOKDnKro/iuW6PjGaxNm8MoPoCF5+hmGwyoKiaGiSV5/Dap4BA",
        categories = "medical,food-beverage",
        tags = "gene,gmo free,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    DnaOff,
    #[cfg(feature = "dna")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAxDaDguDWEQ1CIPg8C+A4FheGoGgiCgxDQIA4C0MYSiWEoWhiHYcgSHoJE0MYTDIY4oDcOQ4hEOY4C0MoSDGOQzC6Ow1j0Lg4DANwgDWQw5DOKoZi6LYFgeCYLDaEoODCWQxg6UIsgKLpViCSg2kYOA5DGZ5pl+UphlSHxNDKDg1GOWA2DYNwthAMZCDOfwgg0MgwnubYbm+L4KoMIA5CCR5po8MaGGiU6JE2QoMo2IQupkIIijKk6VmMbZYhMMY+qaXoXlGh4dqOSpACCPpol2s5squYKunGjQyjSJo3iSJo7jmqJAC2QpErKR5JC2TI7k+uJSgE",
        categories = "medical",
        tags = "gene,gmo,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere"
    ))]
    Dna,
    #[cfg(feature = "dock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBoDIMAiD4PAvgOBYTHIZRjHQIB5gkNAiCAch4gkMohGgZRpGcaB0gkMQ2iEdxpGSBYlhEIIkCKJoTC+GYbhOFoGgiCg2CCLxoDGO4UkEPoBA",
        categories = "layout,design,development,files",
        tags = "desktop,applications,launch,home,menu bar,bottom,line,macos,osx",
        contributors = "danielbayley"
    ))]
    Dock,
    #[cfg(feature = "dog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgyDUIAxDaDg1GiDQ1EwMQyhENx6CIPg8C+A4FiCI4GgiCoShENB2C4NYfiGJolgSJ4JE0NAuDSG4Mg4NA3EEMQzC4MQ1juQpEhsMAgkqSg0iuLg1DYQ5ODEOAuDcMg4CAN4uDiGwyDGEZfDEcw4C0MoOlgIJmhMNJuGGPA3DCO4NnKSZLksLY5DkMwtkIMwwDmMIijSM4FgeNpaDGLIuoOMoCjSiIKlaEKUGOegzDiVAuDANQtg0MA4DMIJonOZpoDObqki6nwunydQyDKfouDcNp6DIOQ3rMNpRp+eoMn2rg5psMQ3rqEw1qMNAtDeEauDKwQzmAIJCryDaahC1Q1mGaLQDUQZcDSuZbnuzZMhGKwgDWaQ3GMMKtDMObOpqy5oDUOZakKta6t2xqrl6Qo+CCOLACCEwwDANsDlueqhlcMJ9rCEKwrK2sKDGrQyDbGI5lirQ4DWnoNDSnrdvGZ6NiChIkgEA",
        categories = "animals",
        tags = "animal,pet,puppy,hound,canine",
        contributors = "kemie,jguddas"
    ))]
    Dog,
    #[cfg(feature = "dollar_sign")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHiCQiDGDYPDGCoTg4eYWgsIg+DwL4DgWHhwGEdBoCAZIKE0MQ3CANRIDkLg1GEM4yCCNQ1CAMI6jyOw3GiM44jeNo7jsMY8DcSA2h2H4kiYPoBA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg1CAMQwGEMoOCCFIPDAIIZDELYUDQLQzEgMQ4hMLg5g+FInhqK4chQNgtDQLg0hCG4ZhuKw2C4MwgDcLgxGMLY6C4MpBDiOgtDGQ46CIPg8C+A4Fk0YxpHIYxsGUIBjHiCQxDIIpZHmXJeCAcoJDOTJOlOVZXD6AQ",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = "danielbayley"
    ))]
    Donut,
    #[cfg(feature = "door_closed_locked")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIaAuDAMQiD4PAvgOBYWhmBoIgoMQ4CAORWDYYQyCCJ4NioLQyiwSA4iaKAgiqM4sigdgxDSFYXhyG4Eh2CRNicMgwGgOI7hiP4+gWB5BkSDg3HaLIximDo1DSMx2DKSI9DwchlGMdAgGgZRpGcaB0gkNQiCAch4gmFAgHcaRkgWCZHCCbwijmbB5nAN5Il+YQ+gEA",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit,lock",
        contributors = "karsa-mistmere,lukedukeus"
    ))]
    DoorClosedLocked,
    #[cfg(feature = "door_closed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIaAuDAMQiD4PAvgOBYWhmBoIgoMQ4CAMgwFYNhhDKIggg2KwtDKLRIDiJ4piuKotiIdgxDSFYXhyG4Eh2CRNiiIxoiOO4Yj8PoBA",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorClosed,
    #[cfg(feature = "door_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDASAyCIPg8C+A4FhSF4GgiCoMCANAuDUNgyHYMQ2C4MQ1DcYYNg0MAgi+LwxC4Mg0DILg5DcTAxDmDgwFYNYhiMYQyg6MJHDALYzDWKZKjgNBsC0NJKEGRZFjGR4eiCIgxHqE4VhqGYEhuCYLg0NBIDiRJGliSZWiUNJfhaY5igWB5lDENAgDEMhoC4MAxnKYYCmOd4KDKVgwGgLQzoKdIB",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorOpen,
    #[cfg(feature = "dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMgug2Dx5hKFIND4PAvgOBYHD6AQ",
        categories = "shapes,text",
        tags = "interpunct,interpoint,middot,step,punctuation,period,full stop,end,finish,final,characters,font,typography,type,center,.",
        contributors = "danielbayley"
    ))]
    Dot,
    #[cfg(feature = "download")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDUVgzCIPg8C+A4FhSF4GgiCgyDGDg1HYNBhg2DQwCCJwxC2JRIDWJAgiaKIOiuKx2C0NIThWGoZgSG4JG0N4OicNQgkQNQtDWOYWj0PoBA",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[cfg(feature = "drafting_compass")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg5DkIA2C4Nw0CAMYODMIAzC4NA0CIPg8C+A4FiCI4GggIhNDEOQuDEMw2hYMhhDEMIWjWN4WC0MQ0C4Mg3DEIAwh+IYmiWBIngkbQykCSwtg0MQ2C2Gw4h6IIikeRoFgeSYZksIA4C4MAyjqPAyDaQ5XiQPBjGkchjGwZQgGMeIJgwIggHKCQyncYx5gkNZomybpwD6AQ",
        categories = "math,design,tools",
        tags = "geometry,trigonometry,radius,diameter,circumference,calculate,measure,arc,curve,draw,sketch",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    DraftingCompass,
    #[cfg(feature = "drama")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDEaAuDAMQiD4PAvgOBYWhmBoIgoMQ0CANoRhOFYXhyG4Eh2CYLDiIokhSFoYiqKYFgeLA2C4NYODMLoQhKMYnjSAoqjeCgyDIIA1GODQ5C2IQxDILQ2g4MhzlMLQzliUZMC2SZSDOIpZHOVAxiIIAziaM4akSNoegsNwuiEOQuDkYwtC4OJ5l4IJ7DKeQgDCaoom2K4fDCPggnEMRDDmigukmcQ3o+k5UnoNp3j2O5SDQLqTj2To9pMNQulSnY7nGLp0juepQDGkKPp2To6g6dZeomr5Op0N6DkOHJGE2dJmDGOZLC6WqvsOcgtq+k5/DSzJyr2GoBA",
        categories = "multimedia",
        tags = "drama,masks,theater,theatre,entertainment,show",
        contributors = "danielbayley"
    ))]
    Drama,
    #[cfg(feature = "drill")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDgYQxg4IINg2EoXHYMoRhOFYOC2FxIDUYQzCCJIdDELQzimE4SieHgxHoIg+DwL4DgWM42gaCIKDGJIMEgNBhDIIJDicLQykcVpBkORYUhOSAyGgOYbi2ToXg4dg2lSToWh+DhsC0Lg4hIMwuDINIaleRguDkNwuDcNQ4EgOIyjSOY4gSOoJgsNAgDQaAzluLoYmmHJWl4MRoimdY1nmeIFgee4PCANhoDSjJ3gKeaRCIbQ1g4MJHCCdIzo2N6apCOxtDeDg4kQLaknajoB",
        categories = "tools,home,devices",
        tags = "power,bit,head,hole,diy,toolbox,build,construction",
        contributors = "danielbayley,jguddas"
    ))]
    Drill,
    #[cfg(feature = "drone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMCAN4QCIPg8C+A4FhSF4GggIhtg8MQ0C0MwgDOE4VhqGYEhuCYeDSDoNDOIomhaKopgWB4siCDouiOJYUjSGICiqOIKiALgyDANQgDQLgxDMORhi6LoNDGDggDULg0k+VwuDgNo+ieNZCjeHILDkLpfhGIJRksIJUg4LZYloMpcl4OIziiY4rgqTAzDaagwmyU5WlWcgzDicZdDYMp4mKGpEE2Zw3DmSgxmeXgyoKbqEokNAyDmiQ4DeYJAGiFByGUYx0CAeIJgwIggGgZRpGcaB0gmdwgHKrQiDGsB5risB3GkZIFgkNIzqiqg+gEA",
        categories = "transportation,devices",
        tags = "quadcopter,uav,aerial,flight,flying,technology,airborne,robotics",
        contributors = "bernatfortet,shopped,karsa-mistmere,jguddas"
    ))]
    Drone,
    #[cfg(feature = "droplet_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg3DENQgDEMwugwNhDgwLgyDmEgxC4OA1DiEg3C4Mw4DSEgwC4NgwDeEg2CAOQuDUYwtDILYeDYLYUDULQ0j4LQ2jMYQxiqLpFg4IAwkqEgth+JwgDKMw4DYIg+DwL4DgWV5agaCAiG0MpRlGSwyDCVpYl2XIEl6CRNg0Nw5hGcA5DcQQxDGHZMkueYijINRDjCeIVCCEYToWEg1GGLoukujoShQMqSCCkQwDiaJZmwPoB",
        categories = "weather,gaming",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "colebemis,ericfennis,csandman,johnletey,jguddas,Footagesus"
    ))]
    DropletOff,
    #[cfg(feature = "droplet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYQ3CCEgwCCFYVDcLQ3GMMAtDILQxC0MwuDmIgtDULg1HOIopC0NIuC0NopGMLYpg6LYNDSJIuCCMg1EMNggDEMQuDEIA1kIM5HkINYRhOFpQhiEx6CIPg8C+A4FD6AQ",
        categories = "weather,gaming",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Droplet,
    #[cfg(feature = "droplets")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ2C4MxjDILgyCAMAgDQLQxC4OAzhgLQ0C4MA1haGgug4LQuDUNwthMMg2iYNwxC0M4nDkUw3hQOQgg8N4kgwNYQGOKQyjuGw0DWJgxDQIITDiGYujuNQ3DYU4eDGGwxCCV4uDUY4Xi6FYbDiGIiiSTIhiMegiD4PAvgOBZtnCBoIgoMYTDUNo8C4NhBDEMAuDmDJ/oGDIXoeDZMjUMAyGOKpNo+FYhjuTIPDUc4ejWJIekENRhg8OZkqCZKIjOWKBlqkw3mybpzD6AQ",
        categories = "weather",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "Andreto,ericfennis"
    ))]
    Droplets,
    #[cfg(feature = "drum")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDA4CAOAiD4PAvgOBYThaBoIgoMoMDILYPhGE4VgQaITGUbBsGkcBzGUIBjHmCQ5CIIByjEIg1jQch4gkMQwjQY48CIMQyhKFIoiqLBlhiJYHgkTQ3CAMQzC4NB2DcLoziOGZMgWTgiE2RJSlaIoUlyApNhuYZRlOVZXlmRokheaJemqDA5HYOBhj4IA1CAMJ/oEMqADAVpamaJQ+gE",
        categories = "multimedia,devices",
        tags = "drummer,kit,sticks,instrument,beat,bang,bass,backing track,band,play,performance,concert,march,music,audio,sound,noise,loud",
        contributors = "danielbayley"
    ))]
    Drum,
    #[cfg(feature = "drumstick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg0CCDAuDYMxhDcLg4DcNQgDaEAzhoMYQhsLgyDMLQ2iMMwgDQLoaDODopDGHggDCM4miiIokCIPg8C+A4FjuPoGggIhtDiIw5hAMguDcMQtkqHJPGGSoalOM4hDALQxhKGorDYNRBlWVY0iCNJGDeEJGDMbJTDmTosDmOo8kEPoBA",
        categories = "food-beverage",
        tags = "food,chicken,meat",
        contributors = "Andreto,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Drumstick,
    #[cfg(feature = "dumbbell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1DkNggDEMguDcNg4GEMgghoMISCCHYUDgMg5C2IYjGwLQxhWF4pisN4ZhuH4yiALoiDiJY1ieOI2jsMoYhqHIzj0OYbjkOIoiqFg3i2Fo/jGHQxh+Q5FiIOR6CIPg8C+A4FlmXIGggIhthQNYbiqZYqDSLQ0liWpfl6BJggmYwwC6UQzC6RJpmubZbnGcIFgeCRNDULgzDQM5mC4NA4DWMJBlGNJVj2R5JDYN4SiuTqQjKJokp6KA2oYNpqqIM6ko+MqRlOoKWDimZKqmHazpSVIjleWZ+l2ApxoKYg5C6EQxDSiwgsSN7Hn2b4B",
        categories = "navigation,sports",
        tags = "barbell,weight,workout,gym",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Dumbbell,
    #[cfg(feature = "ear_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ4C4NRhDOEAghMNQgDCDYYCAN4YGMMAtDGEA3C4OQyC0MoQDIIIpDANAthMNA2CIPg8C+A4FjWOIGgiCoMg8NYfC0Lg3DULgxDOIQuDSJAzDaKJHDSNI2juOoEjyCRNg8OIVkcNRBDaFJhheGYZDGDQ5CCQIfg0Lg2kmSw0iwLg4DGSgwmmEw3jONY3leVoFgeWQximFw2EGhZzmSG5nDGF5AE0MZmDMYYriuZaMnSJ4ig6U5+jkPBsGkbhlCAeQxgkMgiqYMqpqoIB4qgIqvHirazqqfaiqQPoBA",
        categories = "medical,accessibility",
        tags = "hearing,hard of hearing,hearing loss,deafness,noise,silence,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EarOff,
    #[cfg(feature = "ear")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOAuDUYQ2g8IISDUIAwCAMYZhkM4XGOGA2C2DIhgwMQwGEM4TimFoYhoMQtDeFwiD4PAvgOBY0jeBoIgoMYWg6EAyhOQosheFwtiwdgxGEMggk2LYbhgNIzjWOg+gE",
        categories = "medical,accessibility",
        tags = "hearing,noise,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Ear,
    #[cfg(feature = "earth_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMwuDMNBWDUYQzg0IAwheGYVDMIg+DwL4DgWHohgaCIKDEMQgDIMQuDkNRWDEOBhDKKoZhgMAtDKOY1jSNwgDGOY5HYLQxjONY+jiOgyEgMguDANYdh+JIjgSJYJE2KwuDUNI/DUSAxDeRo9jaOwyHYNJaDSUYglWVIFgeVwxjQMhhDGGJ2hmKYYDmaY/hyHpsiKApVnCCgyhgNhWDSYp5hcLZcDAdgymuUw8HIZRjHQIB5gkNgiCAaBlGkZxoHSCZQCAch4gkMafHcaRkgWCQ4p+qwiDGaqApemQ+gEA",
        categories = "security,development,devices",
        tags = "vpn,private,privacy,network,world,browser,security,encryption,protection,connection",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    EarthLock,
    #[cfg(feature = "earth")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg1DQIAxDUSAxDcYQyCCGAwCCGwwC2GAyHYNIODQIg+DwL4DgWJ4qgaCIKDcIAzC4Mw0FYNRhDOMocjyG46DOF4Zj2EZCDIY4bg2DQ5kWGZBhqQwyh+RwtkkLg5h+GYfGiM4ViaKItiyBIugkTQxDGGZKDUVgxDiTpDh6UQym6HYRh+Hx2lSc49nYMhIDILgwDWXopmKJxjGkchjGwZQgGMeIJDEMgiCAcqQDCkxjHmkKSicL6Homiw+gE",
        categories = "navigation",
        tags = "world,browser,language,translate,globe",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Earth,
    #[cfg(feature = "eclipse")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTYUCAMhhDcIIuDAIAxCCMYNjKDobC+IokD6AQ",
        categories = "science,design,development,accessibility,photography",
        tags = "lunar,solar,crescent moon,sun,earth,day,night,planet,space,mode,dark,light,toggle,switch,color,css,styles,display,accessibility,contrast,brightness,blend,shade",
        contributors = "danielbayley"
    ))]
    Eclipse,
    #[cfg(feature = "egg_fried")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyC4NQiCAcoMDOEISGMeIMDEMYXD4PAvgOBYHh8cBhHQaAgGSDBNDMIA4GMMAthYNQgg8NQtDYIA2hCOQgjUMAgDQLg4i6Lg3hCPxzjUMo/juMZCj2N47j0N5UjiU4ylON4+jccwtDcLQymALQ1hCMYzCCHY4i6a5lEONJqkGLg5CCLg4FoIofC+JooD6AQ",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNg8IAxDQLgzDQNxWiEYwwC0NgtDQLQxDILQ4i+MQxC4MA3DgIIqDILgxDUNwuDQM4sDOPpAiCPg5hOFYahmBIbgkTQ2C6Dw2CCVAyDEQ4iDgNwxCAOJCCANIgjaDJliEYY6jqEJuiCIg1DaOpxnOTIWlAPoBA",
        categories = "food-beverage",
        tags = "egg free,vegan,hatched,bad egg",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    EggOff,
    #[cfg(feature = "egg")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyEMOIOCANAghGFAxDQYYRhEMAgh2HQxDaHhjDALQ2C0NAtgwLQ4ioMgiD4PAvgOBQ+gE",
        categories = "food-beverage,animals",
        tags = "bird,chicken,nest,hatch,shell,incubate,soft boiled,hard,breakfast,brunch,morning,easter",
        contributors = "mittalyashu,Andreto,ericfennis,jamiemlaw"
    ))]
    Egg,
    #[cfg(feature = "ellipse")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch5D0Ig2CIIBjgwIgxDKEByHiDQxDCEBjhmFIWD4PAvgOBYHGUPoBA",
        categories = "shapes",
        tags = "shape,geometry,rounded,smooth,outline,form,boundary,curve,shapes,ellipse,oval",
        contributors = "KISHORE-KUMAR-S"
    ))]
    Ellipse,
    #[cfg(feature = "ellipsis_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwiD4PAvgOBYHhmHIGgiCoTg+EYMDWFYXhmG4EiGH4tgeCYLg2JRjhKDQ5imDYYhqIIegE",
        categories = "layout",
        tags = "menu,options,spread,more,further,extra,overflow,dots,…,...",
        contributors = "colebemis"
    ))]
    EllipsisVertical,
    #[cfg(feature = "ellipsis")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwiD4PAvgOBYHhmHIGgiCoTDmFYXhGE4PhmG4EiGH4tgeCYLCINYQGOEoNg8IIWg2GIaiCHoBA",
        categories = "layout,development",
        tags = "et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,coding,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "colebemis"
    ))]
    Ellipsis,
    #[cfg(feature = "equal_approximately")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ1GENgugyEYMDAIIWDEIA3hcIIUh2EochaFobDAIg+DwL4DgWJ4qgaCIKgwOYQiCHoig2Goch6NYhjiJYnimBBoD6AQ",
        categories = "math",
        tags = "about,calculate,math,operater",
        contributors = "ksk3110"
    ))]
    EqualApproximately,
    #[cfg(feature = "equal_not")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig1CKBwygoMQ5g0eYJCKEggHmD4WCIPg8C+A4Fh2IIGHiGoRhOJoMgeFYqhSEIMh2H4EGWIozhiJoXiWCotiyDYIhCEoxiMPoBA",
        categories = "math,development",
        tags = "calculate,off,math,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig1CKBwygoMQ5g0eYPCKEggHmCYWCIPg8C+A4Fh2IIGHiFYRhOGgxgyB4aiuFIQgyHYfgQZQ+gE",
        categories = "math,development",
        tags = "calculate,math,operator,assignment,code,=",
        contributors = "ericfennis"
    ))]
    Equal,
    #[cfg(feature = "eraser")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIIMEgOBhDKDggDCFQgDELQxC4NAyC0Lg1DgNxsC0MwuDkOQ0iWJ4ohKFIWhaDQwC0MguDgMg4GwMYzjuLoTjCGIOjYMg5hUbA1iyRQ2j6F4xheNY3DgTAxlAMw0g4MQiD4PAvgOBZbl6BoICIbZIDCN4YhsMJFDiQw4CCbZRlqXJhD6AQ",
        categories = "text",
        tags = "pencil,drawing,undo,delete,clear,trash,remove",
        contributors = "maxwellito,karsa-mistmere,jguddas"
    ))]
    Eraser,
    #[cfg(feature = "ethernet_port")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAyDAIAzC0MxoDIYQyg4IIQhuDgtDIVg2heGYcDCHoeEgNIihiJIeg4dg5iqGoyhCGAyhUbAzhEegiD4PAvgOBY9kCBoICITQ2CAOB2DGPI+kOQoEkSCRNDGEJKkyPY/lGUIFgeUwxDSSZLk2WpBgKUZekYMQ4mKWJOluAQA",
        categories = "communication,devices,multimedia,gaming",
        tags = "internet,network,connection,cable,lan,port,router,switch,hub,modem,web,online,networking,communication,socket,plug,slot,controller,connector,interface,console,signal,data,input,output",
        contributors = "ericfennis"
    ))]
    EthernetPort,
    #[cfg(feature = "euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGgMQyCIPg8C+A4FhSF4GgiCoMDENBoDmE4VhqGYEhuCRNDEOQgDYYQ3C4NwgjCMgwCCNgwC0NQuDILQyEGMIskGN5EjYNoNDIY42DQLoMDMLg1CAOIzC6U5TDKRJPDgLZVCCO49hKFIWicPoBA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Euro,
    #[cfg(feature = "ev_charger")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDMaAyGEMgghQMAghcMYVhUdoShSFoYiGDQwHYLQ2C4OQ5DiE4bheLgtC4NQ5C0MQuDQMhMDEOAgDUIg+DwL4DgWP5CgaCIKgyFQxFYNYsiCLwyC0MhIk2H4hlCHAxDaPpAkWRIEkaCRNh8MRog+XJBmCX4FgeYgzCAN5mDGaJegKYJtCIbQ5g4MZSCCEAzGyfgznSaoBA",
        categories = "transportation,navigation",
        tags = "electric,charger,station,vehicle,fast,plug,ev,power,electricity,energy,accumulator,charge",
        contributors = "UsamaKhan,karsa-mistmere,ericfennis,colebemis,csandman,johnletey"
    ))]
    EvCharger,
    #[cfg(feature = "expand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIIMCANoQCIPg8C+A4FhSF4GgiCoPDmEAtDaE4VhqGYEhuCRNDIMYODYdg1GgLQ1iOFoniaBYHimKwgDgVgzjGM4UjWGICieOQiE0M4ti8aJBiSNpFjiHBtkqOw2iGNIllGKJIkqPY/k6QxojeXBNh+H5KDOWY2gEA",
        categories = "text,arrows",
        tags = "scale,fullscreen,maximize,minimize,contract",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    Expand,
    #[cfg(feature = "external_link")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNh2DYIg+DwL4DgWFYYgaCIKDEMAgDENAgDIMYOhSFobhqBIcgmCw4iEM4SGEMokCCIIgDELY1DISA1jSNo4iGO47FYOJAjWQomDKO4QiiF4sD6AQ",
        categories = "arrows,text,social",
        tags = "outbound,open,share",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_closed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDgLQuDcMgyC0MwuDINQiD4PAvgOBYch+BoICITQyCAOBhDEMAuDYNINiuLYvCAMI0jYMo1DCG4diKIYEiOCRtjiDg1C0MYSDINgtDILgwhqHIej+PoFgeQQ0kSDpIkqTJOjuUYggKP5VgoOYODgIJIhWF4Zl6PYBA",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,hide,conceal,mask,hidden,visibility,vision",
        contributors = "karsa-mistmere"
    ))]
    EyeClosed,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg3DMMwgDULgwDcNhhgyDg0DQIIZDeGwgDCIYdh0MQuDIMA1CANguDUN4qDGJIiiKMYiC4Ng5DaHYNh8N47hqPozh0LYmhuHAyC4NA5CIPg8C+A4Fk2UIGgiCgxDSFA4hyVwuDENQ4GGEYRkIMQtlgMg0DKZonmmTJOlOUoElSCYLDeSQ3DmHZ2koOYYjyMJ/iONJEhMNAxDcLYTl6MaMoKMgtjeOY/i+k4qmQIJYhsNqJl0NAzm6T5ynGBYHgkbQyCCqIoqkMKgnCAQ",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,hide,conceal,mask,hidden,visibility,vision",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4MA2DIIAxgwMw0DgYQxhEIAwhqGQwC0Lg2DkNoRDALg3DWJImiiG4bhgMQ5C4OA3iOLYdhyGIbiCIopiePIrjcLYvjGM4aCIPg8C+A4FkcYxpHIYxsGUIBjHiCYSCIIBygkM5YGMeZWDKRpIk2T5RD6AQ",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,show,expose,reveal,display,visible,visibility,vision,preview,read",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Eye,
    #[cfg(feature = "factory")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYaAuDAMQiD4PAvgOBYWhmBoIgqD4OhCEoUhaGIEGiG4ngeCRNDODg5GGDYNDAII0jSMhoDENIxCCM41j8MgtDIVg4C4NRhkaRo/jYLQuDcNg5k0NAyDIbAtDQLg0DaMguDgNA0EGSQ1kuDoOmMMQwkYdpCkgNZKjaNZNk+UZZlQTA5k4N4OmkOZUmGbpjnAMQgDme5GFaR4ymQMJCkISKJj2i5Cj0eoVheHIpgWK4KDiIYRhOlomhqAQ",
        categories = "buildings,navigation",
        tags = "building,business,energy,industry,manufacture,sector",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Factory,
    #[cfg(feature = "fan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg4DINwgDENguDMNw5GGFAwg8IIahwMAgiAMQtDgLg2DEOAtDcLgwDAMhsDULg0DEMoSjINYZiyHIejWIIiCCK4tDKJImigbAtDGNwgjGMwyjmG41jyIZTDEIIlicOJAiyLpHkyNJIjeT47jqPZUiqW41leRpJDQNQtl4MhaCIPg8C+A4FnSd4GgiCo0hIMh2iwMZznWeg+gE",
        categories = "home",
        tags = "air,cooler,ventilation,ventilator,blower",
        contributors = "karsa-mistmere"
    ))]
    Fan,
    #[cfg(feature = "fast_forward")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2GGDYNDAIITDEIAzC4NAxDQLQxhmGxsDaDoQCCEoUCCFoTDILg4DIOBsC2Ig2EGEYnhWKINDEOB6CIPg8C+A4Fj6QYGgiCoNg+NYTjeGIahyHpOiGI5KieKYliyLowjKNIljaKJdjqPI+kCBBoD6AQ",
        categories = "multimedia,arrows",
        tags = "music",
        contributors = "colebemis,karsa-mistmere"
    ))]
    FastForward,
    #[cfg(feature = "feather")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg2DcIAxDkYQyCCFQwCCGIYDELg0DENgtC4NQ4DgbA2C4MQ1DQLYnDENwyGENggjKGoZC0OIdDmN45EwNYiDiMg5C4OQxDQQYVheGZKDWEYcDMMg4FYMQ4GEMYRkqNZWDEegiD4PAvgOBZemGBoIgqHwgDiFoWDKXZfmSY4EmWCYLDeIoRDUSA5m6YJyD6AQ",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[cfg(feature = "fence")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMwgDIIA1HYMQ1GMMAgC4NgugwMQgh0MRoDIY4ZCCFwxC2G4eC0MRWDUWgiD4PAvgOBYxjSBoIgoNggDgaA0jCMo3jaBI4gkTY7DGPY/jGM5EkOBYHgkbQxhAMwthAMoThWF4ZimH4eiGI47iaKIciuLYvkyQoCkSUYKDGDJKkCTY1myUI5E2cIenKapOnaRQiG0MoXlaWJahaGIahyHpgiKJJkl6Z4unOQoBA",
        categories = "home,buildings",
        tags = "picket,panels,woodwork,diy,materials,suburban,garden,property,territory",
        contributors = "danielbayley"
    ))]
    Fence,
    #[cfg(feature = "ferris_wheel")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKDIPD4PAvgOBYHhkcBhHQaAgGSDBNg4IAyHYNAihkL4giKH4hiOJQiG0NguDgIAxDULQzC4NYpi2GowGiMoiiSDBtDIMAuDcIA3j6QJCi6RZHjSJo4joOQgj8M5QkOL4zleSY2kyTo7lGP49hiRJjDyRZlG2XAyDKXQtjqXw4mGVpwjOZRNjqdRonuVZvnGNYnjoMQ4k4YZclwMI7CAMAtigMJ8mOAQ",
        categories = "navigation",
        tags = "big wheel,daisy wheel,observation,attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    FerrisWheel,
    #[cfg(feature = "file_archive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg2DUOQgDIMhIDEOBhDKEggDCG4dDILQyFaFwyC4NISiWHYcDALQuDcMA2C0MYti8bAtg0NQ4DiNgujgOBBiSJpAimHQxkESA2hiGoqhuIISHYMYyDUIg+DwL4DgWVJXgaCIKkWTg1GEMQgmKS4cmIMRolKVJWgQaJZm2B4JE0OJjDIdoxlOVZam+BZxgqdIWncMp5myWICnCXJzCANxWDahJ7DwYxpHIYxsGUIBjHiCQ4CKmB5gkMgwp0cqgoSkqUpYPoBA",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileArchive,
    #[cfg(feature = "file_axis_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkbY6DGOg0C0NJqlWbptgWB4JE2cgwHYOIknibIBA",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileAxis3D,
    #[cfg(feature = "file_badge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDIaA1GEMoOCAMIWhgMgtDIVg4hMLg0g6IIYhcMAtC4NwwDYLQxiiKhsC0MwuDUOA4jGM41EEMojjuIYlhgMYhDISA2hOFY/iaFAyHaMgzCIPg8C+A4FlCU4GgiCpBg4doSDEIJekiX5fhGT5RlaVYEleCRtDcLg2DmXw2iAN5wi0MpwDQLo1GGM4ziSX4nm8OIzDkMYwi0OA0DOJ6JDkYZemCJKMnQLgwDChosnoNJyDiEp9DWfwxoEOYaoQMxsnYOQtnmNZllKaZQGMaRyGMbBlCAcoJk4IBjHiCQ2CKvB5gmQaurKtK2D6AQ",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBadge,
    #[cfg(feature = "file_box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMgyEgMQ4GEMoQCAMIZhsMgtDIVoVDILg0hCI4bhoMAtC4NwwDYLQxiuLRsC0M4ODgOI0jYOBBiKJI9ieG4MhASA2haGIohmHoQHaNQ4CIPg8C+A4FlCU4GgiCpCDIdg1GEMQgl+SIal8MRoDWT5RlaVYEleCYLjAN5gg2F5xDEN4zg0N4eC6TpQlKbJrgWB5uDOYI1DGRoXmKKg5o2YIrDaTAuDINKJkCGguDkNg5o8Nw4EwNoQnClpiiUMI1DAMRMDGZA5pmpJADGL6QFaDKvheionoyepwp8OJgjAM6wiie6niqqR6mif5UgKbKDgqdQ3lyypqgEA",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBox,
    #[cfg(feature = "file_braces_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyDIaA0GEMoOCAMIWhgMgtDIVg4hMLoNDKIIYhcMAtC4NwwDYLQxiiKhsC0MwuDUOA4jGM41EGIohiOJYYgyDhIDaE4Vj6JoUDIdg2CIPg8C+A4Fk2UIGgiCpAkkNRhDEIJbkaXJcGgNZMk6U5SgSVIJE0NZchKW5diSLJcHYMpal+PgxnGbp2j+XwxnOdZvneXJjk+Z5mgWB5pDmDp0nqXp4n6G6AiSfZ5nuF6QiwdqSo6cKQoSZYBA",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBracesCorner,
    #[cfg(feature = "file_braces")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxiCD5jiuEwtmOSZikWTp4kWdZOn+eqAnaY5qlWbptgWB5wl2FoloSfQxHaeJ7mSFZjn6kYWnieKUDGlp2DCnQxoebIB",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBraces,
    #[cfg(feature = "file_chart_column_increasing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTY6DGSIYmqVZum2BYHnCSoWnQNJ2myApunuXIMnMdgtDagZ4gEA",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation,trending up",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartColumnIncreasing,
    #[cfg(feature = "file_chart_column")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTY6DGSAtDGapVm6bYFgecJKhadA2nebICm6fJcgycx2C0M6CnmAQ",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartColumn,
    #[cfg(feature = "file_chart_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkbQxgwMQzC2PI7jeGIYEyOgxDeapVm4PoB",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartLine,
    #[cfg(feature = "file_chart_pie")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg5DQMQgDIMhIDEOBhDKEggDCG4dDILQyFaFwyC4NISiWHYcDALQuDcMA2C0MYtDANBsC0MwuDUOA4jeOY7EGJImkGKYdDGQhIDaGIaiqG4ghIdo4DUMQyCIPg8C+A4FlaWYGgiCpGk8NRhhGEZMhyZBoDWVZXlyW4El2CRNDQLgwDENwgDGMpSDIYQ2CCfpnh0OIlDafqDDQN5qlaWJvm6BYHnEOZ4kmZJEDGMYxHYLQ0GOK45DWQQ1jGdAwoMOQ5qKDp2n2f6WCCcw5hOr4OhMY50Dmeg0DSLAzDaDanqmuA3g6qB6mujJagEA",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileChartPie,
    #[cfg(feature = "file_check_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CAMgyEgNhhDKEAgDCGAgDELQyh0Vg0hWF4ZhkMYQh0aA4hULg0hCLIaiWGwuDcMA2jONRsDODg4DgII6DWPBBDKL5Di2JIbhCGQ4HYNgiD4PAvgOBZPlKBoIgoMYtDIdg1GGJomkeMQxGgNZOlCVZUgSVoJG2WZJheFg0C0NJmlGag+gEA",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCheckCorner,
    #[cfg(feature = "file_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkbQ5hYNYShENAtDSapVm4PoBA",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,danielbayley"
    ))]
    FileCheck,
    #[cfg(feature = "file_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGEMoOCAMIWhgMgtDIVg4hMLg0g6IIYhcMAtC4NwwDYLQxiiKhsC0MwuDUOA4jGM41EEMojjuIYlhgMYhDISA2hOFY/iaFAyHaOw4DUIg+DwL4DgWUZUgaCIKkGDh2DUYQxCCYJImGYRok+UZTgQaJWmqB4JE0OJhDSTAuDIbItg0MZQlKV5RGMaRyGMbBlCAYx5gmDAioUeIJDiihygkNp7C+f6BoMPoBA",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    FileClock,
    #[cfg(feature = "file_code_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyC4MQ1FYNBhDIIIWDAIIZDGFwtDIaA4hULoMg+DIZhuDQuDcMA2iqLBsDMLg1DgOAgjGMw4EGJYXiOGo+hwMoZDgdoOhWF4+huHoXGgLYxDMNQiD4PAvgOBZSlWBoIgoMYkHYNRhhyHInj+DRolCUpUgQaJXmqB4JG0NYNDaTY2nUM5RlOWJsgWbgiG0OYXhYM5NoSd5onqAQA",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCodeCorner,
    #[cfg(feature = "file_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxiCQA1CCOgxDUbJMjeapVm6bYFgeCRtl2Fp0hKdIYimaZTn6V4B",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[cfg(feature = "file_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA4GEMQghEMAghQMQthcMRWDIYQyC4NAgh6IIUhaEguDcMA0icMA2GwMwuDUOA4CCL4xDgQYiiGH4VjyEQyhQOB6CIPg8C+A4FkSR4GgiCo/g4dgxhwMohjyFgtlMMhoC2KgxDgMpDkWSpJgSS4JG2LwzDCDQxDmMIvDkMgzC0Lgzl6YJGmSY4FgeCRNiAMQwjCcBWDSHZUiSEohlcaA4neYoCmSfAiG2KgyDKMwxDYLg4DUMpznANJznUM6OnmkJ7kwbQ1punYSqulg4qIOJyC6cKkkSeJIqeZaTqunJYoENw3p6dKzrUMg0qWupKpIbQ4C4MQ0pir6XsWtK2soaJ6ryzpuiEMQuDYOQ2rKxKgtm27Nm2wgzhKmq/sexKjuiu7qicN7tmy0LSvG1r0GMaRyGMbBlCAcoJqQIBjHmCZdCLCh4gkN53wDAsED6AQA",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere,danielbayley,jguddas,UsamaKhan"
    ))]
    FileCog,
    #[cfg(feature = "file_diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCg5hYMBoDaUpUlmWIElqCRNkqFgzFYN5ilWZplgWB5ol0MQ3mCb5kgE",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDiff,
    #[cfg(feature = "file_digit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyFYNBhDIIITDAIIWDGFAtDIaA4hILoMDKIIXiSGQxC4NwwDaKIqGwMwuDUOA4CCL4xDgQYiiGI4WhiFIWDgdoOhKFIkhiG4UCIPg8C+A4FkqTYGgiCgxiEdg1GGJpFiWDRoDWSZLlCT4ElGCRNDGGA2GgMh2DaX5MmOYoFgeZZnhSHA0m6YQ8HIZRjHQIBoGUaRnGgdIJm0IB5gkMaIHIeIJDIIggo8IqRCAdxpGSBYJniSgvnyfg+gE",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileDigit,
    #[cfg(feature = "file_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTZKhaSAtDaapVm6bYFgeCRtDmFg1juggzC0M53myAQA",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDown,
    #[cfg(feature = "file_exclamation_point")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCpKCAOR2DSUpUlmWIElqCRNl0MQ3GgLgwDGYpVmYPoBA",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileExclamationPoint,
    #[cfg(feature = "file_headphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANguDgMw1FYNBhDIIIWDAIIZDGFwtDIaA4hULoMDKI4aieHAxC4Nwwg6LA2GwMwuDUOA4CCMo0DgQYliSJoZhuF4ZDgdgxDKFYXieG4ehcaAtC4Mw0DMIg+DwL4DgWVJXgaCIKDGJB2DUYYpkmKAgDEaA1lOVZalmBJbgkTYWDEOZHhiZYMDCRJ1mQMQtngdp+GENoNnyZoYHaFIWnaSp/C0MZ7j+ZggniapWm4PoBA",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileHeadphone,
    #[cfg(feature = "file_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDIaA1GEMoOCAMIWhgMgtDIVg4hMLg0g6IIYhcMAtC4NwwDYLQxiiKhsC0MwuDUOA4jGM41EEMojjuIYlhgMYhDISA2hOFY/iaFAyHYNwiD4PAvgOBZPlKBoIgqQYOHaEgxCCXZIl6XoRk6UJVlSBJWgkTYyDaFAxDgLg4joLgyDWIp1kCYQ3l4NZxDORY7nigZ2heXZdjIMw4iIOQ2DaMI7m0NoiDgNZFoaJJeiyMwwnsMB6mSUZoD6AQ",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileHeart,
    #[cfg(feature = "file_image")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZlMYxpHIYxsGUIBjHiCQxDAIp0HmdwynocoJn6Uwvm6cJyliBJagkbQyiANwtDELgyDkNqQpKlImDSIZCmSKwtjYNAwjoMBMDmDaCmuiQ+gE",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileImage,
    #[cfg(feature = "file_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxFYNBhDIIITDAIIWDGFAtDIaA4hILoMDKIIXiSGQxC4NwwDaKIqGwMwuDUOA4CCL4xDgQYiiGI4WhiFIWDgdgxDKEoUiSGIbhQSA2kSFYlhuGx2C0MQiD4PAvgOBZVliBoIgoMYhHYNRhiaRolg0aA1lSVpblqBJcgkTYTDENRoDEMJqlebptgWB4JG0OYNjMMwtoOhZ4myAQA",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyHYNRhDEIITDAIIWhaEwxGgNQiD4PAvgOBYfiKBoIgqDQxg8NoeiCJYkgSJoJE2KQ0GgMotiGMYwgWB4zDkLg2DWDgyEgMQ4GEMoOheTIWDILQyFaSAyC6DZUg2GIXC0Lg3DANgtDGXJeGwLQzC4NQ4DiZZnmkQZXg6VZNkyDIOEgNpJkuWQwlCDh2DSOYvDwYxpHIYxsGUIBjHiCZ/okeYJDIMAiCAcqQjmg6FocPoB",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileKey,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAOQuDgVg0GEMgghQMAghcMYVC0MhoDiEwugwMohhiJYaDELg3DANopisbAzC4NQ4DgIIwjIOBBiOIokheGYVhcOB2DEMoThWJYZhyFRoC0MwiD4PAvgOBZPlKBoIgoMYiHYNRhieR4mCAMRoDWTpQlWVIElaCRNDmYQ3HaHJFhaRwtgwMB2DKZZRmmTxyGUYx0CAaBlGkZxoHSCZkCAeYJDENwiCAdxpGSBYJDikB4gmTQgHKmQiDGep+oAPoBA",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileLock,
    #[cfg(feature = "file_minus_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDQVg4GEMguDQIIUhaDYaC0Lg3DANgtDGHYfGwLQzC4NQ4DiJooioQYYheFQghqM4OhYMhIDaE4XjWGwyhcdgxjqP4/jSDZEGgMQyCIPg8C+A4Fk2UIGgiCoPkANRhDGDo9jWWwxGgNZMk6U5SgSVIJE2VwxDgaA2mOT5nD6AQ",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileMinusCorner,
    #[cfg(feature = "file_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQ5hYNRoDaapVm4PoBA",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileMinus,
    #[cfg(feature = "file_music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg2DUIAyDISAxDgYQyhEIAwhqHAyC0MhWhYMguDSEYkhyGwwC0Lg3DANgtg2LQ2GwLQzC4NQ4DiNY3jkQYjiWP4ohwMZAEgNoXhmKYah+ER2DEMAuDMNQiD4PAvgOBZVliBoIgqRJNDUYQxCCY5KhuYwxGiU5VleBBolqboHgkTQ4hEMB2C0NxsDOZIkDcNJUlaW5VGMaRyGMbBlCAYx4gkNgioseYJDIMKQHKk6BC+haHokPoBA",
        categories = "files,multimedia",
        tags = "audio,sound,noise,track,digital,recording,playback,piano,keyboard,keys,notes,chord,midi,octave",
        contributors = "danielbayley,jguddas"
    ))]
    FileMusic,
    #[cfg(feature = "file_output")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4MgyDYIAyDALg5DINRBDKEQgDCG4dhCDhoDEMhhhmGYcieEQtDIVg4iQLg0hGL4dicLQuDcMA2C0MY2jgbAtDMLg1DgOI/kGQ4YjIMoyiiHAxjAMhIDaJIakyKoRHaQIiDcIg+DwL4DgWXZggaCIKk6Vw1GEMQgmuTJsmwaA1lyXpjmKBJkgkbQ1mwMY/CAM5zl+d52gWB55nsMQ3j+P4hDCgZ1gEA",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    FileOutput,
    #[cfg(feature = "file_pen_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLgzDYNAgDEMwuDYMw0GEMgghoMAgh0MAtC4NQwDYLg4DUNBsiEOAzDeG4mDcYYiiKHo1h2FQyjgbI5DgN4ri2GYbjaNYmiiIYjDYbINDCEwtksMA5GEMYSkOIIUDAMA0C2V5ZHoIg+DwL4DgWYJjgaCIKgwLg0j0IA3kUOBBlOU4flSDJuFYMpfmGZplgSZ4JE0ModDEOYVDQNZ5DCQYcjUMQthoMhIDajJDo8MqQFaGKRpaG6QGgOIZmuL4RnWcwuDeWaoiSO4iDENovDWr57mKf5+gWB6BDiEg4GgMa0n2AQ",
        categories = "files",
        tags = "edit",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    FilePenLine,
    #[cfg(feature = "file_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg2DUOQgDIMhIDEOBhDKEggDCG4dDILQyFaF4NDSEguiWHIpC0Lg3DANgtDGLIuGwLQzC4NQ4DiNY3jkQYkiaKIdhwMYlhQNoYhqKYbiCEh2DkLgzDQIg+DwL4DgWVJXgaCIKkSTQ1GEMQgmKSpDmMaA1lOVZalmBJbgmCwwlANw4mODQ2hOYZjkKewzCCNgwDAMxMDiUA2hKcg2DMN5IhmSgxisOA1DSN4ziALg4DYN6YosYY3jefKQg6H6jGynIvg2mQ5o2oQgpWL6YDUMx6mqVpuD6AQA",
        categories = "files",
        tags = "signature",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePen,
    #[cfg(feature = "file_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxDULgwDMM4WjYNIcC4Ng0DefJ+iuRYgC6ShsC2MYzDWKQzDWD6AnOfaLk4LQuDkNg4pUNQ2HaiIynukqQoKIaWDYN6aDaUZTlWbg+gE",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePlay,
    #[cfg(feature = "file_plus_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgzDUIAyDISA2GEMoRCAMIZCAMQtDKHhWDSFoYhqGgxhGHhoDiFguDSEYthuJocC4NwwDaNI2GwMwuDUOA4CCO49DgQQyjCRYuiWHIRhoOB2DWDg1CIPg8C+A4FlOVoGgiCgxi4MpOGGJ4nkmMgxGiUZTlWBBolia4HgmC4uDEORoDaUpUlmbYFm+XA3hwNh2naaZ5gEA",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FilePlusCorner,
    #[cfg(feature = "file_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQ5hYNRoDaapVm6bYFgecJKhaSAtnaU54leAQ",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FilePlus,
    #[cfg(feature = "file_question_mark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCpKhYNxoC4MAxlKVJZliBJagkTQ5C6IQ5GEM47iuRQ1C4OIRDEY4UhmcQzC2fZklWaA+gEA",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileQuestionMark,
    #[cfg(feature = "file_scan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAVg4GEMguDQIIUhaDYaC0Lg3DANgtDGHQwDQbAtDMLg1DgOInimKxBhiF4VCCGo0g6FgyEgNoTheNobDKFx2DGO5AkCNYNkUaA0C4Mw1CIPg8C+A4FlCU4GgiCgxjgdg1GEMYOj6NpfDEaJOlCUoEGiVZpgeCRNkON48kaPgtkWT5Rlaa4Fm2WQ2hcMpymEMZ1nWd5olSApsliC4NlqgY1l+dpnnmiZ7ouDJ/o+YQyoWk5pD6AQA",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileScan,
    #[cfg(feature = "file_search_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgxCAMgyEgNhhDKEAgDCGAgDELQyh0Vg0hWF4ZhmD4eDIaA4hULg0hCLIaiWGwuDcMA2jONRsDMLg1DgOQgjqPA4EEMovkSLYkhuEIZDgdo6DINQiD4PAvgOBZSlWBoIgoMYtDIdg1GGD4PkiMQxGiUJSlSBBolea4HgkbQyiaJwuDgOIdnUOJRlOWJSGMaRyGMbBlCAcoJDMIggGMeYJDEN6JGMeKNDaewvn+gaDD6AQ",
        categories = "files",
        tags = "lost,document,find,browser,lens",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSearchCorner,
    #[cfg(feature = "file_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZlMYxpHIYxsGUIBjHiCQxDGNwinQeZ3jGaQgHKCZAmmUwvm6cJyliBJagkTQxjYM4WDYLqSDENYWDiapVowPoBA",
        categories = "files",
        tags = "lost,document,find,browser,lens",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    FileSearch,
    #[cfg(feature = "file_signal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTY6DENRoC4MAxmqVZum2BYHnAMQxjeFo8iYNYpoaToVDOeZsgKbp+lyhpKGGhqIhWZAgDajJ7gEA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileSignal,
    #[cfg(feature = "file_sliders")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTY6kqJJqlWbptgWB5wDGIAxHYMp1myApunqCpyDedJTnaV6DnmWxNl2Fg2n+gZ3gE",
        categories = "files,development",
        tags = "cogged,gear,mechanical,machinery,configuration,controls,preferences,settings,system,admin,edit,executable",
        contributors = "danielbayley"
    ))]
    FileSliders,
    #[cfg(feature = "file_spreadsheet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTY6DEMxoDKapVm6bYFgecJdhadJ2lOeJXgKbp8gqcg3nWd5soWe5bE2fgxoqgZrnmAQ",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSpreadsheet,
    #[cfg(feature = "file_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDEYYNg0MAghQMQthISA0hEIIThWHYYhgdgtDiHIehaIAxCIPg8C+A4FiyL4GgiCgxDaHQ2iaH4WhiHRIDmOoUjyFwxFaJYSjuHYpiuLYyjGBIzgkTYPCCOQyg6SQwC0Lg1DgNoYC4NAxDQbAtDKZhBleV5Ch8MQ3g4aAtDOQY7j0MR2keSpsigMRojmSJ7ikepMi6UA+gE",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FileStack,
    #[cfg(feature = "file_symlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxFYNBhDIIITDAIIWDGFAtDIaA4hILoMDKIIXiSGQxC4NwwDaKIqGwMwuDUOA4CCL4xDgQYiiGI4WhiFIWDgdgxDKEoUiSGIbhQSA2kSFYlhuGx2C0M5MkaDYahwNwiD4PAvgOBZbl6BoIgoMYhHYNRhiaVY9DEaA1lqXJhmCBJigkbQxhiMwzlKfAznCXZ0D6AQ",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileSymlink,
    #[cfg(feature = "file_terminal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkbY6DGDIZnWGJqlWbptgWB4JE2SoWDgaA0nibIBA",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileTerminal,
    #[cfg(feature = "file_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxhQORIDiapVm6bYFgecAxgwMQznWd5sgKbp8lyfw3oKU54leAQ",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileText,
    #[cfg(feature = "file_type_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIaA2GGDYNDAIIWhYMgtDIVg4hMLg0g6IIXiQMAtC4NwwDYLQxiiKhsC0MwuDUOA4jGM41EEMojjuIYYiQMYhDISIShSJYXhuDh2DYIg+DwL4DgWTpRgaCIKkGSg1GEMQglyP4WlwMRoDWTZPlSU4ElWCRNDOXQ2HaLIzGGM4zkeXIzicNRoDecw1nWP53n4NRWDGTJOlCaZogWB5rDaDoQDKZaIlKAppoyCg3l0NB2DikpngE",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileTypeCorner,
    #[cfg(feature = "file_type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxmMOBoDKapVm6bYFgecJKhaXw2nebICm6fIKDmFgzHYLY3GGN43iuRY3owNZoo4NaQk4IKPDUdo3oKeYBA",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileType,
    #[cfg(feature = "file_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTZKhaXw2mqVZum2BYHgkbQxDWFg1C0M6CoKO52myAQA",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileUp,
    #[cfg(feature = "file_user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkTQxgyDhhiiQ4rC2OgwmqVZulMYxpHIYxsGUIBjHmCQxmkIBygkMwioYeKJDKfKAoKhA+gEA",
        categories = "account,files",
        tags = "person,personal information,people,listing,networking,document,contact,cover letter,resume,cv,curriculum vitae,application form",
        contributors = "danielbayley,colebemis,ericfennis,jguddas"
    ))]
    FileUser,
    #[cfg(feature = "file_video_camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyFYNBhDIIITDAIIWDGFAtDIaA4hILoMDKIIXiSGQxC4NwwDaKIqGwMwuDUOA4CCL4xDgQYiiGI4WhiFIWDgdoOhKFIkhiG4UCIPg8C+A4FkqTYGgiCgxiEdg1GGJpFiWDRoDWSZLlCT4ElGCRtDGGA3C4OA0DONAuDAMwzC2Jw3DWVwuDYNJ4gyPINCALg5DaaQ1DYdp6imEZ4DWipaDELaAoKMA2EyZ4+C4MQ1DeX5MmOShyGUYx0CAaBlGkZxoHSCQ2CIIB4gkM6sHKrgiDGrB3GkZIFgmmggHmCQxquSgvp+oQ+gE",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileVideoCamera,
    #[cfg(feature = "file_volume")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxC4NQ1FYNBhDIIIWDAIIZDGFwtDIaA4hULoMDKI4aieHIPDcMA2C6Kw2GwM4QDgOAgjINY0EGJYkiaGYbheGQ4HYMQyhWF4nhuHoXGgLYPDkNQiD4PAvgOBZSlWBoIgoMYkHYNRhimSIog0aJQlKVIEGiV5pgeCRNkSDZfDUIJzj6DYnDaUZTlia4Fm2Co1lyEAwkWEIQmIMAtC4OAyDaigzDgMRsk2iw5DODQuDYMwxmCd52DGig2DWDwyDQMxIjcYaGnWSKKDWgwxHaMgwiyqqvqydqrDAMRog+RA5p2HKfCCmQ1iWpQzGyDw4pamKaDOtqHnaGaLo2jw4HqepolaAQA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileVolume,
    #[cfg(feature = "file_x_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDISA2GEMoOCAMIWCAMQtDKGxWDSE4VheF4NhwMhoDiEwuDSDoqhiI4ZC4NwwDaMYzGwMwuDUOA4CCOI6DgQQyi2QoriKGYOhcOB2DUIg+DwL4DgWTpRgaCIKDGKwyksYYNg2RovDEaJMk6UIEGiU5mgeCRtDENYZDcIJumOT5UmiBZqCIbQyiMNwtnKTZ0mYPoB",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileXCorner,
    #[cfg(feature = "file_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZliBJagkbZdjeFpADULQ1CCaZTlWbptgWB5wDmc5KnOeJ6mufYBA",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileX,
    #[cfg(feature = "file")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDiEAuDSDYnhWIIWC4NwwDSLgwDYbAzC4NQ4DgII2jgOBBDKKpAiiH4Wg2FA4HYMYPhGEZEheTB6CIPg8C+A4FlOVoGgiCgxigMh2DUYYhiGRIsDEaA1lKVJZD6AQ",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    File,
    #[cfg(feature = "files")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyGgLQ0GEMoOCAMIWhYLYUDIdgxDGE4VheIoVg8OIghSI4XDKGhWDgIg+DwL4DgWMIzgaCIKDENguDcMA2g6PI+EEMguDSQJGikIIMg4dg1GEMZKhiI5QDEaJOkSRpYlKGZBDYLQxl0eovjGNo1gSN4JE2DQ3ieWwwhqTIem2SYbGiJobm6So8DOKwxmOMpnD6AQ",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Files,
    #[cfg(feature = "film")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKERoGUaRnGgdINg+ER5g0M4RhQIojD4PAvgOBYoHAYYLCAZINE0NwgDMdofigL4uguLYvGiMYzDMIA3C4NRoDQIo6jwaI+jCMgiE2QwxDIaI5imTJOkCUJSCAMQ2kaSJKliP5akGUQxjWN5XjuZQ8kyZxNmmRJhkmS5unCXJzl+dZjm2PYBA",
        categories = "photography,multimedia",
        tags = "movie,video,reel,camera,cinema,entertainment",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Film,
    #[cfg(feature = "fingerprint_pattern")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAYYNg0MAghQMAthIY4UDELgwDILQuDEIAyC4NQxiAMg2CANAiD4PAvgOBYujGBoIgoMQ0g4M4hDKGojC4Mw4hUIA2kAOAtiIOAuDgOIti+NIzgSNYJgsNwuDIOYjhyHhjjyIJFDQM4YkCJQtjuHpOjCUpRgWB5Ug2DBhg+DoWkOIgxkcNpplCApSm6CpwDYaIdDGe5rn2bY2E0MockIMQ2l2H4NiEM4nDWQA1jmF56i6aoyoiU4KDWDg5iUQ6XqOeJEg6o4qnGKoqnWIpADSGKGp+NJ/E2Sg2qMMo9leJwuDaYA1kiQIkDUN62p2fK5oqWZFDgYawkOGwglmlwyHYMq3GgPoBA",
        categories = "account,security,medical,devices",
        tags = "2fa,authentication,biometric,identity,security",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FingerprintPattern,
    #[cfg(feature = "fire_extinguisher")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2C4NRWDMYQxCCFQwCCGAwC0MYcGgLQyhSFoZiSG4VDEdgzhAIg+DwL4DgWLYwgaCIKDmFg4GgOIsi6M4ygSNIJgsOAgDOHwzjyL5Aj+BYHkIMYVhMNoOiWGQtlMNh2lCSY+gKQJOgqDQxkYNJckuXpNjWCw3hYMBhDQIJwhqVpEDCWpuDIIJ5nOGJ5DIaA0GGfpVn2IBamaMYBA",
        categories = "home,tools,travel",
        tags = "flames,smoke,foam,water,spray,hose,firefighter,fireman,department,brigade,station,emergency,suppress,compressed,tank,cylinder,safety,equipment,amenities",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FireExtinguisher,
    #[cfg(feature = "fish_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDILg0DcdguDAMxtDALQuDWFISG2GoSDUIA1hUNQ2EENguDcNA0CCKYri0MAgjIMYOiKDBjC0M4bDaMwtDePAthANQzC0OIbC2PQuDMNA4C0MQuDKDZQDEMQ0kKEQzDMIIQg8MY6ksOBtjuLJXhAMA4DgQZHDgMAyCCbJujOc41DGIg2GOO4mnOKQwj2Q4tDeLoaDMMA5g4LpWhoNw4n+iQ3DeT4RDGW47DANQ4CIPg8C+A4FpunoGgiCqCDEMAuDYNxDoKDYkpiIwuDmgoQDeW6ujmdZIjWIpRluKQ1pIMotlCwJQsKI4aDKvobEOrpTkeFggqWPaljuWhtkCTQtDSSw3DIQQ5iqgrhpCc40g6PZ9DeGIOlCjRhuS47iuaDqSt6To7DaDKapyoaggSooJG0MZ9nWQA5kWvaIDQQZvm+MrnpQLoNDIMRIuENRhiQOY9xuPcQnSEQ5mAOQ4E2R5ECAMxoiQMg3GHDr0rnJaIqkbMKlANBNw4bAyjLPr8p3AA+gEA",
        categories = "food-beverage,animals",
        tags = "food,dish,restaurant,course,meal,seafood,animal,pet,sea,marine,allergy,intolerance,diet",
        contributors = "jguddas,kemie,ericfennis"
    ))]
    FishOff,
    #[cfg(feature = "fish_symbol")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ2HMOQtDENQgDIMAtDQQwxDGFQzhUIA4h8OAiD4PAvgOBQ+gE",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "danielbayley"
    ))]
    FishSymbol,
    #[cfg(feature = "fish")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4NQgDEMhjC4OQ0C0MwuDQNggDSE4VhoOINC2GoXDWGgwCCDAwhoMoNDQIA3igLYdCCFw0DeFoYi4NgtjANhzjyDY7iwNQzC2IA1iIWgiD4PAvgOBZMk+BoIgoMQ4g8Mh2g2S5NlKUYElOCRNDGGgxDeEwzGEOQuDeMJrm0IInicMZxC0MQxC4OA2lyTpgl+BYHmKMAxDALg2DcQ4wlcNYNouE4wiwNwzCCjA1GOdoPiGdIOC4MqTgySJ4DKLp4qGnYukinafg0Q6VleVguDCk6Dhqg4XDMM58l6ApgoGVaFhmL6dDYQ6Ep2lJ5lea5mhupwglcMxoowOBhDIILWnKD6ZDmr6GDcbKqpkNK6n6vKAlQbZkrGdJmmiMqeuIQbWticbaDGF5XDIMRImsNRhowOYawCJr1nOGA5ji3LklCAQA",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "kemie"
    ))]
    Fish,
    #[cfg(feature = "fishing_hook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcLg1DgNggDEMQuDQMQ0C0NQuDkMwghqHBhDGEggDCJISC0OIoGwMwuDEMw3C2LIuDcYQuDcMINjcN4miWIoUDKOIOFYMQwCIPg8C+A4FkeSoGggIhNkCFYXCAOIOhAIAyDIIA3kaSJNkcYxpHIYxsGUIBygkMgiCAYx5gmRJsGMeJwDmXgvmKZJmD6AQA",
        categories = "sports,travel",
        tags = "sea,boating,angler,bait,reel,tackle,marine,outdoors,fish,fishing,hook,sports,travel",
        contributors = "7ender,jguddas,karsa-mistmere,jamiemlaw"
    ))]
    FishingHook,
    #[cfg(feature = "fishing_rod")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxGgMQiD4PAvgOBYThaBoIgoOINDUYQyCCIQwCCJAxC2DAwFYMxhDGDYljCLong8Lg1EMMYMiEMokDmIomDgdg0hKFIZhMYxpHIYxsGUIBygkMgiCAYx4gkMQ4lEYx5lWV4TC+R5JksPoBA",
        categories = "sports,travel",
        tags = "fishing,rod,hobby,equipment,reel",
        contributors = "7ender,jguddas,karsa-mistmere"
    ))]
    FishingRod,
    #[cfg(feature = "flag_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMGMLQzCAMAtDULQyC0OIXGGDYNDCE4TC0NIOC4NQyDgIg+DwL4DgWKotgaCAiG0MggjUMofjiKYrjCL4EjGCRNiMMgyFYNI7iyP4+gWB5BDcLg2DWDQyEgOBjhKHw1jYIJPDOXo2HGNYfDMLgwDYNwtC4OBBDGDogh+bY4CANB2DEMAuDMNA3kiPYBA",
        categories = "account,social",
        tags = "unflag,unmark,report,marker,notification,warning,milestone,goal,notice,signal,attention,banner",
        contributors = "karsa-mistmere,cyberalien,ericfennis,jamiemlaw"
    ))]
    FlagOff,
    #[cfg(feature = "flag_triangle_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDIVgyC4OBhhOEwgDCGIYC0MQuDENwtC4NwxEwNQuDQNQgDeIoUhaDYZjCGodDQNBMgwIAxiYNQiD4PAvgOBQ+gE",
        categories = "development,navigation",
        tags = "report,timeline,marker,pin",
        contributors = "tidoni,ericfennis,jamiemlaw"
    ))]
    FlagTriangleLeft,
    #[cfg(feature = "flag_triangle_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyFYMguDgYYShIIAwhcIAxhoLgxDcLQuDcMRsDEMQuDMOAgDULg2DmFA4haGIYhuMwuDQNBMgwMYrDUIg+DwL4DgUPoBA",
        categories = "development,navigation",
        tags = "report,timeline,marker,pin",
        contributors = "tidoni,ericfennis,jamiemlaw"
    ))]
    FlagTriangleRight,
    #[cfg(feature = "flag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyFYNBhDEIITDAIIWhMLg0C0Lg4EENggiCFoYCAOINGMM4XCANYNCANwuDOMYNHEMoqDMLgwDYN4ch6E4ViqEwyhYNB2DEMIShSKoYhwNIdh+IZKkkMYglMYwtikMAtDULQyC0OJcGGIIikoLYMDELg1DIOAiD4PAvgOBQ+gE",
        categories = "account,social",
        tags = "report,marker,notification,warning,milestone,goal,notice,signal,attention,banner",
        contributors = "colebemis,ericfennis,jamiemlaw"
    ))]
    Flag,
    #[cfg(feature = "flame_kindling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGMMQgDODguDWEoVheFg0hUQYWhYMAgiCEQxDcIAxDAYYeiGJomC2J4hGMMAtC6E4yC4NguDGMw5GGDYNiKKwzjQLQyEMOAghuFgxhGDYMg4WgiD4PAvgOBZSlWBoICIbYWDKTQ0C0NJRlOWJXgSWYJlyJpHDENJImOVJnD6AQ",
        categories = "nature,social,gaming",
        tags = "campfire,camping,wilderness,outdoors,lit,warmth,wood,twigs,sticks",
        contributors = "danielbayley"
    ))]
    FlameKindling,
    #[cfg(feature = "flame")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHEMQgDSEggDYLg1HQMwgDWFxhhGEQwCCIQxC0MYTiENYbiKK4fC2GofiuIYoiIYwwC0MolheLo5DULQ1HGNoNDKOg0CIPg8C+A4FD6AQA",
        categories = "weather,social,gaming",
        tags = "heat,burn,light,glow,ignite,passion,ember,fire,lit,burning,spark,embers,smoke,firefighter,fireman,department,brigade,station,emergency",
        contributors = "ericfennis,johnletey,csandman,jamiemlaw"
    ))]
    Flame,
    #[cfg(feature = "flashlight_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg2DUMggDYSAxDgIg+DwL4DgWGIbgaCIKDGEQxDMdgxheGYeh2BIfgmCw2CAMQ2HYNBhhGEQwCCOQxC2NxoC2NY3jqQ48DKPR2C0OBhDQIJMjmTwtC4OI9C4NBslENpRDgQQzCCXZPjGEggDcVg2iiGosiuBYHgkbZCDKOZwmeKoCiybIKDeDg0DkIAyhQNxhDGYZgoKhY0GGXZfkSWIxlKV4OlKS5NkOUAuDUNaNDAMA3nOaYBA",
        categories = "photography,devices",
        tags = "torch,light,beam,emergency,safety,tool,bright",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman,jamiemlaw"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdgxCIPg8C+A4FhSF4GgiCgxDcIAyGEMYOCAMIliSIwxHYNBhDMIIuiaJgxC0Lg2g4Lg4GyNA2jgQQ0CCP4xicMY2gwdg4GGDYNkKM5KEgMQwkmIInjILQylYdgtkiP5BlSNA4lYLg0jqNZfEGLowkMII2DcVgziKJJMg4LQxHqE4VhqGYEhuCRNjYNhogyd4WnsPoBA",
        categories = "photography,devices",
        tags = "torch,light,beam,emergency,safety,tool,bright",
        contributors = "csandman,ericfennis,jamiemlaw"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYMguDMNAzCIPg8C+A4FheGoGgiCgxDSDh2DaEoUhaGIdhyBIegkbQyg6DoNDIMIohmLIrgWB4JE2NIyGGMIwg2DQxC2QRIDaQIxkMIJFDELg3DUNZGC4OQ2GwNQuDIMg3C0OQuDUNoVheN4bgKLI7gqJQ0DUM5NDUSAxDWNoqmeOofE0OJgg4aA3nSOIBA",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,non toxic,lab,chemistry,experiment,test",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    FlaskConicalOff,
    #[cfg(feature = "flask_conical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyHYNhhDKDggDCFYXC4Mg0DULg5DYbIcDUMQgDEMAuDAOBBhOE4WhaIwxDiDgyEiEYrheLgtDELg3DUNQtDKHYfiEMY5iaKIqhSLYXiUIA4FYMgiD4PAvgOBZSlWBoIgoNguhsM4kDUaAxjoMA5DSUZTliV4ElmCRNDgLg1g4aA3miVJsD6AQ",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    FlaskConical,
    #[cfg(feature = "flask_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYNguDIOQyGENwgheDQxCCDQ0hwVgyCIPg8C+A4FiOJoGgiCg1CAMQ1GgMQ0iKJIpiiBIqgkTQ4C6LQyGgN40iWOA+gEA",
        categories = "science,gaming",
        tags = "beaker,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,danielbayley,jamiemlaw"
    ))]
    FlaskRound,
    #[cfg(feature = "flip_horizontal_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCANwgDWDwthANRWDcIg+DwL4DgWGIbgaCIKDIMYNhKD4mhSFoYhqBBoh2LIHgkTQxDIIAyDAdgyheGYei6BYwCKMo0DENI4jqK4cgKL4gkEIA4kWKo8kmPpLjONZPjuLA+gE",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal2,
    #[cfg(feature = "flip_vertical_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAzC0NQgDWEIQGgMQwCIPg8C+A4FhqHYGgiCoMCAMgxhSE4RDWFoYhqHIEGiH4wgeCRNDQIAxDISAyhmG4gjKBY0CITYXjiOg4j2L4egKM4ikQNpGGgLY8i6P5MkGTgyDKUZTkmP4B",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    FlipVertical2,
    #[cfg(feature = "flower_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GEMwghEMAgDGFYShIbQtDOG4QhiFIWDCG4ZhwMx2DETQ5CAOIehOF4UhEM4pisaAxG0NQgDCLY5hcMYjDMbYlGgLY2C2DYmkQIg+DwL4DgWSxjGkchjGwZQgGMeYJDgIggHKCQylwYx4gmDJKkyUZTlWS5OgaCIKgyFQwieYJLk2BBomud4HgmC4NDIMhjDQLoNhQN5EC4Ng2DcIKGDULaCoQLaLDGiKKpKDhamadpPgKepun0IJ/GOj6DjmkqHomhqNCCkI8pOlaLosNaZnWbA+gE",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCAMQ2C4NRBDSLggjENQgDCK44DeMoqjSM4yjcMY4iqOg1GGPY9kCOJHjKS41kkMQtk2GociKJIhiOJYnCKKQyCCRBWDmUwvlUaJXiSJookSKwyEiYYbmOWJmlmKIsjubAxDWYpknKaJbiqdQ1FaeJ6nEPJkn0bQ4CCigxC4OKMo4OKElahpYn2KYxioOaRisNqLpOZaVmeWqJp2K6RC2jaPqCfJapgLp/pmXYsp2rIBA",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MwgDcVg1GEMggi8MAgjIMYwC0Mhog+GgviGI4giKJImCITQxDcIAzjmLowjOTI1i8Mh2jqG49GiP4jiWJwyjWRZRkqMZNjeMBojeGZTkCVpBieRpaEiLZPkyNI3jcdpkjuVA+gEA",
        categories = "photography",
        tags = "camera,lens,photo,dashed",
        contributors = "karsa-mistmere,danielbayley,jguddas,ericfennis"
    ))]
    Focus,
    #[cfg(feature = "fold_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGgNgiD4PAvgOBYThaBoIgoMoMg4aAthGE4VgQaIYiWB4JE2DggDIdgyhKFIZieBYpgqLA4i+MYkheAoohuK4eDSOojjOPo1kCLAyDCRIyiWNIagkbQxDkIA5C0MwglmW47kaGY2G0NYNmIM5YmYM5dk+AQ",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    FoldHorizontal,
    #[cfg(feature = "fold_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDYIg+DwL4DgWFYYgaCIKgwIA4FYMoUhaG4agSHIJE0NAggwSIjhWF4oieBYHioMQwi0MhIDiJIyhmAoojaHg2joaAtjCJYzkGNYdE2D5GkiPomkyKQiG0MQ1i0OQtDOXZdCAM5TkuG5DliWg1mCXprmOGYBA",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    FoldVertical,
    #[cfg(feature = "folder_archive")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ1CKCR5gwMQ5hAcoMDIIg+DwL4DgWB4bHAYR0GgIBkgwTQyDALg5CCFAuDgQQyCCMwwCCNo2DKMwxDgVg4GGM41jeNwtDKRRoC0N4skCNJDjYMQtDELg2DkLYsEwOZTCAM4sjKTY4kOSg5DOWxIDSTJCjiRY0HYMQzmiTpDkEaA1C4MYahyIokiGI4licIhNg6LgxHaUZ4C+ehonyJImiiggxDehYZhuiJ9D6AQ",
        categories = "files",
        tags = "archive,zip,package",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderArchive,
    #[cfg(feature = "folder_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FlCU4GggIhtDkIJIkwIA0C0NJPlGVg+gE",
        categories = "files",
        tags = "done,directory,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderCheck,
    #[cfg(feature = "folder_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDQdgyC4MhsDELoNDEIg+DwL4DgWG4egaCIKDcIAyDASA0GEMomCAMIug4LQyjIVg1iuLYvi8MYmjIaAzC4OY3iyOYOg6Fw5kAbAuDiO4WDKQowkSFg2DeQBIieUJSi0MoahyIYbGMaRyGMbBlCAYx4gmDAiCAcoJDabBjHmapwhsL5hmOZQ+gE",
        categories = "files,time",
        tags = "history,directory,clock",
        contributors = "karsa-mistmere,jguddas,jamiemlaw"
    ))]
    FolderClock,
    #[cfg(feature = "folder_closed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FlCU4GgiCoRDEMBogyT5RlYPoBA",
        categories = "files",
        tags = "directory,closed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderClosed,
    #[cfg(feature = "folder_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMC4NQgDiDgzGwMggDKEAiD4PAvgOBYch+BoICIbQxDSDgwhCGIYhALYXhkNYbh2IohgSI4JE0MoNjsYYwCCDZBhiLxWDiPoskIMIvi8aAtDcLg5keF5CDELQxC4Ng5C2UBMDmWAgDOUBBj+SQgk8OQzmASA0lKQJukqMB2DEM5tmWMB6jOHo3D6AQA",
        categories = "files,development",
        tags = "directory,coding,develop,software",
        contributors = "jguddas,colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderCode,
    #[cfg(feature = "folder_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgzCAMgwEgNBhDKEAgDCGAgDELQyh0Vg1hWF4ZhkMYQh0aAzC4OQ4iKFokhuGwuDYOYrGyMw2jIMhBhaL4aiWFg2GiLY9j+MY9HaKgzCIPg8C+A4Fk2UIGggIhtDENIODANYbjUNYqDkMgzC2Dg4DKTJOlOUoElSCZXDULgyDIOIbDYLg4DWHormKZAzDiS5Nk+bJrgWB5uDGdp4hYMZwnIOJ9n+ZJhoCaaDgKbKGlaiJ3nmEINDcN56n6YAyDSaKClGl6FlWV41lidKMnGc5lmOe6UqgaKEm2mpeg8MgxjMOQ2pCephqagZqqquxthELqgouiZ5nsNKQreyZTpmzKfqGXQuq+060qeyRjGkchjGwZQgGMeIJDEOAiuoebtu8IBygmt7kua6A+gEA",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    FolderCog,
    #[cfg(feature = "folder_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlkYxpHIYxsGUIBjHiCQxDIIplHmaAzmscpoliWpfmGYw+gEA",
        categories = "files,development",
        tags = "directory,root,project,pinned,active,current,cogged,gear,mechanical,machinery,configuration,controls,preferences,settings,system,admin,edit",
        contributors = "danielbayley"
    ))]
    FolderDot,
    #[cfg(feature = "folder_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FlCU4GgiCgxhEMQwHYNpPlGVpVgSV4JG0MQ1CCSAtj4M5smyYJSmQPoBA",
        categories = "files,arrows",
        tags = "directory,download,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderDown,
    #[cfg(feature = "folder_git_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDkYQ1CCEgwCCFQxC0NYZHYOAiD4PAvgOBYfiKBoIgoOQgDIMBIDQYQyiqFoyhgMgtDIVg1i+MYVheKo2GgMwuhCMIwjyDoOC4Ng5kIbAuDgMZIDKOpFjKF5JDeQhIiuU5VkeRB2DWHogiWHxjGkchjGwZQgHKCQyCIIBjHiCQxDOcBjHmdJvh8L5mmiaplmeaZrm0IpvnGeQig+d5zoYMJin2gqAgE",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis,jguddas"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMg8Pg8C+A4FgeGhwGEdBoCAZIME0MgwCCKRhDKKwgiqMYrC0MhWDiLYvjIMI0jQaAtDcLg5jiLoyDELQxC4Ng5C2QRMDmSQgDOQRBi6RIwleQA5DOURIDSQ5XjGNIrHaFJfjqLwyFoIoaC+IYjiCIokiYIhNDENAghQaIVmybhonCI4licN54DOeprhufQ+gE",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit,
    #[cfg(feature = "folder_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DMOAgDIMBIDQYQyhIIAwhoIAxC0MofFYNYXhmG4bDGEofGgMwuDmJIYiaHYdg4OYtGwLg4igMQuDKL4cjGOw2DeLRIhOPpAhkMh2iwNAxDcIg+DwL4DgWUZUgaCIKDENIOhgMQ4jgQQyjwNYSmSHI6h2EQxDWOAzDaF5nmMMplieMoshCZg5DYNhsh+XQ2mYOA1nCLQ5mAOaHj+HQtjsNQwDeGh6lCUpXD6AQ",
        categories = "files",
        tags = "directory,heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FolderHeart,
    #[cfg(feature = "folder_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORWDUYYMgwMAghQMQgDILQyGgMwuDmEYYhWIoXDELg2DmHhsC4OIkC4MoghOIoWiYN4eEgMgwjCMggheEh2DGOYSjsMYahgSA0jqFIWhqGh2C0MQiD4PAvgOBZSlWBoIgqDAxDMaJAlGU5YleBJZgkbQ5jwNggDMLZtm+YZUmUPoBA",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderInput,
    #[cfg(feature = "folder_kanban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlmXIGgiCg4i4MB2DSWJal+XoEmCCRNDGFAxmUMpolubJrgWB5uhGZB2DadZqgEA",
        categories = "charts,development,design,files",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,directory,project,root",
        contributors = "danielbayley"
    ))]
    FolderKanban,
    #[cfg(feature = "folder_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDASA0GEMoOCAMIWCAMQtDKGxWDWE4VheFwxg6GxoDMLg5iCFIihmGQuDYOYpGwLg4iQMQuDKK4Yi2OA2DeKRIg+O49hUMh2jgMw2CIPg8C+A4Fk2UIGgiCgxDmGZHkuTZPgQaJSl6B4JguWAxDQaAykyTpTk0YxpHIYxsGUIBygmaQgGMeIJlcIp4HmdgwmoL5unCcg+gE",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIB3GkZB0GiCg4g0aBlGkZxoHSCg1g2CYLDSDR5goMQ3CIPg8C+A4FikcBhhIIBkgoTQxDAIAyDASA0GEMo4CCN43DELQykQVg1j2P5BCAMY4kQaAzC4OZJj6S5NDELg2DmUhsC4OJXC4MpUkCZJClkN5SEiOZjlaPwyHYMguh6KQvi+EoujAaIyjSOZMDcdpEmyTJAC0NJAnCKIqnYaA+gEA",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[cfg(feature = "folder_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQzGgNgiD4PAvgOBYThaBoIgoMgwCCHRhDKHwgh6JYfC0MhWDiIYjiYMIoigaAtDcLg5iyIomDELQxC4Ng5C2NRMDmPQgDONRBiKOIkkuNA5DORRIDSN5LiWKIfHaDpTi6IwyFqEoUhkPoBA",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[cfg(feature = "folder_open_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ0g0Lg0DULQyC4ORBDIIIZDAIIcDEIA5C4MoPDEMBIDIMBhhmG4dg2EA5g+FQ1GwLQxC4NQ1CANoqhqLYejWFokjcSA0jyLI/DKFBWDUYwwkCNg5hSGoUGgM4WDORo+i6Ng2DaFhsC4OIZjYMpZhyZ4Ql2FhIDEOJmi2H4rHYMgiD4PAvgOBZ2GMaRyGMbBlCAYx4gmDgioIeaFDWhxyoWdZ3nyfqAD6AQA",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = "danielbayley"
    ))]
    FolderOpenDot,
    #[cfg(feature = "folder_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ0g0Lg1C0MguDkQQyCCGAwCCGwxCAOQuDKDwxDASAyDAYYYhqHINhAOYPhQNRsC0MYRg8NophmLIdjSFQ1hANRIDSOYrjwMoTFYNZEjuLZHDIaAzhWS4bh2EA2iAORsC4OIejUMpTkyNQ2DeFRIDEOJglWKh2DIIg+DwL4DgUPoBA",
        categories = "files",
        tags = "directory",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FolderOpen,
    #[cfg(feature = "folder_output")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANwuDUVg1GGDIMDAIIWDEIAyC0MhoDMLg5hOGoXiSGQxC4Ng5iAbAuDiJguDKIoViSGIog4ORIDIMIyjQIIZhQdgxjuFI9DGHIaEgNI8haGIcC2Jw1CIPg8C+A4FlOVoGgiCoMDEMxokKUpUlmWIElqCRtDWPgwC0Mwgm6cJilWZg+gEA",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
    #[cfg(feature = "folder_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQxC4NRWDUYYMgwMAghcMQgDILQyGgMwuDkYwuDeGINC6IAzicN4hGwLg4icMojDQLg2g0IIhiuIRIDIMIUhuJoZkAMh2DGPoVkGDYdhsaAtDmEAiD4PAvgOBZSlWBoIgqDooDeMAxiANgyDYYYahqQgwC2IAwDANJqC6bA0GwLQ1nCGp1DAMQyj+FpBC2EAwDaLw1nKfw4DOJQyi8NxhhCEJJheNaKmIbKKDgN6GoifKQjgOKEn8NaBHqUZTlgPoB",
        categories = "files",
        tags = "directory,rename",
        contributors = "karsa-mistmere"
    ))]
    FolderPen,
    #[cfg(feature = "folder_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAdg2CIPg8C+A4FhSF4GgiCg5g4MxohKFIWgQaIZiWB4JE0MgwCCLBhg2DYtjOLgtDIVg4jCLggjQMI2jYaAtDcLg5jqMo8g4LQxC4Ng5C2RBMDmTAgDORBBjGSI0kMOQzlQSA0kaWY8jaLh2DEM5hj2OwyFqE4VhoPoBA",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[cfg(feature = "folder_root")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlkYxpHIYxsGUIBjHiCQxDIIplHmaAzmscoJmqWQvl+YZjlmXIGgiCppi4NR2DWWJanoPoBA",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley"
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxC4NQiCAcoMDKEISGMeYMDGFoRD4PAvgOBYHh8cBhHQaAgGSDBNDIMAgi4YQyjAIIvjaMAtDIVg4jKNI3DCOY5GgLQ3C4OY9jONwxC2Dw2DkLZGEwOQuDYIAzkYQYzkmNZckUOQzlYSA0kiXI2jmMB2DEM5kj+NAyFoIofC+JooiWJ4pisIhNmoLpgDENJ9CAMQ1oINpxiCdBoD6AQ",
        categories = "files",
        tags = "directory,search,find,lost,browser,lens",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg3CAMgwEgNBhDKEAgDCGAgDELQyh0Vg1hWF4ZhkMYQh0aAzC4OYihaJIbhsLg2DmKxsC4OImDELgyi2GovjoNg3isSIRj2P4XDIdg0C4MQiD4PAvgOBZPlKBoICIbQyiaWgtjoOZdiuTpQlWTxjGkchjGwZQgHKCQzCIIBjHmCQxDecBjHidJ2k8L5mmiag+gEA",
        categories = "files",
        tags = "directory,search,find,lost,browser,lens",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSearch,
    #[cfg(feature = "folder_symlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOQuDMNRWDUYYMgwMAghcMQgDILQyGgMwuDmFIbhiJYaDELg2g4ORsC4OInC4MojhaJYZikN4hEgMgwjONQghqFR2DGPIVj4MYdhsSA0j2F4Zh2HR2C0M5MiaG4dGgNwiD4PAvgOBZbl6BoICIbQ4j8NggDOUprDOWpcmEPoBA",
        categories = "files",
        tags = "directory,symlink,symbolic,link",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSymlink,
    #[cfg(feature = "folder_sync")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgwEgNBhDKDQgDCFQgDELQyhoVg1hKFIWhYMYNhoaAzC4OYfhOIYYhgLg2DmKBsC4OIjDELgyiqF4sjcNg3igSIOjqPIUDIdguDUIg+DwL4DgWS5OgaCIKDGEwxDAdg0GgNJKkyUZQgSUoJG2VYYDSLg1DMNQtj0MIeDUIJwkQOJol2TZhmCBYHgkTQyhOfh2C2WqCnaX4CmGewiG2foYDibJImqLg2m4YZwnKF4Zo6N5Jkud5PgE",
        categories = "files,arrows",
        tags = "directory,synchronize,synchronise,refresh,reconnect,transfer,backup",
        contributors = "danielbayley,jguddas"
    ))]
    FolderSync,
    #[cfg(feature = "folder_tree")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAYQxg4IINhWDgtDEVg2hGE4WDCGIYGgLQyC4NYchKFgxC0Lg4isNBsisOYYC4MhBhKKIUjkMQ1CAM4iDKJ45hWGIOHaJo3kKOoOFoIg+DwL4DgWTpRgaCIKgwIAyDGQYehcMR2C0M5ckKIAxj8Lg5mODYqiyLYlDWMAuDQMorDiR4dmSaJ0C4NhIDGYpIh6RJfneOIpkuTZPlSU4ElWCRNDMIImDKWZJg2lAyGgM6JlCjaMgWB6PpEMx2n8YaYpalaZpuTqdlKAQ",
        categories = "files",
        tags = "directory,tree,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderTree,
    #[cfg(feature = "folder_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FlCU4GgiCgxhEMQwHYNpPlGVpVgSV4JG0OQgkiNAtj6bZglKZA+gE",
        categories = "files,arrows",
        tags = "directory,upload,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderUp,
    #[cfg(feature = "folder_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FlCU4GggIhtjINQgDEMAulyYZPlGVpVgSV4JG0MQ0mCXZfDULZilCUpnD6AQ",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderX,
    #[cfg(feature = "folder")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIIMGEMoOCCDYUg4LQyFYOIQhKFQwheFxoC0NwuDmG4RhUMQtDELg2DkLYkEwOYsCAM4kEGEYnhOOojDkM40EgNImjqFIXg4dgxDOQodhIMhaCIPg8C+A4FD6AQ",
        categories = "files",
        tags = "directory",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Folder,
    #[cfg(feature = "folders")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA1GEMgghGDYNDGEoSHYN4QheFAgDELYRDISA5huEwghWIIgFaD4hieLoWDKIBoDILoPDGNYejiHYWjeNA2GwLg2C4OBhjcNY5keHYVC6Ph6CIPg8C+A4FlCU4GgiCgzCAOJMDaRItkoLY8C4NwzDgVgxiSYIug2IRoDEMYlmyL5kDOMQxk+UZWD6AQA",
        categories = "files",
        tags = "multiple,copy,directories",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Folders,
    #[cfg(feature = "footprints")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ2HYLQyC4Mw4EODAxDELg1CCEg5DeDQwhoIAzCAOBjC4MAzhELg3DKDQuDQOQtDYIA0hqMxDDmE4fi4MQwiMLg4iAIA1hoY4/DMLoYhGRAuDYNpMDiTg4FaDhhi6Lo/DGDQtgwMBaCIPg8C+A4FmKZYGgiCgyj+bIQhKFJHisMY9iiKpJnSXJFDYMgtnaK4tC2GYxjOXY3DYQwxjYNokjSiggDeQYNgyOg1keQIYhyTZPpqUg2lSbJXpqWggj+XpgmKZIEGiZ6rgeCRNg6DQ3GgNJhmOaKtgWr4KhcM61reqpmgE",
        categories = "navigation",
        tags = "steps,walking,foot,feet,trail,shoe",
        contributors = "karsa-mistmere"
    ))]
    Footprints,
    #[cfg(feature = "forklift")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMEgNRhg2DQwCCFQwC2Ex2DUIg+DwL4DgWHohgaCIKDENYODkaA3h2H4kiOBIlgmCw2ioVgyi6IIyjGBYHjSNoMFYN4SCCFIWg6RoZGgMguDENwykWR4VDGDguDQMQ0C4NQ4DYbAzC4OAyDgIJgmIOBBhOSJUg6QQwmGY46jCAoyj+Cg3ioaA0nKPA8GMaRyGMbBlCAYx5gkMQ5CIIBygmOaFHiiAzjqf6BoOHqVoKhKGoiiqMo6ixjpEIoch4L6ZpeAQ",
        categories = "transportation",
        tags = "machinery,industrial,warehouse,lifting,storage,equipment,heavy-duty,moving,vehicle,transport,logistics",
        contributors = "ericfennis,jguddas"
    ))]
    Forklift,
    #[cfg(feature = "form")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0GgNgiD4PAvgOBYThaBoIgqDAyGgMQwhKFIZhMchlGMdAgHcaRkgWCQxhEIB4gkNAiCAaBlGkZxoHSNI2HmLw4jYcozCIMYiC+JooiWJ4pkWNQgkAIoxjiOo8j4IJEi+Nori0aIvhGE5Jk0PoBA",
        categories = "development",
        tags = "document,page,file,layout,paper,stub,formality,structure,template,inputs,design,components",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Form,
    #[cfg(feature = "forward")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDcIA1C2EoUCIPg8C+A4FheGoGggIhNDSDg4HYLQyGGIoiDAIIrDEIA0C0NBoDEMoWhiHQ+gEA",
        categories = "mail",
        tags = "send,share,email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Forward,
    #[cfg(feature = "frame")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyDIIggHkMoKDaDh4hGC4OHmCQihMPg8C+A4Fh2IIGhmCgxDiGIWieFIagyFIWg2HYfgQZYijSB4ahOD4timCougeFoch6I42gWOImiiQJIj2C4NjuPgijKRIB",
        categories = "design,photography",
        tags = "logo,design,tool",
        contributors = "Bowero,ericfennis"
    ))]
    Frame,
    #[cfg(feature = "frown")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ2CCKRzC0MQuDULQyC0NIyjQIIzDSOIZhuIYjhobBpG6CB4DGDA5hAeZGCKSAgHkMpHhAeJQkwLgwDGPAvkGQ5AkKRJUDENZWlgIJFhMNZJkuTZPlGGpal4PoBA",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDMaAyGEMgghQMAghcMYVhUdoShSFoYiGDQwHYLQ2C4OQ5DiE4bheLgtC4NQ5C0MQuDQMhMDEOAgDUIg+DwL4DgWP5CgaCIKgyFQxFYNYsiCLwyC0MhIk2H4hlCHAxDaPpAkWRIEkaCRNh8MRog+XJBmCX4FgeYgzCAOZmDGaJegEA",
        categories = "transportation,navigation",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis,UsamaKhan"
    ))]
    Fuel,
    #[cfg(feature = "fullscreen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PByGUYx0CAeIJDcIggHmCQ4mgcplCIMZoGgZRpGcaB0muaB3GkZIFgkMQwksL5gmIPoBA",
        categories = "layout,multimedia,design,photography",
        tags = "expand,zoom,preview,focus,camera,lens,image",
        contributors = "danielbayley"
    ))]
    Fullscreen,
    #[cfg(feature = "funnel_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLgzDUNAgDMSAzGEMQghcMAghoMAtC4Nw0DKGAuDYNxsDcLgyDINQgigOQ4DkQYiiKHIYhiGgxDQdg2haNo1hoLg1DWDYwDUbIiDEQYXhmG5NjkIAyDEdgtDcYYzk2OAgkEMQ3C0MYODQMRsl+UQ4l6YA4CIPg8C+A4FmuboGgiCgxDYIA2GgNpqmycZwgScoJgsOYSjue5tn8PoBA",
        categories = "layout",
        tags = "filter,hopper,add,create,new",
        contributors = "gubser,ericfennis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    FunnelPlus,
    #[cfg(feature = "funnel_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg1DMMQgDMSAzGGEYRDAIIZDALQuDcNAyCAMQuDYNxsDcLgyDINQgigOQ4DkQYhiGG4iiKGQxDQdg2haNo1hmDoPC6MA1GyIQxEGF4akuOA0CAMgxHYLQ3GGM5MjaDgxDcLYjDMNAxGwLogluYg3DMIg+DwL4DgWaZsgaCAiG0MQ2g6Ep2iwNZomqb5ugScIJG2UJ2DODgtnme5rn8PoBA",
        categories = "layout",
        tags = "filter,hopper,remove,delete",
        contributors = "gubser,ericfennis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    FunnelX,
    #[cfg(feature = "funnel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDAYQxCCEoNhUIAuDUNQzC4OA5DUbAyhMQYShQIIWDENIODEdgtDcYYhiGJ4XDUMQ3C0MQuDMNAxEwMo4DeKQ0C4Ng3iOE4mkiDY+CAMxIDOEZHhYMAtC6QIhjiRBsDcLgyDINQglsOYdEGMJJkeDITDQegiD4PAvgOBQ+gE",
        categories = "layout",
        tags = "filter,hopper",
        contributors = "colebemis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    Funnel,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANx2DEMAiD4PAvgOBYThaBoIgoNggDWDw0hKFIZhMchlGMdAgHIeIJDIIggGgZRpGcaB0gkMQ4i+LAihCLx3GkZIFjeLggHmCQziIL4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMx2DEOAiD4PAvgOBYTHIZRjHQIBoGUaRnGgdIJhAIggHIeIJDKJh3GkZIFiSKwgHmCQziaKQiDaEoUhmG4ThaBoIgoMoMg6JYThWBBoD6AQA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENAiCAdxpGQdBog4MQ4hIeIODOEh5h6Ehyh0IgyCIPg8C+A4FikcBhhcIBkg4TQ0CAMgxGgMYoiqL4Xi6MBojKNA5jeOY7ikL4+GiQIxjMIhNhCRo6jySpBk2QpPlGRY4lSSZLD6AQ",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMhoDEMAiD4PAvgOBYThaBoIgoNQgDaDw0hKFIZhMchlGMdAgHcaRkgWCQxDgIggHmL4RCAch4gkMoyGgZRpGcaB0i+OwgjkIgziIL4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMhoDEOAiD4PAvgOBYTHIZRjHQIBoGUaRnGgdIJDEMgiCAdxpGSBYkhEIByHiCYmCAeYJDaJ4xCIM4ShSGYbhOFoGgiCoMDKDoQjyFYEGgPoBA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig2CIIB5gkIgxDGDR4DKCgxDCDR5haEYTD4PAvgOBYfiKBoPgoOYUhwOIahwMQzhSEIsh+IYEGWJI2g6EAxDKKoXDULgwhOB47DWLYXj2NIljiBZEheLIHi4OJBkOJ4RhmDouhmSo2h8cBhHQaAgGSChNDENwuDMMggDUSA2C4Ng4GENAgnQMAgncMAtDMLg5DcOAgnwNQ5GMLZBDCbwwDUMqGkILoYDGjZno+ixDDKcAwnQOQuDQMQ2CCawxDSnA1p+oQ2GEM6BnirJ3qoMxjDGrAxC4NaGDWoAtDEbK0p0NK6pyohBmua55CCsqbDgMqAp4aKjDMNA0GGxKtsex7BqMNQ4DYTJnscOBjrWtbWrKoapquxqunsY56rQNQ0ramK/m+2q/nAOK2mgMrao0MA3o28ZCpGj6Sv+lAxEGdJ2tWk5qmwegijSX5hD6AQ",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad_directional")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgxDQNggDENQuDgNQ0GGDQyDAN4SC6G4dDAIIiDGHg3DAOIjGyDQ1hGLA2EEMggjKIokhINYSDiFQyDgVgyDGGYSiOQwxC2JQxGgLYYkeQ4kkaRh2C0MoODcMhhjKNJECALg1DgNpGC4NIPHoIg+DwL4DgWZppgaCIKDGOg4jyN5XjOTYSmCYg0C2XJeGyYItoANoZh+HIeiCd4lDCYInDiK5cl+L4xnaNZCnCO4pDkSI/kGipakcdpLkKlZFkeZJmmiBBomuqoHgkTQ2n2Lg0mGD6TlmNoUDGVY3EgM6domTwxlGoqejapRolOu5WliiYenqsqPi2HotoSiIaoapJEC6janmebKsgWroKDkIK/ky2rIsWzqgsqVZ1rieKys+D5/i+1KDtiHb6sGDaNiqgqCrezrlrqVbeqmaoBA",
        categories = "gaming,devices",
        tags = "direction,arrow,controller,navigation,button,move,pointer,arrowhead,console,game,gaming",
        contributors = "felipeajzanetti,jguddas"
    ))]
    GamepadDirectional,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig2CKBwygoMQwg0eYPCIMQyhOCYWhgPg8C+A4Fh2IIGHmGoRg0eIVDiKIaisIIUhANAih2H4EGWIo2g6EA1C4MAxhOFQxDOLI7hmEJDjSI44gWB4mi6KYQDiPY/i+JpUjCFo/kmNodHIZRjHSL4KgwIB3GkZB0GiCgyhIIBoGUaRnGgdIQhiB5rg0ch4niNJemAPoBA",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gauge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDQIA0C0NAiD4PAvgOBYWhmBoICITQzC4M4QDEORhDEMIOimKQxg6Dg3iKDQwhWF4cD6AQ",
        categories = "transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "ericfennis,Andreto,danielbayley,karsa-mistmere"
    ))]
    Gauge,
    #[cfg(feature = "gavel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDMLQ4C4Mw4DEIIShQYYWhYMAgh0MQtDMLgwDCIAzGyGA4DSEYThUIg+DwL4DgWMIzgaCIKDENoOjsNgtDaL4xjaNYEjeCRtDIMQuDWDgwkuEYRkGMpFkSBYHkcOIXCCPpAjCU40gKRZXgqEpMDeS5aDiUpDgEA",
        categories = "navigation,tools",
        tags = "justice,law,court,judgment,legal,hands,penalty,decision,authority,hammer,mallet",
        contributors = "Andreto,ericfennis,jguddas,karsa-mistmere"
    ))]
    Gavel,
    #[cfg(feature = "gem")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CAMwgDgIA5GwNAgDGEQ0C2GQtDKDgtDYIg+DwL4DgWJIngaCIKDEN4QGEMggjIMAgjUMYYC4NguDgbIajGM42kKOAuDCGYzC4Mw4DIbAtDcLg5DmGINDkOA2kCNJDC0MwuDINIvDCTZPlGHJUlYQYylmN5IheT49h+UQ4luUA5DeaJBjWa4vDMeojiWKopgSK4JE2Mg5GgMgwn6JqCD6AQ",
        categories = "gaming,development,finance",
        tags = "diamond,crystal,ruby,jewellery,price,special,present,gift,ring,wedding,proposal,marriage,rubygems",
        contributors = "connium,ericfennis,karsa-mistmere"
    ))]
    Gem,
    #[cfg(feature = "georgian_lari")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg1CAMgxGEN4OCCFIPDAIAxhqFguDMNQtDkIg+DwL4DgWJIngaCIKDEM4aDIVgziOJYqimBIrgkTQ0hAMRoDENo0iaOI3gWB46DmMIykKNoBA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "kivicode"
    ))]
    GeorgianLari,
    #[cfg(feature = "ghost")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQwGgLgwDEIg+DwL4DgWFYYgaCIKDENYNg+EYThWF4EGiGongeCRNDEMggDIYQ4CCMwwCCNgwC2Mw4HaLhsDMLQzjALogDKRBMi6DQ5GyRpFkcMQ3kqPwgDMVoOjKNI3lqOQ4joeoUhaGw+gE",
        categories = "gaming",
        tags = "pac-man,spooky",
        contributors = "mittalyashu,ericfennis"
    ))]
    Ghost,
    #[cfg(feature = "gift")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYMQ0CIPg8C+A4FhSF4GgiCgyDAIAxDEdg4GGDYNh+HwxC2JhIDaJQgicIIpiuKx2C0OIThWGoZgSG4JE0NwuDWDhhDGIIykiRgwC0NRBDQLg4CCUYokeDJEk+UZTkkIJPDWTJVkiKZIDWOYWj2FByGUYx0CAaBlGkZxoHSCYSCAch4gkMQiCAdxpGSBZ5jgIB5gkN57ngIgzmWaZrD6AQ",
        categories = "gaming,account",
        tags = "present,box,birthday,party",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Gift,
    #[cfg(feature = "git_branch_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2GEOQghEMAghQMAthEORWDMIg+DwL4DgWHohgaCIKDIMQgDEOBoC0Nodh+JIeGMaRyGMbBlCAYx4gmKwijoeYJi8IBygmHIeC+NI2jiM41jeORjkEIo+jqPAikORQikeH5Kk8PoB",
        categories = "development",
        tags = "code,version control,vcs,repository,delete,remove,-",
        contributors = "joris-gallot"
    ))]
    GitBranchMinus,
    #[cfg(feature = "git_branch_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMx2DEMgiD4PAvgOBYThaBoIgoMQ4CAORhDODQgDAIAxiSJAtgyIoiiWLooDYeoShSGYYgSGoJgsIAyDGIYjiWJ4uiqI4tiiL4xjOFY3jaBYHjkMQ1CANhhDmH5GimVQ5kmNYCjeToch6UB2DaW5Ll2TYbE2PImDgaIqmWF4BA",
        categories = "development",
        tags = "code,version control,vcs,repository,add,create,+",
        contributors = "mittalyashu,ericfennis"
    ))]
    GitBranchPlus,
    #[cfg(feature = "git_branch")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2GEOQghEMAghQMAthEORWDMIg+DwL4DgWHhjGkchjGwZQgHKCYcCAYx4gkMQ4CKLh5gkNodh+JImiiI4lieKYvjeNBjjYIoyjSKwihyHgvjuQA+gEA",
        categories = "development",
        tags = "code,version control,vcs,repository",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMg4Ig+DwL4DgWB4bGwaRuggeAxhOEB5ieDYPCAeAygwOYpjCLIahyIokiGI4IHmNIZCCKoYi2JoYDWEIvgwMgxjYL44GUPoBA",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommitHorizontal,
    #[cfg(feature = "git_commit_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYNgiD4PAvgOBYTGMaRyGMbBlCAYx5gmDAiCAcoJDOJBjHiIgyhKFIZhuHYThaBoIgqDAgDENYQi6FYEGgPoBA",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "danielbayley"
    ))]
    GitCommitVertical,
    #[cfg(feature = "git_compare_arrows")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgigkeIMDWDxygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDEMggDYaA1GGKYpDAIIxDEIIvHYN4YhqIIih+IYjiUIhtDENQgDkLQzkcIJIheGQvjsaIZhyBoIGOEQiDEOYUhaD4KgwMQ4jmG4ElOPYiiSJooCCXxIDeLo1jKcAxC0MpzFaWZNk+ZY/gwbQ5mqRAzkqSZMjqPg+gE",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "danielbayley"
    ))]
    GitCompareArrows,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CKCR5gyDoQHKDAzCIPg8C+A4FgeGodgaCIKgwNoQGOEgiiYIIWCKGIahyBIihocBhHQaAgGSDBNDEMwgDYaAzGEMggkQMAgkcMZFkUdg3hmG41jeNI2jiOgijySoOEgOJDkuR5JC0MphFYOZPC+URoD6AQ",
        categories = "development",
        tags = "code,version control,diff",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwM4QGMeYMDEOAiD4PAvgOBYHhyH4GgiFoMDaEISCKFIJgsIoohyHoEiSIozgeEYThWF4vhWLoZhuHYjiEPBwGEdBoCAZIME2GQgDkdgyGMMAgC4NgtC4NAgDELQxloSA3GOVw2CAMJclcNJmDEVg5kAL5FkeHJvkiSgikwMpaDIdoUjGcg+gE",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis,danielbayley"
    ))]
    GitFork,
    #[cfg(feature = "git_graph")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQigkeYMDaDxygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDUIA5HaE4ZC+IIihmHIGggY4RCIMQ4g+CoMg4IIVCKF4tjKHg8i+I4lCITQxDIIAzHaOIYhqRoxgSM4JguNw5jqNoTj6FpRhuVZEkaJImDENggDENQuDcQQ5ikIAwnGcwxm+WotlOAQ",
        categories = "development",
        tags = "code,version control,commit graph,commits,gitlens",
        contributors = "danielbayley"
    ))]
    GitGraph,
    #[cfg(feature = "git_merge_conflict")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2GgNBhg2DQwCCFQxCCEx2DcIg+DwL4DgWHohgaCIKDYIIMHYOYdh+JIjgSJYJE0OQgDONggiyHogjGMIFgeM41jWNwzi2PIiDwYxpHIYxsGUIBjHmCQxDgIggHKCZFlAeJTlWO5KkyTg+gE",
        categories = "development",
        tags = "code,version control,commits,diff,error,conflict",
        contributors = "timmy471,colebemis,csandman,karsa-mistmere,ericfennis"
    ))]
    GitMergeConflict,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEOIOGMeYShQPg8C+A4FgeGodgaCIKgyFYXCINoVhGKAihqHIEiKGhwGEdBoCAZILE0NggDIMRWDkYQ5CCQgwCCRZFkIOYthuM41D6AQ",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDaDhjHiCw1CIPg8C+A4FgeGhwGEdBoCAZILE0NQgDkdgxDKGYbiGI4ah2BoIhCCwxDiFIWCIMQ5g6CoMi+HIEjWIIiiSJgiG0MYpDkLQzlAIJRg2GgvjEaJHiOJYni0IA2GgNRhDIIJkDAIJnDGZZlHYN5DlgPoBA",
        categories = "development,arrows",
        tags = "code,version control,open",
        contributors = "danielbayley"
    ))]
    GitPullRequestArrow,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDYIA5HYMQyhiGogiKH4hiOJQiG0MgxCAMwtiiE4ZC+MBojKIokguN45DmPI8i6QIzkONImDEOAgDEMQuDUVgxDWTJBhmHIGgiCoMhSEQilKFIWmYOJMl+HoBA",
        categories = "development",
        tags = "code,version control,rejected,closed,cancelled,x",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_create_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQiCAcoMDODxjHmDA2CIPg8C+A4FgeGhwGEdBoCAZIME0NQgDkdgxDKGYbiGI4giKJImCIbQxikOQtDOPAgj2E4aC+MRojOI4lieLQgDYaA1GEMgglAMAglMMZRlEdpBjCNJGjWSQ5CCOR2hiQpEl2SAiE0MpQDEOBoC2ZJbjKAQ",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreateArrow,
    #[cfg(feature = "git_pull_request_create")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgiCAcoMDODxjHiDIOD4PAvgOBYHhkcBhHQaAgGSDBNDYIA5HYMQyCKGQviCIofiGI4lCITQxDMIA2GgMxhDIIJADAIJDDGQZBHaE4vjEaIziKJImDEOAgDENR2hiGpMk6NYmDKRpSGgLZYjCNA+gE",
        categories = "development",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreate,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEOIOGMeIShQPg8C+A4FgeGodgaCIWgsNoOgqDIVhEIomhqHIEiKGhwGEdBoCAZILE2EwgDYVg1CKLozjWMo0jaOAijoOAgDEMR2C0MZAhuQhohobBpG6CB4DGJYOHmWwiDmDh4DKXAgHmZAiDKUIulaWA+gE",
        categories = "development",
        tags = "code,version control,open,draft,dashed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ4CIIBygwM4QGMeIMg4Ig+DwL4DgWB4bh6BoIhaDA2hWCwiieEYThqHIiiAPBwGEdBoCAZIME0MQzCANhoDMYQyCCQgwCCRQxkOQx2DeLgvjONYbGwaRuggeQxgwOYQHgMomlqV4qhAeZcCIMgxk2UpUD6AQ",
        categories = "development",
        tags = "code,version control,open",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitPullRequest,
    #[cfg(feature = "glass_water")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4MQxDYIA0g0MA0EEMQghcMAghqFw2g2FwzGiDguDcOBhheGYbhgIAuDkOQ0hiEw1EwMQ3g0OQgDIMAuDIMRBDKOYqhyGIMkAMgyEgOAuiaRpChgLQyC0MYkDkegiD4PAvgOBZYluBoIgqEAxDIYQ1CCZoakOEIamaaJOmuV5Zl4PoBA",
        categories = "food-beverage",
        tags = "beverage,drink,glass,water",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GlassWater,
    #[cfg(feature = "glasses")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ1CIIBygwNIQGMeIMDYIg+DwL4DgWB4bh6BoIhIIoUgmC4Ng+CYXg0OIahyIogDwcBhHQaAgGSDBNDENAgg4YQyCCQgwCCRQwC0MpJkOTJHkaSwyjAL41jeG5UjiOgiE0MguDWPwzCCXg3GMLg3C0MQumCaA0ksM5JlKV5WjaWI7DKaJeDGag5CCYwtmWZ5poANZJC2bpRhuU5zD6AQ",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg2DgNgggwQQxDQLg1hKFoYDAIIcDGEgyCAMgyGGFYXhmJ4ch6HQtDKHovh2EoxDmFw0iEMQzCIPg8C+A4FjuPoGgiCo3DIaA4heOo8kGQIEkKCRNi4IA2FYNBhiGIYrDALQ0h0dgykqPZOjschlGMdAgHiCYVCIIB5gkNptHKagiDGbRoGUaRnGgdIJDWbR3GkZIFgkOJhmWZw+gE",
        categories = "security,development,devices",
        tags = "vpn,private,privacy,network,world,browser,security,encryption,protection,connection",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    GlobeLock,
    #[cfg(feature = "globe_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxDENAgDQLg0DYMhBhALg1CCGYbDAIIfDGHAyCAMhhgyHIfiqHAgDkLgzDEM4cDMLg2DQMwiD4PAvgOBY6j2BoIgoMQ1hoNQ3hyRQ1DUNoYhOG4diCUoiDGJAyiSKIoiuIoTDkMg5hILpeDmOY7kCP4EkGCYLkUOJeikLgyDMNJOhqHJPlKW5WieIYrioLY0jagQ4DeZY8mmaIFgeawxDeNZMiMSJXoaZ4Cmmi5Di4MA3lSmqchifZ5iyVYllieJRlsIA4hSEarDQNaUoilqKkITZYDIaIMrGPqzmoIhtlaJYfDIMK7GgPoBA",
        categories = "navigation,connectivity,devices",
        tags = "globe,earth,planet,disable,mute,off,hide,avoid,world,browser,language,translate,internet,offline,disconnected,network,connection,no connection,network failure,signal off",
        contributors = "TimNekk,karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    GlobeOff,
    #[cfg(feature = "globe_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAzCANYQCIPg8C+A4FhSF4GggIhNDIIAxDIaAyDAQQxDCIIoigMYgiCHwyGEMQ0C6EYyjQIIqjiOokiCM41j6Oo5DQLYnhOFYahmBIbgkbQyiwMwthENZGhaSg+gE",
        categories = "connectivity,devices,navigation",
        tags = "globe,internet,offline,disconnected,network,connection,world,no connection,network failure,signal off",
        contributors = "karsa-mistmere,Muhammad-Aqib-Bashir"
    ))]
    GlobeX,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDIYQxDQLg1CCLYvCAMI0jaNQyjWMowjuN43C2OYZhuIYjiCIokiYIhNDKMQyGiQYaC+RBoD6AQ",
        categories = "navigation",
        tags = "world,browser,language,translate",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Globe,
    #[cfg(feature = "goal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVgyGwOAgDQLYUDQIg+DwL4DgWG4egaCIKDIMAuDUNgxg6Jgyi0YQ5CCMAwg6DgtgyJw1C0NQuDIOYahyIYggSIoJE0OAuDAMINDkLg5DkNxhDUIJSjOKozkeMAykgMo/h2Qw+gEA",
        categories = "gaming",
        tags = "flag,bullseye",
        contributors = "guillermo-angeles,jguddas"
    ))]
    Goal,
    #[cfg(feature = "gpu")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ3GgMQ4GGDIMDAIIWhYMgtDIVg3hMIIVheF4bhsSAyCIPg8C+A4FimLIGgiCoUDEVgziiKovi6BIwgkTQ3g0Nx2DMYQxg2IoYkYMRoDWRJGkiFgxC0MR2C2NopiuO4pGMaRyGMbBlCAYx4gkMQ2CKYR5mQMZnHKCYnleW5dl+Wpcl6YJtCKJ5hmMIg4mcY5pCIMZrnCdZzgE",
        categories = "devices,gaming",
        tags = "processor,cores,technology,computer,chip,circuit,specs,graphics processing unit,video card,display adapter,gddr,rendering,digital image processing,crypto mining",
        contributors = "xandykati98,karsa-mistmere"
    ))]
    Gpu,
    #[cfg(feature = "graduation_cap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg0DIIAxDALg5DIMhhDGEQgDCG4bC0LgwDEOQtg0OAzDgTAxDILomCANQuDEOBhhCEIcjaJAuDYNobEyK47DmIIyhmGY2h2HIlDMMhsDgLg1DcIAzhQMIyjSRodg2OobHoIg+DwL4DgWXZggaCIKhaEQwHYNpcl6Y5igSZIJE2O4qk0VgxDYYY7DOVpHjUdgtlENZsl+cA+gE",
        categories = "buildings",
        tags = "school,university,learn,study,mortarboard,education,ceremony,academic,hat,diploma,bachlor's,master's,doctorate",
        contributors = "Tummerhore,ericfennis,karsa-mistmere,jguddas"
    ))]
    GraduationCap,
    #[cfg(feature = "grape")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA1FYMhsC0NQuDgOYOhUOQiD4PAvgOBYcGMaRyGMbBlCAYx5gkMYUhYIggHKCQzi8Yx4isNguDaG4diKJImiGI4lieKYJDcLg0i+MQijOKI2CIOAuDEMY7C+PZCkCPonkqTI1isMguDMNY0ioIpSjmYoclWQY/DyVomiiZItmKTYrDMLg5lOMIylSbhlliQpwisMAuDCGp6kuNJODGUIsnya5+m2j6HlyZAxnYMqJgmOA1jqaZ9n+b5EmWRg0kik6ZmWgw4o6WagkOlaGl0IpzlurJXgE",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ3GEMQghQMAghcMQthQMRIDUYYNg2F4ZC0MolFaH4hhiK4UiYMhoDENIgCCIosjSNB2DaE4ViuJIcEgMwiD4PAvgOBZDkaBoICIbQxDaFQ5jeDQ0C0NJCkSSQ+gE",
        categories = "text,layout,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,data,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme"
    ))]
    Grid2X2Check,
    #[cfg(feature = "grid_2_x_2_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ3GEMQghQMAghcMQthQMRIDUYYNg2F4ZC0MolFaH4hhiK4UiYMhoDENIgCCIosjSNB2DaE4ViuJIcEgMwiD4PAvgOBZDkaBoIgoMQ2hUORoDaQpEkmSIEkqCYLDmNAyHYLZSkORZXD6AQA",
        categories = "text,layout,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,data,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme,jguddas"
    ))]
    Grid2X2Plus,
    #[cfg(feature = "grid_2_x_2_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ3GEMQghQMAghcMQthQMRIDUYYNg2F4ZC0MolFaH4hhiK4UiYMhoDENIgCCIosjSNB2DaE4ViuJIcEgMwiD4PAvgOBZDkaBoICIbQxDaFZPDUIA1kKRJJkiBJKgmTZPDKFA1C2VJDkWWQ+gE",
        categories = "text,layout,math",
        tags = "table,rows,columns,data,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme"
    ))]
    Grid2X2X,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhSF4GgiCgzCCDBohGE4VhqFByGUYx0CAch4gkMgiCCLQiDOMB3GkZIFgmIggHmCY0CAaBlGkZxoHSOoShQL4nikPoBA",
        categories = "text,layout,design,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre,window,skylight",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzHYMQ4CIPg8C+A4FhSF4GgiCgzCAMQyGiEYThWGoZgSG4JE0OYOhCEoUhaKIUHIZRjHQIByHiCQyCIII6CIM49HcaRkgWCYjCAeYJkEIBoGUaRnGgdJHi+FY0jYPoBA",
        categories = "text,math,layout,design",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre,window",
        contributors = "qubrat"
    ))]
    Grid3X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEh3GkZB0GiDoQhIcoUCIMgiD4PAvgOBYoHAYYeCAZIOE0MwgDkaIhigL4uh6LYvGiMYzjUMQ1jiEY6jwaI+jCMgiE0OQgDMdo5imSZLkCTRNkSUZTkeVY/D6AQ",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYLDkIg+DwL4DgWB4bh6BoIhCEoZgmEoVhcIoZhuHYEiOIYwgeD4RCINYqhiDoKgyGociKIICjOJIrDGOIoj2NYShSLpAGWMofiSNgxieFoSkePINk2Q5QiOSINkqN45gyOJblEPoBA",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIOQigkeYMDEMoPHKEgiD4PAvgOBYHhmHIGggY4RCINYVheCYLg2GIaiCHoCgSIYQhKDgghYIgxg+CoMg6GYbjGL4uiKKgxiaM44hSNoXj6QofkCIokkaN45imEomkyT5Oh2Q5WjqJAxjWU4sj+Ww+gE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMYQGMeYMDUIg+DwL4DgWB4bh6BoIgqEw5hWFwihmEYThqHIiiCAoEiOCYpiuJYqhCEoNi6HYzjGMIkimDoVguDYPiyPIbj6HxliGP5ChOSI7hSCZGDGJ5LkGT5NlaGIolKOotlqUJcjSVJgg2J5ekePZbjKXZomyWJpnSZJNmaB5sjeQ5rmid4jD6AQ",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYwwC0MQuhQOQtDIIAyhgaAyCIPg8C+A4FiCI4GgiCgxDcIAzh0Y4UDEIAwhoIAuDmNAyHaHogiKBBoiWPoHgkTQyjGKo6hEIIwC2NoahiGhohiH4hiaQIFkKCorkUSIQhOFYyhiTIXhuOZSjyVQ8HIZRjHQIByHiCQxCIIB3GkZIFgkN5znAIp6CAeZ5nMaBlGkZxoHSCQ1lML5qmyIKNm2dZ3GigQgnwMQwoKhKGogIqKn+cYem6l6LpAPoB",
        categories = "files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[cfg(feature = "guitar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDELg5CAMQyC4MQgDQLg1DENAthaGA0CIPg8C+A4FiCI4GggIhNDIMITCCEgzGGFIUDAII0DALYNDSNRsjiE4ZhCPg0EEMoujWRo0DEN4VC4OAyDgdoNDMNA0GGRJEjaEAthcOA2kANIZkORZYjIN5Mk4IA3GiUZTlWYpHl6GZaDWXBMDKDZKDULowjKb42j0NB6h+IYmiWBIngkbZdDGXZWoKIqGoWBYHgkTQ4C4MgzCAOZMDUQaZpmY4QhQOBhDUIKmqGpqmp+bwxj0OKmhINw3jyDovDiYZXq2RAxqSOo6mOG4VCCXZdsENgtl2v6thWwpWn2QKwj2mKBiCj4kgEA",
        categories = "multimedia",
        tags = "acoustic,instrument,strings,riff,rock,band,country,concert,performance,play,lead,loud,music,audio,sound,noise",
        contributors = "danielbayley,jguddas"
    ))]
    Guitar,
    #[cfg(feature = "ham")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLgxDQNAgDIMYOhAQQ3C4Mg3hEMQwC6EA1CANIhDEIAwhILg4DUNggh2KYrCIPg8C+A4FjKNYGgiCoMhWEYTj2F4ZhuIguDMNohiOJpKDKL4si6Kg2GGGIahENJFkeIohieJ4uDIOA4i2HpeDiMYzjiN4EjmCYLDYLoriSHg0DOJA4C6LJ1DQYZMDUMIlnufZKiWXJ2DULZWlie4olqLYmoadpOo8bAtkwMAyj4LqVDWZY0mmaIFgeCRtnWJJtoUMQtDGm5ngE",
        categories = "food-beverage",
        tags = "food,pork,pig,meat,bone,hock,knuckle,gammon,cured",
        contributors = "karsa-mistmere"
    ))]
    Ham,
    #[cfg(feature = "hamburger")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYSA0GGDYNDCDoXDALQ0GiD4TCCFYXDEIIWhuGguDINQiD4PAvgOBYri6BoIgoNYODKHoghYMQtDKPAgDkIA3iOQ4iDEOJDhSQ46j4MoqiyMYwgSMoJE2NYdkmFpZkwIAzlySpDl0M4cjeYZfhYMwtl2WJKjyPBxlmSpOi2UpRgWB4JG0NguDaQoMCCegxl0NJ7jiZofC4OAtC4NBsDMLgxDWJpNiuc4vgE",
        categories = "food-beverage",
        tags = "burger,cheeseburger,meat,beef,patty,bun,fast food,junk food,takeaway,takeout,snack,dish,restaurant,lunch,meal,savory,savoury,cookery,cooking,grilled,barbecue,barbeque,bbq,lettuce,tomato,relish,pickles,onions,ketchup,mustard,mayonnaise",
        contributors = "danielbayley,kemie,karsa-mistmere,jguddas,jamiemlaw"
    ))]
    Hamburger,
    #[cfg(feature = "hammer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDILQ5C4Mw3DMIIShQMxhDGDggDCHoOC0MwuDAMAxiITIPhcIg+DwL4DgWLYwgaCIKDEOIOg0NAtDSLIujOMoEjSCRtDIMQug0MZHDULZHDkMY7k6UBBDIIJVh+H4cDEOQgDgLgxDcMh2C2Ew0DQYZVleIAwmQNQ4DaTQuDSUBsnENg1Dedp4EENggn2WIgg+SAxn0MxIDkbJHDINIWoqjJ8n6a4diqXg0DgNRWDEMBsmkaJHmAMpolakpanKUJIm+KZekkNJIj6L5CD6AQ",
        categories = "tools,home",
        tags = "mallet,nails,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Hammer,
    #[cfg(feature = "hand_coins")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDUaAyGEMgghQMIOCCFwwC0NBoC0MxjC0Lg2hkLQxC6JwyiYLg0iMTAzg4NwiD4PAvgOBY0jeBoICIbQ3hWDYnDaKw0GMLgziKLQ4iKJInDSTBokWJ4NhcMookmFQukuKRsi2Q4tDSE4VhmZIblYNw1C2Vg5DEbIcC6FAzC4OYzjWOo5gSO4JG2FAxiSf51jaeY0GMaRyGMbBlCAYx4gmfgiCAcoJmukBjHmCZ0jQL6FoeiaEoaiKKpIIgzpWjQiDalaXCINaBpyoQ+gEA",
        categories = "finance,account",
        tags = "savings,banking,money,finance,offers,mortgage,payment,received,wage,payroll,allowance,pocket money,handout,pennies",
        contributors = "danielbayley,kayleyhill"
    ))]
    HandCoins,
    #[cfg(feature = "hand_fist")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgwDMNQgDEN4OgwYQzCCGAwCCGwwC0M4fGwLQuDMMQxiMMAwDIYQuDeDYuhyMYnC4NQwDULQxC4MgyDkbI5DEOY3j+QRBDIIJGh2EoSDALg4DIOISDESIWkaSIxh0LQ0EgOY6DQM4XhmV4cC2DYMg0OA3j2ZItDAN5HmwNxBDSTYYnMOIamKGAxDQYZQlCSYbn4aIrn6YqBC0OBWDcYZVjKYw0hwdoro2GwxjGkAwCIPg8C+A4FpunoGgiCgxDOTQ4lCXA2DYMpFkehoSm6iQ1q6VqWpWGA1pqnKhqCBKigkTQ5CCtKUkqG4RDUVgxpmm6dr+voFgewbDDcVg0rWjqWqUIA0oqOg2Diu7Pp+AQ",
        categories = "social,emoji,communication,sports",
        tags = "clench,strength,power,unity,solidarity,rebellion,victory,triumph,support,fight,combat,brawl",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    HandFist,
    #[cfg(feature = "hand_grab")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDELg1FYORhDIIIWDAIIZDALQyh2FYXhqIochYMh2hANAiD4PAvgOBYri6BoIgoMQ0g4MBWDiIIYiOHYfiWI4ah2Fx2DKKosjGMIEjKCYLhkOQuDmE47kGJI/iGG5CiUdg1keLZLkqBYHk0NoODSVJZlYMpoj2JZekmApLmONINg+bAxg4II2DAdgzGGDYNlkMQtoAaAtmegJBoMOKEiGPJ4nie5vmCAQA",
        categories = "cursors,design,layout",
        tags = "hand",
        contributors = "ericfennis"
    ))]
    HandGrab,
    #[cfg(feature = "hand_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDQaAyGEMgghQMAgheGQthALQzGMLQuDaGAtDELolDKJAuDSIRMDODg2CIPg8C+A4FjKNYGggIhtg+Kg1g4MwuDMOQgDULgwDWG4hDkNBDDIMImDmIg4hWDQ2C4OI/DKDZGDcNYTC6XoVmGP4ZiOKw3DkN4pDgMw3C4Mg3lecYimYMYgDQMJ1EEMpkmOYpmheDJFmQY6CnCWAwhSfQyDSVIlloLpMDYTAxiKDKSDWMYzjiN4EjmCRthQMY/iKMIyjSn6egWB6hDeFaHDaKQ0GOQp4liIKXiquRorSJYNhefZ3iqYw4ikMhsiusorDQYYNsCGIjn2Xgtn0OQxpuqY2gE",
        categories = "social",
        tags = "love,like,emotion",
        contributors = "danielbayley,kayleyhill,karsa-mistmere"
    ))]
    HandHeart,
    #[cfg(feature = "hand_helping")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDIaAyGEMgghQMIOCCFwwC0NBoC0MxjC0Lg2hkLQxC6JwyiYLg0iMTAzg4NAiD4PAvgOBY0jeBoICIbQ3g4OIOiOKw0GMLgziKLQ4iKJInDSTBokWJ4NhcMookmFQukuKRsi0NociyE4VhmZIblYNw1C2Vg5DEbJghQMwuDmM41jqOYEjuCRthQMYwiQNp0jaeA+gEA",
        categories = "emoji",
        tags = "agreement,help,proposal,charity,begging,terms",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    HandHelping,
    #[cfg(feature = "hand_metal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDILg1FYMQwGEMgghcMAghoMAtDKHoWhiG4jh2FwyHYMQuDQIg+DwL4DgWLYwgaCIKDENIODEVg5iGGYOhsLY4DAdgyiyLozjKBI0gmC4ahSERWDWPYjDGQJCHYOZGi+SpJgWB4JG0N4ODULYpDcNplC6Z5ThyQIQDgM4YC4OAyGwMwuDYIJ3DYQw3hGGIpjcIA5C6JoXg+GAyGgMhhg2DZthoOAtDgVg3myP4dlcNZakiAQ",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[cfg(feature = "hand_platter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzFYMgiD4PAvgOBYThaBoICIbQxDULg0CAMQ3iCDguDILQyC4OBhg2DQwiKMYqDgIIqDkbAtDMLg2iYMxjC0LokDgLQxkGIonimK5HDIaAtDSP5FDEIAwkkMZADSSZDkUMo4kUMwwiiRQ0DYNBBlKUowmkIA2C6HpnDkSA1hKFIZhiBIagkTYNDENBogyLY1lOgpoCCfYpnOFZ3naBYHnmIQxDCfg2oidYCnejYKDWIgwGENwgp6apniEMKUoqlqMhsTaanwdg2GGZ6CjCVpnEiEYTomF4BA",
        categories = "food-beverage,people",
        tags = "waiter,waitress,restaurant,table service,served,dinner,dining,meal,course,luxury",
        contributors = "danielbayley"
    ))]
    HandPlatter,
    #[cfg(feature = "hand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDEVg2GEMgghQMAghcMAtDKG4ThWGIghqFAyCIPg8C+A4FiaKYGgiCgxDSDgwFYNIehaIYbh2I4hhiG4VHaJImiiBBoiuRIHgmC4XDEMAuDWEY2jyIo6h+GY9iMdg4iWJ4skaBZIi+DQ4lGS4OCCMQwHaEpijwMQtmIaIbGOGwug2Gg0k4LZ1DYLQ1C4OQ5nQMw0GwLQzC6fKHhKO5WDGFZ1DOdA4DITA3g4NZbkOKoB",
        categories = "cursors,accessibility",
        tags = "wave,move,mouse,grab",
        contributors = "ericfennis"
    ))]
    Hand,
    #[cfg(feature = "handbag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4MA0DgIAxDgLg1DYNhBDIIIZDAIIchwNIaDEaAxDYYYZhuHYpDELg5DUMgtgwNAzDQbIwC0OYYhqKYehGEA4EiJYnjuHQtiuLYZiuFQ2HoIg+DwL4DgWTpRgaCIKhAMQxFaJYgiCPAxCCEAwHYNZNk+VA+gEA",
        categories = "shopping,transportation",
        tags = "bag,baggage,carry,clutch,fashion,luggage,purse,tote,travel",
        contributors = "jamiemlaw,karsa-mistmere"
    ))]
    Handbag,
    #[cfg(feature = "handshake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDcIAyhEYYNg0MIOCCFwzC0MwiD4PAvgOBYfiKBoIgoMQ0g6KgyC4NYRi6FIYheFgghsMxshwLg4DiOo8GEM42hmQwwC0NAuDKKgwjmOw4k2Mo1hWHIcGyLQ4DELZWDEYQ1C4Nw5CCXZfkSGA3C4MA2C2Ow3GwLg0mYMg4GGEoShedoOm6LQyDUTAyg0NIeiCJYkgSJoJG2fpChUMRolmgYhoWhIFgeCRNkGQYSikbA2i4IKcDWUJDjWN6PoOAqFpQIqWCANBoDipaRgEA",
        categories = "account,social,communication,finance,security",
        tags = "agreement,partnership,deal,business,assistance,cooperation,friendship,union,terms",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Handshake,
    #[cfg(feature = "hard_drive_download")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYOAiD4PAvgOBYThaBoICIbQxDYIA2C0NAgDSIoihKFIZhMchlGMdAgHcaRkgWCQyDAIggHiNY4HmCQxDSOByjoIgyjgaBlGkZxoHSCYRhML4si6GIEhqCRNh8MQ4GgLgwDGKIVlSU4FgeVgxDAIJZluXZfiqAQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIA2C0NIQhAIA0CIPg8C+A4FheGoGggIhNDEMggDIdg4haGIdhcchlGMdAgHcaRkgWCQyDAIggGgZRpGcaB0gmJwgHmCQxhUIB4jWOBykgIgyigL4si6HIEh6CRNg0MQ4GgLgwDGT4qgKVIHlYMQwCCWZbl2X5UD6AQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDYaAuDAMQiD4PAvgOBYWhmBoIgoMguDIMQyg4MQuDUNw3GGJIkg2LgtiGIwuDgOQ2FYMQ4isIItCCLo7jsaIPjqPI+DILQyHYLQ1icMoqiyPZQDCMIikaM41EyOInDUIJLDEMRBk+PoNg8Lg3DYIA0EgN4hDSQ5Rj0LYmDcOYOC6Xh6hWF4chuBIdgkTYiC4OQ0meMoTDMSIgDANQ0nmGJ9nyBYHn+hYQhKFIWo+GoB",
        categories = "development,devices",
        tags = "computer,server,memory,data,ssd,disk,hard disk,storage,hardware,backup,media",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    HardDrive,
    #[cfg(feature = "hard_hat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMFYNRhDGDggg2DYTDELQxGgMoShSFoUhgdg1CIPg8C+A4FiaKYGgiCgxDQIA2GENoyhWN4TjUNh2DOJYniyK4Ei2CRNjEMQ1HYLQzjSNogjkLQ2j6KJCiYchlGMdAgHIeIJDEIggHmXYkCCXAiDKXx3GkZIFgkMgwl8aBlGkZxoHSCQ0lKVpYD6AQA",
        categories = "tools",
        tags = "helmet,construction,safety,savety",
        contributors = "Andreto,ericfennis"
    ))]
    HardHat,
    #[cfg(feature = "hash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig0CIIB5gkIg5g0eQygqEoHhUIgyDAIg+DwL4DgWHohgaCIKgyDoQDENYThmK4NHiGYbh2H4kiOBIlhkOIThAM4wiqHIOjIMY0iCOI3gWB4qDaMIuiiD4Kj6QoKDKRIekaIoBA",
        categories = "text,social",
        tags = "hashtag,number,pound",
        contributors = "colebemis,ericfennis"
    ))]
    Hash,
    #[cfg(feature = "hat_glasses")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDgYQyCCEgwCCFQwC2DQwCIPg8C+A4Fh2IIGggIhtDEOYODELQyC4MYrDYLg2DUN4RhOFo4hiLQ3DUMgtDGLg0DgbI/C4Mg3jENgxEGEoUjgMYOhINBIDgLg1jaToXkUOQyDWDguDQNQ2EyXovhyHojiKBIkgkTYSi8aAyhuHYfmuHRjGkchjGwZQgGMeYJg8IggHKCQzoMYx4oEN5nC+eJ6nyd55nufaFCKh5+ooIqMn6gAioKdKPpQPoBA",
        categories = "social,account,security",
        tags = "incognito,disguise,costume,masked,anonymous,anonymity,privacy,private browsing,stealth,hidden,undercover,cloak,invisible,ghost,spy,agent,detective,identity,cap,fedora,spectacles,shades,sunglasses,eyewear",
        contributors = "karsa-mistmere"
    ))]
    HatGlasses,
    #[cfg(feature = "haze")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1C4MggDaDQgDELg0hKFAiD4PAvgOBYZhyBoICITYODEMxoDKGIah+HoEiCCYjDCEomiiGYbi2LIFgeCRtDEN4UCCPg2hYNAthMNIpjaHYCi2OoiDKJA3EiNIqjeS45iGI4ODIMZSkiK5Wi6IgxkKJRhhWFYxmkLQ4CAMJelWH5NE0MYODUVgyC4NZvh2AQA",
        categories = "weather",
        tags = "mist,fog",
        contributors = "ericfennis"
    ))]
    Haze,
    #[cfg(feature = "hd")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDISA2CIPg8C+A4FhSF4GgiCoMg4NRWDmE4VhqGYEhuCYLDSDg0C4NRhi6Lggg2NAgjENRoDEYQyjKPA1jOQINj4LY+HYLQxEGPggkqNYNDENYyDkaJHjCUI/k0LY3HqI4WieJoFgeKQ2h+IZciUPByGUYx0CAeIJDIIggHmCQ1nEdxpGSBZvDCcRoGUaRnGgdIJDENJxHKbginCFAvmmaw+gEA",
        categories = "devices,multimedia",
        tags = "tv,resolution,video,high definition,720p,1080p",
        contributors = "ahtohbi4,jamiemlaw,karsa-mistmere,jguddas"
    ))]
    Hd,
    #[cfg(feature = "hdmi_port")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA5GEMQghEMAghQMAtDGGBIDOEIShWH4XhEMR2DSHYTiCHgxGgMRsg0MorDKLQti8MYmiiFIZDEWgiD4PAvgOBY9kCBoIgoNwuDWEovDmPI+kMPoBA",
        categories = "devices,multimedia,gaming",
        tags = "socket,plug,slot,controller,connector,interface,console,signal,audio,video,visual,av,data,input,output",
        contributors = "danielbayley"
    ))]
    HdmiPort,
    #[cfg(feature = "heading_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHgkbQxDeDQyCAMwtDIdoRi6JIBA",
        categories = "text",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading1,
    #[cfg(feature = "heading_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMgxisaAtDQYwwjkIA0C0M4+C0NggjwMQuDULQykqSI5C0MYii+F4B",
        categories = "text",
        tags = "h2,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading2,
    #[cfg(feature = "heading_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMQ3C4NYNDCNxjDELg3C0MQgDONwgDCQpEj0NRhDIIJMkaRgxC2TAyiKL4XgKJozgqNYNjYNRjkySQggwLgzmOQI3kuTZFmwMJSlKVYkgE",
        categories = "text",
        tags = "h3,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading3,
    #[cfg(feature = "heading_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgVg2CIPg8C+A4FhSF4GgiCgxDeDgwHYMxhDGDggDCJ4piUMRoDOE4VhqGYEhuCRNDKK4hDiL4WjOMoFgeNQ0g4MhojqFI8hiAozkCCpCg+EY7jGAQ",
        categories = "text",
        tags = "h4,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading4,
    #[cfg(feature = "heading_5")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMQ3g0Mx2C0MxoDSIovheAomjOCo1g0NwuDcYwuDQLgyC4OAuDODZRlGUw1CAMAgk4NwtDELgxlqSAtk4NRTDEOZPjeK5VDEMxIjWPokgE",
        categories = "text",
        tags = "h5,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading5,
    #[cfg(feature = "heading_6")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmhMYxpHIYxsGUIBjHmCQxiEIBygkMgijgeI7DmIgvjKNI2iWBYHigMgwg0MBjC0MggDILQzCAMwuDWWQgi2I4wgE",
        categories = "text",
        tags = "h6,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading6,
    #[cfg(feature = "heading")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQyGiDgiD4PAvgOBYThaBoIgqDAyDAVg0hKFIZhiBIagkTQxDgIIeiCIoViYPoBA",
        categories = "text",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading,
    #[cfg(feature = "headphone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDQaAtDELgzDQMwiD4PAvgOBYZhyBoIgoOQuDEMg4CAMwuDQNxBDkIIuDAIIxg2DIODIdophWF4ZhuBBoh6PoHgkbQyCCRQyjGSIYhqH5AgWQoKkiKoPkYMJTDQQZHjKW4NDGLoMhEMRhlqMYzC0MpnHYLY7kyPpOiCCRNDODoQDOY5GluM54jedpklyZ5GEgNZ3kWZYOmeaQtDcYYujCXJGC4NgzDYLQ2hQNg0kuPYdgEA",
        categories = "multimedia,connectivity,communication,devices,gaming",
        tags = "music,audio,sound,mute,off",
        contributors = "colebemis,csandman,ericfennis,jguddas,Need-an-AwP"
    ))]
    HeadphoneOff,
    #[cfg(feature = "headphones")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQ0GgMxhDIIITDAIIWDGFIUHaEYThWF4NC2HhIDWEoahaGIiiIdgtDcYQ5CCMIog2DQ4hcdouh6IIph4aAtDGJofjyKwth2J4ghkMoihAIg+DwL4DgUPoB",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Headphones,
    #[cfg(feature = "headset")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQxGgMxhDIIITDAIIWDGFIUHaEYThWF4NC2HhIDWEoahaGIiiIdgtDUWhtigYQ5CCM4Yg2DQ4heMIXHaJYeiCKYeGgLQxiaH5BisLYdieIIZDKIoQFoIg+DwL4DgWVJXgaCIKDKGQxDYdgyGENAgmWKIhmUNJDDWU5VloPoBA",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound,gaming,headphones,headset,call,center,phone,telephone,voip,video",
        contributors = "ericfennis"
    ))]
    Headset,
    #[cfg(feature = "heart_crack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg0DAOQgDULg4DINBjC0Lg3DCDQ3DkMgtDELgxDUIIiDQOQ2iGDokCCDQxDYNhsi8NQzi6Iw1DYYQuhOJQwCCPwxkAIIaDANxsC2DQyDmNpKkyO49kOP5TkSGw3EyDImDUIg+DwL4DgWXZggaCIKDEM48DAOIuDALgzmcYQyi6UomC2NguDAMQ5EyJYkhiIg1iugQznYLogoSPRhlGUZUkIOY8DkMaFDYNw3C4NqXnSP4UDEOJ4nkQaLjydJCDKcqPDUY4/k6gggDSdoSjwepcl6Yw+gE",
        categories = "emoji",
        tags = "heartbreak,sadness,emotion",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartCrack,
    #[cfg(feature = "heart_handshake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg0DENAghCDoQEMMgxhIMguDgMg4CAMgyhIMQuDWH4hg0NRhDWJAgiuJQwCCMAwC2KA5DELQzC4Ng3DaOo6jGQI3hsMQ4C4MAwDEQYui2LIykCJ4kGOMIaDOEoshGVYuGyLgziWKwzDYMhhiGIZOlOGw3g0MA1mQLgxm2b5AjILZGDANI4h+bgyhGGpvhGMIYjAM54n2e55n6cpAjmR5/h+jpmjELYahwOBsC2Iw4DiN6YhyY4UnmD6JjOOQ0DAOYxpaIw3hiqgxmOj5BpKG4dk+sKApGk4dGyuaDhqPAyCIPg8C+A4FD6AQ",
        categories = "emoji,account,security",
        tags = "agreement,charity,help,deal,terms,emotion,together,handshake",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartHandshake,
    #[cfg(feature = "heart_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg4DcNggDEOAuDkOQtDELgzDYOIShoMgzGEMggiMMAgiYMQtDMIAuDAMQ5EwNYSDUY4YC4NY2jgM4qC4MoqC0NY3GGQYykSJ5HDEIA5jcOYpDMLg2hCNw2lOR4miaDoTicQZBDQOQgl2X5XhKJIjksNZDj2YJqmOKY9DQNIeDUNwyCIPg8C+A4FneeoGggIhNDGMqCGgNp2nifQ+gE",
        categories = "medical,account,multimedia,gaming,social",
        tags = "unlike,unfavorite,remove,delete,damage",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,Ayberkyvs,jguddas"
    ))]
    HeartMinus,
    #[cfg(feature = "heart_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CANAuDgOQzGENYOCCF4PDAIIcDEIAxC4MA5iGFIhDUNoODaHYshyEgxDiHRBhcNA5hkLo1i2IAgDIMggDmDhjh6Eg3DILYhDAMI+DMLgzigLQyC6MA3hALg2DUNQiD4PAvgOBZbl6BoICIbQxikOQ2lSZgumgNwtkwNA1jaTAzDQNhhj6Poch6bwgiIMQ5EyDwxDUY5Hg6hw1m+bwukYMwthqFoYhqOoflENw0m6EQ3DYMZalyYZggSYoJG2eY8hwMgwp+XajD6AQ",
        categories = "social,multimedia",
        tags = "unlike,dislike,hate,emotion",
        contributors = "karsa-mistmere,ericfennis,danielbayley,jguddas"
    ))]
    HeartOff,
    #[cfg(feature = "heart_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg0DcOQgDEOQuDMNw0C0Lg5DcMYaDMORhDIIIiDAIIlDELQzCALgwhMTA1hINRjC2HQ1jQLo2DOKQuDKKQtDWOBhkCMJDiaRgxCCFA1DmKAzC4Ng3DaOJSDUNpGiWJQuDgMQ4iYQZADSEZghGWISiOIpKkKPAgkCJJHhkMoThKDg5CIPg8C+A4FneeoGggIhNDGMKCGgNp2nifZ8gSfoJoGXQxDIdqGneeaLD6AQ",
        categories = "medical,account,multimedia,gaming,social",
        tags = "plus,like,favorite,add,health,support",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas,Ayberkyvs,UsamaKhan"
    ))]
    HeartPlus,
    #[cfg(feature = "heart_pulse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOQuDUYQ1g8IISDUIAwhcIAxg2Dw5DELQzC4Ng3DaD4lDUNoZhiGAuDgMQ4hcQYSDQOYUC6NIqhoIAygyDg1GOGAyC4Mg5C0MYTDSII2DUbAtjMOYMhIMwxDMYYMgyK4akoLgwDEORMhYMY/kaD5kDWIIgkOaIVCIPg8C+A4Fm6cYGgiCohjyGgzEiPhsmWG4MDSEwyC0N4ahOIQ1GiEgyDebZvnQPoBA",
        categories = "medical",
        tags = "heartbeat,pulse,health,medical,blood pressure,cardiac,systole,diastole",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartPulse,
    #[cfg(feature = "heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOQuDUYQ1g8IISDUIAwhcIAxg2Dw5DELQzC4Ng3DaD4lDUNoZhiGAuDgMQ4hcQYSDQOYUC6NIqhoIAygyDg1GOGAyC4Mg5C0MYTDSII2DUbAtjMOYMhIMwxDMYYMgyK4akoLgwDEORMhYMY/kaD5kDWIIgkOaIVCIPg8C+A4FD6AQ",
        categories = "medical,social,multimedia,emoji,gaming,shapes",
        tags = "like,love,emotion,suit,playing,cards",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Heart,
    #[cfg(feature = "heater")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA4GMMgtDMLYRDMIAwC0NgiD4PAvgOBYch+BoIgoMQ1C4NYOhCEoUhKF4ZhuHYiiGBIjgkTQ2CAMQwGgLgwDGMYejWNIFgeN45DENI9j+QYzgKNZGiUMI6DYdgtDSTZDk+RYkgsNJUlaWIckKIJbjaJQ4mCV5ZmWIpRE0MpTDYYQyCCdZTlODZ1DIdo7nSdoXoEMYUnYSA0n+d6Ci0MhWDiiKBnmdoUGgM5sGiRJnE2KZxHYMqWpibwxDmdgwp2n4B",
        categories = "home,devices,travel",
        tags = "heating,warmth,comfort,fire,stove,electric,electronics,amenities",
        contributors = "danielbayley"
    ))]
    Heater,
    #[cfg(feature = "helicopter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDcdg0CIPg8C+A4FhSF4GgiCgxDQIAzHYOBhDIIIlDAIIoiiJQyGgNQuDgNg1hOFYahmBIbgmCw3g6EIShSFo4jeBYHjoMQ4j0YYfh+KopCANAth+SA2k6KgtDgLZUlQNZVimWQgDUdgziSJpdiuJh6jSQYYgKOJFgqJQxDAdozkCNptkSHBNlQMxoDENpqneGpvE2PAyDGfo/jWQp4jmCpIDEMxIDKgZCgE",
        categories = "transportation,travel",
        tags = "transport,flying,rotor,aviation,helipad,gear,flyer,technology,helicopter,aircraft,vehicle",
        contributors = "liloudreams,ericfennis,jamiemlaw"
    ))]
    Helicopter,
    #[cfg(feature = "hexagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FD6AQ",
        categories = "shapes,development",
        tags = "shape,node.js,logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Hexagon,
    #[cfg(feature = "highlighter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQxC0NggDYdgzGgORsDMLQzCIPg8C+A4Fh2IIGgiCgyDKDQyC0NAuhGLA2GGKIoDAII0g8MguDiNRsC0NQuiqPgyjEIIzjWDY1C2OA4EwMQ0CANIch6Iw+gEA",
        categories = "text,design",
        tags = "mark,text",
        contributors = "lscheibel,Andreto,ericfennis"
    ))]
    Highlighter,
    #[cfg(feature = "history")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQyGEOQghEMINCCFA5C2EQ5C4Nw1hKHIehSIgtDaHA0CAMomEyDA4CIPg8C+A4Fi+MoGgiCoMDMdg1GgNYujCNY0gSNoJE2DggDeOxsicMo/jGQw+gEA",
        categories = "arrows,time",
        tags = "time,redo,undo,rewind,timeline,version,time machine,backup,rotate,ccw",
        contributors = "ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    History,
    #[cfg(feature = "hop_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg4DIIAxDYLgxDIYwxC4Ng5hkIAzC4OYYDcOQgDWFA4g6JQyicMIYDUMwtC4MIbDeMAyDcIg+DwL4DgWOY8gaCIKDGGAxDQIAyg0NQ3GMLg1DILgykaTw0kaRIQDSMQ4hELgzDcLg0hMMITDgNo1iUOYwDeVgti0Mowi+Hg0i+TQtliKo4jqP4+gSQIJguEwxDOR4YDANYXhmJQ2oKHg2icOJGlgOJeiAYYfh8IAwpimgujSUA2niO58nuBYHn4MaTiOJZOGGSIOoKrQ4oKmaZDGHYUDUIJYiWjqarSMKNlugRjmyFJeDELZPiUMJuh6aJQDWoJ6gKfKlgqrZKhGRA0kyUZNhCVpHl+VZcDeuZZjGWJzDCJw1s4M4htCOahj206kkETZYDmgr5DMYYMhGs69r+5aBl+TAziWcofhO46njCFYZmiIa1oSNLKwSMY0hOapbDOWA0riHoYiOkYnDYMqVDiu64wGmZfDiNQ0tGor1n2Cqqt+qKHoWH64k+ILhDSV5NrYMQ1ymDq9pqRJzo23LIlwNJvi+JYvi2NInne8rSj+1RNifCLho2/q0y2EQghsOZai0OJMwrCJfwvD7fsWxKJnWXIfzCYcYoXM7012QRthDhKZkjfxoD6AQ",
        categories = "food-beverage",
        tags = "beer,brewery,drink,hop free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    HopOff,
    #[cfg(feature = "hop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg4DIIAxDYLgxDIYwxC4Ng5hkIAzC4OYYDcOQgDWFA4g6JQ1iUMAzhELQuDQMofDeLw5jQLgwDYLYYDKNwyjqHg1i8OJCiUMQ4CIPg8C+A4FkqTYGgiCgxhgNQghMNYXhkNAgDCJIvDMOJXC4NwxjuOA3C4NQyi+MoqkOMoVmcMQ3EGDIRl6eZdCANAuj+EYnDMMxjC4M4lDSHg5hOXIYnSL4VhkOZnmWEZoC2Kw3pUMI0hOlIYDOfQ0laHoYiOfQ4icNgyGGDonqie56mQM4vDcNKErOcQ0o+QpBi8NpmqeSJKkyBBok+xYHgmC4lqqEYTDmWZ+iiGYQjIN5Ynyfg4qwN5pt2sIuh+ZLOlu5A2lyerAjgOJnDMNpJkuULHgWyZThMMYtDKGAwlmGA2syLYeDarpcqeaYgqyG4jnqXofjrDoRmnA8RhmYsMjULcGvCxJOgKyJSguz6MsyFoOhO1ZkxKEJ9DKGw1ty3qZwwIJkjW5qMhO57gDC7Y6n0MLCvGxbzlGyp0h+I4pqsMoNg8INMg6ELph2FJWn2JauuCZsVpW+BjC2MqCn6l6FmCbInj2Zw1DfG7yx69Mgn0OYt3LAYdty49TzwMdt0Pb9FgrLpipCR5aDKXIyDng8piSH5WlWmdotuDoO1qc8CmLUNO5rUp7uqvNV32ToB",
        categories = "food-beverage",
        tags = "beer,brewery,drink",
        contributors = "karsa-mistmere"
    ))]
    Hop,
    #[cfg(feature = "hospital")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYNAiD4PAvgOBYThaBoIgoMQ0CAMgxHYLQzGGDYNDAIIoDALYeDAdgzhKFIZhiBIagmC4eDkaIsjGFY1jSBYHjcMQ4CAMQxGgMolh+KZNDGTAyHYNpLieTgtiYSA0lSTYoDGV5XiIOZbiqRofleSY9jOAo1kKHJFiAVg1mOXJfDISA4nOKpXh8dgxDaaY/gEA",
        categories = "medical,buildings,navigation,travel",
        tags = "infirmary,sanatorium,healthcare,doctor,hospice,clinic,emergency room,ward,building,medical,vet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Hospital,
    #[cfg(feature = "hotel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDIdgtDYLg1DcIg+DwL4DgWGIbgaCIKDEMggDEMRoC4MAxheGYeh2BIfgmC4jDeJ4piuGovi6BYHjEMQ0iQNQuDQMxWg+N4tgKL48iENYkDYYZNk2DZThIIAwkeOZJjuIILDaJImiiKoYjiHJajCIZejSYZYmWHpLE0OJfjWYoslmbpcnGao2mOSByGUYx0CAch4gkMgiCChAiDShx3GkZIFgkMQ2oceaFocaBlGkZxoHShZXmOfqAD6AQ",
        categories = "buildings,navigation,travel",
        tags = "building,hostel,motel,inn",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Hotel,
    #[cfg(feature = "hourglass")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgyGgMQ0CIPg8C+A4FhSF4GgiCoMg+EYThWGoZgSG4JE0MQ3g0Mh2C0NAuikMhhDKDQgDCNo2C0Lg1DgNgtDELg0hETAxjSRRsi6QYRCCL5CDQQY0jSN5TCCKopC4OAyDgVoOiGFoliSBYHieKosi+MYzjWVI3juPQgkCTpEkYMhsk2EZJk6UJqjiN4pCANowDcMpcl6I4BA",
        categories = "time,gaming",
        tags = "timer,time,sandglass",
        contributors = "karsa-mistmere"
    ))]
    Hourglass,
    #[cfg(feature = "house_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NgyCAMQzC4OBBDILgyDUIIWhgIAwhCHwxg8MQwhMMw2GGG4ZimHYgCCEgzDiGguDkNg2GwLYWg4NoyDgNYnjMOYMDmQYsh4MQtDELg1DAN4dHoIg+DwL4DgWUZUgaCIKDOEAwiiGpFh8Lg3DAOZIkoMg4GwNwtieD4Ph6RoyDUOJvmoIA2hWX5wh8Mgxlwdg5l6b4skebhIDWgpgoWOJPlGU4EGgPoBA",
        categories = "home,buildings,medical",
        tags = "home sweet home,abode,building,residence,healthy living,lifestyle",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    HouseHeart,
    #[cfg(feature = "house_plug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIVg4C4OQ2DQIg+DwL4DgWGIbgaCIKDENIOhCEoUhaGIagQaIdiuB4JgsNYkGEMYOCCDYNjWOh2DIYQyCCP44g4LY/DIaAtDSPpAjeTAxkSRB2kSNI2kKOgtDEeoXhmHotgWL4KhKMgyDESA1kqQZNk8MpRDmZ5MjkIAuDcMA5lcLg1DIOBsDcLQ2m6VZAncOJBnsIA2EGRZvjaY4ODAdptomVZEkCR5mpGaQylCRJaiqHIBA",
        categories = "buildings,home,sustainability",
        tags = "home,living,building,residence,architecture,autarky,energy",
        contributors = "jguddas,karsa-mistmere"
    ))]
    HousePlug,
    #[cfg(feature = "house_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgzDUIAyDESA1GEMoRCAMIZCAMQtDKHh2C0OYWhiGoaDEIAuDeHQxC4NQzGwNwtDaJIXiaHIRi4OIZjEIA2EGF42huKIShwMB2g2DwiD4PAvgOBZMk+BoIgoMQ0C6O4MC4NBBiiKI3icNIcDIaAtDQYZehuJgtl4dg4kuTZSlGBJTgmC4QDEOBoDacJOnSc4Fgedp5hwNR2nyTJ+lCAQ",
        categories = "buildings,medical",
        tags = "home,living,medical,new,addition,building,residence,architecture",
        contributors = "karsa-mistmere,jguddas"
    ))]
    HousePlus,
    #[cfg(feature = "house_wifi")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4NQgDEMwuDgNg2GENAghcMAghoMQgg4LgwDEIg+DwL4DgWJIngaCIKDEMoPDcaIgiKJImgQaIpjeB4JE0M4PDAYYvi+GocCALg3DAOQtDGDQyDgbA3C2FZChuVYdDKDQ4kOUAgDYQZUkSDwgDKHQxDAdg5kGY5VhwLZCEgNZqkOVpum4eojiWKo5gWO4KDeP5HDUNBhDgIKFmGZZEniNoogEA",
        categories = "home,buildings,connectivity",
        tags = "home,living,building,wifi,connectivity",
        contributors = "akshaymemane,jguddas,karsa-mistmere"
    ))]
    HouseWifi,
    #[cfg(feature = "house")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyDEdgtDgYQxCCFQwCCGAwC0MYcGgLQ0hSFoZiSG4VhAOAiD4PAvgOBYri6BoIgoM4WDAYQyg6JYjC4NwwDmHAuDUMg4GwNwtDaOI6hqIwykIOI5DCRggDYQY5lGJIVg+Nh2DmSpYhiHZXEgNZfjuYgtDIeoqiyMQ+gE",
        categories = "buildings,home,navigation",
        tags = "home,living,building,residence,architecture",
        contributors = "jguddas,karsa-mistmere"
    ))]
    House,
    #[cfg(feature = "ice_cream_bowl")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcYw1CAMAgDgLQyC4Ng5hULQ2EgNBjhQMwuDMMQgDMIA2hWKRtC0NAgDQaA4i0NAtDMdgzE0NQuDGLwxDEYYjhKQoTg6Rg2C4N4mDAIg+DwL4DgWTpRgaCIKgyPI+kCRJEhSJomkiSoTk2T5UlOBJVgmC47hKSA1kELpDnGRZLC0N5jk6UJoD6AQA",
        categories = "food-beverage",
        tags = "gelato,food,dessert,dish,restaurant,course,meal",
        contributors = "kemie,jguddas,karsa-mistmere,ericfennis"
    ))]
    IceCreamBowl,
    #[cfg(feature = "ice_cream_cone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQxCANAuDAOINDALgzDUYYPg8MAgh2HQxC4OA0h4TAxgyDgiD4PAvgOBYri6BoICITYnCANxBDUII6h+Ho3jeKosjGMIEjKCY1gwNxhDIIJMj2HIQEiSpMk6PocC0NJBi2RQ+gEA",
        categories = "food-beverage",
        tags = "gelato,food",
        contributors = "karsa-mistmere"
    ))]
    IceCreamCone,
    #[cfg(feature = "id_card_lanyard")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg1CAOBoC0MwiD4PAvgOBYWhmBoICIbQxg8MgtDEIAyGgMxhDKJggDCLQgiWKwyHYMQ0iqLIui4MQtjISA3jeK45jCPI8FYNpAi+OomjyKIVheHIbgSHYJgsNguDgOQ5iYMhBg+D5Ci4NwujEMpOhiUpRgWB4JG2WorDMIA2maUA8GMaRyGMbBlCAYx4gkMZlCAcoJhSfB5n8NZmnaeJ6D6AQ",
        categories = "security,account",
        tags = "id-card,id-card-lanyard,identity,employee,gate-pass,badge",
        contributors = "python2911,UsamaKhan,jguddas"
    ))]
    IdCardLanyard,
    #[cfg(feature = "id_card")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDAaAyCIPg8C+A4FhSF4GgiCoMg4NIRhOFYahmBIbgkTQ2C4MQ3g4NRhDMIIxDAII0DEIA1C4NoNDCIoWiaFBjGkchjGwZQgHKCYSCAYx5gkMQxCKTB4gkOY+kKRJGhQchlGMdAgk4Ig1lKVAikscplksaBlGkZxoHSTw0lIdxpGSBZKj2FAvlyXg+gE",
        categories = "security,account",
        tags = "card,badge,identity,authentication,secure",
        contributors = "jguddas,karsa-mistmere"
    ))]
    IdCard,
    #[cfg(feature = "image_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgzCAMgxEgNRhDKEAgDCGAgDELQyh0VoUhaFoZhkMYQh0aAxDSFYXiSG4XDIdoMGwLQzC6HI2DGLIjhoMIdC4OIqC4MIqEwNoQDEIg+DwL4DgWS5OgaCAiG2KobDkIIPDMdgtDULg1kqTJRlCBJSgmVQ3hCFgzjWYZNmWSxjGkchjGwZQgHKCQyCIIBjHiCQ5nwYx5oCbpynSdg+gE",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,download,save,export",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImageDown,
    #[cfg(feature = "image_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5HYMQwGEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNwiD4PAvgOBYxGwaRuGUIB4DKCQyDIIggHmPQiDWQR5DGCZGjuSQiDENowjKN45jEYxpHIYxsjoYx4gkOZBGMeZekEco+lEL5WliWoxjSBoICIbYMCAMQ1C0MwuDAOA2naeJ6iCFoqhsLg4DIOIYEwNoVDGZ5tD6AQ",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgyDIIggHmCYLg2BwxgqEx5hWCwiD4PAvgOBYcHAYR0GgIBkgoTQxDALg0DEIIqiwMRhDIII0DCL4vC0MguDgM46jwM4bh2IokhyH4GhCFgxg6CIKDaDoYgoMQzC4NZMhmU5VkKHoEGWRpdg+WITHiWA4lCEQxlaB4RDKS4clyIA8kSJYnCITZUDUOQgngORBDELg5nqf6BCCN6GnsIA1HYMQ0jONaFpCN40DIaKMGOVQ1pCfwwDUMgtC6DIvjGn55luc4hiOdIom2Lw1FYNaOjakY6joSA5qaqQ+gE",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIIMC4MAwDMYQxg4IAwheFgxC4NQxDcLQuDgNQ5GwNAuDkOQ3CAMonimFIWhiGIVjILg3DEOBsC2JooiqLI8i+M4ZDELYbh2H4hDYegiD4PAvgOBZMk+BoIgoMoVDGLIeFYNRhDKK4ZjELQymISJcl6XoxheYorHYMQ0l2X5phiZxoDaS5NlKUYElOCRtDaK4VDULQ1neTp7kwYxpHIYxsGUIBjHiCQ5CKjx5pKlBygkMqFomi6ND6AQA",
        categories = "photography,text,multimedia,files",
        tags = "picture,gif,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImagePlay,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1GgNgiD4PAvgOBYThaBoIgoMQ5CAMh2hGE4VgQaIYiWB4JE0MgxCAMQxC4NRWh0YQyh8IAwjiLgtjYMhIDWNY3jmOQxjyPBWkCPY6kSH48GgN4xhKFIZieBYpCIbYsi4NQtDMLgwDgNpdl+YZBjaQ44jwLg4DIOI4EyDYslKJIXDwYxpHIYxsGUIBjHmCQ5CIIBygkMqCGMeKAnOd55nsPoB",
        categories = "photography,multimedia,files",
        tags = "add,create,picture",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    ImagePlus,
    #[cfg(feature = "image_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgzCAMgxEgNRhDKEAgDCGAgDELQyh0VoUhaFoZhkMYQh0aAxDSFYXiSG4XDIdoMGwLQzC6HI2DGLIjhoMIdC4OIqC4MIqEwNoQDEIg+DwL4DgWS5OgaCAiG2KobDkLg1CAM41luW5KkyUZQgSUoJgsN4QjELQ1lmYJNmSSxjGkchjGwZQgGMeYJDkIggHKCQyn0Yx4nubpynSdg+gE",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,upload,import",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImageUp,
    #[cfg(feature = "image_upscale")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgNR2DUIg+DwL4DgWFYYgaCIKDENwgDIMRoDIYQyiEIAwimKwyC0MoUhaG4agSHIJE2IggDEMh2DOMIXjSM4FgeCRtjgMwtDUIIThWP4ZgKNJDgoMwgDcVg1iaKIqioMYhi6Poyk+QodG2SY4DQLgxDQNAtmeaQ0GEMQujicY4lqOY5C4N46ikTAxlMMQ5l+QJhjWCg5g4aI9kyYByGUYx0CAaBlGkZxoHSCQxDAIggHiCY9CAeaYDGmx3GkZIFpimggHKnQiqOTKNo8PoB",
        categories = "photography,multimedia",
        tags = "resize,picture,sharpen,increase",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,jguddas"
    ))]
    ImageUpscale,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIB4goM4NgkIojgeIYLCIPg8C+A4FiwYxpHIYxsGUIBjikOYNGOJY7geCoMiwL4yjSNosHAYYSCAZIKG0MgxCAMQ1C0MwuDAOA2lWV5ZGEMggl8MAgmIMAtDILg4DIOJjEwNpgDGK4tkmEg+gEA",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Image,
    #[cfg(feature = "images")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIAxDELQxC4Mg5DaEYThUYQyC4NAghuHQwCCIQwC0M4cDAOIiEyD4ODYIg+DwL4DgWMIzgaCAiE2HQ4hqHoij+JINDIdgxDCPYNiOP5CGiRZHkCSgtDKL4xjaMBjGkchjGwZQgGMeYJDcIggGYaRsGyCRjHUchyGUbh0EMbxsG8cpiHKCQxmIYx4ncM5TC+V5ZluMJsGMdAgHKewilIIB3GkZIFncNJil+ipiokOJiGgZRpGcaB0pGfqEHQPoBA",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,multiple,copy,gallery,album,collection,slideshow,showcase",
        contributors = "karsa-mistmere"
    ))]
    Images,
    #[cfg(feature = "import")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHaDAiD4PAvgOBYThaBoICIbQ4CAMQxCANIiiILQ0hKFIZhiBIagkTYeDUSA0GGDYNDAII3DALY1hAMI0CCNo4kKNRoDENo/kGOZAjsVg3kiQo5juOxoiaKIViwPoBA",
        categories = "arrows,files",
        tags = "save",
        contributors = "mittalyashu,ericfennis"
    ))]
    Import,
    #[cfg(feature = "inbox")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgyDIIAxhMMQ2hSFg0hQNYUDCHAgDiGQghYMgiD4PAvgOBYHgmKBwGEdBoCAZIQE0NQuDSHY4DEMYkhkdg2GGE4Th+Ro/DIaIXkOP5Hh8MgtDIdgtDYbAtDOOQ1lQLg4DkQZECCToUDYLg3hgNBIDcLgyDSTJFmGYQtDGZQ5hQLo9HqJ4pjCMg+gE",
        categories = "account,mail",
        tags = "email",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Inbox,
    #[cfg(feature = "indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMxoDEMgiD4PAvgOBYThaBoIgqDA4g+EYThWBBohiI4HgkbYMDEMwgDgLg1i2EoUhmJYFieHAgisaAzjKIoXgKJobE0OY5DMYw2C4Ng2DcIAwCCSJKDcLQxk4MJTDCPY0gE",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    IndianRupee,
    #[cfg(feature = "infinity")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ2GMNQgDAIA3C0OINDKFhhDQIIchOEwxhIIA4GMLYRDALYVDgLQxhkOIbh2Iohh+IwiD4PAvgOBQ+gEA",
        categories = "multimedia",
        tags = "unlimited,forever,loop,math",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,jamiemlaw"
    ))]
    Infinity,
    #[cfg(feature = "info")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCAMQ2HYLQ0CKGwviKJIhiOJYnCKKQyCAOBoC4MAxjGHI0GgPoB",
        categories = "accessibility,notifications",
        tags = "about,advice,clue,details,help,hint,indicator,information,knowledge,notice,status,support,tooltip",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Info,
    #[cfg(feature = "inspection_panel")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIB3GkZB0GiCQxDiDByggIgygwaBlGkZxoHSF4ZD4PAvgOBYoHAYYVCAZIJE0NwgDcaAuDAMQiigL4uhWLYvGiMYzDGNY3jmO49j8aJBjCMgijQIJGjiOo8imTJOkOUBNkaU5IlaS5CD6AQA",
        categories = "tools",
        tags = "access,cover,tile,metal,materials,screws",
        contributors = "danielbayley"
    ))]
    InspectionPanel,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDAIoHDGCgxDmDh5hAIg0hSCYXCIPg8C+A4Fh2IIGhWCgyg2B4aDWGYmigeIWDGGIdh+BBliKNQgiWG4pgqE45hqJ4Oi+EYrjOIw+gEA",
        categories = "text",
        tags = "oblique,text,format",
        contributors = "colebemis,ericfennis"
    ))]
    Italic,
    #[cfg(feature = "iteration_ccw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAxDQIIQDQLYSCIPg8C+A4FheGoGggIhNDIMIODAYQ4CCJ4jDEIAwC2Jw4GgOIWhiHQ+gE",
        categories = "arrows,design",
        tags = "arrow,right",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCcw,
    #[cfg(feature = "iteration_cw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGEOAghEMINhWEQ4EgNAiD4PAvgOBYch+BoICIbYRDIMgtDSKggiuGoch6BBoD6AQA",
        categories = "arrows,design",
        tags = "arrow,left",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCw,
    #[cfg(feature = "japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA5C4NRWDIMRtDALQxDGEBMDYIAzG2HA2hoMQ4h0Ig+DwL4DgWJ4qgaCIKhwMQ1GiDImiiLYsgSLoJE2MQxjQMo2imOg+gEA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis"
    ))]
    JapaneseYen,
    #[cfg(feature = "joystick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDcYQyCCEgwCCFQwC0MoZEgNYRhOFoghiEgyHYMoehSIYfDIaAxDSJ4phWGokhkWgiD4PAvgOBY3jqBoIgoNoODUdoZjaOI9jyBI+gkTQxhIMQ1FYOZGjmSo3GMaRyGMbBlCAYx5gkNgil4eIJk6YxygkM5UliWpcD6AQA",
        categories = "gaming,devices",
        tags = "game,console,control stick",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Joystick,
    #[cfg(feature = "kanban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMx2DENAiD4PAvgOBYThaBoIgoMQyg0dg4hKFIZhiBIagkTQxDmHwxiGE4ViYPoBA",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Kanban,
    #[cfg(feature = "kayak")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDcYQxg4IAwhSFAthIMR2DEYQyCCHoVhKFQyC0Mh6CIPg8C+A4FimLIGgiCgyDALg5DcIAzC4NocC4NA1j0NYWhUMAtC4NQ4kWRxDDGNIeDaOggk8NoOk2OAuDAM4fjQMw5GGQJAkKFpGDiY5LjmDYPj2Dg3moMZnlqNY3jmO4oiqL4ugSMIJG2Tw3DCN59n+VJjlOTKEnWK55niBYHgkTY3DWHYfhaIolh8aIchmYYhhiJ4pomLYBA",
        categories = "transportation",
        tags = "kayak,boat,paddle,water,sport,recreation,adventure,outdoors,equipment,lake,ocean",
        contributors = "jguddas,jpjacobpadilla"
    ))]
    Kayak,
    #[cfg(feature = "key_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQ4DYIAxDcLg0DENBBDIIIZDAIIchyGQxDgLg4DIOBWDIMRhDGEYdi2HIrDEaAziqLIei0MQtDEdo5jSK42jCORoimMIujeOY7kONY3hGQQuhIMhhhmG5FDGFIWC2DYPGyI5XlwNBhDaDQgmENZLDALQ0mgegiD4PAvgOBZtGMaRyGMbBlCAYx5gmEw1CIIBygmDZ/GMeIJDGZJ/GYaRsGyCRjHUchyGUbh0EMbxsG8cpsm6c51ncPoBA",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = "danielbayley,jguddas"
    ))]
    KeyRound,
    #[cfg(feature = "key_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg0CCDQ3GGDQ1hALoVDAIIZDEIAzg6GhsDWFwgiINYTiOFIaiqHIZh4NBsC2Hg3h0LoSimKYZhuMYfDATA4jUIA5C4OInhWOIrhqOw0HoIg+DwL4DgWTpRgaCAiG0MYPjMM4dk2T5UlOBJVgkbZCg8MQwC4NgtDaQ5ZCCbQ4lkQQyhCKo5nYMY/DgMg4FYMgxGGHIsncIKDGgM6CoahY6DEdgtoGg6FoOkBopGi54o2j6XoSG6GpULgxDcMoTnamaGg6WQthcOA2Gybg0qucg0l6UJiD6AQA",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = "danielbayley,jguddas"
    ))]
    KeySquare,
    #[cfg(feature = "key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDULg1CAN4OCAMguDOE4VGEMQghoMAgh2HQxC4NIeGyFAxC2JoZhuHosh8LYhDQTAxDkIA0CIPg8C+A4FjiO4GgiCgyhoMgtDkLg2CCRg2jeOY+jgYxpHIYxsGUIBygmDQ1CIIBjHiCYRlqXB5gmDIOkwL5QlKVA+gEA",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis"
    ))]
    Key,
    #[cfg(feature = "keyboard_music")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAdxpGQdBogkMgwgweITgwaBlGkZxoHSCQxDaDByhcIgyCIPg8C+A4FikcBhhEIBkgkTQ2CAOBoguKQvi+EYujAaIyjQMQ0jcaAuDAMYoiqPRoj+MYzCITQxDiRpIkqO5Nk+QZRE0MggDEMhohSS48kCW5ClKNphHaOpMmcPJNmmUwwmAMptmWWpxkCc5EnaeJZnCcpdlSf5umaPoBA",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,keys,chord,octave,midi,controller,instrument,electric,signal,digital,studio,production,producer,pianist,piano,play,performance,concert",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    KeyboardMusic,
    #[cfg(feature = "keyboard_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQgDIMAgDQIBBDKDAgg6DgxgyEw2CIPg8C+A4Fh2IIGgiCoZCANggEyJwxDYLg0DGHIeiOIoEiSCYLDcIItiqO4pi2MofjaNYFgeOAgDkLg2DmD49g2D5BjSAo2kaCgxhAOBoC4MIxh2QohlORYlE0MQ4CCWZbl2M5DmGNwiG2E5xg6DZRmyI5VE2T4NEgNBhnGFaADELQyoMVg2n6FIWjuDKDnWYJ3mOKZolyjhokSbhNmYMQylqlJejSAQA",
        categories = "devices,text,development",
        tags = "unkeys,layout,spell,settings,mouse",
        contributors = "Diottodev,karsa-mistmere"
    ))]
    KeyboardOff,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA4GgLgwDEIg+DwL4DgWFYYgaCIKDEMggh+EIShSFobhqBIcgmCw0g6I4ThWF4piiBYHisMQ2iEMoviWMoZgKKY2h4OIuhGMImjOQI1h0TY5g+Ro9ieSoqgoN4hDYaIMlGSYbkITZEiKUIxlIchlGMdAgHiCQyCIIBoGUaRnGgdIJjibR3GkZIFmsMJtHKagimwIB5gkNI9mWZw+gE",
        categories = "text,devices,development",
        tags = "layout,spell,settings,mouse",
        contributors = "it-is-not,ericfennis"
    ))]
    Keyboard,
    #[cfg(feature = "lamp_ceiling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYNQiD4PAvgOBYThaBoIgoMQ0C4OAyDkIAxDULg5DkOBhDMIIrDCI4jC2JQ2DUOAgDCEoUhmGIEhqCRNDIMImg2HQuDYMA2EEMYvi6LpKkCIw2EgNBhkqSpMjCQgtDELgzDkNBsDMLQ3kmS42i8NwgDcaAxDCVJlk0IJCkWRx6jiFY8D6AQ",
        categories = "home",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    LampCeiling,
    #[cfg(feature = "lamp_desk")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgyDkMwgDKDoQGEMQghcMAghqFwxC4NAxDSGxshMNYSC6Jg1C4OQ5iKHgyDIN4WhiG41heKAwDaGAuDYOA3GwLQ3CCModjWHAth6PQ3C2OA2kCL4xC2KosDQLYllaKIzhmNobkiH4hHoIg+DwL4DgWY5mgaCAiG2IYODCQg0C4N4QC0M5fiKd4gDSYpkmmaIEmqCRNhEMgwGEMoSkaNAylYaA0oiioahyigyHYMZaosMZIhgSKQkWk4YkiSJhmOZaBoCBYHgkbQ5C4MA4joNooC2cp0hGtoVqCRpMDEOI7DENxMkKvp9qeZ4BA",
        categories = "home",
        tags = "lighting,household,office,desk,home,furniture",
        contributors = "karsa-mistmere,jguddas,jamiemlaw"
    ))]
    LampDesk,
    #[cfg(feature = "lamp_floor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAdoMCIPg8C+A4FhSF4GgiCgxDcLg5DIOQgh8NoiEEMYOCAMIriqHggDkSA3GGKYpiyLAxC2IAyDgLQxC4Mw3DEbAyC0NYoiqN4qiMMhoDaNJJi2KY7DgLomDkeoThWGoZgSG4JE2TJNDaWoWl4PoBA",
        categories = "home",
        tags = "lighting,household,floor,home,furniture",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    LampFloor,
    #[cfg(feature = "lamp_wall_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg5DIOQgDEOAuDaEBBDGEggDCG4agwIAyDASA5GGGYZhyHAxC2DgyDgLQxC4Mw3DEbAyC0NYYhqKIeiYMxoDaJY6h2GYshSFg5HoIg+DwL4DgWS5OgaCIKDYIAzGEMogh2KZaDIdgyliWo7iqWQyEgNZBieQ4vi8Vg0mmW4ei+SZLk2BBolCd4HgkTQ4CANhom+ZZxhmZR2DWSpMlEPoBA",
        categories = "home",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    LampWallDown,
    #[cfg(feature = "lamp_wall_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg5DIOQgg0NoQEEMQghcMAghqF4MhgMRIDkYYdhuJQxC2DgyDgLQxC4Mw3DEbAyC0NYWhiJYch8IA0GgNojjeGo5ikOAuhQOR6CIPg8C+A4FkqTYGgiCg2hgNRhDIIJYkGN5YDIdgyleWY4hgLZdEiVokluJ5rHYLQ0j+GYmmQMZIkqTIEGiT54geCRNDiGA4Gib5dmOGozl6NJJkuUA+gEA",
        categories = "home",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    LampWallUp,
    #[cfg(feature = "lamp")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMHYNgiD4PAvgOBYThaBoIgoNAuDANw3g4MAuDYMQ1EEMYOCAMIri0NYODIaAxDQYYpimLI4CALg5DIMwtDELgzDgNRsC0M4eiALQ3kEOA0EGDYNjmLImCAMhIDkYZQi2OI/C6TQ2g4Lo9FqEoUhmGIEhqCRNDiVQwlmVZbioMgtjGNJajmKZQHYMY1iqeY/g6V5+jeLQxj+Px6mWFZpD6AQ",
        categories = "home",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Lamp,
    #[cfg(feature = "land_plot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIA4CANgtDMLYRDMdgxDAIg+DwL4DgWG4egaCIKg8MQxC4OQ5C0NQuDUIAzC4MQ0GEMQgjUMAgjiOo2C4Nw0GwOItCANAuDgNhhg2DY7jiSpAi0LZEkaNI2jmVY6C2J4+EwMQ2jYMoahyIYggSIoJG0NguDQOZekWLomC4MINmgM5gh2ZJjgWB4JE0MQ3i2NYMm2EJCDEOYxDWdZigE",
        categories = "design,tools,math,sports,gaming",
        tags = "area,surface,square metres,allotment,parcel,property,plane,acres,measure,distance,isometric,flag,golf course,hole",
        contributors = "danielbayley"
    ))]
    LandPlot,
    #[cfg(feature = "landmark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDgdgtDcIg+DwL4DgWFYYgaCIKDEMQuDEMggDKIQ5DgYYjiODYNDGDguDcNguDAMA2GwNwuDgNg2CAM45DQNxjC6QIyDIM4+DOIA5DQOQtC4MolksORIj6QA0GOTg1DMIAwk4Ng5DWTg3DENpOlCTpSHqFIWhuGoEhyCYLDSDoQhKa4Xm+boFgecYPnSEYThWeIZgKb58gqW5QGiD53m2hZ7h0TY8g+gKNnmAQ",
        categories = "finance,navigation,buildings",
        tags = "bank,building,capitol,finance,money,museum,art gallery,hall,institute,pediment,portico,columns,pillars,classical,architecture,government,institution",
        contributors = "connium,ericfennis,karsa-mistmere"
    ))]
    Landmark,
    #[cfg(feature = "languages")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAOAgDaDwiD4PAvgOBYThaBoIgoNAgDGHQ2C2EAyC0M4ShSGYYgSGoJE0MggDUaAxDKJ4ViuKoFgeLQ3CAMoyjWKYCiuOoKDKL5GC0NQtDEMJJh4MJAjeQo5hsTYfh4OBoDaUYXgEA",
        categories = "text",
        tags = "translate",
        contributors = "ericfennis,mittalyashu,johnletey"
    ))]
    Languages,
    #[cfg(feature = "laptop_minimal_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGiDgiD4PAvgOBYThaBoICIbQ5CAMQwg2Ig0C0NIShSGYTHIZRjHQIBoGUaRnGgdIJDEMgiCAch4gmOAgjwIgzjkdxpGSBY2DiOR5gmJoTC+K4tD6AQ",
        categories = "devices,notifications",
        tags = "computer,screen,remote,success,done,todo,tick,complete,task",
        contributors = "ericfennis,jguddas"
    ))]
    LaptopMinimalCheck,
    #[cfg(feature = "laptop_minimal")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMgiCAdxpGQdBog4MQ4hIeYODSEhyHiDoRCAcodCKJIiCIMwiD4PAvgOBYuGwaRuGUIB4DGI4SHgMojiQeY+igMIcjqQ4ti+NI2D6AQA",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis"
    ))]
    LaptopMinimal,
    #[cfg(feature = "laptop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA1GEMgghEMAghQMYShIdg4C4NQyDaEIYhSIggC4MgxDILg4DkNxsDELgwDaDYoicNxhheF4jDELQuDkIIuDQNRIDMLg2DKNo+hWSY6jwLY/DWLYvjELYzDINxBhGE5JhQNI+DWHIeFaNZYlqSAylMegiD4PAvgOBZqm2BoIgoMgwi8NZcDGXg5DgN5CjwNA2mma5wD6AQ",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis,csandman"
    ))]
    Laptop,
    #[cfg(feature = "lasso_select")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgyGENQghEMAghQMQtDILQ0CIPg8C+A4Fh2IIGgiCoMDENguDkMxjiqKQ0DMIAxi4Lg3DSDYqDmKgxhyHojiKBIkgkTQzC4M43DENBBikOAgkyFZQDGDYyDAYwwhoLg0DIIA0lkOAtk0MZXDgc5iCCRQ1mGFA4GENwuDEOQgm6cJQhYLZGjEMo9h+QZAgWB5DhEMZsluW4WnWGpToadZQDQep7j+ApBoCCpJniDQyGidwwDkYZGDWn6MhcLgyDSdwzDIdgtmKnpHkaN4UoeeKnDSLQwk2FJvqAMJFDKM63GybpHk6rZFDOMaxjKd6+C4NQ5pqXQ0DkbIYs2DJFDgNatqAM4TlGyw4m+jqQn2AQ",
        categories = "arrows,design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    LassoSelect,
    #[cfg(feature = "lasso")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NwwDQIAxDQLg0DYNxhDEMAgDgIIaDGEQggwMQxDUIAyC4Mw3DUIg+DwL4DgWLYwgaCIKDeJgyGGJYlhqHgtDILYMDkOQ0iyLozi0YxpHIYxsGUIBjHmCQxDYIggHKCQylYYx4gmK4tC+SpMk4PoBA",
        categories = "design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman,jguddas"
    ))]
    Lasso,
    #[cfg(feature = "laugh")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDgIAxDMYQ2CCMAwCCMwxC2MA1jGOozjWNwtDUaIUFoIobC+IokhsbBpG6CB4DGCw5g8eZPCKUQgHkMpQg8eJZlULgwDGRIckqTJJkuCJYlqV5UlaToTDWW5dDENZfmGRZkGUPoBA",
        categories = "emoji",
        tags = "emoji,face,happy,good,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Laugh,
    #[cfg(feature = "layers_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIIMC4Nw0GEMgghQMAghcMQthYTAyC4NQgDgLg4DcYQxg6GIpicMAtDGEA0EwMYnh4Mg2hOFYphmOAwGyIogDSI4lieK4qiqLx6CIPg8C+A4FkqTYGggIhtDKGZADIOIgi4NYjDQNYmiiF46hmL4xg0MouhGN4WiqG4YGwLY+C2QIkmCRJji2LxslsLZdDWSZLlAPoB",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy,duplicate,double,shortcuts",
        contributors = "danielbayley,jguddas"
    ))]
    Layers2,
    #[cfg(feature = "layers_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYOjeE4XhaPQgj8LQuhOMYzDgLY3DkYYpiuGoshyPwxHoIg+DwL4DgWWpdgaCIKDGIAxDcaA2lmW5gl+BJhgmCw5ioNB2mmWpcm6bYFgecIWgyUIqlOGpLjgMZKiCTo6haQ6Cg4Mg1kgNw4mqeJegKbp8gqfg3oCUotoSOaHjahY7oyLYdDYNYYGyEpkkcOQ2DKlJsgEA",
        categories = "design,layout",
        tags = "stack,layers,add,new,increase,create,positive,copy,upgrade",
        contributors = "juanisidoro,karsa-mistmere"
    ))]
    LayersPlus,
    #[cfg(feature = "layers")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYdh+GIxjMOAtjcORhimK4aiyHIODMegiD4PAvgOBZSlWBoIgqFoMkiKpLhqQ44DGQogkaOoWhaPYqh4NZBjKNJFjgQZJmCGQylwMpRlOWJXgSWYJE2XA3l6SotmKOZljaY47mqYI/m4MJCnGRp0l+a54ioN57lSfw+gE",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy",
        contributors = "colebemis,danielbayley,jguddas"
    ))]
    Layers,
    #[cfg(feature = "layout_dashboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQ5gwcoICIMYMHmCYLCAdxpGQdBogkNwiD4PAvgOBYoiuBogiKJAiiaB4JDENIbh2FoYhqDYPhGEwiDWJ4pi6LYEgaPI4j6EIShSOYZDKO42gyMIjiWRIqkiR4FCCHIZDaVYhleM4MhiHoOk2QZDCCF5UiiWosgE",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDcIggHIeIODGEYUCIM4RHmDoaCAdxpGQdBog6EA+DwL4DgWJ4qgaCIKgyJYRiCIokg+F4VDSG4dhGE4VCKJ4pgQdIskOH4hiOMoHgmC4NjcIIcCIMY6CCGJTj2VpAiiLZFgWVY8hKWZLjCToQlCOYzkiNomluQw+gE",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQ3gwcoICIMYMHmCYLCAdxpGQdBohQIg+DwL4DgWJ4qgaF4JhqB4dgyDoQhKJQghyGQ0gyIIiiQIoVieKYEHSJxwGGIwgGSCRNDENAgDQaJCiiSIjkeSRokuTZPCAOZTiaVZZliSpMCKTpQDENZgkOVhomSWpmmgIAyDCbJileAQ",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByHiFIMHcaRkHQaIJDeDIZgoIg+DwL4DgWJ4qgaJAxDSDIOhCEohheL4MgiJQghyHogCKIonimBB0iyRAghiGoNg+EYTkCOYUjEIIvlKPYfjaQotD6AQ",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gweIJgsIB3GkZB0GiCQxDiDByhcIgxCIPg8C+A4FimLIGgiJg0gyDoQhKFIWhiDIbh2HwihUIIkiCKIqi+LoEgaPIejiB4gjOQYlicIJRk+NYRhOP5EiuSA+gEA",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gwch4gkMYMHcaRkHQaIZDiDIYgoIg+DwL4DgWJ4qgaHIeiAIg5iOCYLCCF4ZgyDoQhKFIMggIgxDSJooi2LIEgaJAxDaFpKj+GZDg2D4RhMIoVCCL4fgkNZEimSA+gEA",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[cfg(feature = "leaf")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDAQQ3CCEgwCCFYNDkLg4CANguDEQwxDULg1CCJAxhINAuDSGwxDmDhjg0MoOCCKQxhuMobhWIg1C2KQ3iwMAtDGFwwFoIg+DwL4DgWSJLgaCIKjIMgxGOQQzCAMYajyIgzDaJQuDAOAtDYQ4ZiaKQ1jIMZqlcMZtDKR5Jk4PoBA",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,energy,plant,autumn",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Leaf,
    #[cfg(feature = "leafy_green")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyGMMQuDINQtC4OQ4DeDYSDcLYRDkNw1CAM4WC0MoSGENQuDUNggimKwgDCMAgDGIguDiM4qCANI6jKMYxDYLgxhiJQuDMNQzjWIYjiGPoyiMNg5C2KQxDENhBkuSY9jIMgwhaIY3liWIxjSY4siMMA1GGYY5k0MJSjYM40hEMw3kia5Mj2b5UmYLpQjyO5tkQMw0jeQAxDUNRDiMNA5DmM4pDSDA0C4NAwo6VQuDcMaShKMw4kGNQ5DKLAxDmmQ0nYLgwDGO5cpkM4MrEMgiD4PAvgOBa1riBoIgqsozhkN60rauw+gE",
        categories = "food-beverage,emoji,sustainability",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[cfg(feature = "lectern")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDIaAzGEMgghQMAgheFwxC4OQwDILYbDMOBshsMA1DYLQzC4M4sEEMYOhiMYXDKLw2EiEovi+GYYC2HA1DiDguDIOA3iQLomg2KosDQQYUhaMggDWDoQDMIg+DwL4DgWV5agaCIKDGQA2FaOIwjsMIgiAaIplaWJdlcchlGMdAgHmCQxDAIggGgZRpGcaB0ncMp6HiCQ4nodxpGSBaGnocqFCIMZtC+cZzD6AQ",
        categories = "communication,multimedia",
        tags = "pulpit,podium,stand",
        contributors = "gurtt,karsa-mistmere"
    ))]
    Lectern,
    #[cfg(feature = "lens_concave")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMhhDEIIQDAIITDALQuDiEQuDaEQ0h2FIghKEQ2hgQYQiKFQggwMgyGgMQwg+EYgimGAtDGG4fDGHophKNokDiJoyimEwxisegiD4PAvgOBQ+gE",
        categories = "science,tools,shapes",
        tags = "concave,lens,optics,light,magnification,curved,focus,refraction,science,physics,eyeglass,telescope,microscope",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    LensConcave,
    #[cfg(feature = "lens_convex")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg0DMMwgDIYQxCCFQwCCGIVC4OAyDSDg0DiFoiDGIoYhqGYWDkLgxDANIWjCJ4WC2HIeiAOBoC0MocDYNoUjGKQxjSHQ0jQNIhiOSYyhcLQxiuLQ0EGFYXkGFgwC4NQ2DeEh6CIPg8C+A4FD6AQA",
        categories = "science,tools,shapes",
        tags = "convex,lens,optics,magnification,focus,light,refraction,physics,eyeglass,telescope,microscope,curved,science",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    LensConvex,
    #[cfg(feature = "library_big")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIB3GkZB0GiCg4g0aBlGkZxoHSCgxhUIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKE0NwgDMdoeCKKAvi6Eoti8aIxjMMgwC4NAgh4Lg5GMLgyC4NQtC4MZIlKUQ2lQMxsC0MZKC4NxjlENZNluVZVlwM5WEwMZclOYgxmCY5PmaUguDaZJZlwOZRl+T5RDKVJslidRajqKY9GgPoBA",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    LibraryBig,
    #[cfg(feature = "library")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIINDQIAxDQIg+DwL4DgWFYYgaCAiE0MQyg4doShSFobhqBIcgkTQ4CAOIjDKJYXimKIFgeK4QDSIw2jKJ4BA",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "johnletey,csandman,ericfennis"
    ))]
    Library,
    #[cfg(feature = "life_buoy")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQ0C4OQzCCKosi4LgyDSMYzCKGwviKJIhiOJYnCIbQxioOItDkLgxDeNQ0C2Ko2jiOhojyJImiiQgukQIJWliTY0lyN4clCUo+iiRpIlmQwzkyMpdmuX45j2G4egaCIXhmFoLg2D4RgwNJunKIIBA",
        categories = "accessibility,medical",
        tags = "preserver,life belt,lifesaver,help,rescue,ship,ring,raft,inflatable,wheel,donut",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    LifeBuoy,
    #[cfg(feature = "ligature")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDIaAyHYOAiD4PAvgOBYWhmBoIgqDAgDIMBoDSFYXhyG4Eh2CRNDaDoQiWFoYiqKYFgeLIuiKJImjOGoCiqN4KDiIQwFYOBhg2DQwCCSwxCANwuDQNg0C0Mo8iiAQA",
        categories = "text",
        tags = "text,font,typography,alternates,alternatives",
        contributors = "danielbayley"
    ))]
    Ligature,
    #[cfg(feature = "lightbulb_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLg4CAMQxC4Mhjg4LQuDmEITC0MoaDILQzhMYQ2CCJAwCCJwwC0OQuDMLQ1CIPg8C+A4FjKNYGggIhth2PYnDIMIxjOOI3gSOYJE2DQziWLRhDQLg2DcIJPlGKJWieEodDWE4VDcLpShKS4SDWGpkDILowjKNJGkWBYHkiGQxDgaA2kKa42gKRpvgoMY/DIaA0naRIB",
        categories = "photography",
        tags = "lights",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,danielbayley"
    ))]
    LightbulbOff,
    #[cfg(feature = "lightbulb")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDQYwuDILQxCALg3hSF4OC4NQtDKHIOC0Lg5huHYfDKJQtDOHBBDYIIuDAIIxjGLg4GOMYVhIIIniUIIrDWF4aDELgzj2Q4Nh8NQiD4PAvgOBZMk+BoIgqJAxDgaA2kuTZSlGBJTgmC4xDIMhoDSW5Ol8PoB",
        categories = "photography",
        tags = "idea,bright,lights",
        contributors = "ericfennis,danielbayley"
    ))]
    Lightbulb,
    #[cfg(feature = "line_dot_right_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQgDMIAxDIIBMg4NYODIIg+DwL4DgWGBjGkchjGwZQgGMeYJg8IggHKCQzikYx4icOIXhmHogiIPoBA",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station,last,end",
        contributors = "colebemis,ericfennis,johnletey,nathan-de-pachtere"
    ))]
    LineDotRightHorizontal,
    #[cfg(feature = "line_squiggle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMwuDUYw1C0MgggwMoPg0IA0EMMYYDEMAghQMQ1CCJAxDaEYhCAOQth8IAxDQLQ3HOHoOhINIvDIYwthKFw1g+LQxCANoTCIPg8C+A4FD6AQA",
        categories = "shapes,math,design",
        tags = "line,snakes,annotate,curve,doodle,stroke,pen,tool,gesture,draw,wave,art,road",
        contributors = "chessurisme,jguddas"
    ))]
    LineSquiggle,
    #[cfg(feature = "line_style")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA1GgMgiD4PAvgOBYThaBoIgoMQ1CAMQyGgNoShSGYYgSGoJgsOYOhCJIViiJ4FgeKgzh+IYjhOMIXgKKI0gqNgxDkaAxDiL4mj2M4bE2NoPhGOomgEA",
        categories = "design,tools",
        tags = "line,stroke,style,dashed,border",
        contributors = "dg-ac"
    ))]
    LineStyle,
    #[cfg(feature = "link_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoIg+DwL4DgWHohgaCIKDGEw3GgMhhhOFYXg0IA0CAOIdh+JIeGwaRuGUIB4DKCQxDIIggHkMZBkOPpHCKNZFkAIpCjYL46jyOY7j0eJLkmP4JDKSZGl2RB5k+XpSlQZQ+gEA",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoaAyCIPg8C+A4FiCI4GgiCgxhMN4dGGE4Vg2MYYDAaAth6IIigQaIgGwaRuGUIB4DKCQxDYIpBDGCQ4kceZJCIMYeCAeZDk+N4hj2Pw+gE",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    Link2,
    #[cfg(feature = "link")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDMYQ1CCEoNhUIA3C4NQ0hkNBsDMLYQhKFAghULYYDAN4mC6KBsC0MQuDcMoOjAMQiD4PAvgOBY3jqBoIgoMQ0g4MYRhOJJHDCKoaC2HItDMIIhkaFoNicN4XisNxsi8Nwxi6NI2jiPQ+gE",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Link,
    #[cfg(feature = "list_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagmCwxh4OYhhOFYmiWBYHgkbQxDWHg4CAMo6CANAtDSIovheAQ",
        categories = "text",
        tags = "done,check,tick,complete,list,to-do,bom",
        contributors = "guanboo-yang,karsa-mistmere"
    ))]
    ListCheck,
    #[cfg(feature = "list_checks")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA1GgOAiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagmC4NDEOYhhOFYmiWBYHgkbYqDcIAyjgIA0C0NIii+F4CiaMwijUII3jmOY8j6LokgE",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListChecks,
    #[cfg(feature = "list_chevrons_down_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoDgIg+DwL4DgWEoVgaCIKgwMQyg+EYThiF4EhmCYLCAMQ5h+EoUiSI4FgeCRtDENYNCCDI4C0M4gi2FoCiSMQijONYpjeOo3jePIigE",
        categories = "text,arrows",
        tags = "options,items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold,vertical",
        contributors = "colebemis,ericfennis,ocavue,jguddas,PeterlitsZo,mittalyashu,juliankellydesign,karsa-mistmere"
    ))]
    ListChevronsDownUp,
    #[cfg(feature = "list_chevrons_up_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoDgIg+DwL4DgWEoVgaCIKgwMQyg+EYThiF4EhmCYLCAMQ5h+EoUiSI4FgeCRtDENQgDgIAzC2DI7iCLYWgKJIxCKM41DENo4kiOQzj2IoBA",
        categories = "text,arrows",
        tags = "options,items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold,vertical",
        contributors = "colebemis,ericfennis,ocavue,jguddas,PeterlitsZo,mittalyashu,juliankellydesign,karsa-mistmere"
    ))]
    ListChevronsUpDown,
    #[cfg(feature = "list_collapse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA1GgMQxCIPg8C+A4FhSF4GgiCoMCAMQyhCEoUhaBBohmJoHgmC4NDEOYihOFYaiiBYqCIbQzh+DQzC2PI+jGJYYgKKYcjgIAyjuPZKDOQIzgE",
        categories = "text",
        tags = "items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold",
        contributors = "ocavue,jguddas,karsa-mistmere"
    ))]
    ListCollapse,
    #[cfg(feature = "list_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagkTQ5h4OYhhOFYmiWBYHgkbYdgwLQzCCOY7iKL4XgKJozgoMgxg4dofGEMggkoMAgk0MQtkoMhoC0No9iSAQA",
        categories = "multimedia,text",
        tags = "queue,bottom,end,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListEnd,
    #[cfg(feature = "list_filter_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1EgMgiD4PAvgOBYThaBoIgoNgggwaIMhKFIZhiBIagkTQ5h4ORoDaIoViaJYFgeKAxh0NYti+JICiaNIKDGKg4FaEYTjCF4B",
        categories = "text,layout",
        tags = "filter,plus,options,add",
        contributors = "abdeniz,karsa-mistmere"
    ))]
    ListFilterPlus,
    #[cfg(feature = "list_filter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANRoDIMAiD4PAvgOBYThaBoIgoNggDEMhoh+EoUhmGIEhqCRNDmHg5GgNojhWJw+gE",
        categories = "text",
        tags = "options",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListFilter,
    #[cfg(feature = "list_indent_decrease")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMQxCIPg8C+A4FhSF4GgiCoMCAMQyhCEoUhaBBohmJoHgmC4NDEOYihOFYaiiBYqCIbQ3CAOAtDQII9j+MYlhiAQ",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis,karsa-mistmere"
    ))]
    ListIndentDecrease,
    #[cfg(feature = "list_indent_increase")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMQxCIPg8C+A4FhSF4GgiCoMCAMQyhCEoUhaBBohmJoHgmC4NDEOYihOFYaiiBYqCIbQzCAOAgDSPAtj0NIxiWGIBA",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis,karsa-mistmere"
    ))]
    ListIndentIncrease,
    #[cfg(feature = "list_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgoMQxCAMQyhCEoUhmGIEhqCYLg0MQ5iKE4VieJoFgeKQyh6IBoC0NojjCF4BA",
        categories = "multimedia,text",
        tags = "playlist,remove,song,subtract,delete,unqueue",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListMinus,
    #[cfg(feature = "list_music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgoMQxCAMQyhCEoUhmGIEhqCYLh4MQ5iKE4VieJoFgeKQyisNhWDWI4whcPBjGkchjGwZQgGMeIJDEOAikUeZIDaSxygmEYvj+QZDD6AQA",
        categories = "multimedia",
        tags = "playlist,queue,music,audio,playback",
        contributors = "karsa-mistmere"
    ))]
    ListMusic,
    #[cfg(feature = "list_ordered")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA1GgMQwCIPg8C+A4FhSF4GgiCoMCAMQyhCEoUhaBBohmJoHgmC4NDEOYihOFYaiiBYqgoNAgDSEB2DWMYlhiAophwTY4i8Mo+jOQY1kMNguDUIAyDASAzC4NBjDALYNDILg2lkLg5DKT5bl2VA1GEMZOh+aQwCCbJYmOXgwkeJIzgEA",
        categories = "text",
        tags = "number,order,queue",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ListOrdered,
    #[cfg(feature = "list_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgoMQxCAMQyhCEoUhmGIEhqCYLg0MQ5iKE4VieJoFgeKQxDgIA5HYNojjCF4CieNIKDKHogGgLY7i+JYBA",
        categories = "multimedia,text",
        tags = "playlist,add,song,track,new",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListPlus,
    #[cfg(feature = "list_restart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgoNwgDEMoQhKFIZhiBIagkTYdDEOYhhOFYmiWBYHiiH4eDgYQ1g4IAwjuPQ5C0MwgDQLo5kOOY8kgLZGkqRBjC0MQuDOQQwC0MpEkYNJAC4NINlCXBMDGXQ0iKL4XgKJozgqYYeDAdg0GiY4uiSAQ",
        categories = "multimedia,text",
        tags = "reset,refresh,reload,playlist,replay",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListRestart,
    #[cfg(feature = "list_start")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoDYIg+DwL4DgWEoVgaCIKgwMQyGgMQzhGE4YheBIZgmCwgDEOYfiGEoUiaJYFgeCRtDENggDgLQzjsII8i6I4xgKJo0goMgxioORWDcYQyCCTgwCCUQwC0MpVGgLYQi+JIBA",
        categories = "multimedia,text",
        tags = "queue,top,start,next,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListStart,
    #[cfg(feature = "list_todo")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA1GgOAiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagmC4NDEOYhhOFYmiWBYHgkbYqDcIAyjgIA0C0NIii+Fw8HIZRjHQIB5gmPggGgZRpGcaB0gkNgiCAch4gkMZUlcIgzlQdxpGSBZSj+Q5FD6AQ",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ListTodo,
    #[cfg(feature = "list_tree")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANRoDEMwiD4PAvgOBYThaBoIgqEAgDEMhoDiEoUhmGIEhqCRNh0MQ5iGI4VieJoFgeKQzh4MBhDIII6DAII9j2OoghGE4wheAonjSCo2DUdofjmO4+lGQI7GiQ4kjGAQA",
        categories = "files,text,layout",
        tags = "tree,browser",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListTree,
    #[cfg(feature = "list_video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgoMQwCAMQyhCEoUhmGIEhqCRNh2Hw5iKE4VieJoFgeKQxDWHwyC4MAwDMYYNg2Hoej8Lg1DENwtC4OA1DkbA0C4OQ5DcII5lAN4+h8IJBliQguDcMQ4GwLZOlWU5PlGV5AlmHwtDGRJGkgOA2HqI4wheAQ",
        categories = "multimedia",
        tags = "playlist,video,playback",
        contributors = "karsa-mistmere"
    ))]
    ListVideo,
    #[cfg(feature = "list_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgoMQxCAMQyhCEoUhmGIEhqCYLg0MQ5iKE4VieJoFgeCRtDENQuDUIA5jmDoOiOMIXgKJ40CIbQyDCPY8DULY6DWQIlgEA",
        categories = "multimedia,text",
        tags = "playlist,subtract,remove,delete,unqueue",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListX,
    #[cfg(feature = "list")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoC4MAxCIPg8C+A4FhSF4GgiCoMDEMoPhGE4VhqGYEhuCYLCAMQ5iGEoUhaJ4mgWB4pDiDRoDEM4jjGGICieNYKjeH45juMIlj+NIcE2Q4tjqPIlgEA",
        categories = "text",
        tags = "options",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    List,
    #[cfg(feature = "loader_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYQ5CCEgwg6DgtDYLoMDkLQ4C4NQ2CIPg8C+A4FD6AQA",
        categories = "cursors,multimedia,layout",
        tags = "loading,wait,busy,progress,spinner,spinning,throbber,circle",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,ericfennis"
    ))]
    LoaderCircle,
    #[cfg(feature = "loader_pinwheel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIYQxg4IAwhSDgtDGFYVhKEoahSGIaCIPg8C+A4FiOJoGgiCg3CAMgwC4N4RhOG4TDULQ4jGE4djQII3jkNoiiSKYogSKoJE2LQzC4M4zjyHI+CCQI7haPA1lILpBiOJZGiMYxpHIYxsGUIBjHmCYPCKZR4mgMpqHKaAwkIL5fmGYw+gEA",
        categories = "cursors,design",
        tags = "loading,wait,busy,progress,throbber,spinner,spinning,beach ball,frozen,freeze",
        contributors = "danielbayley,jguddas"
    ))]
    LoaderPinwheel,
    #[cfg(feature = "loader")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYNAiD4PAvgOBYThaBoICIbQxDYLoNDcLg4g4Lg5C0MomhKFIZhiBIagmC4kgwaIRhOFYvi6BYHgmHYfg2HogiUOZDiuOIXgKL48gqDAgDEOIQkaLZJjuGxtDSJpODkLgxkOKIqjeU4ZksTZADKNZSjmVIwhyWJEm6Q5FmGOYBA",
        categories = "cursors,multimedia,layout,design",
        tags = "loading,wait,busy,progress,spinner,spinning,throbber",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyCIIB5DKCgxgyDoJCKEoNHiEAiDUIg+DwL4DgWHohgaD4RhOGYKDKEx5hWF4Hi4OYdh+JIjgSBophaLIViiLoshqHIeiCN42gWB4ai+CIng2LYRjKDoaiuM5DiIPBjGkchjGyBhygoN4NGMeZLCAYx4kuQpXlmW4emmWpcgoM5gmaOpgmKdJolibg+gE",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDkdgzCIPg8C+A4FhSF4GgiCoMCAMoRhOFYahmBIbgmCw4C4OA5g4MwuDINBhDcII0DAII3DALYqDEM47C6PYihaJolgWB4og+DgyGiEoUkOGICiaR4Kg2DJMkKJJRkaHBtg2Xo3DIMJYkSWongoNwuDANY1mkNYzjWOJxjcOQui2dA5mOGIBA",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "fdev,jamiemlaw"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyCIIB5gkIgxgyBwygoNYNHmFYRgwPg8C+A4Fh2IIGg+CoSg0eIaDKE4ZiaE4IiYOQih2H4EGWIo2g6EIvhqJ4OhqF4HhCJ40iOOIFhSLoYkOMo/gqK4okOHIekYPBjGkchjGyBhygoN4NGMeJKCAYx5kqNJXlmWw+gEA",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[cfg(feature = "lock_keyhole_open")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYSDYIg+DwL4DgWB4bHIZRjHQIB3GkZB0GiEg4g4aBlGkZxoHSEoUCCEQiDODhyjiNoXgwMIahyIokhscBhioIBkgsTQ3CAMQwFYNxhDUIJVDAIJYDEIA5C4MwzC0MguDWQgvkeKg+gEA",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis,cd16b,danielbayley,karsa-mistmere"
    ))]
    LockKeyholeOpen,
    #[cfg(feature = "lock_keyhole")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ2hAcoTCIPg8C+A4FgeGhyGUYx0CAcoLCKDwgHcaRkHQaITDiEBoGUaRnGgdITimJwzhCEoNDCGYbiGI4aHAYYuCAZIME0NwgDEMBWDcYQ1CCVAwCCVwxk6VwwHaPIaC+RouD6AQA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis,cd16b,danielbayley,karsa-mistmere"
    ))]
    LockKeyhole,
    #[cfg(feature = "lock_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQxDGERyHiDQyhGGQiDOF4TCKGwgGgZRpGcaB0hSFg+DwL4DgWLRwGGCwgGSDRNDcIIVFYNxhDUIJADAIJDDEIA5C4OQtiyLozgsPoBA",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LockOpen,
    #[cfg(feature = "lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCKBx4gqDAgg8Igzg0aBlGkZxoHSCgxDGDYJCKHoNHcaRkHQaIdDgIg+DwL4DgWLRwGGKAgGSChNDcIIeFYNxhDUIJADAIJDDGO5DDAdg0iyLozigPoBA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[cfg(feature = "log_in")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAxDcIA1C2EoUCIPg8C+A4FheGoGggIhNDENYODISAzhaGIdhyBIegmIYjDMaA0GEMggjWDYNDGNo2HYMYzjWNwgjkLZAGgLQ0iiGYsD6AQA",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[cfg(feature = "log_out")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAxDcIA1C2EoUCIPg8C+A4FheGoGggIhNDIMYODISA5haGIdhyBIegkTQ5CCIhIDUYQyjEIAwjiDgtDKPBWjSNo2jmOYjj0MhoDSKIZiwPoBA",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[cfg(feature = "logs")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoDEIg+DwL4DgWEoVgaCIKgwMQyg+EYThiF4EhmCYLCAMQ5h+EoUiSI4FgeJg4g2K4hi6AokjGCozh2NYthaOIwhoTY8iqEIsiKQYlgoMYMg4OIgj8aIvksTZNiiHpQkiN4YjqVociqWo2haAQA",
        categories = "text",
        tags = "options,list,menu,order,queue,tasks,logs",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    Logs,
    #[cfg(feature = "lollipop")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQxCIIBygwOIQGMeIMg4Ig+DwL4DgWB4bHAYR0GgIBkgwbQyDEIIqC0NAuDOLowhqHIiiSIYjiWJwiE2Dggg4YQyiwIAwkSRg0kcIJIkWTAtDiRg2CCUZMkYMZCDCNAvjYaA+gEA",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = "danielbayley"
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgwGEMoNCAMITCAMQtDKGBWDiEIShSFAxg2GBoDEModhGH4WhIMh2DGD4RiiFYXjAIg+DwL4DgWNo5gaCIKDiFg4FYNInhWIIiDIaJEjCRoqjCLQ0jWN48juBI9gkTYug0MJKlKOJWjYYxpHIYxsGUIBjHmCYOCKaB4gkMQ2m0cprl6YpkmaYZjmWZ5pmsMJtGObwiDic51jYL53nwPoBA",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "magnet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDUIA0hEIg+DwL4DgWFYYgaCAiE0MguDMNYNDEMAuDYNA4GEMQuDIMIQiyLoQDAII0jaDguDcMA0GyIAyDkNggj6QIri2L44jKNZKjSLI6hIMBsDYLgwj8LZSlQOYrg6SgxlsMwgDMbJWlOPwgleP5FkmMZHjeN5NjuPYtkCQpyDaaZHmuM5LjiTo1lGIQ2DWYwzDYNxBl2XZtCAOI5DGQQ0i0OAyHqFIWhuGoEhyCRthAOIRhOFYXpkPoBA",
        categories = "design",
        tags = "horseshoe,lock,science,snap",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Magnet,
    #[cfg(feature = "mail_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDMVg2GGDYNDAIIWDALQyhoSA0hMIIVheF4aiAdgxDIY4WDELorDmIIvDIaA4CIPg8C+A4FjWOIGggIhtgwIA3C0OAuDkNwgDULg3GGLQ0g6RZOhiDoaC4MA2hcTINDeNI2juOoEjyCRtDGVwxi6FAgDQLQ0lyN5gD6AQA",
        categories = "mail",
        tags = "email,message,letter,subscribe,delivered,success,read,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailCheck,
    #[cfg(feature = "mail_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDUVg2GGDYNDAIIWDALQyhoSA0hMIIVheF4aiAdgxDIY4WDELorDmIIvDIaA4CIPg8C+A4FjWOIGggIhtgwIA3C0OAuDkNwgDULg3GGLQ0g6RZOhiDoaC4MA2hcTINDeNI2juOoEjyCRNDGVwxDkaA2lyN5gD6AQ",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailMinus,
    #[cfg(feature = "mail_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELgyCAOAuDQYwuDULgzhGEQ5DcLg4CCDQ2HYMQwGGD4PDAIIoDELYmEgNIlCCJ4ph+LIsFaI4wjKKggh0LYgGwOAtDaOYzjsMoSimQAgDYWgiD4PAvgOBZPlKBoICIbQyg+IwthoNwghYNxhg0OQ0h8LplkWNJHDANopEyWwwk6UJVD6AQ",
        categories = "mail",
        tags = "email,message,letter,read",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailOpen,
    #[cfg(feature = "mail_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDMVg2GGDYNDAIIWDALQyhoSA0hMIIVheF4aiAdgxDIY4WDELorDmIIvDIaA4CIPg8C+A4FjWOIGggIhtgwIA3C0OAuDkNwgDULg3GGLQ0g6RZOhiDoaC4MA2hcTINDeNI2juOoEjyCRNDGLgxDYdg2lyN5gl+BYHmKZoODkaJpjWa45gEA",
        categories = "mail",
        tags = "email,message,letter,add,create,new,compose",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailPlus,
    #[cfg(feature = "mail_question_mark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDALg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHYMQyGOGAxC6LQ5iKMQyGiKYRCIPg8C+A4FjiO4GggIhtgwIA3C0OAuDkNwgDULg3GGLw0g6SJRhqDocC4MA2hkTINDeN45j6PYEj+CRNDEOIOkwMg4GMLodC4NIRC0LpHDkLQxhWLoinqVQxnsNpwm0M5whGdIRlIM4ki0M4mh2FpfjqY5igWB5lDKGIMHaWAxpCYYBA",
        categories = "mail",
        tags = "email,message,letter,delivery,undelivered",
        contributors = "karsa-mistmere"
    ))]
    MailQuestionMark,
    #[cfg(feature = "mail_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDILg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHaDxjhgMQuiwOYijAMhoDeEQiD4PAvgOBY3jqBoICIbYMCANwtDgLg5DcIA1C4Nxhi4NIOkeUIag6HAuDANoZEyDQ3jaOI9jyBI+gkTQxDiIgxGEMwgmuK4khuWZrm2b4kDYWpejmYo3GMaRyGMbBlCAYx5gmZgiCAcoJDOhxjHihQ4nifJ+oCYYFgeCZBhaHYsDULacniYIBA",
        categories = "mail",
        tags = "email,message,letter,search,lens",
        contributors = "karsa-mistmere"
    ))]
    MailSearch,
    #[cfg(feature = "mail_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDALg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHYMQyGOGAxC6LQ5iKMQyGiKYRCIPg8C+A4FjiO4GggIhtgwIA3C0OAuDkNwgDULg3GGLw0g6SJRhqDocC4MA2hkTINDeN45j6PYEj+CYLiwNB2DSX46mOYoFgeZQyhiDB2lgMZrmGAQ",
        categories = "mail",
        tags = "email,message,letter,delivery error,exclamation mark",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailWarning,
    #[cfg(feature = "mail_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDMVg2GGDYNDAIIWDALQyhoSA0hMIIVheF4aiAdgxDIY4WDELorDmIIvDIaA5CIPg8C+A4FjWOIGggIhtgwIA3C0OAuDkNwgDULg3GGLQ0g6RZOhiDoaC4MA2hcTINDeNI2juOoEjyCRtDGR5kCCTg0lyN5gl+BYHmIMgxg6QpomqXoB",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailX,
    #[cfg(feature = "mail")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIA3C0OAuDkOQxCANQuDcMg3GGDYNDAIIfDELQyC4MAwDmIBMg0NwiD4PAvgOBYuHIZRjHQIB4gkMgiCAaBlGkZxoHSCQxDaPB3GkZIFjoMI8HKOQijsIB5gkNIti+NI2D6AQ",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA0hEIIThWDoYhgVg5C4NRDg0NwgDQIA1CANoliwSAxDgYwyC6HorDELg4iqKh2DgWgiD4PAvgOBZDHAbxsHkbBpG4ZQgkmTh0HOCQxDULA5g4OJZlsLAxDGQpEkmS5Nk+SIEgaCIKi+LYmlqLZgCCKZyiQNRWg+HIehaGIdmKRZpkOZpQHkMpWDAIggHgMYJDaiR4oYIg3okeaMCIMaIkML6DD6AQ",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDkYQyCCEgwCCFQxC2EgyEgNIRhOFoghgMoZHYLQ4h6FIhg4LQxC4NwzDIIg+DwL4DgWM42gaCAiG0MoSDULg1C0NguDSDwgDQLoMhCGogheGZKDaFBMg2QA1jKNI5jMchlGMdAgHIeIJjEIB5gkMwiCCYgiDeaR3GkZIFgkMZXCAaBlGkZxoHSc4xjML5cl4PoB",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Mails,
    #[cfg(feature = "map_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDkLQxC4MQwDYLQuDUNQyGEMggh0MAgiAMIRC4Nw4DiIRsC0MwuDYNQ5g4Lg4DMQYNg2IoOCAM4ODkLgzDgMRWDaLQxDgYY2iGSYNhgNQzhcOA5DQbA0kyTgyC4Mg3DeHIekmII2iWJ4plQMgxh2V4UDaXIfl6MYmigMBsiyLoQhKM41jmOINmUIJUDaDxWDENAiD4PAvgOBaFoiBoICITQxDUIA1iUNg0oGg6FoeBBooqm4HgkTZ8kUaAtDahKGounYFp+jowiwMgzDYdqQqemqJgE",
        categories = "navigation,travel",
        tags = "location,navigation,travel,drop,delete,remove,erase",
        contributors = "colebemis,karsa-mistmere,ericfennis,MarianoFranzese,jguddas"
    ))]
    MapMinus,
    #[cfg(feature = "map_pin_check_inside")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDQLg5DkMwtDULg1DMOYODALgxhYLQ3C6HIdDEMQuDeFRhDGDggg2DQxC2KYMDKMBDDmGocCCDIghYIA0g6E4VDOQYeGEOAgkqMYvDENowCIPg8C+A4FlOVoGggIhtieDY3jcNAtDSUpUlkPoBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,done,tick,complete,task,added",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinCheckInside,
    #[cfg(feature = "map_pin_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg0DMIAxDILg5DMNRjC6Fg3C2FA2DcLg1hsMYUDUNYghuE4VDUYQ4CCLQwCCMAwC0MQ2jEY4wDSFIVCCJg1DMOYRDALoMhCH5AkEMYjDcOQ5GEMYRjGUowiMMgwDKUgzhODA3CCWpEDmXYylILg4DEM4cDcMg4CIPg8C+A4Fm4YxpHIYxsGUIBygkMwiCAYx5gkMQwn4Yx4oIMptm+dJ2nibpxgaCAiG2NYRi2WJYDQLQ0oqcIEGgPoBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,done,tick,complete,task,added",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinCheck,
    #[cfg(feature = "map_pin_house")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyDIYQxCCEgwCCFQxC2GAxHYLQ0hGE4WiGEguDQNA1C0Lg4DMMhsDMLYQhKFIihMLgxhSLYOEGMYhheDgyhMNx2h6O4VheGYTHoIg+DwL4DgWS5OgaCIKDEOITDAYZWlaRYWhkNoWGOFQ0C4OQ5DMIA1C4NQzDmV41mYIA3C6bJtjYLg3mWH4ylwLg2DAMQuDKSpMlGUIElKCYLlaD4cDOg5NoeSxjGkchjGwZQgHKCaOCAYx4gkMQwCKnR5qCopLC+k6VpcPoBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,home,living,building,residence,architecture,address,poi,real estate,property,navigation,destination,geolocation,place,landmark",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinHouse,
    #[cfg(feature = "map_pin_minus_inside")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDQLg5DkMwtDULg1DMOYODALgxhYLQ3C6HIdDEMQuDeFRhDGDggg2DQxC2KYMDKMBDDmGocCCDIghYIA0g6E4VDOQYeGEOAgkqMYvDENowCIPg8C+A4FlOVoGgiConDAaA2lKVJZD6AQA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinMinusInside,
    #[cfg(feature = "map_pin_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg5DcNwgDENBDDEOQuDaEgyC4NwwDEIAyDCEgxC4Mw0DOIIiDEMBhDgIIuiKMQtDGGQwGOIg0g4OYoDULg1DMOYSDALoWigN4lDmQQxiQN5JGGH4fjEIIqC6IQylMIAzleWpYlILg4DINAthwMw4CIPg8C+A4FmgYxpHIYxsGUIBjHiCQxDIIp0HmdwwnocoJDOZ5pm6cJymia4GgiCo0hIOBoDag5qgQaA+gE",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinMinus,
    #[cfg(feature = "map_pin_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg3DUIA3C4MA5GEMwghcMAghoMQgg0MQ2h4LogCIPg8C+A4FiaKYGgiCgxhIMA3DIIIwhOMxjC0MQuDYMw0iKMAtDMLg1DIN4YC4OYMC0NAuDQN4dk2Mw3GGHYdhqHI6C4MgwjQMBDDmRAzDmHgwiMOYXj8MZNDmaAgmoMBhDgIJzliNY1k4MwykyRA2DiJYniyK4Ei2CRtjSiIalygIooSg4FgeCRNDiToPiIOAxDgQZznWG53lyNQwGOHIjDgMwtC4M4dg0Mw3DeqKYkgNQzDOjKCgKhKRgqYQxhevAzhaGKenaQw3j+xQ0rajoB",
        categories = "navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[cfg(feature = "map_pin_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg5DcIA5C4MwwDQQQ4CCGAwCCG4bDIIAxDAY4bDQLg2DkIIlDgOIQhINQ2h+DQwDKHwxDELg0DYOAiD4PAvgOBY9kCBoIgoMo3DMN4YDENomDINhhDGIIclQMAtDMLgwhWV5ZhUbAtiUMJSmEMQyGGH4fh2HAtC4NQwk0OA1DSXwuDiSQgDKdQ3GGbZtlWVJOk4bJ5iybJ2nuaJ/hudZymybg2HqPI+kOPRjGkchjGwZQgGMeIJiEIqcHmnwwqEcoJDOkgvpamKaD6AQ",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,edit",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,sachinkr7368"
    ))]
    MapPinPen,
    #[cfg(feature = "map_pin_plus_inside")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDQLg5DkMwtDULg1DMOYODALgxhYLQ3C6HIdDEMQuDeFRhDGDggg2DQxC2KYMDKMBDDmGocCCDIghYIA0g6E4VDOQYeGEOAgkqMYvDENowCIPg8C+A4FlOVoGgiCgxjcNx2DaUpUlmWIElqCRNicMBomGU5VmYPoBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,add,create,new",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinPlusInside,
    #[cfg(feature = "map_pin_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg5DENAgDEMQuDEMA1EENwuDIOQ4CCGoch4MAgiOIwyiOFhhh6IokiQLQxDaJBjiMNIODkMwgDULg1DMOYSDCFY3h8Lo9j6EwuDcOQ5GEMYSi2JYShsMAyi0M5UlaT4tC4OAyDQLZIDMOAiD4PAvgOBZkGMaRyGMbBlCAcoJDMIggGMeIJDEMp0GMeZ4DCY5lmqbJumSZ4GgiCowhIOBoDagJmgQaKFpGB4JguRg1HajpkpCaIBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,add,create,new",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinPlus,
    #[cfg(feature = "map_pin_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQgDEMguDINA4CAMgxC4OQ2DkIBhgyHAwCCHgxCALQwC4OA0hmIwuDENwgEMIA5C4NQzhkMokDEOQzCANIMDSFo4jqDIehuEoSh6IIMDaH4thOIIkDQMoMjCF4hDGJImlSMA4lqDIVDINgyCIPg8C+A4FmKZYGggIhtDKUJtC2FZanCJQ4mGY5omIYxpHIYxsGUIBjHmCZVCKgB4oOYAgHKCQznYL56nyfp5nufZ/GOhwiDGdaAoKmabosIqNmKj6UpKAQ",
        categories = "text,navigation,travel,account",
        tags = "location,navigation,travel,waypoint,marker,drop",
        contributors = "colebemis,karsa-mistmere,ericfennis,csandman,TonySullivan"
    ))]
    MapPinSearch,
    #[cfg(feature = "map_pin_x_inside")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDQLg5DkMwtDULg1DMOYODALgxhYLQ3C6HIdDEMQuDeFRhDGDggg2DQxC2KYMDKMBDDmGocCCDIghYIA0g6E4VDOQYeGEOAgkqMYvDENowCIPg8C+A4FlOVoGggIhtDGEw1CCJA1hgIA1lKVJZliBJagkbY6mCYplmWZ5VmsPoB",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinXInside,
    #[cfg(feature = "map_pin_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg3DUMggDEMQuDkMAxEEN4ODgIIaDeHAwCCIYhDKIQxDAYYciCIoiC0MQ2iIY4hDSFQ5DMIA1C4NQzDmEgwC6DI3hqPI9hODg5DkYQxhKLIjhILolhGJpFj2TohC4MA5C2WA3DcIg+DwL4DgWYBjGkchjGwZQgHKCQzCIIBjHmCYnnAYx4nQMpfmGZpomqYJjgaCAiG0MoUDWEo5DULaIDWe5igQaKApGB4JoWhwgiWOqMoyj6BD6AQA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinX,
    #[cfg(feature = "map_pin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDQLg5DkMwtDULg1DMOYODALgxhYLQ3C6HIdDEMQuDeFRhDGDggg2DQxC2KYMDKMBDDmGocCCDIghYIA0g6E4VDOQYeGEOAgkqMYvDENowCIPg8C+A4FlMYxpHIYxsGUIBjHiCQxDIIggHKCQzmUYx5mIMJSlSWZbl0PoBA",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[cfg(feature = "map_pinned")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4GMMAgDMLg2DEMwthMOA2DkIA3C4NAyDkLQ1C4Mw5DODguDcOQ1GEMQgi+EYRDELQxC4MgxDQIAwEMOQuDgN4wiSIIcDaMI2hWKJGDgYZGkaMowjAMo7CIPg8C+A4FlYYxpHIYxsGUIBjHmCQ4CIIBygkMpnGMeIJDGa5WC+XJemCVpZgaCIKDiKo5jANBohifYulGUAwC0Lg5DSfA2DgMxsC0MguDAMI6DYQYvjGO6bigMgyGiDKEpqUKJouNYlDENqQDILQ2qKm4yoiioihQOKAoIN5wlWV54D6AQ",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MapPinned,
    #[cfg(feature = "map_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDkLQxC4MQwDYLQuDUNQyGEMggh0MAgiAMIRC4Nw4DiIRsC0MwuDYNQ5g4Lg4DMQYNg2IoOCAM4ODkLgzDgMRWDaLQxDgYY2iGSYNhgNQzhcOA5DQbA0kyTgyC4Mg3DeHIekmII2iWJ4plQMgxh2V4UDaXIfl6MYmigMBsiyLoQhKM41jmOINmUIJUDaDxWmYIg+DwL4DgWhKHgaCAiE0MQ1CANYlDYNKBDKg6FoqiYEouCaOiijx2DamKGpym4Fgenp8kUaAtqOhKloiAqcqmjYwiwMgzDYdqPqSmoB",
        categories = "navigation",
        tags = "location,navigation,travel,new,add,create",
        contributors = "colebemis,karsa-mistmere,ericfennis,Seanw265"
    ))]
    MapPlus,
    #[cfg(feature = "map")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLgxDANggDULg1DUMxhDIIIZDAIIchwMQuDcOA4h0bAzC4Ng1DkLYgDgMxBDEIIxh6MoajGDQ2DEOR2DEMohDYNBhjGM4djILYUhYLg4DkNBsC2DYVDOGguDINw3hiGpFh+LIhiOJZPlSPQtj6Dw2liG5alyIokDCTonikOYykqL5DlqNZSjoLgzDgMRWDaKAxDiQo1jSMZIDOR5Lk2UIWmOVJWmedpDl2bB6CIPg8C+A4Fpim4GgiCgxDWEo/DSPA1pemaep2BKfgkTZxicMgzDap6ppqrQ+gE",
        categories = "text,navigation",
        tags = "location,navigation,travel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Map,
    #[cfg(feature = "mars_stroke")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIA2CCDQ0CIPg8C+A4FhSF4GggIhNDENwgDMaA0HaEoUhaBBohmKYHgkbQyDGIQtDcLg3DUII0jaE4VhqFBjGkchjGwZQgGMeYJDENQiCAcoJDaSxjHiCQ5jsL4/kGQw+gE",
        categories = "medical",
        tags = "gender,androgyne,transgender",
        contributors = "jamiemlaw"
    ))]
    MarsStroke,
    #[cfg(feature = "mars")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgNR2DUIg+DwL4DgWFYYgaCAiG0Mgxg4LQ2C4Nw1CCJImhSFobhUYxpHIYxsGUIBjHmCQxDQIggHKCQ2jsYx4jgMIrC+L4xjMPoBA",
        categories = "medical",
        tags = "gender,sex,male,masculine,man,boy",
        contributors = "jguddas,jamiemlaw"
    ))]
    Mars,
    #[cfg(feature = "martini")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgyGgOAiD4PAvgOBYThaBoIgoMQyCAMQxHaIIShSGYYgSGoJG0MQ5CAMwtDcIA4jALQ4FqJIVigPoB",
        categories = "food-beverage",
        tags = "cocktail,alcohol,beverage,bar,drink,glass",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    Martini,
    #[cfg(feature = "maximize_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNh2DYIg+DwL4DgWFYYgaCAiG0Mgxg4LQ3CAN4UhaG4agSHIJG0MwgiCJYjieF4riqBYHgkTQ5jAMRIDMdgthOFY1hmAQ",
        categories = "arrows,layout,design",
        tags = "fullscreen,arrows,expand",
        contributors = "colebemis,ericfennis"
    ))]
    Maximize2,
    #[cfg(feature = "maximize")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMxIDUYQyCCEgwCCFQwC2EgyHYMwiD4PAvgOBYfiKBoIgoMgxCAOBWhCGoWjCGAyhkaAth2H4hgQaIkjqB4JE0MwgDENochGE4xjCGhojeIIljyBY+gqQ4TDGS5GhSSIzhuNoek2Og+gEA",
        categories = "layout,design",
        tags = "fullscreen,expand,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Maximize,
    #[cfg(feature = "medal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4MgxCAMQ1CAMguDYNgggwMQ0GEMoTCAMIfhAIAuDEMwthQMhMDQLg0hMLg4EGHYdiCIIPhcMhoDEMoch6NIiDGFYvGyQI3iSG4yiGNYjhqLopDENguDcOYQDUIg+DwL4DgWV5agaCIKDGD46CANYkjKDZWliXZcgSXoJG2JYQh2ZQ4DgLQ5i+aZZm2bIFgeCRNDiGBoDieprDwYxpHIYxsGUIBygmVQgGMeIJjoIqTHmlg3nqiaLo2fZumCHQxDgdonGgLQulWV57luAQ",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[cfg(feature = "megaphone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg2DMNggDYQQxDMIIVCAMIZhsMQ5C4NAgDMLgyheJYahoMQgDKKQ0HaDAuDMNA0CIPg8C+A4FjWOIGgiCgxDSMA3DiF5ADMNQ3hSFoYieHIpj8SA1GGJIkkwMQtDKVxWDiUoqhuKIqlcaAxjSNo7jqBI8gkbZTiqGgyDCZI3miZ4FgeCRNhGPxhDGJJ8l6Gwyh8IA3iOXZUn+IpYoENBBkOQ5VheKIzjWco5gKaJ2gqjh2DacZmgE",
        categories = "multimedia,notifications",
        tags = "advertisement,announcement,attention,alert,loudspeaker,megaphone,notification,disable,silent",
        contributors = "jamiemlaw"
    ))]
    MegaphoneOff,
    #[cfg(feature = "megaphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA2GEMQzCCEQgDCFYXDgLg0C0MguDgQYNg2FoWg0MoNDQdgxDKEITheJAtDELg2h6IIShSI4XgyEw0EgNRhDIIJAjgMYchwVg4j+QYui0MocHoIg+DwL4DgWUZUgaCIKDaO4QkCKpLhaHQ0CANwukCZ5gCAM5mhyGhBDgIJwkOE4kDSUJSleVoEliCRNnANh2Did5TnsPoBA",
        categories = "multimedia,notifications",
        tags = "advertisement,announcement,attention,alert,loudspeaker,megaphone,notification",
        contributors = "jamiemlaw"
    ))]
    Megaphone,
    #[cfg(feature = "meh")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bGwaRuggeQxhMNYQHiJwiDiEB5DKKIqjGDQ2CKGwviKJIhiOJYsDmKo/jODA5C4MAxi+NJAjiOhljyJAgiaRJBjKUZKkODQ1kaSJMj0PoB",
        categories = "emoji",
        tags = "emoji,face,neutral,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Meh,
    #[cfg(feature = "memory_stick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMHYLQyCIPg8C+A4FhSF4GgiCoMg4OIQhKFIWgQaIZiWB4JgsNoODKIYThWGongWKYdiwMYghGMIkhiAoohwTYNDEMRoDELg1juMo+jSQAyDCH4viOSoajWQQwkeDgxEgMoijGJYzhuKg0lCOpSl+S5hgoOItlGXo9lSQJrjibY8iYPByGUYx0CAch4gmEggHcaRkgWfwwCIIB5gkNqIGgZRpGcaB0gkMaHCCfgil0L54noPoBA",
        categories = "devices,gaming",
        tags = "ram,random access,technology,computer,chip,circuit,specs,capacity,gigabytes,gb",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MemoryStick,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANRoDENgiD4PAvgOBYThaBoIgqDAxDKD4RhOFYEGiGIkgeCYLCAMQ5iCEoUhkPoBA",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Menu,
    #[cfg(feature = "merge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CANggDQLQ0g+DwiD4PAvgOBYWhmBoICITQxDIIAyHYMQwC4MxhhKEgwCCLQxC0MQuDEN4iDILg4jUTISDIMoVheHIbgSHYJG0Motj0LQ1kqP4YkMPoBA",
        categories = "development,arrows",
        tags = "combine,join,unite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Merge,
    #[cfg(feature = "message_circle_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCAiG0OYRhmSQgDSVpfmGZg+gEA",
        categories = "social,account",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,moderate,check,done,todo,complete",
        contributors = "Shrinks99"
    ))]
    MessageCircleCheck,
    #[cfg(feature = "message_circle_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIA5C0MwghGEwiD4PAvgOBYWhmBoIgoMQ0CAMQ1hKEImDOFYXhyG4Eh2CRNDILg5DkMoiDYLgzDQMhhjWNYNg0MQgC4MA5iEMQuDENg3GwLZHDANokDMLgyDkYZBkGPwgkCUwzDaIpIDYOBslINAxDMLYyDkOI8CCPpaiKX5EDmQ40iKQJZlgLQ0C4N59nqfAxDmKYYi0PoBA",
        categories = "development,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleCode,
    #[cfg(feature = "message_circle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgMhhgwIIWDAIIZg8MwuDiGgiD4PAvgOBYiiWBoIgoMYdDmEAxh4MQ4hWG4ZjaFwth2HwwiGI4oieBIpgmCw3C4Ngwi6HQ3hSFoYhqF4QkaLoRDePYkkGQIFgeQ4RjIMoXi2NIXjeG4ajmHpWj+ApBluCgyg0MofDGRQ2DGYpOhsLZUlENg5mmWJrlqKhNDKMA4jKY4OneZJQhmOp/iagZCgqSqFCANguDMOZ2k2jIPlSepSpAaJZpMbaYDENgzi+DpzqEOQwpgOA1hWjZPDELYwDIMw2rmDp9GwLg5DYNavDiow+gEA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleDashed,
    #[cfg(feature = "message_circle_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCIKDcLg4DIOIRi8MA3EEM4ukuHYRhAOJYDYNBhnSdJUCANYgDCI4MDIMojn+doRkQOAzhmIKPiiQA0DeLguDaF40owMYpC4NA1kQMAwDEepfmGZg+gE",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleHeart,
    #[cfg(feature = "message_circle_more")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCIKDiEQyGiIAxl+YZmmWBJngkTQxhCeJtDCb5gmKdJzgWB52hKa57n2caAgE",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleMore,
    #[cfg(feature = "message_circle_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNDQLg5DMIIfDkMg5GEMYQikIIQi0LQxiAMw4CAMYwDQMYOg2LI7DEIAuDAOQ0jQLgxDYNxsi+Pw2DUIAzC6JoojSO4tkMMgzDaQ5FDgbJOjcMwtiAOQ4GGOZUj2MJADmP4ljSKpUmaMAzDAMpJmIOYThWGoZgSG4JE0OAuDOTAyC4Ng5EGK4rmaDZxjQNaFDWeIWnwPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleOff,
    #[cfg(feature = "message_circle_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCIKDiEQyGgOJfmGZplgSZ4JE0MYQDgdpumCYpzD6AQ",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCirclePlus,
    #[cfg(feature = "message_circle_question_mark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCIKk+IQgjIM4ukuHQgDULg4m4MRjhsMgtm6Q58l+YZmmWBJngkTQxhAMQ3GiIAxn+YqDD6AQ",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,help",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleQuestionMark,
    #[cfg(feature = "message_circle_reply")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCAiG0MYdDULZDDOLpul+YZmmWBJngkTQ3hEMhokeGZLh2SZJHYMZzmKdw+gE",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleReply,
    #[cfg(feature = "message_circle_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCIKDGEA4HYNJfmGZplgSZ4JE2aoRDYaIgDGb5inMPoBA",
        categories = "social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleWarning,
    #[cfg(feature = "message_circle_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgWYJjgaCAiG0MYtDkLY7DaX5hmaZYEmeCRtDkIJ5m+cZinUPoBA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleX,
    #[cfg(feature = "message_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4OQ5DIIAxDYLgzDQMhhhCEAwCCGwxCALgwDkNIRC6Eg3GwLQxiANg1CAMwuDIORhh6HobjaJAyDMNokhIOBsi8NAxDMLYNDkOIYCCGochGJIhDmIIPhGHY3jULQ0C4N5ZlaWAxDkIg+DwL4DgUPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[cfg(feature = "message_square_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3jiI4WiGIYmh4NhWDWIIijwMoYGgMQ2kSO4Oj0Mh6CIPg8C+A4FlOVoGggIhtDmDgxk8IA0C0NJSlSWQ+gEA",
        categories = "social,account",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,moderate,check,done,todo,complete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MessageSquareCheck,
    #[cfg(feature = "message_square_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCAiG0MYUDgLQzCCY5llOVZalmBJbgmXg0g6bwzmKcwzmeVprD6AQ",
        categories = "development,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareCode,
    #[cfg(feature = "message_square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAzGgMgiD4PAvgOBYThaBoIgoMQ2CAMQ5GgLYRhOFYEGiGIngeCRNDKHwyHaI4ShSGYpgWK4Ki6HR2DULgyDgNhhC4NwxkMMQgDCSJKkUMgxDILg1DAMhskWDA5C2VQ0DmM4mheAoqhuLZJiAYYui6SZoCAMojjEMZcjWX43mEMo6DAdokjSJ42hqLJ0CANhWDWZZqkqaIjjKJZwhmOBNg0M6DmehYjmodpuomepxnyCg4h+IZ4l2KKZoynIPp+NYB",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageSquareDashed,
    #[cfg(feature = "message_square_diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDGFg1GgNJTlWWpZgSW4JE2XggDmYZjlaZ5mgWB5pDGDQ3HaYpUnCWIBA",
        categories = "development,files,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add,patch,difference,plus,minus,plus-minus,math,code review,coding,version control,git",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareDiff,
    #[cfg(feature = "message_square_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg3CAMxIDQYQyCCFQwCCGAwC2FQyHYMQ2C4Mg4DYYYODGJ4ZiqGIoDKDAuDUMAyGyDQyjKHIijIQYdisIAxCCIQ4iOPg5EiNoUhaPYYDKHB2C0NIOCIPg8C+A4FlMYxpHIYxsGUIBjHiCQxDkIpfHmCQ2mUcoJDOUpUlmW5dD6AQ",
        categories = "social,notifications",
        tags = "unread,unresolved,comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareDot,
    #[cfg(feature = "message_square_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDeKwgDmKxjhQLg2DiXgyDYNYOC4Mw4DOZQ5DebA4DQNBsnAMAwDkIJwmkNBhiaKZsoOFKGCCJwwl4M6DDGT6HmQN5vC0Lp6DShKXoKl6QogMw5DilKMnilQwqCfg2DQQQyjic6rDec6ckuX5hmuq61l+sYNh+tohriPAtiqa4mDWU5VloPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareHeart,
    #[cfg(feature = "message_square_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA4C4NRWDUYYNg0MAghcMAtDKGxIDSFAghaGIYhuIR2DENguDIOA2GELg3DGLwxiOGQgjEMgxDKEAwDIbI6DKPIbiqPBBhWNI2CCKQ4iuNg5EgMQwCIPg8C+A4FlOVoGgiCpAjYNR2huIIihkLQ0hgdgylKVJZlMchlGMdAgGgZRpGcaB0gkNQiCAch4gkMZ7n4IgxDSex3GkZIFgkOJ7Hmf56lML5unAPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,secure,encrypted",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageSquareLock,
    #[cfg(feature = "message_square_more")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDGDQxDEaAuDAMZTlWWpZgSW4JE2S4OmGY5llSVpqmmBYHmwOJvmKZJmnSWIBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareMore,
    #[cfg(feature = "message_square_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIIMEgNguDgMg4GEMgghcMAghoMAtDELg0DENAuDUOA2GwLQyC4MgwheKosDIQQuDeM4bjYMYYhiH4UDYVg1haOYcg6DguDEOA0h6EgyDYIg+DwL4DgWTpRgaCAiG2LoYhqLJNk+VJTgSVYJE0OAuDYNQ2CAMxIiyQIZjeOQyHYMYfDMNA0l2UJhD6AQA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareOff,
    #[cfg(feature = "message_square_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDGDQ4HYNpTlWWpZgSW4JE0OYODEaJilSVpnD6AQ",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquarePlus,
    #[cfg(feature = "message_square_quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIIMGEMgghEMAghSFAyC0MhWDgaIZCIPg8C+A4FiCI4GgiCgyhEMQ3hCEoVjAMYZhISA2C4OAyDiLoTjAMAtDELg0gwLg1DgNhshkLgyDCEQykqTBBC4N5AlOPYOi8MpAjkNhWDWO5WDGEoZGgMQ2l+FpXk0eofiGJolgSJ4JE0OIODSZ5WhiGg4EgOJsiKcA+gEA",
        categories = "social,text",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MessageSquareQuote,
    #[cfg(feature = "message_square_reply")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCAiG0MYUDgLQzCCY5llOVZalmBJbgkTYPg4NB2iWTYUnWGIYEgN5nlaaw+gE",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareReply,
    #[cfg(feature = "message_square_share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzEgNBhg2DQwCCFQwC2Ex2DENguDIOA2GELg3DGIwxhaKIViUMoMC4NQwDIbAyh6MIZjQMhBhOKQgieHQ4h+PA5EgMgwhIIIUjsMoZHYLQ0CIPg8C+A4FlCU4GgiCocg4aA2HYNpPlGVpVgSV4JG2Wg5CANgtl+UJSmQPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,network,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareShare,
    #[cfg(feature = "message_square_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDeDgxkoMJTlWWpZgSW4JE2XgxDUaA2mOVpnmaBYHmmXg3GgOJwmWAQA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareText,
    #[cfg(feature = "message_square_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCIKDGDQxDUaAuDAMZTlWWpZgSW4JE2XggDcdg0maVpqD6AQ",
        categories = "social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareWarning,
    #[cfg(feature = "message_square_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgWVJXgaCAiG2KYrCAOIrC0NQgDWU5VlqWYEluCRtDmYJimWc5olabA+gEA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareX,
    #[cfg(feature = "message_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA2C4OAyDiEQghOFYVC0MQuDQMQ0C4NQ4DYbIYC4MgwhKMYzEELg3iaOYjhaIYhiaHg2FYNYgiKPQyhgaAxDaRY8g6PgyHoIg+DwL4DgUPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MessageSquare,
    #[cfg(feature = "messages_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDAYQyCCEgwCCFQxC2EgyEgNguDgMg4hGE4WiQMAtDELg0DENAuDUOA2GyGQuDIMIajONRBC4N4ojuJYOiOK4zi8Vg0iKFIkDGE4ZGiD5Gj6SYaHoIg+DwL4DgWVJXgaCIKjQIA5k6FYXiMMh2g+Qg2GGOo8kmYoOieMwxDKLY1jGc40DKMp4EGGo+hcNwuDEN4SDEORIk2fZuhieZlieU5VloPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubbles,copy,multiple,discussion,interview,debate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessagesSquare,
    #[cfg(feature = "metronome")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDELg0FYOQuDEIg+DwL4DgWGIbgaCAiG2DIODcIA2C4NQ5C2J4pheGYeh2BIfgmIg1C4MA1CCNg3C0LgyDEOI9DYOQxGEMwgkcMAgkoMAtjYNg2kkTA0hGQIOhSQw1EEMYOkuXpKjYM4lj8aAxDOPg1DMYZclyTJeC4OQ1DELYQDMMRMkCEY5DGJwyi6GoyhgYxpHIYxsGUIBjHmCQ5CIIBygmfqJHikQwn+g6FocPoB",
        categories = "multimedia,time",
        tags = "metronome,tempo,rhythm,beat,bpm,music,audio,sound,practice,timing,timer,time,pulse,sync,cadence,control,playback,studio,tool",
        contributors = "jguddas,edwloef"
    ))]
    Metronome,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDkdgzCIPg8C+A4FhSF4GgiCgxDUIA5C4Mw0FYNRhDMIIoDAIIrDALQ1C4Ng4C0MYihKFIWgQaIZjqB4JgsNguDmHwxkGQxBDcIJJi2DggkQMh2C0MoThWGo8gWPodDgLg4DmDgzC4MgzkiSosmaK4Pg6UJSlSOYYgKPYcG2DZ0isMgwm2VpwliHBNl6EAzieKZnmaMIMCAMgugyeY6D6AQ",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,LieOnLion"
    ))]
    MicOff,
    #[cfg(feature = "mic_vocal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIA3C4NgwDELQ1C4OQ5DQIA4C4MQ5GGDYNDAIIiiKHAgDELgyDkOBsC4OAxg+Lw4h+J4jjaIooDMMQ0C4MA4DcTAxhUMA5icMgiD4PAvgOBZJkyBoICITQxDYLg1CAMoojANBDkKVpYDCX47C4Mw3DKYInDOYBjC0Mo9DUOIjC0M4WDKcZuDMNQ2C0NpYm2PZmC2ZJ6n8Nw3DWc5kDYOQtiiiI8DWSJKk+SRjGkchjGwZQgGMeYJDcIggHKCaRpweIJlSkgvpamKaD6AQ",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "jguddas"
    ))]
    MicVocal,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDkdgzCIPg8C+A4FhSF4GgiCoPg4MB2DIYQ3CCJAwCCJwxC0MQ0igdgtDKE4VhqFByGUYx0CAdxpGSBYJDYIggGgZRpGcaB0gkMYSCAch4gmSx5gmMQgk4Ig5jIL42jgPoBA",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,listen,radio,podcast,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Mic,
    #[cfg(feature = "microchip")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIaA0CIPg8C+A4FhSF4GgiCoMg4N4RhOFYahmBIbgmC4NiCEoUhaJolgWB4oDEOIOhAMoii6GICiaModjWNBojiLYkjyMYcguNQ2kKOZFhqPhNDSNpMkSL5GieCpSkGQ4jlaT5IlKS5cjoaIUHIZRjHQIB4gkNgiCAcpsCKOAgGgZRpGcaB0gkMgwm8eZ8m8dxpGSBYJg+OZnmkPoBA",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,integrated circuit,memory,ram,specs,gpu,gigahertz,ghz",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    Microchip,
    #[cfg(feature = "microscope")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ4GgOAiD4PAvgOBYThaBoIgoMwgDIMhog6EoUhmGIEhqCRNDENIeDIYQ3CCMAwg0IIzDALYrGiOIjhWJ4mgWB4pDmDQ0GgMo8iWAonkGCpDDGLgyh6NZTDELQylYVg2GgNh2DQYZRlGNoNlaHhakiPpKkCG4qlENhWDMYQxjSYo3lUMRIDmcZzlOdYNHYM5nheAQ",
        categories = "science,medical",
        tags = "medical,education,science,imaging,research",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Microscope,
    #[cfg(feature = "microwave")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDSDB3GkZB0GiCQyDCDBygiCoMGgZRpGcaB0gkMQ1CIPg8C+A4FimLIGh+IYjgkN4Mg4Ig4hGE4VgmOYHgkNoahwMYoiqL4pHAYYVCAZIJE0MQ4CAOB2jWKQvkmFZIkoaJMk4NggDEOR2guVpYGiWpLk0IpPlGYZjkWV5bD6AQ",
        categories = "food-beverage,home",
        tags = "oven,cooker,toaster oven,bake",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Microwave,
    #[cfg(feature = "milestone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg4CIPg8C+A4FhSF4GgiCoMCCEAzhOFYahmBIbgmCw4C4MQ3g0Nhhg2DQwCCMwxg4Lg0DENAuDUOA2GwMguDANggkGQxhDELgyDAN43kuTYzjWNI3DcMA4GwLZGkSWowkWU41C2SY5juPQ2EgNJIg6X4OmGYRWDeaY2lGagxmEeoihaJg+gE",
        categories = "arrows,navigation,development,gaming",
        tags = "signpost,direction,right,east,forward,version control,waypoint",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Milestone,
    #[cfg(feature = "milk_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhoDgIg+DwL4DgWEoVgaCIKDmDR2DELgzDQMxNDENYdDILg3DgORhDQIIuDAIIxjELg2DcMoNC4MgxDkbI1DUNguDkOA0i2L4ykgMQgjWN45DIMoeC4MQzDETQ3C6DJXDgbAtlIMg4lIOQyEGLowkiMQ3CAMQwjoMQyFYMgwGGOI4jOSJ0GgNpzg2Z53C2UAtDOEYThiF4EhmCZWmoNRhkENJpo6aZ2kqJoxpGOKXn2MQzC6Ig1p0Mw1oOFKHhIbBpG4ZQgHkMYJDIIggHirQiq+sQyq6tR5retKvhIL6nqkPoBA",
        categories = "food-beverage",
        tags = "lactose free,bottle,beverage,drink,water,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MilkOff,
    #[cfg(feature = "milk")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhoDgIg+DwL4DgWEoVgaCIKDmDR2DILg3DgORhDQIIlDAIIoDELQuDYNwyg0LgyDEORsiwNg1DYLg5DgNBBiWJ4pkINwgDEMIyDEMhWDIMBhjCMIolGDYNGgNpOlOUooDILQyHYLQ5iCIokiaQpRjeL5ckiNY3jmLI8j6QJlkWRQ1iaYQ4kuEYThiF4EhmCRNkQMQ1GGOg0i8IKGoiUgxCCdYooqiQuoecooo+eoUn4PoB",
        categories = "food-beverage",
        tags = "lactose,bottle,beverage,drink,water,diet",
        contributors = "karsa-mistmere"
    ))]
    Milk,
    #[cfg(feature = "minimize_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDAIA3C0NwiD4PAvgOBYWhmBoICITQyhCDxoC0NhWDSFYXhyG4Eh2CRtDMIAyDGEYTimGItiyBYHgkTYNgwaA2HYNo3iuAQ",
        categories = "arrows,layout,design",
        tags = "exit fullscreen,arrows,close,shrink",
        contributors = "colebemis,ericfennis"
    ))]
    Minimize2,
    #[cfg(feature = "minimize")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMx2DMYQyCCEgwCCFQxC2EgyEgMwiD4PAvgOBYfiKBoIgoMgxCAOBoC2EIahaMYYDKGRWh2H4hgQaIkjqB4JE0MwgDENhoi+E4xheRwyg+HogiWPIFj6CpDhMMR2i6EZHhWSY0DKRZNjmI4BA",
        categories = "layout,design",
        tags = "exit fullscreen,close,shrink",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Minimize,
    #[cfg(feature = "minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FD6AQ",
        categories = "math,development,text,tools",
        tags = "subtract,remove,decrease,decrement,reduce,negative,calculate,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    Minus,
    #[cfg(feature = "mirror_rectangular")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA2CAOAgDkIg+DwL4DgWFYYgaCAiG0MYPDcLYRDiFIWhuFRyGUYx0CAeYJDIIggHiCQ0jIaBlGkZxoHSMAwjIco0CKMQgHcaRkgWCYgiYL4qiwPoBA",
        categories = "science,home,tools",
        tags = "reflection,optics,glass,surface,image,physics,science,bathroom,decor,cosmetic,shiny,periscope,vanity",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    MirrorRectangular,
    #[cfg(feature = "mirror_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA2C4NggDiEISCIPg8C+A4FheGoGgiCgxDIIAxDgdg0haGIdhyBIegmCw1CANwujAOYziMM4ohmLIrgWB4uDcIAyDIaIMjmKg8GMaRyGMbBlCAYx4gmIQiCAcoJDiVBjHmUgwjmSZLk0PoBA",
        categories = "science,home,tools",
        tags = "reflection,optics,glass,surface,image,physics,science,bathroom,vanity,makeup,decor,cosmetic,shiny,periscope",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    MirrorRound,
    #[cfg(feature = "monitor_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMoQCANAtDQIg+DwL4DgWGByGUYx0CAaBlGkZxoHSCQxhYIB5gkMwiCAdxpGSBYJDIMIwHIeI2jCOwiDKF4Zh6IIYhuBoICITQxhEMQ3HaFoYhqBBokWU4HgkTQ4hAMRoDiQZShyAQ",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_cloud")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDMYQzCCEgwg6FgyC4OAzC0NBIDENBhDIIIihWFYNhUNB6CIPg8C+A4FiyL4GgiCgxiIMQ3HYNIri2MoxgSM4JE0OIjDEaA4jyLpAiwchlGMdAgHIeIJDIIgglMIpVCAeYJDOVh3GkZIFlQMJWGgZRpGcaB0gmH5Jk2Tw+gE",
        categories = "connectivity,devices,development",
        tags = "virtual machine,virtual desktop,vm,vdi,computing,remote work,monitoring,infrastructure,software as a service,saas,workstation,environment,tv,screen,display",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCloud,
    #[cfg(feature = "monitor_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GggIhtDENAuDMMA1CANwuDUMwuDkMgzC2IQ4DKE4VhqGYEhuCYeDULgyDIOAgiAOA1DKLori2LwzjKFo2jWBYHjgMQ2C6QYNimPA4i4Mw4kaK4ShSSoYgKNpOh2UJSkIIA4C4Nw3kOR4qiySY0mGTYch4OQuh+PpVj2R5EDKXYzkuc43mSd4oCCdw2DkNpYjCfqAl8aJMoQbQyDCapsj+Zgym8NKNkiXpyhqY6Vpea4NiaeadkecaCqOHBNjyDgzHYMhhg2DQwCCugxC2uBIDStwgrmu4Or6vhWDWwrEryw6+GgN6tmCr4JE2PgyDEaA4tKkg8GMaRyGMbBlCAYx4gkMbbCAcoJki5R5gkNpJt+4bjD6AQ",
        categories = "connectivity,devices",
        tags = "tv,screen,display,virtual machine,vm,executable,settings,cog,edit,gear,configuration,preferences,system,control panel,network,computing",
        contributors = "karsa-mistmere,colebemis,UsamaKhan"
    ))]
    MonitorCog,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GgiCgyg2DAuDMMA3FYMQ1GGDYNDAIIrDELYpEgNIoCCKosg6L4vFaJ4pjaLY0i8aA4C4Ng5DOE4VhqGYEhuCRNDiNAxkGR4WkuFBjGkchjGwZQgGMeIJDEOQil0eYJDaYxygmRoUC+V5ZlsPoBA",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GggIhtDENYODALQzCAM4jiOE4VhqFByGUYx0CAeIJDIIggHmCQzjUaBlGkZxoHSCQxDSNR3GkZIFjMMI1HKMgijSFAvi2L4ZgSG4JguDQxDcdpDlCK4ClWB5XDgIAyDEaA4imFpVD6AQ",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,download",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorDown,
    #[cfg(feature = "monitor_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GgiCoPg4NxIDQYYNg0MAgiYMQtDKKhWDWIwgiWJ4Og4LgxDgNAtDELg4DINoThWGoZgSG4JG2JIwiYMgwj+FpDkKBYHgkTQ4jAMRoDiTJBgKQ5RgoOAuDYNQ2CAMxIkqL4xiiMIwHYMQwmiMooC0LgyDkM40DANAylmToBA",
        categories = "connectivity,devices",
        tags = "share",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    MonitorOff,
    #[cfg(feature = "monitor_pause")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDMVg3CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIUHIZRjHQIB4gkMgiCAaBlGkZxoHSCYejAeYJDOMByi0IovCAdxpGSBYuDCE4ViiKoZiWB4JgsMoODcdg0kqJIYgKT4cE0OAgDIMRoDiWIaD6AQ",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgwDMMwgDkLg0DQYQuDYNA3hiGggDCHggDGIAxC4MQyGwLQ0g4Ng1CAMguDMNQyheGYNjWIIfDELQuDkNg4jsNQ2FaG4ZDiNA0jaSI4iEII8DYN5ADYegiD4PAvgOBZVliBoIgqJohDcdg0lSVpblqBJcgkTQ4i4MRoDiZJXmiVRyGUYx0CAeYJDMIggGgZRpGcaB0gkMZjCAeIJDKfR3GkZIFooMJ9HKiQiouVQvnWdw+gEA",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdg3h+FYiiAMhoDgIg+DwL4DgWMIzgaCIKDGFgxDkdgtDMLg5DYIJADENYvjGNo1gSN4JE0NwgjwaJHjCMpMjAchlGMdAgHmCQxDIIggGgZRpGcaB0l8MJiHiXw2mIcptCKYQgHcaRkgWCZvlWWZbD6AQ",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NQgDIMBIDgIg+DwL4DgWFYYgaCIKDENwgDkaAuDAMYUhaG4VHIZRjHQIB3GkZIFgkMQwCIIBoGUaRnGgdI0DaNx4jQMo3HmCQ0jccpCCKRIVC+K4thqBIcgkTQ4CANhIDQYQyg8IAwl+Xwtl0Mh2DaXJemCapeDIaJIk6KQ8GMaRyGMbBlCAY5GCIMQ1kmNI3GOS4ficL5znWdw+gE",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GgiCg4CAMgxGgOIThWGoUHIZRjHQIB3GkZIFgkMgwCIIByHiMY0jcIgyjQaBlGkZxoHSCQxhIIB5gkM4kC+KIqieKYrkgIg3j2P5BkMIg2jmCQ5jSNpEjSLYvGiCZahSTJQD6AQ",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMwtDOEIQCIPg8C+A4FheGoGggIhNDEMoNDMVg3haGIdhcchlGMdAgHIeIJDIIggHcaRkgWMwwjUaBlGkZxoHSCQxDSNYyCKNAgHmCQzigL4si6HIEh6CYhiMMQ3HaRoXhmVJTgWB5WDgIAyDEaA4k+KoB",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CAMQyg4LQ1hQIg+DwL4DgWGIbgaCIKDmDoQhKD4VDWF4Zh6GByGUYx0CAch4gkMgiCAaBlGkZxoHSCYMjYeYJDONh3GkZIFjQMI2jMIo1hgL4ti+HYEh+CRNhGEA3HYNIphqVJTgWB5WDgIAyDEaA4l2K4B",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHIeINDKERoGUaRnGgdINDENIRhSDoRHmDQzCIPg8C+A4FikbBpG4ZQgHkMYVDGIY1CIOIhDKHg2iSPYOjeKQvi+MYujCMh4jkMYWjOQQyjeM5MDePIehaRJGGUPoBA",
        categories = "connectivity,devices",
        tags = "tv,screen,display,virtual machine,vm",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    Monitor,
    #[cfg(feature = "moon_star")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA1GgNAiD4PAvgOBYThaBoIgoMgwCAMx2hGE4VgQaIYiWB4JE2HQuDkOA1CAMQyC4NA4DYYQ5CCOYeDGMQtDmNA3DOP5BDIY40DANQtC4MAyjMNgxDeNA2kiMw4DAMxhDYIJbh6XggDgLgyDaDZhmMOJHDMNA0ksMgxDULg4DKSpMDANJxliSAxhKFIZD6AQ",
        categories = "accessibility,weather",
        tags = "dark,night,star",
        contributors = "karsa-mistmere"
    ))]
    MoonStar,
    #[cfg(feature = "moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg5DgNQgDEMguDQOA2GEOQghoMIShILQ5hUNwziCIgyGOFQwDULQuDAMoUDYMQ3hUNophQOAwDMYQ2CCPIdj8IA4C4Mg2DiQZDkWKAzDQNIsDIMQ1C4OAyiuLQwDSUo5ikMQiD4PAvgOBQ+gE",
        categories = "accessibility",
        tags = "dark,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Moon,
    #[cfg(feature = "motorbike")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDQLQxC0MwiD4PAvgOBYWhmBoIgoMwgDkIA2CAMhhDKJQgDCKoOiULQyGiJooiiK4rDGDguDmIgxC4OAxhWF4chuBIdgkTYNDENxoDMYY3jeNYshKN4kiSUJTC2JJOiyVoRHYLQuDcNRBDUIJklCNg3mWQIYkSFhjGkchjGwZQgGMeYJkkIggHKCYUnUeJ4Dma5vnGc5unCcp0GOgAiDWep8CKfp2ngN6DoihoBA",
        categories = "transportation",
        tags = "moto,motorcycle,transport,vehicle,drive,ride,trip,race,racing,journey,delivery",
        contributors = "jamiemlaw"
    ))]
    Motorbike,
    #[cfg(feature = "mountain_snow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMwgDQIIMDULQ1CCFQxDUSAyEyDAzHoIg+DwL4DgWIYkgaCAiE0NAuDGEIYC4MA4GMMguDYMgtDELg1DeFguDINI5C4NIODcLg4DaQwyCCNQ3i8Lg5hANZDDmTIRj8M4tDmIIiicPoB",
        categories = "nature",
        tags = "alpine,climb,snow",
        contributors = "kerkeslager,ericfennis"
    ))]
    MountainSnow,
    #[cfg(feature = "mountain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMwgDQIIMDULQ1CCFQxDUSAyEyDAzHoIg+DwL4DgUPoBA",
        categories = "nature,gaming",
        tags = "climb,hike,rock",
        contributors = "kerkeslager,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Mountain,
    #[cfg(feature = "mouse_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3C4MwxDgVgxDAIg+DwL4DgWGIbgaCIKDUIIVHYNRhDeDggDCKosDENIqFYORjDALQzC4NQyDcLQyC4NgwDgLQ2jcMQ1kELQ3heGYehgYxpHIYxsGUIBjHiCZICAcoJDIIpTHmCQ0kkL5Nk+UQ+gE",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,marvfash"
    ))]
    MouseLeft,
    #[cfg(feature = "mouse_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2HYLgzDQMwiD4PAvgOBYWhmBoIgoMQ4C4MogCCIIiiAQQ3CCKgwCCLQxCANYlDUVg5GGKosi6JQgC4Nw4DILQzicOIVheHIbgSHYJgsOYlkKEgzjWKYrjqLYtiENQ2CAMguDkMAykWGJJkiBYHksMoNmiW5bmGR4BA",
        categories = "devices",
        tags = "device,scroll,click,disabled",
        contributors = "karsa-mistmere,mittalyashu,ericfennis"
    ))]
    MouseOff,
    #[cfg(feature = "mouse_pointer_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDULg1DUIA4C4NIQg0MQzDgIAyC4MA4DcYYOg4IAwiMIAxC2HA2DMLg5DQNxsC0NguDEMg0iaDg4GEMoaiWJAwC0MYThiNw0DMNYwkENQ3DkIIyjQNogg2EI+iaKItDeKQzEyEoUiaUg1DEIg+DwL4DgWY5mgaCAiE0Mo7m+GgymKZJpmiBJqgkbYyDgMQ2iaSQyDgLYbDeS4xC4OA0iANA5g2jJTiUMQgC4NpgiilQxGye42oSS5zmWdw+gE",
        categories = "arrows,cursors",
        tags = "pointer,mouse,cursor,off,disable,arrow,navigation,selection,select,click,no-click,interaction",
        contributors = "ericfennis,csandman,domingasp,jguddas"
    ))]
    MousePointer2Off,
    #[cfg(feature = "mouse_pointer_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4MAzDcIIMDYOA4GELg0DkNYXhkIAwh0IAxCALg2DUMQtiOJRsDENggDYLg1haGg1h+Homg0NgzC4OQ0DcbAti4MQyDSIIvhUMggkeHpKC0MYXDMOJEDQMw1j6TQ1DcOYtC6QQ2jGL40iCJ47DeNwzHoIg+DwL4DgUPoBA",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "ericfennis,csandman"
    ))]
    MousePointer2,
    #[cfg(feature = "mouse_pointer_ban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4MAzDQIIMDYOAxGELg0DkOIXhkIAwh0IAxCALg2DQNwtiOJRsDkIAzC4NYWDWLofh4MYng6LQ5DQNBMhoMgwhAN4uDSL4hiGHpHicNg2iMNhsC0MYNkqLIXDQM4wjKR4gieOQ0g0MwzHoIg+DwL4DgWYxjGkchjGwZQgGMeYJDENgiCAcoJnSbx4nKdJjC+aZrm2Y5mgaCAiG0MZQDiIKKCCGoQo+YpkoQPoBA",
        categories = "arrows,cursors",
        tags = "wait,busy,loading,blocked,frozen,freeze",
        contributors = "danielbayley"
    ))]
    MousePointerBan,
    #[cfg(feature = "mouse_pointer_click")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0C4MQgDEMggDYIg+DwL4DgWGIbgaCAiG0NYQCAOAtDILg5C0Lg4heGYeh2BIfgkbQ2hIMgtDGKQgDKLoajKMYFgeCRNDcLoUiiFA4CCIwxj6MICjKQ4KDkLgwDMNwglUNg5GELg0DkOJfmEIAwmWEggC4Ng1DOK5rDMbAxhGDw1l6Iw1meZgxisMA3g8OZgGwLYPDOYISlYNAxGGEYRmajorn4Lg3DMOaCjoMINoSdQuneeYSisOYjn0NB6k+QIB",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    MousePointerClick,
    #[cfg(feature = "mouse_pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg1DgNgggyDoQhIOYWCIPg8C+A4FhqHYGgiCgzC4Ng4DgIIkDAMw3GELg0DkN4vjEIAwjWNQtiUNQxjoMRsDaDoSDULg5kWLg1DCPJIDGN42jaRA0DeOQwDYMhskoNg5C2QAwDgMxhDIIJhk6EoSi8NA4C2PJRDmPwugwNJqg4Nw5keQZkk+VAzjkOZRHqGYbiAPoB",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "ashygee,ericfennis"
    ))]
    MousePointer,
    #[cfg(feature = "mouse_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3C4MwxDgVgxDAIg+DwL4DgWGIbgaCIKDEOQghUdg1GEN4OCAMIriQLQxDSKxWDkYwwC0MwuDUMopDILg2DAOAtDaOQxDUIA2C0N4XhmHoYGMaRyGMbBlCAYx4gkMZKCAcoJDIIpVHmCQ0ksL5PlGUw+gEA",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,marvfash"
    ))]
    MouseRight,
    #[cfg(feature = "mouse")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0Ig3CIIB5goMoNHcaRkHQaIKDENINGgZRpGcaB0hAMINgkIg1CIPg8C+A4FikcBhhYIBkgoTQxDIIA2HaGopC+L4WD6AQA",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Mouse,
    #[cfg(feature = "move_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMx2DENhohAIg+DwL4DgWFYYgaCAiG2DAxDkIA2C0NoUhaG4agSHIJG0Mojg0LQzg2DYnheK4qgWB4tDEOAghCNAzjKNYVjeGYB",
        categories = "design",
        tags = "arrows,axis,gizmo,coordinates,transform,translate",
        contributors = "lscheibel,ericfennis"
    ))]
    Move3D,
    #[cfg(feature = "move_diagonal_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDMdg2GgLQ2CIPg8C+A4FheGoGgiCg1g4MRWDUaIVheGYEGiHIqgeCRtiGIQxDSDg0haGIdD6AQ",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDkSA1HYLQ2CIPg8C+A4FheGoGgiCgxDMIA1GgNh2hWF4ZgQaIciuB4JgsOYjjOD4WhiHQ+gEA",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal,
    #[cfg(feature = "move_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDkSA1FYMQzCIPg8C+A4FheGoGgiCoPCANRMDWDg5haGIdD6AQ",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "siarie,ericfennis,karsa-mistmere,jonas-hoebenreich"
    ))]
    MoveDownLeft,
    #[cfg(feature = "move_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDMVoMEiDwiD4PAvgOBYWhmBoIgoNQgDUTIMg4OYVheHA+gE",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDownRight,
    #[cfg(feature = "move_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ4EwMQyCAMgyhANoNDgIg+DwL4DgWG4egaCIKhGExWhSGociEPoBA",
        categories = "arrows",
        tags = "arrow,direction,downwards,south",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDown,
    #[cfg(feature = "move_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIINDQIA0C2EA0CIPg8C+A4FheGoGggIhNDIIAxDIaAyDCFoYh2HIEh6CRtDaDoThGNIVheGYtD6AQ",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis,csandman"
    ))]
    MoveHorizontal,
    #[cfg(feature = "move_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOBMDIIAxDITIMDENgiD4PAvgOBYZhyBoIgqEISEgMgyhiGofD6AQ",
        categories = "arrows",
        tags = "arrow,direction,back,west",
        contributors = "jonas-hoebenreich"
    ))]
    MoveLeft,
    #[cfg(feature = "move_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4EwMgyCAMQyEyDITDYIg+DwL4DgWG4egaCIKhKFBIhGGociEPoBA",
        categories = "arrows",
        tags = "arrow,direction,trend flat,east",
        contributors = "jonas-hoebenreich"
    ))]
    MoveRight,
    #[cfg(feature = "move_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQxFYNRIg4Ig+DwL4DgWFYYgaCIKgwNRMDEOYNDmFIWhsPoBA",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpLeft,
    #[cfg(feature = "move_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA1EgMQ5FYMQxCIPg8C+A4FheGoGgiCoRg4TA1CCEYWhiHQ+gE",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpRight,
    #[cfg(feature = "move_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANhMDEMggDKDw2g0Ig+DwL4DgWGIbgaCIKhCEhWDIMoXhmHg+gE",
        categories = "arrows",
        tags = "arrow,direction,upwards,north",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUp,
    #[cfg(feature = "move_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgwCIPg8C+A4FhSF4GggIhtDgIAxh8NAgiMNAtDSE4VhqGYEhuCYeCANokieJIkimFotD6AQ",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgwCIPg8C+A4FhSF4GggIhtDENQgDEOQtDMIAziSJIThWGoZgSG4Jh4OQgjKJYnjWKoWi6LYFgeCRNg2DBohGOIsgKLo9h2IIjjWJomkSOpGjyHBtjKII2k0M5PhiAQ",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ4CKCR4gyDwgHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME0MQyCCDhWDIbA3CCGIaC+IYjD6AQ",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ4hAcoMDQIg+DwL4DgWB4bHAYR0GgIBkgwTQxDYIIUFaD4bC+IokD6AQA",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWG4egaCAiG2DIMhOFYahyIYbGMaRyGMbBlCAYx5gmDgijQeIJDaORygmGYbC+LowjKLYvjGMxjjsIo4jSNpNj0II/CKQYckSSQ+gE",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWGxjGkchjGwZQgGMeYJg4IonHiCQ2iwcoJhmGwviGI4liCIokiaMgihmLYqDiLIoiqMI1jePA+gE",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4MwxCCDIOCANQgDIMRsDcLQ0CANwgDQLQxC4MYZDOIg3CIPg8C+A4FimLIGgiCgxDQLg1DMIA4C4OA4CAMQyhUbIgiYIIliOKIqi+KRsGkbhlCAeQygkMgyCIIB4lEIpTlUeAxlKVR5l2WZHC+S5ND6AQ",
        categories = "navigation",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Navigation2Off,
    #[cfg(feature = "navigation_2")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDIIITDEOYUDEIIShoNwgDWGIahMMgiD4PAvgOBYHG4PoBA",
        categories = "navigation",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation2,
    #[cfg(feature = "navigation_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NAzCCDIOCCDwxDEbA4CAMoZhCGQuDUNwtDWDQzCIPg8C+A4FiaKYGgiCgxDcLgzDkIIVC4N4PDKGgyGwLQ5jeDw0C4NgxiWJ4siYbBpG4ZQgHkMYJDIIggHiUAilKTgylGWB4lqV5SiYL5KkwPoBA",
        categories = "navigation",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NavigationOff,
    #[cfg(feature = "navigation")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgzCAMQxCAMgyhaE4SDKFYUhoIIShQIg+DwL4DgWBxuD6AQ",
        categories = "navigation",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation,
    #[cfg(feature = "network")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHmDgxhAIB4hSFh3GkZB0GiDoWHKGAiDEIg+DwL4DgWJ4qgaG4dh+D4RiMMoRhOJIWgiCoMiCEYihSJooi2LIEi6HIej2B4JguDYyhKDo1heDg5j6I4lieKZFiccBhh4IBkg4TQ1CCFR2C0MxhDGZAgDCbJrDELQxGgMQymma5tm2ap6HYM5BC+XIeluXRol+YZ0mQMhWDifqAGgPoB",
        categories = "development",
        tags = "tree",
        contributors = "ericfennis,johnletey,csandman,karsa-mistmere"
    ))]
    Network,
    #[cfg(feature = "newspaper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDgaAtDUIg+DwL4DgWFYYgaCIKg+Dg0hEOIUhaG4agSHIJE0NAgDIMhoDENhhDKLQgDCNo4DILQyFYNIzjWN5BjuOxIDiP40kGNo7i0doxkeOI3DELYsDAdgtDmT5JDGLY7GgMokheKIVHIZRjHQIB3GkZIFgmIwgGgZRpGcaB0gkNAiCAeIJDEMJ4HKegiDGeB5gkNpgmSZg+gE",
        categories = "multimedia,communication",
        tags = "news,feed,home,magazine,article,headline",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Newspaper,
    #[cfg(feature = "nfc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOAuDMMhhDcLg0DMIIThUIAwhoIAxhyEwzDYIg+DwL4DgWJIngaCIKDmFIMDYLgyDEYQxDELg3gyNo4gyG4bh6P43DUOIjiWKopgSK4JE0MQyC4OYeDQLo0DENZPh6VZXhyPwgC4MJYlaRIkiaSZIgWB5LDGMQzDcIIRDIMJTgycJyluHYcnCRZkiiAQA",
        categories = "communication,finance,devices",
        tags = "contactless,payment,near-field communication",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Nfc,
    #[cfg(feature = "non_binary")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQwCIPg8C+A4FhSF4GggIhtDgLg1CANAgDeIoThWGoZgSG4Jh6IAgDiJAtDSJ4WiuFBjGkchjGwZQgGMeIJgwIggHKCQ1kQYx5kIN41jmO49D6AQ",
        categories = "medical",
        tags = "gender,nonbinary,enby",
        contributors = "jamiemlaw"
    ))]
    NonBinary,
    #[cfg(feature = "notebook_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg0CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsdgtDeDgiD4PAvgOBY0jeBoIgqFQ2GgNIzjWOo5gSO4JE2FQxDCQJCjaRpFgWB5IkoNJNjST44gKRpTj0IAxDiV5DlCW5SjySQxC4Mw3DgIA1C4NgyhMMZfhmdAwC2DQwDANJ5C6ew0GwLZvDCdKEieKYViueAuDUMA2C4OA1oELaRmuEKRDcYaNo2IoZnAMqgGyoQ4DelQ4muiaehikaTpWjg2HqTpEgEA",
        categories = "text,social",
        tags = "pencil,notepad,notes,noted,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,eraser,rubber",
        contributors = "danielbayley"
    ))]
    NotebookPen,
    #[cfg(feature = "notebook_tabs")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAeIJhAIB5gkMgiCAaBlGkZxoHSQwwkUco+CKRAgHcaRkgWCQxDaIAvjmO4viWCgxDUIAyHYMpOiyIoCiSMZimQNxoDWXJrhibhNmOKAynKdI3naGp4mQMZxnOaougE",
        categories = "account,communication,social",
        tags = "notepad,notes,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking,alphabetical,a-z,organizer,organiser,planner,diary,stationery",
        contributors = "danielbayley"
    ))]
    NotebookTabs,
    #[cfg(feature = "notebook_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAdxpGSBYJDENgiCAch4gkMpFHmSZFGgZRpGcaB0kkMJFkgIoQiyOY7i+JYKDkLg1CCNA1iCLYWgKJIxmCYooDISJDmeIpqjCGhNmGY5DnGWo2haAQ",
        categories = "text,social",
        tags = "notepad,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,lines,opened",
        contributors = "danielbayley"
    ))]
    NotebookText,
    #[cfg(feature = "notebook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAeIJhAIBoGUaRnGgdIJDIMAiCAeZIksdxpGSBYJDENpLHKPgiDKIAvjmO4viWCpVCAMh2kmXIigEA",
        categories = "text,communication,social,design",
        tags = "notepad,notes,stationery,sketchbook,moleskine,closure,strap,band,elastic,organizer,organiser,planner,diary,journal,writing,written,writer,reading,high school,university,college,academy,student,study,homework,research",
        contributors = "danielbayley"
    ))]
    Notebook,
    #[cfg(feature = "notepad_text_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDEMoNg+EYThiF4EhmCRNDENoehCEoUiSI4FgeJooCANBoDIYYdh0MAgjsMYNh4Mogi2FoCiSMYKDKPYOkGLIikWMIaE2SQgDEOB2jeOY8loMQtjkaAtDGQpOhiR4nDODQyl+TIhi6T4lgoN5oEgNo4j+O49l2XR2l2YptmSUQ0lQNJ7muQxoi+bxNoEOBWnSWZ3lSDZdjafZEn+JoMDEMBoDalaHm6ZaZjUOKeoioZUDgaA1qWAQA",
        categories = "text,social",
        tags = "notebook,notes,pages,paper,stationery,diary,journal,writing,write,written,draft,template,lines",
        contributors = "danielbayley"
    ))]
    NotepadTextDashed,
    #[cfg(feature = "notepad_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDEMoNg+EYThiF4EhmCRNDENoehCEoUiSEhyGUYx0CAeIJhAIByjUIgyCIIB3GkZIFgmKI9HmNo9GgZRpGcaB0kMOIgC+MIyiOBYHiaDAxDAaA2lGIoCiSV4KlkNBolCLJfhiYoLCAMQ4GgNZei6AQ",
        categories = "text,social",
        tags = "notebook,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,homework,research,lines,opened",
        contributors = "danielbayley"
    ))]
    NotepadText,
    #[cfg(feature = "nut_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0FYMgiD4PAvgOBYThaBoIgoNQgDEMB2DQYQ3C4MAwDQIIkiaKAwCCLYtDULgyDcNwgDYLg3DgNxjC4NIMC6Hw0C4OAwDKMg5g0MZAkULg1kgTIMCAMgyGwLg2DIMQtlaWI8DOWgzlYOZaDQOA4h6SwylqTgyiOJQximbounKLZCgwNQtkYOQzDmEoUhmGIEhqCYLDmHogmAMw0DOfYVoGgIFgeg5RgwYwtkqiZik0N5ekoOQwneSomneRqjmoNI3p6aahmSZwzDWXqklKlpuDiWg2DQNKzqeswwrWaaqiUMg4jiWZKDYOQuDkMa1DOHpnr2xYlqySg3DUN54iWzZps2QA5rmOA3jcOK5koNQ1m+1JikaWBtDGxg1iiMQ5DmO5DjqY7WmewpaDiypnuYNqzjmNahDOeQxDaZw2DizZKDG+cNrMMr0rwM8BtQNa+tmeAtl6Tccrmua1t8N4sroNqhwu2MPt2EYTo2Fw8GwaRuGUIB5DGCYRzcMs6zseM5CLP890LLoUzPNQ+gE",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NutOff,
    #[cfg(feature = "nut")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0FYMgiD4PAvgOBYThaBoIgoNQgDEMB2DQYQ3C4MAwDQIIkiaKAwCCLYtDULgyDcNwgDYLg3DgNxjC4NIMC6Hw0C4OAwDKMg5g0MZAkULg1kgTIMCAMgyGwLg2DIMQtlaWI8DOWgzlYOZaDQOA4h6SwylqTgyEGKgwDOKYlm+Lp0i0MQ5h4NB2C0NIShSGYYgSGoJguDQ0EOZookKHQ2g4IA4GOWgyDSYA5DeWg5neZw5DWaYNnCSgzlmJQ4kkLqWmmRwgl4MQgk2QpIqepZnDMNaNg2pg0p2Wg2DSNZKpwNAtkoMA5maRg1C2DZNpyp6cmcMQ1q2Sg0jOUpNlKtKjle05Dlew4lDWx5NsqrpXDmp40tCV5ntWcLItmSgysWapkmcNg4pgNo1p6Z5vkamaNkq+KgkANY1qC4Aypa4JvDa4A3uKyoll6aZeuSabCsK+Q0FqfoVoIPoBA",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,diet",
        contributors = "karsa-mistmere"
    ))]
    Nut,
    #[cfg(feature = "octagon_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYaAuDAMQiD4PAvgOBYWhmBoIgqDAgDgdg0hWF4chuBIdgmCw1C4M4gDIYYNg0MAgjUMYOC4NAxDQLg1DgNhsj0Ng4DgIJDkUQYzjaTI4DKDQ4C6RIiDaUgyDSMggjSTQtj6QI5jsNBsC2SJGmWWZbjcLQxjqPJeDYSJRlOaJMmqbJhl2P5BmSUpFnyU5KlqdYOoIMYti8MhWnKRZ0jWNwgm+a5tmKZZ/kmS6OoSi5GDIeolhiKQ+gE",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    OctagonAlert,
    #[cfg(feature = "octagon_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQ4DYIAxDYLg3DINhBDIIIZDAIIcDGGoRDULgzDEMhWDgLg2DgOBhhmG4dhEIINg8LQxC4NAxDQbA0imKwtjyKg4hiIIch4IIokGGhohMNgyDSLZEjCH42jiPIODaO49DgIJAiuQ4vkYMoZkiKx2kyTpQmCEQtjOEJUjkbI/lqXJammMIejWN45m0SJkiyLp3mubw0myVx6CIPg8C+A4FomjIGgiCpbiUaA4oiiqPD6AQ",
        categories = "transportation",
        tags = "stop,forbidden,subtract,remove,decrease,reduce,-,traffic,halt,restricted",
        contributors = "colebemis,jguddas"
    ))]
    OctagonMinus,
    #[cfg(feature = "octagon_pause")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDUVg5CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIZiWB4JE0MguDUOA2g4NguDcMg2EEMggjiDYNDGOYgC4MwxDIVg4C4Ng4DgYY4joII8CCLYvC0MQuDSHhsDSRpIC2WJHDiN4+juDggkWXY5GiMg2DINJKmCTZilOVZYi4NpXlkOAglySJfkyTgyjiZJIHaaJqmyfIOC2UIwnCVpbnaeJ2oWbo8lKVIeokSKAkmS6Soeiw0oicx6hOFYaD6AQ",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "mittalyashu,jguddas"
    ))]
    OctagonPause,
    #[cfg(feature = "octagon_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIA5C0NggDYIg+DwL4DgWFYYgaCAiE0MguDUOIRDENguDcMg2EEMggiwMAgi8MYtCCDAuDMMQyFYOAuDYOA4GGLIujCNAgiGIwtDELg0DENBsDSPI+C2T49DiK4zi+MQgjuVItGiJg2DINJAleQ4ykmS5PiINpOlAOAglOPpWkKWQyiyW4+HaX5hmOc40C2RokkqTBslKbZvm2fJDjGSKCmmIxIneP5Boqfpnkyf5qHqFIWhuGoEhyCRtDmDoShKm4Xp8PoBA",
        categories = "math,notifications",
        tags = "delete,stop,alert,warning,times,clear,math",
        contributors = "colebemis,ericfennis"
    ))]
    OctagonX,
    #[cfg(feature = "octagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQ4DYIAxDYLg3DINhBDIIIZDAIIcDGGoRDULgzDEMhWDgLg2DgOBhhmG4dhEIINg8LQxC4NAxDQbA0imKwtjyKg4hiIIch4IIokGGhohMNgyDSLZEjCH42jiPIODaO49DgIJAiuQ4vkYMoZkiKx2kyTpQmCEQtjOEJUjkbI/lqXJammMIejWN45m0SJkiyLp3mubw0myVx6CIPg8C+A4FD6AQ",
        categories = "shapes",
        tags = "stop,shape",
        contributors = "colebemis,jguddas"
    ))]
    Octagon,
    #[cfg(feature = "omega")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgwGgNAuDUYYShIIAwheGYSC2Eh2hwMg4DKFA1DKEgyhmGAwh8NA3hwNAzDcIA4jKGQxCCNg4C4NA5DSHAwDAMYmiaKIXisN46DMOB2C6IIihUNZEhiTxIDIMQiD4PAvgOBQ+gEA",
        categories = "math,development,text,science",
        tags = "greek,symbol,mathematics,education,physics,engineering,ohms,electrical resistance,angular frequency,dynamical systems,astronomy,constellations,philosophy",
        contributors = "karsa-mistmere"
    ))]
    Omega,
    #[cfg(feature = "option")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoDYbA2CAMQ4g8Ig+DwL4DgWGIbgaCIKDENINGgN4XhmHg+gE",
        categories = "development",
        tags = "keyboard,key,mac,alt,button",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Option,
    #[cfg(feature = "orbit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgzDQMQgDYLg0DgNBBDEMAghkIIahqEYZC4Mg2DYIAyDELg4DUIg+DwL4DgWLYwgaCIKDMLg2DUOYbDcLg1DENoYh+HodhuG43DcNImC4MQ1DKLIujOLRjGkchjGwZQgGMeYJDGT5aHiXZfHKCQzlAL5UlaWJTlWV5ZluCYrmCXQ5CIIJkCKT4tmibZrDyaZuneCZfnAIgxnWcwiiue6AmuAQ",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "origami")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMFYNBhDGDggDCFYUDELQxGgNguDIOQ3hKFIWhaEwuDYNYTDELg3DUORsC0NInDkNggjIMAyDUIg+DwL4DgWPI/gaCAiG2DAgDKGQ3C4NAxDQLZLk0NBBg2DYkhQNIODILgxDcMhWh2TQ1hILgwjiDplmeV4qiwMA3C2bQ3EwMoWnSaQ5juPZCkGBJDgmRpbkmWQzC4Mw4hMOJMk6DoyjQNoihOa5wDENg3mgMQ5i+GpcDYOKXDYM6QheJZwDcMIdh8MRIh0M4pqKkonjmcJJDkTKEjmDqJDir4XiaoIZisN6HGyMqUDWZQyDeeY+n0PoBA",
        categories = "animals,design",
        tags = "paper,bird",
        contributors = "gurtt"
    ))]
    Origami,
    #[cfg(feature = "package_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYNgiD4PAvgOBYThaBoIgoMQ2C4Nw2g4YYNg0MAgiYMQgDELg4ioLgxGwMguDIMwgDQLg0DcOYjCCJYnioIIzisOA5DEVgxjuJI/igLYkEgNY8j6TAyk0Vg5jgNwylGS5AkIMQtiwOQ0EwNY4DWNovEGSomigIA3jMNIOHqEoUhmGIEhqCRNDMLgwDWcZXDAMQzGgMZvkQM50hWeA+gEA",
        categories = "files,development",
        tags = "box,container,storage,sealed,packed,unopened,undelivered,archive,zip",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Package2,
    #[cfg(feature = "package_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIVoMCIPg8C+A4FhSF4GggIhtDENggDEN4OiQNAtDSE4VhqGYEhuCRNDIMYhDELoMDcVg4GGDYNDAII9DALQxkELg3DMbAtDeJ46iSP4+C2PJHiMNBBjuPpWj0MwgDgdo5lWTY9jKNA3DIORslKS48leJAuDAMAxGyNAzDILZEDUM4phaLYsgWB4vDMLpkCCI4MiEMhsDiRJCDWeIrgKLZ9h0NwuDUIA0oCI6IDkOYjDWNQ0DijJ6gE",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task,delivered",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[cfg(feature = "package_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIVoMCIPg8C+A4FhSF4GgiCgxDYIAxDcaA2hOFYahmBIbgkTQyDGIAzFYOBhg2DQwCCNgwC0MY6C4NwzGwLQ3C0NIzg6N5HjmNZADcIA0EGNJIkcMwgDgdoylCOJHi4MY9DIORskyRJYlGDQuDAMAxGyXA2DcNQtC4OQ1DWJYWimKIFgeKwzC6XggkyDIgDIbA4j2O5zhSdYYgKKZ5CIbQ3C4NZNnyTKEDkOZMDULgxDQOJ0ieAQA",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[cfg(feature = "package_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDkIg+DwL4DgWFYYgaCIKDENQuDEN4OC4MgxGEMQuDaI4pisIAwi8IAxjKKgzi8TImCANAuDUN4oC4OY2imQYxjCM4wDMLgzDYTA4C4OINDGOw3DmPw2DUNY0leWYwkYLYtDSN5CDILg0DOP5EkONpdjKLwtkmSx6hSFobhqBIcgkTQykYMx2kkOI+mQMA2iSg5FjKX4hjOKQ4DMbAtoSSQwDiaJCkCa4xDGiQ3DiL6PDabwupMQaCoSpaHjOYQxDaTw3FYMQznOF53naBYHnmOYMmWZ5qjSRJsl2oZLk2T42mQMpWqqKphsCiQ2msTI2juPaVr6mLACCcA2Gyugxp2rA4DaVpLlq5LYi2a5yhWs4ZgEA",
        categories = "files,development",
        tags = "box,container,storage,unpack,unarchive,unzip,opened,delivered",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIVoMCIPg8C+A4FhSF4GgiCgxDYIAxDcaA2hOFYahmBIbgmCw5iANB2iSFIWimKIFgeKwyDGIAwC4NQzDUVg4GGDYNDAIJGDALQxkoLg3DMbAtDcLQ0kODpHleSZFlANwgDQQZEliVwzCAOB2kKYJIleOgxk0Mg5GyXJUmiYYNC4MAwDEbJsDYNw1C0Lg5DUNYljOGICimN4KDMLpuCCXIMiAMhsDiTZLoOMonoeNocG0N49l2jJcpQOQ5lwNQuDENA4oSJ4BA",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIVoMCIPg8C+A4FhSF4GgiCgyDALgyDcIAxDiIIig+DgwhOFYahmBIbgkTQyDGI4fDQOQ4FYOBhg2DQwCCPwwC0MZDC4NwzGwLQ3C0NI8g6QJQkKPpJiINBBj2UZQDMIA4HaO5YkGUI0DGRgyDkbJVk6PpZg0LgwDAMRsC6OAtC4NQ1DmK4Wi+LoFgeMQziAOQgiKDIjDIbIlDeRA1nqLYCi+fwiG0N52CANImlycw5iINQuDENA4o6fA8GMaRyGMbBlCAcoJDKdgiCAYx5gkMQ2rCsh4rWJaNhQL6mqiqg+gE",
        categories = "files,development",
        tags = "find,product process,lens",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIVoMCIPg8C+A4FhSF4GggIhtDENguDUIAxDSIQgiINYThWGoZgSG4Jh6IIiDEOYmDULYphSFoui2BYHgkTQyDGIwwiEVg4GGDYNDAIJMDALQxlALg3DMbAtDcLQ0kmDpNl2T5LlYNwgDQQZKl6XQzCAOB2kiZpOl2QwxlMMg5GyYpam6Z4NC4MAwDEbAuDEMwtnwNw0iqO4YgKLo/goMwunQIJigyIwyGwOJTlGOYrjyi4+hwbQ3iaJQymKmA5DmYg1oENA4oiLIBA",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageX,
    #[cfg(feature = "package")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDELg3DMYQyg4IAwheGYVDAbA3C0NBBhWG4ZhiDwgDENhWDiFIWhiLgtDGMIRDMbAth4NIsiOL4bjUNwgiCIokhkMwgDgdorkGLoZg2EISHoIg+DwL4DgWUZUgaCIKDGIgyFaW5QlKV5WG8bB5GwaRuGUIBwG+aB0HOCQzC4Mg5CCPpbieIgwhGDQ3mCU5kmaaBllaBJYgkbQ3C4NY/nOPp1DULgxDWf5igE",
        categories = "files,development",
        tags = "box,container,storage,sealed,delivery,undelivered,unopened,packed,archive,zip,module",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley,jguddas,sezze"
    ))]
    Package,
    #[cfg(feature = "paint_bucket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA3CANggDIIg+DwL4DgWFYYgaCIKDEOAuDkOQyCAMQyEgMguDANAxhSFobhqBIcgkTQyDELgxDQNYliAMw4EEMwuDMNAgkGQwgDCSIlhKSQxDYLg1GGRpFkKSpNC2N45juNw4DgYwtlANw1C4NA2mAOA1luKgymeaYllAOQ1EGJIkkmdpMhIMhhnSVpKmyKQwDINRjDCYA1DiYAymiWI4DObQ1owNZxi6F4yjGBYHgkbYgjsNJQhKOA0oiKY5DgYY3DIMJqqmO53g2Nw3iuSBsDcLgyDmEa1rcNqnraqpvqyfYNk0LqxDQbAtrWkoksqI5SC4NgxjuQbRq2SgxC2Y4MnWyJBl2iLel2z7VlS5Kuki2Y4iYTJjDaD61DMM6UjCAQA",
        categories = "design,tools",
        tags = "fill,paint,bucket,color,colour",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[cfg(feature = "paint_roller")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAdxpGQdBogkMQ2gwaBlGkZxoHSCYVCAeYJgsIBygiCgiD4PAvgOBYoHAYYRCAZIJE0MQwCCFB2C0MhhDIII9jaNgxj6OhoDiPI+CCQJJkMMhWDeR4/ksMI6kSOonimLoRiiK4GiSE4WhiGocCKHoOhCEgiDSDIlDiDIgCKFJXiqBB0D6AQ",
        categories = "text,design,home,tools",
        tags = "brush,color,colour,decoration,diy",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PaintRoller,
    #[cfg(feature = "paintbrush_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYMgiD4PAvgOBYThaBoIgoMQ0g4dg0hKFIZhiBIagmCw3g4YQxCCLYNg2LYyHYORIDYVgziyLggjCOwxC0MR6iKFYmiWBYHigNouDKOovjyPJAi4dgxGEMoOk+PZWDIaJMjKWI+lIMguDmVZXjGT4eDAVgxDeTZfjKQJcmWVo9g0Mgtg+QJunWQJAkOJIBA",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PaintbrushVertical,
    #[cfg(feature = "paintbrush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg2DIMggDENwuDgOQ3C0MQwg4OAtDILg5DEMwiD4PAvgOBYliiBoICITQxDgLgzDcNggh+DwyGEMYSCAMI8jsMwuDAMIRkGQwyEyE4yjUOYODQMxhC4NZSj2VY+j4Lg3DANxsiANINDmXxhh8NI7mSO5XjyPpBDQMA4GwLZemCYpSlSaQxnGWg3j0TIxDMNQ0CCFAzDQOJRlMNZWmqeZbl2YQ0nGj5jC6ZY2pSaJVkClJuj2jpfnKh52oqWZbj0eokiaK4qgSLIJE0OQgDgY4ZhUMKBh8N54kGFwgmwNgtDaUg4DOvZeoaUpbsie5pDCcQzkSFQxDkbKDhGMQ4sOOpqooMQujCUwyrYQwxrif42hubaJDGNbrlkOYRuuEg1qiJ6sD6AQ",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[cfg(feature = "palette")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYQxCCEgwCCFYUC0MoXhUOYWh6EgxhwIA1iOHoXC2JA1GiGQuDINYRC4N4kDGMYkhWNwtjQNIOC4OBsC4MwuDSMIyhONYmhOOZCjwOB6CIPg8C+A4FlAYxpHIYxsGUIBmGkbBsgkYx1HIchlG4dBDG8bBvHIIggGMeYJDYLg1m4coJnSbhjHiCQxkGdZQC+VpYlqVZXlmW5dl+YZjmWZ5pmubQgncIp5m+fAiDEN6WnCfQwnmgaDoihqElunaZDKlqKmAIpimSZpomqbJ2nidaXnKoJRqKhQ8ruiZeqyrqOrGkZ6pgOKcnEIqbralK5oKh6FgE",
        categories = "text,design,photography",
        tags = "colors,colours,theme,scheme,paint,watercolor,watercolour,artist",
        contributors = "ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    Palette,
    #[cfg(feature = "panda")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgyDUIAxDeDg1GiDQ1EwMQyhEOB6CIPg8C+A4FiCI4GggIhtDGEIaCCGwyh+IYmiWBIngmCw4CANguDUYY8jwIAwkGQQtjyRQ1jGIo1jSBYHjcMgwC4Ng5CAOZSDcYQ0kCWoQkIMZEhMMA0C0NZADgLgzhCZ5pkOQgwC0M5okOXAgnSXpgC6YggmUNRDDILg0lSDIOi6EZ/DiOYbDGdKKlYNA3jqgKJoqLwyHMMZvn8NQzhGb4TDUY5vg0NgykUNA4nCaAtg0MwtloOAzkmM4CjWToKDakY9oAOZloGXZDl+Ro8rKS60k2KBtoKpYvsSJIBA",
        categories = "animals",
        tags = "animal,wildlife,bear,zoo,bamboo",
        contributors = "chessurisme,karsa-mistmere"
    ))]
    Panda,
    #[cfg(feature = "panel_bottom_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh3GkZB0GiDoQhIcoUCIMoSHmFQiD4PAvgOBYoHAYYbCAZIOE0MwgDENRoh6KAvi6G4ti8aIxg4bY3CAOAtjUM5IkiJ4pj0aA+gEA",
        categories = "layout,arrows",
        tags = "drawer,dock,hide,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0MQ0CAMQ1GgMQiigL4uhaLYvGiMYzDEOY3jmEY9j8aJBjCMgiE0M5IGiSopkyTpDlATZHjiOo8leQg+gE",
        categories = "layout",
        tags = "drawer,dock,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelBottomDashed,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0MwgDENRohkIooC+LoWi2LxojGCRtDmNgwCAMwtjWTI7imPhoD6AQ",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIBoGUaRnGgdIJDEOIMHKDgiDKDB3GkZB0GiGIaD4PAvgOBYoHAYYkCAZIJE0MwgDENRohkIooC+LokD6AQA",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0OQgDMdoZCKKAvi6Foti8aIxgkbQxDYIAxDULQzkqNpKjqKY9GgPoB",
        categories = "layout,arrows",
        tags = "primary,drawer,hide,chevron,<",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0OQgDENB2DEIooC+LoWi2LxojGM41DEOR2hGPI+GiQIwjIIo0CAM5IjuKZLk2QpPlGR46kqQQ+gEA",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelLeftDashed,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoHAYYWCAZIJE0OQgDMdoZieKYuhaLYvGiMYJG0MQ0CCNQzjYLZIguKAvjwaA+gEA",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal,chevron,right,>",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left_right_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDAVg5CIPg8C+A4FhSF4GgiCoMg4NR2C0MYThWGoZgSG4JguDQyDGIQyiSFooieBYHiqHg1FYM4xiaAoojaCg5g6EIShSMoYj6NYcE2QoMiGI5Gj2GpAkwIIti+PIzkmKZBCCOY7lGWhyGUYx0CAeIJjsIB5mkIggGgZRpGcaB0gkMQ4m4cpoCKMAgHcaRkgWdp4kaY5lD6AQA",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,vertical,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta,jguddas"
    ))]
    PanelLeftRightDashed,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByHiCQygyGYKgwdxpGQdBohSFg+DwL4DgWJxwGGIwgGSCRNDkIAzHaFQiicL4tiMPoBA",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEaBlGkZxoHSDYPhEch4g0MoRiEIoVD4PAvgOBYoHAYYLCAZINE0MQ1CAMx2h6KAvi6C4ti8aIxg0bQ4CAOY3jcLQzjcIo7j0aA+gEA",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide,chevron,>",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEaBlGkZxoHSDYPhEch4g0MoRiEIoVD4PAvgOBYoHAYYLCAZINE0MQ1CAMQ0HYMQiigL4uguLYvGiMYzjWNw5HaI49j8aJBjCMgijSNgzkmPIpkyTpDlCUggkiO5LkIPoBA",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelRightDashed,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMg+CoMGgZRpGcaB0gkMQ4gwdxpGQdBohyHg+DwL4DgWJxwGGIwgGSCRNDENQgDMdodCKJwvi2I4si4aIwgkbQxDAII0C0M5JjaSY6iiPRoD6AQ",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal,chevron,left,<",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByHiCQygwdxpGQdBohSFgghmCgiD4PAvgOBYoHAYYgCAZIJE0MQ1CAMx2hWJ4pi6IA+gE",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_bottom_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDUaAxCIPg8C+A4FhSF4GgiCoMCAOYRhOFYahmBIbgmCw5g6EAyiKFomiWBYHigMYqiCLYUi+GICiaM4KDOKxojiI4wjyMocE2QI3i6JJGieCoqg+IY5k2Go+E2NpTkSOxyGUYx0CAeYJDMIggHcaRkgWCQxDiZRyHiCYtCCcAimQIBoGUaRnGgdJrm2OZdl8PoB",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,horizontal,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta,jguddas"
    ))]
    PanelTopBottomDashed,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoHAYYLCAZINE0MwgDkaIaigL4uguLYvGiMYNG0OQgDENggDMLY1kuJ4pjwaA+gEA",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEhyhQIgyhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOE0MQ0CAORoDGJ4pi6I4ti8aIxjMMQ5jcaIeigL49GiP4wjIIhNDORpIjyQJNkGTxNkWOI6kmSw+gEA",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelTopDashed,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwch4gkMoMGgZRpGcaB0hOFQghiCgiD4PAvgOBYoHAYYRCAZIJE0MwgDkaIUieKYuhGLYvGiMYJG0MQ1CAMQ0C2NQzkmSY6C+PBoD6AQ",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAeYJgsIByggIgyCIPg8C+A4FikcBhhEIBkgkTQzCAORohSKIqi+EQ+gE",
        categories = "layout,design,development",
        tags = "drawer,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[cfg(feature = "panels_left_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAcoICIMoMHmCYLD4PAvgOBYoHAYYRCAZIJE0OQgDMdoUCKKAvi6EYti8aIxjONQxDUaAxiOO49GgPoBA",
        categories = "layout",
        tags = "drawers,sidebar,primary",
        contributors = "danielbayley"
    ))]
    PanelsLeftBottom,
    #[cfg(feature = "panels_right_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOE0MwgDENRoDGJooC+LoXi2LxojGM43CAMx2hqPI+GgPoBA",
        categories = "layout",
        tags = "drawers,sidebar,secondary",
        contributors = "danielbayley"
    ))]
    PanelsRightBottom,
    #[cfg(feature = "panels_top_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0MwgDkaIZCKKAvi6Foti8aIxjMOQgDIMRWDmOopj0aA+gE",
        categories = "layout,design,development",
        tags = "menu bar,sidebar,primary,drawers,window,webpage,projects,overview",
        contributors = "colebemis,ericfennis"
    ))]
    PanelsTopLeft,
    #[cfg(feature = "paperclip")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIA2C0OAuDQMQ0CCEQ1DgNhhDIIIcDAIIfh8MguDgMg5h2JImGyEYTDSEAuhiGoVhWHwxiALQ1C4Ng1DeOI6jwbIvDMN4nhcNQxGGDYNjWIIWhIOA1k4NJQisLpDDmLw1kcIg+DwL4DgUPoBA",
        categories = "text,design,files,mail",
        tags = "attachment,file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Paperclip,
    #[cfg(feature = "parentheses")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxHMLQ0C0M4RC0OQghKF4ZCIPg8C+A4Fh2IIGgiCgxDYIAzHMNIphgIA5hGL4xDmHIeiMPoBA",
        categories = "development,files,math",
        tags = "code,token,parenthesis,parens,brackets,parameters,arguments,args,input,call,math,formula,function,(,)",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Parentheses,
    #[cfg(feature = "parking_meter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDUaAyCIPg8C+A4FhSF4GgiCgxDKDgyHYM4ThWGoZgSG4JguHwxDmIokhaKIngWB4qg8LgyDiLA5GGDYNDAIJAkALg5DQOAtC4Ng4GwMguDMNwtDaRA4DgYQ3CCV5Aj8LQxDOOJBkyTpXlIOZUj2DpBmmQ5FDiSQ4HqMImgKKI1goOQgjwMwgnqWpoDaQZxjKAQ",
        categories = "transportation,navigation",
        tags = "driving,car park,pay,sidewalk,pavement",
        contributors = "danielbayley,jguddas"
    ))]
    ParkingMeter,
    #[cfg(feature = "party_popper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4OAgDEMQuDMIAyhQMhsDEMAuDcLQzhsOQiD4PAvgOBYiiWBoIgoNAgDMaAuDAMYhiOKIngSKYJE0MoVDiL4xjOJI3jaBYHjkMQ1hSPoyiKQYmgKN5FgqO4UDCSpAjWT5EiobZTDILQyC4Mg0hsNRhmAOYUC6aAwCCbAwC2EQ5DaLQuDEMhjnWDQ2C0Lg1DeDwuDYM5wC4NJIhGghonwMw4GOfA4nOb6IoGhA3nOEQ0DQTAxiyGZXkKWY4CKXIVDGg4Nl6Egzo6eqLDShA4mCXpxg6EYQqyEIbnwN6lmGs6FhOEY7EgMQ3p+ToolEbYQhQIKqqieAzmOkJ8r2saEsytBDDmfYVmOaJogwNYVmgNphDMVrGkyWLJioTbZqucbBmqE5gDiE5jsWzQ1o+Hr3h2MIcwELb8r8OaDvKX4NoO+Ycl6SMLv2dAwDfAQgkgMhascaA+gEA",
        categories = "emoji",
        tags = "emoji,congratulations,celebration,party,tada,🎉,🎊,excitement,exciting,excites,confetti",
        contributors = "karsa-mistmere"
    ))]
    PartyPopper,
    #[cfg(feature = "pause")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkNYMGgZRpGcaB0gkMQ4gweIcDSDByiAIgxCIPg8C+A4FimLIGgiCojiWJwgiWFINg+EYThWF4ZhuJoeimK4EHQPoB",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDEMYOGMeYLDQIg+DwL4DgWB4bh6BoIgqDIVhcIg4hWEQiDGKobh2BIjiGMoHg+KAxDaK4LDIMIOiWDYwiKIA8HAYR0GgIBkgsTQ5CAMQwGENQglMMAglYMZUlQdgzC4NRhl2U5hleZAxC0NguDgNJPC4MA0DUUZoDUMpPDcLg0DgIA0ncNpPmiahBmOY5WliVJek8MBahqHJGkgPoB",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHmDQyhEeINDWERyhcIoVCAaBlGkZxoHSFAwCIPg8C+A4FikcBhgsIBkg0TQxDUIIPGgLgwDGKIqi+C4ujAaIyjQOQgDYaA2j4L5AGiQoxjMIhNkcMQwkqTJOD6AQ",
        categories = "devices,gaming",
        tags = "computer,chassis",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PcCase,
    #[cfg(feature = "pen_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDEaA4CIPg8C+A4FhSF4GgiCoPC4MQ3DQIA2C4OAxDIYQxCCKgwCCLQwC0MwuDkOA2jGMw4DcTIyDgNAyiuJIgDQYY/j+L4uC0Lg1iUMxsC0MQuDODwgDSUQ1iiSpKi6W4tC4NgyjKXwyGyVQzDUM5PlGKJFlyW5MkkNA5DceoThWGg+gEA",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    PenLine,
    #[cfg(feature = "pen_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIIMC0NguDENQ3CCEQxDYMhhDIIIcg2HwtC4NQuDgMwzGwLQxC4MwyhwNIrDYYYiiIIIfjUIAuhkMo5DINBsi8Mw1DiKYrDIM4bh2N42iQM4hDUTAxDSDgzC4OQ4DIIg+DwL4DgWWpdgaCIKDGO5XDkIA3hIN4uisNQ5C2QA0jEMYOjedJ0lSVg2CCeQ4DaKJADUMwgoEM5ZluYJfgSYYJG2HKPg0MgwoeXKLD6AQ",
        categories = "text,design,tools",
        tags = "disabled,inactive,non-editable,locked,read-only,unmodifiable,frozen,restricted,pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PenOff,
    #[cfg(feature = "pen_tool")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg3DANwgDIMQuDIOQzGEMQghoMAgh0MQthQNAxDSHhsiELg1DgNooioNoZhuHoyhyKIjDQbINi4LY5iuMIcjOGwujaJoUi6QYuj6MofjOQokHoIg+DwL4DgWUZUgaCAiG0MQ4hsM4oDMNw1C0NguDgNw0kmHZrC2Dg0iyDg3DYTAzhUMw1hILgwDIOJqkqKAyhCQaBDcTINneG4NmcOZ+msIJxmWaJzDEM4bDiUJSleVoEliCRtDILqVqClQ3hWKwgqWfA2piU6clEYxpHIYxsGUIBygkMgiCAYx4gkMQxroYx5r6wJRC+sKyrQPoB",
        categories = "text,design,cursors",
        tags = "vector,drawing,path",
        contributors = "ashygee,mittalyashu,ericfennis,jguddas"
    ))]
    PenTool,
    #[cfg(feature = "pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELgxDcNAgDYLg4DEMhhDEIIZDAIIcDALQzC4OQ4DaIIiDgNxMiEOA0DKGoTg8NBhi6Loeh0LQuDWFAzGwLYNDODAgDQLgzDWF45jmHZKhwLg2DKIZODIbJDkUM4+kSF40kuSo7jgNA5DcegiD4PAvgOBQ+gEA",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pen,
    #[cfg(feature = "pencil_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDEaA4CIPg8C+A4FhSF4GggIhtDENQgiANAgDSE4VhqGYEhuCRNg8LgxDeIw2C4OAxDIYQxCCOQwCCPAwC0MwuDkOA2kCQg4DcTJBDgNAyjqM4wDQYZOk6Po9C0Lg1jQMxsC0MQuDOD4kmANY3lmWY9mmPAuDYMpBm0MhsDSZAzl6YI3lSappluWA0DkNx6iaFoqD6AQ",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    PencilLine,
    #[cfg(feature = "pencil_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIIMC0NguDENQ3CCEQxDYMhhDIIIcg2HwtC4NQuDgMwzGwLQxC4MwyhwNIrDYYYiiIIIfjUIAuhkMo5DINBsi8Mw1DiKYrDIM4bh2N42iQM4hDUTAxDSDgzC4OQ4DIIg+DwL4DgWWpdgaCIKDGO5XDkIA3hIN4uisNQ5C2QA0jEMYOjedJ0lSVg2CCeQ4DaKJADUMwgoEM5ZluYJfgSYYJgsNQgo+Ug0oeXKLoqBYHo2HKbg0MgwpSiYBA",
        categories = "design,cursors,tools,text",
        tags = "disabled,inactive,non-editable,locked,read-only,unmodifiable,frozen,restricted,rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley,karsa-mistmere"
    ))]
    PencilOff,
    #[cfg(feature = "pencil_ruler")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA3CAOAug8MoSGGFA0DEIIXhkMAgh0MAtDMLg0h4TIUg8NQuDOFojhmG4ejCHwgiINBMg+DAiD4PAvgOBY6j2BoICIbQ4CANoaC0Mo5juQI/gSQYJG0MZFDGRwykmS48k+ToFgeUQxjeGQ0ioIJjDMYwuDmY5qmmJIXkeHY0GySQuladRjC2bZtnQNA2nuNIlDGGZglmTYCk+XoKDIMQumCJJ+DgMQyGGg4xh6IZpDgNqYDkOA3EyIg4DQMgglWjQ3DSFoapaIAuikOAznOjAzouZYqDWk6uq6rAgnUMoiDYMgyGyZg1DMLazpOpKkjKHQurCeQ0DkNx6oWW6Hl2QpSDUILciQNLWj6AQ",
        categories = "tools,design,layout,text",
        tags = "edit,create,draw,sketch,draft,writer,writing,stationery,artist,measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PencilRuler,
    #[cfg(feature = "pencil")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELgxDcNAgDYLg4DEMhhDEIIZDAIIcDALQzC4OQ4DaIIiDgNxMiEOA0DKGoTg8NBhi6Loeh0LQuDWFAzGwLYNDODAgDQLgzDWF45jmHZKhwLg2DKIZODIbJDkUM4+kSF40kuSo7jgNA5DcegiD4PAvgOBZkmeBoICIbQxDUIJwhENJjmWag+gEA",
        categories = "design,cursors,tools,text",
        tags = "rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Pencil,
    #[cfg(feature = "pentagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg4DMIAyC4Mw4GEMoRCAMIZCAMYRhMNIZGwOAgDULg3DSFoYhqGodiaEISDINRsC0MwuDCIA5C4Mg2imF4rhwLQxC4OYchMNxIDeNoohePobDGQZDlAMw3EyEodgyRo9huLAgi4LYwDUegiD4PAvgOBQ+gEA",
        categories = "shapes",
        tags = "shape",
        contributors = "danielbayley,jguddas"
    ))]
    Pentagon,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDkIggHiCQiDWDh4DGCoMg4eYWhIIg+DwL4DgWHhjGkchjGyBhjHiCg2C6EwgGMeYsi6DhygoMo0h4L4kiaKIjiWJ4piuCw3jSMIykSRo2CKOITjqPJBD6AQ",
        categories = "math,development,finance,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwNYQHKDAxCIPg8C+A4FgeGhwGEdBoCAZIMG0OQgDIMAgDMLQ2i0IA2hmG4hiOIIiiSJgiG2MA4jKKoyC2D4aC+NhojiI4lgwTYOCAMQwHYNI0kaOQ+gE",
        categories = "accessibility,people",
        tags = "people,human,accessibility,stick figure",
        contributors = "mittalyashu,ericfennis"
    ))]
    PersonStanding,
    #[cfg(feature = "philippine_peso")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDESA0CIPg8C+A4FhSF4GgiCoMCAN4RhOFYahmBIbgkTQ3CAMgxFYNBhDGDggg2DYxDELQxGiL42jOPYxjUMhIDeIoWiYPoB",
        categories = "finance",
        tags = "currency,peso,money,php",
        contributors = "jguddas,kasutu,karsa-mistmere"
    ))]
    PhilippinePeso,
    #[cfg(feature = "phone_call")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyGEOQghEMAghQMYShIIg+DwL4DgWG4egaCIKgwIA2GENQgimFIWiqKoahyIYggSIoJgsMwuDgMwyCAMQ2C4NQ2DgYYXheLIVj0LgygwLQuDMMAzGyTg1DWTQ0DYNRBjyPJHkUN49DUaAzGGW5Ii2Wx2mOZZdC2WxBDEOI9nGXYOCANJknWdAym2Yp4lySIXmiap5oCTZxDELg2GyVpClMMZEj2ZoVk0Mg5jyiAyDODQxDSPadkeFI/DOlomk4OA0jCHY0D6AQA",
        categories = "connectivity,devices,communication",
        tags = "ring",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    PhoneCall,
    #[cfg(feature = "phone_forwarded")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA2GgOAiD4PAvgOBYThaBoICIbQxDgIAyCCDQ0C2I4ShSGYYgSGoJgsMwuDgM4hDENguDUNg4GEMQgjsMAgj6PgxC4MgxDMLQuDMMAzGySA1DWRw0DYNRBiGIZAjyPA3jwNRoDMYZVj+YY7lUdpemCVwxC2VRBh6PIfmiIIil+cZwDKapdnOVpinEMplnmYZBkeH5CDYbJQjiTQxjqWJXDCRwyDmM5DDMM48g2DKAmGNQzpGDpIDgNInhWKw+gEA",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneForwarded,
    #[cfg(feature = "phone_incoming")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyHYNhoDYIg+DwL4DgWFYYgaCAiG0Mgyg4LYNhOFYXgQaIaiiB4JgsMwuDgM4hgwLg1DYOBhDEII6DAII9j0MQuDIMQzC0LgzDAMxskcNQ1kYNA2DUQYhiGP47jsN47DUaAzGGVI+mCOpUHaXZflYMQtlQQQxDiO5tmeDggDSXpxnAMpplydJVmGcYPmWdZhkabZBDYbJPjeTAxjmV5WDCRgyDmM5CDMM47DSlpglYNpHpEIKbDMOA0hSFobD6AQ",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAyCCDQ2CIPg8C+A4FhSF4GgiCgyg8MgthGE4VhqGYEhuCRNDEMwuDgM4PgwLg1DYOBhDEII3DAII6joMQuDKKwtC4MwwDMbJDDUNZCDQNg1EGH47lGNwxDeOA1GgMxhlCPI4g6Dh2lmW5SC2HxBDEOI4miXI3g8NJal6a4OmSWJvg+cYfmCdZRj2Qpoj4NhskuNJIDGNpdlwMJCDIOYwj8MwzjgNKRnuUQ2kOjIQkMOA0iOFonD6AQA",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMQzC4ORhDENIQhcMAghqGoSDcMwyCAMguDYNg4hCKIchuEAuDKEQtC4MwwDMbIxDUNYwDQNg1EGIYhiqDwxDeEA1GgMxhj6K4ag+Ph2keSZAC2SQxieVJKhALQxiMNwyDgLQ1i2XAiD4PAvgOBZkmeBoIgoMpJj4MpjmWappgSa4JE0NAuDcNoQhINQ4DIQZWlaQIiCANJIoehgylKRqKj+K5MiKTqQlcMYwlWJBsjmJo2DGFYpkqMAyDmIQxi0MwzhirIqhqLQ0noNIfnKZp2D6AQA",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIA4CANgtDYIg+DwL4DgWFYYgaCAiE0Mgyg4VgyGiEoUhaG4agSHIJE0MQzC4OAziGDAuDUNg4GEMQgjsMAgj6PgxC4MovC0LgzDAMxskcNQ1kYNA2DUQYhiGQI8jwN48DUaAzGGVI/mCO5UHaXZflYMQtlQQQxg+bJgkEIIhDSXpxm+VwymmXJ0lWYZ1DKZJ7naaIxjwLg2GyT44kwMY6leVgwkYMg5jSQwzDOPA0pido+DaR6ThCRw4DSJ4XisPoB",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneOutgoing,
    #[cfg(feature = "phone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg4DMMggDENguDUNg4GEMYSCAMIch4MQuDKDAtC4MwwDMbIlDUNYkDQNg1EGEYRh2HYaDEN4SDUaAzGGMoejUIIyHaPI+jSEgtjIQQxDiEpMkaGoRDSPZBj+GwykiO5TjOH5UDKQ5alUMYkkyIA2GyLYXioMYZhuRgwiQMg5hGIAyDMM4SDSeJVh2FAznIIJ9DgNAiD4PAvgOBQ+gE",
        categories = "text,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Phone,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig0CIIB4DKCg5g0eIJCKEoHhAIgyDAIg+DwL4DgWHhwGEdBoCAZIKE0NAgDcYwwC0MQuDcIIyDMLQzCCNwzGgMQzh2H4kiaI4lieKQiE0MQ4CCGxjjGMwgjCN42jiOBWgyHgvkIaA+gEA",
        categories = "development,math",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "piano")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg1CAOBjC0MQuDQIAwC0MguDYLQuDgLQzC4MoZEENoeDcIImDiKAwheLgyCAOR2DEMRhjCMItjkII3GgMQ2jaO4ujqIwyHYLYNDUQwyjAOYbjsMIVhAIIMg6EAiD4PAvgOBZYluBoIgqMAxDQaAyDCV5Zl6XYEl+CRNDaUw0HYNJolqbJrgWB5uDGLZjnOdZqgKbJ6gqY5xn+WJ2lygp5mCCw4oedKJmqAQA",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,chord,keys,octave,acoustic,instrument,play,pianist,performance,concert",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Piano,
    #[cfg(feature = "pickaxe")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDMLQ4C4Mw4DEIIShQYYWhYMAgh0MQtDMLgwDCIAzEwMYWDkLg5i0Ig+DwL4DgWMIzgaCAiE0MQ1iwNwzCANIjDINxBg+Do/h2SQgjwOQwDIIAyhOPhjC0MYTDmGA0DKVYjDmT5RjuEpDDeUAuDaJYaisOYOmqHpuhaPA4DKT4ik4NIvjGNo1gSN4JjoNojiWDpWi0OZpiybKIkqFp1DKDZxo4YwuDSlIOhMNprlEMg2pYNg3pyUZniKQw4kWP5GkqHQyh2EgwpieIynye4FgefgxhgNZ0hOuYaC6qw1pav5ukmXA3DCDQwGwLY8DIOacsyzq9sKVrCqmbwusYNBspqzpls0NrSDCwLUuKw7XtmHhstANrLr60bkuOvrltYMLFsceqwnqAQ",
        categories = "tools,gaming",
        tags = "mining,mine,land worker,extraction,labor,construction,progress,advancement,crafting,building,creation",
        contributors = "karsa-mistmere"
    ))]
    Pickaxe,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdgxDAY4WDELorDmIIkGgNAiD4PAvgOBY0HIZRjHQIB5gkMQzCIIB3GkZIFkAMJDHiQAykMaBlGkZxoHSCQ3kMcpMCKTo0C+Oo8D6AQ",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQwGgNhWDQIg+DwL4DgWFYYgaCAiG2DA0CANoihSFobhqBIcgmCwxg0MBWDcYYMgwMAgjUMAtDKORoC0N4lheKYogWB4rDODQ0HYMoyCCNI2k6MxoDOP4nDwchlGMdAgHIeIJDEIggHmXYTCCXAiDEMpfGgZRpGcaB0gmPggHcaRkgWXQwj+VpYD6AQA",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis,jguddas,karsa-mistmere"
    ))]
    PictureInPicture,
    #[cfg(feature = "piggy_bank")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDcaAzHYMhhg2DQwCCGIYhYaIUhaGYghsLQxHYLQzGEMwuDENggimK4hiAMgtDIaAxhWDowiKJIzjeF4hiOIxoiMYQ1CCRYahmMwtDQVonDQIJPkgMImC4MoOC4NhsC0LopDQSIMGGLIslILZiHaNpFkeMJWDQdonh+Uo4DGHY9jmDojHoIg+DwL4DgWe5+gaCIKi8MQwGgLgwDGep8oGgIEoKCRNlYOJnGGVpWnGmI1oyfaQD6AQA",
        categories = "finance",
        tags = "money,savings",
        contributors = "ericfennis,jamiemlaw"
    ))]
    PiggyBank,
    #[cfg(feature = "pilcrow_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAzHYMQxCIPg8C+A4FhSF4GgiCoMCAORoC0MxhDODggDCJwgDGJwtDYaA5hOFYahmBIbgmCw4g6EIShSFo1jSBYHjcMgyioOBIDIbA0C0NIxj6GICjWQgiG0NggkSTJMk6M4BA",
        categories = "text",
        tags = "direction,paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "zaaakher,danielbayley,jonas-hoebenreich"
    ))]
    PilcrowLeft,
    #[cfg(feature = "pilcrow_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzHYMQxCIPg8C+A4FhSF4GgiCoMCAORIDcYQxCCJINg2JgtDYaA4hOFYahmBIbgmCw0g6EIShSFoyjGBYHgkbQxDiJY2kUSAyi6O4YgKMo/CIbQyDKJQ4C2RZJjCAQA",
        categories = "text",
        tags = "direction,paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "zaaakher,danielbayley,jonas-hoebenreich"
    ))]
    PilcrowRight,
    #[cfg(feature = "pilcrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA0HYMQ2CIPg8C+A4FhSF4GgiCgxDeDoQhKFIWgQaIZiWB4JgsOYOEgOQuDUYQ0jCDo0DAII3jkIA5EiDIThWGg+gE",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Pilcrow,
    #[cfg(feature = "pill_bottle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDEaAtDQYQxg4IAwheFwthUMR2DWFIWhiIoWhANAiD4PAvgOBYoiuBoIgoNggDcdgxDMYQyCCOYjhiOQyGgOI4jqGYjDILQyFYN4nimLooHIZRjHQIB3GkZIFgkMQ2CIIB5gkMpbGgZRpGcaB0gkNZbHiCYmCAcpqCIMZLC+T5RD6AQ",
        categories = "medical",
        tags = "medicine,medication,prescription,drug,supplement,vitamin,capsule,jar,container,healthcare,pharmaceutical,tablet",
        contributors = "karsa-mistmere"
    ))]
    PillBottle,
    #[cfg(feature = "pill")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg1CAMoNg+DAtgwYQ0C4OYPhiGggDAIAxh4LQ3iMbIVh+FochuGYPiiHggDeMBaCIPg8C+A4FjWOIGgiCg4g4II/g+MQ3jSNo7D6AQ",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,tablet,pharmacy",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Pill,
    #[cfg(feature = "pin_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg1CIPg8C+A4FhSF4GgiCgxDUIA5C4Mw0FYNxhDGDggDCKopDELYog2DYrjOKgtDQSA3C4OA5hOFYahmBIbgkbYxCAMorkePYWkGQIFgeCRNDmIB2DELg3DYYZFjSLpVDGKJVDcORsi+Vg4C4ORBlqLIrh+HguDKJAxliX5riyXxol6So/gE",
        categories = "navigation",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg1CIPg8C+A4FhSF4GgiCg5g4MAuDcNhhg2DQwCCJwxC0MQuDEMYOiEORsiuIQ4C4ORBiWKI7icNYODULgyDQVgxiOL4vieSYOg4aIMGGR48juKgxHYLYhiOOpKDCNIujQN4yl4OJWjiWZSj+H5XFYN5PkuSpHisIJlloLQ0EgOIknGUZKDSS5ImaRx6hOFYaD6AQ",
        categories = "navigation,account",
        tags = "save,map,lock,fix",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pin,
    #[cfg(feature = "pipette")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIA5C0OAuDQMQ0CCEYTDQQYNg0MAgh2HQzCAMYRDgMg4HYMQuDMNA0GGG4ejAMQtC4NQ4DaIoShSGgghyMQgDMLolDiPAxGiKYri2L4fjGOQ0jONQ2EwMQ1iIMgiD4PAvgOBZYluBoIgqI4OCCEoSGEMYij6MohDMbAtkAOJvkGZ5ph2aJoDObxsmWFZAk6fp0midppmwepXlmXpdgSX4JG2G4Nk2M4YoeWqLD6AQ",
        categories = "text,design,science",
        tags = "eye dropper,color picker,lab,chemistry",
        contributors = "Andreto,ericfennis,karsa-mistmere,jguddas"
    ))]
    Pipette,
    #[cfg(feature = "pizza")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDQLQxg4Ig+DwL4DgWFYYgaCIKDEMwuDcNYODgLgyDWEYmiMMQuDQMoUhaG4agSHIJE0MQ3iGIggDULg2DUNBhDGPQ2DiDpEkYMAgkoMIRDILoMhKDJQi+FYXjSM4FgeNgxiWRg5C4M5Cg6S5lkqT4SjkN4wleGYCjSWwiE0MosDkNg0CAMgwiEMwymOEpMg6KQyn2Domn0bIRDgLQ1n+ZqCj4OYoiyhAyEEMZgkWDqZkmZZSj0N6Fk8MAzDejqBlKh5PDakh6myMoBA",
        categories = "food-beverage",
        tags = "pie,quiche,food",
        contributors = "karsa-mistmere,ericfennis,jguddas,jamiemlaw"
    ))]
    Pizza,
    #[cfg(feature = "plane_landing")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyGgMgwCIPg8C+A4FhSF4GgiCgzC4Nw3CAMQwh+IYMDkbAyC0NAuDWIguDGLQ1GOMguDIOAuDkLg4iwOYvDQNRzC4M4ujEMQ3jmPw1EwOAgjgNRsDMLQ2i8MA1i0MxhgyDAwCCXQxlWPoxDUMhsh+DJXDSWoNl6bZfC4MJii2ZYsDSDY2jQNAyjaew3k+RZwjEOQ2maVIxDAM55joOA4C2cKGjmjaHlSewxGwLZGjgMYzo4NJIDANqODkNaODCKoxDMN6ODINBMiwMpikYNZrlybQxo6ZKODMOBahOFYaD6AQ",
        categories = "transportation,travel",
        tags = "arrival,plane,trip,airplane,landing",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneLanding,
    #[cfg(feature = "plane_takeoff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyGgMgwCIPg8C+A4FhSF4GgiCg2C4Mw2CAMQ3C4NAgiaIxsC0MgtigLgxC0Lg1DUYYMgwMAgjgMYiC4OI5GyL4kDGNYNjmRo6j2ORMj4MYMDUIA2kAOYxDSNI2keIoNC4MA5C4MhsDSW4MDORI3lgMovl6YIvlOaAwDaNYljuaA0juOI6jwNwzjGIxMDKOw3GEMYljyJp3lmPQ3jwOQ5iqHw4C4Nw2GOMQyDOJQ2jGHg4DQLaDDCiqfDgTIkDWTIkDKZZYjCg4Oi8OBahOFYaD6AQ",
        categories = "transportation,travel",
        tags = "departure,plane,trip,airplane,takeoff",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneTakeoff,
    #[cfg(feature = "plane")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg4CAMQ5C4MoQDaEAxGwMwuDULYaDUQwyDEIIWiGGwgDQIIhCAMxjC0MQthuHQgDALQ0iYMYbEwMQzCCD42g8NoTi2MQui8LoSi+OI4DUbIwh6QwykSIoiC6PI4DMTA5hAMpNhQMxIDSTZTiuKZllaLh2h2GQtl6Jg1lUY5VC6P43nIMxskQMpxDSTguDaMINhyOAyHoIg+DwL4DgUPoBA",
        categories = "transportation,travel,navigation",
        tags = "plane,trip,airplane",
        contributors = "ahtohbi4,csandman,ericfennis"
    ))]
    Plane,
    #[cfg(feature = "play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANRhDIIIQDAIITDEIAzC4MAwDgLQxC4NwyDgbAxh4OQ5DcIA2C6Jg4g+EYUjCFoZDAM4XC4NA1iKHYQDcQYQhKMYNCAMQ5HoIg+DwL4DgUPoBA",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Play,
    #[cfg(feature = "plug_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMh2DYIg+DwL4DgWEoVgaCIKDENYNg+EYThiF4EhmCRNDEMggDENx2DWIIUiSI4FgeJodDgaAxDSL4igKJI0goNoqDEVo3igdgzGGQZBDCKoqC2KAgDAWo7jGAQ",
        categories = "devices,development",
        tags = "electricity,energy,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[cfg(feature = "plug_zap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4MwgDIMINGEMguDSD4VCAMIZhsM4YDATAxDIIAxDgbAtDaJwthSDorhOGIUhaGoyhyFRaCIPg8C+A4FjiO4GggIhtiIMoiDMLQzjeOY+j2BI/gkTQ3C4NYjh2UwxhoMQxkmOpNkyBYHk+V5SiODJWg4MQ0luS4Ck2YJBiQIJGhYNBoDaJpzmqXYBA",
        categories = "devices",
        tags = "electricity,energy,electronics,charge,charging,battery,connect",
        contributors = "mittalyashu,ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    PlugZap,
    #[cfg(feature = "plug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDUIg+DwL4DgWFYYgaCIKDENQgDgVgyhSFobhqBIcgmCw3iEYQxCCMAwCCM4wjYdg0GENAgjuM41C2Ow0GiQI6jyNJHDGQJAFYOYvjGR41jELQxHqJYXimKIFgeKw5iGI5WieAQ",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet,power,voltage,current,charger",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Plug,
    #[cfg(feature = "plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FhSF4GgiCoOCANR2hGE4VhoPoBA",
        categories = "math,tools,development,text,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis"
    ))]
    Plus,
    #[cfg(feature = "pocket_knife")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DEYwwCAMYNhOFQyFODA2CCDA3HMMoWiAMgtiAMYkheIYNCIPg8C+A4FiyL4GgiCgxDgIA2GgLgwDGK4tjKMYEjOCRNhuNo6jyPoukKQYFgeRAyDALg4gwOJTDMYQ0CCWoSl0LQ1C4Ng2l+YQ2GwLQxiUMpZlsIIShSEpgmIIJyDYWpKkCApCk+NY3DEMZlFYMprlqXJuocNAtDQVg2niTIB",
        categories = "tools",
        tags = "swiss army knife,penknife,multi-tool,multitask,blade,cutter,gadget,corkscrew",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PocketKnife,
    #[cfg(feature = "podcast")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZhpGwbA9CIYx1HIchlG4dBDG8bBvHIIggGSDBNDEMwgDENxhDGIQgDCJgwC0MonGwLg1CANIvGEMIvieNoojmKR6CIPg8C+A4Fj6QYGhwIoeDYLg4jAMQ4i8OBhDkIJSiiJYqDkLg3iePY/kSQ4EkWHQ4iENBhjCMJViaYwwlyQJgj4YxpHIYxsGUIBjHiDAxDKGhynqGhjHmegxhqCIKgyDoQhKFIWhibZxnOdQ+gE",
        categories = "multimedia,social",
        tags = "audio,music,mic,talk,voice,subscribe,subscription,stream",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere,jguddas"
    ))]
    Podcast,
    #[cfg(feature = "pointer_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA0C4NRWDQYQyCCFYNhgLQyC4NAxC0MQuDkNQ3CIPg8C+A4FiaKYGgiCgxDOIQgDiHIUhYIIYjiHwuDINo7DKIolieLIrgSLYJE0MogDcIAxDaPBBDiM44lSDQyhUMQ0HYLQzjaF5NjoNI4lsMZelWOoxDYM47DENQ4kKKJGkWBYHgkbZMm2Ow4nqZo5DCGguDcOYWC4OA2EwNpNDmgRhDegZio4N5in4IKJhsMxoDIYZSlKlA1C4NojoAMw0DOcJEgKRp1gqiQ2Hab4mnGKqpnSLhthWuJWDCp5ygEA",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis,jguddas"
    ))]
    PointerOff,
    #[cfg(feature = "pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDQYQ4CCEgwCCFQxC2Eg4CIPg8C+A4Fh2IIGgiCgxhIMQxHYLQxGGDYNhWMQtDKM4uCCMIWhaM43hyHojiKBIkgkTYPg4MBWDmNo4jKNAykqOZMjcdgxj2H5BkCBYHkMMYVDkLg1FaEIvlCOpNk+MZllKXJVj+ApBlqJooi2Y4Xg4IA0hYdgzhGE5khiGhojMY4zC6FAtDSXwtoUNgtDULg5DmhAzDQbAtDMLqMpcNpnjkMY3oUM6EDgMhMDeDg1myV4BA",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis"
    ))]
    Pointer,
    #[cfg(feature = "popcorn")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4GEMgghEMAghSFgtDSEoahaFYYhWGoTh8MIehSEYhheGYlhuIofDQIg+DwL4DgWMIzgaCIKDGJYRDmDovjGNo1gSN4JG0MYZDKEQxC2R4/jKQ5CgWB4JE0MoUDgYwuDWHwuDkLg0C6DQxGwLQyC4NggDEMhjC0LgxlqbQ3mmTAukoSA3myZ4dm+S5fnQMpMEwM51CAOZ5n0NguDOTAgmGTBak6QYBA",
        categories = "food-beverage,multimedia",
        tags = "cinema,movies,films,salted,sweet,sugar,candy,snack",
        contributors = "danielbayley"
    ))]
    Popcorn,
    #[cfg(feature = "popsicle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg2CAMQ0C4NBjC4OAthaFgtDIIAwhuFhsC2DQxiILgxGGEg5DUIIpisMIQh0LQ3iYIIzDEbIjCCI4VDkLg3CAMomj6QAuDmGAxFoIg+DwL4DgWS5OgaCAiG0MoclYLQ1C4NZZluSpMlEPoBA",
        categories = "food-beverage",
        tags = "ice lolly,ice cream,sweet,food",
        contributors = "danielbayley"
    ))]
    Popsicle,
    #[cfg(feature = "pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA3GMMAtDULgzhULQ4hKFIWg0MAiD4PAvgOBYfiKBoIgoMQwg4dgxDSHogiWJIEiaCRNDYIAyDEaAxDKL4hjOMoFgeNY3DEM47h2H4/iOAQA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[cfg(feature = "power_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLgzDYIA2C4Ng0EEOQghcMAghoMQgDIMAuDcNwgDENQiD4PAvgOBYoiuBoIgqEgxhCMg2GGF4ZiSG4kDKEw4jyPonimLotgSL4JgsMoeHYNJCiqRpFgWB4JG2SpWhqH5OkSAQ",
        categories = "connectivity",
        tags = "on,off,device,switch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PowerOff,
    #[cfg(feature = "power")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQwCIPg8C+A4FhSF4GgiCgxDgLg0CANguDYYQ5CCJwwCAMYrC2DAuDcNwuDANIThWGg+gEA",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Power,
    #[cfg(feature = "presentation")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMxoDIMAiD4PAvgOBYThaBoIgoMgxg0dgxDEYYMgwMAgiYMQtiQSA1iMIIlicIIpDKKhWDOEoUhmGIEhqCRtDeL4eDULQ1CCRQ1jiFY8D6AQA",
        categories = "multimedia,photography,devices,communication,design",
        tags = "screen,whiteboard,marker pens,markers,blackboard,chalk,easel,school,learning,lesson,office,meeting,project,planning",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Presentation,
    #[cfg(feature = "printer_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg1CAMgyEgNxhDEIIWDAIIZDELYcDEdgtDaFYXhqJYWh4aAxDCI4YiaJIfg4Ig+DwL4DgWM42gaCAiG0MQ2hcOYQkINAtDSMo0jmOIEjqCRNj8MQ4EgNBhDKQoZhsLQylmIA1lSVouloMopiKVZVleJJlHYMpHjWS5KgWB5Nj8ORWDOLIlhuF4dimK4nniL4XHYNpskmAQ",
        categories = "devices",
        tags = "fax,office,device,success,printed",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    PrinterCheck,
    #[cfg(feature = "printer_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg1DMMQgDIMhIDcYYRhEMAghoMQth0MR2C0NoXCCGYbiWJYeGgNguDMNw3CIPg8C+A4FjKNYGggIhtDGLA1iWPggj8NYxjOOI3gSOYJjyQQyDGDpCC2RIyjSSZIgWB4JE0NolDgSA0GEMoSieHAtDKZohDWYZjhqHISmYaI9muYptiiYgyHaT5TkaVoCkmWYKlwORWDOJImm6H5xDChpkiiGB2DaRZVjaAQA",
        categories = "devices",
        tags = "fax,office,device,cross,cancel,remove,error",
        contributors = "colebemis,csandman,ericfennis,jguddas,lt25106"
    ))]
    PrinterX,
    #[cfg(feature = "printer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ4EgNBhDIIITDAIIWDELQyhodgtDWEoUheIgxhSGhoDENoghWI4hDIdofhOK4YhqFBohoIg+DwL4DgWOY8gaCIKgwORWDMYYkiSFoYg0LQxicMJHg2IpLkgdg2jiOo/jkchlGMdAgHcaRkgWCQxDIIggHIeJlmgeZlDSaJrCKVwgGgZRpGcaB0gkOJYC+XJeD6AQ",
        categories = "devices",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANwgDMIA1CIPg8C+A4FhSF4GgiCg5CANhWDOE4VhqGYEhuCRtDGEIODILQyiOFonhQYxpHIYxsGUIBygmIggGMeYJisIo/HiCQ5jGNY3jmJoFgeCRNDEMQuDiEAxDISAyDAYQyCCXQwCCYAxl6Xh2DSXJkmCYovl4SJnl2X5hCAMYvi8dgtm+aZymOLgyGgMguDEN4xiWAonk+CgxDacw2n+hIzgEA",
        categories = "multimedia,photography,devices,communication",
        tags = "cinema,film,movie,home video,presentation,slideshow,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Projector,
    #[cfg(feature = "proportions")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAdxpGQdBog4MgwhIeYODSEhyHiGYSiIIgyCIPg8C+A4FikcBhhcIBkg4TQxDIIA5HYMQxiiKovheLowGiMo0jcORoDEMxhjeNwwCCTgxCCTB2DmPQvj8aA+gE",
        categories = "layout,design,photography,devices",
        tags = "screens,sizes,rotate,rotation,adjust,aspect ratio,16:9,widescreen,4:3,resolution,responsive,mobile,desktop,dimensions,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Proportions,
    #[cfg(feature = "puzzle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgzDkIA0g4ORhDEIIWDAIIZhkMQuDYOAtC4NA3DQIAyC4NYmiiGoXi0MwuDAMYli+MYphaGIsDCIYjiWHYfGyPg4DOF4eDgMhhicNIyiqSolhuLYZi+TRMDEOYejeDYPhWUIsDELZBjuJIqimJ4phyGgtjSS5qjaXJniKYpBGyX5FkOQZHkmS55k6XZpiKSwwEwOJXheVg2DGW44hudIfnCJZlmOXYXn6MQ0pSDItoqLKOowOJzkEM6dnif6PqSOZQn6U4Sg8IKDoeianjeRaOpGkJnlGMIypebaal6nJyqCopIqae6xCCUqAHoIg+DwL4DgUPoBA",
        categories = "development,gaming",
        tags = "component,module,part,piece",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    Puzzle,
    #[cfg(feature = "pyramid")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQgDENguDgOBhDGDwgDCGIPC0LgzDILQxC4NAzGwOYgDMLgwDKFYXhmGYWiENg0hiJYPigMAxiyFouhuHQyg+Ig0GwLQ4g2Fg0hINhhj+P48DGIAuDkOIYFoIg+DwL4DgWV5agaCIKDGTB2DIMJWliXQ+gEA",
        categories = "shapes,math,travel",
        tags = "prism,triangle,triangular,hierarchy,structure,geometry,ancient,egyptian,landmark,tourism",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Pyramid,
    #[cfg(feature = "qr_code")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDUIggHiDgzhEdxpGQdBog6EAgHmFIRHKEwiDEIg+DwL4DgWJ4qgaIoOiUIIXhmG4PhGHwihWB4JguDY2hKMA2iaKItiyBIGiOOoviSEYIgqDIchaGIalGHpBkOKZHiccBhhoIBkg4TQyDEIAxDYaAtDMYQyCCbAwCCbwwC2bAyHaFYnC+XIaluXRol+YZjm0MR2C4MIlniehonyXpgCITQxmwN52mubZwpYMZzm0SA3liiaLn6jRNDOZQyGihaHiing8omf6OpAIAzqahqdn2n6to+bJmoSs6IrWq59reZqkGiqJ5r6rKhoGkK7sWqrImGr5jHYLbNrWAQ",
        categories = "development,social",
        tags = "barcode,scan,link,url,information,digital",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    QrCode,
    #[cfg(feature = "quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGEMgghEMAghQMAthEMh2DaEIShWH4UhkIAxiOIIliQMR2DGHYTh8MYYh6KImheKB2DIYYyhaLolg2DY6hQNgtDYVg1iyM4YhgegiD4PAvgOBZMk+BoIgoNYOkaP4whqHIij+HoRjmO41iuXYulqJ5HjWN5hjqMo9jMIJBkORZlhaSAykqTJOgQaA+gE",
        categories = "text",
        tags = "quotation",
        contributors = "Billiam,jguddas,karsa-mistmere"
    ))]
    Quote,
    #[cfg(feature = "rabbit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDYYYNg0MAghQMQgDILgyDQIA1CIPg8C+A4FiCI4GgiCgxDiDgyGgLgwDGH4hiaJYEieCYLisMgxGgLQ4GGHIchSFgtDSRQgDeSIVkuFw3C0N4uDITA5C4NggDYLg0GGF4XhaDoYC4OAthkOBMDENZhkiURjDMLoTleYJJlYNh2DEYQyhiS5EniLQtnaEp6hULYSjKIo2jWBYHjgMoUDgLg1DQVpanyTKCkIdgzoWNICjaioKDeVQxnioqPhuEQgm+XZ+lUIA0m6mqHgEA",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = "danielbayley"
    ))]
    Rabbit,
    #[cfg(feature = "radar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLgwDcIA0C4OQzEEMQwCCFwghiHAgDaEw5CAMwuDMNAiD4PAvgOBYoiuBoIgoNIeGiDgxieKYui2BIvgkTQyC4Mohg0NgyhaGIakeGwgDIMYkDEIA4iQNY3iqO46gWB49DGHwyjINwuDcNhBDaHpKk+GJRDIM4Zh8Ng3lSOYCjuWYKDEMoZDiNAwjaKJViycpYjCC5fDmIQxk0NpimSZIdk8MQ1mCEJbC6bpwlYPBjGkchjGwZQgGMeIJnYIggHKCQyqQYx5qKqJ9pmm6dlePAiG0MYjDSjgwC4NYhpCiQtr4NqWiyAQA",
        categories = "navigation,security,communication",
        tags = "scan,sonar,detect,find,locate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radar,
    #[cfg(feature = "radiation")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMGgLgwDEIg+DwL4DgWFYYgaCIKDENIODULg0DYNAxGGIIgDAIIrDELYqEwNwuDUMg3DgNoODkLg3DQNQ5DcIBBg6Q4rkUIIyDkOQzDAMwgDIMQuDENpPDGDotkaRpShEMA2j+TpQlKVJDlWWIODaIw3k+IAxjqPI+DceoUhaG4agSHIJguOIMigIIqiyLAtDILQziOJRsDKaKCDQLgyDgMhhlWZJ/pGhQtC6Sw1lampZn2W44mcOQ0EGkZ/i2X4ODKcYVhedp1gWB54DiqJ8n6pqCoSJA0jGM6Np0Mg1DSkJEqULZQiSlqYpoMZlDCL6eCCoKiqSZZNgyqpzq2AQ",
        categories = "science",
        tags = "radioactive,nuclear,fallout,waste,atomic,physics,particle,element,molecule",
        contributors = "karsa-mistmere,danielbayley,ericfennis"
    ))]
    Radiation,
    #[cfg(feature = "radical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQyGgMwuDIOBhDGDQgDCGIXC4OQ0DgLg2DgNBsDKEg5DgIA3hwMw0GELg1i+GoZhmHA2C0LgwDQNBMDGEQ4DIIA0C4Nw3DEQYWhaM4XDGQg3DmQA0EgMgxCIPg8C+A4FD6AQ",
        categories = "development,math",
        tags = "calculate,formula,math,operator,root,square,symbol",
        contributors = "smnandre,jguddas"
    ))]
    Radical,
    #[cfg(feature = "radio_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg0DENAggyDoQGEMgghcMIShILQyC4OAyDiHYfiEIg+DwL4DgWJ4qgaCIKDENguDINA3CANwuDcNgxGENggj6GoaDGEo5DSEQ0C4NQ3DKJooi2LIEi6CYLDkLgwDcNQgkgOQzDMYQxkKQQgkKGIzDOEZgjmTInimUZQgWB4JG2F50hoMgwk2bYrgKUZxgqWwylkMZVDANg3l+YZjoqQwwC2EAugwNJ5k+fJwi8TY4lgM4SjIMgzDmPY/oqZKNDiDg3Dik5ugE",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "kongsgard"
    ))]
    RadioOff,
    #[cfg(feature = "radio_receiver")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ2HYMgiD4PAvgOBYThaBoIgoMQ5g2D4RhOFYEGiExyGUYx0CAeIJhEIBoGUaRnGgdIJDgIggHcaRkgWLQwjgeY2jgcosCKIYUieKYYiSB4JE0MQ4g0MhoC4MAxhKFIZD6AQ",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[cfg(feature = "radio_tower")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4OQgDENguDEQwxg8MguDKDwgDULg4CCDIODGDQiD4PAvgOBYliiBoIgoN4dh8Lg3GGEQxDQII1jcMAgjsMAtjCLw1iSJoriUYxpHIYxsGUIBjHmCQ5CIIBygkMpSGMeIJDGVolC+R5JkuKoEiyCRNhCGIxDgY4ZmyGA2huEoih6Lw0DeQ4nmOYoFgeZQxDmEoPg0YZ/Dmb6Em+PYajuNoSneRYCmOfIKn8NYPDgaJCl2j4rpIbYeDKGQ0C0MYVjepKOnmAQ",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[cfg(feature = "radio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLgyDQNwgDcLg3DYMRhDYIIZDAIIcDGHQgDgLoQDgIg+DwL4DgWJ4qgaCIKDEOQuDANw1CANAuDkMwzGEMYehyQAgh+Ho4DEMw0iaKItiyBIugkTY4DkMo2jGMw2DePY/iCHodC0MZFkeSYpk2TIFgeT4TjUM5Cg0MgzDmGIaluQpdiKJJiksPBjGkchjGwZQgGMeYJDEMgioEeKEoYIBygmhonC+e59n8PoBA",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    Radio,
    #[cfg(feature = "radius")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgzDQIAxDcLg1DIYQxDCEYZhkMQgDALQyC4OAyCCIYjCIPg8C+A4FikYxpHIYxsGUIBjHiCQxDkIo1HmOI6CAcoJDKKIqi+MYzimLIGggIhtDEMwuDSHZPlGHQ0C4MQ4CCV5ZkSK4EGiLowjKNJBCKQ41jcIgxmgY49muQ4pC+RpkD6AQ",
        categories = "shapes,math,design,tools",
        tags = "shape,circle,geometry,trigonometry,radii,calculate,measure,size",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radius,
    #[cfg(feature = "rainbow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYQxDCDoThUIAwC0MoVCIPg8C+A4Fh2IIGgiCg2g6EInieFgxg6DQwhyHojiKBIkgkTYSigYYNi+F4OCANIXjGH41D6AQ",
        categories = "weather",
        tags = "colors,colours,spectrum,light,prism,arc,clear,sunshine",
        contributors = "danielbayley"
    ))]
    Rainbow,
    #[cfg(feature = "rat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDISA0GEMoOCAMIWCAMYWC0NBoDEMgiD4PAvgOBYiiWBoIgqDAuDIMw2hkOBhg2DYXjYLQyi0LQ1iGI4oieBIpgmC4wDkaAuDAMY9iSQZAgWB5DDENguDiFAzC4OYSjSGIahqV4uDcIA0lQNg4GwMZUDENYOC4NQ4DcYZomucoYhcMQtnSOQxGyOJUDcMgtC4NA1DOMwgjWdQtleawzkuP4CkGUIrmGYw5DiMpbnaGw1i2bAwDUMhBmGYY2hgNIZmOSZrqep6lhcOIxo6TYB",
        categories = "animals",
        tags = "mouse,mice,gerbil,rodent,pet,pest,plague,disease",
        contributors = "henri42,jguddas,karsa-mistmere,danielbayley"
    ))]
    Rat,
    #[cfg(feature = "ratio")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINgiCAeYJDKDByggIoQCAaBlGkZxoHSDwwgwdxpGQdBogkMYQD4PAvgOBYoiuBoOgqH4hiOHYMheGYbiWFYSg+DITieKYuD6AQ",
        categories = "layout,design,photography",
        tags = "screens,sizes,rotate,rotation,adjust,aspect ratio,proportions,16:9,widescreen,4:3,resolution,responsive,mobile,desktop,dimensions,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[cfg(feature = "receipt_cent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYMQwCIPg8C+A4FhSF4GgiCgxDQLg4DIOAgh6IAyDkYQ0CCKgwCCLQxC0NQuDYNQ2i6K44i2L4ujGMw1DeOIsjcMQgjKNI2hKFIWgQaIZkyB4JE2KgzGGRJEjqJIkC2VguDOJJdjeOwuDcLgyGwLg5DMM4zlWYAxmCWIvC6LJnmkNAtmyb5enqYZZm+dJomqeZunCfZ/i6dZqngNptnuhY7ocMKJh+jJ8nycZfoCaQzosQaWo+fg5CAMpVlmWJWiSEA4qWV5DlufqEl6p54DeeJmnim6Lo2X6ymGW5zoiuAzpSu6Xq6kRssKnKDo6vY6r+dLKsyvKGtCwaBne07Gs+yLSpWsbVmOZZ+tWMAxHqE4VhoPoBA",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,cents,dollar,usd,$,¢",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptCent,
    #[cfg(feature = "receipt_euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg4DIOAgDENIODIORhDQIIZDAIIcDELYNDYNQ2h2GomhyHodiALoiDeJobiUMQgiGI4dCIPg8C+A4FjiO4GgiCoZDMYYyjKKIShILZFC4M4SkyJYpC4NwuDIbAuDkMwziyRJPDGT5Hh4LoblaWA0C2W5ek2aZQkiXpjleWZol2X5sm6HZklmZw2lyap0imdgwniFJ7mua5gk6b5YDOehBoWfptDkIAykSSJHkWEh2DEOKUkaMZKm2c5NpaZw3meVZnoqep8k6opQkqYp3qgM6DquhqeoAbKyoucp9q2KKvmOuq8qydbArGcJmsOtq/riwqEqGxZSlSbbFh8MR6jeOY+j2BI/gkTYRDEMhoDW2Y6t0PoBA",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,€",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptEuro,
    #[cfg(feature = "receipt_indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMxhDEIIQDAIIThAMQthYLgzhGGoUh6EAuDcLgyGwLg5DMMwuDaD4dDGHYTjCHIMDCJYnDQLYqiyG4uhuMYZjONYojmPIcj2HoVC6QImiiOIrkSRI+jKFJBDSQ4ti+R5SjSSwzk0QZPliFYRDkIAyg+EZZhaER2DEOJnhKH4YmiYJGhWOA3jiJI4ieXZWjuYYUhiSZTnsM5Vk6V51hGgpAoWfaIn+igwoyhJcn6RZppSNKFjel5Qkemhso6nqAheIYjnOmYXDEegiD4PAvgOBavrKBoIgoOIRDEaA4q6sK1rSBK2gkTa5DevK+rGwrBgWB7EmQNxhgyM4fh4OBIDgbIbDKybAgE",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,inr,₹",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptIndianRupee,
    #[cfg(feature = "receipt_japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDAIAzC0MwiD4PAvgOBYWhmBoICITQ0hEYQxg4IIQhCJAxC2KQuDODotiaMYkC4NwuDIbAuDkMwzC4NojjAMYwieMguiEMI4joNAtj2P4ukGLpDiyRpIjuTJPi+UIxiiRYmlSEpWkCQoyi+U45DMNJgk6YpRlyR5ml8NhBleV5Rg4OQgDKI4lnWKR2DEOJ6iSUYriWc5rg6Sw3kuN5LjqcJNliWomiubRso2Z5ppGg5BlOl6PoaWYnpSnZvpmdIyqOXaXkqpqHDCqZHp6rahoiNI2oWkoqroeoVheHIbgSHYJE2dwxDEaA2r2GLBsCBYHsOxQ1siyq/gKwbPgqdw3hGER2De1LMgEA",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,jpy,¥",
        contributors = "it-is-not,ericfennis,karsa-mistmere"
    ))]
    ReceiptJapaneseYen,
    #[cfg(feature = "receipt_pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDcVg5C4NRhDGDggg2DYWDWGAiD4PAvgOBYfiKBoIgoNAgDOFYXhmFwxC2FgxC4M4OjSGI4hYLg3C4MhsC4OQzDMLg2hWN4zjWLoaC6KQwj+QQ0C2RJGjWSI4hqNpNk+QpTlaVpKlmGJbDOUpFl6N4ujKTJikAMw0l2R5oleYZOm2ZJEEGZ5JjmDg5CAMoshaaYvHYMQ4oGc4wjKNpVnKGpSDeUo+lKQZ3macZ7hmMZrk6lJunCjaZg6m5ap6lpUoycwwqSYqmqCqZpqynZtlGr5fleshsq6l6homkI9i+vqKHqHogiWJIEiaCRNDiDgzGgNbFiGybIgWB7Ls2DxoDe0rHgE",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,british,currency,gbp,£",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptPoundSterling,
    #[cfg(feature = "receipt_russian_ruble")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMxhDEIIQDAIIThAMQthYLgzhGGoUh6EAuDcLgyGwLg5DMMwuDaD4dDGHYTjCHIMDCJYnDQLYqiyG4uhuMYZjONYojmPIcj2HoVC6QImiiOIrkSRI+jKFJBDSQ4ti+R5SjSSwzk0QZPliFYRDkIAyg+EZZhaER2DEOJnhKH4YmiYJGhWOA3jiJI4ieXZWjuYYUhiSZTnsM5Vk6V51hGgpAoWfaIn+igwoyhJcn6RZppSNKFjel5Qkemhso6nqAheIYjnOmYXDEegiD4PAvgOBavrKBoIgoOIRDEaA1GEMpllmMAtDQaAtDObAwq6sK1rSBK2gkTa5DENa8sqsbOD6AQ",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,rub,₽",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptRussianRuble,
    #[cfg(feature = "receipt_swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDEaA0CIPg8C+A4FhSF4GgiCoMg4NxWDcaA1hOFYahmBIbgkTQ0CAMxhDGDggg2DYxDELY2C4M4OjqM4+jELg3C4MhsC4OQzDMLg2jCPQxj2NI/C6LQwkWRw0C2SpMjuTo7lCOZTlWSJZlyPJdj6NZSjOYQzliS5kmSXo8mCRgzDSY5Nk+Z5ymqdJskoQZvnmNYODkIAyjCMpxjYdgxDiiIxl6OIyoGZo1lgN5YkSWJHn6bp4pWM44mmVKbnWd5boKDqimCpadlqZZ6DCq5qq2p6wpGTqsnSV62nCZ6zqSfa9qmN5BkOk56jeyh6iWFopiiBYHisOIODWI7NieAQ",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,chf,₣",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptSwissFranc,
    #[cfg(feature = "receipt_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDYSA4CIPg8C+A4FhSF4GgiCgxDQIA4hGE4VhqGYEhuCYLDaDgyiKFIWieJoFgeKYfDMYQxg4IAwjuOgxC2OQxC6DZCg2PI8jkLg3C4MhsC4OQzDMLg2jiQ4OlaR49kKHwwk6UA0C2U5VkSWJaleXJelGYpFleRo9kgLpok+UZhlSbJslmQZxjuaQ0muVp4m+Z58nMM51EGd5lkiDg5CAMo4jqeY+HYMQ4pCOZ5kCPqAoqDphDeYZNmGUKGn+ZJukeQJ7l2owzn6dqcqinpbnyralrCp6CDCqporapptoKP60qyc5gr+gapsMbK+riwKZkqTI+sGQJAHqI4whiAQ",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,details,small print,terms,conditions,contract",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReceiptText,
    #[cfg(feature = "receipt_turkish_lira")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA3HaDBhDUIITg2FoUC0NQiD4PAvgOBYch+BoICIbQxDQIA4C0NggDOG4diKIYEiOCRNigMxhDEII6heOgxC2PguDOO5CCCPQgC4NwuDIbAuDkMwzC4No5kUMZFheDZWigMJNk8NAtlKVJDlaQ5HlqRpdlCYZkkSZZGm+Z5ck6UJglObJsmYLpbmkNJrlWV5vlmeponMM51EGd6AlmOw5CAMo5jugaRj6EA4pCPJwkCk5/m6WZgDeYJMmCT6Gn6Y6KkaQKDlyowzn2dqcpKP5xGyralrCp6dqmtK2qabayqqe6tl+vp4oGwZor2uK/j2n5LpOwKzHqL4ejMPoB",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,try,₺",
        contributors = "danielbayley,jamiemlaw,karsa-mistmere"
    ))]
    ReceiptTurkishLira,
    #[cfg(feature = "receipt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcVg3CIPg8C+A4FhSF4GgiCgxDYIA4GgLQ2GGDYNDAIIoioIA0GgNIlCCJ4pg6Mw0EgOIThWGoZgSG4JE0NAgDMYQxjSK5FDELZIC4M4OkyM4okULg3C4MhsC4OQzDMLokDGT5ek2K5RC6QQwleWQ0C2XJEl+T5Hk6ZZnlqa5gk6YZQnCKZyDOapdm2d5RnmZpYDMNJ0n+eJjnGhJ8lwQZ1nWbwxDmMZEkaM5Ig4dgxDilpFkeSo0pCbqYmoN5qlaapZo2fpNpGUJKmSeqqoWh6uqSUaxnGtKsmyt6Aimuqzoytp2niSZeruhJpsWr4qsKZq8s2uIOqaVaiseSpKHqOYWj0PoBA",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,dollar,usd,$",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[cfg(feature = "rectangle_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0HYMQ2EgMxhDEIIWDAIIZDELYcDEVg1hWF4aiSFoeHoIg+DwL4DgWKhjGkchjGwZQgGMeIJgwIggHKCQ4jsYx5jkMopiuMIyjQPoBA",
        categories = "development,text",
        tags = "compose,keyboard,key,button",
        contributors = "zefir-git,jguddas"
    ))]
    RectangleCircle,
    #[cfg(feature = "rectangle_ellipsis")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoIgqDBoGUaRnGgdIJDGCwgHcaRkHQaIJDIMIMHmCQ2CIPg8C+A4FikcBhh8IBkgkTYZCCGRoC4MAxiiKovh+LowGiMo0DEN43DKOY7j0L4/GiQYxjMIhNkeOI6jyKZNkIPoBA",
        categories = "text,development",
        tags = "login,password,authenticate,2fa,field,fill,ellipsis,et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "mittalyashu,ericfennis"
    ))]
    RectangleEllipsis,
    #[cfg(feature = "rectangle_goggles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA2GEMgghGDYNDGEoSHYOIQheFAgDELYRDIaAtDSG4TCCFQtDELg2C0Lg4GyKosiALgxDMYYWhaHQwjINooEwOYsh4NwuDIQYhiiSYWDiHg4EiJZIh2HwyiAVoalGSoSiAegiD4PAvgOBQ+gE",
        categories = "devices,gaming,multimedia,connectivity",
        tags = "vr,virtual,augmented,reality,headset,goggles",
        contributors = "EthanHazel,jguddas"
    ))]
    RectangleGoggles,
    #[cfg(feature = "rectangle_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMgiCAdxpGQdBog4MgwhIeYODaEhyHiGYSiIIoRD4PAvgOBQ+gEA",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDIIggHiDQ2hEcoTCKEAgGgZRpGcaB0g0MgwhEeYiCIPg8C+A4FD6AQ",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,9:16,vertical,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleVertical,
    #[cfg(feature = "recycle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ5EgNAuDgMQ1GEMYSDODYYCAMIcg0LYXDUNwthKE4aDcOA1ieKYeh0MYkDAMA0iALooDQTA3C6Dg2CAOQuDUIg+DwL4DgWQpFgaCIKDEMYNDkaA4C4MgwDOFobhcOIZh2W4aDUNQ2iQOA5isNJki2LY0DcNw1GyNAyDKYAyjoMpBkOSJHgSSYJG0MZlDGYIZoEIAznWRJ5niBYHgkTZRDIOYZDEM4/DmPI5juPY/oOOoNDCkw4oWd4CnmigiG2PgzDSZQ1hKTIaDCj40mIMhBlikIblyTZMC4OYslWF42mauJdDQNolDgbKSDkNIZsUOLLqCh6iomSp8pIM4opgNgzhmEavjyF6vDirg5iO3aUtCRoBA",
        categories = "sustainability",
        tags = "sustainability,salvage,arrows",
        contributors = "karsa-mistmere"
    ))]
    Recycle,
    #[cfg(feature = "redo_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDQIA1C2EoUCIPg8C+A4FheGoGggIhNDIMAgDkSA5C4NRBDWKIRiyI4vCCEIPiiKosiuDYwiOJ4NiISAxDOFoYh0PoBA",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Redo2,
    #[cfg(feature = "redo_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ3hAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MgxCANx2DYaAtDaGYbiGI4giKJImCITQzCCFBhDkIJADAIJDioOQtkCSZEkuKg2CAMguDMbI8lCFYaC+NBoD6AQ",
        categories = "text,arrows",
        tags = "redo,history,step,over,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RedoDot,
    #[cfg(feature = "redo")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3HYNhoC0NgiD4PAvgOBYWhmBoIgoMwgDENxhDkIIlDAIIog0OQtiWLopjCDQ2CAMguDMbIgjUN4VheHA+gE",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Redo,
    #[cfg(feature = "refresh_ccw_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYQ5CCEgwCCFQwC0OYZhMLg3DWHIehaIoYDaHQ0CAMomEwMwgDgIg+DwL4DgWMIzgaCIKiwMx2DUaA1i+MY2jWBI3gkTYsg+EYTiOIoSk6HYfDmUJMhWJQ3DQLYplcTIMg4NpAjKRJDgWB5GDENpej6PJgkIPBjGkchjGwZQgGMeYJg8Ip1HieAynocp4mCb5xnMPoBA",
        categories = "arrows,development",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = "csandman,ericfennis,danielbayley,jamiemlaw"
    ))]
    RefreshCcwDot,
    #[cfg(feature = "refresh_ccw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYQ5CCEgwCCFQwC0OYZhMLg3DWHIehaIoYDaHQ0CAMomEwMwgDgIg+DwL4DgWMIzgaCIKiwMx2DUaA1i+MY2jWBI3gkTYsg+EYTiOIoSk6HYfDmUJMhWJQ3DQLYplcTIMg4NpAjKRJDgWB5GDENpej6PJgkKAQ",
        categories = "arrows",
        tags = "arrows,rotate,reload,rerun,synchronise,synchronize,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCcw,
    #[cfg(feature = "refresh_cw_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA4EwMQ4C4Nw0CANYTDQQQ5hMNQghsN4dDAIIiiIMQyCAMxDDGDQzCAMQwC4MItDMLgxDaHo1jMLg0DcIg+DwL4DgWP5CgaCIKDiLg2EgMx2DWPpAkWRIEkaCRNi2JhDi2Gw1g2FQ3C4Mo3hcNoVmQNJQkGVJTgWB4JG2WI3DKYZynSGocjiIIjnuJYngwY5zDQOZ7DSGAtg0NguDMNgtnOZZplKApUm6CoMi4MhjiULY1jcMQuDkN6bjwIJzDiPY/mqQ6Sm2R4LiyThoC2T6opGRaUgufp+qSkJrgEA",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle,cancel,no,stop,error,disconnect,ignore",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCwOff,
    #[cfg(feature = "refresh_cw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQyGEOQghEMAghQMYSC2EQ5C4Nw1hKHIehSFggDaHA0CAMomEwMoXDgIg+DwL4DgWMIzgaCIKiwIAzHYNRoC0NYvjGNo1gSN4JE2OoOhCEoVk4MYZk2G4dh+VIig0LYlDcNAtimWxMgwMQ2kKMpGkWBYHkgOINDYSI8kGMJljSAQ",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCw,
    #[cfg(feature = "refrigerator")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANhhDQIIQDAIITDGEQtDQaIOhCEoUCCFocHYMQ0GEMggiaE4VC2JgyEgN4lieHoqDKKxWDYWgiD4PAvgOBY6j2BoIgqDAxDAaIjjmO5Aj+BJBgkTQxgwNx2DaSY8k0PoBA",
        categories = "food-beverage,home",
        tags = "frigerator,fridge,freezer,cooler,icebox,chiller,cold storage",
        contributors = "karsa-mistmere"
    ))]
    Refrigerator,
    #[cfg(feature = "regex")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzHYMQwCIPg8C+A4FhSF4GggIhtDEMguDaDQ1C4NQgDiIQ2CANYThWGoZgSG4Jh6IIiCCEYlieKQtiyFIWjGMIFgeCRNDmNw3GEMggkoMAgk0MAtDKURIDWSZLk6WJQkoMh2DKVpMlmVwyGiXpbmGTZSlyUR6i2P4YgE",
        categories = "text,development",
        tags = "search,text,code",
        contributors = "mittalyashu,ericfennis"
    ))]
    Regex,
    #[cfg(feature = "remove_formatting")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANxWDQaAxDYdgzCIPg8C+A4FheGoGgiCg1CAMgwGgNoWhiHYcgSHoJE0MQzCCDA4iIMInhmK4qgWB4JG0MYhj4IIhDWNopgKK47CIbYjCCPgtkKRI4gEA",
        categories = "text",
        tags = "text,font,typography,format,x,remove,delete,times,clear",
        contributors = "ericfennis"
    ))]
    RemoveFormatting,
    #[cfg(feature = "repeat_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAyCANIQC2EQ0CIPg8C+A4FheGoGggIhNDMIAxDEdgtDEYYUCAMIriOEg0GgMYVheGYEGiHI2geCRtg0MgyhOE4vhaGIdjiBY6iAMgxiMMx2iiKosiwMZBDQSAzkONYbgKOYfE2JIjDCMR2jORI2D6AQ",
        categories = "multimedia",
        tags = "replay",
        contributors = "ericfennis"
    ))]
    Repeat1,
    #[cfg(feature = "repeat_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOQgDMLQzg+DwiD4PAvgOBYWhmBoICITQxhIMQ4EgNxhgyDAwCCKgxC0MouFYNoVheHIbgSHYJguDAxDWEYPhGEYzhiN42gWB4JiAMQgDYaA2icIIpisIJKigdgxDCQo1gE",
        categories = "arrows,social,multimedia",
        tags = "arrows,retweet,repost,share,repeat,loop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Repeat2,
    #[cfg(feature = "repeat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAyCANIQC2EQ0CIPg8C+A4FheGoGggIhNDMIAxDEdgtDEYYUCAMIriOEg0GgMYVheGYEGiHI2geCRtg0MgyhOE4vhaGIdjiBY6iAMgxiMMx2iiKosiwMZBDQSAzkONYbgE",
        categories = "arrows,multimedia",
        tags = "loop,arrows",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Repeat,
    #[cfg(feature = "replace_all")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIIMGEMYOCAMIThKEQxHYNYQhKFIUDELYXCIPg8C+A4FiOJoGgiCoMCANIbhGHYWiCIokimKIEiqCYLDWDgwjCFYeiCNIjiWOY4gWB47DEOYOi+F5BhaDoZkCMofiGRY3gKOZKgoMoRk+HJBkMMY1kaJ5bkmKxNl8IA5lWFZXg6ZpaimXRtDMIA3CCeZ9C0M50keaY6goNo+FaGgyCCipWosLQyGgMqBmgchlGMdAgHmCYMCIIB3GkZIFgkN6dHiCaACAcqmCKZQgGgZRpGcaB0qOZqVpcPoBA",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0GEMQghEMAghSEQxC0MQiD4PAvgOBYch+BoIgoMQ1hIMIQhKFYshiLobh2IohgSI4JE0MoRg+F4shQMIZhmMIejSM4FgeNo4CAOYqhOLYZhKQYygKNJGCIbQzCANwgleWwtDOUJDlKRYkE0NooFYNRhDIIJqj2KwyC0MhoDKX4gDwchlGMdAgHIeIJhoIJ9CKXggHmfg0CIIB3GkZIFgkN6IGgZRpGcaB0o6QZ3nkPoB",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Replace,
    #[cfg(feature = "reply_all")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDcLQ1hEIISDUIg+DwL4DgWGIbgaCAiE0MoNDEOB2C0MhhDQIIrDAIIuDALQ0jISA3heGYeh2BIfgkbQ3g6EIVhSEY3hqOw+gEA",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    ReplyAll,
    #[cfg(feature = "reply")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDgdgtDIYQ0CCFYNhgLQ0hoSA0CIPg8C+A4FiCI4GggIhtDmDg3C0NYuCCLw1h+IYmD6AQ",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Reply,
    #[cfg(feature = "rewind")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA2GGDYNDAIITDALQzC4NAxDQLQxhmGxsC0NoOhAIIShSKITDILg4DIOBsiMNhBhGKYogwIAxDgegiD4PAvgOBY9kCBoIgoMoNg+NIVhSF4fhyHoaDSIYxiWJ5LiqLIujCDoziaNYqg2OY7j2P4EGgPoBA",
        categories = "arrows,multimedia",
        tags = "music",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Rewind,
    #[cfg(feature = "ribbon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDELgyDIQ4PCAOQuDkOQ3g4MIWhwIA4GGDYNh2HQxCANAgDAY4mC2GA5DgIAyC4MAwDILYzDCJwzhEMgiD4PAvgOBZAkOBoICIbYMg6MYzDUNwtjwNY/kGRpFgSR4JE0NoRDQM4WjQMQ2GGG4biWDoOhANQwlCNAwDmVJCliV4FgeWoXDMNYODQLg1l+S4Pj2cZWgKWJ2gqeJ6DGfJ+EMNwuDcMoxgyXQ2CClgxDCPYnpYNxhpaepnieS4ri6NZ6jyEgthANw3jENYRnmUQul4IKPDUbJSk+KJ9DKnqjiqwQxi4Mgwn+tAzGyqwuDgOYphAM5jsCorLDMOA0sQMQ1EygA4sqMw2DcOQglIOQzGG07CtWFwyDGybLDgNg1tWfromi1IRsayw0hIeqDnOAQA",
        categories = "social,medical,emoji",
        tags = "awareness,strip,band,tape,strap,cordon",
        contributors = "karsa-mistmere"
    ))]
    Ribbon,
    #[cfg(feature = "road")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GgiCoMCANRWDOE4VhqGYEhuCYLg0OR2iKFIWieJoFgeKQyC4MA3DeDg4C4NA0DkQYNg0MAgkSRA0CAMgxGgMQ2GGQpFlGRAxC4OQyDQLY2DUNRsC2WAxDSQZJlKUZNCAMxIDiT5jkaRQtlSVpIlQNA1HqI4whiAQ",
        categories = "transportation",
        tags = "road,street,highway,route,path,transport,traffic,drive,map",
        contributors = "uibalint,karsa-mistmere,jguddas"
    ))]
    Road,
    #[cfg(feature = "rocket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDUdg1HMMwuDAMwtC4NQ1CANAtDIYwxhUOAtiENoNDALYbigNQiD4PAvgOBYujGBoIgoNIZg4NoZGOJI5iEMg2h4IA1kOEoUDeHY5kWHwuDcMYYDiOA3h4LgxheFQ5lUOQxGEMpWDgIJfDGYQwCCZool+XIYDAOYti+NIzgSNYJE0OYODKXoNDKJ5ng6YgthQOQ1EGDAuDiYaGoifpmDGYp7GOZpfDcMoYDeYQ3hkLQ2g6XZ8C4NKPqCjIOC2oQyHqb4wnOcoFgedZ3gwSA0HOGZFhSFqADSIAuiaPgwmGG4VhuwgwiyLqrjKAQ",
        categories = "gaming,development",
        tags = "release,boost,launch,space,version",
        contributors = "ericfennis,jguddas"
    ))]
    Rocket,
    #[cfg(feature = "rocking_chair")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDMIAzC4NwwDgIA3C4NAxDYIg+DwL4DgWHohgaCAiE2EAxDkYYMg6DQwCCMIwDGFgwh2H4kiOBIlgkbYQDKEQuDIMQgDkLg2DMMxBkCQIyjEIA4C4MQwDmDgzEiNI3iCO46gWB49lWDwthKFIWhiGoch6W4igEA",
        categories = "home",
        tags = "chair,furniture,seat,comfort,relax",
        contributors = "connium,ericfennis,jamiemlaw"
    ))]
    RockingChair,
    #[cfg(feature = "roller_coaster")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ5FYNQiD4PAvgOBYThaBoIgoMQwg2Dw2C4OIShSGYYgSGoJE0MQ0h8dgtDeIokhWKIngWB4qDEOAgDUdg0jOJoCiiOIcjuDovDaQI1kKN4bE0Mgyh8Vg5kqF5MimCpRg6Uxhi2LYeh4MQgDQLQ0GOUYei0MQuDMMwggwNBzl6b5jl2YwgmGeAtDMLYhDaEYTjSF4B",
        categories = "navigation",
        tags = "attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    RollerCoaster,
    #[cfg(feature = "rose")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDAaAtDEYQ0CCFQwg6GQ0C0NB2C4NQzDQIg+DwL4DgWJIngaCIKgwIA2GiE4VhcIIYDGDguDQMggDcLg3DQbAtDILgyDkLg4DcYQ2i+NZNDELQ1C4MwzDmEgwC4Ng4GyQwwDaVQxlIMYjiWKopgSK4JE0NIfg4NxjkMOAtmya40lGUQ4HOYA4CCQ47kMMwgDOPhjkIII5C2gpRhua5xlKEpEnINpCo6YJVDOHJEmOJpnmaBYHmmRg3g0MQyEOFQxDWfJ8n6q6amUPBjGkchjGwZQgGMeYJDgIggHKCQyrwYx4gmDKarGs61D6AQ",
        categories = "nature,seasons,sustainability,home,social",
        tags = "roses,thorns,petals,plant,stem,leaves,spring,bloom,blossom,gardening,botanical,flora,florist,bouquet,bunch,gift,date,romance,romantic,valentines day,special occasion",
        contributors = "danielbayley,jguddas,jamiemlaw,mittalyashu"
    ))]
    Rose,
    #[cfg(feature = "rotate_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLg0DYNggDcLg1EMMQ1C4Ng0DMIA0C4MgzDcIAxDMLg5DUMggikMYpikOYfDMOYqhIIINDQN4iiKLBzDKMIyDEMAgDWIwwGMLgzDSKZBhmOAtC4MA2j+TgyCIPg8C+A4FlaWYGggIhthcLgxDkNIjiUNwwiKJQ4DGZQxC4OA2C2b5xCCa5tlWV5cluBJdgmC4/hgNQ3GOc5wDCHg4DgNQth4Mg3m6DgzC2OqSC2gogCAMJzpuPYgDmnKXHOHo3Del5EpcY6JDMOKaCAOJwDeHJvDcMati+rQ0nmWJ9D6AQ",
        categories = "design",
        tags = "gizmo,transform,orientation,orbit,axis",
        contributors = "lscheibel"
    ))]
    Rotate3D,
    #[cfg(feature = "rotate_ccw_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYNgiD4PAvgOBYThaBoIgqDAgDkaAyhKFIZhiBIagkTQzCCDBhDmHggDCK4wh4LYuDkLg3DSHo4jqMY+C0No8CAMo8EyKg4iKFYmiWBYHiiKgzHYNRoDWSYkDwYxpHIYxsGUIBygmIQgGMeIJgwIpjHmZpVhML5ZluXQ+gE",
        categories = "security,account",
        tags = "password,key,refresh,change",
        contributors = "karsa-mistmere,pgbradbury,jguddas"
    ))]
    RotateCcwKey,
    #[cfg(feature = "rotate_ccw_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA5FYNxhDIIITg2FgtDKGBoC0NgiD4PAvgOBYfiKBoICIbQxDWFAtDMIIujCHogiWJIEiaCYLg0MQzHYNYShQIIWCAMYYhQSA2j+FZBkOGIYhCSZLjqLAyGgMoyiGNg+gE",
        categories = "layout,design,photography,tools,arrows",
        tags = "left,counter-clockwise,rotate,image,90,45,degrees,°",
        contributors = "danielbayley"
    ))]
    RotateCcwSquare,
    #[cfg(feature = "rotate_ccw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQyGEOQghEMINCCFA5C2EQ5C4Nw1hKHIehSIgtDaHA0CAMomEyDA4CIPg8C+A4Fi+MoGgiCoMDMdg1GgNYujCNQ+gE",
        categories = "arrows,design,photography",
        tags = "arrow,left,counter-clockwise,restart,reload,rerun,refresh,backup,undo,replay,redo,retry,rewind,reverse",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCcw,
    #[cfg(feature = "rotate_cw_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1EgNhhg2DQwCCFQwC2Ex2DMIg+DwL4DgWHohgaCAiG0OQgDgIAzC2LYvh2H4kiOBIlgkTQ0CAMQ0HYNISCCFIWkKExogyP5BheQIZFYN5HkKF4ZhkaIZjGII1D6AQA",
        categories = "layout,design,photography,tools,arrows",
        tags = "right,clockwise,rotate,image,90,45,degrees,°",
        contributors = "danielbayley"
    ))]
    RotateCwSquare,
    #[cfg(feature = "rotate_cw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYQ5CCEgwg6DgtDmGBjDILg1DIIIVDQLg5DOFg2C4Nw0CCHIpEyDAgDgIg+DwL4DgWM42gaCIKi8Mx2DUaAtDWMo0jkPoB",
        categories = "arrows,design,photography",
        tags = "arrow,right,clockwise,refresh,reload,rerun,redo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCw,
    #[cfg(feature = "route_off")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEOYOGMeILDYIg+DwL4DgWB4bHAYR0GgIBkgsTQ5CCExoDgLg1GMLg0CAMAgC4OQtC4MYrC4M45DKGociKJIhiOJYnCITQ1C4MggksMhBDOLwglINQzjSWI1DaUwxDISJdkEL5DGiRYkiaCxtk2ao1DIMJhmOZZHigMo7DGSwzGGVZUlONZ9C2Uo+oCb5GnGZ5JnaThoC0NI9oORICgSBoIhCCw1g6CoMhWFwiDEOJhh6kg+gE",
        categories = "navigation",
        tags = "path,journey,planner,points,stops,stations,reset,clear,cancelled,closed,blocked",
        contributors = "danielbayley"
    ))]
    RouteOff,
    #[cfg(feature = "route")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmCwxDkIg+DwL4DgWB4bHAYR0GgIBkgsTQ5CCGBoDgLg1GEM4vCCMg1CAMI3jkMAtDcaAtDEMYxjONY6iuN48EgMQ1hqHIiiSG4egaCIQhcOIUhYIpLgmC4NhuHYElIPoBA",
        categories = "navigation",
        tags = "path,journey,planner,points,stops,stations",
        contributors = "danielbayley"
    ))]
    Route,
    #[cfg(feature = "router")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goMQ0g0aBlGkZxoHSCg4g0dxpGQdBogoMgwg2CYLCIPg8C+A4FikcBhiAIBkgoTQ2C4MAxCAMQ4EgNooiqL4gi6MBojKNAxDCN45jsSJIj8L5BGiQ4xjMIhNDENY6DAdoSimUJElORZVlcNwuDgNAgmUMQ3GGaJoDAIJwDALQ1C4Ng2nGT5RmGRpWiOdp4DQLgzDQYQ4CCh5ynELQxDGg45iSXp7gE",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHIeINDKER5g0M4Rh0IojD4PAvgOBYoHAYYLCAZINE0MwgDEMhohqKAvi6Cw+gE",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawer",
        contributors = "danielbayley"
    ))]
    Rows2,
    #[cfg(feature = "rows_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NHcaRkHQaIKDEOINGgZRpGcaB0hiGgggkIoRD4PAvgOBYoHAYYWCAZIKE0MgxCAORIieKYuhaLYvGiMYzjUIAxDWOQiigL48GgPoBA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers",
        contributors = "danielbayley"
    ))]
    Rows3,
    #[cfg(feature = "rows_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIBoGUaRnGgdIJDEOIMHcaRkHQaIYhoIByggIgyCIPg8C+A4FikcBhh8IBkgkTQyDEIA3C4NRIguKQvi+H4ujAaIyjSNggDEMo8iiKpAGiQoxjMIo1jcMQ2jqS4+k4PoBA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers,grill",
        contributors = "danielbayley"
    ))]
    Rows4,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxGEOQghEMAghQMYShIIg+DwL4DgWG4egaCIKgwNBhDENoNimFIWiqKoahyIYbGMaRyGMbBlCAYx4gkNQijoeYJDEOY/HKQowC+NI2jgPoBA",
        categories = "development,social",
        tags = "feed,subscribe,news,updates,notifications,content,blog,articles,broadcast,syndication,reader,channels,posts,publishing,digest,alert,following,inbox,newsletter,weblog,podcast",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Rss,
    #[cfg(feature = "ruler_dimension_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDUdgtDMIg+DwL4DgWFYYgaCIKDENIOhCEoUhaG4agSHIJgsOIhhGE4VheKIngWB4qDIIA4FYNIkjGGYCiiNYKDKNw2EgMo8iaP40h0TZDjiOpIjKSopgoNotiOMJJHIZRjHQIByHiCZHCAeYJDGYx3GkZIFmIMAiCCYQimMaBlGkZxoHSCQ4jyW5dD6AQ",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "jguddas,karsa-mistmere"
    ))]
    RulerDimensionLine,
    #[cfg(feature = "ruler")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELgzCAMQ1g4YQyC4NAghWFwwCCGwxhwIAzhYbAthUNoYC4NoUhaJ4ah8MQtiGGhMhUNwgDgLg3ioNIehmHobh2HIwiKJYkiiOo8haPouiCKwwFoIg+DwL4DgWUZUgaCAiG0MQ0C4NYQhWXwyiSUJSleVoEliCZbg2Xw5l6GJklGU5pmiBYHmuN5fDacJjDKZZ0lWAppniWgxDecIRn2cpmnWAQ",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Ruler,
    #[cfg(feature = "russian_ruble")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQxGgOBhDQIITDAIIWhgLQ4EgOR2DEOAiD4PAvgOBYiiWBoIgqDAxDWEIhiOKA+gE",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    RussianRuble,
    #[cfg(feature = "sailboat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYMQ1CIPg8C+A4FhSF4GgiCg3g4MhhDQIIig2DQxC0NIoCAMYrCCJYticMRoDENhhiyLIvjeLYiiSLorioNB6hOFYahmBIbgkTQ5C6EQ5g4Lg0jWOo5isLg1DKJ5MDkMxsksOQ3h4OAuDkOBBlOPosDKJgzEgNI2i2OQtC4OAyikMZWDYN5ChSFpHD6AQ",
        categories = "transportation,travel",
        tags = "ship,boat,harbor,harbour,dock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Sailboat,
    #[cfg(feature = "salad")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgxGgMQwCIPg8C+A4FhSF4GgiCgxDKDQxGEOQgiMMAgiaJg5C0ORIDOIokieMYpiQWoThWGoZgSG4JE0MQxC4Mw4CCHhhDILg0g2R4ykMLZHC0NAuDeDJGkiVJLDEIAzC4MgtkaUpJlWSookOWZHDeTQ2DOYJrmOWJaDODJvlOSpWm0LY/m6UZJDWWJGnyVwgC4MJqniNoWjqOYFgeCRtDGhIfDST6GjiAo6ouHQwC6Iw3lsNRBloOYjqCJZLkiERjiaUZah+Pw0j8NZVpOiIB",
        categories = "food-beverage,emoji",
        tags = "food,vegetarian,dish,restaurant,course,meal,side,vegetables,health",
        contributors = "kemie"
    ))]
    Salad,
    #[cfg(feature = "sandwich")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyC4Mw3CAMQxC4MgyDMIA4g0NwyC0NguDeHxhDIIIiDAIIlDGIwuDUMQ2iYbIYg6KIdh8NwiD4PAvgOBY3jqBoICITQyigMQ1GGQ4mkiRwxHYMpGhCSInC2QxoC0NYTDWNo4j2PIEj6CRNhaRJOiiJZllKEJMmOUJJhAaA5lmOZdlyBYHgkbYdDaD5ECCHQxhYNAuDaIYjmuJYMDgLQuDQbAzC6RAtoAMpwlsPByGUYx0CAeYJhEIggHcaRkgWCQyDCnhoGUaRnGgdIJDSnhyHinKerIIqSjcL6WpgPoBA",
        categories = "food-beverage",
        tags = "food,snack,dish,restaurant,lunch,meal",
        contributors = "kemie,jamiemlaw"
    ))]
    Sandwich,
    #[cfg(feature = "satellite_dish")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGENwuDMMQghGEwgDCGIag6DQwFoIg+DwL4DgWIYkgaCAiG0OYNDUIAzC0M4giKJ4mgSKIJE0MQ3g0MxhDYIJAhmQwtDaRYziON42gWB45DKFAxDMQYchyQ4bhSMohkmJYBA",
        categories = "connectivity,devices,multimedia",
        tags = "antenna,receiver,dish aerial,saucer",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SatelliteDish,
    #[cfg(feature = "satellite")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CANoOC2DQxDQOITC6FQ4GEMQuDIMIPh2H4PDAIIlDALYdDcMA0iYTIRDMNQyCANQuDaFoch6IAgiKO4niaQIqiwTA5g6PAwg4Ig+DwL4DgWS5OgaCAiE0MYRg8N5GDEOY0kqTJRlCBJSgmC5ZiGSIPhSFggmqG49iGOokkGQQuisNBsC0MoeDkNggnoMp8jmI48nGQIlDGKZ1iyLoMloNJJkuTZimGBYHgkTZcDIMRhn2fY/igNgtDaXqSk+ApipaVJFjGMwxkiN5uoWb6GrSQp3n+fJ+nsNqCjus6foSdomGyj6AqGxaBrOv60iitp4riobQr2cKDp+ibCDAeqkmCAQA",
        categories = "connectivity,science",
        tags = "space station,orbit,transmitter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Satellite,
    #[cfg(feature = "saudi_riyal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDAIAxDkLg1C0NYRg4LgyCIPg8C+A4FhqHYGggIhNDENIVDQdgxDGFwyGEMYOCCDYyhYMg0DILg5DcTIMg6FIYhqHIEGiH5CgeCYLjgNw4g6EAzDWL4UDUNA5C2KwzDYMxBDIIJbjODQxl8NhWj+G4gkSBZGiOPJgCANIODOEYZmWQg+gEA",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "null78,jguddas"
    ))]
    SaudiRiyal,
    #[cfg(feature = "save_all")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYMxhDEIITg2FoUhQaA1CIPg8C+A4Fh2IIGgiCgxDiFA4HYLQ2hKGIXDALQxjIaIsi6FQghaMoUHYNoch6I4igSJIJguKAyDISA0GEMoOjmT4zDILQyFaPodh+Q5CgWB5FiiJ5Mk6F5RlMVpLk2TZig6UxoDkLgxDcMpgmiUIUC4NAxDQLg1DgNhsDILg4DKR6AoIQZnk+DYTkgIA2oQOBWDGLaHmKUwuDCih6j+WIhgEA",
        categories = "text,files",
        tags = "floppy disks,copy",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    SaveAll,
    #[cfg(feature = "save_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIIMEgOBhDGDggDCFYVC2EwxHYNwiD4PAvgOBYfiKBoIgoMQ0CAOBoDGHogiWJIEiaCYLDcIAyhsLQ0i+IYzjKBYHgkbQyjiOIWDIMI9jGAozkKCpJC4NITlGUxBkWRYWhaGg5jgMRIDUYZYheWwtDKZhWmGY5ahQLg1DmGZSi6H4+iOTZBicTQyDmboODGbhzDUIKCioNZLj+d40gqXQzGgNguDKYpGmyGpSC4NhsDMLg4CCmoRmuF4TpeDpSFYMaGnSMYBA",
        categories = "text,files",
        tags = "floppy disk,unsalvageable",
        contributors = "AnnaSasDev"
    ))]
    SaveOff,
    #[cfg(feature = "save")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgyCAMxhg+DwwCCFQxCAMQuDQLg2GwMwuDiEIhhIIIUhaGQgh2GYbFYMQ5iWJ4XC2ExIDWMYojMMo0FaN4TjmKY7DIegiD4PAvgOBZGkmBoIgoMQ3iYMR2C0NxhhiGIVloLQxlwSA4leKZahaXIZHYN5FkeTJLgSTYJE2UQzHYNJhlmQJYGiaJGkibQ+gE",
        categories = "text,files",
        tags = "floppy disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Save,
    #[cfg(feature = "scale_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANx2DEMRhDEIITDAIIWhaEwxGiEAiD4PAvgOBYfiKBoIgoNQuDIOQzhQOAuDcMA3hSGgzh6IIlh8YxpHIYxsGUIBygkMgiCAYx4gkMQ5kUYx5kmS4fC+O49j+Oo8j6QJHgkNZMk4IpckGQ43lKV5VgE",
        categories = "design",
        tags = "gizmo,transform,size,axis",
        contributors = "lscheibel,ericfennis,jguddas"
    ))]
    Scale3D,
    #[cfg(feature = "scale")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhSF4GggIhtDEOQgDiDohGENQgiYMAgikMQtDaKh6FYN4ThWGoZgSG4JE0MwgDcaAxGEMQ3CCQYqkWKQ4C2DZEkSKZNiEIAyj6M4WjeNoFgeCRtiaIo7DiJYnkaQ4ti+MZTjWAo3liCpCDIMY+DCZpVgE",
        categories = "navigation,science,finance",
        tags = "balance,legal,license,right,rule,law,justice,weight,measure,compare,judge,fair,ethics,decision",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Scale,
    #[cfg(feature = "scaling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzEgNRhg2DQwCCFQwC2Ex2DENISCCFIWiGExohyHogheH4ZHYLQ3CIPg8C+A4Fi+MoGgiCocCAMQ1EgOYrDWLowjWNIEjaCYLDaDhoDUdpAi+MZFkSBYHkcMgxg4IA5jqTpClGAQ",
        categories = "design",
        tags = "scale,resize,design",
        contributors = "karsa-mistmere"
    ))]
    Scaling,
    #[cfg(feature = "scan_barcode")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2Ao8icTQ4g0dgxDCS44iOX5PmEMYSDeZZnl2OZOiaPopmSZpok2AQA",
        categories = "shopping,devices",
        tags = "checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer",
        contributors = "danielbayley"
    ))]
    ScanBarcode,
    #[cfg(feature = "scan_eye")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgHKCQxCIIBjHiaIdmseZuksL5gmKZJOiaPgxDgLg5DQNAgDEMguDMMxhkGV5XC0Lg2DaDQuDWj6RhWlAtDEMwuDimoxoGiaUCCjKODekKSp6gaYpoOIWnOTYBA",
        categories = "photography,multimedia,accessibility,security,devices,account",
        tags = "preview,zoom,expand,fullscreen,gallery,image,camera,watch,surveillance,retina,focus,lens,biometric,identification,authentication,access,login",
        contributors = "danielbayley"
    ))]
    ScanEye,
    #[cfg(feature = "scan_face")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2Ao8icTQ4CAMQ0HMMQuDWLw0muRw0lyTI5k6Jo+DkIA5GgLgwDGS44iOX5PmEMZqniep8l2coBA",
        categories = "account,security,devices,social",
        tags = "face,biometric,identification,authentication,2fa,access,login,dashed",
        contributors = "karsa-mistmere"
    ))]
    ScanFace,
    #[cfg(feature = "scan_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGgMhhDIIITDAIIWDGFIUHYMgiD4PAvgOBYfiKBoIgoMoZgyHIShqFoYC2EwyGiMYeiCJYkgSJoJE0MwgDcVg1i2FYXCCGQyjGEI2iGOo5gWB48g2KRIkKMpFjCSAyHaNYfkyI4CjqUIKDcLg4DIOJGDMLgwDcQY+j6L5GkaEw4C4Nw2DQYZvlecg0C6DITn4MwwDYbAtmoNA0g2ag2hGKp8DELQxC6iQ5hcepLjiAQ",
        categories = "medical",
        tags = "health,heart rate,pulse,monitoring,healthiness,screening,dashed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ScanHeart,
    #[cfg(feature = "scan_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2Ao8icTYqDGGwxDCS44iOAQ",
        categories = "devices,shopping",
        tags = "checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer,qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ScanLine,
    #[cfg(feature = "scan_qr_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDIdg0GEMYOCAMIWg4LYUDEaAtDQIg+DwL4DgWIYkgaCIKgwIAzGgMhhDIIIxheF4UjGEAyiCIoniaBIogmC4NDgVg3jqI4+j2BYHkAMobDcdovjeGI1C2N4djmIZHiWAo+kuCgzCANxWDWMIylOFQylWLpGjyXJKikTYNgwaAuDAMZskibo/gqDZNEiZJSjSGZphCVZ4lschlGMdAgHIeIJncIB5gmRQgo8IqVHcaRkgWCQ1CIIBoGUaRnGgdKekaiaLD6AQA",
        categories = "account,shopping,devices,security",
        tags = "barcode,scan,qrcode,url,information,digital,scanner",
        contributors = "jguddas,vexkiddy"
    ))]
    ScanQrCode,
    #[cfg(feature = "scan_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgGMeIJDGHZmHmaZrHKCQzksL5gmKZJOiaCRtDENggnwLQxC4OaAoKc5NgE",
        categories = "photography,multimedia,accessibility",
        tags = "preview,zoom,expand,fullscreen,gallery,image,focus,lens",
        contributors = "danielbayley"
    ))]
    ScanSearch,
    #[cfg(feature = "scan_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2Ao8icTYqDgaA4kuOIjl+T5hioMYbDEMJnl6JZQmIIAxDYaA2nKOQ+gE",
        categories = "text,devices",
        tags = "recognition,read,translate,copy,lines",
        contributors = "danielbayley"
    ))]
    ScanText,
    #[cfg(feature = "scan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2AQ",
        categories = "devices,shopping,security,social,gaming",
        tags = "qr-code,barcode,checkout,augmented reality,ar,target,surveillance,camera,lens,focus,frame,select,box,boundary,bounds,area,square,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    Scan,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyDEdgtDMYQyg4IAwheFwtg0MB2DMIg+DwL4DgWIYkgaCIKDEOAgDQLg5DMMxWg+IIiieJoEiiCRtg0NggDcLgxDANoSC4Nw5hSFoYhgMQgDGRg4iwMBMDKGA2jWI45jiBYHjuPgxDGRQ1hUMpBDQNxhk2TZLhoLg0DgLg4DUNBWDGSJkhmbJkGgMQ2kmFZshgMgtDKEQ1mmTp5m2bwtnENQzEyK5ODGWI3gKOZdgqPoujCMo0iGWYlDwYxpHIYxsGUIBjHiCQxDIIggHKCavqoeYJDmWKkqaqA+gEA",
        categories = "buildings,navigation",
        tags = "building,education,childhood,university,learning,campus,scholar,student,lecture,degree,course,academia,study,knowledge,classroom,research,diploma,graduation,professor,tutorial,homework,assignment,exam",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NAyCAOYNg8OAgDEMgiD4PAvgOBYZGMaRyGMbBlCAYx5gkOAiCAcoJheJR4gkNIYhqH4hiOGYcgaCAiG0MQ0CANgtDgLg1hSQ5FjOG4EGiHogiKJIsCKLhjjAIoyiWJwiDENpJjWT44kuB4JE0MQwC6FI+meFY/DGKYZkqHYCmGO5kDaFQyGgLYXm+OZggWYoKDKD4Wnme4an2AQA",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDgLgxDIIIoioIIuiqGIaiCIofiGI4lCITQyDAIA0iyKYrDENQuDgOIyC+NBohmHIGgiFYLDGR4JguDYPhEIoThmG4Ek6NoiiSJgxDSRYvmQOAgjyaQwkiSg+gE",
        categories = "text,design,tools",
        tags = "cut,snip,chop,stationery,crafts",
        contributors = "colebemis,ericfennis"
    ))]
    Scissors,
    #[cfg(feature = "scooter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA0GgLQzC4NRsDIIAxDELgwDUIg+DwL4DgWHohgaCIKDYLg5DWFw3GgNQuDENAyGOEwyDMIAwCCKQ1C0Lg0DANoXhoNgzj0OQxDYYYoiuS44k6DYvDMNI8i8MAwDmHYfiSHhjGkchjGwZQgGMeYJDEN4TCKYx4mYOZpCAcoJDKaYeC+XZfmGXJemCYpkmaaIcnCcpvGObAiDSdIfnefA+gEA",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey,transport,electric,ride,urban,commute,speed",
        contributors = "Ahmed-Dghaies,karsa-mistmere"
    ))]
    Scooter,
    #[cfg(feature = "screen_share_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAzEgNBhDIIITDAIIWDALYTDIdgxDCEoUheIoWhsaAxDaIIViOFIaHYLQzCIPg8C+A4FjKNYGgiCg4hQMRoDiMYzjiN4EjmCYLhMMQ3HYNJBjSRZEgWB4JG0MoTDMLQ1CANZOkOApFlMIhtkqDpbluXZQgEA",
        categories = "connectivity,devices,communication",
        tags = "desktop,disconnect,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShareOff,
    #[cfg(feature = "screen_share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAzEgNBhDIIITDAIIWDALYTDIdgxDCEoUheIoWhsaAxDaIIViOFIaHYLQzCIPg8C+A4FjKNYGgiCg4hQMRoDiMYzjiN4EjmCYLhMMQ3HYNJBjSRZEgWB4JG2SggjwNQtDWTpDgKRZTgqVgzGgNR2luMpPjaAQ",
        categories = "connectivity,devices,communication",
        tags = "host,desktop,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShare,
    #[cfg(feature = "scroll_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDIaAtDUIg+DwL4DgWFYYgaCIKgwIA4hGE4VheBBohqJoHgmCw5g4NxWDUYQyCCMwwCCNgwC0Mo6EgNIUhaG4ogWKoKDiNAxGiD4yjSN5NjaOwyHYLQxGEMYOk6N5TlMSAxlSVpWjiWZfHaVIzjWVwxC0NI3jCS5nmCao3HYMpVleYY2l8aAzj+JYZgE",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ScrollText,
    #[cfg(feature = "scroll")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDcVg1GEMgghQMAghcMAtDKGxIDQIg+DwL4DgWIYkgaCIKDiFQxGgMQyhOFYYjOF4cDIdgtDEYQxg6NIYjmORIDGOo8jyGY/kUdo6hSFo9DELQ0hiEYxk2RpQhgdowkWPoXkUaAziCIonD6AQA",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Scroll,
    #[cfg(feature = "search_alert")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCKCR5gyDoQHKDA4CIPg8C+A4FgeGhwGEdBoCAZIMG0MgxCCKQtDQLgzi2L4ZhuIYjiCIokiYIhNg4IA3HYNIzC+NRojeI4lgyPIqDENRoC4MIPhqQ44D6AQ",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,warning,alert,error,anomaly,lens",
        contributors = "colebemis,ericfennis,jguddas,Veatec22"
    ))]
    SearchAlert,
    #[cfg(feature = "search_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMQxCAMoQCANAtDQIg+DwL4DgWGBjGkchjGwZQgGMeYJg4IggHKCQ4ikYx4icMYXhmHogiKGIbgaCIKDKD49hULgzkAM4zhqBBoD6AQ",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick,lens",
        contributors = "danielbayley"
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMIIMC4NQgDILQyhCFIUhAIg+DwL4DgWG4egaCIKDIMYSDELQ0C4M4piuGociGIIEiKCRNDkIA4hAIA3g4MRsDKEoZhuHYzhsYxpHIYxsGUIBjHmCQxDEIggHKCQ4lMYx4lCUpDkeSZLD6AQ",
        categories = "text,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>,lens",
        contributors = "danielbayley,jguddas"
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FhcYxpHIYxsGUIBjHiCQxDEIojHmJooCAcoJDiFoYh2H4hheGoGgiCgyDEII8C0NAuDOQJCjKGYEGgPoBA",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/,lens",
        contributors = "danielbayley"
    ))]
    SearchSlash,
    #[cfg(feature = "search_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FheGoGgiCoRg+IQghSFoYh2FxjGkchjGwZQgGMeIJDEMQijAeYzjUIBygkOImC+Kosi6HIEh6CRtDIMQgkkLQ0C4M5Nk+P4ogEA",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,lens",
        contributors = "danielbayley"
    ))]
    SearchX,
    #[cfg(feature = "search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIIMC0NAuDMNIQhINAiD4PAvgOBYZGMaRyGMbBlCAYx4gkMQxCKJR5iiKggHKCQ4hiGofiGIw+gEA",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,lens",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    Search,
    #[cfg(feature = "section")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1GENAgDMIAwhSFAtDiFBjhWEYZhOGQ3hCEoWhUMYYhQIg+DwL4DgWKotgaCIKhkMQ5iKE4VjkIIZDCGwtDSGAtDOQYhhGOIWDGO4oiqLIEGgPoBA",
        categories = "text",
        tags = "mark,typography,punctuation,legal,type,text,prose,symbol",
        contributors = "gurtt,karsa-mistmere"
    ))]
    Section,
    #[cfg(feature = "send_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NwxDQIIMDANA4GELg0DkOIXhkIAwh2HQtC4Ng4gwNgyDcbAyC4OA0DMIA3iKJxhDIII0h6Hgxh8MQuDMOQ2GwLYqiyNIwiYN4WhiGpJh+NwgiIOIqkYbAxDgLYaDWFg1C4NZMkyIQ5hMegiD4PAvgOBZkmeBoIgoNggDEMhoDENpjmWag+gEA",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SendHorizontal,
    #[cfg(feature = "send_to_back")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIB4gmC4NHKDwiDKDRoGUaRnGgdIJDiDR3GkZB0GiHgiD4PAvgOBYoiuBoTgmFoOjGF4ZhuHQih8IIhiOJY5g2CIVieKYuigcBhiQIBkgkTQ3CCCx2DEYQyCCVAwCCV5XlQMhoDGQwvkeJJGkgaJKkyCwgDeXZTlWWJuDGbQylGX5hGgPoB",
        categories = "design,layout",
        tags = "bring,send,move,under,back,backwards,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    SendToBack,
    #[cfg(feature = "send")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1DMNggDIMQuDYOA2GGDoOCAMIch4Lg5DMNwtC4MAyDQbA2g4LQxDmGQ0DmKowhGHY1iQNgzDWN45GyLA5CCKg1hkNYbjWHIkiaDYhDcbA3iAMwgDMLgxDgYQyhKHodDEIIUDEMZXl0MR6CIPg8C+A4FmWaIGggIhthMLg4DUNISlMNIjDEMIgnSeZPDmZJmmsPoB",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Send,
    #[cfg(feature = "separator_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIIMC0NAgDSEIQCIPg8C+A4FheGoGggIhNDODgyGgMQ4haGIdhyBIegkbQ4CCMIThGNIohmLA+gEA",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhSF4GggIhtDENggh8IA0C2JImhOFYahmBIbgkbQ4CAOIliONA0iiFosD6AQ",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[cfg(feature = "server_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg4DUMggDENAuDcNwyC0LgzDgMwuDkMgzCIPg8C+A4FiOJoGggIhNDGHYTDiEoUhYMhhDMII3DCEggDALQyC4Mg5DYLQ1C4NQ0DQbIZhsM4Zh+IYjiWBBoiiU4HgmC4vDSMQ5kAMg4hqHJOiCIokimVYFleCoukaNwxkUNpCksOIYh4Mg0jaOI8jsMY+kCcpFkcNJllKJ4ClaK4LjOF4Sg2D4/k+c5QmaU5oiqWIThWjJsjCdodkyhJnoeaYrE2FA1o0SJ4hGEY6jqfYYDIVqrCCrZ7DGtY+GgMQ2GGrJ7q+ta1HatK2q+Pq1GiGQ1qGlajpeLKnjKqq+sKro8sgMrEtWxp7qyu69r+146rEdgtsWwLYrGypGs2hopmoTQ2hIOBoC4MAxu6VLPvG8w2va+L6paahtl0Mpfo2DoQmOTZhpOhb7vCicGwinJbwzDsCgE",
        categories = "development,devices",
        tags = "cloud,storage,computing,cog,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    ServerCog,
    #[cfg(feature = "server_crash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQwEgNBhDIIITDAIIWDELQyhoVoRhOFYXg2FIaGgMQ2hKFIhhiKQyHaHophaGIahQaIaCIPg8C+A4FjiO4GgiCoMDENIQiiIIxjOLYvkeIYfiWJ4fiqTYaHYLZLlIMIaiSNo4jqBBoj2X4HgmCwgDYaAuDAMY3jmPphgWY5Bg0OJomqbJejyApikAbQxDOZpWmYaA2GygQ2neboBA",
        categories = "development,devices",
        tags = "cloud,storage,problem,error",
        contributors = "mittalyashu,ericfennis"
    ))]
    ServerCrash,
    #[cfg(feature = "server_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMhoDEMxhDKDQgDCFQgDGFAyHYNIShSFoWDELYTg4LQ1CIPg8C+A4FimLIGgiCgxiGFgyC4NYNjcQ4khqN4UDUdoRjyIIXiQaA2HqKIqi+LoEjCCRNDKEwxDcdgtDGHoTkQMIjiMaJXkqK5Ok2BYHlANIYh2Q4XlyJIclmbJFg2Dw2jcbIijeeQ1C0OJ8EgNJJimYotgKTpmgoNoYDgaAuDAMZhkyhZljEbY8DKNQwpCY4B",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    ServerOff,
    #[cfg(feature = "server")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB3GkZB0GiCgyDCDRoGUaRnGgdIKDiDYJguDRyHiFINiWIg+DwL4DgWKotgaJImgeIYMg6EIShSFgghiGoch6J4ziEMQ0CKKosgQdIqGwaRuGUIB5DKCg2icMZTieUgiDYLgwDGIJWlqRorkyTpLk2Tx5mAMYflCWZrlWVwgHiWZbl2YgvmQZQ+gE",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDcSA1CIPg8C+A4FhSF4GgiCgxDkIA3GgLQ5hOFYahQYxpHIYxsGUIBygkMwiCAYx4gmD4zGMeY3DeJQvimK4tiiKosi6MAijKNI2CKPY0juTI+kCRQ+gEA",
        categories = "account",
        tags = "cog,edit,gear,preferences,slider",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4Ng3DEIA0C4MQzDYYQyC4Mw0CCGIaCAMIfCCEISDYNQ5iGHYbimIYgiAM4ZDGJwxC4OQxDWHIZiqOYsiKOAzDOEQuDCP4+jqHotjyLw4DOEIriuSAxC2HZAhKQ5Ak6O5IDALYvkyMo0jaRZilALYkiaKI7k+LJchkMgtjONY3liR4hlGU5lkKRJzhuWofmySwxEGe48hANoZjehgwDUMYXmmWZJjAOZvmANQiD4PAvgOBaXGMaRyGMbBlCAYx5gkMQyCIIBygkM6pGMeKmqilwvp2n6hD6AQA",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4MwgDEMBhC4N4TCAMIWg8LQuDYMg2C0MQuDANw5EwMYgDQIAzhKFA3hiFwxg8LgxDkOIaDANAziUNoNCCDA5iuFYXi+Gg1DcMoxDEWgiD4PAvgOBZMHIZRjHQIB5gkMQ0CIIB4gkM5bHcaRkgWCQ3lsaBlGkZxoHSZZbHKXQiDGS5NlKVJMGMaRyGMbBlCAY5XnKFA1m+XguoSf5xDGg50C+eZ7n0PoBA",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = "danielbayley"
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEOIOGMeYLDUIg+DwL4DgWB4bh6BoIhCCw2hWFwiDEMoOgqDIahyIoggKBIjg+KQxDmFYRiqFIJguDYbh2NYzGwaRuggeAyhINQuDSLAgHkMYSDMLg1DGDh5kuKg3k6OggHiUwiDiVo6kKRpIhuaIIlqEgwl6DphkyTpYmCW5kDWX5SiaVpYmeRxlD6AQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQzCIPg8C+A4FhSF4GggIhtDENggDYLQ0iOIwgDSE4VhqGYEhuCRNDQIIMHYOBhg2DQwCCOY5jcaIMjaDo6kKPAtg8LQ4imFotD6AQA",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIBoGUaRnGgdIKDEOINHIeIKgwIB3GkZB0GiFYXCCGgiDODYJicIg+DwL4DgWLRsGkbhliUMYKiiJQyhsMYpjwIg5imOJBiyLozjWMo0jYeJEjoeZEDENYNHiQAyj4IB5kCUpGC+SBlkqNZZkSQo3gqZZaj2VJAkKLZekuYY2mmC5Yk2FZTjud5DmeXZfD6AQ",
        categories = "text,files",
        tags = "spreadsheets,table,excel",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Sheet,
    #[cfg(feature = "shell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDEYQyCCEgwg6Dgtg2FYNhkIIVDEIA4h0IA2iOIoeC0MYUiCK4Vh6Dokh6MYih8MQtDKLYODELg5DOOY7j2OIfDILg0DILQ3C4MoSkuM4WDMLgxiSQw0DQIg+DwL4DgUPoBA",
        categories = "animals,development,nature,science,travel,food-beverage,home",
        tags = "beach,sand,holiday,sealife,fossil,ammonite,biology,ocean,terminal,command line,session,bash,zsh,roll,wrap,chewing gum,bubble gum,sweet,sugar,hosepipe,carpet,string,spiral,spinner,hypnotise,hypnosis",
        contributors = "danielbayley"
    ))]
    Shell,
    #[cfg(feature = "shelving_unit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMFYORhDGDggDCFYVC0MYZEiEYThOFoghmDh2DMIg+DwL4DgWJ4qgaCIKDENggDIMB2C0M4ShSIIYhoMRoC0Mo5h+FwwiIMYkiaKItiyBIugkTY0jMMhWDKSYpk2TIFgeTw0g4MhojGVpLgKTZbgqXY0mANpiliZJai8TZoHaNJsiubpOmcIA0mqdRoD6AQ",
        categories = "home",
        tags = "ledge,rack,storage,inventory,furniture,sill,shelves,shelf,organize,display,store,arrange,unit,cabinet,fixture,retail,warehouse",
        contributors = "karsa-mistmere"
    ))]
    ShelvingUnit,
    #[cfg(feature = "shield_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCgxkcOB2DSdp4n2fIEn6CRNoGDg2GgLowoaeaKD6AQ",
        categories = "account,security,development,notifications,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,warning,emergency,attention,urgent,alarm,crest,bravery,strength,tough,attacked,damaged,injured,hit,expired,disabled,inactive,error,exclamation mark,!",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldAlert,
    #[cfg(feature = "shield_ban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGggIhtmUMg0DOb5Ml6ZQzm4MZSDSVJ2nifQ+gE",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,cancel,error,crest,bravery,attacked,damaged,injured,hit,expired,eliminated,disabled,inactive,/",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldBan,
    #[cfg(feature = "shield_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGggIhtm4MZHoSNwtDSdp4n0PoBA",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secured,safety,protection,protected,guardian,guarded,armored,armoured,defense,defence,defended,blocked,threat,prevention,prevented,antivirus,vigilance,vigilant,active,activated,enabled,detection,scanned,found,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audited,admin,verification,verified,certification,certified,tested,passed,qualified,cleared,cleaned,disinfected,uninfected,task,completed,todo,done,ticked,checked,crest,bravery",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldCheck,
    #[cfg(feature = "shield_cog_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDIYwtDMLg4DANgtDELg0DULQ3hILg5DaF4eDkVg2GGDYNDAIIrimGBjDKLAgDQLochmMQ2C4Mg0C0MguDcMooC4MQ3CCGZEjKLZGjWMQwEMMY0DWDYTDiKZFDWRg5CANYokaSZeikdg0CIPg8C+A4FmWaIGgiCpQiAMgzkaOQ1DSRZQnOQw2mOZZngQaJqn+B4JG2bw5nGRg4hQNAzC2cITDMOAzmSZproGBaDm6dJ2kaNKHnIMY5qENKdpSfppgKgptoWm5FDIMIaDaHgupGE6HnylZ/pebKEDGig4oynaPrSkqOp+pqWqmmKrDEOY6DKWAyhkMw5DGjqRDKxo7siurKrwIhtq+sZ3q2j7XpK3KomumbhrANKyomi62nGxKTn2yRjGkchjGwZQgGMeIJkQLg2DkNQiv8ecCDfBMGwgcoJvaZr5vu/Q+gE",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,shieldcog,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere,RajnishKMehta"
    ))]
    ShieldCogCorner,
    #[cfg(feature = "shield_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg5DIOQgDENAuDQNg3C0LgzDgM4ODINAiD4PAvgOBYiiWBoICITYMh6EQ4h4M4Sg0NYWCAOIhiOKIngSKYJiyHQyDINY3jGEodDYMA4jeOYkj2PIFgeCYLkgMA3hINYaDkMYZhsMoZg+IIik6JoCj2UoKhMLg4DSEYtjUN4xl2HJNjuZpRiqC4UmybgymsNIdg+HYbDOdZPnePorDIMJHGOjA1C2HZEnGkJxDYNpFDkNRhDGEggoyjJcC6F4ZDAMRDpQIKLC6RA0hKS6uDEMxWDanKeqCnpcDEYwyp8IIUpAMQur0NrDDQLZ+DcMqcC4MZXsKzq+qGEqsr0MBDmoNadh0OKdtGRAxhGm7etKuYSHqhpliiaBtDmGgwuCNA0hicoanSY52uuebuDO8ISn6bAzmAMqDveOqHGMaRyGMbBlCAYx4gkMZ+lYNwiw8ecSsINqaxccoJoWY8JwvDQ+gE",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere,RajnishKMehta"
    ))]
    ShieldCog,
    #[cfg(feature = "shield_ellipsis")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCo6DEMhoC6MJ2nifZ8gSfoJE2goOoOhQxoeeaLoqBYHo0MYeoKhKGnelZ7gE",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,securing,protecting,guarding,armoring,armouring,defending,blocking,preventing,antivirus,detecting,scanning,finding,auditing,admin,verifying,crest,upgrading,loader,loading,throbber,progress,dots,more,etc,...,…",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldEllipsis,
    #[cfg(feature = "shield_half")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCgxkcMgyFYMp2nifQ+gE",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,logo,sigil,flag,team,faction,fraternity,university,college,academy,school,education,uniform,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,ranking,army,cadet,scout",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldHalf,
    #[cfg(feature = "shield_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCpuDEMhoDadp4n0PoBA",
        categories = "account,security,development,gaming",
        tags = "unshield,cybersecurity,unsecure,unguard,unblock,antivirus,clean,clear,disinfect,patch,fix,stop,cancel,remove,relax,admin,crest,bravery,weakened,damaged,hit,unarm,disable,deactivate,decommission,downgraded,minimum,-",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldMinus,
    #[cfg(feature = "shield_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNDUIA1GEMQgiOEImC2IwxHYNxjhCHwzC6Hw3jEIIzDYNwgDgLg5DSIokCCJpACALo3C4MAxGMMguDMNQtC4OIMDQLg0DiKI7jgNY7C2MA3DGE4VhqGYEhuCRNDmSwwDkIIwDYNQyEEMZKDKboknKdJBhAMQxC4Mg0g2fA4iILgxjie6EkKeYkjGDAwEMMZSDWI4wDiKZYiSaohimiI/imKxhmcNw4n6oKipsMZODAOKKDINg0l+FpjD6AQ",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,interception,threat,prevention,unprevented,antivirus,detection,undetected,exploit,vulnerability,vulnerable,weakness,infected,infection,comprimised,data leak,unaudited,admin,verification,unverified,inactive,cancelled,error,crest,bravery,damaged,injured,hit,expired,eliminated",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShieldOff,
    #[cfg(feature = "shield_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCpuDEMhoDadp4n2fIEn6CRNoIIA5HahZ3nmig+gE",
        categories = "account,security,development,gaming,medical",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,extra,added,professional,enterprise,full,maximum,upgraded,ultra,activate,enable,audit,admin,verification,crest,medic,+",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldPlus,
    #[cfg(feature = "shield_question_mark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCg5lcIA5GEMwgoaXZvC4OJHkWDQyhSh6RDOdp4n2fIEn6CRNDGjQ3GgLowpWeaZD6AQ",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,undetected,scan,find,exploit,vulnerability,vulnerable,weakness,infection,comprimised,data leak,audit,admin,verification,unverified,uncertified,uncertain,unknown,inactive,crest,question mark,?",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ShieldQuestionMark,
    #[cfg(feature = "shield_user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGgiCpODMN4eDGIA5DEYYeh6XYOksMg0DkLgwDAM52nifZ3GMaRyGMbBlCAYx4gkMQyCKoB5qMMamHKCQ0pYL6apyng+gEA",
        categories = "account,security,development",
        tags = "shield,user,admin,protection,protected,safety,guard",
        contributors = "sebinemeth,ksk3110,karsa-mistmere,colebemis"
    ))]
    ShieldUser,
    #[cfg(feature = "shield_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FneeoGggIhtmSNg5hqFw1naeJ9nyBJ+gkbaEhekJvm+iJ5owPoBA",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,prevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,inactive,cancel,error,wrong,false,crest,bravery,attacked,damaged,injured,hit,dead,deceased,expired,eliminated,exterminated",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldX,
    #[cfg(feature = "shield")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDMY4NDULQzC4NQgDeFgthkNg2CAOAuDkNRhDGDggg2DQxC0Lg2DeKwwDEQ4ZheDIWCANIODiN4ODMVg2iSJooiaKgxGMMonjeGgxC6Rw2kwNAtDILg3DKJAuDEN4OleWZCiWSw1kcMBDDENIWiWFQ4l6WYXDEOQgiOXpIimQx6CIPg8C+A4FD6AQA",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Shield,
    #[cfg(feature = "ship_wheel")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwOIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCAMh2DcLg1hqHIiiSIYjiWJwiG0MQ5CANQtDULgyDOPZBDOMQvjMaI1iSJooDIMggg4aAti6MIbkiNpLjeDI6jyO4/kWYJCkeSZak0IoplAMQ0i8VpPmSWQ8kmZ4pDCRZRDMLg3DePZRDmcI0nKNp0DmL5RDISIPleZaCkyOJ1ncMZ2kKfZWjKcYegaCIWhiD4RgwMovhWC4NoqHKZiCAQ",
        categories = "transportation,navigation,travel",
        tags = "steering,rudder,boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "danielbayley"
    ))]
    ShipWheel,
    #[cfg(feature = "ship")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDALgxDgORWDENAiD4PAvgOBYZhyBoIgqDAgDIdgzhiGofh6BIggmCw5g4MxWDcYYNg0MAgjgMAtDKPBIjSNo5kKO42HYNoohuLIrgWB4uDEOQuDMOIkDAQQxDELg2g6WJajqQgyDGDg0GwLQ4hEOA4C0M5ZDMOY1iSQ45C2XAyDSORMDOYhhleWZbn2Xo4DILg4mENwuDeR4ZkmHYCiyTYKjYMRjlkLg1g4LoNmGgqWpqlZfpWPAgDWoZYnmOJYlClqbg4c5YpmJKepKq6BqCDajpmUZCqinqrDGSIqgEA",
        categories = "transportation,navigation,travel",
        tags = "boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip,releases",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Ship,
    #[cfg(feature = "shirt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgzDgIAzC4NA2CAMYVDIYQ0CCGwwCCHgxC2EAwEyEg2DKEYTDYYYoiiHovC0MYOhsMguDIMxsC4NYQhINA3GEMYWh+Q4eC4OQ5C4OA0EgNh2DEMBjiALoyDkIItlYaA4iyVpEkMMgtDIVpPGiNQxDWQJCi+Q5GDkLZJDSOY7C2PY/leagwjGM5gjYMx6CIPg8C+A4FD6AQ",
        categories = "shopping",
        tags = "t-shirt,shopping,store,clothing,clothes",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Shirt,
    #[cfg(feature = "shopping_bag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDAYQ0CCEgwCCFQxC0OIWCIPg8C+A4Fh2IIGgiCgzC6DwzCANguDAMw0GgMQ3C4Nw5DSHIeiOIoEiSCRNieEg1C4NA2DcYQyCCSIVksLZDg4LgyFYMoQkiSoWleVYxDSR5JleS5JC2UYsDaRZclaTJDC0MZQGyYZhC6ZA3EGVZeleMpJEiRp0l8MJqnALg4HqOIfjwPoBA",
        categories = "shopping",
        tags = "ecommerce,cart,purchase,store",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ShoppingBag,
    #[cfg(feature = "shopping_basket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDELQxCAOQiD4PAvgOBYWhmBoIgoMQ5g6EA0C0N4VheHIbgSHYJE0MoiGgMgwieGIriqBYHgkbQzC6DYPg4Lg2CANwuDQYYvi8MAgkqSovDGQRoDkLg4kcIJJkuWAyhGQRsk8N4lkWNIpgKK45CITQ0j2Dg1j0aIMmKNpkjiHhtj6EokiaFo1hqcosgqIY/hKFJ6imAQ",
        categories = "shopping",
        tags = "cart,e-commerce,store,purchase,products,items,ingredients",
        contributors = "danielbayley"
    ))]
    ShoppingBasket,
    #[cfg(feature = "shopping_cart")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIOAiCAcoMDGDxjHmDAyhMPg8C+A4FgeGodgaCIVheE4QhKFILCIMQ5CKGocgSIoaHAYR0GgIBkgwTQyC4MA1CCPI+GgMhsjwNg2CAMY8DQMhhDKQAgDCUZTk8MQuDUOBoDkLg3DiTpQlKYZJC4OQ1C2Vg1DcbJWDaZg3C4NAzEgNQukqLobjSNg+gE",
        categories = "shopping",
        tags = "trolley,cart,basket,e-commerce,store,purchase,products,items,ingredients",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShoppingCart,
    #[cfg(feature = "shovel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg1DYIA0g4Nhhg0NQghYIAwhqGIcDILgxDIMhsC0Lg0DeJg3GEMwgiyG4bDELYSgwMolDCLI4hyMIajILgyDQMxsiYNIlDQNIVg6GJJi+HYfiEMYaHoIg+DwL4DgWVJXgaCIKiyIoVh2TIxmMdgtDODg4hSUJQmIII+DkM4lDcMA3GyZwzDUNZmC6eA1kgMgwheDaAhebYNnMNIanab4QmcMg5mqPqBkqhI6mGSqIiSd55i2fJ5mCbIcjELpziijwzlKVJWgQaJZqyB4JG0OYYheKA4DcOQtrYNw4lOVZaD6AQ",
        categories = "nature,tools,gaming",
        tags = "dig,spade,treasure",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Shovel,
    #[cfg(feature = "shower_head")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CCDAyC4NQgg8NQiD4PAvgOBYWhmBoICITQxDOEAgDaEBhDQLg5hGKIqCAMIui4LQ3CAN4VheHIbgSHYJiCEY+CAMYUhaGI6jmBYHjwMYMDENx2C4MAxjaRIagKOpIh8MYvDENpOlCUo4lWR4eiAM5ADOXZRkOYIcleIA2kAMJol+RZhjuWAxhKcZPmmN50myY5MkANJymqfpWmMMpaDGhJ9hqAQ",
        categories = "home,travel",
        tags = "shower,bath,bathroom,amenities,services",
        contributors = "karsa-mistmere"
    ))]
    ShowerHead,
    #[cfg(feature = "shredder")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQzFYNBhDIIITDAIIWDGFAtDIaA4hILoMDKIIXiSGQxC4NwwDaKIqGwMwuDUOA4CCL4xDgQYiiGI4WhiFIWDgdg1CIPg8C+A4FkSR4GgiCgxiGQRhiaJI8g2DRokKRJGgQaJJluB4JE0MYWDIMh2C2WJFkqXYFl+TYMDEOZmDKQ5plua5LmAMYzDIMJmDOdJakiApekwTYTg4aJ8oCaqDmyhQ2j6fqLnaAQA",
        categories = "mail,files",
        tags = "file,paper,tear,cut,delete,destroy,remove,erase,document,destruction,secure,security,confidential,data,trash,dispose,disposal,information,waste,permanent",
        contributors = "Alirashidy,colebemis,danielbayley,ericfennis,jguddas,karsa-mistmere"
    ))]
    Shredder,
    #[cfg(feature = "shrimp")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDIaAuDAMQiD4PAvgOBYWhmBoIgoMQzCAMgyGMLg1C2JoOC6Dwtg0Momi2LQuDMOAgDALQyiiJ4vieFIWhiBBohuQYHgmCw0iIYQzC4Mo1kuTY2lEMQtk8Mg3ioNw5DgbAtDaKw3joNgxEGL40leZQ4lcMIOlENIzmkIA4EgMQ1iYYYNg2a5rnmDohniUZ6jYLQ1nMMhhleapSC2YA3FYOIVheHJDgWRYfkgOBhDiKaaDWgJsmuj4/pKApEh6Cw2g4NhjDKbYwkgNJdpCQIagEA",
        categories = "animals",
        tags = "seafood,shellfish,crustacean,prawn,scallop,whelk,arthropod,littleneck,quahog,cherrystone",
        contributors = "karsa-mistmere"
    ))]
    Shrimp,
    #[cfg(feature = "shrink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIIMCANoQG0LQ2hQdg0C4OBtDALYYDgaIeCIPg8C+A4FiOJoGggIhNDmDg5hkVoMhsIAwEiGAyi2Dg1GyFIQiKJIpiiBIqgkTYPjgVg5jQMIghmR4NDkbIVDaQIlkSQ4FgeRoukmSwwjWNwujmLouDMIAzlaQoB",
        categories = "layout,arrows",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Shrink,
    #[cfg(feature = "shrub")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDULgxDcMhhg2DQwCCGwwC0Lg1DgNgtDELg0DENBMDmIAgDEM4gCIPg8C+A4FjKNYGgiCooiyPA1i2DYVjGM44jeBI5gmCw3CAOAuDgQQ2CCUYdi2LYvDiDgwEgMQwlCLA2iyVAxCCSw4GGP4/mKLYdHqQ40kcPoBA",
        categories = "nature",
        tags = "forest,undergrowth,park,nature",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Shrub,
    #[cfg(feature = "shuffle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDQIIQDQLYSCIPg8C+A4FheGoGgiCoMCAMoRhGFIRhaGIdhyBIegkTYjgwaAxC4OQ3DMYYSCAMI6jwMwuDMLYzDcbA1C4NA1hMOAuDaOIkjuOwxCCPpAkISAyDKKIZiyK4FgeLojDaMo0DcMpNhCT4OlKS4iC6WIXlqG4CiyXgii+MA4GgLQ2C4MA0DGZo8lALZTkELg4GwLY/DUOaJkeWYqgEA",
        categories = "multimedia,arrows",
        tags = "music,random,reorder",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Shuffle,
    #[cfg(feature = "sigma")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA3FYNRhDEIITDAIIWDALQxhoSA2C6EYfh+F4jhkLg0C4OBsicNQgDYYQyCCMIYhSIwyiYbAtiuLRhiGLIzhaJooEgMQ3hKNI/hSGh2C0MgiD4PAvgOBQ+gEA",
        categories = "text,math,science",
        tags = "sum,calculate,formula,math,enumeration,enumerate",
        contributors = "mittalyashu,johnletey,ericfennis"
    ))]
    Sigma,
    #[cfg(feature = "signal_high")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGgLgwDEIg+DwL4DgWFYYgaCIKDeDQwHYLQ0hSFobhqBIcgkTQxgyDoiDiJYXimKIFgeKwxh+DhWjGFYzhmAQA",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalHigh,
    #[cfg(feature = "signal_low")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGgLgwDEIg+DwL4DgWFYYgaCIKDeDQwHYLQ0hSFobD6AQA",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalLow,
    #[cfg(feature = "signal_medium")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGgLgwDEIg+DwL4DgWFYYgaCIKDeDQwHYLQ0hSFobhqBIcgkTQxgyDoiDiJYXikPoBA",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalMedium,
    #[cfg(feature = "signal_zero")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGgLgwDEIg+DwL4DgUPoBA",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g,lost",
        contributors = "ericfennis,azdle"
    ))]
    SignalZero,
    #[cfg(feature = "signal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGgLgwDEIg+DwL4DgWFYYgaCIKDeDQwHYLQ0hSFobhqBIcgkTQxgyDoiDiJYXimKIFgeKwxh+DhWjGFYzhmAopjeCgygwNB2DENoyieAQA",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    Signal,
    #[cfg(feature = "signature")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIAxDcLQyC4MQ1DYLQxC4OA2DgQQuDWHggDCIYjDEOIOh8NR2h4YYNg2IoiDGF4OGiEYsg6I4wheFxjDCEYeDQNQtDMLg5DmMZDDmEA4h4LQ0jaLo4iMNRjDSEw1DOI5VDeQIXhgMg5DUIIfDcMA4heQw1GGEphmuJI3kMM4NkMMg4DQIg+DwL4DgWeJ7gaCAiE2WIMGiJZ3nmfg+gE",
        categories = "text",
        tags = "text,format,input,contract,autograph,handwriting,sign,cursive,ink,scribble,authorize,personal,agreement,legal,document,identity,authentic,approval,verification,unique",
        contributors = "AnnaSasDev,jguddas"
    ))]
    Signature,
    #[cfg(feature = "signpost_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5EgNBMDIIA3GwMgtDIaA2CIPg8C+A4Fh2IIGgiCgxDQIA1hqFggheE4ZC2G4dh+BBoiKNYHgmC4NDIMhWDQYYvCCDQxCCRYoDAdgxDiHIeiON4FjmCg4i2GZMjOT4BA",
        categories = "arrows,navigation,development,gaming",
        tags = "bidirectional,left,right,east,west",
        contributors = "danielbayley"
    ))]
    SignpostBig,
    #[cfg(feature = "signpost")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg4CIPg8C+A4FhSF4GgiCoMCCEAzhOFYahmBIbgkTQyC4Mw1DSDgwiuLRhDELgyDAN4OjWNwgDCPIOjwLY0DcMA4GyKgwDYLZHDYQYNg2PY9DEIA1C4OAyDgIA2GiDIrDQNBhk6PpRjkNAxDQLg1DgNpGC6SAgkuM46jiNI2jiUI/lELpDkWSptDab5+mCb5ig6QQumWZ5pDYSJUlYOKCk+PgxoaiAtmiah6iKFomD6AQ",
        categories = "arrows,navigation,development,gaming",
        tags = "navigation,direction,arrow,wayfinding,guide,location,pointer,route,indicator,marker,bidirectional,left,right,east,west",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Signpost,
    #[cfg(feature = "siren")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ4HYLQ2GENQghQMINhgMYXDAdg2CIPg8C+A4FiCI4GgiCoUDIMRhDGGIbCCF4uDEaAxDKLYvjGOgxC0MYQiwMggkGMAwC0MpGEgNxhkGQ46kWTB6h+IYmiWBIngkTYrg0Mo1lKIpWlWBYHliDguhQNJmg0OIVl6VIClaY4KkGNpdiCX4km+YooE2NpCHYMZtmCeZXCIbZoDkMg5CCh6JC4NwwDejqQoGeImnGfJzDKHaUGgPoBA",
        categories = "medical",
        tags = "police,ambulance,emergency,security,alert,alarm,light",
        contributors = "karsa-mistmere"
    ))]
    Siren,
    #[cfg(feature = "skip_back")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg5DcMQgDQLgyDgNRBDIIIZDAIIchEMoRDYdgxDIYYZhuHQgDELQzC4MAyDmKguhANRsC0OYOg8LQ1jkOImhqKYeC2LgwDOLAuDQMwyHoIg+DwL4DgWTpRgaCIKDOGgwFYNJNk+VA+gEA",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA0HYMQ2CIPg8C+A4FhSF4GgiCg2C4MAyDmDguDIOA1EEMggikMAgiyLAzCANoQDIYYpiuLY4DOH4hCAMQuDcMQ1GwOQuDkOQ3C0NZFDkOI1iqOIuCCHwwDMLY6DQMwyHqE4VhoPoB",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg1CAMQ3C2DgtDGE4PDEaAxHoIg+DwL4DgWHohgaCAiE0MYPDIMhhDGEAgDCMIyhYMR2hUYQyCCOYxjyEIODYLQzC4MoPDgIJGjGLgwhUMQugyMBBjmO4yjGRgyDAdgxi2L49kmEIch6IIEGiHhjGkchjGwZQgGMeIJikIpsHmbwynEcpvh2H5mmiaplmeaZrncIgxnGbYJDmhZzoOdZhnugA+gEA",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIINg8MgiD4PAvgOBQ+gEA",
        categories = "development,math",
        tags = "divide,division,or,/",
        contributors = "danielbayley"
    ))]
    Slash,
    #[cfg(feature = "slice")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDYLg1DgNhWDEORhg2DQwCCGwxC2GRIDITAxDgLgzDcIAzC4NgzhiDoci+DQzikbAtDmK4sCCNw2iyLoajCHgxC4NAxDSHBMDiDg0CIPg8C+A4FD6AQ",
        categories = "design",
        tags = "cutter,scalpel,knife",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    Slice,
    #[cfg(feature = "sliders_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA1EgMwiD4PAvgOBYThaBoIgoMQyCAMQ5hCEoUhmGIEhqCYLDQIAzHYNIjhWJ4mgWB4pDENofDeLowiWAonjWCgyDGHwyGgLQ5jyMo+jSGxNkKHw5kYNZJheS4okGQw1kYN5UGiM5XE0OIfDCO4TjGVYZkCYZEiKZolgE",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA4GgNAiD4PAvgOBYThaBoIgoMQyCAMgxHYLQ5hKFIZhiBIagmC4eDgVgziWFYpiiBYHisMQ3CAMQ2hCMYngKKY2hwOY6DKL4+jOQI1huC5EiCIg1kiF5KiqCgzjoNI9hOMpThmQhNDWOgwkeW4/l6TJhk8LQ3lIaA+gE",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    SlidersVertical,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggGgZRpGcaB0g0MgwhEeINDWER5hmERyh0IgyiOIYmCIPg8C+A4FiwcBhgsIBkg0TQxDILg2DYNwgDgIAxDCQQyGgNBsC2Oo8j6EIsC+MoLD6AQ",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINgiCAeIJDKDBoGUaRnGgdIJDGEAgHKDgiDGDB3GkZB0GiCQ3CIPg8C+A4FikcBhiQIBkgkTQxDMIA4C4MwyGENwuDSN4+kAIAwkQIAxkaPgzguKQvi+JIujAaIyjQMQ2j8NgglcMgxGEMQxC4N5Zl+YZZkWRZImiYA1DiKIqk8aJRjGMwijUOQuDmSA0C6XQxDWeJIn6gJGmgIAuDCgZ/m2TZwD6AQ",
        categories = "communication,finance,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAch5g4MoSHcaRkHQaIODENISHIeIWhKIwiDWEoVg8Ig+DwL4DgWLRwGGHAgGSDhNDEMggDEOBoC4MAxiyLozhwPoB",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDEdgxGEMQwg6FYVDGDgtDkLYUCIPg8C+A4FiCI4GgiCg4g4NBzDELg1CCDQ0jEIA0C2Mo3h+IYmiAbBpG4ZQgHgMYJDkIpCDKRQuDAMZHHmSQikYIB5kSUY6C+PpAj2P5BkOCQxDWTpQlIeJQmCS5NlOVZGiCWJciWBIngkTQxDYIA1GgNpXjyApxgecwxDmMR2nqbY8gE",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgDENBzDELg1CAMggDSMYzC2Mg0jeGYbiGI4aGwaRuggeAxgwOYQHgMpGC4MAxhAeZFCKRwgHmSpSjsL5AkKP5BkOUQxDWSJWmCTJOlSVpTlCRpYloZQ+gE",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[cfg(feature = "snail")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQzGENgghEMINCCFAxgyFA0CCG4XhYLQ4hYIIMhmIoahYIg+DwL4DgWKhjGkchjGwZQgGMeIJDEMAiCAcoJDiPBjHmOQzimK4wjKNIqi2BoIgqJAxGiGBjDQLodCAOAtDMLoRlkOBWDcYYkiIMYfh0dg2kaLIEGiS5sgeCRNDGIQzg0OQumUNQuDKapMm6BZwk+DJ1DIMAuDkIJ6nyKpri6AQ",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Snail,
    #[cfg(feature = "snowflake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAyDALQxC4Mg1C0MguDUTA2CAMQ4CIPg8C+A4FiCI4GggIhNgwIA0CAOAuDcNQgDaGIzjOH4hiaJYEieCYLi2D4chOFYXhmHYch6IIijyO4FgePgxi2LYShSDoYEyRw2jiS4kgKPJPgoMQ3g4MQtDMLQ2GgLQ0luOpek6KILmOZwzjaEoyDObZMm+PYpDKHAyGiNJGg0OZ6l2JpgG2QYMhGNZ/neDqHGiTZ9E0MqQoGaJXlGHA1pOlaKlODaRDITJTmySpuomcZjDIMQgmcNqOhWeaqnurI+nOsYzGiqY5kyAQ",
        categories = "weather,seasons",
        tags = "cold,weather,freeze,snow,winter",
        contributors = "karsa-mistmere,lscheibel,ericfennis"
    ))]
    Snowflake,
    #[cfg(feature = "soap_dispenser_droplet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CAMh2DQIg+DwL4DgWFYYgaCIKDENIQEgNxhDKEAgDCJ4nC2JQyhSFobhqBIcgmCw5C4Mg5CCHwuDcNhBDYLg2DcIJAkKKYoDGOpDDEMRhkWRJBkeOorjeOQzjwNhjC0MQuDENQuDkMpbjySQyC4MA0mMN5JlcMY5iiZgyiWXA4CANJng+IJ3DANRzmmdAznYLZ7DUYwwmMMQ2C2Dg3lQMqKlyawtm0OYuheMoxgWB40jYNgwkMMgxEgNokiaKJIiuKx2C2I4slKZYrGgNxWiOSZJqeKgxlsSA5GGtpSoeth2DOlowgE",
        categories = "home,travel",
        tags = "wash,bath,water,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "Andreto,ericfennis,jguddas"
    ))]
    SoapDispenserDroplet,
    #[cfg(feature = "sofa")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA5FYNhhDIIITg2FgtDKGBIhGE4VCCF4dHYMwiD4PAvgOBYliiBoIgqEwxhyFIfjODYdGiMISjKFozhkMh2C0NY5h6Fw0h8dgxC6QZJkmNAgDELZLDUaAtDEMRhlGTZPkmUA1FaVZCk0MAtkUMB6iSJoriqBIsgkTZFDEOB2DKZ4nmuaoFgebYMk6cZziWdYpgKa55goMYTDQdg5nSaYBA",
        categories = "home",
        tags = "armchair,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere"
    ))]
    Sofa,
    #[cfg(feature = "solar_panel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyGgMgiD4PAvgOBYThaBoICIbQxDQLgyDgIIeC2Hw1DYIA4hKFIZhiBIagkbQyg0MgyC0MQuDUNQ4iUSImjuK4Vi+LoFgeCRNDOIwwHaEYTkKF4Ci+RoKDaIA0DWIw1C4MA0EEMoOCAMJhiOKYjDQaAxDIYYNg2Ypig0Lg4DYNIjjkMA1GwLQzC6DAglqVw3l6YJumQMYojWPprmShAxC2cQ2DON52DYepBi2UZFhsTQ3g4YZ0nSjIlCANKWkOmIwhwOAuDaKA3quKI4DSbAurKpYXgE",
        categories = "home,science,sustainability,weather",
        tags = "solar panel,solar,panel,sun,energy,electricity,light",
        contributors = "UsamaKhan,jguddas,karsa-mistmere"
    ))]
    SolarPanel,
    #[cfg(feature = "soup")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDEYQ5CCEgwCCFYVDkLQ5EgM4RhOFoghiExaCIPg8C+A4FiaKYGgiCg3g4MRoDEMIlieLIrgSLYJgsOQuDUIIMg6DQ2jaKI6jmBYHjwMQ2C4MpADMY5PDcLgxC4OI/DMLg3kCVwzDYLQuDCTg4DOYg5DOQZPC0MYOmMMpiDANZclkMw0msMg0lyapXDYMpGjiAo6kuCgxleUAglKVJWliWpcniX5hmOdJmmifZsC4OQ4m8MJxmOTg3namJ6lyDZ+oCJpHiqg5Ki4TZOomiwylWV5ZDWW5dmuYJymWZ6aqSbadp+c51C6d55nsN6RC6f6BkiAQA",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,bowl,starter",
        contributors = "kemie"
    ))]
    Soup,
    #[cfg(feature = "space")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcdgxGMMAgC4NQthaDgtDGDhIDMY4YDUIAwhuIYlDEdobCIPg8C+A4FD6AQ",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    Space,
    #[cfg(feature = "spade")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgdg0CIPg8C+A4FhSF4GgiCoNDENAuDQOQ5GENQuDUIImigMAgiyLA5icOQxCAMwuDYN4mDaNotjyMwuDiDwuDAMAxEGKopiePIuCAMoeiANRjDALQyC4Mg5C0MYnC0NAtDMLYqGyX4hDkMpiDODBhg2DZLlKXpCDITIoDgLojDkY5YkmWQ1l2NJVnyKoThWGg+gEA",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Spade,
    #[cfg(feature = "sparkle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgwDENwgDILg4DENBhDEIIZDAIIchmDQ5DYNodGyDQwDWGQ1C4NQ1DgYQyhKHYyh6Kw5DSGo1DQbIqiwOI4icMYYhqM5DjSIQ2GwLY8i2P4oi+MYclELYNDWNo4lWOpTg6KAgkuLofkQMZakeJJakCSori2T4wlGHZalib42kmXpmk6YJthuY4ijuaQ4nWQYwmyYY5nENB6CIPg8C+A4FD6AQ",
        categories = "shapes",
        tags = "star,effect,filter,night,magic,shiny,glitter,twinkle,celebration",
        contributors = "Shiva953,karsa-mistmere"
    ))]
    Sparkle,
    #[cfg(feature = "sparkles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgwDENwgDILg4DENBhDEIIZDAIIchmDQ5DYNodGyDQwDWGQ1C4NQ1DgYQyhKHYyh6Kw5DSGo1DQbIqiwOI4icMYYhqM5DjSIQ2GwLY8i2P4oi+MYclELYNDWNo4lWOpTg6KAgkuLofkQMZakeJJakCSori2T4wlGHZalib42kmXpmk6YJthuY4ijuaQ4nWQYwmyYY5nENB6CIPg8C+A4FomjIGgiCgyhwMh2DSiKKo+joEpCCRNDKMA0GgLaXomi6cokYxpHIYxsGUIBjHiCaXCAcoJDIIqwHmtwwpgL6qqyrg+gE",
        categories = "cursors,multimedia,gaming,weather",
        tags = "stars,effect,filter,night,magic",
        contributors = "karsa-mistmere"
    ))]
    Sparkles,
    #[cfg(feature = "speaker")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAdxpGQdBogkMQ2gwaBlGkZxoHSCQyDCDB4gkNIMHKIYKCIPg8C+A4FikcBhhEIBkgkTQxDIIA2GgLgwDGKIqi+EYpGMaRyGMbBlCAY4mjaDBjggIgxiMIByiKPgvkORZHi6MBojKNI2CCUY6jyVpAGgPoBA",
        categories = "multimedia,devices",
        tags = "sound,audio,music,tweeter,subwoofer,bass,production,producer,dj",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Speaker,
    #[cfg(feature = "speech")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4OAgDIMB2C0NAuDEbAxC4OQuDIYQyC4M4Ph8IAwiOJYeDENg0C2JxWgwMxBDWHw3CCMQzjOJI4g8IIMDINRjiSHoMDYNQ2CAMwuDANQ0CAMQghQNQ1GGMQ3jOU43iWTZIDIOYhDcNQ4EwMoPDAIg+DwL4DgWZppgaCIKDGGoODEN4NGGdA1CCd4ljmSAwDMLQxDALg2n6ZZnmya4Em2CRNnOTJRkeeKRnuIwtloNYThmXqGmiig+gE",
        categories = "accessibility,communication",
        tags = "disability,disabled,dda,human,accessibility,people,sound",
        contributors = "doerge,airone01,jguddas,karsa-mistmere"
    ))]
    Speech,
    #[cfg(feature = "spell_check_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMYMDYLQxDIIIMhIIg+DwL4DgWGIbgaCAiE0OINDIaA4heGYeh2BIfgkTQ0CAMgxGMMQuDEIAwg2NoRjELgzhEc41jeNwyj6DY0jaOI6DGPJFj+Q5JjmQoNj0M5UlOUo7kSPo8liS5Uk6V5RkuTZcDGKIaiwPoB",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck2,
    #[cfg(feature = "spell_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMYMDYLQxDIIIMhIIg+DwL4DgWGIbgaCAiE0OINDIaA4heGYeh2BIfgkbYOCAMgwjGNA0C0NIohqLA+gEA",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck,
    #[cfg(feature = "spline_pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgwDMNAggwLg2DgMRhC4NA5DiGYbCAMIfhIIIUDQNwtiQNxsDkIAzC4NYYDWLohiAMYng+LQ5DQMxsC2LQ0j+EoOhUYQxiKIJHicNg2hQNo8DGQg3iyGY6jCMpHhKJ45i2Nx6CIPg8C+A4Fl+YoGgiCg1hINxBgyEgyjOIgxlENZemCZZfGMaRyGMbBlCAYx5gmdJ/HiCQxDkIggHKCQynUL55nufZ4nqfJ+osIqNn+gQioeiRjoUIp0l+j6UpKAQ",
        categories = "arrows,cursors,design,tools",
        tags = "path,tool,curve,node,click,pointer,target,vector",
        contributors = "kaleidosium,mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    SplinePointer,
    #[cfg(feature = "spline")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ5CKCR5gwNYQHKDAyCIPg8C+A4FgeGodgaCIKhOFYXhAY4Sg2D4ahyBIihocBhHQaAgGSDBNDUIAxDcQQxDKO5ADAIJDDGOw3CCFItjKNA+gE",
        categories = "design",
        tags = "path,pen,tool,shape,curve,draw",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgNR2DUIg+DwL4DgWFYYgaCIKDiDhIDOEoUhaG4agSHIJgsMggDIMh2C0OAuDMYQ0CCNgwCCOQwC0MQuDENwyC0MguDiQRMDODokheKIngWB4JG0MQ1CAOQgDYLQ2kuJoBA",
        categories = "development,arrows",
        tags = "break,disband,divide,separate,branch,disunite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Split,
    #[cfg(feature = "spool")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDMLg0DQIA0hENAyg4NwuDAOAyEGGIYDAIIiiKFQ5h0IAyDESAxDkYYgiOMYiC4NQ1DgLYQDmKhsC0MQuDEMQ1C0Lgzh6MIkg6GYOhUOAzDcVoaDYNgiD4PAvgOBZVliBoICIbYNDEMI0DaDgyjSNo4C4NoXh+KYyjGLYbDEOAgDMSA1i+bpIDCQ41nSOY7j6QA1kSRp6nAIINDmPw2DMdoagwOJUlaWw+gE",
        categories = "communication,tools,social",
        tags = "bobbin,spindle,yarn,thread,string,sewing,needlework",
        contributors = "karsa-mistmere"
    ))]
    Spool,
    #[cfg(feature = "sport_shoe")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDALg0DIIA0C4OAtDULgwDcIg+DwL4DgWHohgaCAiE0MQ5g4OBoDOHYfiSI4EiWCRNDkLoNDKEwyDGEQxDQII3DSDBBjsIAwkeSY8C6Ew2hEbIYC4NgxC2FQyDAOBBDGDpJkiSI/CAMx2DIYZGl6Dgtj0Mw5kCPQ5DANhMDiUg3DcIJzDANQzlqXJnkgOJBGMLQugyDZOmsMwtDKGg4hMOZVkGZQghOfpJDQaI/i+IIzD6AQ",
        categories = "sports",
        tags = "footwear,sports,running,athletic,shoe,sneaker,training,exercise,fitness",
        contributors = "Youya-ui"
    ))]
    SportShoe,
    #[cfg(feature = "spotlight")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgyDkNQgDEOQuDUNgyhINggDIMgiD4PAvgOBYfiKBoICIbQxDeGQgDMLg3DUOIbC4MA5DiHogiWJIEiaCYpDmEgyhWLY0DINgthWNo4iGPI7gWB4JE0NwuDYMQgDYLgzGEM4tCAMJel4LYuDmGAxlkbAtmYM4ykINw5luXZfnKEpZkQOQxGyWA4kCLg1DkNxhlaVpzl+ag0DKSA0DQNxsi4MQwkeWAyDEMaBhKYJyomi5plkNAxHqS46gKPJQgqMg5FaHYfkyI4BA",
        categories = "devices,photography,multimedia,communication",
        tags = "winner,soapbox,stage,entertainment,drama,podium,actor,actress,singer,light,beam,play,theatre,show,focus,concert,performance,lens,leaderboard,followspot,best,highlight",
        contributors = "chessurisme,jguddas,karsa-mistmere,ericfennis"
    ))]
    Spotlight,
    #[cfg(feature = "spray_can")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoC4MAxCIPg8C+A4FhSF4GgiCg3CANYPhGE4VhqGYEhuCRNDEMQgDeIYShSFoniaBYHimDIuhCMIkjOAonjaHQgDmL4jjKGI+jWHILCCK5EjGJQ8HIZRjHQIB3GkZIFgkNAiCAeIJDENZdHmCZiCAaBlGkZxoHSW5FlKVI0igIhtDEOZCCAMp5HYMQwGMMAgC4NgtC4NJMC2LAxGgLQ2GOhA2CAMKIoQNKTDEVorGwMgtDKRZQhqQJ1gwMaGDinKej2oIcqKTJ3qanZPjOAQ",
        categories = "design,tools",
        tags = "paint,color,graffiti,decoration,aerosol,deodorant,shaving foam,air freshener",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SprayCan,
    #[cfg(feature = "sprout")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA5C4NQzDYVg3GGDYNDAIIZDEIA0C0NBoDGEBhhCEIaieHIlDUVg1haHYnhuH4vheMIajINBjhkMgghwM48CCLQ1kCNQxC2PQiD4PAvgOBZJkyBoIgqDQ5GGQpChmGwgDiL5WkQLQ4h+SJKk+ToElCCRNkIMgxiENJikuZg+gE",
        categories = "nature,gaming,sustainability",
        tags = "eco,green,growth,leaf,nature,plant,seed,spring,sustainability",
        contributors = "ericfennis,mittalyashu,jamiemlaw,karsa-mistmere,jguddas"
    ))]
    Sprout,
    #[cfg(feature = "square_activity")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoHAYYLCAZINE0MQ3CAMQyGgLQyGyOwgDWOwtDEMI+DUSA3ieKYugsPoBA",
        categories = "medical,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "danielbayley"
    ))]
    SquareActivity,
    #[cfg(feature = "square_arrow_down_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJG0MQ2CAOAtDiNwiigL4uhqLYvGiMYJE2NQgjUSA4FaJ4pj8aA+gEA",
        categories = "arrows,gaming",
        tags = "direction,south-west,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowDownLeft,
    #[cfg(feature = "square_arrow_down_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gweIJgsIByhcIgygwaBlGkZxoHSE4VD4PAvgOBYoHAYYRCAZIJG0OAgjWNwiigL4uhGLYvGiMYJE0MQ2jYdg4EiJ4pjwaA+gE",
        categories = "arrows,gaming",
        tags = "direction,south-east,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowDownRight,
    #[cfg(feature = "square_arrow_down")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NHcaRkHQaIKDEOINgkIoRCAaBlGkZxoHSGIaD4PAvgOBYoHAYYWCAZIKE0MQyCAOB2ieKYuhaLYvGiMYKG0OAgjUIA0keRwtDQIooC+PBoD6AQ",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowDown,
    #[cfg(feature = "square_arrow_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoSHcaRkHQaIOhCEoVCIM4SHmDojD4PAvgOBYoHAYYcCAZIOG0MQyCAOAtDQII6jwIooC+Loci2LxojGDhNDENggjUSIRj+QRoD6AQ",
        categories = "arrows",
        tags = "previous,back,direction,west,sign,keyboard,button,<-",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowLeft,
    #[cfg(feature = "square_arrow_out_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDEaA2GEMoOCAMIWhgMgtDIVg1hOFYXiGG4bEiHoUhSIYWhuDh2DYIg+DwL4DgWMIzgaCAiG2DYPCAOQtDmL4xjaNYEjeCRNDmDgxEgMx2C2LowjKRQ+gEA",
        categories = "arrows",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutDownLeft,
    #[cfg(feature = "square_arrow_out_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDEVg1GEMgghQMAghcMAtDKGxIhKFIWhiGIbhUdgxDSE4ViKGYqDIaA2CIPg8C+A4FjKNYGggIhtgyFQxC0OZAjGM44jeBI5gmC4NDENR2DYaAtjCMo0kcPoBA",
        categories = "arrows",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutDownRight,
    #[cfg(feature = "square_arrow_out_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAzGgNhhDIIITDAIIWDGFIUHYMQ0hKGoWhgLYTDISA1h+FYXCAMYjiMdgtDYIg+DwL4DgWM42gaCAiG2DYNDkIA5jKNI5jiBI6gkTY/FaD4xjONZHD6AQ",
        categories = "arrows",
        tags = "outwards,direction,north-west,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutUpLeft,
    #[cfg(feature = "square_arrow_out_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDMdg2GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNgiD4PAvgOBYxjSBoICIbYMCAMwtDkIA5jCMo3jaBI4gkTQxDWPYuhGQ4zkcPoB",
        categories = "arrows,social",
        tags = "outwards,direction,north-east,diagonal,share,open,external,link",
        contributors = "danielbayley"
    ))]
    SquareArrowOutUpRight,
    #[cfg(feature = "square_arrow_right_enter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAxDYIA0C2EoUCIPg8C+A4FheGoGggIhNDODgyGgMQxhaGIdhyBIegmIQgDgVg1GEMggjWDYNDGNgtiQMQ0jSNggjiDpBDIdo+kCN5Cg6PI2EiM41kqOY8jwdgtDOKIZiwPoBA",
        categories = "arrows,shapes,layout,multimedia",
        tags = "left,in,inside,input,insert,source,import,place,->",
        contributors = "ethanhazel,karsa-mistmere,ericfennis"
    ))]
    SquareArrowRightEnter,
    #[cfg(feature = "square_arrow_right_exit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDIaAxDEIg+DwL4DgWFYYgaCAiG0MQ3g4NggDQLYlieFIWhuGoEhyCRNDIMQgDYLgzDQNBWDUYQyCCPINj8LQykESI6jyPggkCRh2DENI7j2SJQg2RoRk2RpRlCQgyHYLQxjWN4pheLQ+gE",
        categories = "arrows,shapes,layout,multimedia",
        tags = "out,outside,output,export,->",
        contributors = "ethanhazel,karsa-mistmere,ericfennis"
    ))]
    SquareArrowRightExit,
    #[cfg(feature = "square_arrow_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHmCYLCAaBlGkZxoHSCQxDiDB3GkZB0GiHYfD4PAvgOBYoHAYYkCAZIJE0OAgDEMhoieKYuiSLYvGiMYJG2N42DYIA0C2SJKCKKAvjwaA+gEA",
        categories = "arrows",
        tags = "next,forward,direction,west,sign,keyboard,button,->",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowRight,
    #[cfg(feature = "square_arrow_up_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINHcaRkHQaIVhcIIJCIM4NHmCojD4PAvgOBYoHAYYcCAZIKE0OAgDENhWDgaIXigL4uhyLYvGiMYzjeNg2CCNY8imPxoD6AQ",
        categories = "arrows",
        tags = "direction,north-west,diagonal,sign,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowUpLeft,
    #[cfg(feature = "square_arrow_up_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwch4gkMoMGgZRpGcaB0hOFQghiCgiD4PAvgOBYoHAYYRCAZIJE0OAgDgaA4HaFYoC+LoRi2LxojGCRtjUMQ2jYLY7imPhoD6AQ",
        categories = "arrows,social",
        tags = "direction,north-east,diagonal,sign,keyboard,button,share",
        contributors = "danielbayley"
    ))]
    SquareArrowUpRight,
    #[cfg(feature = "square_arrow_up")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEhyhQIgyhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOG0MQ2CAMQyC0NI6joIA0ieKYuiOLYvGiMYOE2OI3DYVoRigL5CGgPoB",
        categories = "arrows",
        tags = "forward,direction,north,sign,keyboard,button",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowUp,
    #[cfg(feature = "square_asterisk")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJE0MQyCAOB2ieKYuhqLYvGiMYJG0OAuDUIAxDQIA3C0NAiigL48GiPowjIIpDkWRwwkoIJNk+UQ+gEA",
        categories = "text,security,math,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[cfg(feature = "square_bottom_dashed_scissors")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig1CKBwygoMQ5g0eYPCIM4TgmFgiD4PAvgOBYch+Bh5hmDIHhmF4OgqKYUhCEoch6BBliGMongoMgxhOFYRhiComHiFY4huHYijSBQgi0IpCkiGZLkCEAwg2CIKi+RIykaI5NjmNgiDENI6jeW5Pl2DIwkUPBwGEdBoCAZIKE0IAzCANQgEEMggncMAgnoMZznGQwvmma4coKbJuCKcIRnGdZ3nmewgn2OJzoChaEmqhpvn6kp2nij58ouPIwpWaKXm2maSoqnKOp+ipCqKl4cGMaRyGMbIGHKEAuiYYx5goOK6g0Yx4r6wIwrKtK2liSI7DKPQiDmug2DANg2lKGbQDW0rUlKzKAmeIpMhAN7chANAuDizbLuW57plOXbjmaVw8seta3rmu7DCKv67r2ZLFh29LJgKNbuti2rViq75gCK47hl25g0DMOQzl+8YggE",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley,eden881"
    ))]
    SquareBottomDashedScissors,
    #[cfg(feature = "square_centerline_dashed_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMxIDUYQyCCEgwCCFQwC2EgyHYMQ0GOFQxC6IQ5hOJQyGgMwiD4PAvgOBYri6BoIgoMQ2g2KIRiWFwgDGJocDSOYUhaPIZhMaAtimK4tgQaIwkyB4JE0MYaDAdgyiqLIxk6BZQjSEodlaWJLi+ApPjOUoSDiYZKlqZZcmeU4TmuWZMD6AQ",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    SquareCenterlineDashedHorizontal,
    #[cfg(feature = "square_centerline_dashed_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA4FYNRhDIIITDAIIWDALQyhoSIRhOFYXheGoUHYMwiD4PAvgOBYoiuBoIgqDAgDENolhKFIhhYMYjDKHY3iCOoahodgtiaKIqgQaItkmB4JE0NIzj0MonimLpLgWTYKDGOo9DiVJIiyApMjATY0lEaIal+VpiliZAyhMMQymiU5HlaAQ",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    SquareCenterlineDashedVertical,
    #[cfg(feature = "square_chart_gantt")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEeYdhEcocCIMgiD4PAvgOBYoHAYYLCAZINE0OQgDgaA3ieKYuguLYvGiMYzDgIAxDIaA2joL48GiPowjIIhNDEMZEDYaA1kmSw+gE",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    SquareChartGantt,
    #[cfg(feature = "square_check_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDALg2DUNhWDEORhDIIIZDAIIcDELYZDISA1hiGodieHwyiAVokiGJ4ehqIBoDEMguDMNA0CIPg8C+A4FjuPoGggIhtDmDoNDMIAzEwMoZjmO49gQaA+gEA",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    SquareCheckBig,
    #[cfg(feature = "square_check")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwaBlGkZxoHSCQxDiDB3GkZB0GiHYfD4PAvgOBYoHAYYkCAZIJG0OQgDEMggjiOA0C0NAiigL4uiQPoBA",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "danielbayley"
    ))]
    SquareCheck,
    #[cfg(feature = "square_chevron_down")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINgkIgzg0eYKhsIB3GkZB0GiFYXD4PAvgOBYoHAYYkCAZIKG0MQ2CAMQwC0NAgDSOo6CKKAvi6JA+gEA",
        categories = "arrows,navigation",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronDown,
    #[cfg(feature = "square_chevron_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDB3GkZB0GiFIWCAeIJgsIByiAIgyCIPg8C+A4FikcBhhsIBkgkbQxDQIAxDYLQ0jsII8DSKIqi+Gw+gEA",
        categories = "arrows,navigation",
        tags = "back,previous,less than,fewer,menu,panel,button,keyboard,<",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronLeft,
    #[cfg(feature = "square_chevron_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEhyhQIgyhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOG0MQwCAOAgDSOQtjoNInimLojD6AQ",
        categories = "arrows,navigation,development",
        tags = "forward,next,more than,greater,menu,panel,code,coding,command line,terminal,prompt,shell,console,>",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronRight,
    #[cfg(feature = "square_chevron_up")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NgkIoRCAdxpGQdBogoMQ4g0aBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIKG0OAgDENAgDQLY4jwIooC+LoaD6AQ",
        categories = "arrows,navigation,math",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronUp,
    #[cfg(feature = "square_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIA5C0MwghGEwiD4PAvgOBYWhmBoIgoMQ0CAMQ1hKEImDOFYXhyFhyGUYx0CAdxpGSBYJDEOAiCAeYJigIBoGUaRnGgdI2jgIB4jyORykgIgyikL4ti8PoB",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = "danielbayley,jguddas,karsa-mistmere,ericfennis"
    ))]
    SquareCode,
    #[cfg(feature = "square_dashed_bottom_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5C4NQgDgIAxDIbAyCAMoQCIPg8C+A4Fh2IIGgiCgxDSGQxGgMYch6I4igSJIJG2J4OhCGYZhALYYhoNYth+MYwgWB4JE2EQyDEYY8CCDYNDGO47FYNZKjiTYUhmO4rDSVIYlYMY4DIdonlyTJlk+PI/i+AoxkSCg5imK5pkGAQ",
        categories = "development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottomCode,
    #[cfg(feature = "square_dashed_bottom")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxGEMoNCAMITCAMQtDKGBWDWEIShSFAxg2GBoDENIdhGH4WhIMh2iWJ4ViCGINCIPg8C+A4FjWOIGgiCg5g0MYkjSNo7jqBI8gkTYlkCQo1jeRw+gEA",
        categories = "development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottom,
    #[cfg(feature = "square_dashed_kanban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANx2DcIg+DwL4DgWEoVgaCIKDEMoNHYNIRhOGIXgSGYJE0MQ2h4OYhhSJYkgWB4nDUIAzGGHYdDAII6DALY4i2I4CiWMoKDmNRoDGQIvkKMYaigNJHkmEouhaTImhuRo2jiO5cDEII/lOQYYkQTQyl4OR2lKIpLmOTpmCAMQ0mmSpVm2J5vDEOY3l+XI6DGPpfnQaIwleT5fDGSKCoSZJGmaiZhmyQ5OjSZp7jmXY+j6ipWmQM5wnKapUoOnJOp6aKhiOAQ",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,draft,template,boilerplate,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedKanban,
    #[cfg(feature = "square_dashed_mouse_pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgwDMNAggwLg2DgMRhC4NA5DiGYbCAMIfhIIIUDQNwtiQNxsDkIAzC4NYYDWLohiAMYng+LQ5DQMxsC2LQ0j+EoOhUYQxiKIJHicNg2hQNo8DGQg3iyGY6jCMpHhKJ45i2Nx6CIPg8C+A4Fl+YoGgiCg1iwYQyCCbJXDALZsDKXpgmWZIEmaCYLisM5rm2M4inKdJhnid4FgeeppDKF5yoCNQynGg52gKeKIgqfBoDGkqFpSh5nE2K6LGic5foSY6dnmCgxhEM6Zpup5lpYTQzCAOR2pqpaTrGn6LrUdqknWnK7nqtKrrerxoD6AQ",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "danielbayley"
    ))]
    SquareDashedMousePointer,
    #[cfg(feature = "square_dashed_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyDEaAxCIPg8C+A4FhSF4GgiCoMCAM4RhOFYahmBIbgmCw5h8YQyg4IAwi8IAxi4MoihaJolgWB4og+Mg0HaEoUjeGICiaO4Kj0MQ5iyLowjAMQti2NZCiSRY6hwTY9DmQI2lWGpHE0M4+lyVI4laJ4KmKW5BiOZpflgNYODGTItk6MpRlGXZukacIrlKMZOlGDp6kSb4oDeMgyhEMKEGiOZoE2iAxDYaA2o2j5gogOBoDil5nmCKoPiGZaFnyKIqiCbJDo6AQA",
        categories = "text,cursors",
        tags = "find,search,selection,dashed",
        contributors = "danielbayley"
    ))]
    SquareDashedText,
    #[cfg(feature = "square_dashed_top_solid")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyDEaAxCIPg8C+A4FhSF4GgiCoPCCDB2hKFIWgQaIZiWB4JE2HgxDkYQyg4IAwjKHwtjAMoThWGongWKYdDEIA5iGOYkhiAoohwTQzh8NJDiOO5Hj2SZLDWL4xjOM5ADKNoRDSVowliH4xjiT4ljyG4qkuQoijqZpRmiCg1g4MZfjSWY2jaRJQhqPhNDmc4RnqZoB",
        categories = "design,development,layout",
        tags = "square,border,width,layout,style,design,rectangular,marquee,dashed,box,rectangle,aspect ratio,1:1",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,chessurisme,danielbayley,colebemis,juanpablofernandez"
    ))]
    SquareDashedTopSolid,
    #[cfg(feature = "square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxhDIIIQDAIITDALYQDIIg+DwL4DgWG4egaCIKDEOYNg+EYUioMYphmG4dgQaIgjGB4JE0MosiWKISiuF4RhqHIhjOBY1gqDI4juKoTDGF4XkCMIfgKNIjE2JgzGgMZPkKUpElSJo4liWoxkOIo2DENINmGL5biGRRNmeEQxmqQZjlyZYKDMIA5HaWZrnWbZUjiep8mKUaAjaeZnoSfqGlONqCoqfZ0h+AQA",
        categories = "text,design",
        tags = "selection,square,rectangular,marquee,tool,dashed,box",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,chessurisme"
    ))]
    SquareDashed,
    #[cfg(feature = "square_divide")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIIJCIM4NHiCoRCAdxpGQdBogoMQ4g0aBlGkZxoHSHIegeE4LCIPg8C+A4FiwbBpG4ZYODKHIMg4MY4hKNwiDENoSjsIoeiwL4yjSMYzjUeZDkCDR5j6TwgHiTo5HiUoMkaSBlkqNI6gqJ5RmGPY8lSVori2XA+gEA",
        categories = "math",
        tags = "calculate,math,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    SquareDivide,
    #[cfg(feature = "square_dot")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoGMaRyGMbBlgeFINGOJQiDGDAgGOIY6CKKAvi6MIyD6AQ",
        categories = "development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareDot,
    #[cfg(feature = "square_equal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByggIgygwaBlGkZxoHSE4VD4PAvgOBYoHAYYRCAZIJE0NwgDEMBojcIooC+LoRi2LxojGM41DENI5DCO4pj4aA+gE",
        categories = "math",
        tags = "calculate,=",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareEqual,
    #[cfg(feature = "square_function")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwaBlGkZxoHSCQxDiDByg6FIMHcaRkHQaIdh8Pg8C+A4FiwcBhigIBkgkTQ5CAMQ3GMMggDAIAyC4OAtDGQZDC2Qg4FYMQwGMMJJjoLQzC4MwglQMpTCKLAvjKKIxjMaI1jeOQxDELgyGgNQuDeW4tl4aA+gE",
        categories = "development,math",
        tags = "programming,code,automation,math",
        contributors = "mittalyashu,ericfennis"
    ))]
    SquareFunction,
    #[cfg(feature = "square_kanban")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJE0OAgDcdg3CKKAvi6Goti8aIxjMMQyjYdg0jqKY9GiP4wjIIhNDENpGDmSY8kAPoBA",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    SquareKanban,
    #[cfg(feature = "square_library")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEch4g0MoRGgZRpGcaB0g2D4RhgIoVD4PAvgOBYoHAYYLCAZINE0NwgDcdgxDAIooC+LoLi2LxojGMwxDGNo4jqPI+GiQIwjIIhtDENY2CAMggjmO4pksPoBA",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    SquareLibrary,
    #[cfg(feature = "square_m")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ2FYOAuDUYYShIIAwheDQgC4OQtC4MxsDILg3CAM4SDkOYUDWFoYi2G4MDCIYjC2Jg2iqLIZDGGw5h8VoOCIPg8C+A4FkEchlGMdAgHmCQzCIIBoGUaRnGgdIJDEOJPHiTZPHKWwiDKTx3GkZIFleWZBC+R5JD6AQ",
        categories = "transportation,navigation",
        tags = "metro,subway,underground,track,line",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    SquareM,
    #[cfg(feature = "square_menu")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQxDiDB3GkZB0GiFIWCAcoICIMoMHmCYLD4PAvgOBYoHAYYbCAZIJE0NwgDgaAxDAIooC+Lobi2LxojGM41DEMo4jqPI+GiQIwjIIo0CAMQ2kiO4pksPoBA",
        categories = "layout",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    SquareMenu,
    #[cfg(feature = "square_minus")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMGgZRpGcaB0gkMQ4gwdxpGQdBohmGwgg+CgiD4PAvgOBYoHAYYgCAZIJE0OAgDEMhohuKAvi6IA+gE",
        categories = "math,development,text,tools,devices",
        tags = "subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    SquareMinus,
    #[cfg(feature = "square_mouse_pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgwDMNAggwLg2DgMRhC4NA5DiGYbCAMIfhIIIUDQNwtiQNxsDkIAzC4NYYDWLohiAMYng+LQ5DQMxsC2LQ0j+EoOhUYQxiKIJHicNg2hQNo8DGQg3iyGY6jCMpHhKJ45i2Nx6CIPg8C+A4Fl+YoGgiCgykUMQxFaLwyCCb5XDALQynQSJunCM4fnScB2DENBhm+cZ6oIaA2l6YJlD6AQ",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    SquareMousePointer,
    #[cfg(feature = "square_parking_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NgggwNhBDIIITDAIIWDEIA1g8aAxDQYYThWFwghmIR2h6IIUiOGAtC4NQ5iQLg0DEIg+DwL4DgWNo5gaCIKDMIA4C4NxWDEOYpiKFoWiGHQwC4M41jePI7gSPYJG2IYUksMJRjiVZUgWB4JE0MZAmUYZAkCGIrC0NhIDkdgyl2U4ClWYoKjAMQ3HYLQyk+c5fgEA",
        categories = "transportation,navigation",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    SquareParkingOff,
    #[cfg(feature = "square_parking")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh3GkZB0GiDoQhIcoUCIMoSHmFQiD4PAvgOBYoHAYYbCAZIOE0OQgDENxWDcaA0GEMwgj4MAgkEMZCCANhIDmJ4pi6Gw+gE",
        categories = "transportation,navigation",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    SquareParking,
    #[cfg(feature = "square_pause")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEeIdhEcogCIMgiD4PAvgOBYoGwaRuGUIB5DKDQ5hsMYZDWH40g4MIfjiPYnimLowi2L4xHiPAxDSG48jaMpADGOggHiUZMigL5EGUPoBA",
        categories = "multimedia",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SquarePause,
    #[cfg(feature = "square_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzEgNRhg2DQwCCFQwC2Ex2DENISCCFIWiGExohyHogheH4ZHYLQ3CIPg8C+A4Fi+MoGgiCgxDgLgzDcNYfC4NgyhEMQgkSKJEDODhsC0OQuDAMZJk2T4dhOIYVDELQuDgNQzC4NQwDWSwyloN5dDiHZel6VpFlmQZtDIbJaDSGZkDOJprkSXgwDaWZbDIeoujCNQ+gEA",
        categories = "text",
        tags = "pencil,edit,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    SquarePen,
    #[cfg(feature = "square_percent")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAeYJgsIByggIgyCIPg8C+A4FikcBhhEIBkgkbQxDUIA5C0NggDaKIqi+EYujAaIygkTQ5jgaAuDAMY+C+QBokKMYzCITY2CCNpKkyTpQD6AQ",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    SquarePercent,
    #[cfg(feature = "square_pi")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoHAYYLCAZINE0NwgDcaAxDCJ4pi6C4ti8aIxjOOY2HaOY7C+PRoj+MIyCITQxDYIAxDcYQyCCVwwCCWgxC0MpeFYN5IkoPoBA",
        categories = "development,math",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    SquarePi,
    #[cfg(feature = "square_pilcrow")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKE0MQyCCNRIDkLg1GEMo7CCPg1CAMJDjeQwtDUSAxDcIooC+LoSi2LxojGM41CANx2DEMJNimUBolKMIyCKNA2liWpck6Xw+gEA",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "danielbayley"
    ))]
    SquarePilcrow,
    #[cfg(feature = "square_play")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEch4g0MoRiEIoeD4PAvgOBYoHAYYLCAZINE0OQgDkLgwDAMxhDEII9DAIJAj0MQuDUMQ3C0Lg4DUORsDQLg5DkNwgDKUJSjyPpBlqP4+C4N4PGwLZPlGU5VmQQZDlqQo2j6Yw5DYegiigL4ugsPoB",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SquarePlay,
    #[cfg(feature = "square_plus")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIB5goM4NgkIojD4PAvgOBYoHAYYSCAZIKE0OAgDEMhohaKAvi6Eoti8aIxjONwgDgdo6imPRoD6AQA",
        categories = "math,tools,development,text",
        tags = "add,new,increase,increment,positive,calculate,calculator,button,keyboard,toolbar,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    SquarePlus,
    #[cfg(feature = "square_power")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3HYNAiD4PAvgOBYThaBoIgoNwuDkOQ4CAOQuDAMAzGENQgikMAgDEIIsDgLYkDANYShSGYTHIZRjHQIByHiCQyCIIBoGUaRnGgdIJDEOJDkAIgzkMdxpGSBZLk0IB5gmUYTC+Oo8D6AQ",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley,jguddas"
    ))]
    SquarePower,
    #[cfg(feature = "square_radical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQyGgMhsDIIA1CAMgtDEMBoDQIg+DwL4DgWHhyGUYx0CAeIJDMIggGgZRpGcaB0gkMQ4iwcopCIMosHmKosHcaRkgWNI2h4L4kiYPoBA",
        categories = "development,math",
        tags = "calculate,formula,math,operator,root,square,symbol",
        contributors = "smnandre"
    ))]
    SquareRadical,
    #[cfg(feature = "square_round_corner")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDEYQ4CCEgwCCFQwC0OIZCIPg8C+A4Fh2IIGgiCoMg4NR2DQYQyCCLYXg4LYtDISA1iyLoWjkMYyjIVo2jOOYVg0MoyGgNIch6Iw+gE",
        categories = "design,development,layout",
        tags = "border,radius,style,design,corner,layout,round,rounded",
        contributors = "liamb13,jguddas"
    ))]
    SquareRoundCorner,
    #[cfg(feature = "square_scissors")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoGMaRyGMbBlCAY4kDgLg1hIY4gCKN45CAcoZjiJ4pi6MIyigbBpG6Mx4DGDg5jgNgwDYNofDKGYmCAeJYg+Wh5k8IpRDWU5VkQL5KkySZLk2YQxDeG5dDENAuDiX5ynSdofm6cIomibIti+MYzjuGQ1kOQJCj+NYOj6Z5GoOa5MCAeZdnCW5ypeYIZnQNAzDkMw0huYZjmWVp+mkZQ+gE",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley,eden881"
    ))]
    SquareScissors,
    #[cfg(feature = "square_sigma")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDYJCIM4NGgZRpGcaB0hSFggHmCoaD4PAvgOBYoHAYYSCAZIKE0MQ2CAOAuDkVg3EgOBsDQIA1C2QA1GgOB2C0MY5CKKAvi6Eg+gEA",
        categories = "text,math",
        tags = "sum,calculate,formula,math,enumeration,enumerate",
        contributors = "danielbayley"
    ))]
    SquareSigma,
    #[cfg(feature = "square_slash")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEch4g0MoRiEIoeD4PAvgOBYoGwaRuGUIB4DGDQ5huNIODWJAyhmOggHmPAijaKAvi6MA+gEA",
        categories = "development,math",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareSlash,
    #[cfg(feature = "square_split_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ5EgNRjC0MQgDALQyhOF4XFYNxjhaFAxhcIIYDIaAzCIPg8C+A4FimLIGgiCgxDYIA1iYY4UDCI4NjsMh2DEMIeg2E4jiKJQtieKYrgQaIpGwaRuGUIB4DGCQxDIIpTDKVpYCAeZVCINJZHmWwiDIMIoiqT5RD6AQ",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SquareSplitHorizontal,
    #[cfg(feature = "square_split_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAOBWDUYwwC0MQgDELQyCAMoXGgMQwGOFAwhmFYiDIdgzCIPg8C+A4FimLIGgiCgxDmFQ2iaEYVhOGYXhkSA3GOOoShqFoaiULYnimK4EGiKRsGkbhlCAeQygkMQyCIIB4lQIgyDCWB5DGVZXlmYQiDSKIqk6UA+gEA",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SquareSplitVertical,
    #[cfg(feature = "square_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoiuBolCKEIShSFoYjGEYIgqDI3CCIYyiCGYnimLg+gEA",
        categories = "layout",
        tags = "float,center,rectangle",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    SquareSquare,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGMLQxC4MQgDALQyC0Lg5heFxWDQY4WhKEobDIIIYDIaIfiKFYmCCGotDIIg+DwL4DgWM42gaCIKg6DQ2hCK4WhiGocDIdgth+IYTkSJYnikY5Bi2L5NjKNI5jMchlGMdAgHIeIJjEIBoGUaRnGgdIJDgIggl8IgxDSax3GkZIFmmax5gmb5VC+WZbD6AQ",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = "danielbayley"
    ))]
    SquareStack,
    #[cfg(feature = "square_star")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgwDMNQgDcLg2DkYQxCCGAwCCG4Yg0OQwDmDgyDQbAuDcMw3hkLg0DUMoXhmHIyhuJ4pC4NYQGyDQ2DMNAuDINQ2jCGoyhiNw4DiKw4DANhsC2DQxDcMorDENg4kOM4cC2Pw4DILg4DaTY/DUOYrDYMQzliG4dk+Nw0h4LpoiWbQ0DYNZbDcNZqjOWw5DGUwwk6DZ1DWJ56h6WQxm2OA5m2aAziaQA4m2Z5poia5alwMpbmCTaOkGn6WjGmJGDWUptl0Mo6hQMwzluQJCpeWY1DeW44DUegiD4PAvgOBa7HIZRjHQIB4gkMwiCAcrGCIMrJHcaRkgWCQxDiyRoGUaRnGgdLUtYIB5seuq8sGww+gE",
        categories = "sports,gaming",
        tags = "badge,medal,honour,decoration,order,pin,laurel,trophy,medallion,insignia,bronze,silver,gold",
        contributors = "karsa-mistmere"
    ))]
    SquareStar,
    #[cfg(feature = "square_stop")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHmDgzhIeIdhIcogCIMgiD4PAvgOBYoiuBoIgqDIODaIokDGEoUhaGAijQIIcCIOYfg6QYoiqBB0D6AQ",
        categories = "multimedia",
        tags = "media,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SquareStop,
    #[cfg(feature = "square_terminal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQxCAMgthGEwiD4PAvgOBYWhmBoICITYOg0MxoDSFYXhyFhyGUYx0CAch4gkMgiCAeYJDOMx3GkZIFgkMQ4jOMAijeLo1CKMggGgZRpGcaB0j2P4WC+KosD6AQ",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    SquareTerminal,
    #[cfg(feature = "square_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYQ2CCEgwCCFQwC0MQyhYIg+DwL4DgWHhjGkchjGwZQgGMeYJDEMQiCAcoJDSMBjHiLQyh2H4kiaKIeHIZRjHQIB3GkZIFi0OIwiwIgzjAaBlGkZxoHSSYwjeTYwHKWI5h4L5AkIPoBA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUserRound,
    #[cfg(feature = "square_user")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwch4gkMoMGgZRpGcaB0hOFQghiCgiD4PAvgOBYoGMaRyGMbBlCAY4lDGGo0ggIgxDCFoJguKAvi6MIyigcBhhEIBkgkTQ3CAMgxHYLQyGEMpPCAMJYCAMZPlMaA2lWV5ZlmXJWDIdoakGR4RD6AQ",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUser,
    #[cfg(feature = "square_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAch5gkMoMiGCoMHKCAiiMPg8C+A4FiwcBhhEIBkgkbQxDUIA5C0NggDYIosC+MoRjGMxojWNw5juP4/kGLZEGgPoB",
        categories = "math,notifications",
        tags = "cancel,close,delete,remove,times,clear,math,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    SquareX,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh3GkZB0GiDoQhIch4g4MoSiEIoWD4PAvgOBQ+gEA",
        categories = "shapes,multimedia",
        tags = "stop,playback,music,audio,video,rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
    #[cfg(feature = "squares_exclude")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDIdgyGEMgghQMAghcMQthQMhIDkYQxg6GIjDALYhDEdgzhOFYkiOHBoDEMIrhaLQyhsVoxjOLYljYMhoDAIg+DwL4DgWQpFgaCIKDSDg2jqF4ZhuNw0k+I4hj2MIyhyLZXhWKYgiKUIOiaDhoC0NZVlCG5eDKQZDkgPoBA",
        categories = "design",
        tags = "square,pathfinder,path,exclude,invert,xor,shape,vector",
        contributors = "EthanHazel,jguddas,jamiemlaw,karsa-mistmere"
    ))]
    SquaresExclude,
    #[cfg(feature = "squares_intersect")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDIYQyg4IINg0MQtDKGAiD4PAvgOBYch+BoIgoMQ0g6EYThUIAxhMMobh2IohgSI4JgsNoODIaIahyHo0jOBYHjaEoMFYOIwj6IICjSQoKhINIphKK4thmL49jKS5BiQTQyg0OJRhSYZUg6SJYiKTZckQNB2laMY/lmNZOhKXZglOGJkleb5nluJwxDadZihiPJukqe42DiLAwoCFoOhgaA1GGLYtlOLIsHakJzmGFp3DISA5pGlZ2hcMR6mWepMluiI6m2SRoD6AQ",
        categories = "design",
        tags = "square,pathfinder,path,intersect,shape,include,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresIntersect,
    #[cfg(feature = "squares_subtract")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDIYQyg4IINg0MQtDKGAiD4PAvgOBYch+BoIgoMQ2g4MhohqHIegQaIhi6B4JguJw0hGE4VhSGIYEiNoShKOQwhiDh2gyN5AhSSY/GgMxhDEIJPkGUAtDEdgtDWR5JhaDoYGiWJPlGWpTDEeobh2IowgWMoKDKDQ4lmOZPj+ZotiCAoxiQTYPlANB2DKdJoneap5nubZwkmF5ziyaIBA",
        categories = "design",
        tags = "square,pathfinder,path,minus,subtract,subtraction,shape,front,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresSubtract,
    #[cfg(feature = "squares_unite")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ2GEMgghEMAghQMQtDKGBWDSEIShWHwxhKGBoDEMIdhOIIeDIdgzGGIYhhSMYNg0aIthGKIWiodolieH4WhiEhIjyN4+g2GIYHYLYti+RQwC2FwxHoIg+DwL4DgUPoBA",
        categories = "design",
        tags = "square,pathfinder,path,unite,union,shape,merge,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresUnite,
    #[cfg(feature = "squircle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg3DcIINDANAzGEMw0hGGAwCCGwwC2DQ1hoIg+DwL4DgWJIngaCIKgyDg3DEIAyDALg5DUNoWDOEY6h0IAxh8LohDELgwDAMYjiWKopgSK4JE2MwuDEOI+DeDg0GMLZBjGQwxDULZDDIOY+jUMwtDILoUmKZw0DSSImkyS4FgeTpQlIIA2C4Mg1DmWJal+UQ0Dif5hj+Qw5mGZpoDSiYUDibpKgKTJzgqUI2hAMY0DKFQzjqnIcp+MYbiCbYkm+KKRnKLBNhKFI+pmm4Yhen49kSRoRkENJHqWkIqpMTZ4DINoyjQMQ3nyf5dlkNQwoKhplmAOaMouawzo6u5wqiTYKsCwoNDgMpYlwNA5n6zpjoS0oyoqj5wgEA",
        categories = "development,shapes,design",
        tags = "shape,pending,progress,issue,draft,code,coding,version control",
        contributors = "jguddas,aramsoneson"
    ))]
    SquircleDashed,
    #[cfg(feature = "squircle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzGMNwug0MAgDkIAxC4OIVhUcwthiGg5C2FohiGH4iiKF4ZiiJAiD4PAvgOBQ+gE",
        categories = "shapes",
        tags = "shape",
        contributors = "jguddas"
    ))]
    Squircle,
    #[cfg(feature = "squirrel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgyDMNggDIMhhDMIIWDAIIZDALQyg4LQ1CIPg8C+A4FiOJoGgiCgxhEMgwhWF4ajMMYXC0MxoDEYQyhKM4bhKHR2h0YQ0CCRY/hwNAtDQVg0iKJIpiiBIqgmCw4CAMY4C4MAxk+JZTlKBYHlUMZXDaRJGj6GpLmkNwgm6SAtm4NxjhwNZGiCeAxDALg1kSfZGoCGY1hwOYzh6d6ImqGZ3DAQ5unuMgxjWFgxnSGYeleHo8nenY5DCXpRgE",
        categories = "animals",
        tags = "animal,rodent,pet,pest,nuts,retrieve,updates,storage,stash",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Squirrel,
    #[cfg(feature = "stamp")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDMVg4C4NRDgwIA3g4NYXhkIA1GEMwgiAMAgiMMAtDaJBjiMMoOCCLAxiGExWg8Ig+DwL4DgWNo5gaCIKDKIwxDWExhDKE4ukeJYkC2Rg1kyExoC0MQxEGTZIhqSojg2QoyDENxhjCMJZi0MRogyYItmMMZSHqNY3jyO4Ej2CRNhoMgymYNJujicg+gE",
        categories = "design,cursors,tools",
        tags = "mark,print,clone,loyalty,library",
        contributors = "karsa-mistmere"
    ))]
    Stamp,
    #[cfg(feature = "star_half")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDgLgzDMOBhDILgxCCFoYDAIIcDALQuDkOA3C4Mg0DQTA2hEOQ2hkMQuDAMRhC4NQzjQM4djkMYgDcN4gDUNhsC4OA4jsNYXDMOYVheDYag2HodiANgxjuL4jDkTIai0OQuDcOQ1jONY3jmHIYiUOQ0iAOQwkGRwxDYNY8DWYJOhmTJkjqNA5j6L5vGyFgzDAOQtDQLg2l4QY3mOUIYgyGQiD4PAvgOBQ+gE",
        categories = "social,multimedia",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    StarHalf,
    #[cfg(feature = "star_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALgzDQNAgDQLg2DgOAgDELgxDgMQtDKDg5DMYQuDUM4kDMIAwimGAgC4OQ1ikbIfDMMYShQNw5GGHwxDIII7j2KpBhiJIvkMMQ2GwNYaDYNguDcNQ2iOJYniuKo1C4Mg5hMOQwDQbAtiYMgzDcIImDENQyCIPg8C+A4FmuboGgiCgxDeLg0jCdZ3koNIoh8NQwDWUomiWVYYC2Tg3hmUJfhMNobh4Lg0DIOI6hqPY/oYMAthkOQ3igMBMk0Mw5DaPoZDAMaDlSQodomiKMC6FodkoMakpaPI+pemqIo+HYZDiOBMjupg5k6L6roWrYtlkNKIlySK1DYNaIk+goXheQoqpKiAwDkOZqmycZwgScoJG2mI+ioMgwuGbblD6AQ",
        categories = "multimedia,social",
        tags = "dislike,unlike,remove,unrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StarOff,
    #[cfg(feature = "star")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg1DINQgDILgyDkNRhg4M4ZCAMIcCAMQgC6FocGyEwziANAuDYNw5GGEwxDIM4SC6MIyh2N4fg6I4NDENhsDWNA2DYLg3DUNoYDWGpJh6HYghQOYpDkMA0GwLYaDcMw2CCGg2DMOIujSMYzjWTIcC2KoMjkOA3DgbAuDgOAyCCQAxDSSJKjaHgxmcNw3g2RpVikNgxDgLYTDQMpfi8Mpyouco4DALYNDmWIcEyQwzDmWgyg0MAxneG44nuRA3megJvDie50pmYIwo2YaPkyZ6DnuDZrDkTIvloOZEhaoJLqKIYVDSZ5Sj6dA2DWfA1hejpjoyZZNjqpY8DYegiD4PAvgOBQ+gE",
        categories = "account,social,shapes,multimedia,weather,emoji,gaming",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "colebemis,jguddas"
    ))]
    Star,
    #[cfg(feature = "step_back")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg5DcMQgDQLgyDgNRBDIIIZDAIIchEMQ3CANh2DEMhhhmG4dCAMQtg0MAyDmKwuhANRsC0OYOg8LQ1jkOInhqKoeC0LgwDAM4tC4NAzDIegiD4PAvgOBZPlKBoIgoMoRDIMBWDSTpQlUPoBA",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgwDIOQgDQLgyDgNRBDIIIZDAIIchwNwgDYdgxDIYYZhuHYpDODoQCAMQuDcMQ1GwOQuDkOQ3C0NY2DkOImhqKYeCCDgwDMLYrDQMwyHoIg+DwL4DgWTpRgaCIKDOEojDaTZPlQPoB",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyHYMgiD4PAvgOBYThaBoIgoNYOhCEoUhmGIEhqCRNh0MxIDQYQyg4IAwi+Lwti2D4rDYII3jCOggDGLQwFYNYsi6OwwjOMxoC0MYghWJIjgWB4mDiPJBjeOYxjCPYvHYLQzkuIg8GMaRyGMbBlCAcoJhEIBjHiaQwCKax5gkMZvhML5hmOZQ+gE",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[cfg(feature = "sticker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5GEMguDQIIRhMMAghcMAtC4NwwDYLQxhyHhsC0MwuDUOA4iWJ4pEGFYUhKGIyhcMQ1CAMxIDWEIUjOGAtDKFB2DENI7kCGYykAMhokORY9hcMo/HoIg+DwL4DgWVJXgaCIKjWNx2jqDYNkeNAgDEaA1lOVZalmBJbgkTQ4mYMxoC4MAxmqVpum2BYHnAMQ2nOdZ3nmbICm6fpdjQNhzC6coNkAMRjiEM5IiCFIgoWe4B",
        categories = "social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Sticker,
    #[cfg(feature = "sticky_note")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5GEMguDQIIRhMMAghcMAtC4NwwDYLQxhyHhsC0MwuDUOA4iWJ4pEGFYUhKGIyhcMQ1CAMxIDWEIUjOGAtDKFB2DENI7kCGYykAMhokORY9hcMo/HoIg+DwL4DgWVJXgaCIKjWNx2jqDYNkeNAgDEaA1lOVZaD6AQ",
        categories = "text,social",
        tags = "post-it,comment,annotation,reaction,memo,reminder,todo,task,idea,brainstorm,document,page,paper,sheet,stationary,office",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    StickyNote,
    #[cfg(feature = "stone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgyDYNAgDKDgwDUQYRhEMAghqGg2C4NAyCANIOgwbAthEOBhhiG4shqDQzDUOQgDULoMDcbA2iKKoii2LIjDQMw4huOInjuGY9g0NQ3DaJwujEMomDILQ2kaPQwC2Ew1DOWJODMegiD4PAvgOBZhmSBoIgqDAuDmMgyiEMYRDGUA3C4OJvCAM41DgNJgmKZ5mgSaIJgucohDiHw3hILgzDAMp+mOgg+gE",
        categories = "nature",
        tags = "mineral,geology,nature,solid,pebble,crystal,ore,hard,coal,stone,rock,boulder",
        contributors = "Alportan,karsa-mistmere"
    ))]
    Stone,
    #[cfg(feature = "store")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyDEdgtDUYQxCCFQwCCGAwC0MYcGgLQ0hSFoZiSG4VhANQiD4PAvgOBYri6BoIgoMQ3C4Nw3DSFgwC4MwxhQLgxDKFpBkOGoZhwLg1DQOYkDKSoOlCR4dDMLg0DWF5EkKWpGiWSZLDiTpQk+DZTC2VZXl0MZFlyJZImuS5NhiZJRmWJJUjcN5nC4Mg0DgbJPDgOA5iCQQ4DQQZDl2GIVDeDhoDEMBhoqbonC4Ng1lUOA3DOgAuoODQ0kEOQypOY5Snee44jqVZ9DSKosjGMIEjKCRNjqkQuDkNRWDEOaTg6bpzo+QrAouTgtDKEQ4C4MIpiuLa0D6AQ",
        categories = "buildings,navigation,shopping",
        tags = "shop,supermarket,stand,boutique,building",
        contributors = "karsa-mistmere"
    ))]
    Store,
    #[cfg(feature = "stretch_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAaBlGkZxoHSCQ2gweIJDKDByhcIoZCAdxpGQdBohgMAiD4PAvgOBYoiuBobhiDIOhCEoUhaMYfiGI4lgyCAiDGC4oiqBB0D6AQ",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIBoGUaRnGgdINDIMIQHmGIQHiDQ0hAcofCIMgiD4PAvgOBYoiuBoIgqDIOh6DQxiEIIciWEIShSFoYhoIIjh2KIqgQdA+gEA",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA0EgORhDMIITDAIIWDALQyC4OITDQIg+DwL4DgWIYkgaCIKDENAgDEMhhiyLIYi2FwgDgSA2iCIoniEbBpG4ZQgHgMYJh8IB5kQIouCKQgygkMgwkweZOkoMo6C+PpAD6AQ",
        categories = "text",
        tags = "cross out,delete,remove,format",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Strikethrough,
    #[cfg(feature = "subscript")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CANQgDiDwiD4PAvgOBYThaBoIgoMQyg0LYQDiEoUhmGIEhqCRNDIMAgDEORoC0NBjDALQxC4NQuDQNAth6Ng1jyNxTiuLY4DMMwgkMMYyjSOQ3C0Lgxk4Lg5DOTw0DiNQuDIORhDKUAxkiXwgiyZJADYMpWjKVpeDKOwuDcNAumeTw5i0LgwDeI4VicPoBA",
        categories = "text",
        tags = "text",
        contributors = "nabanita-sarkar,ericfennis,mittalyashu"
    ))]
    Subscript,
    #[cfg(feature = "sun_dim")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCANBoC4MAxhqHIiiSIYjiWJwiE0MgwCCDoti+MQvjMaI1iSJooiqO4/jCG5CjaRY3igNI9DKS5BkOUJHjkMQ3C4Ng1DcIA2C4Mw0DOVpNlgPJDlqKZdl+YZcl6YJojKT5rjabZjmUM49m+dIukydo0niRo4E2e5mmKZJmnWTo0gEA",
        categories = "accessibility,weather",
        tags = "brightness,dim,low,brightness low",
        contributors = "mittalyashu,bduffany,karsa-mistmere"
    ))]
    SunDim,
    #[cfg(feature = "sun_medium")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME2DggDMdgxhmG4hiOIIiiSJgiigMggDIMIsi4L4wGiMojiWJwzCCDhoi2Go+jOQY0ieOpGDKSI9j+TZDCIbQxDgLgzDYNAgDULg2l0LQuDcMA3maaJUkwPI/lcbQ2lwNJFDGaQ2DUN5lmeaZ8myMZujOcJhmMNpgmKXZqn2a5KlWgZCjWWZ3nmRqToul5/kCAQ",
        categories = "accessibility,weather",
        tags = "brightness,medium",
        contributors = "mittalyashu,karsa-mistmere"
    ))]
    SunMedium,
    #[cfg(feature = "sun_moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgiD4PAvgOBYThaBoIgoMQ0C4OAzDcIAxDYLgzDgNRhDYIIrDCI4jC0NwuDIMgzjGM40GMLg2DINAtC6HYyDmMg2iUNwxDWI4zDQOBhDQIJPi6UggDWM4rlUMg1DmOg1DgOY/lmSQxiYOYlDAOZKj0OAukcNYShSGYYgSGoJguK4Mk6UAglMMAtj4NJvhWc5ygWB4JG0MZoDULZjlmd4zDUNqBnGApzoaCgyi6DBohGE6CheAQA",
        categories = "accessibility",
        tags = "dark,light,moon,sun,brightness,theme,auto theme,system theme,appearance",
        contributors = "zishankadri,jamiemlaw,jguddas"
    ))]
    SunMoon,
    #[cfg(feature = "sun_snow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDEdgtDEIg+DwL4DgWFYYgaCIKgwIA0FYM4UhaG4agSHIJguDQ5GEMwgi+DYyCCDQ2iSF4oieBYHgkbQxDSDoNDELgyDULQyC4NRMDEOAgkyN4mgKKI8CKPpAkCQ5Fg6SZLk2NoVjiGZSjuHY+DeDgxC0MwtDaTpJmoSAyDKUI5mOKZVDGZ5ri+bZDDWMJ0mKG5UE0MpODIaITmCUaDmUMpCDCEpJg6bp/nOi51o2PQzC4NpYDgLgzDYLg3C2pKBGiOp3G0NKhkCowzDSpqlqemIZgE",
        categories = "weather",
        tags = "weather,air conditioning,temperature,hot,cold,seasons",
        contributors = "karsa-mistmere"
    ))]
    SunSnow,
    #[cfg(feature = "sun")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCAMh2g+GwviKJIhiOJYnCKKQyisMIthqHIxGiM4kiaDBtDQLg5DMIJGkgIAxC4NAxk2Twxj2MI0kGNZEDENwuDYNpNlyXpSlCY5Ui+P5YkON45g4aIuj6Vw8j+ahNDIMJNDKbpVmico0mobQ2C4Mw0mCXQ2C2TpkomZpwjKfZCjYbQxDkLgwDeSpHDOiJTmWe5XgE",
        categories = "accessibility,weather,seasons,sustainability",
        tags = "brightness,weather,light,summer",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Sun,
    #[cfg(feature = "sunrise")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYOAiD4PAvgOBYThaBoICIbQ0C4OQzCAMQwh+IQxC4NAxiKKAxhKFIZhiBIagkTYNDEOBoDKLoVjKMYFgeNAyDCIo4jqE48heAoykCHAxDkLgwDeIokiALYnimK4pjuMJKj+G41g0MgyEiRovj2XYzhwOAgDYIA0C0NJum6W5nhmTILm2NxhnGcZDn4LZrDCdIXgEA",
        categories = "arrows,weather,time",
        tags = "weather,time,morning,day",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunrise,
    #[cfg(feature = "sunset")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAVgyCIPg8C+A4FhSF4GggIhtDQLg5DODgwiCIgxC4NAxg6KAxhOFYahmBIbgkTYNDEOBohKFIWjKMYFgeNAyDCDo4jqL49gKMpAh0MQ5C4MA3iOJQtieKYrimLo8hiSY/hyNYNDIMhIkaWhoj6M5MDYIA2C0NAgDSbZtlmMJcmiC5qjcYZum6Q59C0OAgDCc49gEA",
        categories = "arrows,weather",
        tags = "weather,time,evening,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunset,
    #[cfg(feature = "superscript")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CAMQ5CAOAtDgIg+DwL4DgWFYYgaCIKDEMoNDmEoShSFobhqBIcgkTQyDCDQyGgLQ0GMMAtDELg1C4NA0DILYgjcNY+jgU4thALgzDODJFDcLgwDAMo0C2Og3j0LgxDeUg5DOUg0DgNI2C4Mg5GEMpWDANQgmUMZnCCLpukINpXlwMw2GOXJlDIM4iC4NwzDgLpxl8Lg4DmD43DANolheKQ+gEA",
        categories = "text",
        tags = "text,exponent",
        contributors = "nabanita-sarkar,ericfennis"
    ))]
    Superscript,
    #[cfg(feature = "swatch_book")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDcYQ0CCEgwCCFQxC0OIWFYNRhDIIIfhWF4gC0MhoDSHoghaK4Nh8MhaCIPg8C+A4FjKNYGgiCgxDYLg3g4MxIDEOYpiGLIqDIdooi6K4XiWIBIDeMYzjiN4EjmCRNCCP4PGgLgwDGU40leVoFgeCRtgwIIaDILgziWboeC6EpthSRwznMMA0l+ehMDEOAuDaW6BnKdJzk2Dggl8MqCngNAzDQTA5C4OYOpMOJilWAQ",
        categories = "design,home,photography",
        tags = "colors,colours,swatches,pantone,shades,tint,hue,saturation,brightness,theme,scheme,palette,samples,textile,carpet",
        contributors = "danielbayley"
    ))]
    SwatchBook,
    #[cfg(feature = "swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDEVgzGgOAiD4PAvgOBYWhmBoIgoNggDENhoDmFYXhyG4Eh2CYLg0OQuDUaA3iaGIqD6AQ",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    SwissFranc,
    #[cfg(feature = "switch_camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDkSA0GEMgghQMAghcMQtDKGxWDeE4VhiIoNhwMhoDUIg+DwL4DgWKotgaCIKDEMwgDUaIfhSFojiEMh2DEMIgjuGYbhUaAtiiKosgQaIqGMaRyGMbBlCAYx5gkMQyCIIBygkM5bGMeJYlqSpPlGU4vkyB4JG0MQ4hWHAzC2NZyl+SowmmBZrCIbQ2iGdJzCCdorniAQ",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[cfg(feature = "sword")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDEIAxDkLQ2hEIg+DwL4DgWFYYgaCIKDUIAyDELQyiOFIWhuGoEhyCRtDiDoSDQIA0iaF4qimBYHgkTQ5C6HwxDePYgg0NhWDMaAtDMTA2kEMQ0j2NIogEA",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQLg1CAMQ3hQIAzCANoah6HYbheGYShQIg+DwL4DgWB4JieLIKHkMoQDEMwiCAeIyhEOY2HgMYzjUIB5j6OomiiL4ugiMJDDENo8kuTY3jkMgwjaMYQlORQvkcPIvjeS47lGVwxlWQwymOQY5DGO4nlqSYniqBpJgsb4Ng+EYThUNojDiHpmn0MYchaGIVDmJZsnCW5dkKMw0lWaQ4k6EA1jyOZrkabpcnKi4RDekQip2aJXlSYQio2bKJpqUpnpuaqekCOKSlmR4B",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Swords,
    #[cfg(feature = "syringe")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAyCANIQCIPg8C+A4FhSF4GgiCgxDcIIfDMLQzhOFYahmBIbgkTQxDkIIuDgLofi0LgzGMLQxCAMQtDILg1jqIwuhEMBsC0Lg2kYNo3juTI8j4IAwkENBMDGPw1iWFopiiBYHgkbYuDGOYRDSWIngKKZdgqP4tiMIIkhSWYYmeXIcguYwgDaeJllqAQA",
        categories = "science,medical",
        tags = "medicine,medical,needle,pump,plunger,nozzle,blood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Syringe,
    #[cfg(feature = "table_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMxIDUYQyCCEgwCCFQwC2EgyHYNBtDYLQ2GgMQwhGE4WicMYmhsNILg0dgxDgbYXiKJIaieF4ThkVg5i0Mgxg+JYUiiGY6DmMoWiIOAiD4PAvgOBQ+gEA",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "abejenaru,karsa-mistmere,ericfennis"
    ))]
    Table2,
    #[cfg(feature = "table_cells_merge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDEdgtDYIg+DwL4DgWFYYgaCIKgwIA5FYM4UhaG4agSHIJE0MwgDENRoDEOIkheKIngWB4qiwOYwjKFY0hkPByGUYx0CAdxpGSBYJjEIggGgZRpGcaB0kuMggHIeIJDKTR5gmIwglkIojj6QpED6AQ",
        categories = "text,files",
        tags = "spreadsheet,grid,row",
        contributors = "chessurisme"
    ))]
    TableCellsMerge,
    #[cfg(feature = "table_cells_split")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDUVg5CIPg8C+A4FhSF4GgiCgzg4NRoDEOIThWGoZgSG4JE2Hg5iGI4UhaKIUHIZRjHQIBoGUaRnGgdIJiIIggHmCQzkEch4gkMpBHcaRkgWP4jCCSAikWMI0jYPoBA",
        categories = "text,files",
        tags = "spreadsheet,grid,row",
        contributors = "chessurisme"
    ))]
    TableCellsSplit,
    #[cfg(feature = "table_columns_split")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIIMHYMgiD4PAvgOBYThaBoIgqDAgDIMIQhKFIZhiBIagmC4NDKIYThWJolgWB4oh0OIsiOL4CiaMoKDKDg1GgOIii6F45jGGxNj0MxoDYYY9j0MAglAMYeh4doMk2VJQlILZOEiEYtiSRYnjwIA5kCQphhmO5Ij0MY/C0NJojiapHDKSRolyWJPlGUZclWV5OnyWpUDIaJfjeRJ0iidplnicZgnOOpHDUIAzlaQaQheAQ",
        categories = "text,files",
        tags = "spreadsheet,grid,cut,break,divide,separate,segment",
        contributors = "chessurisme"
    ))]
    TableColumnsSplit,
    #[cfg(feature = "table_of_contents")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA1EgMwiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagmC4NDEOYhhOFYmiWBYHigMgxg4aAuDAMYii+F4CiaM4KjWHgyjiOo8iSP4yhsTZDiuRo7i6JIB",
        categories = "text",
        tags = "toc,outline,navigation,document structure,index,overview,sections,chapters,content,documentation,manual,knowledge base,faq",
        contributors = "karsa-mistmere"
    ))]
    TableOfContents,
    #[cfg(feature = "table_properties")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzHYMQ4CIPg8C+A4FhQchlGMdAgHcaRkgWCYRCIIB4gkM4lHmKIlGgZRpGcaB0iOEggHKJwiDKE4VhqHIUheBoIgoMgxCAORIimFIWgQaI/kyB4JE2RAggySI7kuGIBA",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table_rows_split")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDAaAyCIPg8C+A4FhSF4GgiCgxDUIAyDIdgtDiE4VhqGYEhuCYLh+Ig0iaFoqimBYHiwMoOhCEoUjKGICiqNoKDIMI5hGMYoj+NYcE0M4ODkaAxiWPJIhqQZMiCIgtDYYY4jgMQzh+RAxiALQylANJciAIA0mGDpqiINpHjOSYrgqTYiDKaY4mwIJEkSXZnnqDpgn2hQymUVo7iec5VksOJFoqPRojSdRNDmWIjlKi4+o2LKXi+coYgE",
        categories = "text,files",
        tags = "spreadsheet,grid,cut,break,divide,separate,segment",
        contributors = "chessurisme"
    ))]
    TableRowsSplit,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhQchlGMdAgHiCQzCIIByh8IgyiIaBlGkZxoHSCYRiIeYgiIdxpGSBYvhKFAvhqHIUheBoIgoMwgDkaIwjuQI/gSQYJE2RAxDWR46hWSoBA",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDAIggHiDQzhEaBlGkZxoHSDQxDSER5g0OIRHKEwiDIIg+DwL4DgWKhwGGCwgGSDRNDUIA0GEMggjsMAgj4MY8C0MhoDEMo6jyP5KkGOwyHYMQ2kiPZLkOPBokMLofioL4wguL4xGiM41DgIAxDgaAuDAMYpiuXRoD6AQA",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = "danielbayley"
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB3GkZB0GiCgxDaDRoGUaRnGgdIKDIMINHiCg0g2CYLg0coiicPg8C+A4FiwbBpG4ZQgHgMYUgyNgyjkLgwDGJY8CIMQ4iWOJDkWLAvjKNA+gE",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDUIggGMeYLDeDhjHiEgiD4PAvgOBYHhmHIGgiFYLDGE4PhEIolg6CoMhiGogh4PBwGEdBoCAZILE0MQyCCJRoDEMIuC+M41hmRI2jgIhtDMLg0DaPQwC4NQ0CANwuDAOAtlaWJCkcPoB",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,pills,pharmacy",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Tablets,
    #[cfg(feature = "tag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg1DgNggg2Dw2EEMoSCAMIZhsMQxC4MQ3hcMhIDQYYihuGgwC2Ih2DeH4hiaGIphuDoQCCHg0DENBsDgLg3DANAgj2P4lg0NAyhGRpIiiGwzC6R4ZGwNo1C2U4PiaT5LkqEYzikLZOkcegiD4PAvgOBZkGMaRyGMbBlCAZhpGwbIJGMdRyHIZRuHQQxvGwbxyCIIBjHmCYuDWghjHihoOoIcoJo2ZAvmqbJuD6AQ",
        categories = "account",
        tags = "label,badge,ticket,mark",
        contributors = "colebemis,csandman,aaofyi,ericfennis,karsa-mistmere"
    ))]
    Tag,
    #[cfg(feature = "tags")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLgxDcMggDIYYRhEMAghcMQgDELg0DENAuDUOA2GwNguDeGomiiFIdhKLYXhmGAgg0NAwDgbAtiANQ5hGOo8iwNIukGMIbC2NI2hiOIqDELZLEGFYyjENggDmDoQFYMxhhqGpElsLQxHoIg+DwL4DgWY5mgaCIKhENx2iaD4TlCRIXiGI4bh2H4lieKZ8kCQpRjKDQxDmHKEDOYpkmmYxjGkchjGwZQgGMeIJDEMIhCIIBmGkbBsgkYx1HIchlG4dBDG8bBvHKmhygmmaTHmCYmDWiQvo2j6RD6AQ",
        categories = "account",
        tags = "labels,badges,tickets,marks,copy,multiple",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Tags,
    #[cfg(feature = "tally_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBQ+gEA",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,one,1,first,bar,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally1,
    #[cfg(feature = "tally_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBoD6AQ",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally2,
    #[cfg(feature = "tally_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZD6AQ",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally3,
    #[cfg(feature = "tally_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviMPoB",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally4,
    #[cfg(feature = "tally_5")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviOMYaigMgyCANggj8MQ4i6IoXgEA",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = "danielbayley"
    ))]
    Tally5,
    #[cfg(feature = "tangent")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ3CKCR5gwNIQHKDAyCIPg8C+A4FgeGhwGEdBoCAZIME0MQ1C4NQ5CCKg0DGLgujAIIpisOYZhuIYjhqHYGgiFgihiCYLCKFIRgyDo5hyBI/iCIokiYIooDIIAyDIcwtDQLQ5C0MYrl6Xw1FOV41lWZQxhiGgvjsaA+gE",
        categories = "shapes,math,design,tools",
        tags = "tangential,shape,circle,geometry,trigonometry,bezier curve",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Tangent,
    #[cfg(feature = "target")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bh6BoIheGYWguDYPhGDA2CKG4dgSI4hjGB4JhiKYniaKwihqHIiiCAQA",
        categories = "gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[cfg(feature = "telescope")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALgwDYNQgDEMguDQOQzC0NguDEOISC4M4cGELoXDSIwzDQIAwimEgtDGGwwDgLQuDcMAyGyMg1DMNwthQMQ1GGLgwDeHpCiuKgxCALg2DkMYtC4MoQGwMQzC4NQwDQLYlDQNAiD4PAvgOBZemGBoIgqU5VDaEouDcNJDiUMwzDKMg5DKXJemCBBomOeoHgmC5qDKTZUgwNYZk8MZdl+ZJ8gWfgiE0MYaDQOIRDWIw0GEMggpuKpHh4NA1oaFA0DINZSg4OYyDINwykCEpGrCLqChSrQ3qgNY+CCGoPq+SKeiyM6sh6ggzjeQQ5k+raapysZNqSppOqENR6oqeZigKfZmG2Go+h0OJKnGHgxDGKJaDUNrWoy2aOtuHaCCChAwoaGqCuqepeGMaRyGMbBlCAYx4gmEwiCAcoJDLBRjHnAwzta+r8v4PoB",
        categories = "science,development,tools",
        tags = "astronomy,space,discovery,exploration,explore,vision,perspective,focus,stargazing,observe,view",
        contributors = "karsa-mistmere"
    ))]
    Telescope,
    #[cfg(feature = "tent_tree")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINAigkeYMg4IBygwMgiD4PAvgOBYHhkcBhHQaAgGSDBtDENAgDUIAzC0M4siyGIaiCIofiGI4lCKJ4pDEMIsi6MAzjIL40GiNoiiSDBNDENwgigVoXhmRI3keOJKkyTg0EgNxsC2Kw4GgMgwFqQ5FlWSQiE0OJZHYOJllQPJFmgbQ5lmKggm6UpmgE",
        categories = "travel,nature",
        tags = "camping,campsite,holiday,retreat,nomadic,wilderness,outdoors",
        contributors = "danielbayley"
    ))]
    TentTree,
    #[cfg(feature = "tent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NQgDIMQgDENAgDMIg+DwL4DgWGIbgaCIKDIMINg+EQxDCFYXhmHodgSH4JE0MQ1iSEISDKEg1GwLYMg4NoqhqLotgWB4wjeEBoiKP4sgE",
        categories = "travel,nature,sustainability",
        tags = "tipi,teepee,wigwam,lodge,camping,campsite,holiday,retreat,nomadic,native american,indian,wilderness,outdoors",
        contributors = "MoltenCoffee,ericfennis,danielbayley"
    ))]
    Tent,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDkaA4CIPg8C+A4FhSF4GggIhtDSDg3CANgtiOJYThWGg+gE",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "colebemis,ericfennis"
    ))]
    Terminal,
    #[cfg(feature = "test_tube_diagonal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3CANguDgMgggwLgxDgYQyhIM4VhwIAwiAIAxC0MwuDkOQtC4MAxhqH4bDiHYhiGDQwC0NBMDGDwzCIPg8C+A4Fj6QYGggIhtDENoVhCEI9j+RJDgSRYJE0MYUkkSA0k6QJSD6AQ",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TestTubeDiagonal,
    #[cfg(feature = "test_tube")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMh2DEN4OGMMAgDELg0C2GQxhCDgtDKDofDUY4choIAwiGIIdisNYuFYMgiD4PAvgOBY0jeBoIgoOIjDIaA3jONY6jmBI7gmC4Ng8MQ2GgLQ1kONpHD6AQ",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube,
    #[cfg(feature = "test_tubes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMh2DENwuDUQQyhKDYWDAIIZDEIA2hYMgyhSH4YhoIIcDSJg5hIVgyCIPg8C+A4Fi+MoGgiCgyhmDoQhIYYVDWF5AhmGwtj+QY+iOQolDGRYSk0NYsi6MI1jSBI2gkTQzg0aA3lKMZWlWBYHlgMYoDKXJelSApWmOCoMDENhIDSaZgmuYo3E2OYmDYaAtDWdIzgEA",
        categories = "science",
        tags = "tubes,vials,phials,flasks,ampoules,ampules,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTubes,
    #[cfg(feature = "text_align_center")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgoMQ3CAMQyEgN4ShSGYYgSGoJE0MQ5h8ORIDWJIVigPoBA",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignCenter,
    #[cfg(feature = "text_align_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgqDAgDEMhIDmEoUhmGIEhqCYLg0MQ5EgN4jhWJw+gE",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignEnd,
    #[cfg(feature = "text_align_justify")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANRoDEOAiD4PAvgOBYThaBoIgqDAxDKD4RhOFYEGiGIkgeCYLCAMQ5iCEoUhkPoBA",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignJustify,
    #[cfg(feature = "text_align_start")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgoMQ1CAMQyhCEoUhmGIEhqCRNDEN4fDmIoThWJw+gE",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignStart,
    #[cfg(feature = "text_cursor_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAtDEYYNg0MAghcMQtDKG4Oh6F4Zh0MhIDYIg+DwL4DgWJ4qgaCIKDEMwgDgaA3hSH4YCAMYeDIdg0jeFo5hqFYRDeJooi2LIEi6CRNDWOg2EiP4VjmIYcj0LZTjiGYOhsaAxkeKZLkqBYHk0NggDSX5AlWOo8m+IJuleX5hkmApLmaCg5CANh2gydZjgEA",
        categories = "text,layout",
        tags = "select",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    TextCursorInput,
    #[cfg(feature = "text_cursor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAyDIaAtDEYQ0CCFQwCCGAxC0NIcFYNoUhaGYjDGFocGgMQiD4PAvgOBYri6BoIgqDYPiiIYXiOGIdDQdoSiqLIxjCBIygkTY1jeFY5hqIo9imK4tkQPoB",
        categories = "text,cursors",
        tags = "select",
        contributors = "ericfennis"
    ))]
    TextCursor,
    #[cfg(feature = "text_initial")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA1GgNgiD4PAvgOBYThaBoIgqDAgDEMoQhKFIZhiBIagkTQzh4ORoDEOIihWJolgWB4JG2KofCAMwuDUNQzC0NwuDcMg0GGPI8CAMJJh4IAuDgOQ0kkTAxDGHgyjCJICiaNYKjsOQyh4MIQC4MYRhOMYXgEA",
        categories = "text",
        tags = "drop cap,text,format,typography,letter,font size",
        contributors = "jguddas,GRA0007,karsa-mistmere"
    ))]
    TextInitial,
    #[cfg(feature = "text_quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIA1EgMwiD4PAvgOBYThaBoIgoMgxCAMQyEgOIShSGYYgSGoJE2HYfDmIokhWKIngWB4qDOHwyHYN4wiaAQ",
        categories = "text",
        tags = "blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextQuote,
    #[cfg(feature = "text_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA1EgMwiD4PAvgOBYThaBoIgoMQwCAMQyhCEoUhmGIEhqCRNh2Hw5iKE4VieExjGkchjGwZQgGMeIJDENwijkeY8DWPxygmEYvjONY3iaBYHgkbYMiwLQxC4OZTlWI4wheAQ",
        categories = "text",
        tags = "find,data,copy,txt,pdf,document,scan,magnifier,magnifying glass,lens",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextSearch,
    #[cfg(feature = "text_wrap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIIMC0MwghGEwiD4PAvgOBYWhmBoICITYRDEMhoDENAuDUYQxg4IAwiyK4tDcSAxDOFYXhyG4Eh2CYgg4ORoDaNYYjmOIFgeO4RDWJA4kGN4BA",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis,karsa-mistmere"
    ))]
    TextWrap,
    #[cfg(feature = "theater")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQwHMMwtDMIIRDgIg+DwL4DgWGIbgaCIKDKDIOHOEomhWF4Zh6HYEh+CRNg4IAyGMMAgDQLg0hILg2CAOAtDiPYphqLYsgWB4vDENIyjSNo4hSO49lGFoYkOHICi2R4hg2D4Ml0IA1kKK5XkaIILiODwtl2aZfmGRJji6CpADENRolOKpuh6WYLjIMh2C0MRhl6NY1DGMppGgNKBjIIKDg2i59DGbZWnmZZJnyfqAoKjKODKh6JpqhKPHakZUiuAQ",
        categories = "buildings,social",
        tags = "theater,theatre,entertainment,podium,stage,musical",
        contributors = "danielbayley"
    ))]
    Theater,
    #[cfg(feature = "thermometer_snowflake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAyDALQxC4Mg1C0MguDUTA2CAMQ4CIPg8C+A4FiCI4GggIhNgwIA0CAOAuDcNQgDaGIzjOH4hiaJYEieCYqDCGA4jIMQ1EiDI4iKPI7gWB4+DKHAyGiNIZisOZIjqAo8k2KYPhwNIYDQYYti2DQxhwLZkFaYZPk+DZliwIAwHqV5KlmTIoG2LYrhKMgyEyeg0nSJJ2j2Cg3g6ZgzC0NoRhgLQzoIaJLoUbaHDMIKXDYaAypEPoBA",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,cold,freeze,freezing",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ThermometerSnowflake,
    #[cfg(feature = "thermometer_sun")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgiD4PAvgOBYThaBoIgqDAgDgYQ0CCIQwCCJAwC0MQuDYNA1CAN4qDQN4ShSGYYgSGoJE2DYMGiEYThWN42gWB45DKJAxDQLg1DSIIiiUIAxlALYjFaTINg2JpQk4MB6jOQIXgKN5ECIbZJDkM4iC6Z5QC4NJRimbpejWYZDhsbQ2C4M4hDGLw2DaKJtm+gZykGAQ",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,warm,hot",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    ThermometerSun,
    #[cfg(feature = "thermometer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0HYMQwC4NQ0GGDYNDAIAxhoLYYFaFQyCCIYZhmG4YFoIg+DwL4DgUPoB",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Thermometer,
    #[cfg(feature = "thumbs_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4C4MQyg0MINDQSA0hANxhhKEoUhQMQtDELg5DILQyC4NQ2GyJwzDMLQ4EGHAgh6DQgDaKAgDISAyDCG45jOQAxj8Mh2DiPodkGJo5GiJguDcNpHkCHohk6DIiDEMRMhGOQyGEM4QDMIJfDGYY0iCLpfDgOBaCIPg8C+A4Fm6cYGgiCgxDeFRWDKbZvnQPoBA",
        categories = "account,social,emoji",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[cfg(feature = "thumbs_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA1C4OA4CAMQ0hMMBog8OAzGEMggh0MAgiAMYTC4OYdDILg1DYbAtigMwzCAOBBieIY1iMMQ3imHgyEgNIch6NYii2LR2C0OI/h+Noei0aIoDcNpIkGNguDcOQtDELgxDETAxicYQzlmMJgDGMIgiIIJihAOBaCIPg8C+A4Fm6cYGgiCg3hYdpdm2b50D6AQ",
        categories = "account,social,emoji",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[cfg(feature = "ticket_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5jKHYnDQLQ0kWR5MD6AQ",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,booked,purchased,receipt,redeemed,validated,verified,certified,checked,used",
        contributors = "danielbayley"
    ))]
    TicketCheck,
    #[cfg(feature = "ticket_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoOYyDIaA2kWR5MD6AQA",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,remove,cancel,unbook,subtract,decrease,-",
        contributors = "danielbayley"
    ))]
    TicketMinus,
    #[cfg(feature = "ticket_percent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIAxhQIITDYdgyGGDIMhOHwgh0aAxDaHIhheKITDILQyHaLIPhGKIVhUMAtDYVg3iaHopiyLBIDSOopheLIhFoIg+DwL4DgWSJLgaCIKDmDRoC4MAxkeSZOk2BJPgkbQxDWDY2CANpYkqXJbgWB4JE2YIUDWVJWmaWoBA",
        categories = "transportation,shopping",
        tags = "discount,reduced,offer,voucher,entry,pass,event,concert,show,book,purchase,%",
        contributors = "danielbayley"
    ))]
    TicketPercent,
    #[cfg(feature = "ticket_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoOYyDIaA2kWR5MkuBJNgkTQxgwOR2lSRpIlkPoBA",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,book,purchase,add,+",
        contributors = "danielbayley"
    ))]
    TicketPlus,
    #[cfg(feature = "ticket_slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5C4NYyDSUggDULQ1kWR5MD6AQ",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,redeemed,used,marked,checked,verified,spoiled,invalidated,void,denied,refused,banned,barred,forbidden,prohibited,cancelled,cancellation,refunded,delete,remove,clear,error",
        contributors = "danielbayley"
    ))]
    TicketSlash,
    #[cfg(feature = "ticket_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5C4NYyDSUggDULQ1kWR5MkuBJNgmUJWlGU5lluSJfD6AQ",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,cancelled,cancellation,refunded,used,void,invalidated,spoiled,denied,refused,banned,barred,forbidden,prohibited,delete,remove,clear,error,x",
        contributors = "danielbayley"
    ))]
    TicketX,
    #[cfg(feature = "ticket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoMYQDWGpFkeTJLgSTYJE2UIyDeU5GkiWJXgWB5alwMQxl+VZigEA",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,perforated,dashed",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Ticket,
    #[cfg(feature = "tickets_plane")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1CAMQ3GgMQuDIMg3GEMgghoMAgh2HYUDMNA1C2DgyEwMQ4hAMgiD4PAvgOBYujGBoICIbQxhoMQzg4II8DcOQ0g4MA2i2L40jOBI1gkbY8hEMwgDgLophAMQtDWGYbh6WwxhsLg2DQNwuDkOQziiUg1DaUZGjCSpJgWB4JE2aoMFYOJskiApKnGCp0DQdgxnibp6nCNpzhAOR2iyLptjIPByGUYx0CAch4gmLAgHcaRkgWlwwCIIB5gmdwgGgZRpGcaB0gmO6gpYIqLi+kKSD6AQ",
        categories = "transportation,travel",
        tags = "plane,trip,airplane,flight,travel,fly,takeoff,vacation,passenger,pass,check-in,airport",
        contributors = "jguddas,karsa-mistmere"
    ))]
    TicketsPlane,
    #[cfg(feature = "tickets")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzC4MQ3DMIA4g0OAgDEMQtDUYQyCCGwwCCHgxhwLg2DQNwuDkOQzEwMYSDUNoRCIPg8C+A4FjKNYGggIhNi8MQwFYOIxjOOI3gSOYJjyFQ0HYMZCjSRpFgWB5Ij0OR2DKTpEDwchlGMdAgHmCZBCAch4gmWAgmYIpoHcaRkgWZwwCIIBoGUaRnGgdIJDEM5OlyXg+gEA",
        categories = "travel,account,transportation",
        tags = "trip,travel,pass,entry,voucher,event,concert,show,perforated,dashed",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Tickets,
    #[cfg(feature = "timer_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyGgNAiD4PAvgOBYThaBoIgoNAuDYIAxDEYQ4CCJINieIAuDeJYqiWLoog0OItDGKoShSGYYgSGoJE0NwuDQII+DSI4vCCDQxiAMAuDOIIuiaRpNC4OZJC4Mo2hWOo5gWB4JG0MoOg6DQyDCV44gKOpcgoMZfmsdgtlaE5YheAQ",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    TimerOff,
    #[cfg(feature = "timer_reset")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyGgNAiD4PAvgOBYThaBoIgoMQyCAMQ0HYLYRhOFYEGiGIngeCRNDSHwzGEOAgjKDYNDGMwtDeM47jaHwtDULgzh8NBMi4MQ3C4NoShSGYpgWK4KDmHw3EiIQ1kuJoXgE",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis"
    ))]
    TimerReset,
    #[cfg(feature = "timer")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDQIggHkMYKDKDh4hGCwwg4eYJCKEw+DwL4DgWHohgaCIKDENYUhYMYTg+GwxDGGYrg2HoggQZYeGMaRyGMbIGHKCg4g4Yx5ieDQgGMeInh2H46jyPg+gEA",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDmDhjHmCwxDIIg+DwL4DgWB4bHIZRjHQIB3GkZB0GiCwyDCDhoGUaRnGgdIXDSDoRCKGYJjkN4OhYIg1hqHIiiQPoB",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDENYOGMeYSDIIg+DwL4DgWB4bHIZRjHQIIRCKGQghcIoUCAdxpGQdBogsMgwg4aBlGkZxoHSEg0g4conDeGociKJA+gEA",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    ToggleRight,
    #[cfg(feature = "toilet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQyGgMQzGEMYNCAMIWhWFIUDUIIcheFwxC2HA1GgLQuDUOQ4GGJ4nhiH4mDQMg0C4Nw2DUbAxicNA0CAMguDQN4rDWLYfg2MIyjSNhIkMNAwDKQpEhiIY/jKJo1DUTIMDEOAiD4PAvgOBZemGBoIgoOINiqI4ukYNYiFYNBhDKPZshQMgtg+KpznORZ2j0dpcl6YIEGgPoBA",
        categories = "devices,home",
        tags = "toilet,potty,bathroom,washroom",
        contributors = "EthanHazel,staffanmowitz,karsa-mistmere,jguddas"
    ))]
    Toilet,
    #[cfg(feature = "tool_case")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDUaA0CIPg8C+A4FhSF4GggIhtDENAuDgMQ3g4MAuDkOQ1C2Jw3DELQxC4NA1g4LgwDMNIvC4MgzDIYQyCCP4NkILQyjUMoqDOOgzDgbI5DiRQzDYNBMDmJwxCCSQ4DgNY+kAIJCl8LZJDaRwuDcNA4EwNguh+V5rDUNZNjAN5lDQMg2l2QZfmGbA5liZg1DYbAuDYNQ3joN4ThWGoZgSG4Jh4OIhDKP4MieKZAjoNgtDULpLGGV5XmAMIrnAN45DMMZpDGa4pDSmQ5qCDp7kOMAyiILg1DMM5NicMqvkUMaUoqFqOo2BYHgkTavsKNQwDYQahrSs4giizAxEgMaxtKYLSDEdg3nm04uj8MhIni5bjkSRB6sSjIB",
        categories = "tools,development,home",
        tags = "tools,maintenance,repair",
        contributors = "AlexNaskida,karsa-mistmere,jguddas"
    ))]
    ToolCase,
    #[cfg(feature = "toolbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDIdg0CIPg8C+A4FhSF4GgiCoMCANhhDIIIiDAIIlDGDguDQMQ0C4NQ4DYbA0CANBBiKJImg6I4ig8dg3iGI45icLY3EgNJAjiQwykQdgtj+N5CjqLowC0MYqiyMgtjWUIlicIA4h8eoThWGoZgSG4JguDQ2FaR5ckKRJEGiWpIlEMJEiMdgymOFpnmaBYHmmPA0GgMgwnyZYCmegYKmCPYShSfYYgE",
        categories = "tools,home",
        tags = "toolkit,tools,trunk,chest,box,storage,utility,utilities,container,kit,set,repair,fix,service,maintenance,mechanic,workshop,construction,hardware,equipment,gear,handyman,engineering,craft,diy",
        contributors = "karsa-mistmere"
    ))]
    Toolbox,
    #[cfg(feature = "tornado")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA0EgMwiD4PAvgOBYThaBoIgoMQ4CAOBIDaEoUhmGIEhqCRNDEOQgDEMhIDmI4VieJoFgeKQxDaLQ2GgLYihOM4XgKJ43hyDQyDCMIyiWAQA",
        categories = "weather",
        tags = "weather,wind,storm,hurricane",
        contributors = "ericfennis"
    ))]
    Tornado,
    #[cfg(feature = "torus")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch4D0IgzCIIBjHmDQxDGEByhMIgyhAY4MCIMYbD4PAvgOBYHGWIolgaCIRhmIAuDWHIeiCF4zDCF4ZDiMAiiKJIEiuKIBA",
        categories = "shapes,design,tools,food-beverage",
        tags = "donut,doughnut,ring,hollow,3d,fast food,junk food,snack,treat,sweet,sugar,dessert",
        contributors = "danielbayley,jguddas"
    ))]
    Torus,
    #[cfg(feature = "touchpad_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAdgtDYIg+DwL4DgWFYYgaCIKDEOQuDYNQ2CAMQ0EgMgyhSFobhqBIcgkTYNiYaIMiuF4vi6BYHgkbYNj8MIODCN4tgKL48gqD5CEgNBhj8IJBkEMQtDKVBWDaToOlCWwxg6VJEjmRo7h0TYgiKJIng+WYNlGJZaDIdgxDALgzDQNJghmAQ",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    TouchpadOff,
    #[cfg(feature = "touchpad")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAch4g4MoSHmDg0hIdxpGQdBohYMIShUIoXD4PAvgOBYoHAYYgCAZIOE0MggDENBoDKI4oC+Logi2LxojGMwxjWOh2C2EY8j4aA+gE",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Touchpad,
    #[cfg(feature = "towel_rack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA3GgLQyCIPg8C+A4FhSF4GgiCg2C4NQgDMaAxDEQQyh8IIniAMAgiwMYpiwNYfFYMgwGGL4viyLgtjiEA5jcII5i2QY8jwVoyDWQJCjoLYrFYMQ3kqQ46kGQRoDSE4VhqGYEhuCRNDmDhIhKFIWl0PoBA",
        categories = "home,travel",
        tags = "flannel,bathroom,toiletries,sanitation,clean,fresh,dry,laundry,laundrette,hospitality,housekeeping,room service,spa break,health club,amenities,hanging",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    TowelRack,
    #[cfg(feature = "tower_control")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLgyCAMQyg4NwgDIMAgDYSA0GwMQuDiGITGEMYQCCF4mCALg5DULg3DMaAxDALg1iKJInheKQ2C2LAzFoIg+DwL4DgWP5CgaCIKh8MQzHYOY+kCRZEgSRoJgsNoVDIdgtk2P5BlKUYFgeCRtDmGIkDeTpdkOApSmEIhtDENYYC2I5nlyUJrmCR4Lg8NhWDKaJ3kWbYLDOFRoC2f52l6AQ",
        categories = "travel,transportation",
        tags = "airport,travel,tower,transportation,lighthouse",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    TowerControl,
    #[cfg(feature = "toy_brick")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJDiDBoGUaRnGgdIJDEMoMHcaRkHQaIYhAIByggIgxCIPg8C+A4FikcBhh8IBkgkTQxDAIA4FYNRjDALQuDaPg0C0MZDkMSA2GEMQgkqN5NkOSx2guKQvi+H4ujAaIyjQMQ5jiOo8j6QAukKRJlGgLQzkmSwgk2bJPDGUYoiqVRoD6AQ",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIAxDGDoQC4ORhhCEINg2Eg4g4LgxGwLQuDYNg1CANIdDUOIVg4IIYg6IA5DgOAuDgNAyEgMgwCIPg8C+A4FjuPoGggIhNDENoODgaAtDWOo8kGQIEkKCZFhsNYqheLIsC2Fh2DULg1DcM5Nj2UZQgWB5TDOJRojIMQyhSFpZhkIITDmMw2DMTAxmqDwuDINA2mOT4ClGaJEDSERWDSgploSZ5DE0N4ODUaAuDAMaMj+jpSkSG4Mh2iqZGiOxjGkchjGwZQgHKCQyCIIBjHmCQxDirxjHis61jsL6lqeqakqaqKqqwIpMrCsgiDGxq3gkN5jr2wg+gE",
        categories = "transportation,sustainability,food-beverage",
        tags = "farming,farmer,ranch,harvest,equipment,vehicle",
        contributors = "danielbayley,jguddas"
    ))]
    Tractor,
    #[cfg(feature = "traffic_cone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLgwDUIAxDALg5DYNhhhAMguhAMAgh0MQtDgLgxh4Ig+DwL4DgWJ4qgaCAiG2DIUDIM4RDSDg0DkII3DQOAghoMA0GGJIkh2Hwgg4MJEC4OAzDEbIhhsNw0CAM4UGEMo/h6W4gDELoWh6UIiDWUwtlYOQxkOEZbkcMAtl6TRsjwOA0C2QA0iaKItiyBIugmC4NDmOY2iOQoZhuXIRC0OYUh4TISg4No1lYNZYlqRprlYOJTh4ep5imfZ8gWB5/owMaCCCDZkhiP6IpiHQ1l+RafnuAQ",
        categories = "transportation",
        tags = "roadworks,tarmac,safety,block",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw"
    ))]
    TrafficCone,
    #[cfg(feature = "train_front_tunnel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyFYMQyGEMQwCCFAghUMYWg2FQwHaFAiD4PAvgOBYiiWBoIgoMQ1CANguDiHwuDQYQzg2MIYhsMQtDaGBWi8OIhiOKIngSKYJE2F4sGgLgwDGQokkaRYFgeSAxDSFg1kyTpQkSApGlWK4ZDkYZYliHYWC0NJqHYLQzGGPY9hmOoMh6b5mjmGZqCANBal2UpflSKhtDmFg5C2DAzn+JqBkcIhtiyhoNCCioilGJoBA",
        categories = "transportation,navigation",
        tags = "railway,metro,subway,underground,speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFrontTunnel,
    #[cfg(feature = "train_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMwuDEVg3GENAghQMAgheF4MDAVoODEIg+DwL4DgWIYkgaCAiG0OQgDENQtDGMIgiKJ4mgSKIJG2LotDWLYyiGI43jaBYHgkTYsDEORjC0MguhsLYvk0MpQlAdgtDQYYMhuGIti0NoYHYNBjheTQ4kwLgyCCL49DUWozkGJYCjeRYqgySZMg2b41nKRIpjqX5JCCaQznqQoBA",
        categories = "transportation",
        tags = "railway,metro,subway,underground,high-speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFront,
    #[cfg(feature = "train_track")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ3g2DwyCIPg8C+A4FhSF4GggIhtgwMQ0CAOIihOFYahmBIbgkbQ1g0MYiiSFIWimKIFgeK4jjmMYmjSAopjeHQxi+LY5iWM4Yj6NocG2IAggyRYyieSYqgqEYMDKDA3kaJ4BA",
        categories = "transportation,navigation",
        tags = "railway,line",
        contributors = "danielbayley"
    ))]
    TrainTrack,
    #[cfg(feature = "tram_front")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAch4g4MoShUIg0hIdxpGQdBog6EISHmDgzCIPg8C+A4FikcBhiAIBkg4TQ0CAMQxGiI4pC+L4gi6MBojKNAxDIIAzHYOIoiqPhokCMYzCIbQ4jcOQtkaJ48k2T5ClEbQxlQMgylcLZZkyQZckMIhNlQMQ1GgLgwDGS49mgPJNmoTYQjeb5xnOWpogEA",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "transgender")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDYdg2CIPg8C+A4FhSF4GgiCgxDQIAyDAaAtDSE4VhqGYEhuCYLDiIBoDQdolhSFoqimBYHgkbYNg0NwuDENwgj6QImjWGICiqOYKg0NQuDMNQ1FYMhoDOTg1DeRYokiOIcjuPAtkOQZhlmNpbiuCouDUIJqDiZJHGMaRyGMbBlCAYx4gmDAinYeZ5DKexygmM4VnCcp0D6AQA",
        categories = "medical,accessibility",
        tags = "gender,inclusive",
        contributors = "jamiemlaw"
    ))]
    Transgender,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDEdg2CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIZiWB4JgsOQgDYdoeGEMggjKDYNDELYyDISA3jGMwgjWDo4jgVoihWGongWKYKDOLRoDEOITkaJZIhuKg4i0Vg0j2NI/g6M44GiWo5l2No+DIdgylGJIYgEA",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash2,
    #[cfg(feature = "trash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA2HYMQ0GEMgghQMAghcMQthQMhIDeE4VhiIoaDKGxWDYIg+DwL4DgWKotgaCIKDODhoDEOIpiuMIvgSMYJE0OIOFaEociKGYVhsaJEiGF5HhwdgyjmLI9D6AQ",
        categories = "files,mail",
        tags = "empty,deletion,cleanup,junk,clear,garbage,delete,remove,bin,waste,recycle,discard,binoculars,rubbish",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash,
    #[cfg(feature = "tree_deciduous")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ5GENAghEMAghQMQtDILgyDQLQ3C4MwyEEMwuDUIIjiWFIWCAOQgDYLgwDMVg2GEM4mhWDY4DaFR2i8NI0iSJpAimOIjhsIA1C4Ng1EGEYTjcMYNjqDhaCIPg8C+A4FlaWYGgiCgxDKDQ5HYM5VleXA+gEA",
        categories = "nature,sustainability",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TreeDeciduous,
    #[cfg(feature = "tree_palm")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA4GMMAtDILg3DaEguDSFg1C0NQuhsNRTDIIIdDINAgiIOBoDIbAxC0MQgi8MRoDQIg+DwL4DgWNo5gaCIKgwIA3C4MQ0EGHQ4iKR4iDAIJMjENoeCANhjDMLgwiaTIdDWJwuiWI5RDUaAtDMbIumaMZiDONY3jyO4Ej2CRNkcOQgDmFAxGOFwxluE57heDYdDQNwtC4M5bkINJkDSXQ0C2iwyh2g4UnehA3i+fQynoMp5DELg5hunQ5hqXaDp0OAtoiE6GmuOJvm6BYHnEMYxloY5RhOG5DDcIKLqCUoejMY6ZlqhKgpmLZEqybYB",
        categories = "nature,sustainability",
        tags = "vacation,leisure,island",
        contributors = "ericfennis"
    ))]
    TreePalm,
    #[cfg(feature = "tree_pine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDQIAzhELgzGEMYOCAMIZg4LQug0MYeEgNIehaGIahoMYdDcLYgDcTIfDQaIdhWF4XieHIeiyHhMDkIA5jILgyEGNYbigIA4CAN4UEwMQyhEbIQiONImhuKQukiLRIDENRshIM4UiWNpViqDohgwWgiD4PAvgOBZqm2BoICITZNCAMgyHYLQzmma5wD6AQ",
        categories = "nature,sustainability",
        tags = "tree,pine,forest,park,nature",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    TreePine,
    #[cfg(feature = "trees")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMHYLgyEEMwghSDYNDEIA4C4OYODYSA1GGFIWCCGAtDELQ1C4OBWgyIoViWMYZDaJRaCIPg8C+A4FjiO4GgiCg3h4dg2jeOY+j2BI/gmC4UDEOR2DORo6kqSYFgeTAxDKDg5GiGwzGGGYZheMQuDeJ5mEwMQ4g4NBoC6YJijGZJmmgN5qjSXYRmGDpzmUOJ2mqIxsmgNIOC4NZTkiAQ",
        categories = "nature,sustainability",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere"
    ))]
    Trees,
    #[cfg(feature = "trending_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDcaA2HYLQ2CIPg8C+A4FheGoGggIhtDIMoODcLQ4C4NYmigLQ1CANRMiMN4WhiHQ+gE",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingDown,
    #[cfg(feature = "trending_up_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg4DIOAggyDoQCAMgxhYMQiD4PAvgOBYch+BoIgqF4SDYdg1GgLQ1huHYiiGBIjgkbYmDMLQ5CAOQtDSPAtDYIA2i6HoyjGBYHgkTYmDgVgziuLYckSIIB",
        categories = "charts,arrows",
        tags = "arrows,estimated,indeterminate,data fluctuation,uncertain,forecast,variable,prediction,dynamic,volatile",
        contributors = "Alportan"
    ))]
    TrendingUpDown,
    #[cfg(feature = "trending_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA3GgNh2DYIg+DwL4DgWFYYgaCAiG0Mgyg4LQ4C4NQgiQNQtikNRMiEMQ3hSFobD6AQA",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingUp,
    #[cfg(feature = "triangle_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELg3DMIAxDgLYTDENBhDIIIZDAIIcDALQzC4NA4h0bIUhENBBhmG4di0NIaDEaAxDaGIai2HoRg4M4gCIPg8C+A4Fj6QYGggIhNDGGQ5HYNI9j+RJDgSRYJkiGQxDcaAuDAMZOkCUg+gEA",
        categories = "notifications,shapes,development",
        tags = "warning,alert,danger,exclamation mark,linter",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    TriangleAlert,
    #[cfg(feature = "triangle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxDcIA0g4OQzGEMgghcMAghoMQgDMLg2iELgwDEMwiD4PAvgOBYoiuBoIgoMQ0hgMRoDKJ4pi6LYEi+CRtDENQuDgN4zDcLpEDMIIdDGRwzjeKIqjyO4FgePgxDiQg0DmSgyC4OQ1heS5Nk+OZSgKPJVgoMpMDgMozleDg4haGIbnUMQtmwMw1hiQpujiUYsmeVIwE2Egwm6NJzhmdp4kIMw5C2XQ4o+f46oKPQiG2QYlDaXJemCeJKmOlZmi6aRNDiNI2qSgamjAbZYgyF5GkioZMDeTqsGgPoBA",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,Yohh"
    ))]
    TriangleDashed,
    #[cfg(feature = "triangle_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgYYNg0MAghQMQthISAzGMLQxC4MYVh0LgzC0Lg2iUNIiDMTAyDALg0CANIjGMLg5iUN4OiaKI5DYLg3FoIg+DwL4DgUPoB",
        categories = "shapes,math",
        tags = "volume,controls,controller,tv remote,geometry,delta,ramp,slope,incline,increase",
        contributors = "danielbayley"
    ))]
    TriangleRight,
    #[cfg(feature = "triangle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg3DMIA0GEMgghQMAghcMAtg0NA2hgbAtDgIAxDQQYUhaGIpDSFQxGgMQ2hOFYphmI4ODOGxaCIPg8C+A4FD6AQA",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Triangle,
    #[cfg(feature = "trophy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDQLg2DYdgxhEMg2GEMgghqDYNDELQuDkNw2g6EQ5DYQQ1CCKodCCDQ3huFYiDgIg+DwL4DgWNo5gaCIKg+DoQhKFIWhiGoci6SYhiOJQ2ieKYrkmHoOjAMoyDeNI2jiBBojuXIHgmCw4CAORohUNRhDGDpSlILQ1EgMZZjePJegWYIKDSGwymYNo1nOXJ1j2YYkDkYYkiSLYehwVgzmma6JC2HwxEgN6OmqkJqDEep+luOoCl+PhNoQSIQmimZspebhIn2Wp0gE",
        categories = "sports,gaming",
        tags = "prize,sports,winner,achievement,award,champion,celebration,victory",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Trophy,
    #[cfg(feature = "truck_electric")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDkVg3GEMgghQMAghcMAtDKGxIDkIg+DwL4DgWIYkgaCIKDENYODmHogiKJ4mgSKIJgsOYtGgMhhDGDoYj+FwxC0MR2C0MwuDYNY8j6GYYC0LgyhySAyEwMQ4C4MwgDmWA4kuPZNhoLg3DiTwzDgSIMjCI40jOBYHjaFAxDMdpKj2X5Aj4MY6mqMoCjSb4Kg2WQyC6KwgDWhZKC4NA5oijIsmAIJYogOA2jqiRhosN6aniPaTC6lRMlmWwwDKfJsDwYxpHIYxsGUIBygmpggGMeIJDENwirQea3h+IQvqqrKuiGwatq8Y68CKD66rEIqzrWCa5r+xbDgEA",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry,electric",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley,jordan808,LienMaas,jguddas,AnnaSasDev"
    ))]
    TruckElectric,
    #[cfg(feature = "truck")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDgVg2GEMgghQMAghcMAtDKGxIDSE4VhiIoahQMh2DEMRhDGDojiKKwxGgMgiD4PAvgOBY0jeBoIgoMQ1g4OBIDmM41jqOYEjuCYLDmQIxiqLIZi4LQxHYLQzC4Ng1k+K5RhoLgyhyWAyDQbJWC4NA4C0NAuDMNRBi+LYXDENwuDWFJBgyRI2kiNBjGkchjGwZQgHKCYyCAYx4gmcwiogeaLDiep+oCgp9n+gaDomCQ3o2hQiocY6PCKD6SpelYB",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley,jordan808"
    ))]
    Truck,
    #[cfg(feature = "turkish_lira")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA0CCDQ5CIPg8C+A4FhSF4GggIhtgwIA4C4NQtDEMIQhOFYahmBIbgmCw4CAMQyGEOQgjWJomDELY1DkVgziiFosD6AQ",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "jamiemlaw"
    ))]
    TurkishLira,
    #[cfg(feature = "turntable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDILgwDEaIRDEIg+DwL4DgWGIbgaCIKDEOAgDgdg0GGI4jg2DQxC0MYRDcIA0heGYehgYxpHIYxsGUIBygmMwgGMeYJg8IpCHiRQwjQL44jqPIYHIZRjHQIJECKQRoGUaRnGgdJFDaR5JCIMpHHcaRkgWCQykuPpjmWGAvlKVA+gEA",
        categories = "multimedia,home",
        tags = "record player,gramophone,stereo,phonograph,vinyl,lp,disc,platter,cut,music,analog,retro,dj deck,disc jockey,scratch,spinning",
        contributors = "karsa-mistmere"
    ))]
    Turntable,
    #[cfg(feature = "turtle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDAIINDQdgzGEMYOCCEIahgMRoDKFoYhuEAxC0MR2C2FQ4CCKojhmJQ2hmFIgheIoch6M4ZjmI4licMxsDILQ0GgNBaCIPg8C+A4FkeSoGggIhNDQLg4g0NwuDmK4ODCRpIk2TIEk6CRNDENQuDGKpWliDJalySZgl+BYHmIMQ2lcM5aEgMgwGGDYNhuNAgDSeZtl6AQ",
        categories = "animals",
        tags = "animal,pet,tortoise,slow,speed",
        contributors = "danielbayley"
    ))]
    Turtle,
    #[cfg(feature = "tv_minimal_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgwDMMwgDkLg0DQYQuDYNA3hiGggDCHggDGIAxC4MQyGwLQ0g4Ng1CAMguDMNQyheGYNjWIIfDELQuDkNg4jsNQ2FaG4ZDiNA0jaSI4iEII8DYN5ADYegiD4PAvgOBZVliBoIgoN4uDEaAxDCVJWluVRyGUYx0CAaBlGkZxoHSCQxDQIggHIeIJDKdx5gkM53HcaRkgWe5kCCeginyVQvmmaw+gEA",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,movie,live,ott,running,start,film,home cinema,entertainment,showtime,channels,catchup",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    TvMinimalPlay,
    #[cfg(feature = "tv_minimal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgxGgMQwCIPg8C+A4FhQchlGMdAgGgZRpGcaB0gkMQ0CIIB5gkM4oHIeIJDKKB3GkZIFjCEggi8IoxhQL4ahwPoBA",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    TvMinimal,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAyC0NQgDWEIQCIPg8C+A4FhcchlGMdAgHiCQyCIIByiIIokCAaBlGkZxoHSCQxDWJR3GkZIFiMMIlHmCQ3haGIdh8PoBA",
        categories = "devices,multimedia,communication",
        tags = "television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,entertainment,showtime,channels,terrestrial,satellite,cable,broadcast,live,frequency,tune,scan,aerial,receiver,transmission,signal,connection,connectivity",
        contributors = "colebemis,ericfennis"
    ))]
    Tv,
    #[cfg(feature = "type_outline")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDYLg1GGEYRCAMIWhiFA1GiERhDIIIfheFwxhgNBIDmHoghiI4WC0NIchKGorhkNQthEdgtiiMoii2EY2hsLQzhMNYVjwMI/hEVg4imIYYDGLoWFaEofk2LAyC0Mhog+TIziSVB2kKVJdlAMI4h2O4rj+PxokGQ5FmmGhaCIPg8C+A4FD6AQ",
        categories = "text",
        tags = "text,font,typography,silhouette,profile,contour,stroke,line",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    TypeOutline,
    #[cfg(feature = "type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0HYMQ2CIPg8C+A4FhSF4GgiCg0CANxWDUYQxCCJAwCCJ4kDELQxGgMQ0iOJYojOKolHYMoThWGoZgSG4JE0OQgDIMBohKFIWj0PoBA",
        categories = "text",
        tags = "text,font,typography",
        contributors = "colebemis,ericfennis"
    ))]
    Type,
    #[cfg(feature = "umbrella_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg3GGDYNDAIIWhYNIXCIPg8C+A4Fh2IIGgiCoMCAMh2DKHIeiOIoEiSCYLDgLg2DUNoODMaAyC4MwzDYYQxg6F5EhYLg5DcLQxC4Mg3hoMQwkwOJPlEMpTkWF5KjwMJJDcLg1DGLIfjCL4FgeCRthSKIWDIMJii6AowmeCg1kcNpCnUOQ1hKUJSg6VQ4lgMAtDORwyg2dQ3DYORBkKQoYkQM45GiUJvmSAQ",
        categories = "weather",
        tags = "rain,weather,uncovered,uninsured,antivirus,unprotected,risky",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    UmbrellaOff,
    #[cfg(feature = "umbrella")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMdg3GGDYNDAIIWhYNIXCIPg8C+A4Fh2IIGgiCoMCAMh2DKHIeiOIoEiSCRNDIMAuDkOYNg8YQxg6F4+haNg3C0MQuDIN4aDGNQyDiSJKkyP4XkMOY2DIM4XEGPI8hiPpWg8eosh+MA+gE",
        categories = "weather",
        tags = "rain,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Umbrella,
    #[cfg(feature = "underline")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CANB2DYYYMgwMAghSFAxDKFRWDQIg+DwL4DgWHhsGkbhlCAeQygkMgwCKKAxiuLQgHiMAihyM4qCKLIdh+JImD6AQA",
        categories = "text",
        tags = "text,format",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Underline,
    #[cfg(feature = "undo_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ0CCDw5GwNQtDUIg+DwL4DgWGIbgaCIKhEaAxDALg1GENYmCCKQ1CAMIug2K4qiyKIziqL4vDGFY2DUSAxDGF4Zh4PoBA",
        categories = "text,arrows",
        tags = "redo,rerun,history,back,return,reverse,revert,direction,u-turn",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Undo2,
    #[cfg(feature = "undo_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDcYQ5CCEgwCCFQwC0MQ1C0NguDcTAzg4MwiD4PAvgOBYliiBoIgqIQ3HYNhoDaJImiuJRjGkchjGwZQgGMeYJg8IggHKQpEGMeJCDKNQvjmO49D6AQ",
        categories = "text,arrows",
        tags = "redo,history,step,back",
        contributors = "danielbayley"
    ))]
    UndoDot,
    #[cfg(feature = "undo")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANx2DYaA2CIPg8C+A4FhSF4GgiCgyDEIAxDcYQ5CCJAwCCJwwC0OYriWLopigLQ2CAMguDMTIMDEM4ThWGg+gE",
        categories = "text,arrows",
        tags = "redo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Undo,
    #[cfg(feature = "unfold_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDIaA2CIPg8C+A4FhSF4GgiCg4g4MhIDKE4VhqGYEhuCYLDIIAyHaIoUhaJ4mgWB4pg8IA4i6I4xhiAonjWCo3DENI6jCJY+jSHIqiwMJFiSMpIigIhtDEOYODUIAzC2WpcjuR4akAbZYDmW5ZmYM5ejKAQ",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    UnfoldHorizontal,
    #[cfg(feature = "unfold_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDYIg+DwL4DgWFYYgaCIKgwIA4FYMoUhaG4agSHIJE0NAggwSIjhWF4oieBYHioMQwi0MhIDiJIyhmAoojaHg2joaAtjCJYzkGNYdE2D5GkiPomkyKQiG0MQ1i0OQtDMIAzl2XZTkuG5DliWg1mGYJeDOY4ZgE",
        categories = "arrows,layout",
        tags = "arrow,expand,vertical,dashed",
        contributors = "danielbayley"
    ))]
    UnfoldVertical,
    #[cfg(feature = "ungroup")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINQiCAeYJDSDB3GkZB0GiCQ4gwaBlGkZxoHSCQ2gwcoICIMQiD4PAvgOBYoiuBoShSFgihiB4JDGJggiONoZhuHYfCKIYNjaEIoiqBB0D6AQ",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,separate",
        contributors = "danielbayley"
    ))]
    Ungroup,
    #[cfg(feature = "university")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAyDEdgtDMYQyg4IAwheFwtg0MB2DMIg+DwL4DgWIYkgaCIKDEOAgDEMhoC4MAxiCIoniaBIogmC4sDENowjKNIjjiN4FgeOgyhUNxhDGLYZhgMAtDGURoC0MoUhaT4tlELoMDMLQuDMNQ5EwMQzlwNAzg6YA2leFZZlAMguDIOA2l8MIyEwNpnmkNQuDYNBBhWboZkwNQgDYSITkyTJvlGLR2i6bZOhmghoj2kpvg6VR6kGNoCjiRoKDaLYvjGM4hkKJafkWKRNqOPY/qeNZDDwYxpHIYxsGUIBjHmCQxDAIq8HivwysIcoJsaqK2riug+gE",
        categories = "buildings,navigation",
        tags = "building,education,childhood,school,college,academy,institute",
        contributors = "karsa-mistmere"
    ))]
    University,
    #[cfg(feature = "unlink_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA3GgMhhg2DQwCCFQxhYIAxDAaAtDIbQtDaFhIDcQYThmF4Og6EAiD4PAvgOBQ+gE",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink2,
    #[cfg(feature = "unlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgLg4DQIAxDILgyDWEQuDcMgtDGGAxGgLQuDAMhhDWIQwhCJQwicIAwiyLIghILQ3iENwgikMA2jaJo5i2PQtDYLg5hYMBshuGAyhcNwxCIPg8C+A4Fk2UIGgiColDGNQxhwNw1kaSpJDGJImiiY4uj0IAuhIIIzDCNY3jmb5mi6QJCiwbJbDGXpLk2T4EGiTRsGkbhlCAeAxgkOAioUMqIooeaHCIMqOowIg1kyTqBoOgKCoShoJpKi4JpYIKPo2pKUomfKZGWm6Dqen6gqUIgxDmiqerMNq2pQMa5qqnKtoQea7rmhaQrSuqwo6xq9piv4BA",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unplug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIA1CAMwtDMIg+DwL4DgWFYYgaCIKDIIAyh+EYThWF4EGiGongeCRNDYLgziAMIvGEMguDSII2CAMI6jwM45DATAxh8MQ4GwLQ2kcLY1jCS40jmNY3juUo9jYWoUhaG4pgWKwiE0Nwug8MY+mGOwxDGV4mhmAoqh0TQxjKYYumGMAxDSaJZmuW4dguHw2CCfp+kuSozlCOJRjyUoSjaRo1kijZOjehZTDCipRlaJZZgEA",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet,disconnect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Unplug,
    #[cfg(feature = "upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHaDAiD4PAvgOBYThaBoICIbQxDcIA4C0NYiiIIA1hKFIZhiBIagkTQyDEIAxDUdg0GGDYNDAII6DELY4EgNY3CCOY7jKPo+HYLQ0iiFYsD6AQ",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[cfg(feature = "usb")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMIOGMeYLDcIg+DwL4DgWB4bh6BoIhaCwyhSCYShWEQiDSGociKIA8HAYR0GgIBkgsTQ0C4NwgDEOQuDOPw5CANYvC+NI2huSo3jkIhtDIMQgDMLZDlMMggDIWpIk2TI1k6OpBDINggDcLg2DiRo/DIbJZkeG5JmCX42jiCxthOPw0muWQzC4NZWn+XZzjOYJ2lAMZqDGWQxC2U6Po6P5cnGXoBA",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAxDEIAyhEIA0C0NAiD4PAvgOBYZhyBoICITYMhEMR2C0MhhDSFAgDCLYthaFhIDaKosi6N4WhQdgyhiGofhkYxpHIYxsGUIBjHiCQ5CKRx5gkN5MHKCYXhkL5BkORQ+gE",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDUSA2GENAghSDYXC2FA0HYMgiD4PAvgOBYfiKBoICIbQxDQLgzDANYODYLg1DMLg5DIMwtiwOIdh+IYEGiJI/geCYpDULgyDIOIOjQOA1DKOY2jiOgzh6IIlkGBZDigMYxk0MoODKR5JjkMw4lKUZVj6I4CkKJ4pl2ToODcLg3DeT5TjUMg0mmV5slmbgxDkLoqkoMZhkgOJTlCN58j+WImkSgYyDODqJDYOQ2mSO6LnuPZ9iWWhtDIMJ0naSwul6eQ0pqVKeo6fqQiio6ll+XKDDSiY2iuZatlarxjGkchjGwZQgHKCZUCAYx4gkMQ4CKyh5s0NZpsCwrEh+1rDsUY7SCIN7QsuCQ5tCxwip2ILatiAQ",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis,UsamaKhan"
    ))]
    UserCog,
    #[cfg(feature = "user_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDEdg2CIPg8C+A4FhSF4GgiCoMg4MxoDKE4VhqGYEhuCRNDMIAyhALQyGENAgjKDYNDGMwtDQaA2jGMwgjWDosC4MA3DILg1DcOIjhaJ4UGMaRyGMbBlCAYx4gkMQwCIIBygkNJbGMeYJDeS5PlGU5OlCUpUlaCYMmCYgiDEOZbl0IoihQL5mmsPoBA",
        categories = "account",
        tags = "passkey,password,login,authentication,authorization,roles,permissions,private,public,security,person,account,contact",
        contributors = "colebemis,csandman,ericfennis,mittalyashu,karsa-mistmere"
    ))]
    UserKey,
    #[cfg(feature = "user_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDYdgtDIYQyCCFQwCCGAwC0NIZHYMgiD4PAvgOBYiiWBoIgoOQuDWDg1EgNxhh2HYahmHAgDSH4hiOKIiGMaRyGMbBlCAYx5gkNwiCAcoJDSSxjHiCQxDCPAvkCQpEiIchlGMdAgGgZRpGcaB0gkNZLHKUgiC4OA5DmS5ICKD5LmsMQzksdxpGSBYJDiVpcl4PoB",
        categories = "account,security",
        tags = "person,lock,locked,account,secure",
        contributors = "anthony-mariotti,jguddas"
    ))]
    UserLock,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwijceIJDmPBygkNIqiyMYzjWKxsGkbo2HkMYJDEMY8HiUAiDKKQgHkMpRlMIB4lwIoMkUL5Lk0PoBA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg1CAMQ1EgNxhDQIIWDAIIZDALYWDQdgyCIPg8C+A4FiOJoGgiCgyg0Mw3DiEA2C4NgyDYYQxhCGo7hwMwuDAMA0C2PpADQbIdj+OQ0kkMhhDIIJPhuGgtg4MIzDgNZGlQOIvlALg4hSDoOjyO40DKZhsmeYJbi+TpQmSGZflmVA1lYeoiiSKYjGMaRyGMbBlCAcoJDQIggGMeIJDEMKGGMeYJDeeAvnyfqAD6AQ",
        categories = "account",
        tags = "person,account,contact,profile,edit,change",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    UserPen,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx4gkOQiCAcoJDSPBjHmCQ3iqLIxjONYrGwaRujYeAxgkMY7CAeAylKVB5lEIg4jweZXCIMZAisL5Mk6S5NjaWpSDGPJWlINpulsMopCCX5skaZZpD6AQA",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[cfg(feature = "user_round_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCKLh5gkOI0HKCQ1iGI4oiqLIiiWBoICIbQxDaGA5g2TA0C0NI9iSBBoD6AQ",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserRoundCheck,
    #[cfg(feature = "user_round_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLgzDANQgDEOQuDUMwuDkMgzC2Dg4DIIg+DwL4DgWIYkgaCIKDENQuDIMg4hINguDgNQyhyGYbh0M4giKJ4mgSKIJguMo0DKEosi4OIcDMOI5jiPIjkCP4FgeQgxkSNQgDIMAuDcN42jqGAyDSUI+gKQJViqFIMjCK4ti+Oo3hqZZSmeVIpguFIWloMQuDYOQ2kuHpymSIZRiWdpBCITZGDIMRhjCMAwCCkwxhKXA0DMNAtDefofoaZonmkbZbl2X4xjONZipuOp0oiop4qWXpGhMLpsquragnUYxpHIYxsGUIBjHmCQ4CIIBygkNbHGMeIJDEMJQryvrAiG06/sGyQijuwrOCIMbGsKxLfsahrXtWAQA",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    UserRoundCog,
    #[cfg(feature = "user_round_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDEdg2CIPg8C+A4FhSF4GgiCoMg4MxoDKE4VhqGYEhuCRNDIIAyDEYQ4CCMAwCCMwxg4MguDgNg4C0NguDMNA5iOFonhQYxpHIYxsGUIBjHmCQ4CIIBygkNZSGMeIJDEMJDkeSZLkaSJKkyTpakKU4JiKTZZCKDJdmKYIBA",
        categories = "account",
        tags = "passkey,password,login,authentication,authorization,roles,permissions,private,public,security,person,account,contact",
        contributors = "colebemis,csandman,ericfennis,mittalyashu,karsa-mistmere"
    ))]
    UserRoundKey,
    #[cfg(feature = "user_round_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx5gkOAiCAcoJDWMxjHiCQxDCIYjiiKosiKJYGgiCgygwMQ5GiH4+iSBBoD6AQ",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundMinus,
    #[cfg(feature = "user_round_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMAuDiDgtDcLg0DgNwiD4PAvgOBYliiBoIgqDguDMN4RDENguDYMg2GGF4XhSPQtDMLgwDANI/kGQxsC0NJBheSgwDEMhhgyDI9hULQuDUMI1DgNQ0kiG4xg2Gw3GGV5XhWZ4UjYMpqGya4ilYOIxlGDZomeG5claWA2HqJImiuJRjGkchjGwZQgGMeIJhkIqHHmCQ4owcoJDWfQvoGg6FD6AQ",
        categories = "account",
        tags = "person,account,contact,profile,edit,change",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    UserRoundPen,
    #[cfg(feature = "user_round_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCKLh5gkOI0HKCQ1iGI4oiqLIiiWBoIgoMQ5hgNh2iCIokgQaJCk+B4JguDJHGiH49k6JoBA",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundPlus,
    #[cfg(feature = "user_round_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIOAiCAcoMDWDxjHiDAxDAIg+DwL4DgWB4bHAYR0GgIBkgwTQyCAMgxGEOAgi8MAgjIMQghgLg0DMNAtDcLg2DKGociKJIbh6BoIhEIgzhSCwiDGDoJhaToOhuHYEkeIYjiWJwiG0Moql8LQxC4OZimSQQvkMaA+gEA",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass,lens",
        contributors = "jmsv,karsa-mistmere"
    ))]
    UserRoundSearch,
    #[cfg(feature = "user_round_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMQuDgNwzC0NwiD4PAvgOBYiGMaRyGMbBlCAcoJDUIggGMeIJDEMIyGMeYJDiIYjiiKosiKJYGggIhtDEN4YkoNQgjGIokgQaJDlKB4JG0MoMkkLZNk+I5ED6AQ",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserRoundX,
    #[cfg(feature = "user_round")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNYQGMeYMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQyDAIAyDEYQ4CCMIrjMLQxDYIAwhqHIiiQPoBA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserRound,
    #[cfg(feature = "user_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINwigkeIMDEMIPHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME2EwuDMIAxDUSA3GENAgjIMAgjUMAtjINB2DKGYbiGI4ah2BoIGOEQiDGDgghYIgzg+CoSg6GocgSRIgiKJImCIbQyDEIJdC0MQuDmYZjj4L5AGgPoB",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass,lens",
        contributors = "jmsv,jguddas,colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserSearch,
    #[cfg(feature = "user_star")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLgwDUMQgDEMguDaDBhhGEQwCCG4ZC4OQwDmDgyDQbAuDcMw3hILg0DUMoYhKHIyhuJ4pC4NQzDUbAxhUMw0C4Mg1DaMIajKEY3DgOIrDgMA2GwLY8DENwyiuDA4kSM4cC2QA4hQOA2k6QA1DmK4WDOWIbh2UI3DSHgxj6T48DQNg1lsNw1miM5bDmE4cnGLJ0ieeIZlkMZrjgOZrm8M4mkEOJrmaeZplsMpdluX5OoqQqamehJpjGN5TmuXQyjuPQzpSQqSjKNQ3luOA1HoIg+DwL4DgWtK3gaCIKkoMQ1EgNxhDQILEp8MAtsQNB2DKs61rqtBjGkchjGwZQgGMeIJDEMAiCAcoJDS3hjHmCQ3s4L7StS1g+gEA",
        categories = "account",
        tags = "person,account,favorite,contact,like,review,rating,admin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas,MArtytraM99"
    ))]
    UserStar,
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwijceIJDmPBygkNIqiyMYzjWKxsGkbo2HgMoJDKKQgHgMYJDGOwgHmVgiDiPB5lAIgxDORQvkuTZKkyTphliPJVlGU5bgmXpamyZIrmaag+gE",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgOYUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeYJDeOxjHiCQximKwvjGM41D6AQA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYYNg0MAghQMAtDENoVCIPg8C+A4Fh0YxpHIYxsGUIBjHiCQxDAIopHmCQ4i8coJDWHIeiOJYnh2IIGgiCgyDKDgwGOFwzC4Mw3C0MgtDYLg1C0NAtDgYQ1CCV4WhULQuDSUQ4kmOIfgQaA+gE",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    UsersRound,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYri6BoIgqDAgDMLgxDIOIhheIwxiMNwuDcNA0iqLIxjCBIygkTQyDKDoQhKPIkhkLQzlYLg4DeRotkqKxjGkchjGwZQgGMeIJDkIggHKCZFmYeYJluKwvmCYpkD6AQ",
        categories = "account",
        tags = "group,people",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Users,
    #[cfg(feature = "utensils_crossed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAyC0MguDODoSGGE4TDAIIZhsIA0C4MhsDELg4CCIg4hYIIYhqK4eDKGhMDKLg4CIPg8C+A4FjWOIGggIhNDENYlkEM4SimFYth2H4rhyGQ2GwN5FlAMxjC6UA3g4IJVhSJAwEyQJCFobYclcN40jaO46gSPIJG2EQxg6JggDYLg0C2cwzmaN5qmmBYHmwMQ5CANQtmSeZogE",
        categories = "food-beverage,travel,navigation",
        tags = "fork,knife,cutlery,flatware,tableware,silverware,food,restaurant,meal,breakfast,dinner,supper",
        contributors = "karsa-mistmere"
    ))]
    UtensilsCrossed,
    #[cfg(feature = "utensils")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DcYwwCAMQuhQOYNhgMhoDQYQyhiEogg0LQyFYMgiD4PAvgOBYoiuBoIgoN4NHYMgwieKYui2BIvgkTQyDGEw1iUYQ1CCRYhDALZFDUdg2hGE4VC6F4elQaAzFobYgg+N4qjsPoB",
        categories = "food-beverage,travel,navigation",
        tags = "fork,knife,cutlery,flatware,tableware,silverware,food,restaurant,meal,breakfast,dinner,supper",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Utensils,
    #[cfg(feature = "utility_pole")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgwCIPg8C+A4FhSF4GgiCoNDUaIRhOFYahmBIbgkTQzCAM4QiKFomiWBYHigN4ri2FIvhiAomjOCgxjWLAyi6JI7jKHBNDIMY2kKOJEhqPRtDEOQgDULY1DeVpWkOMIBA",
        categories = "buildings,home,sustainability",
        tags = "electricity,energy,transmission line,telegraph pole,power lines,phone",
        contributors = "karsa-mistmere"
    ))]
    UtilityPole,
    #[cfg(feature = "van")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA2HYNRhDEIITDAIIWhaEwxGgNguDEMAyhKFIXiSEwuDcMQyC4Mg5DgbAuDiLQuDkMYihWJQgisOA4ieIBWDEN42iSGQthoaAtDMIg+DwL4DgWS5OgaCIKDWFA4EgM5ChiFJFkUVg4GEMggmKW4TDILQyGiKRjDGHokiqbQ4mMLg0hSMBsm2QA2CANIrkqTJRlCBJSgkTQ5lYaA1n+TaDksYxpHIYxsGUIBjHiCQxDYIqVHmmA4pscoJDKi6PpGk6OpCkqUqEIqjpWlwiDemxjp0Igxp+SwvqWqg+gEA",
        categories = "transportation",
        tags = "minivan,cart,wagon,truck,lorry,trailer,camper,vehicle,drive,trip,journey,van,transport,carriage,delivery,travel",
        contributors = "Ahmed-Dghaies,karsa-mistmere"
    ))]
    Van,
    #[cfg(feature = "variable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxHMLQ0C0M4RC0OQghKF4ZCIPg8C+A4Fh2IIGgiCgxDYIAzHMNIphgIA5hGL4xDmHIeiOHRsGkbhlCAeAxgkMQ1CIIB5j8Io0kQMpAkKPZKkeNQvjmO44jqPJFgmSB4k6QZDHmW5Mj6WJQlIZQ+gE",
        categories = "development,math",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[cfg(feature = "vault")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMGgZRpGcaB0gkMQ4gyD4KgwdxpGQdBohmGw+DwL4DgWJxjGkchjGwZQgGOHQ3C4NYMGOCAijaOAgGYaRsGyCRjHUcoDG4dBDG8bBvHKDBygmNwiicL4ti+MYnHAYYjCAZIJG2Ng5CCYggDILg3maaJUiiW4jiyLowjKOoJj2DJAkKRJGkiSpMk6UJSj6NIZDaU5VlecpalwaJemAMQzC4NAgDEMAuDaag3C2Zw3mwL5uGicJYnONZTj+QZDCKRZHGWSZLk2T4NoGOY7DGhY4oecZZDyn6NCKYQumOtQuDGmKamuVafqGcqxCKpaDCKwqCrStp3qeeqrq2fpPrioqKl2X6+o+kaTpCkqbpinbJDyiIxjOHQxhGzLxnS0IRtyiYBA",
        categories = "security,travel,home",
        tags = "safe,lockbox,deposit,locker,coffer,strongbox,safety,secure,storage,valuables,bank",
        contributors = "danielbayley"
    ))]
    Vault,
    #[cfg(feature = "vector_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg1CANxhDINAghMIAwhcIAxhkMQwCIPg8C+A4FiCI4GgiCg0g6EIShSFoYjCHIeiCIoEGiJY2geCRNDeGoNDWLYVhSMYYh2F4fiGJo4gWOoKj2KpAhaL4chqMJIjWJA8HIZRjHQIB5gkMQ3CIIByHiYZkGgZRpGcaB0gkNZkHcaRkgWcJkmcIpileW5diCfZemaaAgnme5fgkMpynSdginEIJqmybp3jSgJ/lyXp5omZaFoqdRonej5rm2b6NmSYJ6mOlKXpaXQgnOnqgpCo6gpmpqImSgp6nyq4BA",
        categories = "shapes,math,design,tools",
        tags = "shape,geometry,art,width,height,size,calculate,measure,select,graphics,box",
        contributors = "chessurisme,jguddas"
    ))]
    VectorSquare,
    #[cfg(feature = "vegan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA4HGDQwCANgthSEYVhMIg+DwL4DgWG4egaCIKDENwuDQMQgDMLg1DkYQxhKMAgjGM4qiqGociGIIEiKCRNDIIAyGEMg2C6DZEkaNY0jIMgwGMLg5hULg4kAMYsC0OYsCANAtDENI4h2PA+gE",
        categories = "food-beverage,sustainability",
        tags = "vegetarian,fruitarian,herbivorous,animal rights,diet",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Vegan,
    #[cfg(feature = "venetian_mask")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDEYwtDELg1CAMAtDKFIUC0MwgDIIg+DwL4DgWIYkgaCIKDQIA2GEMoehaMYXi8Mh2DQYYVhUMIyCCOQgg2DY7jsMY9jCQI8kQNYYkWOo8koNRWDiLowkKFoYhgaIcGGR5VheFYvlyMgtkoMh6iCIoniaBIogkTQ2g6EITk2GQ1hQIIdh+IYjmsPoBA",
        categories = "account,gaming",
        tags = "mask,masquerade,impersonate,secret,incognito",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    VenetianMask,
    #[cfg(feature = "venus_and_mars")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDAaA0CIPg8C+A4FhSF4GgiCgxDIIAxDYdg2hOFYahmBIbgmCw3g6ER2hKFIWimKIFgeCRtDIMYOC0NQuDQNggj6QIljOGA8GMaRyGMbBlCAYx4gmHgik8eZSDGVBygkNZFkmS5ND6AQ",
        categories = "medical",
        tags = "gender,sex,intersex,androgynous,hermaphrodite",
        contributors = "jamiemlaw"
    ))]
    VenusAndMars,
    #[cfg(feature = "venus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDUdg3CIPg8C+A4FhSF4GgiCg5g4ORoDaE4VhqFBjGkchjGwZQgHKCYiCAYx4gmDAijEeYJDmIwvieKYrD6AQA",
        categories = "medical",
        tags = "gender,sex,female,feminine,woman,girl",
        contributors = "jguddas,jamiemlaw"
    ))]
    Venus,
    #[cfg(feature = "vibrate_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiHotgeCRNg4OB2DEMBjDAIAuDUNQuDQNQgDGQpCGgNhjjyQY6DELY/kGTAxHaEoYhqH4vgWMQiE0MQ2kIMAuDMNBWkcMJNj2TZAC2UJqGgLQ0mANJUiyHQ8GwaRuGUIB4DKCYjCIIB5nwIp+noMZ9n8eaGoOcp2ngPoB",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiGRyGUYx0CAeYJDUIggGgZRpGcaB0gkMQ0jgch4kCOB3GkZIFgkOI4kUIpNiuMIyD6AQ",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DYIA2EgMQ0GEMgghcMAghoMYYhgdgyC4NRsDULgyDQOAtDMLgwDYMhBiKIobjOHQyhcNwuDgNx2DgLgxDkNgiD4PAvgOBZDkaBoIgoMYQk2FoehqHAthcMhIhWVYzlMMpUFYOJQhmNIYlQaAykKRJJkiBJKgkbZZDKGpwmeRZrD6AQ",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAxDMIA1C4MgyhAMwuDQOAyGELoSDUIAwiCIguDeJQthiDBWDcLg4DeHIeiKIQwicNw1DKJw0DMMhMgyDgwh0Ig+DwL4DgWQhyGUYx0CAeIJDIIggHmCQ2lAdxpGSBYJDENJQGgZRpGcaB0lqTwgHKTQik+QgvkiSg+gE",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoIgqDB5gkNIMHcaRkHQaIJDIMIMGgZRpGcaB0gkMQ2CIPg8C+A4FikcBhhkIBkgkTQyCAOBohyKIqi+GYpGMaRyGMbBlg2G4MGOEgiDGFAgGODw4jsL5AkKRIujAaIyjQOAgDEMholGKQvj0aI/kGQ5FkmJZNHKR5Og+JpSlSaA+gEA",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = "danielbayley"
    ))]
    Videotape,
    #[cfg(feature = "view")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDcdgyGEMgghQMAghcMQthQMhIDWE4VhiIoaDKGx2hsIg+DwL4DgWKotgaCIKgwIA3FaH4ciKFwwhuG4eiCFo6huFYRimK4wioYxpHIYxsGUIBjHiCQxDIIggHKU5WGMeZTlWKgvkqTJOi+BIxgkTQxDgLg5DQNIODILgzDMYYNg2O5CC4Ng2jULg1nyfp3jwMQzC4OKGiODo6oqeZ7DefZ/oqGaEoYOIYkaLJlD6AQ",
        categories = "design,photography",
        tags = "eye,look",
        contributors = "zenoamaro,ericfennis,csandman,karsa-mistmere"
    ))]
    View,
    #[cfg(feature = "voicemail")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeILDaDhjHmCwxDIIg+DwL4DgWB4bh6BoIhWF4ZgmC4Ng+EQiDEOIahyIogDwbBpG6CB5DGF4TCAeQyjuDh4joIo8HiP4ti+GwvjWNw+gE",
        categories = "connectivity,devices,social",
        tags = "phone,cassette,tape,reel,recording,audio",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Voicemail,
    #[cfg(feature = "volleyball")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELgxCAN4OGEMQ2C4NQ1CCFIWhgMAgh2DwxDALg5CANAiD4PAvgOBYoiuBoIgoMQyhkMoTDILg2jSOIejwMQtDgLg3CANYnimLotgSL4JguFQ4hkM44hOFYXhmU4cj0LYkhGRIoiqSZIgWB5LDKIpCDEN42C6ToymqPIdDCP5BC2GAxlAM5PC6d5vhmHgtiGRZeiyApJmKCoVneUA4lKG5Voye4fiOGYNlyRpfDwYxpHIYxsGUIBygmfwgGMeKgDIIqiHmpaApimqcD6AQ",
        categories = "sports,gaming,travel",
        tags = "beach,sand,net,holiday,vacation,summer,soccer,football,futbol,kick,pitch,goal,score,bounce,leather,wool,yarn,knitting,sewing,thread,embroidery,textile",
        contributors = "danielbayley,jguddas"
    ))]
    Volleyball,
    #[cfg(feature = "volume_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0C4NwwDIYYQDANYVDUIAwhqGgtDELgyDAMwtC4NA5DgTA2iUMQzCANwuDUOA3EGHw0CCNYchuDYXDQMQ2CAOBIDMYYNg2G5Hh6Nx2DaRI3jmHJFGgMorkyOI4keTguDkOYvj0MxsDMLgzDiLZhmMNBBhiGJPjqRQ5iCJx6CIPg8C+A4FnSd4GgiCo+CAORhhmGZYkYIA2nOdZ6D6AQ",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Volume1,
    #[cfg(feature = "volume_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0C4NwwDIYYQDANYVDUIAwhqGgtDELgyDAMwtC4NA5DgTA2iUMQzCANwuDUOA3EGHw0CCNYchuDYXDQMQ2CAOBIDMYYNg2G5Hh6Nx2DaRI3jmHJFGgMorkyOI4keTguDkOYvj0MxsDMLgzDiLZhmMNBBhiGJPjqRQ5iCJx6CIPg8C+A4FnSd4GgiCo+CAORhhmGZYkYIA2nOdZ6nmBJ7gmC5vDMNo2DEOJipIYQ5n+bIdDGUw3DIOKInajA+gEA",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Volume2,
    #[cfg(feature = "volume_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA5GENQghEMAghQMQgC4OYRDILgyDkMwiD4PAvgOBYiiWBoIgoMQ5C4Mw2DSEguDaLxhDmDoVjmFwxC4OA4jeLQ5DaIYjiiJ4EimCRtDIIJMDKFJPkSJJIkeBYHkoNwgDcLQuDUOA3l2XxBjyMZkjmFoyDSDAgDgSAzGGO5nhULY7HYNpwCCF4UnueZ5GiHJqneZpmnyF4ZDmYJqDMbAzi4OAzCCjQzDgNBBC4NwwDWl6ZnKFo7i2Hg4FYMQxlKRoCkiV4Ki0OAyDgIA0C4MQ3DKlg2DgNozrico7hesQ2DUNx2rqQ4ilOJoBA",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    VolumeOff,
    #[cfg(feature = "volume_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0C4NwwDIYYQDANYVDUIAwhqGgtDELgyDAMwtC4NA5DgTA2iUMQzCANwuDUOA3EGHw0CCNYchuDYXDQMQ2CAOBIDMYYNg2G5Hh6Nx2DaRI3jmHJFGgMorkyOI4keTguDkOYvj0MxsDMLgzDiLZhmMNBBhiGJPjqRQ5iCJx6CIPg8C+A4FnQbBpG4ZQgHgMYJDIMgiCAeQygkMQ1oQeKHCKPqEHmgAiDmc51nqfJ5nufZ/ogNqLo2gqQpKlKFo2iaVC+lxlD6AQA",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    VolumeX,
    #[cfg(feature = "volume")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0C4NwwDIYYQDANYVDUIAwhqGgtDELgyDAMwtC4NA5DgTA2iUMQzCANwuDUOA3EGHw0CCNYchuDYXDQMQ2CAOBIDMYYNg2G5Hh6Nx2DaRI3jmHJFGgMorkyOI4keTguDkOYvj0MxsDMLgzDiLZhmMNBBhiGJPjqRQ5iCJx6CIPg8C+A4FD6AQ",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis"
    ))]
    Volume,
    #[cfg(feature = "vote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQyCCD4PDQLQ0CIPg8C+A4FheGoGggIhNDUIA3GMMAtDELooDkLYRiwaAxDAYYRCAMI0g2EIQHaDhIDUVg3FqFoYh2HIEh6CRNDKDwxDkSAykGGZFD6AQ",
        categories = "social",
        tags = "vote,poll,ballot,political,social,check,tick",
        contributors = "ptrgast,karsa-mistmere"
    ))]
    Vote,
    #[cfg(feature = "wallet_cards")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHmCYLCAdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJE0MwgDkYQyCCOQwCCPAxjoLQyGgMQ0jiOo9kiP45hGKAvi6Goti8aIxjONQxDEaAzGMLg4kkLg2C6NQyC4MQuDkbJlmUORjmUNggm0IA0mSb5fCANQuDePZomQLZmlsNZ9DWdAzn0OY6nyZhIDIMQik2TxoD6AQ",
        categories = "account,finance",
        tags = "money,finance,pocket,credit,purchase,payment,shopping,retail,consumer,cc",
        contributors = "danielbayley"
    ))]
    WalletCards,
    #[cfg(feature = "wallet_minimal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDQaAuDAMQiD4PAvgOBYWhmBoIgqDQ3GgMQyGEMggiYMAgikMYnicdgxDCJYtimKwtiYMhIDWMooiqDo2jYVo6jePYrieNoiDSFYXhwPoBA",
        categories = "account,finance",
        tags = "finance,pocket",
        contributors = "danielbayley"
    ))]
    WalletMinimal,
    #[cfg(feature = "wallet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA3FYNBhDEIITDAIIWDALQxhoSA1GEMggiCGIXiQNBoDGHoThWJIqhQdomC0M4fiGJIjhaJoyi2NoUhodgtDKEoUjWF4ahoIg+DwL4DgWSJLgaCIKDMIA1HYMYRiCIpDliJ4pkKO4bDGPg0keSZOD6AQ",
        categories = "account,finance",
        tags = "money,finance,pocket",
        contributors = "mittalyashu,ahtohbi4,ericfennis"
    ))]
    Wallet,
    #[cfg(feature = "wallpaper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcdg0CIPg8C+A4FhSF4GgiCg4CAMgxGgOIThWGoZgSG4JG0OYODcIA2C4MQtjAMRhg2DQwCCOQxh8Lg4DELgwDETAyg0MQ1iSFoohQYxpHIYxsGUIBjHiCYjlMeYJDkIggHKCQykmTZPlGFByGUYx0CAaBlGkZxoHSCQxhIIB3GkZIFl8MJclkIgzlwcpVCKYAgoGYIUC+ZpoD6AQ",
        categories = "account,devices",
        tags = "background,texture,image,art,design,visual,decor,pattern,screen,cover,lock screen",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Wallpaper,
    #[cfg(feature = "wand_sparkles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELg2DQIAzg4NAtg0Mg4hULoXGGFgxCCHQgDCIYhhkNwyiETAyC4Mw2h8OIThyGoeiCIo1iODYmGyFg4h+Gg4jGJ4WiONo4icMIpg2DwgDWKw2kCPZGkOJJFCIPg8C+A4FlaWYGgiCgxhAN4RhGVZXlyW4El2CRNDUIA2HYNJlliaZogWB5rDEOYfDScJymeAppncIhNDGIgyHYMp+nSgJ2l4TZiDgSAzoqWqMmqg4Mh8NhoC2cZWnOlZcoKhIeDMSA5pQaA+gE",
        categories = "design,gaming,cursors,photography",
        tags = "magic,wizard,magician",
        contributors = "karsa-mistmere"
    ))]
    WandSparkles,
    #[cfg(feature = "wand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA0FYMgiD4PAvgOBYThaBoIgqDAgDENh2C2EYThWBBohiJoHgkTQ4CAORoiOFIZiiBYqgoMgwi6MISjKJo0hqKwxDcLotDEMZEh4OYeDOPIlheAophuC4Ni8LgwDGTYzlCNZSkKSA2C4MpJCANZZj6W5ACIbQzCAMgxi4LQ5maT4ZjaCwymEIJgmKRpknOJ4BA",
        categories = "design,gaming,cursors,photography",
        tags = "magic,selection",
        contributors = "mittalyashu,ericfennis"
    ))]
    Wand,
    #[cfg(feature = "warehouse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEVgxDAYQxCCFQwCCGAwC0MYcEgN4UhaGYjhuFQxHYMQxCIPg8C+A4FiyL4GgiCgyDKFg5GGN43hqFgtjsSA0jqDokj4Mo/FYOJDjyI4mC4MQzkcMQuDgMAzGwNwuDkNQtDOWg3kKO5Fk4OAzDeGZYloNINl4OZgEGYo9hWNggDgeori2MoxgSM4JE0NoWDMaAxDKeIunye4FgefqADEN6DoWLKHjCAQA",
        categories = "buildings,navigation",
        tags = "storage,storehouse,depot,depository,repository,stockroom,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "washing_machine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDMIg+DwL4DgWEoVgaCIKDEN4NGgLgwDGEYThiEhyGUYx0CAaBlGkZxoHSCQyDAIggHcaRkgWCQxDiNR5jKNRyHiQAgkMIoQhIL4nimEhjGkchjGwZQgGORgxDKNRjj8IgxhAIBygkNYjC+TpQlKF4EhmCRNlcII8GEMguDUIJxnMMAgneeQtnOdZ0nKeKADGeJ7mOJYBA",
        categories = "home,devices,travel",
        tags = "tumble dryer,amenities,electronics,cycle,clothes,rinse,spin,drum",
        contributors = "danielbayley"
    ))]
    WashingMachine,
    #[cfg(feature = "watch")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDAdgyC4MhsDELg2g4Ig+DwL4DgWG4egaCAiG0MQ2C4MQzCAN4XDYLQuDgMQtDQLgwDUYYNg0MAgjsMAtDILYWDYMRoj+Fw4jgII6jyPI/g6FwxGyLw3DgII0jaGociGIIEiKCRtiwOJViYLgzieVQ0kmS49kqT5DGiEg3DKapMmyQJClGMIylcNZZh2XYbGMaRyGMbBlCAcoJDYIggGMeIJgyjBjHmkAyn6gqEoYPoB",
        categories = "time",
        tags = "clock,time",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Watch,
    #[cfg(feature = "waves_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDATIMCAMgiD4PAvgOBYWhmBoIgoMQ2CANoRg2DxMDiIoVheHIbgSHYJE2JQ1EMMguiEMQ1C4NQgDMLoliENI6g4NhDDeQwgkYMQ0CAOZCkoQ4MC4MYOksMQxC4OZHkqTpEDGSY3kmVZNjuTwyDALookqE5XlWQY7DKMoqhiLotgWB4wg0MgxjSNprkKPZ5g2boTDKRaEkiEwwkyQpmlCNZTmaDpXlmcJUoyhZeoemaRDGY6JjSZ4opGegukuZqXmucosgE",
        categories = "weather,sustainability",
        tags = "water,sea,level,sound,hertz,wavelength,vibrate,low,tide,ocean,rising,down,falling",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    WavesArrowDown,
    #[cfg(feature = "waves_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYOAiD4PAvgOBYThaBoIgqDQxDUYwuDYLg1CAMQuh2DojiWKYkDCLAtg0NYwikNggi4MguDSDggiSDY4i2L4xjOJgzjaJQuDmKo/iWEoUhmGIEhqCRNj4MYgiKJImiiSwxiyRo/jOMo+iGX45juPZejeI5hkMLpFi6JpJj2Kgxk2FZRlCBYHgkbQ4CCNQ0C2OqDnaT4BA",
        categories = "weather,sustainability",
        tags = "water,sea,level,sound,hertz,wavelength,vibrate,high,tide,ocean,rising",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    WavesArrowUp,
    #[cfg(feature = "waves_ladder")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA1GEMgghEMAghQMAthEMh2DEMQiD4PAvgOBYfiKBoIgqEQxDgYwuDYLg1CAMQuimEovjGNYwhQMovhiDo9jsNoVjUNISg6RY7jmOI9DWPYyDOQoyDmNpIjGHogiWJIEiaCRNDeMQzGgMQwlaIZalmBYHlyXg5mGY4fmWI4ClqaYKg2D4ZkKFo/huHZvliAQ",
        categories = "sports,home",
        tags = "swimming,water,pool,lifeguard,ocean,🌊,🏊‍♂️,🏊‍♀️,🏊,🥽",
        contributors = "karsa-mistmere"
    ))]
    WavesLadder,
    #[cfg(feature = "waves")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhjC4NguDUIAxC6DAxCAMoShQQw3CCHoehMOYbDUY4aDYIAwhkLg0hkIITgyGoTiqMgtgwNY2hQLgzimOojjCGwxCIPg8C+A4FkSR4GgiCoXDKD4RhOFYXiuUpVj2NY3jmJ5YiyLowleNISjmOIXjuPYVj+V5CkSRoEGiSZvgeCYLhQOJQkGFoUmuYZVmSW4Ql2LY3i6MpdmWL45hWPIqmmG6GmyRZKD6AQA",
        categories = "weather,navigation,multimedia,sustainability",
        tags = "water,sea,sound,hertz,wavelength,vibrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Waves,
    #[cfg(feature = "waypoints")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDALg1DgNggDULg0DENAthMMQ3DKEguhoMgiD4PAvgOBYiiWBoIgoMQ4g6EAgDEM4UhaGIehuHYfiGI4oieBIpgkTYRDEMhokOOokj6IhjGkchjGwZQgGMeIJkYIBygmIJRHmWAwkeS5Nk+SpMk6UJSlSWRjlsIg0CKVpYl6Y5hDyX5klGUwiDKXZamebZXnicJgGWYqBnaCZsm6f57CKRoiC+dJhgEA",
        categories = "security,account,navigation,development,social",
        tags = "indirection,vpn,virtual private network,proxy,connections,bounce,reroute,path,journey,planner,stops,stations,shared,spread,viral",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Waypoints,
    #[cfg(feature = "webcam")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwOIQGMeYMDEMAiD4PAvgOBYHhyH4GgiCoYg+CYXg2GoRgwM4bh2I4hDwcBhHQaAgGSDBNDcIAyDIaIZjAL41jeHJFjiOgiE2Do+DIdgtDSQ5ID6AQ",
        categories = "connectivity,devices,communication",
        tags = "camera,security",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Webcam,
    #[cfg(feature = "webhook_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIIMGgLQ1GMLQxC4MA5C2FgyhQLg5DSHYbDILg1g6HRBDMIIoDCDosiINYNDENQiD4PAvgOBY0jeBoIgoOYpC4NBhDQIJDiuKwxCANojiINg2jONY6jmBI7gkbQ2g6DQzC4MYRC4OBhi4IJhkYIIrhaLwtiIMIyjSNpTlKBYHgkTQyDALoonWd5CkSZZ9lyIpaDeT5ujiApTnKCgxDgLpXDEM57kWfo/DOL4/DQMQ0oOUaGnGPBtDEMpJCCjIOpqb6clQIhtqGrIrnWpo4gEA",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere,jguddas"
    ))]
    WebhookOff,
    #[cfg(feature = "webhook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDYLg5DgaAtDWEQ5GMLQxC4MQgDCGoRhYOQ0C0MguDSDYbDkQQ0CCLQwh6DggDKDg3GMLgwDELQuDcLgyiANAuDUN4lCIPg8C+A4FkeSoGggIhtDaNQgDOHAzhWPA4jcNZXhGPY6iaDI7DULZVDEYYti+ModhAOA5C2QQwDaRpIk2TIEk6CRtDGNJSmYMwghYNwzEMMYWDaUp8jyDoQDmDqAgyj5oi6MYwh2MA4nSSZ4D6AQ",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere"
    ))]
    Webhook,
    #[cfg(feature = "weight_tilde")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4NQgDgYQyCCEgwCCFQwC0MQuDkMA2CCGg0DYTAyC4MYfDiDRBhKFIWi0NITDEaAxDaEYTi2F4fhsMg1C2JA1DQTAxDkLovkMNYqjaOIVDEN5EDiDx6CIPg8C+A4FlOVoGgiCpNDmXofDWEYNhOY44iaL4Vj6ZIOkoIJolKVJZlMYxpHIYxsGUIBjHiCQxDIIp6HmCQ1oAcoJDOcAvnSdp4D6AQ",
        categories = "math",
        tags = "measure,scale,estimate,load,balance,size,measurement,quantity,mass",
        contributors = "nathan-de-pachtere"
    ))]
    WeightTilde,
    #[cfg(feature = "weight")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINQigkeIMDEMoPHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0NguDUIA4GEMggi8MAgjIMAtDELg5DCK43DQNhMDILgxCAMQ4ioQYvjGM5KDSMAxGgMQ2i6MJKjSQ44DINQtkANQ0EwMQ5C6TJgDWR5TlWMgxDeYQ4iwWoZhuIYjD6AQ",
        categories = "math",
        tags = "mass,heavy,lead,metal,measure,geometry,scales,balance",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Weight,
    #[cfg(feature = "wheat_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMoMDEMAthAIg+DwL4DgWFYYgaCIKDENggDiEguDENwgDGJA3hSFobhqBIcgkTQzC4NImDEMguDUMwgDWJwxGyKI6ieOQzGGM49kcIAwkqJ5MDQLg5DQTI9DEORsiOOpYkWSZJkuSwxkoLZPlEWorheL4ugWB4JG0OIhC2RJEkaOQgl2TJeCCY5SDmJw1kCRJaGOOQ1nANQ1C4OIiigMqHDmipQiqFZnhmAovmsIhNhCUJgocMg2oIMaEC6nguDMNKFkENpwDcMxMDGOwzn+QpBludJ2l8IKjm6Tw2DKZotpWaodE0MpLDIaAyHYMhhDSeZ3ieYp5GgLQyFYNrMs6eJgqcNJlpKwIbpemYojWJw3jSNo7lWV60kOOpzkidLamKULNDCU4nlatJavCdbyky271kq3osmiwYwh6IIfoWR61vGPbznqShMnyoayDO/IzDSfMany2pDDkN5wo6v8GuGwwxDgLg3s2rwuDAOaCp6cKhjkMZwqbK8YjkNhMDKYAxj+WsZra/5evSvZwDIOMlpQbBpG4ZQgHgMoJg4IggHkMdW1geNbCKvtZ1XYK+pLT9RD6AQ",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WheatOff,
    #[cfg(feature = "wheat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMoMDENggDgIg+DwL4DgWFYYgaCIKDMLg0DcIAxDILg1DMIA1iMMRsDGJooi6Jxhh+Ko0CAMI3iOOQ0C4OQ0EyKgxDkbAtjEM5Fi+M4mCCNo4jgMY3C2PI+FqFIWhuGoEhyCRNDeIIiDiLwgDkIA3i2YpGkqNZLk6Oo4lOP5kDENZEkaSIyjaTY5lAMJSj0NJVhWF5almBYHlwMYuiEII8ieI4oDOZ6OmmeZsnuO5/EwMYwiyd5HpSS56k+UZwoGV6EgKWqHgoMo4DIaAyHYMhhDSjI5k+UqMGgLQyFYNq0rabZQDSUqmoOGapoaHRNomX4jl6i6biOQ6eiOSaViqwp+j6N5AtOkqfteoaWqOcI3saWLJluCpzs6m7uiKc51mi4prtme7brUMBMnKdJ2qC9q3jq5gwuiqIbquzA5s7C6LDKUKJvOk71ky5Ijvm3bSxG/8UqLA5/ueVrHGgPoBA",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten",
        contributors = "karsa-mistmere"
    ))]
    Wheat,
    #[cfg(feature = "whole_word")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDeDhjHmCwxDIIg+DwL4DgWB4bHAYR0GgIBkgsTQxDAIA5HYNoahyIokhuHoGgiFYXhmD4RCIMYTgmC4NhuHYEjaIYjiWJwiikNAgDcdg4jAL4yGiR4kiaKAyDIII+HYMRjisLg1C2YpcC0MZcEgMxjmQNQgDCZ5tnEMR2meUpUD6AQ",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLgzDANQgDEOQuDUMwuDkMgzC2Dg4DIIg+DwL4DgWIYkgaCIKDENQuDIMg4hINguDgNQyhyGYbh0M4giKJ4mgSKIJguMo0DKEosi4OIcDMOI5jiPIjkCP4FgeQgxkSNQgDIMAuDcN42jqGAyDSUI+gKQJViqFIMjCK4ti+Oo3hqZZSmeVIpguFIWloMQuDYOQ2kuHpymSIZRiWdpBCITZGDeMwyGGK5HCAMKUhKWqVDCdKIieaRtluXZfjGM41mINKCjuhpmp2eKgl6RoTC6bKmjqmxolOihNhEMZ9jQOKRpUMaZpYMYSn0NQtn0N40rauJpE0OIVkcLg0DIORhhGEbDsUMrUDGG59DMMbNDwYxpHIYxsGUIBjHiCQxDgIggHKCY7uwebvvGhrmui6g+gE",
        categories = "connectivity,devices,files",
        tags = "connection,signal,wireless,directory,settings,control,preferences,cog,edit,gear",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere,luisdlopera"
    ))]
    WifiCog,
    #[cfg(feature = "wifi_high")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAuDAMQiD4PAvgOBYWhmBoIgoNQggwLg4DUORhDEMIhimK4hiENAgDCFYXhyG4Eh2CRNDgLogDENguDQMomiCIIsDEIA3jCMoYjYPoBA",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiHigh,
    #[cfg(feature = "wifi_low")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAuDAMQiD4PAvgOBYWhmBoIgoOAuDUIAxDYLg0DIORhiKIgwCCLQxCAN4uhWF4cD6AQ",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiLow,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAuDAMQiD4PAvgOBYWhmBoIgoOAuDUIAxDYLg0DIORhiKIgwCCLQxCAN4uhWF4chuBIdgkTYigwLg4DWKQxi+LZEiMIA1C4MQ3C0MguDYOY0hiOI3gWB46DEOYjk2P5BkOLpfDCTISDCSwxiEMgzlGNoCjiVoKg2IA4DIYQxjyLJfjANJJDeS5NDYNJphaUoamyVYeE0Mpwj6c51iOd5EC0MZmDIOA4C0MwuDef5qlOhY5CIbYNqKLYPpyGoB",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[cfg(feature = "wifi_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOAuDgMhhDENQghMIAwheFQgDKGAwCIPg8C+A4FiCI4GgiCgyDELgzDcOIVDYLg2DINoShqHYXC0MwuDAMA0jqPI+GwLQ0jwMQgkUMAxhGDIMjgMAtC4NQwjEOA1DSQ4Pi2G4PDcYZSlKGY4jIMpkGyZQ4DeUQ4i0YZNmKGYPleUZTDYeofiGJolgSJ4JE2FJLnIOYShgMZPhqhpSC2ZQyo2eIinye4FgefoOoCMQ0DKg4UhSOJHDMLYrDSVKPnqAQ",
        categories = "connectivity,devices",
        tags = "edit,wifi,pen,change,network",
        contributors = "karsa-mistmere,jguddas,danielbayley,luisdlopera"
    ))]
    WifiPen,
    #[cfg(feature = "wifi_sync")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg5DYNQgDEMAuhMNR2DQTAxDMLoRDEModGGEYRDAIIlDEIA4hKHQiD4PAvgOBYujGBoIgqDIOhCEg0hUMA1GgNIti+NIzgSNYJgsN45h4OI9j+GQyhQNAzhIOQuDcMYiCCJImhILQ4C2DQ1kKMJGkWBYHkgMopC4OAyGEMYelyJwglGJpkkSApGmmCgyg2D4RDKIIWHYLZBi6ZYynqaI2E2HogDgNpwieJaVhIIAzC2IAwDMMp4mai5HgqTYeDYLg0DIORoC4MAxp+MoBA",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless,synchronize,reconnect,reset,restart",
        contributors = "colebemis,ericfennis,jguddas,danielbayley,luisdlopera"
    ))]
    WifiSync,
    #[cfg(feature = "wifi_zero")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAuDAMQiD4PAvgOBQ+gE",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiZero,
    #[cfg(feature = "wifi")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaAuDAMQiD4PAvgOBYWhmBoIgqDQ4C4OAyGEMQ1CCJggDCKoog6KwwhWF4chuBIdgkTYngyIg1DmJYrDGL4sDGKA0iqMYYjWNIFgeN4hjkNguDQMo9ieJ5BkMN5GhaSIagEA",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    Wifi,
    #[cfg(feature = "wind_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYOAiD4PAvgOBYThaBoIgoMQyC4OIODELg2EEMoOCCDQxigIAxDSLA4EgMoShSGYYgSGoJgsNwuDWLAwGGHo9kGK4qiqJg0jGM4VjeNoFgeCRtDYIJSi6VQtDSSo1gE",
        categories = "weather,sustainability",
        tags = "weather,air,pressure,blow",
        contributors = "colebemis,csandman,ericfennis,jamiemlaw"
    ))]
    WindArrowDown,
    #[cfg(feature = "wind")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg4CAMQ5C4NhBDIIIWDCEAghkMQ0hANhIDIIg+DwL4DgWJIngaCIKDENwuDUIA4GGDYxjWG4aDGFwgDSIYjiWKopgSK4JE2EoPDQLg0hWO4cjmOg4j6JImkMPoBA",
        categories = "weather,sustainability",
        tags = "weather,air,blow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Wind,
    #[cfg(feature = "wine_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgyGgOAiD4PAvgOBYThaBoIgoNwgDEMBoDMbYdiALQxC4Mw0DOEoUhmGIEhqCRNDEMoeDUdg3iyFYwi+BYHjIN4oDCHZBDOQxBjSKAzh4MpKCAMJPlGHYfGENQglaUJZCCRQ5DEIA0iiXYLC4NgzgyTQ5DgMRDDgLg3laTQ2DaDJtDgN41k2KQgDmDRoDYY4nlYNINCANqEgyUAuDSQwtC4MA1C4OAwDmjQxDSVonDGaY6i4PBsGkbhlCAeAxgkMgiqMMqmqcIB5qoIoOqgealrCOqfqEPoBA",
        categories = "food-beverage",
        tags = "alcohol,beverage,drink,glass,alcohol free,abstinence,abstaining,teetotalism,allergy,intolerance",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WineOff,
    #[cfg(feature = "wine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgyGgOAiD4PAvgOBYThaBoIgoNwgDEMBoh+EoUhmGIEhqCRNDEMoeDUdg3iOFYniaBYHimK4tGENQgjsMAgj6Pg1C0NRjDALQyC0LpCDSRwtDgSA5GMLQxkoIJMiwNpHCAOI6jyP5fkGPBajGJYBA",
        categories = "food-beverage",
        tags = "alcohol,beverage,bar,drink,glass,sommelier,vineyard,winery",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wine,
    #[cfg(feature = "workflow")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMGgZRpGcaB0gkOIMHmCYLCAdxpGQdBohkIg+DwL4DgWJxwGGIwgGSCRNDcIAxDEdg0GEMggjsMAgj6Po7DIaA0iaKItiOJ4qgaDoJhEIIThWF4lgeCQxh6HAileDIgiKJAihqJ4pgQdA+gE",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "worm")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAxDILQxC4NQgDMIg+DwL4DgWGIbgaCAiE2DAuDYM4ODgLg4DEIAyDKLAwheGYeh2BIfgkTQ2C4NA3CCKAyDMYYSDYOIOiSRAwCCSIrDKOg0kUOQzGwLYkk6TAwDgYY5DcNgglqXJImCDgwC4MZcDeJA3GyOoQC4Mg3kGDpJnEMAtkwNwzC0NJtDGUprm2b4SluRaCmEMZ1iSeISDkOZqDYNKHlcQY5DaXKTl+cpIDMLg5k6mqMlOZw0nCK5KnKTA1hSegzDQeoxhqNQ+gEA",
        categories = "animals,security",
        tags = "invertebrate,grub,larva,snake,crawl,wiggle,slither,pest control,computer virus,malware",
        contributors = "karsa-mistmere"
    ))]
    Worm,
    #[cfg(feature = "wrench")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg3CANguDMYQxCCFQwCCGIahYLg0GwMQuDaHA2hSFoZieGIgDSGRsDMLgxDANgti6MA1GOEgyC2OAyC4OA2DOOgyjwOQ4i4MgxDgYYiiKGwxC0OAuDINQ5CANwuDANQ3GwLZWDmFZdDGJYXieTpDDkOYzGyYJcC6XpKhCKImlaWA3k+UZTjcNAzlAMY8DWDQ2DaPJHDmbQ4DQegiD4PAvgOBQ+gEA",
        categories = "account,development,tools",
        tags = "account,settings,spanner,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Wrench,
    #[cfg(feature = "x_line_top")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA0EgNgiD4PAvgOBYThaBoIgqDAgg0NggDIMIShSGYYgSGoJG2IINDEMggi6JIVigPoBA",
        categories = "notifications,math",
        tags = "line,top,arrow,navigation,up,pointer,direction,vector,symbol,cancel,close,delete,remove,times,clear,math,multiply,multiplication,mean,median,average,x̄",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    XLineTop,
    #[cfg(feature = "x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2g4IIMCIPg8C+A4FhSF4GggIhtg+DwxDKEQyhOFYaD6AQA",
        categories = "notifications,math",
        tags = "cancel,close,cross,delete,ex,remove,times,clear,math,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    X,
    #[cfg(feature = "zap_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg1DEMwgDQLg4DUNgghALgxDIIAyhoNxhg6DggDCJIYCCFA2C4NA2GwLQxC4Mw3DeEoxDENwiD4PAvgOBY6j2BoIgoMQ1C4NoWhgMBIDIMBhDGJ4liWTwuDcOIYkYM4ujAN4clsNwzjmO5Aj+BJBgmC4qDKX4Ymma4MhSVgyjAOAziGRQ1iaUgtime4sGyMA5DILYqDAMhBk+T5RiYMaIDQSA0k6UKLnuVYvlgbITDUMA3C2Ew2DSYI6jyZZkgWB4JG2HKqiWTJhqOPoBA",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[cfg(feature = "zap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0GEMYNCAMITg0LQuDcOAtDELg2DMbA5C4OYbDALgyGELg1imFYUhELg4DYLg0DYbIbiIMggjEMAyEGEYRhSQINDODQwGgN4QhKQYuhmDYdh8LYhDmRImiiKg1iyFovDaF4zGyHA5DILY6jyPpYi2Pg0HoIg+DwL4DgUPoBA",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Zap,
    #[cfg(feature = "zodiac_aquarius")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMQwCAMguDQNQ2C0MwuDYOA0GELg3h0IIPg8MYNC6DoVC4MAxDMbIRDMOQghcNIqhyHg3iCN4iigOYnDCKYshIMAyhaEgzDKNIfiGDYkDAOQ4iCPw0kGMJEkaHZIjiS5Nk+LQ4DmQ4yDOR42kmIwxiUMA2iiKhMDKDIOCIPg8C+A4FnGdIGgiCpuDiKJShGE4VheGYblaY43mWZ48iqP4ulOYJiliOZMmOUJBl+RaQmSWZODClYMjGmKFpGm5bC4M5dl+M6ipqZommoM5snufQynCcp3D6AQA",
        categories = "social,emoji",
        tags = "water bearer,waves,innovation,air,future,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacAquarius,
    #[cfg(feature = "zodiac_aries")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3C4NRhDSEAghMNQgDAIAxhoIIXhYIg+DwL4DgWIYkgaCIKDeGgyhKFIWhiHIbDULYWFYMgxiCIonD6AQ",
        categories = "social,emoji",
        tags = "ram,horns,fire,energy,initiative,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacAries,
    #[cfg(feature = "zodiac_cancer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDQLg1EEOQgDaEQgDCGIOCANYXDEOQiD4PAvgOBYiiWBoIgoMwgDmEYThWF4ZhmDQxDiFw1iGI4oiIYxpHIYxsGUIBjHmCYPhEIggHKCQzkmRB4kcN5JiIL4+kCQo9j+QZDGOUQihaOZLk2T5FgmLo5lWV5cD6AQ",
        categories = "social,emoji",
        tags = "crab,shell,protection,water,intuition,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacCancer,
    #[cfg(feature = "zodiac_capricorn")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDEYQzCCEgwCCFYVDMLQzFYNguDUYYNg2F4WC0N4WCIPg8C+A4FimLIGgiComDEOYchGE4WjkMIahoaAwiiKovikYxpHIYxsGUIBygkMwiCAYx5gkMQ3k4Yx4lKVIpC+RJGkgPoB",
        categories = "social,emoji",
        tags = "goat,mountain,ambition,earth,discipline,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacCapricorn,
    #[cfg(feature = "zodiac_gemini")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA0C4NQyDUdgxg8OQ0DgIg+DwL4DgWG4egaCIKDIMAgDMQQxDcIIqCCJomDGDonhqHIhiCBIigkTQ0CAMgxGGLYti+LIsg0MI0h2OI3gWB46DiDoQhKFIWhiSI2gE",
        categories = "social,emoji",
        tags = "twins,duality,communication,air,adaptability,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacGemini,
    #[cfg(feature = "zodiac_leo")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDYYwwC0NAtDOEwuDWFQtDgYQ1CCHoNg0MYOiGEQgDMLg0DYNoVCANguDEOYsDOJBhjSNIhCCDQ2joIg+DwL4DgWPxjGkchjGwZQgHKCQzCIIBjHiCQ3k8Yx5gmD4+kCRZHkkPoB",
        categories = "social,emoji",
        tags = "lion,crown,leadership,fire,confidence,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacLeo,
    #[cfg(feature = "zodiac_libra")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQ2GgNguDgNQ3GMLoODILQuDAMQyhcOYaDMMgzhsMw4iEOBhDYIIrDCDYvDQLgyh0IAwGOGgxDWJAwhSOAyDULgzDYOIlkSJhIjMIg+DwL4DgWS5OgaCIKgwMgwGgMQ4kqTJRD6AQ",
        categories = "social,emoji",
        tags = "scales,balance,justice,air,harmony,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacLibra,
    #[cfg(feature = "zodiac_ophiuchus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQwEENguDANgghGEwgDCGINg0MoNhmEIShSFoUhmJQgDIMYeCIPg8C+A4FiyL4GgiCoUDMdgxDIYYihqJo5hgVgziuLYyD6AQ",
        categories = "social,emoji",
        tags = "serpent,snake holder,healing,knowledge,astronomy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacOphiuchus,
    #[cfg(feature = "zodiac_pisces")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAyDEYQxDUIISCAMIWhSFgtDEOAiD4PAvgOBYfiKBoIgoMoXDEMhIDSHogiWJIEiaCRNhMM4RhOFYXiqGIci+IYzD6AQ",
        categories = "social,emoji",
        tags = "fish,duality,water,dreams,empathy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacPisces,
    #[cfg(feature = "zodiac_sagittarius")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNh2DYIg+DwL4DgWFYYgaCIKDIMYOiGH4UhaG4agSHIJG0OQgiwNgghOFYXigPoBA",
        categories = "social,emoji",
        tags = "archer,arrow,exploration,fire,philosophy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacSagittarius,
    #[cfg(feature = "zodiac_scorpio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDkVg1C4NRhDGDggg2DYWDWGBWDENxhDIIIihmGIjiMaA1GwLQziwIg+DwL4DgWMIzgaCAiG0Moig+LAgDOL4xjaNYEjeCRNhyD4RhOFYXiWG4YkGMpFkSBYHkeHISDUQQyhOI5eiWDZdhyQIwlONIBA",
        categories = "social,emoji",
        tags = "scorpion,stinger,intensity,water,transformation,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacScorpio,
    #[cfg(feature = "zodiac_taurus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ1hAcoMDYIg+DwL4DgWB4bHAYR0GgIBkgwTQxDgIAzEENggi8MAgjIMYwiyGociKJA+gEA",
        categories = "social,emoji",
        tags = "bull,strength,stability,earth,endurance,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacTaurus,
    #[cfg(feature = "zodiac_virgo")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA1C4NRhg2DQwCCFYNDWFhWDENhhhmGYViGDoOCIPg8C+A4FiaKYGgiCocCCDIQhKMYWjaGIahyHojiIMQth+JYniyK4Ei2CRNDaMQ5FaHQzCCToiDALQzlMaAwkGKJFkSBYHkeSYPhGE42hePIbDmWJDgEA",
        categories = "social,emoji",
        tags = "virgin,maiden,harvest,precision,earth,analysis,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacVirgo,
    #[cfg(feature = "zoom_in")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCIIBygwOIQGMeYMg4Ig+DwL4DgWB4bGwaRuggeAxgwMoPCAeAyhgNguDYNYQHmJwiimM4tg2L4xhqHIiiSIYjggeY5DENIQiyGIqiaSozjWFIbC+PxlkGJAgjSTYrk+OJZkmDZHlGUw+gE",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQxCIIBygwOIQGMeIMg4Ig+DwL4DgWB4bGwaRuggeQxgwMoPCAeInCKKYQHgMoYDYLg2DWEB5jKDY0jaGociKJIhiOCIxhgNI4i2GYri2FAgjmGIPhsL5AGUPoBA",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
