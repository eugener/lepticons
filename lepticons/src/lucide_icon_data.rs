use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideGlyph {
    #[cfg(feature = "accessibility")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDSDhjHiCwxDYIg+DwL4DgWB4bHAYR0GgIBkgsbQxDgIAxDmLAtDcLQ2iyGociKJIhiOJYnCIbQ1CCKwzC0MwgDULo/kIMguDOMwzkeNQvjcaI5iSJoLE0NAuDINIslkNRhj+PwwCCY5jDYLg4iuGYblGOpUjuVwxk4N4zDEN5HmCRZknsMIymgOIylCUg+gEA",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[cfg(feature = "activity_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEaBlGkZxoHSDYPhEeIUhEcogCIMgiD4PAvgOBYoHAYYLCAZINE0MQ3CAMQyGgLQyGyOwgDWOwtDEMI+DUSA3ieKYugsPoBA",
        categories = "medical,account,social,science,multimedia,shapes",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "danielbayley"
    ))]
    ActivitySquare,
    #[cfg(feature = "activity")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIaAtDQbAtDMIA5EwOQgDOFIWDkSAyCIPg8C+A4FD6AQA",
        categories = "medical,account,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "colebemis"
    ))]
    Activity,
    #[cfg(feature = "air_vent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQyEgNBhDIIITDAIIWDELQyhoVg1hKFIXiEMYUhoaAxDaH4ViKIAyHaHoTiqGIahQaIaCIPg8C+A4FjiO4GgiCoMDiJgyjeOY+j2BI/gkTQxDgLgzg0NwuDeEguDWFJXiGMgzC6Jwgl0OJRDKV5jmWW4NC0MZeDSHIOkaOpKkmBYHkwNgugwMQ1ngQYwiuGIYDcdgtDWcJIgE",
        categories = "home",
        tags = "air conditioner,ac,central air,cooling,climate-control",
        contributors = "karsa-mistmere"
    ))]
    AirVent,
    #[cfg(feature = "airplay")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ3EgNBhDIIITDAIIWDELQyhoVg1hKFIXiEMYUhoaAxDaH4ViKIAyHYMQwimIYYhqFBoC0MQiD4PAvgOBY6HAbxsHkZxvG4IJAGkbh0HOCQxhMMYMg6FIjDeU4Nk8NY5juQJCkQbg+gEA",
        categories = "multimedia,connectivity,devices,brands",
        tags = "stream,cast,mirroring,screen,monitor",
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
    #[cfg(feature = "alarm_clock_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4OA3CCDIOGEOAghQMAgDEIIXDEMQuDINoYh2HwiD4PAvgOBYliiBoIgoMQ5C4OYYDSHg1hOFYajkMAtjAMQ1jwLo+iSJoriqBIsgkbQyDKEAtDOTpDieR5GgWB4JguHogDEOAuDaDw0CAMgxlGRYCkeVgikqYZhhcMgwmSU5mlWLRNmCYJMDacIpgE",
        categories = "devices,notifications,time",
        tags = "morning,turn-off",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    AlarmClockOff,
    #[cfg(feature = "alarm_clock")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwOIQGMeYMDEMwiD4PAvgOBYHhwcBhHQaAgGSDBNg4IA5HYNBsDIIIPhwL4jiWIokiaKAiE0NQgDOMggDaG4djYaI4iWJ4MG0MoxDYLQzlCRI1jmSI6ikNguDMOAgDEOAuDcIA0jIMZTkaVpKjwMQ3C4Npjl6bZhDIMJkmaVYB",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_minus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHCITQ5k0MxojCG5CmeAQ",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmMinus,
    #[cfg(feature = "alarm_plus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHCITYOk0MB2jCG5CmcPJDmkTQ5k0MxonCMpngE",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmPlus,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgGgZRpGcaB0goMQ4g0dxpGQdBohmGwggkIgzg2D4mCIPg8C+A4FiwcBvGweRsGkbhlCCMo3HQc4ZDEIAzCAMZAkSQw0CAOJDDeQ5FkyJ4sC+Mo0jaOA+gE",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "alert_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhsGkboIHgMYTg8IB5iYIg4hAeQyieEB4jCDYPhoL4hiOIIigiKoTDaLo0DGQAgiWMZFkIMguDAMYZhuORlD6AQ",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[cfg(feature = "alert_octagon")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig3C4OA2CAMggDENguDENIWhaF4ShSH4YhqHIkhuHQyiCE4ViqHoZiiHohhWM4WCIPg8C+A4Fgcbo4GyDBlCAeAyhAMQyCKQwxkaSAgHmSwiDiSR5kUIpHjeOZAG4ZY/kGSpMkmRJMC4MAxlOVYZlOUJpjgL5alyAQA",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertOctagon,
    #[cfg(feature = "alert_triangle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELg3DMIAxDgLYTDENBhDIIIZDAIIcDALQzC4NA4h0bIUhENBBhmG4di0NIaDEaAxDaGIai2HoRg4M4gFoIg+DwL4DgWP5CgaCAiE0MYZDkdg0j6QJFkSBJGgmSYZDENxoC4MAxk+QZTD6AQ",
        categories = "notifications,shapes,development",
        tags = "warning,alert,danger,exclamation mark,linter",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    AlertTriangle,
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
    #[cfg(feature = "align_center")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig2CKBwxgqDAgHiCQiDODR4g8IgyDEIg+DwL4DgWHohgaE4KDeDYIgoMQyimGYsheL4oh6IIEGWI42g6Kw4heFA1imFAxjyEovDmHYfiQPoBA",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB5g0MoQGgZRpGcaB0g0MYPCAeINDSEByiAIoUD4PAvgOBYoiuBoSCIOYVheGYbjGEIlDGIoHgmC4Nh6JITCKKIqgQdIoHAYYLCAZINE0MgyCCUBIieKZJgsPoBA",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQ2gweYJDSDBygiCoMHcaRkHQaIJDGFQ+DwL4DgWJ4qgaHA5haJIZg2D4RhMIoVCCG4JgsIIfiGIwijCJ4pgQdInHAYYiCAZIJE0MgyCCUBWguRJJiIPoBA",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENAiCAeYODWEhyHiDgyhKGQihEIB3GkZB0GiDg2CIPg8C+A4FimLIGiGI4lCKJ4HgmC4Ng8MIShQIg3heHYbCCHYQiiKovikcBhiQIBkg4TQxDcIAyDIdgthaKQvkqJJJksaJNk+UQgDcVoblmWxol2TJOCITZSlSVgzkaWpemqX5sm4IA1mWc5oD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINAiCAcoICIMoMHmCQ1gwdxpGQdBogkNoMGgZRpGcaB0gkMYLD4PAvgOBYoiuBoPiaEoJDeDIOgmEQgheGYbCKHQgh+IYjiUMAiiiKoEHSKBwGGGggGSCRNDEMAgDIdgykSRpLhqSpMGiTpQleVJWliKZaGgPoBA",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAch4gkMoMg8Ig0gwaBlGkZxoHSCQxhUIB3GkZB0GiCQ2CIPg8C+A4FimLIGhOHoMiGI4lCKJ4NhOEYHgkN4WhiGocCIMQwiiKovikcBhiQIBkgkTQ0CAMh2DKRYpC+SokkmSxok2T4elKVJWiqWRoD6AQ",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDENINHcaRkHQaIKDaDYJguDR5goNQiD4PAvgOBYoiuBoZhuHQih8IIhDGNIkCIN4Ng+EYThUMINgiCoMiiKoEHSKBwGGHAgGSChNDEMggDIdgykGRpLhwPoBA",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAdxpGQdBogkNoMGgZRpGcaB0gkMQ0gwch4gkMoMiEIojD4PAvgOBYoiuBoICIN4fiWI4Ng+EYThWF4ZhsIgxDCJIcieKYuigcBhhEIBkgkTQyDIIAyHYMpAigL5HhEPoBA",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAch4gkMoMHcaRkHQaIJDaDIPCKGQgGgZRpGcaB0gkMQ0CIPg8C+A4FimLIGhOFYXhyDIICIN4aiWHYOhCDIfiGI4lDCKIqi+KRwGGFggGSCRNDIIAyHYMpDikL5IhYPoBA",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMAiCAeIODmEh5g4N4SHKFAiDKEh3GkZB0GiDg2CIPg8C+A4FikcBhiQIBkg4TQ0CAMgyFaH4pC+L4ki6MBojKNAyDCN45juKo+GgPoBA",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIByHiDQyhAeYNDWEITCIM4QGgZRpGcaB0g0MQ0CIPg8C+A4FimLIGhsMYZCCFwiDeHogiKJAiDEMIQhKFIQgiCoMg6KIqi+KRwGGCwgGSDRNDMIAyHYMo+ikL5LguSpMGiTpQDIMZTlWV4qloaA+gEA",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_justify")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig2CIIB4gkIgzg0eAygoMgxg0eYVgsIg+DwL4DgWHohgaGoKDEMoZhCKIThCEoOhuF4dh+JIjgSBoPgqL4mCIMQ4hOMYYgeK4/h6II3D6AQ",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[cfg(feature = "align_left")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgzCIIB5DGCg2g0eYJCKEoHhAIgyDEIg+DwL4DgWHohgaCIKgyGIKDENYThUMQyhOGYvh2H4kiOBIGg+Kg4i2O4NHiMg3j+FYMh6II4D6AQ",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[cfg(feature = "align_right")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgzCIIB5DGCg2g0eIQCIMgxg0eYJCKEg+DwL4DgWHohgaCIKDmE4VheGYbDEMoZhWLgih6IIEGWI42geKoYgeGw3jCCgxDiLJBkONIkD6AQ",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINgiCAch4gkMoMHcaRkHQaIJgsIIPCINIMGgZRpGcaB0gkMYLD4PAvgOBYoiuBoThWF4KgyGwxh0IIfiGI4JDmDIIjODYbhGKIqgQdIoHAYYWCAZIJE0MgyCAMhIkOKZJhYPoBA",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDYJCKFggHmCgxDSDR3GkZB0GiCg5CIPg8C+A4FimLIGhwIofg6EIShSGYXhWIIiiSHYagiCoMimK4EHSKRwGGJAgGSChNDIIAyHYMgwiiKpIiQPoBA",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDR3GkZB0GiCgxDSDR5h2HwggkIg1CIPg8C+A4FimLIGg+EYThWIIKiOJQ3g2CIKgwIIYhqHAiDEMIoiqL4pHAYYbCAZIKE0MgyCANxoC2J4pC+SYbkiShokyTg3lISAxkWWJcluS5NCKT5RDGUwtDOZJZGiZ5dmkTQ1CCbRIgyV5yD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHmDYPhEaBlGkZxoHSDQ2hEch4g0MoRiEIg1CIPg8C+A4FimLIGiCIoWhiGocCKHoSg2EIHgmC4UDCJINDeKIqi+KRwGGCwgGSDRNDIIAyDAaJRkQL5IguR5JGiS5Nk8MZSlSKZWloPoBA",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDSDYJCINYNGgZRpGcaB0goNoNHmFIWD4PAvgOBYoiuBoPhGEwiDEMIXgoN4kgqFoHhiDAghyHogiIIooiqBB0igcBhhIIBkgoTQyCCFRoDKNZFkqEpJksaJNk+UQ0lSVoplgaA+gE",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDR5goMYWCCCQiDWDR3GkZB0GiGQ0CIPg8C+A4FimLIGhiC4giKJIZDCDYPhGE4Vg2CIKgyHIKDeKIqi+KRwGGJAgGSChNDIIAxDIaAyjeKQvkiJA+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHmDgxDKER4g4NYRHKGAihYIB3GkZB0GiFA0CIPg8C+A4FimLIGiGI4lCIMQwhuHYfgiCoMg6EAgh0N4RhOHooiqL4pHAYYkCAZIOE0MggDIMhoDKNopC+SYkD6AQ",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDSDRoGUaRnGgdIKDaDYJCINYNHmFIfD4PAvgOBYoiuBoIgqDAghiGoch6DYPhGEwiDEMIggoN4kjeKIqgQdIoHAYYSCAZIKE0MggDIaAyj6RJJhIPoBA",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDAIggGgZRpGcaB0g0NoRHiDQ3hEeYNDmERyhwIgyCIPg8C+A4FikcBhgsIBkg0TQyDIIAyDASInikL4vguLowGiMo0jYIA0juKIqj8aA+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDSDRoGUaRnGgdIKDaDYJCINYNHmFIjD4PAvgOBYoiuBoYhqHIeiCCg3g2CIKgyDoQhKFAwiSCgzCKKIqgQdIoHAYYSCAZIKE0MggDIMRoDKP5EkmEpIkoaJMk6UAzlSVoplgaA+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceBetween,
    #[cfg(feature = "ampersand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMQyGMMAgDQLg0C0MwuDYIA4C0OIdEGFoPiMIIUhQMQgg8MQ1g6EwthyHg0h2H4OGEMwgjiKImjCJoTjkIAyg6HZDhGEAzCIPg8C+A4FkqTYGgiCgxhyERokiSpMgQaA+gEA",
        categories = "text,development",
        tags = "and,typography,operator,join,concatenate,code,&",
        contributors = "danielbayley,karsa-mistmere"
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
    #[cfg(feature = "anchor")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINQigkeIMDEMoPHKDAzCIPg8C+A4FgeGhsGkboIHgMYShQIB4DKJ4PHmKwiDiLYmCIMoUhoL4hiOGhwGEdBoCAZIME0NQghMSAyGEMQwkWS5NCCSwyk0aAthiN48j4PoBA",
        categories = "transportation,text,maps",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Anchor,
    #[cfg(feature = "angry")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ2CCKRzC0MQuDULQyC0NIyjQIIzDSOIZhuIYjiCIokiYIhNDeMAgDiKwwCAOY8C+PhokCI4lgwbQxjoOY4jCLpOlCUpCieWQxDAaIYhqT5Bl+VJEDENZKmWXZpgE",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[cfg(feature = "annoyed")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ4CAMQ1GgOAihsL4iiSIYjiWJwiikIA5GiGocjMaI1iSJooDENI7j2MI/jYPoBA",
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
    #[cfg(feature = "aperture")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bGwaRuggeQxgwOIQHiJ4NDQLgzDGKgygwMgwC4MA1hAeYzg0NwuDkNAihsL4iiSIYjggeI8DIMQuDEN46iyKQgiuDA5C4Ng5jqPIpkORRlkeJJUiyPgzlOSoTDOToPCCJoZluNI3DaQocl+YZJjyag5jmY5WliWptiwMZzm2PA2nKdJEkid6BhOhI7o6MpxDgM4qoKLowomdg8l+VI8hQLg4lOboNmyVYNocNpspAIpMj+QZeouAQ",
        categories = "photography",
        tags = "camera,photo",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAaBlGkZxoHSCQxDaDB4gkMoMHcaRkHQaIZDCDByhgIoaD4PAvgOBYoHAYYfCAZIJE0MQwCANB2guKAvi6H4ti8aIxjMMggDgaAyiKO49GiP4wjIIhNDaN45CKSpAD6AQA",
        categories = "design,files,layout",
        tags = "application,executable",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    AppWindow,
    #[cfg(feature = "apple")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDALg5DQYwxC4NQgDCDguDeF4VDANggDQIIeiAM4YCANgtDiKAtgwLgyDIQQ0hEMYhjSJ4ZhkMQ3CANRjC0Movg0MAtiKFQ0DQLYXDKLQthaKZBDcOJAkqQBhjMOY2lmOYYkqNpSEODQxiKS4NiuMIUi+F4ZkENYtC6H4hm+HxaCIPg8C+A4FneeoGgiCgxmyFAghaDqGg0NZ2nifQ+gEA",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Apple,
    #[cfg(feature = "archive_restore")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIBoGUaRnGgdIKDWDR3GkZB0GiCgyDCDR5goM4NgkIgyCIPg8C+A4FikcBhhsIBkgoTQ0CAOB2DEMRhDIII9DAIJAkCPQyGiJ4pC+L4bi6MBojKNIejeOY7kSQZWDELZEGiWYoiqShokyMYzCIbQ5CAMQ1CAMwtDOapql2SZNmGTpjE0MY9ncdg5nCXw+gE",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHmDQzhEch4g0MYRGgZRpGcaB0g0NYRhiDgiD4PAvgOBYoHAYYLCAZINE0NAgDgdgxDEYQyCCPAwCCP4/jwMhoDEMo7j2QJKkILQyFYOInimLoLi2LxojGDRtDkLg1CAMQ3CANQtiOKAvlMaJVjCMgilqXJejyXZklKVg+gEA",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIIJCIMoNHcaRkHQaIKDIMINHmCgzg0aBlGkZxoHSCg1CIPg8C+A4FikcBhhYIBkgoTQ0CAOB2DEMRhDIII9DAIJAkCPQyGgMQyjyPpBkuQwtDIVg4iiKovhaLowGiMo0DGQJHGgNJSC+VBoD6AQA",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Archive,
    #[cfg(feature = "area_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFYYgaCIKDcIAxDIdg1hIMhWDgbAtDUIA1C0NIuFqFIWhsPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph,area",
        contributors = "nstokoe"
    ))]
    AreaChart,
    #[cfg(feature = "armchair")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA5FYNhhDIIITDAIIWDALQyhoSA3hKFIXiGGYTDIdgzCIPg8C+A4FimLIGgiCgzCAMYRiSIohiQaAxDSH4VjiG4lC0NY+jiGQ0hcdgyh0doakWGIXC2SAwFqKIqi+LoEjCCRNDWNA4kqVorlqWYFgeXIMl+YYpmOLYBA",
        categories = "furniture",
        tags = "sofa,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Armchair,
    #[cfg(feature = "arrow_big_down_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA1EgOQiD4PAvgOBYThaBoIgqDAgDkdgzGgNBsC0NwgDeJYliIVg5GgNh6hKFIZD6AQA",
        categories = "arrows,navigation,gaming,files",
        tags = "backwards,reverse,slow,direction,south,download",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigDownDash,
    #[cfg(feature = "arrow_big_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2HYNhoDQbAtDcIA3hWFYSFaEQ2HoIg+DwL4DgUPoBA",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,direction,south",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigDown,
    #[cfg(feature = "arrow_big_left_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDUVg5CIPg8C+A4FhSF4GgiCoPg4NRoC0Mx2DQbAtDeJwgigN4kGiIw2HqE4VhoPoBA",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigLeftDash,
    #[cfg(feature = "arrow_big_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDUaAtDYdg0GwLQ3hcIIYDeFBohMNh6CIPg8C+A4FD6AQA",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigLeft,
    #[cfg(feature = "arrow_big_right_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAOR2DYIg+DwL4DgWEoVgaCIKDmDRoDMVg1GwNwgDcLYjDcdgtDQSA5FYOR6hGE4YD6AQA",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigRightDash,
    #[cfg(feature = "arrow_big_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAORoDYVg1GwNwgDcLYUDcdgtDQSIQDkegiD4PAvgOBQ+gE",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigRight,
    #[cfg(feature = "arrow_big_up_dash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ5GgNgiD4PAvgOBYThaBoIgqDAxDUdgtDMSA1GwNwtDcIIoDcaAtDQdoiDkeoShSGQ+gE",
        categories = "arrows,navigation,text,development,gaming",
        tags = "caps lock,capitals,keyboard,button,mac,forward,direction,north,faster,speed,boost",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigUpDash,
    #[cfg(feature = "arrow_big_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4HYLQ2EgNRsDcLQ3CCGA3GgLQ0HaEg5HoIg+DwL4DgUPoBA",
        categories = "arrows,navigation,text,development,gaming",
        tags = "shift,keyboard,button,mac,capitalize,capitalise,forward,direction,north",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigUp,
    #[cfg(feature = "arrow_down_01")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiFxyGUYx0CAeIJDENQiCAdxpGSBYJhUIByHmCQyjaQAij0aBlGkZxoHSCQ2haGIti+HIqgeCRNDGIokHYLQ2GgLZCiiHZTgWVYgjSIwwGiJ4YmKAQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDGIgxiUNBoC0MoWhiHYsgWLogDENYNDAaInjqKoXHIZRjHQIB4gmQAiCAeZPhUIBoGUaRnGgdIJDaURylMIo4CAdxpGSBYJkUL5JksPoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown10,
    #[cfg(feature = "arrow_down_az")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNiQIA4GgLQ1haGIdiyBYuiAMQ1g2JQ2C4NRhDKQ4jkcMAgkoMQgj+JQxDCN4phuAoth8TY+g0NBoDUbI1CANpdlOOYBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownAZ,
    #[cfg(feature = "arrow_down_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCAOB2DgIobC+IokiGI4licIhtDgIIqDQII9DQLQ0i+HIyGgPoB",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownCircle,
    #[cfg(feature = "arrow_down_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAzEgNQiD4PAvgOBYThaBoIgoMQyCAMgxFYN4ShSGYYgSGoJG0NggDENQgiyMQtDaJIVigPoBA",
        categories = "arrows,navigation,files",
        tags = "backwards,reverse,direction,south,download,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownFromLine,
    #[cfg(feature = "arrow_down_left_from_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGEMQwg2EoSDGDYThMIg+DwL4DgWG4egaCAiG2DAygyEQthGGociGIIEiKCRNDgIInEgMh2C0Nosh2MA+gE",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftFromCircle,
    #[cfg(feature = "arrow_down_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIB3GkZB0GiCQxDiDByggIgygwaBlGkZxoHSF4ZD4PAvgOBYoHAYYVCAZIJG0MQ2CAOAtDiNwiigL4uhWLYvGiMYJE2NQgjUSA4FaJ4pj8aA+gEA",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-west,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftSquare,
    #[cfg(feature = "arrow_down_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIINg2DAiD4PAvgOBYThaBoIgqDAggwSA3FYN4ShSGQ+gE",
        categories = "arrows,navigation",
        tags = "direction,south-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownLeft,
    #[cfg(feature = "arrow_down_narrow_wide")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDEMYQGiJ4Yh2LIFi6IIxCAOBoDeFo2iqOIei+PAxDIaAxDCQYphuAQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownNarrowWide,
    #[cfg(feature = "arrow_down_right_from_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYQxDAIISCCEwxhSFAwC2EgiD4PAvgOBYfiKBoIgqD4Og2DIUDKHogiWJIEiaCRNikMQ2HYNhoC0NoviGMw+gE",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownRightFromCircle,
    #[cfg(feature = "arrow_down_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKER5g0M4RGgZRpGcaB0g2D4RhQIoaD4PAvgOBYoHAYYLCAZING0OAgjWNwiigL4uguLYvGiMYNE0MQ2jYdg4EiEI6jwaA+gE",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-east,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownRightSquare,
    #[cfg(feature = "arrow_down_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPCIPg8C+A4FhSF4GggIhNDGDA3HaDxIDeE4VhoPoBA",
        categories = "arrows,navigation",
        tags = "direction,south-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownRight,
    #[cfg(feature = "arrow_down_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIB3GkZB0GiCQxDiDByg4IgygwaBlGkZxoHSF4ZD4PAvgOBYoHAYYVCAZIJE0MQyCAOB2ieKYuhWLYvGiMYJG0OAgjUIA0keRwtDQIooC+PBoD6AQ",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQ0CIPg8C+A4FhSF4GggIhtDEOQgDkLQ3CAN4jiOE4VhqFBjGkchjGwZQgGMeYJDIMQiCAcoJjiMx4jwMopC+LYvjEPoB",
        categories = "arrows,navigation,maps",
        tags = "direction,south,waypoint,location,step,into",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowDownToDot,
    #[cfg(feature = "arrow_down_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcVgzCIPg8C+A4FhSF4GggIhtDaDgxCCH4jC0NoThWGoZgSG4JgsOQgDIMRIDWJ4WisPoBA",
        categories = "arrows,navigation,files,development",
        tags = "behind,direction,south,download,save,git,version control,pull,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownToLine,
    #[cfg(feature = "arrow_down_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRtDIMQgDiFI1hGJ4Yh2LIFi6IAxiINB2g6Fo5ioPoB",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,traffic,flow,mobile data,internet,sort,reorder,move",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownUp,
    #[cfg(feature = "arrow_down_wide_narrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDEMYQGgMQwhaGIdiyBYuiCMQgDgaA3jeKYbgKLYfjCMgxDIaInjiKg+gEA",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowDownWideNarrow,
    #[cfg(feature = "arrow_down_za")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDeEB2g6FoYh2HIEh6CRNDENYQGgNRsC2Lw2jGJoZiqKYFgeLIuCAMgwHYLQzC4NRhDKRpAkoMAgk0MQgi8MBWkGOIogKKo9iCQYNDgaI0laOoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownZA,
    #[cfg(feature = "arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1HYMQ0CIPg8C+A4FhSF4GggIhtDEOQggwLQ3CAN4jiOE4VhoPoBA",
        categories = "arrows,navigation",
        tags = "backwards,reverse,direction,south",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDown,
    #[cfg(feature = "arrow_left_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ2CCDhIDiGYbiGI4giKJImCIbYOCAOAtDQII9j+LwvjEaA+gEA",
        categories = "arrows,navigation,shapes,gaming",
        tags = "previous,back,direction,west,sign,turn,button,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeftCircle,
    #[cfg(feature = "arrow_left_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CANgtDaDYRDYIg+DwL4DgWFYYgaCAiE0MwgDEMhoDENIUhaG4agSHIJE0MgxiEORWDWJ4XisPoBA",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,expand,fold,horizontal,<-|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftFromLine,
    #[cfg(feature = "arrow_left_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMwgDQIA3GyEA0CIPg8C+A4FheGoGgiCoQDcaAxDaFoYh2HIEh6CRtiQIAyDGDwtDSM4ziaGYqimBYHgkTQyDAIAxDcSIVheOIbgEA",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "wojtekmaj,ericfennis,karsa-mistmere"
    ))]
    ArrowLeftRight,
    #[cfg(feature = "arrow_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAeYJgsIByggIgyCIPg8C+A4FikcBhhEIBkgkbQxDIIA4C0NAgjuPYoiqL4Ri6MBojKCRNDENggjYSIVikL5BGgPoB",
        categories = "arrows,navigation,shapes",
        tags = "previous,back,direction,west,sign,keyboard,button,<-",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowLeftSquare,
    #[cfg(feature = "arrow_left_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQ5FYNQiD4PAvgOBYThaBoICIbQxgwNgtDYIIiiSEoUhmGIEhqCRNDeDQyGgMQ0iaFYqD6AQ",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,collapse,fold,horizontal,|<-",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftToLine,
    #[cfg(feature = "arrow_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDkLQ3hEIISDcIg+DwL4DgWGIbgaCAiE2D4ODISA1heGYeD6AQ",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeft,
    #[cfg(feature = "arrow_right_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgg4aA4hmG4hiOIIiiSJgiG2DoqDYIA0C2PI+i4L4wGgPoBA",
        categories = "arrows,navigation,shapes,gaming",
        tags = "next,forward,direction,east,sign,turn,button,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRightCircle,
    #[cfg(feature = "arrow_right_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANR2DENAiD4PAvgOBYThaBoIgoMgxCAMQyEgN4ShSGYYgSGoJG0MQ1h8OAgDYLYxjOJIVigPoBA",
        categories = "arrows,navigation",
        tags = "next,forward,direction,east,export,expand,fold,horizontal,|->",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightFromLine,
    #[cfg(feature = "arrow_right_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAzCANIQC2EQ0CIPg8C+A4FheGoGggIhNDIMAgDcSIVheGYEGiHIqgeCRtDgIAyDGE4ThKJ4Yh2LIFi6IIRDENxogyFo5ioPoB",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "karsa-mistmere"
    ))]
    ArrowRightLeft,
    #[cfg(feature = "arrow_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEeYUhEcoTCIMoRGgZRpGcaB0g2DwiD4PAvgOBYoHAYYLCAZINE0OAgDEMhohCKAvi6C4ti8aIxg0bY3jYNggDQLZJkuJ4pj0aA+gEA",
        categories = "arrows,navigation,shapes",
        tags = "next,forward,direction,west,sign,keyboard,button,->",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowRightSquare,
    #[cfg(feature = "arrow_right_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDISAzCIPg8C+A4FhSF4GggIhtDEMYODgIA2C2JImhOFYahmBIbgkTQyiANR2DENIohaLA+gEA",
        categories = "arrows,navigation,development",
        tags = "next,forward,direction,east,tab,keyboard,mac,indent,collapse,fold,horizontal,->|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightToLine,
    #[cfg(feature = "arrow_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FhSF4GggIhtg4IIMDcIA3C2Ig3hOFYaD6AQ",
        categories = "arrows,navigation",
        tags = "forward,next,direction,east,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRight,
    #[cfg(feature = "arrow_up_01")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyFhyGUYx0CAeYJDQIggHKLwiDKMh3GkZIFjCMh4gkMQ1jIaBlGkZxoHSCYlhYL4qiyG4Eh2CRNDGIQyDAdgtDYaAtjeTIogKUYHlOQQglcaIxl+UQ+gEA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp01,
    #[cfg(feature = "arrow_up_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDGIQxDAVg0GgLQyiaGIqimBYHiwMQ1CCMBoDSN4oDwchlGMdAgHcaRkgWCZDCAch5gmNgglQIgxlEaBlGkZxoHSCYlCAeIJj6N5HkkPoB",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp10,
    #[cfg(feature = "arrow_up_az")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDIMINGgLQ1iaGIqimBYHiwMQ1CAMQwFYNguDUYQykIIJFjyL4vDEIJJFaPo0iiAoqjmH47j0NBoDUbIyCANpalGNoBA",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpAZ,
    #[cfg(feature = "arrow_up_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDYIIOC0NIui4IA0CKGwviKJIhiOJYnCITYOiwNhWDiNIcjcaA+gEA",
        categories = "arrows,navigation,shapes,gaming",
        tags = "forward,direction,north,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpCircle,
    #[cfg(feature = "arrow_up_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIAxDYLQ0CANIRhEIg+DwL4DgWGIbgaCAiE0MQ3CAMgwFYNIXhmHodgSH4JG0MwgDiE4RhOE4qhqLotgWB4JE2JA0HaD45iyAQ",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,mobile data,internet,sort,reorder,move",
        contributors = "it-is-not,karsa-mistmere,ericfennis"
    ))]
    ArrowUpDown,
    #[cfg(feature = "arrow_up_from_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAOQgDcLQ3g+DwiD4PAvgOBYWhmBoICITQxDIIAxDYVgyhWF4chYYxpHIYxsGUIBjHiCYhCIIByjWNxjHmCQyDGKAviyLowD6AQ",
        categories = "arrows,navigation,maps",
        tags = "direction,north,step,out",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowUpFromDot,
    #[cfg(feature = "arrow_up_from_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIA5C0NoQhAIA2CIPg8C+A4FheGoGggIhNDEMggDMdgxDSFoYh2HIEh6CRNDUIAyDEaInimGYtD6AQ",
        categories = "arrows,navigation,files,development",
        tags = "forward,direction,north,upload,git,version control,push,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpFromLine,
    #[cfg(feature = "arrow_up_left_from_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBWDIaA2CIPg8C+A4FhSF4GggIhtgyDAxDAIIhhOFYahmBIbgkTQxh8QYhiOIoiDGIwgiAMolhaKQ+gEA",
        categories = "arrows,navigation,maps,development",
        tags = "outwards,direction,north-west,diagonal,keyboard,button,escape",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftFromCircle,
    #[cfg(feature = "arrow_up_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEaBlGkZxoHSDYPhEeYUhEcoTCIMgiD4PAvgOBYoHAYYLCAZINE0OAgDENhWDgaIQigL4uguLYvGiMYzjeNg2CCNY8imPxoD6AQ",
        categories = "arrows,navigation,shapes",
        tags = "direction,north-west,diagonal,sign,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftSquare,
    #[cfg(feature = "arrow_up_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ3FYNxoDEMAiD4PAvgOBYWhmBoIgqDoNgyIoVheHA+gE",
        categories = "arrows,navigation",
        tags = "direction,north-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpLeft,
    #[cfg(feature = "arrow_up_narrow_wide")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDEMQgDEMhoDSJoYiqKYFgeLIujANhoDeNYogKKo6h+PAyDAaAxDCQY3gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "lukesmurray,ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowUpNarrowWide,
    #[cfg(feature = "arrow_up_right_from_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIQQxDCDoThMMYOg6DQyCIPg8C+A4Fh2IIGgiCoMCCDYPhmHIeiOIoEiSCRNDENooGgNh2DaLIfjAPoBA",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,north-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowUpRightFromCircle,
    #[cfg(feature = "arrow_up_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEaBlGkZxoHSDYPhEeIUhEcogCIMgiD4PAvgOBYoHAYYLCAZINE0OAgDgaA4HaEIoC+LoLi2LxojGDRtjUMQ2jYLY7imPhoD6AQ",
        categories = "arrows,navigation,shapes,social",
        tags = "direction,north-east,diagonal,sign,keyboard,button,share",
        contributors = "danielbayley"
    ))]
    ArrowUpRightSquare,
    #[cfg(feature = "arrow_up_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CANxoDEMB2hAIg+DwL4DgWFYYgaCIKgwMYfgwN4UhaGw+gE",
        categories = "arrows,navigation",
        tags = "direction,north-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpRight,
    #[cfg(feature = "arrow_up_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEaBlGkZxoHSDYPhEch4g0MoRiEIoVD4PAvgOBYoHAYYLCAZING0MQ2CAMQyC0NI6joIA0CKKAvi6C4ti8aIxg0TY4jcNhWhCQZDGgPoB",
        categories = "arrows,navigation,shapes",
        tags = "forward,direction,north,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowUpSquare,
    #[cfg(feature = "arrow_up_to_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxoDENAiD4PAvgOBYThaBoICIbQxDgIAxDMLQ2iOIwgDaEoUhmGIEhqCRNDEMggDcdoQimFYtD6AQ",
        categories = "arrows,navigation,files",
        tags = "forward,direction,north,upload,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpToLine,
    #[cfg(feature = "arrow_up_wide_narrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDEMQgDEMhoDEMImhiKopgWB4si6MA2GgN42iiAoqjuH49DIMBoDSQo4gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpWideNarrow,
    #[cfg(feature = "arrow_up_za")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDENYPGgNRsC2Lw2jGJoYiqKYFgeLIuCAMgwHYLQzC4NRhDKRpAkoMAgk0MQgi8MBWkGOIogKKo9h+QQgDEOBojSVo6gE",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpZA,
    #[cfg(feature = "arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAMQyCANwtDeEIQCIPg8C+A4FheGoGggIhNg6DQ5FYNYWhiHQ+gE",
        categories = "arrows,navigation",
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
        categories = "text,maths,development",
        tags = "reference,times,multiply,multiplication,operator,code,glob pattern,wildcard,*",
        contributors = "mittalyashu,ericfennis"
    ))]
    Asterisk,
    #[cfg(feature = "at_sign")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeYLDEMoOGMeIShQPg8C+A4FgeGhwGEdBoCAZILE0MQ2CAOB2DUYQzCCMAwCCM4zioMB2C0MRhDGM49jQIAxjQLQ0isIoaC+IYjD6AQA",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMYQGMeITg8Pg8C+A4FgeGhwGEdBoCAZIME0MgwC4MggimKxjDILgwDQLYxDAM4yDILQ3C4Mw2C0NAuDULQxDELg5kCQo0kENY6DkLg4DcLQ2kqRJGkiTI1jINItjIMwtjkII8j4IJMCCRZHmWSpqk0IJPlEIJUDWXJoDmbBaCKGgviGI4giKJImCITQxDULg3mehQ3GOTI6kyXJyjSb6HlmdZajelo0mOP45laaZZoybI0pCbpQlKZp1l2N6plymphpWTJ4nqfBoD6AQ",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[cfg(feature = "award")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwOIQHKDA2CIPg8C+A4FgeGhwGEdBoCAZIME0MQ1C4NA3DcIIOC4OA5i+LgyDIbAtDULQzjkIAzi8Lg1DKPA5C4MQxhmG4hiMPoBA",
        categories = "account,sports,gaming",
        tags = "achievement,badge,rosette,prize,winner",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Award,
    #[cfg(feature = "axe")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDILQ4C4NQghINRhDILoPCCGYbDCDoOC0M4iEwMQxCAOQiD4PAvgOBYri6BoICITQxhQMQzigIA3GwNAtg0NggDYaAzGEOIVCCH4fDELQ3jseoqiyMQ+gEA",
        categories = "tools,gaming",
        tags = "hatchet",
        contributors = "Andreto,ericfennis,csandman,jguddas,karsa-mistmere"
    ))]
    Axe,
    #[cfg(feature = "axis_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENhohAIg+DwL4DgWFYYgaCAiG2DAyDAIA3C0N4UhaGw+gE",
        categories = "design",
        tags = "gizmo,coordinates",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Axis3D,
    #[cfg(feature = "baby")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQyGgLgwDEIg+DwL4DgWFYYgaCIKDENYNg+EYThWF4EGiGongeCRNDEMINDYYwuDULgzg0LgyjMIAyCCMxzDGMwtjiO5CDWFIWhuKYFiuHoMDaNRhgyDIvi8MY3DgIAzC6DI8jyVINCCL5aDYIJSmGZwxC0MQ3C6ZIvl2Z5VmELZjEGZpflaDpZGOXpZjqQJWlqII5DUc5ClyQZdjMY5ClgMJqkELg0pANZqkeJoZgEA",
        categories = "accessibility,people",
        tags = "child,childproof,children",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Baby,
    #[cfg(feature = "backpack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGGDIMDAIITDEIA0C0NBoDiEIXhSH4WhEdoOGEMggiaE4VC2JgyEgNolieH4qDKKxaCIPg8C+A4FjiO4GgiCg5CANhWDSMIoiCJ4rGgMpHjKDYxDIdgyjeOY+j2BI/gkTQ4icMR2C0NZOimUI0DIaJGiyT4Wiwdg1lWOpZliBYHluXYOhucJXgKWZ1gqdw4nmOJxjyAQ",
        categories = "gaming,photography,travel",
        tags = "bag,hiking,travel,camping,school,childhood",
        contributors = "karsa-mistmere"
    ))]
    Backpack,
    #[cfg(feature = "badge_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDEMgimMMZmmgIB5msIg4mkeZlCKZ5YlqX5hl6YJinSZg2nOcAxoGapsmmZJsC4MAxngL56GUPoBA",
        categories = "account,social,shapes",
        tags = "check,verified,unverified,security,safety,issue",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeAlert,
    #[cfg(feature = "badge_cent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxDIIA3HYMQwliWpfl6BJggkTQxDULoWmiFY8jePQ0mqW5uD6AQ",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    BadgeCent,
    #[cfg(feature = "badge_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDmNgyCCZpmDSIZYlqXw+gE",
        categories = "account,social,shapes",
        tags = "verified,check",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeCheck,
    #[cfg(feature = "badge_dollar_sign")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxDaEBokYYQyCCaYoksNBoDSaJqlKNo9DQSA4liWpfl6BJggkTQxmkMQ4FYNp5lufQ+gE",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,usd,$",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeDollarSign,
    #[cfg(feature = "badge_euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCokDEMhoDWWJal+XoEmCCRNDGDw5C4NIVjyN49DULgymiW5sD6AQ",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,€",
        contributors = "danielbayley"
    ))]
    BadgeEuro,
    #[cfg(feature = "badge_help")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCg5C4MA5CAORhDMIJqkuHA1g2agxGOGQyC2agznaa5YlqX5ZGwaRuGUIB5DKCQxDcIggHgMaGDKiR4oUIgxDKZAxokeaMpKiJZC+f6BD6AQA",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeHelp,
    #[cfg(feature = "badge_indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCpCDgaA4liWpfl6BJggkTZCDEMplmeW5rmqBYHgkbQxDONg3C0NQtDEaAxhWPJLDALZmlmdJdgEA",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,inr,₹",
        contributors = "danielbayley"
    ))]
    BadgeIndianRupee,
    #[cfg(feature = "badge_info")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHkMoJDEMgiCAeJlCKZ5pHgMZmmiY5xm0NpYlqX5hl6YJinCcppHmdQ4m+bJnC4MAxoGbKElkL56GUPoBA",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeInfo,
    #[cfg(feature = "badge_japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDmEAgDOZh2DeWJal+XoEmCCRtDEMo2hwMwtDOa5bm+boFgeCRNmScxoDaeptgKb5/gqgg2oShp8gEA",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,jpy,¥",
        contributors = "danielbayley"
    ))]
    BadgeJapaneseYen,
    #[cfg(feature = "badge_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDENgiCAeZlCIMQymkeQxmab5jnIIg4liWpfmEPoBA",
        categories = "account,social,shapes",
        tags = "verified,unverified,delete,remove,erase",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeMinus,
    #[cfg(feature = "badge_percent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGggIhtDGDw5kaKZYlqX5egSYIJE0OQgDkaAuDAMZplubZsgWB5vmSNg1nSdp4muAQ",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePercent,
    #[cfg(feature = "badge_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMYJDEMgimMMpmmgIB5mUIg4mkeZrCIMQ2liWpfmGXpgmIeJ1nec5wmec6Bm2ZIJnKWQvnsZQ+gEA",
        categories = "account,social,shapes",
        tags = "verified,unverified,add,create,new",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePlus,
    #[cfg(feature = "badge_pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCpCDEMhoDSWJal+XoEmCCRNDGGw2FYOQuDUYQynUIJ4kiUggkiaJbmya4FgebpjDYaA3oCaoBA",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,british,gbp,£",
        contributors = "danielbayley"
    ))]
    BadgePoundSterling,
    #[cfg(feature = "badge_russian_ruble")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCg5jYNhoDWWJal+XoEmCCRNmMMQymYYQyCCdYoieIRoC0Mx2DmaJbmwPoB",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,rub,₽",
        contributors = "danielbayley"
    ))]
    BadgeRussianRuble,
    #[cfg(feature = "badge_swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCgxhwMQ3FYOBoDSWJal+XoEmCCRNmONgyGgM5rlub5ugWB5xDmNg2mmeJtgEA",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,chf,₣",
        contributors = "danielbayley"
    ))]
    BadgeSwissFranc,
    #[cfg(feature = "badge_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMYJDENQiCAeZlCIOZpHgMoJm6apxCKZ5YlqX5hl6YJinCZpomqbJzmScppHmdZ3lkL56GUPoB",
        categories = "account,social,shapes",
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
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgSA2GGDYNDAIIWDELQyhoVg3hMIIVheF4ahoIg+DwL4DgWJ4qgaCIKDEN4ODQVg0h+IYWDCJAyGgLQxjeIo5hqIB2DEMImiiLYnHIZRjHQIB4gkOAiCAaBlGkZxoHSUpUHmCQ2lQdxpGSBYJDEM5UHKUQiDGSAvkyTonGMaRyGMbBlCAY5rg+VBjl4IgykcIBygkMpunOdZ3nKdJ2nieoJDmfZ/oGaaFoejKKgEA",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQ0C4OQgiqLAxioMggjALoahyIokD6AQ",
        categories = "account",
        tags = "cancel,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,circle,slash,null,void",
        contributors = "danielbayley"
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
    #[cfg(feature = "banknote")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoIgqDBoGUaRnGgdIJDGCwgHmCQ2gwdxpGQdBogkMgwCIPg8C+A4FikYxpHIYxsGWDYlgwY4cCKGY3g+O4pC+L4xjOKRwGGIwgGSCRNDYIIZGgLgwDETQxDiTQyk+UYoiqRYjD6AQ",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "bar_chart_2")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDAIggHiCYLDiDh5DGCgyg2D4WhIIg+DwL4DgWHohgYeIbDEMoOhCCoohSG4YhSEQ0h2H4kiOBIlhENoqhuO4HhEMYzgeL4Nh6II4D6AQ",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart2,
    #[cfg(feature = "bar_chart_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFYYgaCIKhAIAxDcVg5hSFobhqBIcgkTQxgyIRWDWJYXimKIFgeKw4iANx2C0M4yieAQA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart3,
    #[cfg(feature = "bar_chart_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFYYgaCIKDGDAxDcVg5hSFobhqBIcgkTYQCCIRWDWJYXimKIFgeKw4i4Nx2C0M4yieAQA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart4,
    #[cfg(feature = "bar_chart_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFRyGUYx0CAaBlGkZxoHSCQ3CIIB4iaKByioIgxigdxpGSBYJDSKB5gkMQwhSFobh2Goch6LgxDWLJEjGM41CKNwgjkIpGh+IYjiWLwyj0L4/HQPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartBig,
    #[cfg(feature = "bar_chart_horizontal_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFRyGUYx0CAaBlGkZxoHSCQ0CIIB3GkZIFgkMQyigeYJDWKB4gkN4oHKNgiDGFIWhuHYahyHoqiwaI3jGLgziiIIiiSJo1kgII6i6PgvkAdA+gE",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartHorizontalBig,
    #[cfg(feature = "bar_chart_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFYYgaCIKDcIAxDYaA4hSFobhqBIcgkTYfDEMYSDKJYXimKIFgeK4fiIM4yieAQ",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChartHorizontal,
    #[cfg(feature = "bar_chart")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgyDAIoHDKCgxg0IB4gkIgxDKDh4hCF4ZD4PAvgOBYfiKBh5hwNIahwMQ4hqFosg6CIKgwIofiGBBliSOIHhaNIPhENougqQYUhyQY2iUPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart,
    #[cfg(feature = "barcode")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANR2DENAiD4PAvgOBYThaBoIgoOINg+EYThWBBohiI4HgkTQxDKHoQhKFIZiWBYngoMQ3iyIIviOMYaigMgxjeLoiheAQ",
        categories = "shopping",
        tags = "scan,checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer",
        contributors = "danielbayley"
    ))]
    Barcode,
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
        svg = "gAPBwGEdBoCAZA9CITQ5CANoNC4NQgDOEBhDGEAghaEQwCCGwwC0MQthAQw0C4Ng4DOEggDSEolieKoqhAVgxDcYQyCCNodhyN43GgMQyjWO45hsMgtDIdgtDUIg+DwL4DgWSxsGkbhlCAeAxgkMQwCKVQygkOJbHmVwikkIB5l0Ig3kqTJRlOUJSlSYZYDKW5Wgmc5cnad5mnKagvmwZZulOeJomCZwyDGYJiDEOZ0mKaZLn6b6BlSdQijOdJnpeZaGoiZaKoykJ/D6AQ",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bath,
    #[cfg(feature = "battery_charging")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA3GgMRhDIIITDAIIWDGFIUHYNoShqFoYC2EwyGiIgiD4PAvgOBYoiuBoIgoNoOEgNIehWF4XiKG4diOOIghqJAxieKYui2BIvgkbQxhkNwtDMIA1GgNBsk6UJDiqR4oGwaRuGUIB4DKCQyDIIggHkMYJkuZR5mEIgxDOZR4mgIpjleW5dD6AQ",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    BatteryCharging,
    #[cfg(feature = "battery_full")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAch4gkMoMGgZRpGcaB0gkMQwgwdxpGQdBohkNoMg8IoRg2CImCIPg8C+A4FiwbBpG4ZQgHgMoQiceY4CIMQzgweQxhkMYkkKJoRiwL4yjSMYzjWQZDiSPIjjaRpUjuGY/kmSxlk2NI2jyGpAmGP5VhmG4HkYMZEluTpek+apEgeZJShkNJFnaK4tlwPoBA",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAdxpGQdBogkMQ2gwaBlGkZxoHSEwwgwch4gkMoMiEIojCAcoIiYIg+DwL4DgWLRsGkbhlCAeAxiKJx5jkIgxDGJAyjqDB5kKPgziyLozjWMo0jaOIJhWB49j+RJGDGSI3kaFYtC+SxlD6AQ",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDYIggHIeINDKEYUCKFoSHmFYRhwIg3hEaBlGkZxoHSDQxDAIg+DwL4DgWLRsGkbhlCAeAxhWGh4DKOoejmDgxh6PYODOLIujONYyjSNh5kQMZGjeQIQjeRJUHmQAxkKLQvkkZZLjUIJOimUY4imK5ilmQpVmeR5dkwPoBA",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA3GgMhhDIIITDAIIWDGFIUHYNhjhgLYZDILYTDIaIjCIPg8C+A4FimLIGgiCg2g4SA0hKGoWjmI4bh2GAgiGGolDKKIqi+KRsGkbhlCAeQxgkMQxCKTAyk8M5SHiTgiDKQwgHiVJakOKQvkiSpHkmS5NgkN5Xl8MQwlIeZtlaXZZm6RJjmeZpKkydZrnST5vl2baBnGTw3C4MJRmKZBlD6AQ",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAggmC4NHcaRkHQaIKDENoNg8Ig3g0aBlGkZxoHSGwwCIPg8C+A4FiwbBpG4ZQgHkMobDODR4jiC4RHgMYQhEeZBCIMQxiuLYyjQPoBA",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,johnletey"
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
        svg = "gAPBwGEdBoCAZA9CITQ5CAORjC0Lg2DSEQ0C0MQuDUMoXDkNYVDILg0DAMgghcMQ2DUQQ2CCKgwCCLYtDgIAyDIYQxDMLg5iqNo4iyLo+DmOAthMMQiD4PAvgOBZGkmBoIgoMQwC4Nw1CANQuDAOQzimK4+i0MYyiMOBji2Hw0DEMYQDaX4TDYOIWhEOAziuEQ2kWR5MkuBJNgkTZWDOZoklENo0DQIKFi+Pg2iAOA3iQLgyDAOBNlCEYjn6ZhhhMMAxlSmqcl2JIylcM4Mh8MA0naSJ6kYbBpG4ZQgHkMoJjMIggHiswirWsQxrSth4r2uqpq2rw+gE",
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
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedDouble,
    #[cfg(feature = "bed_single")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgwHYLQ4GEMoNCAMIWCAMYNC0MhoDENIThWF4XhqFAyHYOAiD4PAvgOBYri6BoIgoNYZDAVg2iGFIjhmG4dDEMI6hiJIVicNIqiyMYwgSMoJguGQ4h6KYri2TA+gE",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedSingle,
    #[cfg(feature = "bed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANB2DENgiD4PAvgOBYThaBoIgqDA4GgMQ4GGDIMDAIIlDEIIjg8MIShSGYYgSGoJgsIAxDcaAyiyE4VjGMIFgeMw2CAOB2DmLY8heAQ",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bed,
    #[cfg(feature = "beef")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyC4NQiCAcoMg+EYJHmDA4hAIg+DwL4DgWB4eHAYR0GgIBkgwTYOhAIAyGENoujINQgDCNo2C2MgyDIIA0C4NhjC0MQuDEIAzkUMwtC4Nw4kcLg5C2SAxk6MgwDgQQzkeOI3jeNZUGMNI4hsNJDC6TgxkSZY/DMQY0CCb5djiLY1DIWodh+JYniSJooioIhtlSM4vkUOY+hCMQuDSVaKk6cpGC4M5Gj2Mg0oalaGo8LYPDYIA1C4MhDDGn5iDIMKgCCaYvj0N6rGGWpapqnA4mYNg2EyD5iDGNJ4C+ehoD6AQ",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak",
        contributors = "kemie,ericfennis"
    ))]
    Beef,
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
        svg = "gAPBwGEdBoCAZA9CITQxDkLg0CAMQ0C4ORDDIMAuDKEA2g4IAyDGEA3h2HwxDcSAzHMMwthmKQ5GMMAtDMLgzh0Lg3C0NggDaNwgjUIAwhCMguiOE4yCIPg8C+A4FkeSoGgiCgxheM4eGEMYTg+Vg5g+P5cCCMZbkaSJNkcYxpHIYxsGUIBjHiCQxDgIprHmCZwCAcoJDOYQvmWZ5pD6AQ",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley"
    ))]
    BellDot,
    #[cfg(feature = "bell_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg0CAMQyGMLg4CAM4UCAMguDYIA1hmG4dEgMxzDMLQyhYLQ5GMMAthcM4fDcLYcDaMoQhgMIWg6GA0C6HgyCIPg8C+A4FkGRIGgiCgxDALovDIMRhDELg5g+UpUCCOJZjmDwwkCQpHkaBJIgmC4eDgaA2l6Q5iD6AQ",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder,delete,remove,erase",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellMinus,
    #[cfg(feature = "bell_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NwgDMQQ2CCEgwCCFQxCAMQ4CAOBhDIMQuDMIIfiGFomhULoSDUIg+DwL4DgWLYwgaCIKDGDo3EgMxzDMLQyg8LQ5GENIpg6RA2g6FYXCCIQtiAN4si6M4ygSNIJE0MQwiWHxhiAOQ0hkLpfieJgzC6YAwlGL5VlSBYHgkbY/nKFQymmLZrjGAQ",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    BellOff,
    #[cfg(feature = "bell_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLgzCAMQ0C4OBDDIMAuDGEA2C4NAgDKGQxDeHogDcSAzHMMwtDIIIpDkYwwC0M4Oh4Lg3C0NggDaN4QCAMIQC6DYrDKE41CIPg8C+A4FkeSoGgiCgxheD4fGEMZAh2Vg5h2PpciyHI9kaSJNkyBJOgmCw1CAOBoDaYZJmWZIFgeZwxDgIA1HabZHm+S4BA",
        categories = "notifications",
        tags = "notification,silent,reminder,add,create,new",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellPlus,
    #[cfg(feature = "bell_ring")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOBhgyDAwCCEgxCAMQyhMY4SDcIAzCAOYdh8SAzHMMwthiJg5CIPg8C+A4FiyL4GgiCgxDALoeDIMRhDELg5DSFo+kCEpEh0LpDiuLYyjGBIzgkTZADIQwyC4OJGhyGA1C6WINkmLpNkyBYHk8MoYDiGonjgLZVC0NJqDILQ2l6S4BA",
        categories = "notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "ericfennis,danielbayley"
    ))]
    BellRing,
    #[cfg(feature = "bell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOBhgyDAwCCEgxCAMQyhMY4SDcIAzCAOYdh8SAzHMMwthiJg5CIPg8C+A4FiyL4GgiCgxDALoeDIMRhDELg5DSFo+kCEpEh0LpDiuLYyD6AQA",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    Bell,
    #[cfg(feature = "bike")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMLg1CIIBjHiCwxDiDoQGMeYUDeFw+DwL4DgWB4eiGBoIgqDIXhGGgiDGHIPhGEwiDWHYfiWI4CgSJoJhSGIsjCEoUg+HogjqOBwGEdBoCAZILE0MQyCCLoOFYMQ0GwLQzlkIA0luUQzGgMgikSSJKD6AQ",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMQ0CIIB5gmC4NHcaRkHQaIJgwIBoGUaRnGgdIJDaDRyggIgyCIPg8C+A4FimLIGiSCYnCCE4VhcIoZg+OINiWIoahyHogCKIopiuBB0ikcBhhYIBkgkTQ2CAMgwGiDJFkqFpJksaJNk+CwgDGVJWiqWBolqTJOCKUJgDQaAyHaRJkluZ5cmkTZfm2b5xC+ZQ+gE",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDGDYPHmEgxC4OQiD4PAvgOBYHhwcBhHQaAgGSCxNDYLg3CAMwuDQYwthkIAyC4NQgDAIA1C4Mo1j0IIrDcQ4rjgOYuiwIA5C4No1CAMYYDaG4diOJYiiSJooCIbQ4jQMQwC4MZPjALg4lML5VGiV4lieKQxDcLgzkiMY0jaOI6jwMgtjaPpCj8NAthiPp5jOTYrkeGA1meaZrlmCxtDGPJil+YaBmSZocmiWKNm0IhNDGfgymAOIyjYNozoANJMnuq6qDcLY8DOM4+qarJiqoOAtq6O49oumw8mmnafj4MYvDkdpRr6VrAliwrFjevIxoGs57tScgwsqarMmyWqfi2oIwGOLIzt+PaWk2iasoqmaMtujqejyOLPDm4rGmWYw4mOOJ2tkPoBA",
        categories = "science",
        tags = "fallout,waste,biology,chemistry,chemical,element",
        contributors = "danielbayley,ericfennis"
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
    #[cfg(feature = "bitcoin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg3DYNwgDEOQuDAOA5GMNAuDkMoaDgNg4CANguDENAtiMMAyDWEguDIMQ2icLoXDQbQtg2Lg2iKMg5DQTA1jKOQxDiFQ0DcbY/DmFosDANAyC0LgzkWLA5kaDQ1DYMwtkOHw0hmG4dkAOY6iWMYpiuNwxDWMYXDONZoiuI5sC0M4biYLg2jyR4kDWaojDITJDDKYoaDINp6kmIYNkwMpQDQOI2hsNxNDcLqOCAMgwlANg0GydAxoWNqUDcMg3CIPg8C+A4FD6AQA",
        categories = "brands,currency,development,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Bitcoin,
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMYMHcaRkHQaIJDeDBoGUaRnGgdIYgyDwiDENAiD4PAvgOBYoHAYYWCAZIJE0MQwCAMgxFYOBhDEII9jaQAtDGQhIDSPI+CCQJJkKPh2DEMpHj+SZTj0MRok+UZTkqQwxHYLQ1lmSgwkKRILigL4uhYPoB",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPC0NQgDUVgyGyFA1EyDg3CIPg8C+A4FiAbBpG4ZQgHkMoJDEMgiCAeIsCIMgxjAeAxi0OIwHmOQii6H4hiaKIlieKY4gkM43jMNo8j6QIqjOQIgC+QxlD6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDIYYNg0MAghSFgtDgSA2HYOAiD4PAvgOBYfiKBoIgoMQ1CAMgwhEIIThWMQwhiGocFqHogiUPoB",
        categories = "text",
        tags = "text,strong,format",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Bold,
    #[cfg(feature = "bomb")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQzCKCR4gwMQxhAcoMDkIg+DwL4DgWB4bHAYR0GgIBkgwTQxDQLgzDUIIrDaLgxDaLAgDILg3GGNw0DGNgujwIAwkEII9DOP5BGwMQuDaRJLjqR47kOQo9kKRg0GwLZKDmMguluGociKJIhiOJYnCIbQyDKNpZC6XA1l8L5hGgPoBA",
        categories = "",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCAiG0OINDMIA0C0N5HCAN44jqP4+gSQIJE2D4yDEMRoDULpNjmO5SD6AQ",
        categories = "text,maps,gaming",
        tags = "dictionary,define,definition,thesaurus,encyclopedia,encyclopaedia,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,alphabetical,a-z,ordered",
        contributors = "danielbayley"
    ))]
    BookA,
    #[cfg(feature = "book_audio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDgIA4HYM44jqP4+gSQIJE0MQyiAdg3kiO5MkuBYHk4MQ2kSRpVkqAQ",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookAudio,
    #[cfg(feature = "book_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCAiG0OQgg+GwyhkIA0C0NI4jqPw+gEA",
        categories = "text,development,gaming",
        tags = "read,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "danielbayley"
    ))]
    BookCheck,
    #[cfg(feature = "book_copy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ2FYNBhgyDAwCCFQxCAMgtDIaAxDEIg+DwL4DgWIYkgaCIKDWDQ0EiEYThaDYxhUNIdiCIoniaBIogmC4MDEOBIh6EoZjGGIVjSHQxFYNpCDGRIUjOG4ZHYMQyjeI47D6AQ",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,clone,fork,duplicate,multiple,books,library,copies,copied,plagiarism,plagiarised,plagiarized,reading list,information,informed,knowledge,knowledgeable,knowledgable,education,high school,university,college,academy,student,study,learning,research,smart,intelligent,intellectual",
        contributors = "danielbayley"
    ))]
    BookCopy,
    #[cfg(feature = "book_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAyDIaAtDIIg+DwL4DgWFYYgaCIKgwIAxDUdoQhKFIWhuGoEhyCRNDSIA5C4NRWiGJoXiqKYFgeLIfDgdgzjWKICiqOoKDEOIOGgMojkCN5CjmHYtiAMRWDmTIZk6K5FDKSIThWNpXhuRBNDGW4PkmVhojiWZjlsMQ3meXpBmGUJHg8SA2jEYQyjGDp8g2DQxCAMAtDUSA4miapii6IgtjEQZ7DWfaRn+IAgnikQyoaiIB",
        categories = "development",
        tags = "code,coding,version control,git,repository,template,draft,script,screenplay,writing,writer,author,unwritten,unpublished,untold",
        contributors = "danielbayley,jguddas"
    ))]
    BookDashed,
    #[cfg(feature = "book_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDEMoNDMVg3jiOo/j6BJAgkbQ5g2HQzCCVAzC0M5JjuTQ+gEA",
        categories = "development",
        tags = "code,coding,version control,git,repository,pull",
        contributors = "danielbayley"
    ))]
    BookDown,
    #[cfg(feature = "book_headphones")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORjGkchjGwZQgGMeYJDEMgiCAcpIksYx4gkOY4jqP5BkOOY8gaCIKDiDQyhIMhhgyDIwh+XomkqOY7gQaI+kCQpElCSA1k+RwikmS5NniVAvlacQ+gEA",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookHeadphones,
    #[cfg(feature = "book_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDENggDgLgyEOQwgDeDYbkQMQzC4OIgGMLZSjMMQuDQLgzhOUguDmVg0lYNoTlaYZZDiZxDDmIJFkub5GDIY4dC6IQzg2RwukyWQ2GgMJJjGWYyDEMoNnihaHHMMpem2UZdlEMZ/GOWpinuEwgpeWQ3HqOI6j8PoBA",
        categories = "social,text,communication,gaming",
        tags = "diary,romance,novel,journal,entry,entries,personal,private,secret,crush,like,love,emotion,feminine,girls,teens,teenager,therapy,theraputic,therapist,planner,organizer,organiser,notes,notepad,stationery,sketchbook,writing,written,reading,favorite,favourite,high school",
        contributors = "danielbayley"
    ))]
    BookHeart,
    #[cfg(feature = "book_image")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORjGkchjGwZQgGMeIJDGN5FHmCQ4CIIBygkMo4jqP5BkOOY8gaCAiG2JYNDMLg3C2FwxmQLgxGMLQuDia5tDKbpnDiHBMg8N4NDeVI7gQaA+gE",
        categories = "photography,text,multimedia,files,social,shopping,travel",
        tags = "images,pictures,photos,album,collection,event,magazine,catalog,catalogue,brochure,browse,gallery",
        contributors = "danielbayley"
    ))]
    BookImage,
    #[cfg(feature = "book_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDENAiD4PAvgOBYoiuBoIgoModDgdolEiIQ1GGGoZhiHYdDGHAtDUSIyieKYuigYxpHIYxsGUIBygkMgiCAYx4gmJZUGMeYJDiRgvkqTJOi2BIvgkbYyhkLQ0hibA1l+SICmWB5ng4IAzh8MZwmUPoBA",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,public,secret,unlocked,hidden,revealed,knowledge,learning",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDEMAiD4PAvgOBYoiuBoIgoModhQdg3EiIQ1GGGoZhiHYzhwLQ1EiMonimLooHIZRjHQIB4gkMQyCIIByk4IgxlIdxpGSBYJDiUhoGUaRnGgdIJDWUh5gkNpFC+SZLi2BIvgkTQxDiIBWDSOoZh4MZAgwMB2lGKIqnEPoBA",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,secret,hidden,knowledge",
        contributors = "danielbayley"
    ))]
    BookLock,
    #[cfg(feature = "book_marked")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORwG8bB5GwaRuGUII/kQdBzgkMYdDKDYxjEMwgDeDQ2lCVoZjiOo/kGQ5FD6AQ",
        categories = "text,development,gaming",
        tags = "dictionary,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,saved,later,future,reference,index,code,coding,version control,git,repository",
        contributors = "danielbayley"
    ))]
    BookMarked,
    #[cfg(feature = "book_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDmDQwGgNo4jqPw+gE",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,censor,cancel,forbid,prohibit,ban,uneducated,re-educate,unlearn,downgrade",
        contributors = "danielbayley"
    ))]
    BookMinus,
    #[cfg(feature = "book_open_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMxIDIdgxDUaA3GMMQuDcIAwg0IIXDOHAzFaFQwC0MguDILYXDgLQ0iyLBaCIPg8C+A4FjKNYGggIhtDENodDIIJAkCLQ0jGM44jeBI5gkTQykANhWDMaAtDYY4lieGosh0LorDQIA0hENBjiSF4Zh4LYfDOaIUHaVwzkaNJKD6AQ",
        categories = "text,development,gaming",
        tags = "read,pages,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "schmidt-oliver,karsa-mistmere,ericfennis"
    ))]
    BookOpenCheck,
    #[cfg(feature = "book_open_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMxoDYYQ0CCEgwCCFQxhOEx2DENBhDODYWiEMAtDOJBIDIegiD4PAvgOBYri6BoIgoMoMg4LYQhKFIiC2Oobh2H4fhWF4NiQaA3imK4tgQaIwkyB4JE0NggDgaAyiqLIxk6BZQgqUwxDKVpYkuL4Ck+MxNDGU5VleSpamaXJomoIJgmKbpMD6AQ",
        categories = "text,development",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed",
        contributors = "danielbayley"
    ))]
    BookOpenText,
    #[cfg(feature = "book_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMxoDYYQ0CCEgwCCFQxhOEx2DENBhDODYWiEMAtDOJBIDIegiD4PAvgOBYri6BoIgoMoMg4LYQhKFIiC2Oobh2H4fhWF4NiQaA3imK4tgQaA+gE",
        categories = "text,development,gaming",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,screenplay,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed,blank,plain",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    BookOpen,
    #[cfg(feature = "book_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDmDQwGgNo4jqP4+gSQIJE0MQyCANx2keOY7kwPoBA",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,read,write,author,publish,inform,graduate,re-educate,study,learn,research,knowledge,improve,upgrade,level up",
        contributors = "danielbayley"
    ))]
    BookPlus,
    #[cfg(feature = "book_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDgIA3GgNo4jqP4+gSQIJE2QwxDEaA4kiO5MD6AQA",
        categories = "text,development,gaming",
        tags = "reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "danielbayley"
    ))]
    BookText,
    #[cfg(feature = "book_type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDENggDgVg2EgOInjiOo/j6BJAgkTQxDKIB2DeTI7lCT4FgeUgxjEMxoDSWZOgE",
        categories = "text,design,gaming",
        tags = "thesaurus,synonym,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,typography,fonts,collection",
        contributors = "danielbayley"
    ))]
    BookType,
    #[cfg(feature = "book_up_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMgiD4PAvgOBYliiBoIgoMQ4hkaAyHYMgwEiIQ1GGGoZhiHYdDGHAtDUSI1iSJoriqBIsgkTQxDKDQzFYN5GieSpJgWB4JG0OYNh0MwtDMIJhDOVJIgKSpZCKWwghuX5jmKZZWgE",
        categories = "development",
        tags = "code,coding,version control,git,repository,push,force",
        contributors = ""
    ))]
    BookUp2,
    #[cfg(feature = "book_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCIKDEMoNDMVg3jiOo/j6BJAgkbQ5g2HQzC0MwglYM5JjuTQ+gEA",
        categories = "development",
        tags = "code,coding,version control,git,repository,push",
        contributors = "danielbayley"
    ))]
    BookUp,
    #[cfg(feature = "book_user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORjGkchjGwZQgHKCQyCIIBjHmCQ4kkYx4gkMZIjkL4/kGQ45jyBoIgqFINDMYQzCCY4xjMNocjiOpbD6AQ",
        categories = "account,connectivity,communication,social",
        tags = "person,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking",
        contributors = "danielbayley"
    ))]
    BookUser,
    #[cfg(feature = "book_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWOY8gaCAiG0MQ0hgN40CANY4jqP4+gSQIJG2D4bDeSZJkuO5PD6AQ",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,reading,misinformation,disinformation,misinformed,charlatan,sophistry,false,lies,untruth,propaganda,censored,cancelled,forbidden,prohibited,banned,uneducated,re-education,unlearn",
        contributors = "danielbayley"
    ))]
    BookX,
    #[cfg(feature = "book")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgUPoBA",
        categories = "text,development,gaming",
        tags = "reading,paperback,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Book,
    #[cfg(feature = "bookmark_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIWgiD4PAvgOBYxjSBoIgqDYmhyG4UDSMIyjcPoBA",
        categories = "account",
        tags = "read,finished,complete,clip,marker,tag,task,todo",
        contributors = ""
    ))]
    BookmarkCheck,
    #[cfg(feature = "bookmark_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIdgxDYegiD4PAvgOBY0GwaRuGUIB4DKCQ5CKPgxgkMQ1kMeZACKJpJkWTAwjONY6jwPoBA",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkMinus,
    #[cfg(feature = "bookmark_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIdgxDYegiD4PAvgOBY0GwaRuGUIB5DKCQxDMIggHgMZBDKRB4kAIgxkmPpHCIN4zjWOo8jmO49kuCQ5kqUQxDWRB5l8MJikyJpUC+VhlD6AQ",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkPlus,
    #[cfg(feature = "bookmark_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIWgiD4PAvgOBYxjSBoIgoMQ0C4NQgDePQtj4NYwjKN42gSOIJG0OY9j+TpDkWM5JD6AQ",
        categories = "account",
        tags = "read,clip,marker,tag,cancel,close,delete,remove,clear",
        contributors = ""
    ))]
    BookmarkX,
    #[cfg(feature = "bookmark")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIdgxDYegiD4PAvgOBQ+gEA",
        categories = "account",
        tags = "read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis"
    ))]
    Bookmark,
    #[cfg(feature = "boom_box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAORWDUYQyCCEgwCCFQxhMLQyGgMQyhGE4WiGGISDIdg0CIPg8C+A4FimLIGgiCg4CAOB2DGKIqi+LoEjCCRNh2NI2jiK48juBYHj4MQ2kGN4pkSLQ8HIZRjHQIBoGUaRnGgdIJh0IggHcaRkgWCQyDCXx4mWXx5gkOZfHKaQiDKQ5SlSKRjGkchjGwZQgGOcQ4l8Y5sCIMQ1m+apOniep8neeZ7n2g5docIBymqfpxkqQ6LpAPoBA",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = ""
    ))]
    BoomBox,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA4FYNBIDgIg+DwL4DgWFRyGUYx0CAeIJDQIggGgZRpGcaB0gmDIjHKIAiDKIx3GkZIFisNojHmCYThUL4bh2FYYgaCIKg0MQ0GiMY9kKQYEkOCRNDIMAgkeSYUhaTICk6B5QDENZUDMdpKliTpNgWXIKDmYJileF5lgE",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "ericfennis"
    ))]
    Bot,
    #[cfg(feature = "box_select")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxhDIIIQDAIITDALYQDIIg+DwL4DgWG4egaCIKDEOYNg+EYUioMYphmG4dgQaIgjGB4JE0MosiWKISiuF4RhqHIhjOBY1gqDI4juKoTDGF4XkCMIfgKNIjE2JgzGgMZPkKUpElSJo4liWoxkOIo2DENINmGL5biGRRNmeEQxmqQZjlyZYKDMIA5HaWZrnWbZUjiep8mKUaAjaeZnoSfqGlONqCoqfZ0h+AQA",
        categories = "text,design",
        tags = "selection,square,rectangular,marquee,tool,dashed",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere"
    ))]
    BoxSelect,
    #[cfg(feature = "box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA4GEMgghEMAghQMAtDGGAuDcMxsC0NwtDSEIShWJYXhOHg3CANBBhGE4mCAM4OHaD4ujCFINDGG4diqIo2haJYoiCLI/jCDAgDENhaCIPg8C+A4Fk2UIGggIhtDMLoyioOIbCANYOhsLQ1kyTpTlKBJUgkTQxi4MhWmyZJPmgPoBA",
        categories = "shapes,gaming,development,maths",
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
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgMx2DEOBoC0MwiD4PAvgOBYZhyBoIgoOAgDIMRIDUVoPheGYbgQaA+gEA",
        categories = "development,files",
        tags = "code,token,array,list,square,[,]",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Brackets,
    #[cfg(feature = "brain_circuit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0C4NRhDKEAghMNQgDCGIYC2Dw5DYLQuDQNoVhSFoahkMAtDELg5DgIAziSF4miiG4rDODYPDINIvjyNIZC4M47DWEIuiaM4niSHovC4MJFiWFI+g6LIrDANRMgyFQwFaDw1FoIg+DwL4DgWYJjgaCIKDGIw4FYNRjimK4rDkLYNDKdJfmGZplgSZ4JguDQxDMaA0niYp8nuBYHn6WAxDgaA2hKFZIDGkgyHYMaFnqAp8oqaYNo4OKZoem6JmgTQyDCFA4GGEJQCClAxiqGqtheNKwhiXpgoaZKkn2aQ2hSgaskOtavq+so/sSk7GDCuZ5qOZqdqeqYyDGw6urCyAgrSy6Us2oq8tGpqNhQM7XsW2betuyq2syzq7GgPoBA",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,ericfennis"
    ))]
    BrainCircuit,
    #[cfg(feature = "brain_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCANAuDUYQyi0IIwDUIAwjWNQtiwOQ2C0Lg0DaMoxjON42DALQxC4OQ4CAM5BjSQ5FjiSAzDKKwuDINJMlqUY2C4M5ZDWLZLkOUJEkGO5MC4MA4EGZIxlwIIqDEOYti+QpvmaOg2j6WZujScIwjsLQzmqS5Nk2cJeDQLZhDWY53n+RJHl4Mo5lefaQmaRpIkqg5tpmcIqiwNYahyIokiGI4licIhtDGYQ3nEMI+j2dA0qUL6nGiqYkiaDBtnSVQxoSlZJj6uK6ryq6/sMLpAq8Lg3j2i5JsiqrKr6rQxrOS7BtOtbWqgPK6tmrqwnGhA1rW07hru46quWwaykm67HhuubXu+vasq6s40tAN60tW97Jvqy7aoQMQgsHAg5u0PoBA",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    BrainCog,
    #[cfg(feature = "brain")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4NQgDIQQyg2D4TDAIIWDEIAxDIIA0g0dgxDUYYSg6JIXicMQth4OQ2C4NA0hSJYVigLYSiwLQzC4MA4CAM49ieGAtC4Mw0C0NYNjyJomhaGIakMMoqC4MowkqM5NDELg5DiOIRhOS4oCCDIlFoIg+DwL4DgWZppgaCIKDGHoll2MoOkyKIcnGIIilWdZAh2WYti+MaDnaFo2DaOI6jyPo+oUIJDkWRw1kmXpWhcLZYDOUIelOg5fkymJZlsM5zoSfpwl6ZJmmiBBoD6AQ",
        categories = "medical,science",
        tags = "medical,mind,intellect,cerebral,consciousness,genius,artificial intelligence,ai",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    Brain,
    #[cfg(feature = "briefcase")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHiDQyhEcoTg6Fh5hSEYbCIN4RGgZRpGcaB0g0MQ0CIPg8C+A4FiwcBhgsIBkg0TQxDYIAyDEVg1GEMo7CAMJDkMLQykcaAtDSQJCkST5Hjsdo5iuLYygsPoBA",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goOINgkIoRCAdxpGQdBohCDRoGUaRnGgdIcD4PAvgOBYlHAYYaCAZIKE0NAgDEMBhDIII3DAII6DELQyj4Vg0jaOI7kUMY4j4aJCjeOZGkSDIlC+K4aiqLBoi6MAxjIMo1kyRY6jqTJKkOTZgkgMh2C2S5EmYMI+j4IpRlMaA+gEA",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[cfg(feature = "brush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5C4MA2CAMQxC4OQgDiDQ3C2FoOGEMguDgNQgh2HwgDCEImDSDQzCCKAwDMbIZg2D4aDgIg+DwL4DgWNo5gaCAiE0N4XhCKA5DQYwthINoPDALYqhIMw1k0IAzg0MokhALgzDMLYdiCEg1DKXIhlWWAwDiWAxmMNIUh0MJWDSY5umOVolDSSIeisLYsDQYZUDCaZ+mmJaDk2TZVHqNY3jwPoBA",
        categories = "text,design,tools",
        tags = "draw,paint,color",
        contributors = "ericfennis"
    ))]
    Brush,
    #[cfg(feature = "bug_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA3C4MQzFYNhhDMIIWDAIIZDALQ1hANAtDKEBMDgIAyCIPg8C+A4FimLIGgiCgxDSEAyhcLg4iUMQ2iaKIqi+LoEjCCRNDKNoRGgLQ0HaIRhDQIJPhuGpKkqSQxC4M4+iuQpBgWB5EDIMAuDkNwgDUY4ZiIMQtlePAzjgLZvg0NJakCApCl8IhtjafJpDCdZcneXoxE2D5loYQZPlGGqMjwMQxHYMxhjyPJShmj4Qhab4RoCLaCkOMp9kwOKdGiXagE2jgzEiJ4plunovnkTYWDIMZoiGEAglcN5xmONw4kqpQ+gE",
        categories = "development,animals",
        tags = "debug,code,insect,kill,exterminate,pest control",
        contributors = "danielbayley"
    ))]
    BugOff,
    #[cfg(feature = "bug_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMggDELg4gyEISCIPg8C+A4FheGoGggIhNDENAuDGDgzhGEw2g2FoYh2HIEh6CRNDkIA3iQMx2C0MRhicMAwDMII9j8IAwg+RoqDAdgxiyGYwi+BYHjIMYTjsNAglaRZZC0NJbGiWxhlaWJEkSW5XHYMxhDaJAgmoMZjlmDZXC4NZMi6AowlGIJqDWQA5EOI4qDiEZBjWa5AnSF5Nhud5Qh8TYqDEMxIDKdZOoyMYgkAMgxGMMAtDKa4QDcLYnjSJw4lulaLh2eRNDIMAuDkNwgDWnYNiSOguiqp6knOV6qGiT6YG2JYPg6DA1C2yBasAPoBA",
        categories = "development,animals",
        tags = "debug,code,insect",
        contributors = "danielbayley"
    ))]
    BugPlay,
    #[cfg(feature = "bug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMggDELg4gyEISCIPg8C+A4FheGoGggIhNDENAuDGDgzhGEw2g2FoYh2HIEh6CRNDkIA3iQMx2C0MRhicMAwDMII9j8IAwg+RoqDAdgxiyGYwi+BYHjKJYNDAYwtieQAwC0NgtDILg3luW45DMYQ0CCZpFkUMZnC0NBoDSZZnkSc5rmYNB2DMY5FliXZfCCXIqDaTIugKMJRiCUwykkLQ5oOTqFlCHxNDYLg1kAORDiOKg4hGQY1iSng1o6G6QjGIIqDEMxIDKoxok+phNkAMgxnqfZrhCYInjSJw4m2ravocTaKC4OQ3CANZ6g2JI6C6Kq8lelZnr+pbBDKDqpGivoXk2pIdsEMY2tcNxjl6EJrryD7EkGnQ0r+AQ",
        categories = "development,animals",
        tags = "issue,report,debug,code,insect",
        contributors = "danielbayley"
    ))]
    Bug,
    #[cfg(feature = "building_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgyFYNBhDKDQgDCFQgDGDQtDIaA4hKFIWhaGYTDIdgxDgWgiD4PAvgOBYri6BoIgqDAxDISIRiSF4hhuDR2DaH4TiGF4kGgMoqiyMYwgSMoJE2JwgDmRpBjuGIUiUOZUkMMY9hyG5Ii2TJLgWB5ODGFg2GgNJgkqApMmWCpnhgMJqmyYpumSM5PiINJ1iuYYvniTZxiIOJ+kmYoBA",
        categories = "account,buildings",
        tags = "business,company,enterprise,skyscraper,organisation,organization",
        contributors = "maxim-s-barabash,ericfennis,karsa-mistmere,jguddas"
    ))]
    Building2,
    #[cfg(feature = "building")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINAiCAcoICIMoMHcaRkHQaIJDENoMHIeYJhEIIdhCDBoGUaRnGgdIeDAIg+DwL4DgWLRwGGFggGSCRNDkIAyDIdgtDQaA2HaC4tC+M4WjKNBojaOA4CANhoC4MAxiyLpHGiSY1jcIhNhmT5RlOVZGkqWZLluXQyl+UpUkWV5lkyXAxmkMQwmCbJWmQPJXnCaAgDGQJrmKbp6kqfJenSdqCnme5noegJhm2i6FmeTqIoGkZIoSWpNn6j53mOSIBA",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANggDIIA3CIPg8C+A4FhSF4GgiCgxDCDRoDSE4VhqGYEhuCRtDKEA3C0MgtDGI4WieFByGUYx0CAeYJDMIggHIeIJDKPhoGUaRnGgdIJDENo+HcaRkgWS5NCCQQiiKFAvjaOImgWB4JgsIAxDEaJMjKJYCieX4KDiYg1GgLgwjGWZohqaxNkybpwnKZ40mmXocE2DgxDkdpDnSfp2oEMZtDIMR2i6fYYgE",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANh2DYIg+DwL4DgWEoVgaCIKDENYNg+EYThiF4EhmCRNDIIAxDIaAxDkLoQhKFIkiOBYHiYMYMjgaAzHMLg1C0MQuDcLg4C0MpEGMLgxC0Lg0C4MpMDiT5Ak8IAwkwNJMkuRJMlCQQyGyVJZDUQwyDCSoNkSKYuDGDYpgwNhIDQYYoigMJWlaRggDIdgxDCO4gjKFg8GMaRyGMbBlCAYx4gkNwioseYJjikBygkMqBoWh6JjSJYKDmbxoDWgYioShqIoqjKThCkaTDilaXpmp6cgEA",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzGgLgwDEIg+DwL4DgWFYYgaCIKDENAgDKEIShSFobhqBIcgkbQyCAOYhDALQ1iWF4piiBYHgmC4tDEMhWDYLozhWNYZDwchlGMdAgHcaRkgWCQxDYIggGgZRpGcaB0lAMJTHIeIJDOU5fCINJTHmUAyjSR5JjeKoKi+PR2kKJo2gKKY5h4NQgnGc5EGibZ4E2IAxDcaJRjSJ4BA",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[cfg(feature = "cable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAORhDIIIQDAIITDELQyhcVg1GgNh2DKD4RhSIoWhAMhaCIPg8C+A4FimLIGgiCgzCANRWDOKIqi+LoEjCCRNDeNI2jiK48juBYHj4MQ5CAMY1DYLg1GEM5QCCUw1iKEwwC2QAwHYMQxlKVJWliTJbhQVg5kOOoCjySIKDGQAyDEdoXmqRZskeMRNnKEZznWKZEi2eI9goMoQkoaAth2F4ghKI4RhcaIfiWZAxiGJp2i2AQ",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley"
    ))]
    Cable,
    #[cfg(feature = "cake_slice")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDmDhjHmCw3CIPg8C+A4FgeGhwGEdBoCAZILE0NwuDIIIpDkIAzCAMQxHYORjDAIAuDYLg0jGPQxGgMQ2GOOQgjcMQtjuMQtjMLY1DALQyC0MwtDYLQ3C0OBslKRAyjmGYbiGI4giKJImCITZBjEMxIDOXwvmEaJjiOJYnmkMQ3mybpwD6AQA",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = ""
    ))]
    CakeSlice,
    #[cfg(feature = "cake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAyDEdgtDgYQyg4IINhgLQyhoSA2hSFoYheGoOHYOAiD4PAvgOBYoiuBoIgoNAgDENhzC4NQtDGDo5g6N4WjKFQyjeIw0iOQg1j+FobjqS4nimLotgSL4JguDgxGiDJOiqUpRgWB5UDcIA4HYM5alCApSl+CgxhWY5liiW4smiXowE0MZhm6ZpcnOU4KmENBoDALgwDGepyi6ap2hWgKCoShhol2fZ2n+gaDoWcJQgEA",
        categories = "food-beverage,social,account",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,fondant,icing sugar,sweet,baking",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Cake,
    #[cfg(feature = "calculator")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDYIggGgZRpGcaB0g0MgwhEeINDSERyh0IgyhEeYZCIPg8C+A4FikbBpG4ZQgHkMYNhAIB4jUIg4hwMoNg+JY+CKEIpC+L4xi6MIyjSP4fjiOpAjOQgxjyOJTkSKpHGWKRwGGCwgGSDRNg8IAxDAaAuDAMYoiqXYLlyXhomCYgxDKZZnmma5Fm4aJwl+YQiE0OJ3miapsC+fJ+nKgJjnYMQ0oWeptnGipzoGg6PpGh6JDyfKWo2ZQ4pqe6Up2cafpiop5pulIBA",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[cfg(feature = "calendar_check_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDQVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2g+IIWiOIQyGgOAiD4PAvgOBYxGwaRuGUIB5DKCQ2CIIB4j0Igxj+OwxgkMpAHiSJEj+MQvjeOY2jiOo8j6QB5k2SpBk2L5BkOL5QlIZZUjmR4JDEMJZkOapLkODJLk0M4wjKZIxjSBoICIbZFhWF4lCANAtDSdYzgQaA+gE",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[cfg(feature = "calendar_check")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODSEhyHiDgyhKGQiDOEh3GkZB0GiDoQheFAihsPg8C+A4FiwbBpG4ZQgHgMomDaHAxjmEh5jyKo+jgIo6iwL4yjSMYzjUeZDjqNpDhGNpAlKP4aCKRpIGWSo0lODoflCGgxj6QAxDCQommeWZLiwcBhiQIBkg4bQ5CAMQ2CAMp5CANAthaRpuiQPoB",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[cfg(feature = "calendar_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3C4NRWDYYQyCCFQwCCGAwC0MocEgNYUhaGYjhuFQyHYMQ0iGF4kiIMhoDOEAiD4PAvgOBY0jeBoIgoMQ2hYdg0jONY6jmBI7gkTQ4kCQo0jaR5GgWB5JDMIAxDAaA1kOT44gKR5Tj2Dw1laYpWj+PguDKEYpluRZelKPILhWPhhj+P4Yg0MQtDGLJ2i2eYXFqbZQgEA",
        categories = "time",
        tags = "date,time,event,clock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarClock,
    #[cfg(feature = "calendar_days")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAcoICIMoMHiCQzgwcoThCDB3GkZB0GiCQxDiDBoGUaRnGgdIgiIPg8C+A4FiwbBpG4ZQgHgMogDaDB5DGCYRjaPQiDGOoHjgIo6iwL4yjSMYzjUeZGkQeJBiKNpGlWPI+CKSZLGWTY0kCFISkYMgxjuQQxDCO5GmmW4tl2LBwGGHggGSCRNDgIAxDQaAuDCZpJnKHpxnMaJ1ncMQynqfJ+oCLaCGihJ0nYIhNkOi59n+bgvpCkqGpSeJ6DimaOpyhaeoelaJqKpKbp0PKQqmlg2qyjauqeAQA",
        categories = "time",
        tags = "date,time,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[cfg(feature = "calendar_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDAVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENBjhcMQuiwOYhiUaA3CIPg8C+A4FjWOIGgiCgxDaFR2DSNI2juOoEjyCRNDiQZDjWN5IkeBYHkoM4ODAaAxDiRJQjmApIlSCoMC4MovigLg3hMLg0lYMprlaGYYhsLg2DULQuDUMhjncM4thyeA3C4M53DieAzGyfA0oINJ8nai5qmyFZviOcpunWdw1DOe6Cn6mA2oKdw3Dmhp3DkNQuDmjINm6mZkCCfQ3DQTAxoENYVDIbJ9DYLZ9DUNRjiyHLBg2LIonOhQxDmvJoDQWpckaAQA",
        categories = "time",
        tags = "date,time,event,heart,favourite,subscribe",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[cfg(feature = "calendar_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDMVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENIghaI4hDIaA4CIPg8C+A4FjIbBpG4ZQgHgMoJDENgijwMY/kEIB5kQIgykIeY+CKQYyC+OI6jeOY7j2CYwkOWZMk6RpIgmS5RlMZZVjqWwiDOQpYkoMZMkkMQwl2P5ymOVpmleTgykuR5wDmcwiDGf5okCMYzmQPoBA",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarMinus,
    #[cfg(feature = "calendar_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4MQ4CCDIOEEMgghQMAgheFwzCANh2DENBhhSFoYiSIhoh+IYViSGQgDELg4DILYug4Ig+DwL4DgWNo5gaCIKDIMYtDULg1FYNopiOGQtjEMhIDmRI1jePI7gSPYJE0MQ2hUdg0lGOJVlSBYHleGwxDAaA3l6U4ClWY4/kGZhoC2Qw1mqYA8GwaRuGUIB5DGCQyCIIB4n8IqBoMMqAoceaJoagY2C+eZ7D6AQA",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDMVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENIghaI4hDIaA4CIPg8C+A4FjIbBpG4ZQgHkMoJDYIggHgMYJDGQI8kQIgykEeI+CKRoxjOOI6jeOY7kOCYwkKTpaHmSZLjyTpAjIL5TGWVY6kKSQzkGPZFDCTJOgybZJDGcJkmaaI7m6SpgliTw5nGRaBkiRZjlKVp6luCQymCfAxoSf5QoWgJRmWiYBA",
        categories = "time",
        tags = "date,time,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAaBlGkZxoHSCQxDiDB4gkM4MHcaRkHQaIUhYIByggIgygwcoYiYIg+DwL4DgWLRsGkbhlCAeAyhQNoMHkMYJieN4+CIMY7geOQijuLQvjONYyjSNh4kKIo4gmIo9j+PJHkmLpMGWTo1geQgxDCF5ChqN5HDIMZZhSZJKl2LRwGGHwgGSCRNDENwgDENBoC2WwvnKH5xnMaJ1ncMQznsOBIDeLIuoIaKEnSdgiE2ep8GgLgwmuSqRpOhqVnimA4pqnKPoGhQ+gE",
        categories = "time",
        tags = "date,time,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[cfg(feature = "calendar_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENBjhcMQuiwOYhiUaA3C4NQiD4PAvgOBY3jqBoIgoMQ2hUdg0jaOI9jyBI+gkTQ4kORY3jmSpJgWB5MDODgwGgMQ4kaUo7gKSpWkCToMGGWJYiuIwtkKaIjm8Nh2DAWpekiYZVj8bQyiWHIsDULZ+nWU4B",
        categories = "time",
        tags = "date,time,search,events",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarSearch,
    #[cfg(feature = "calendar_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDMVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENIghaI4hDIaA4CIPg8C+A4FjIbBpG4ZQgHgMYJDENgiCAeY+CIMpCHgMo/kGQ5KCKQYyC+OI6jeOY7j2CYwkORZHk2CZMkmWYxjOUxllWOo8kUM5Ik6DJCkSPwwm+TgxnKUZlmeV5FDEN5vnufZekaXZhoKY5SlaeZbgkMpdHmdKAlgIp8myi5HneiIBA",
        categories = "time",
        tags = "date,time,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[cfg(feature = "calendar_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEeYNDSERoGUaRnGgdINg+ERyhMIgyiOF4mCIPg8C+A4FiwbBpG4ZQgHkMYNieNgyg0NoRHiPIOj6Eo4kKK4tjKNIxjONY3jmP5FhCO49j+QYQiwL5JGWS40kSFIWkEMQwhaRZilWOQxkeWZMlyTZlhmXoOmOU4OlKQIhhmWJam2cQxnAeZhnaYZzk6Dp5kibIBA",
        categories = "time",
        tags = "date,time,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAdxpGQdBogkMQ4gweIJDODByhcIgyhqCIdgwaBlGkZxoHSE4VD4PAvgOBYrGwaRuGWBwygkNoWDGE44CAeI2CIMY8HmOohisL4xjOMIyjQeY/kKRIej2RIVj2P4qiySBlkqM5VgkMgxjmGIMk2EwwmORAxmaRpZD6AQ",
        categories = "time",
        tags = "date,birthdate,birthday,time,event",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2B4RguDh4gmC4ND4PAvgOBYcHAYR0GgIBkgoTQ3CANxIDQYQyCCMAwCCMwwC2MAyHYOYvjGNI+jOOBoDENgihwL4iiSIYjiWJwiE0OQuDUIA0GgNRMDGKg3GgM48jKPgxj2OQ3lGRYdkgaJKiSJooDENAuDEMpgDENZvnEQQzCCeIznIIJQDgOAgDEMAun+ZZHksPoBA",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CANBoC0NRMDcIA3EgNBhDIIIcDAIIfDALYcDIdg5huHYgiqH4kGgMQ2iiHorh2IxWieJIziIMojhIMxsiODgtDMegiD4PAvgOBZGGMaRyGMbBlCAYx5gkMQzCIIBygmV5SHiVQykWR5Mk6UA+gE",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[cfg(feature = "candlestick_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CANR2DQIg+DwL4DgWEhyGUYx0CAdxpGSBYJhAIByHiCQxCIIBoGUaRnGgdIJDaKIlCIN4oHmCQ5hGE4YhqEoVgaCIKgwMYODKOoUgQaI+kmB4JE0MQ3CAMx2kaEpIhYPI8huKosi6CQ4jKJg1jaCZjhyHogCKIokiaR5akuBZNgqUAgDGUwzkeP5wkCTgzlIdgxDgaKBnmSQ+gE",
        categories = "charts,money",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,exchange,hedge fund,statistics,diagram,graph",
        contributors = ""
    ))]
    CandlestickChart,
    #[cfg(feature = "candy_cane")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NwgDIMRhDKDwgDCFQgDELQzC4NQtDIbA4C4NgtDENBhDYIIohaFgxhgMAuDSKYUhOLIYhqMA2DSHozheLQwjcNI5h4WgiD4PAvgOBZGkmBoIgoMQ3g0NQgg4MZTDILgxkWR5MkuBJNgkTQxi8OQgDQLg4hgMwgDmW5Il+XoFgeCRtlGZQ5g2FJnDSbpdgKX5zgqZ5liWeZViGbZGm+SoBA",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = ""
    ))]
    CandyCane,
    #[cfg(feature = "candy_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4C4NQggwNQtDEIAxGENAuDmDoXhkIAwh2Hw3CANxsDGEgiD4PAvgOBYoiuBoICITQxDELg4DQMwgDYLgxDgNxBhsNIhkCIYeh6EwxjqDg3g2FoYkEIJDh+RoUC4MwxjiFw2DUN4nimLotgSL4JjINIUkkVgxDSXYqmGYIFgeY5pjmDR2jSNgzmuX4CmGcIxDGRgwHaSw1nmbZ7m+MBtkiIoUC2Do0DMNpLDYOAzEGOI4kWHwxDkLg3DAOAgDMSAyDGdQuDIOQyGGmJSh+VQxiGkA0DETAyDKIhso6FKFiyh5igqoaxhIIIRpCkgtC6lKWq2moeheqa4qUSAzHaEqoqqrKiq4MLJlYN7XDOtK2hSI7Gr0aIoGwaRuGUIB5DKCa3CK7wxvK9B4vYIgyvi8b7vyKAvuu7Q+gEA",
        categories = "food-beverage",
        tags = "sugar free,food,sweet,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CandyOff,
    #[cfg(feature = "candy")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5C4NQgDeDQtDIIAyGENAuDmDoXhkIAwCAMYdg+DxsDKEoWhiGooiGIAwC0N4uFoIg+DwL4DgWM42gaCAiE0MQ0CANoNHYMQwjKNI5jiBI6gmPYehANZDkWM41kqSYFgeCRtDENoiDELYODELgzhANg4EEMwgmiHprh+DA3mkSAyDGQ5iGOHguDSQZhDkMpihOYQzDMTAyhMNxsl+H5GlSN4CkqWIKDiH4viANQtoCLwumWZ5piGbIXmichIDMdqWmIYZomqnQtn6paBoOkhspUMaKkiAQ",
        categories = "food-beverage",
        tags = "sugar,food,sweet",
        contributors = "karsa-mistmere"
    ))]
    Candy,
    #[cfg(feature = "car_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIA4C0MggDILQxC4NQtDMLg3EGEYRDAIIfh8MQ1C4Ng0DYIA1EgOAuDQYYdiCMQwhQLg5DAMwghUMg1DcTA1jmH44DgIg+DwL4DgWRZIgaCAiE0N45DQaAuDAMZEkaS5KgSTIJE0MZQDGUpUlaRZHluRRyGUYx0CAeIJDMIggHmCQxDCcR3GkZIFnSQwgGgZRpGcaB0gmfRym4IgylcL5pmuWoFgeXY/DEOB2oqZZZgKW6Rk4MQ5jmlaXliZ4BA",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyGgNAiD4PAvgOBYThaBoICIbQyDEIA4C0MoOC0MQuDULQzC4NxBiOI4NjAIAxDULg2DQNggDUSA4C4NBhi4IIxDCJQuDkMAzjILgyDUNxMDWMoNkgOIShSGYYgSGoJE0N4yDQaAuDAMZUhWWJXgWB5aDGXAxl6YJihOZIXDwchlGMdAgHIeIJDIIggHcaRkgWCQxlMIBoGUaRnGgdIJoUeaDDCfZ6CIM5jnSdpmlmCpPoQdp8nCVoCliaIKDEOYyDinpjlaAQA",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDcaAyGMLg2CAMIOC0Lg0hgMR2C0MxjDCGQ5hkNwtDELomigNYnC4ORDDEOIpg4MIUg6FQxhcMY4DAc4tDOLQ0C0MguDKQwuiCGYshqLQxiWLQ4iUSA1GOGYViKKIokKWouGyQQgkQORBDOM5kDeFpohcMoODIdg0iEIIUhqDp0DGEQiD4PAvgOBZ5GMaRyGMbBlCAYx5gmDwioUeIJDeihygkMp4nqf6BoOeZ8gaCIKg2DxoDak57gQaJ+oCgqEoaiKOCCkAipKi6qqGlanD6AQ",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[cfg(feature = "caravan")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIOQiCAdxpGQdBogkNIMGgZRpGcaB0hODB4gkMgiD4PAvgOBYiiWBoIgqDIOhCEgihQIIWhiGoJDEMIdjaOIiiSBB0iIcBhhEIBkgkTQxDgIAxDkVg5GENAglAMAglMMAtDSVxIDaT5RlSXpWlANB2DgYQyCCZpVl6ZgyGiII8kGEYiGMaRyGMbBlCAY4qkuDBjh4Ig4gwcofiGI5znWd5AkIaJEkaN5KDkaAxDIdgtm6I5wGgPoB",
        categories = "transportation,travel,nature",
        tags = "trailer,tow,camping,campsite,mobile home,holiday,nomadic,wilderness,outdoors",
        contributors = "danielbayley"
    ))]
    Caravan,
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gwN4QHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCAOR2DaGYbiGI4ah2BoIhYIoYhGDAxhSCYLg2D4ahyBI1iCIokiYIooDQIA3HYOIvC+MRoD6AQ",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ1CANAtDiEAgDgIg+DwL4DgWGIbgaCAiE0NINDMaA2heGYehgYxpHIYxsGUIBygkMwiCAYx5gkMQyjYYx4jqFoYC+LIujCHYEh+CRNDIMQgDkdonkKKoB",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseSensitive,
    #[cfg(feature = "case_upper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ1CANAtDiEAgDgIg+DwL4DgWGIbgaCAiE0NINDMaA2heGYeh2BIfgkTYOg0MRoDQLg1GEMggjgMAgjsMY8hASIOFYN4zjeOY/j2Pw0iiGosD6AQ",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    CaseUpper,
    #[cfg(feature = "cassette_tape")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAeIODKEh3GkZB0GiFQwhIcoUCKFggHmDg0CIPg8C+A4FikYxpHIYxsGUIByhWEhjiWD4eCAY4hDiKIqi+MYzikcBhhsIBkg4TQ4CAMQyGiQIpC+R4bi6MIyjSOYODGPI2iKOIhhCQQvkOWpGkgaJKg4bQ2CAMgwCALg3C0MguDkQQxC4NJPnwIJynIMQgDgLqDhAaA3C4OBhnufaOoCkaHn8MRsnQIAzmWVhoD6AQ",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = ""
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBWDYYYMgwMAghQMQgDILQyGgMYQhKFYgheEh2DEMoRhiIIWhqGBoC0NgiD4PAvgOBYxjSBoIgqDIlGEOQgj6FIWg2DYwjKN42gSOIJgsIIdGENQglCQZNCANJVkWM5JjEbBpG4ZQgHkMYJDIMAiCAeAymMLgwDGZh5mkIpkmYeJinGWJcl4PoBA",
        categories = "devices,connectivity",
        tags = "chromecast,airplay,screen",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cast,
    #[cfg(feature = "castle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAyDAdgtDkSAyHYORhg2DQwCCG4bhkaAxDaGIOhyJYeC0MhaCIPg8C+A4FiyL4GgiCgxDgIAxDEVg0EgNh2DeK4tjKMYEjOCRNDENYOhULQ0GEMwglCHYcC0M5VHYMJPlGJpUlAMx2DSQYukWRIFgeR4MjiOg5mKQ4CkWZ4Kg2ORWmyLJjjCb5mjQTQ2CANBWDKbZknqRo1jegKCneboynGSIbomg55o2fAxDSf6BpIaA+gEA",
        categories = "buildings,gaming,maps",
        tags = "fortress,stronghold,palace,chateau,building",
        contributors = "karsa-mistmere"
    ))]
    Castle,
    #[cfg(feature = "cat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GMLg2DcIAwCAMQuDMNQuDAOQgg0LgyDaFguDcOAtg2GgwDOJwuDgNAgDYLg0DKLIhiMNAuDWJoyg0NwtjwIITjmE4XDCRIeiCLwxCAM4yDQQwyksMQ3C6HQxjEOYTlGFoNlEcwtDkLYrmGVA1DYYwwC2FwyhoNYskqYpOhSFJqi2YYxjOP5ukSGJhjkOAgjgNwyiAM4wjmSAyDMQQ5huL6NDCL4VhWUooFoIg+DwL4DgWmacgaCIKoAMQ0HaOaYpqn6egSoIJguIqkqYNaopurKrgWB6uDGaw1haMZsGiFw1EyDIWDcbI/DebokDWl6ZrWnYBA",
        categories = "animals",
        tags = "animal,pet,kitten,feline",
        contributors = "kemie,karsa-mistmere"
    ))]
    Cat,
    #[cfg(feature = "check_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2CANwgDENxsC0NYWCIPg8C+A4FhqHYGggIhtDIMoSDALQ3C4NYQisTAxDOEg2hmG4gD6AQ",
        categories = "notifications",
        tags = "done,received,double,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    CheckCheck,
    #[cfg(feature = "check_circle_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQ5CCDggDKLQgDQLQ0CKGwviKJA+gEA",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis"
    ))]
    CheckCircle2,
    #[cfg(feature = "check_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDELgwDgVgxDIYQxDCDoZhkMYOC0NQuDkMwtDkLgxDQIg+DwL4DgWKotgaCAiG0OYOh0MwgDMTIMCCKIqiyBBoD6AQ",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckCircle,
    #[cfg(feature = "check_square_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKERoGUaRnGgdINg+EYUCIM4RHmDYjD4PAvgOBYoHAYYLCAZING0OQgDEMggjiOA0C0NAiigL4ugsPoBA",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = ""
    ))]
    CheckSquare2,
    #[cfg(feature = "check_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQxCAM4QEwMgyCANAiD4PAvgOBYZhyBoICITQyg8MQyHYNxhhWFQwCCLQxC2KxIDWKggiyLoNjGMRWjSK44i+NoxGiDoYhqHw+gEA",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckSquare,
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
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQzC4OA3EENAghQMAghcMQgDcLg0hoNhhDULgxhqIokhiKIaDELgwDULYrDWFA1CCM4XhmG4sDiGBBiaJYjhqNoNg0NguDUOQggyFIWimDY6g6EA3FYMgxEgNhaCIPg8C+A4FlkbBpG4ZQgHgMYJDYIggHmZQiDEN5oHgMoJDEOJoHmcZsm6WQvl+YQ+gE",
        categories = "food-beverage",
        tags = "cooking,food,kitchen,restaurant",
        contributors = "karsa-mistmere,ericfennis"
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
    #[cfg(feature = "chevron_down_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQxDYIINC0NAgDSLouCKGwviKJA+gEA",
        categories = "arrows,navigation,shapes",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownCircle,
    #[cfg(feature = "chevron_down_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NHcaRkHQaIKDEOINgkIoRCAaBlGkZxoHSGIaD4PAvgOBYoHAYYWCAZIKG0MQ2CAMQwC0NAgDSOo6CKKAvi6Fg+gEA",
        categories = "arrows,navigation,shapes",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownSquare,
    #[cfg(feature = "chevron_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAOQggyEAtDYIg+DwL4DgUPoBA",
        categories = "arrows,navigation,gaming",
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
    #[cfg(feature = "chevron_left_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ0CAMQ2C0NIuCCLw0hmG4hiMPoBA",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[cfg(feature = "chevron_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NGgZRpGcaB0goMQ4g0dxpGQdBohmGwgHmCoRD4PAvgOBYoHAYYgCAZIKG0MQ0CAMQ2C0NI6CCOw0CKKAvi6IA+gEA",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,panel,button,keyboard,<",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronLeftSquare,
    #[cfg(feature = "chevron_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDgLQ2hEIISDYIg+DwL4DgUPoBA",
        categories = "arrows,navigation",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "colebemis"
    ))]
    ChevronLeft,
    #[cfg(feature = "chevron_right_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbYUCAOAgDSLQti4NAihsL4iiQPoBA",
        categories = "arrows,navigation,shapes",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    ChevronRightCircle,
    #[cfg(feature = "chevron_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NHcaRkHQaIKDEOINGgZRpGcaB0hiGggHmCoRD4PAvgOBYoHAYYWCAZIKG0MQwCAOAgDSOQtjoNAiigL4uhYPoBA",
        categories = "arrows,navigation,shapes,development",
        tags = "forward,next,more than,greater,menu,panel,code,coding,command line,terminal,prompt,shell,console,>",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronRightSquare,
    #[cfg(feature = "chevron_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQ4CANgthGEwiD4PAvgOBQ+gE",
        categories = "arrows,navigation,maths,development",
        tags = "forward,next,more than,greater,menu,code,coding,command line,terminal,prompt,shell,>",
        contributors = "colebemis"
    ))]
    ChevronRight,
    #[cfg(feature = "chevron_up_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQ4CAMQ0CANAti2MQihsL4iiQPoBA",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[cfg(feature = "chevron_up_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKG0OAgDENAgDQLY4jwIooC+LoSD6AQ",
        categories = "arrows,navigation,maths,shapes",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronUpSquare,
    #[cfg(feature = "chevron_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDULQ2hGEQgDYIg+DwL4DgUPoBA",
        categories = "arrows,navigation,maths,gaming",
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
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slower",
        contributors = "colebemis"
    ))]
    ChevronsDown,
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
        categories = "arrows,navigation,gaming",
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
        categories = "arrows,navigation,gaming",
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
        categories = "arrows,navigation,gaming",
        tags = "forward,ahead,faster,speed,boost",
        contributors = "colebemis"
    ))]
    ChevronsUp,
    #[cfg(feature = "chrome")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCBjhKDYPCCFgiDSEIKhOD4ahyBIihobBpG6CB4DGDAyDELgxDeEB4DKLoQHmOgiDiRZDkiGYbjWN40jaOJLDgLg1isIJGgwNguDANpBkcMwuDkNZKhOK4wk8ZZRjeWZLDGWI5hcLg4kmWZHjyY5xm8NQuDSX5plIPoB",
        categories = "brands",
        tags = "browser,logo",
        contributors = "colebemis,ericfennis"
    ))]
    Chrome,
    #[cfg(feature = "church")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIA3CANAgDIdgxDEYQyhIIAwhoIAxC2GAyEgNIXhmG4bh4MofFYORsDSHwiD4PAvgOBYxjSBoICITQxhEMoTC2I4ghyJofh8dgwiSGImhqH4SHYNIwjKN42gSOIJjuDY+FYNRsC0NgtDOXggDOFA3lGM5VlSBYHlcMYYDcdg1meU4ClWbI6DGGw5GiUIxmiNYBA",
        categories = "buildings,maps",
        tags = "temple,building",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Church,
    #[cfg(feature = "cigarette_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2D4JguEx5hGCwiD4PAvgOBYcHAYR0GgIBkgoTQxDIIIqEgMh2DQaAxDSG4diKJIhiOJYnCITYMiyL40hwL43GiOYkiaKAxDiQBoC0Lg1jWRI6keO4oDeQIwlKRZVkmPZLCAOBjDALQylCZZlmcMgtlGQ5cDyRZej6K5imSZg1mid5om2NpUgE",
        categories = "travel,transportation,medical",
        tags = "smoking,no-smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CigaretteOff,
    #[cfg(feature = "cigarette")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDISAyHYNBoDENgiD4PAvgOBYZhyBoIgoMgyg6Eg0hiGofh6BIggkTQ3iWE4ohuLIrgWB4ugwIA4GMMAtDILg1j+P5BkMNYziqAosjiIokjyPpAkIMpElILZHhmNIdgE",
        categories = "travel,transportation,medical",
        tags = "smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Cigarette,
    #[cfg(feature = "circle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgYQ5C4OQzCCFYXCAMIcCCDwzC4OIcCIPg8C+A4FiaKYGgiCgxDcLg2CCIQ3DGFIWDWGY5h6HYPhENg5hALg3iWJ4siuBItgkTQyDGIgyh+DY3hqGJVj2H4eiEOJGiiSpJgWB5MDKDQykKMIyjgOY6hqOodj4LYRDeQ5Bl2SICkqYoviGQpOlCag0jsOaBm+HwtluJIml6Kp4mGLhNDYLqBmQLpmmqbI8oUMZxjIOackWip3iyepNhKH58mqVoWhimocoeIp2l+jZLgqNYPpENKXoKboej+RKcnWoZfgE",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDashed,
    #[cfg(feature = "circle_dollar_sign")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ2CAOBoC0NhhDIIIxDAIAxCCNI0DQaA0jCMo3jWQI5EgOIZhuIYjiCIokiYIoojEMQ4FYNpFC+RxoD6AQA",
        categories = "shapes,money,currency",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[cfg(feature = "circle_dot_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgYQ5C4OQzCCFYXCAMIcCCDwzC4OIcCIPg8C+A4FiaKYGgiCgxDcLg2CCIQ3DGFIWDWGY5h6HYPhENg5hALg3iWJ4siuBItgkTQyDGIgyh+DY3hqGJVj2H4eiEOJGiiSpJgWB5MDKDQykKMIyjgOY6hqOodj4LYRDeQ5Bl2SICkqYoviGQpOlCag0jsOaBm+HwtluJIml6Kp4mGLhNDYLqBmQLpmmqbI8oUMZxjIOackWip3iyepNhKH58mqVoWhimocoeIp2l+jZLgqNYPpENKXoKboej+RKcnWoayGMaRyGMbBlCAYx4gkMQyCKyh5s2zwgHKzZdsSxrID6AQA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCBjhKDYPgmC4mhWE4ZhuIYfgEA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ3CCDhoC4MAxhmG4hiOIIiiSJgiigMorDKLYvjEL4zGiNYjiWJ4qiyLowhqQY2D6AQ",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwGiDgiD4PAvgOBYThaBoIgqDAxDSEAwhKFIZhMYxpHIYxsGUIBygmEQgGMeIuDIIowHmM4iC+JooioPoBA",
        categories = "maths,shapes",
        tags = "calculate,shape,=",
        contributors = "danielbayley"
    ))]
    CircleEqual,
    #[cfg(feature = "circle_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNDgLgzDWDQuDYORBDGEIoCCEIpg0MYgCAMQ1iQNYThWGoZgSG4JE0MQ5C4MA4jGP5BieKYsjGSQ0C4OYMkuTY2haOg+gEA",
        categories = "shapes",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure",
        contributors = "danielbayley"
    ))]
    CircleOff,
    #[cfg(feature = "circle_slash_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQyDIIIri2GociKJA+gE",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig5CIIB5gkIgxDWDR5DKCoMgeFoRhMPg8C+A4Fh0YxpHIYxsgYYx4goMQyg0Yx5iuLQgHKKwwCKHQviOJYnD6AQ",
        categories = "shapes,development,maths",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[cfg(feature = "circle_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDAYQ2CCEgwCCFQwC0MQyhYIg+DwL4DgWHhjGkchjGwZQgGMeIJhoIoqHmLQwi8coJDSHYfiSJooiOJYnikY4xCKLggjWQ4ziqLJDDKOAvjqPw+gEA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUserRound,
    #[cfg(feature = "circle_user")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCBjhKDYYCCFgiDOEIKhOD4ahyBIihocBhHQaAgGSDBNDcIAyDALg2DYMhWDEORhDKPggDCSwgDGPgtDIaA2kiSpMkyT5JDIdgxkGQ4ZhuNY3D6AQA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUser,
    #[cfg(feature = "circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWBw+gEA",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[cfg(feature = "circuit_board")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEch4g0MoRGgZRpGcaB0g2D4RhgIoVD4PAvgOBYoHAYYLCAZINE0MQxCAORoDQYQyCCPAwCCP4/DILQyFaJ4pi6C4oGMaRyGMbBlCAY4TCIOYRGOJZWCAcoZCKKAvkyTpQi2LxojGMw3j0MR2C2Oo8j6QAgjaQwyjmXpImWS5Nk+UZcCKGpSiUMQ1leVKDneYJ7mOAQ",
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
        svg = "gAPBwGEdBoCAZA9CITQyDALgyCANggDMIAxDEbAtC4OQtDILg0GOGAzC0MQuiOIYcg+JYbC4NRsDEM4rC2HojDGIAgicLoThwNYUjgWgiD4PAvgOBZAkOBoICIbQ2g4IA1jiEoklAOY/kGRpFgSR4JG0MYcDSUJei8MQgDSVJCliV4FgeCRNhOFRoDEOB2DgYYPg8MAgneNJ1EgNZ0jaeKAnqG4+kCZpEgE",
        categories = "multimedia",
        tags = "movie,film,video,camera,cinema,cut,action,television,tv,show,entertainment",
        contributors = "it-is-not,ericfennis,danielbayley"
    ))]
    Clapperboard,
    #[cfg(feature = "clipboard_check")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAch5gkMYMHcaRkHQaIJguDYICKEQgGgZRpGcaB0gkNIMg8IgyCIPg8C+A4FiwcBhhYIBkgkTQxDYIA0GgMhhDIIJADAIJDDGQZBHYMQ0j+R5DkULZADISA2kyQpECAMZQlAVpUlGV5FkGUI9iuLYyhaMYzGiNYJG0OZYDSR5ADQLYmiwL5mGgPoBA",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAch5gkMYMHcaRkHQaIJgsIIPCIMoMHKCAihEIBoGUaRnGgdIJDQIg+DwL4DgWLRwGGFggGSCRNDgIA0EgNhhDIIJADAIJDDALZADIdgxDSP5BkST5DkgaAxDKTZClCQZHHaR4si6M4WjKNBojaOAxDaOxolWSJYDGTpJiuLQvl8aJhjWNwiE0MptksSAxhGcZznWY53G0MQ1CAMZGDSO6MnCXpiD6AQ",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_edit")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAch4gkMYMHcaRkHQaIJDiDBoGUaRnGgdIJDSDIPCKGYNggIoRD4PAvgOBYrHAYYWCAZIJE0MQwC4NAyCAMQyC4NgxGGPwxCCRAgDCPZKj8OQ3kYLpNEwN5QDWRpFDSRgyGyUA5C0M5UCANY6DMLZiDQNBaCKKwvjGFowjIaI0jYMQ2CANBoDKQ5GkifJFjwMh2DENJ6jySZJDELZ/GiZQuDWaosm0aJvjONQiE2WAxl8NRWDahJ8oeRqJnij5snAPoB",
        categories = "text",
        tags = "edit,paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardEdit,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDSEB4g2DwgHKGwiDGEByHmDYiCCJQiDIIg+DwL4DgWLRwGGCwgGSDRNDENggDQaAyGEMggkEMAgkQMZCkIdgxDSQJIkSRgtkEMhIDaTZDkUIAxlGURWlWUpYkaQpRj6LIujOC4yjQaI2jgMZBDEMRohmLQvmcaJpjWNwijmbw2nKZZ1mqeJrnoTQ4lmcQuDCIp0nag5snuh46GiiqMmagoBA",
        categories = "text",
        tags = "copy,paste,tasks",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardList,
    #[cfg(feature = "clipboard_paste")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgORhDEIITDAIIWDALYTDEdgyGOFguDYLg0hSJQxGgNhjiGF4UC2I4tDEVgzh+Lg2i4NIajkMRaCIPg8C+A4Fj6QYGgiCg4CANBIDYYQyg6LIYC2TgyHYMQ0k2T4YiyUxoDEMpYk6WoWDKUoLDaSRol+U5QiWU4dguGw0l0MI9j+RJDgSRYJG0MQ3hSFokjigZ1kCeQ+gE",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "xnousnow,ericfennis"
    ))]
    ClipboardPaste,
    #[cfg(feature = "clipboard_signature")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAch4gkMYMHcaRkHQaIJDiDByggIoRCAaBlGkZxoHSCQ0gyDwihkPg8C+A4FiwcBhhYIBkgkTQ4CANBIDYYQyCCPwwCCQgwC2PwyHYMQ0j6QJDk6QpHGgMQykyQZPkCRh2C0Lg1CKLAvjKFoxjMaI1jcMQ2joaJUkeVwxCAMQuDcM5wl6LZhGiY40jYIhNDEOAuDSPw5C4Ngxj4LpvDKiZOm+iguDkN5ApANxMmikA1nCkgxnSfxspAOQtDOmI6oEMwtDSgQ0FqdpgmSeplnyOJwDiUqtngPoBA",
        categories = "text,account",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardSignature,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCKBx5gqDAgHcaRkHQaIKDiDRoGUaRnGgdIKDSDYJCKGQgg8IgyCIPg8C+A4FiwcBhhYIBkgoTQxDYIA0GgMhhDIIJADAIJDDGQZBHYMQ0j+R5DkULZADISA2kyQpECAMZQlAVpUlGV5FkGUI9iuLYyhaMYzGiNY3DmWAyHYLQxGgNpJmQL5mGiaI0jYIo4kYMQ3mOLJ3mmepqnyOJADEMR2Dadp4D6AQ",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAdxpGQdBogmCwgHKCAiDGDByHmCYZCAaBlGkZxoHSCQ0gyHAiDIIg+DwL4DgWLRwGGEQgGSCRNDENggDQaAyGEMggkEMAgkQMZCkIdgxDSQJIkSRgtkEMhIDaTZDkUIAxlGURWlWUpYkaQpRj6LIujOEYyjQaI2gkbQxDWWZajsNplC+ZxommNY3CIbQ5nEIJznWdw+gEA",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDSEByHiDQxhCHIOhoeYdhCIwiDIIg+DwL4DgWKhwGGCwgGSDRNDENggDQaAyGEMggj4MAgkEMY/j8dgxDSPZFkGQwtj4MhIDaSpAkIIAxk6ThWlKT5VkOP5OjuKYrjCCw+gE",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDKMQ0C4NQgDgIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDIIA4jGDobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4OQuDUIA4hmG4hiOJYnD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxhqHIiiSJooD6AQ",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4MYwDGGIaC+IYjiWJw+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4MQ2C4NYzhmG4hiOJYnD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock3,
    #[cfg(feature = "clock_4")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4MYwDENIZhuIYjiWJw+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[cfg(feature = "clock_5")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDKMQ0C4NYxDYIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi6MgxDYLg1hmG4hiOJYnD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4OQuDWMQ2hmG4hiOJYnD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDIIA4jENAihsL4iiSJooD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[cfg(feature = "clock_9")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDIIA3C4NYzCKGwviKJImigPoBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock9,
    #[cfg(feature = "clock")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDKMYwDENAihsL4iiSJooD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ3hAcoMDMIg+DwL4DgWB4bHAYR0GgIBkgwTQ0C4MggDENQuDEQQ3CCMwwi2N4uC4NwxCAOBoDGOg5GGKg1CCRAgjaNo8DILpFDiK4ahyIokiGI4licIhtjmMwxk8NAtC4OZghmGwvlMaJViSJoMG0OYri2L5imGY5RmaVpplebAxDALg2CAMp8DcLgzmAOZ1meeJrlkMQzjCLYqkyhJhoedw8meipao2fqAjqYJfDGlJUpaVqYnsLg4o+g6eC2oJloio5qlgbZPDOLZPn4MaeqGaKwnmi4qlyL6orkLg0rsPoBA",
        categories = "development",
        tags = "computing,ai,cluster,network",
        contributors = "karsa-mistmere"
    ))]
    CloudCog,
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
        svg = "gAPBwGEdBoCAZA9CITQxDALgwDgMwgDkQQ2g4MAyCCFQwhcIAwh0IAxiANggDQYQ0C4Mg0hGJ4phGHovhmGRjh4MooDILQxigMIjicMQ2C2EQ1C4MQ5DcIg+DwL4DgWSJLgaCIKhEMgwGENQglaHohiEOAuDkLQ0EgMQzGGEYuh+IYYkKKZHkmTpNgST4JguaAwHYMpskqcJvgWB5yDeIA5naeJugEA",
        categories = "weather",
        tags = "weather,partly,night,rainfall",
        contributors = "it-is-not,karsa-mistmere,jguddas"
    ))]
    CloudMoonRain,
    #[cfg(feature = "cloud_moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDYYYNg0MIOhWFA2EgNxhDUIIchQMYVDQLg5C0NhaCIPg8C+A4FimLIGgiCgxDALogDkQQ2CCOYUh+Do5DQYYiDINAgkKRI8CCF46kuO5Jg4LYNDWNQ5DeKIqi8PoBA",
        categories = "weather",
        tags = "weather,night",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudMoon,
    #[cfg(feature = "cloud_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDAyDCDQwCIPg8C+A4FhSF4GggIhNDULg3DiDIfiEMhBDcIIohCKwgDkIAxDkaA4C4NRhDSNAgjcNQgiyEAxC4MwwDcLQujAM4ThWGoZgSG4JE0Mo/DUM4MDENo0EGOo5jiPYvDeOAxDAaAtj8Nw5icLgwDAOIpmiao8m+PoQh+QpIhaTA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMgxEgORhDcIIWDAIAxhoIA2C4NwxC0ORoDGH4UDSDggiiD4ZhuG4ZDkWgiD4PAvgOBY0jeBoIgoMgyhoMBhDMIJDhmRgtDOSBoC0MguDIMA3GENYODCP5TDWVQgkaWgtDEMIflWDozjWOg+gE",
        categories = "weather",
        tags = "weather,clouds",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Cloudy,
    #[cfg(feature = "clover")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLgyCAMwuDgYQyC4NwghWFwwCCGwwC2EQ4DGHBsC0Lg0C4Mw4iUNIrhSFoYi+HYch+EoPDAQ4NDcMwgicOA1CCDQ2heQQ0CAOJGGyRZKC0NBjDGKAzC2TwzDYIJPDIN40DENYOkyDh6CIPg8C+A4FmKZYGgiCpHDgY5SigNpvDOUYRlub5Yl4MolDKLoXhmHKAjKIAxGyKA4iaK4mn2MIaoGgIgDIQ49j8MQ3g6QwuDaRaVlCRpWDaYZjmiZ4EmmCYLlWDJOnCVqdnWlKXjyDoOouf4yh2NIhiShqJiyip/rajoepAQwxDkLpbkCFo7pyVLKkKn5GqGZKlqSBYHqelpHDIMK0sCMaOoOI6IoaJq/i+waCjWHKrDCcZPu6rQxnqFY+nsMZ5oSTL7kqbpTsycJ3her57rIMpgmK1JmgKpbYCIbYXpULY/DW06jgE",
        categories = "gaming",
        tags = "leaf,luck,plant",
        contributors = "ericfennis,csandman"
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
    #[cfg(feature = "code_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIAxDYIA0C2EoUCIPg8C+A4FheGoGgiCoQDiE4RiQNIWhiHYcgSHoJgsNAuDWEQtjGD4nhmKw+gEA",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "ericfennis,mittalyashu"
    ))]
    Code2,
    #[cfg(feature = "code")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDYIAxDgIAyDKFIZhIIA2CIPg8C+A4FgeCYgiOBoIgqDBug6EIWhOG4ZhaFYfiGKIlGUPoB",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "colebemis"
    ))]
    Code,
    #[cfg(feature = "codepen")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDIIITDKEw4C4NYUhMMQ1hkIIShuFIgh6GoXh+IQyCIPg8C+A4FgcbosGyDBlCAeQyhCHYZCIIB4jmEYqjcMYQhaPR4kSQYri2NBuGWLIvHmTY2guDYPCKFgghiGohjuJpajyLIugSUo1lCZJTgob4Mg6RYkiiJ4mhyJZLmOBZTjONY+kCEpHkmfZDkWPY4hCW51niAQA",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[cfg(feature = "codesandbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FlAcBvGweRsGkbhlCCVpbHQc4JDcLg1i4LpLDGFA2C4OI9myZg0mgMZPlGVpYlqXJVleWZbl2XxumGY5lg4OYeDkIJkmYMZyDaNIODKdZSnyeRlnueJ+l4b5gmIIpphSD6EowLqOqGi6GDcOaSnefZ6gKlKZoCggiDOaItmwOalqAMguDCDQyDCPwgrgNqrrCrqVCAeQygmagiCAeLMCKzrQDGzaRsq1qerwMA4pKlQ+gEA",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIA4GgMRhDQIITDAIAxhcIIWg8LQxCIPg8C+A4FiCI4GgiCgzg6EA0HYOYShSGoyDELYTDQSA3jCFYzjWNRah+IYmiAbBpG4ZQgHkMoJDQIggHiSgiDaTR5DGCQyk0eJVlGQAvkSRpDkWR5ZgkMQwlOWpXk6UJllOUJMiCXZhmCRpIm6WJrkySJoliWgxm+IZeGUPoB",
        categories = "food-beverage",
        tags = "drink,cup,mug,tea,cafe,hot,beverage",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Coffee,
    #[cfg(feature = "cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAYQ4CCEgwCAMQghUMAtDENoTh6GYYiGHBaCIPg8C+A4FiaKYGgiCoMhYNBhg2DYVheGQtDSDo7iCIA0iSJoogQaIrkOB4JguNB2DKJYniyRYFkeL40DIdgtkyQZPgKRouG0MQ3g4MAuDYNobhsLg3DOTZCiqW5Ri6C4XDGYgymCYAzC4Mw0muWoslIbYPmOHZfmeaYbnyQ5Qi2CRtnieggmAMZoDOFqIm2fpwDGOoMGgOKWkSbqLgqDacliTqJqGf6BmSkKFpQMafoqf6OpukaToeWaopijJfCCtIbhak6xqmXQxnKeKGjoNguDmaq5iqAQA",
        categories = "account",
        tags = "computing,settings,cog,edit,gear,preferences",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Cog,
    #[cfg(feature = "coins")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDYIggGMeILDiDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDEOAuDAOQgDEMAuDMNxBDYII0DCLY4i6MA0i2E4ZC+IIih+IYjiUIhNDeNRoDEdg0hiGpBGiQ4iiSCxtDENguDcMYtDMLg4ikN5aDELQyl8MggmYOAyk+QJED6AQA",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[cfg(feature = "columns")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRyHmCoMCCCQiDODYZh6DRoGUaRnGgdIUhYPg8C+A4FiwbBpG4ZYcDKFIbHkMYKh8IB5jeCwxg0eI7CIMYMiwL4yjQPoBA",
        categories = "layout,design,text",
        tags = "split,parallel,vertical,horizontal",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDgIggHmDgyhEdxpGQdBog6EAgHIeIUhGIAihUPg8C+A4FiYcBhhoIBkg4TQxDQIAyGMMQuDEIAwjUIAuDmPQyHYNBjjyOAxC2P41C0Mo1CKJgviyGori0aIvjEMo8jaR47j2SpNkKRJGjmSZADKTJOlCUholSLowCKMpGDgSA1GMLY4DeOwtDOdwunyfx2neT4nmuKxvGweRsGkbhlCAcBvosdBzg6eQyjoMZyCCeQxDWg5RoeiaLGWJopgaH4hhKDozhaGIahyEYIgqDKvCCI6rlCpQ+gE",
        categories = "shapes,development,files",
        tags = "cubes,packages,parts,units,collection,cluster,combine,gather,merge",
        contributors = "danielbayley,ericfennis"
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkZxvG4IIiGkbh0HOEw2C4Mg0CANwuDcNggDENAug6OY7j2NY3jmMIyCAOQuDgOJGkiSgxkSM5BDYIobC+IokiYbg+gE",
        categories = "navigation,maps,travel",
        tags = "direction,north,east,south,west,safari,browser",
        contributors = "colebemis"
    ))]
    Compass,
    #[cfg(feature = "component")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NQgDiDQgDkIAxDIbAtDOEYZDUTAyhSFobhiDRaCIPg8C+A4FiaKYGggIhthUIIehsIIbEyMYThCDoMg6MQyiSJoogQaIrkOB4JE0MY6g+EQyh6FYXjSNgxj2IINiINZAieLJFgWR4vjGVI1hqDY3h6TpRleU5PlmJZbkMPoBA",
        categories = "design,development",
        tags = "design,element,group,module,part,symbol",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Component,
    #[cfg(feature = "computer")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINQiCAcoICIMoMHcaRkHQaIJDENIMGgZRpGcaB0gkOIMHmCYRD4PAvgOBYoiuBoOiaEoUhaJgwiSGIageMQghyHogiIIooiqBB0igcBhhYIBkgkTQ2CAMQ4GiJ4pkeFpGkgaJKkwMQyk+UQ2kGVJYD6AQA",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = ""
    ))]
    Computer,
    #[cfg(feature = "concierge_bell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ4GGDIMDAIITDEIAyC0MhoDENoQheFIghaER2DISAyHaGRaCIPg8C+A4FiyL4GgiCgyhWHQ4CCOYVhQLYchSK4tjKMYEjOCRNDGDA0HYNJBi6RZEgWB5HDGEw0GiTYsk+MIBA",
        categories = "travel",
        tags = "reception,bell,porter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ConciergeBell,
    #[cfg(feature = "cone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDALg5CAMQ4C4NQ1C0OAtDENYODgYQxhAIAwiCIIYC4OIgGyFoQhoOQ4CIPg8C+A4Fi8ZRsGwaRwHMZQgHIeIJDkIggGMeYJDGQI8kQIgzkEY4+CIMQyi6MI1jeORlD6AQA",
        categories = "shapes,maths",
        tags = "conical,triangle,triangular,geometry,filter,funnel,hopper,spotlight,searchlight",
        contributors = ""
    ))]
    Cone,
    #[cfg(feature = "construction")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHmDQ2hEaBlGkZxoHSDQ4hEch4g0MYRiGDgiD4PAvgOBYoHAYYLCAZINE0MQ3CAMQ0HYN4nimLoLi2LxojGM42jiOo8C+PhokCMIyCKNI2DMdgzkiSpMkKThNlGU5VkGV5Dk8MQwjcNAgDILgzCANpol2Pw8kqYBtjiaggDcLo2naO4okmXpvkGcQ4nSgYenuVoBA",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDgYQ0CCEgwCCFQwC0OIWCIPg8C+A4Fh0YxpHIYxsGUIBjHmCQxDEIggHKCQzi8Yx4iwMoch6I4lieHRyGUYx0CAaBlGkZxoHSLA4i8co2CKOAgHcaRkgWSovk6MwgisIg0jkL4/kGHRsGkbooHkMYJlAeJoCKS5aDKCZdCAeJwm2XpjmWYpkiia4sDaLx5nWcp0n6gJsjiHQvngZQ+gEA",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDgYQyCCEgwCCFQwC0MoZEgOYRhOFoghiEgyCIPg8C+A4FiYchlGMdAgHcaRkgWCYPCIIBoGUaRnGgdI1DiNx5gkNI3HiCQzjccpGCKJImC+LIuiYYxpHIYxsGUIBjksMYklmQgiDEMJJgmTYnlOVZXiYbBpG6WB4DKCZACAeAxnGQZ1kyQZwCKRJOmubZqmybp4DENp3mSepDkWe6FiWJ5/GUPoBA",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "lscheibel,karsa-mistmere,FPDK,ericfennis"
    ))]
    Contact,
    #[cfg(feature = "container")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA3C4NxjDALQuDaFA0C0MQuDKFA4hkLg1GwLQ2C4MwtDMLg5GGGg3g2LINDAIIxhOLIyiIMQwiUIA2GOFA1huFA5C6HopCCGg0HaJI8jGIAuDSRobkOUIhiQMwgiiKovlCLYyl2MY1DAbI4iWIxjiCFJYhkIIph8NRaCIPg8C+A4FnGdIGgiCo4CAMoaDkVgxDQTAyC4MQgkIMZwnKd52gSeIJG2e6Bkafojimipzo6jYFgeCRNpMMZCDgdgtDihaYoyAqOp2eg4kaDw1FaQg0qimoBA",
        categories = "development,transportation,mail",
        tags = "storage,shipping,freight,supply chain,docker,environment,devops,code,coding",
        contributors = ""
    ))]
    Container,
    #[cfg(feature = "contrast")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTYUCAMQ4GENggi8MAgjKNAthQdoUHoIobC+IokD6AQA",
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
    #[cfg(feature = "copy_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDUIINg0NAtDQIg+DwL4DgWGByGUYx0CAch5gkMgiCAaBlGkZxoHSCQxhaIh4iWJ4kCIOInjON4nHcaRkgWL4WhgL4eiCGIbgaCAiE0NIODYYwtDELgxCAMAtDILQuDmV5XFYNBjlaUpSluEpXGgMQwGOYpVhEIJam2JpDkgPoBA",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIoHDKCgxDiDh5gmCw1hSEIXCIPg8C+A4Fh0chlGMdAgGgZRpGcaB0hENIOHIeIKg0IB3GkZB0GiLoUgqE4Hj2MB5jOHIeiOJYdHAYY5CAZIKE0NAgDENhjC0MQuDEIAwC0MgtC4OZblsVg0GOWpWlaXwyCCXAyGgMQwGOZpZmoIJenODYdC+SY5D6AQA",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDUIggHmCYLDiDh4DGCoMg4eYWgsMgiD4PAvgOBYfiKBoVheHYHhEMYTg+K4Ng+G4Yh+IYEGWHxyGUYx0geCotGgZRpGcaB0hcNIOHIeIKikdxpGQdBokaSB5kuGY+h6II5juHxwGGUAgGSChNDQIAxDYYwtDELgxCAMAtDILQuDmb5vFYNBjm6apqnMMggnAMhoDEMBjnqbZ+CCcqHh2NJdlAPoB",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDgIggHgMoKgyDh5hGCwyg4eIJhcIg+DwL4DgWHhyGUYx0CAch5gqGIoHiK4UgqDQgGgZRpGcaB0hINIZjGDh3GkZB0GiOodh+JImh4cBhkIIBkgoTQ0CAMQ2GMLQxC4MQgDALQyC0Lg5lyXBWDQY5bleV5gDIIJdDIaAxDAY5nlqawgl+dIYh4L5KkIPoBA",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIggHkMoKDEOIOHiEYLhSD4JguDQ+DwL4DgWHohgYeYbhODoQhKDYHheKIHieHYfiSHhyGUYx0CAdxpGQdBohINIOGgZRpGcaB0kCFYKhkeZLg4ch4gqLByk0IoyC+No4h4cBhj4IBkgoTQ0CAMQ2GMLQxC4MQgDALQyC0Lg5m6bhWDQY5tmmaZyDIIJvDIaAxDAY55myfQgnGhpXlyPg+gEA",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAcoICIMoMHcaRkHQaIJDENIMGgZRpGcaB0hiGggHmCYLg2JYQCIPg8C+A4FiwcBhhYIBkgkTQ0CAMQ2GMLQxC4MQgDALQyC0Lg5kSRBWDQY5Dj+P5IDIIJFDIaAxDAY5PkKUwgkeXIRiwL4yhYPoBA",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[cfg(feature = "copyleft")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ5C4MQ3CAMQ0C4OAzGENAgjUMIuCCOAwC0NQuDYNgihsL4iiQPoB",
        categories = "",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[cfg(feature = "copyright")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDQLg4DMIIqiwMxhDQII0DCL44DALQ1C4Ng2CKGwviKJA+gEA",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyright,
    #[cfg(feature = "corner_down_left")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig5CAMQwCANITDUIISDIMAiD4PAvgOBYHgmHxwGEdBoCAZIQE2HIWHYNxhheF4VhUMQtjQSA0h6IInikPoBA",
        categories = "arrows",
        tags = "arrow,return",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownLeft,
    #[cfg(feature = "corner_down_right")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDUIAxDAIAyhaEoUhOGAiD4PAvgOBYHgmHxwGEdBoCAZIQE0NAgDQdg3GGL4vhaN4wjAaAxDKHogieKQ+gE",
        categories = "arrows,text,development",
        tags = "arrow,indent,tab",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownRight,
    #[cfg(feature = "corner_left_down")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQIAxDUIA5CAMgwCCE4VCIPg8C+A4FgeCYgHAYR0GgIBkhATYahwaAtDcYYThOG44C2Nh2DEMofiGKIqD6AQ",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftDown,
    #[cfg(feature = "corner_left_up")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQIA5hQIIThMOQiD4PAvgOBYHgmHBwGEdBoCAZIQE0MgwCCLBoC0NxhhgIIti0MQtDSORWDSG4diSJg+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftUp,
    #[cfg(feature = "corner_right_down")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDAIAxDWFIWDKE4ZhcIg+DwL4DgWB4Jh4cBhHQaAgGSEBNDQIA0GgNxhi6LoThMMYvi8dgxDKHYfiaKA+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightDown,
    #[cfg(feature = "corner_right_up")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDAIA5CAMQ1CANAgDKEw5CIPg8C+A4FgeCYgHAYR0GgIBkhATYahwaA3GGGoahON4ZC0NBWDSH4hiiKg+gE",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightUp,
    #[cfg(feature = "corner_up_left")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig5CAMQ0CCFYShINAiD4PAvgOBYHgmHBwGEdBoCAZIQE0MgwCCLB2C0NxhhWFYtjYLQ0jgSIahyHolGgPoBA",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpLeft,
    #[cfg(feature = "corner_up_right")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDUIAxDQIAyDAIA5hSEw0CIPg8C+A4FgeCYgHAYR0GgIBkhATYWhgdgtDcYYWhaGYZDEIA0C0NBoDEMofiGKIqD6AQ",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpRight,
    #[cfg(feature = "cpu")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINAiCAeYJgsIB3GkZB0GiCQxDaDBoGUaRnGgdIXhkIByggIgyCIPg8C+A4FimLIGhKFIWCKIoOCIOYahyHogjSDIljiKYrgQdIpHAYYVCAZIJE0MQ1CAMh2ieQZGhWRZHGiSZLk2TwwlGKIqlQaJWkiSgiE0Mggk0aJSmCV5jliZZnCAOZrl8L5hm+WZmDIMJpDWdZTm4PJhnqZ59nSbJ3oKhJxDmT5eoGVaDlehaOnykJtlWAQ",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCAOQuDMYQyC4OAgjCMoqjYLQzC4NQgDEII5j2PwgjaQpEjkNIzC4N5IkqQ4qjmOwxhmG4hiOIIiiSJgiiiSosi6NJIjWRAwjiOo8j4LpAmiYprkaS5uk2Z5QlIL5UGgPoB",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggGgZRpGcaB0g0MQ0hEeINDKER5g0NYRHKHYOCIPg8C+A4FikbBpG4ZQgHgMoeh8IB5DGGYQjOOonjiNQiDGEIpC+L4xD6AQ",
        categories = "account,money",
        tags = "bank,purchase,payment,cc",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    CreditCard,
    #[cfg(feature = "croissant")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0C4NggDEMwuDEMQgDULg3DkLYRDIMRjDELg4hmHwwDUIIMhiD4XDgIIRDeFItDEbIaC4MgyhWIAxEMOIgCAMoRDENoXDmD4Wj6JoNg+PwxFoIg+DwL4DgWTpRgaCIKDEMAuiUOZaC0MQtDKNA5EOXI2kENIrjuK4OmsSIMDUQ5hiiDpmlqPQgjsNRhDeF4UnyLggDCgaDjaDA4DOTZPlSU4ElWCRNmsYwwl6WpFDQLaXDKmJgoGYJ2mEMQ3p6JQ0omUKNoyBYHgkbQxm+SafmKDxjheEQzimHIXriFIfDUdoRDWkopi4LaxDWo49lqe59CCf4UoKgpfoYM5gqai4Co2qwiE0MYrkCHqVoOl40DSJp3oKmqgDeybHuWYQ1taqIBA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyGEMoOCAMIThMLYRDIdg1EgNIQhKFIgheDh2DIY4UDELooDmEoYGgNYaiYIIoiqLIOGiD4YhWIIOhcdgtDWLoehGO4UDKPYXkKOoWkYMhoj8VodjmRIXheTgyHoIg+DwL4DgUPoB",
        categories = "shapes",
        tags = "healthcare,first aid",
        contributors = "lscheibel,ericfennis"
    ))]
    Cross,
    #[cfg(feature = "crosshair")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bGwaRuggeAxgwMoPCAeAyhMOIQHmLYNioeYnjMIobC+IokiGI4ljKNIyg6EImgwNowjaQ45jsZY9iQII1kaMJAkSSYqiyGY4hyTJOiWVpTi6RJCjSNoplqOo+D6AQ",
        categories = "photography",
        tags = "aim,target",
        contributors = "colebemis,ericfennis"
    ))]
    Crosshair,
    #[cfg(feature = "crown")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCANAgDMIAxDIaAxDQbAzC2EgtDYIA3C0NAth6DoeDaIR6G2EAxDaFA0CIPg8C+A4FD6AQA",
        categories = "gaming",
        tags = "king,winner,favourite",
        contributors = "ahtohbi4,ericfennis,csandman"
    ))]
    Crown,
    #[cfg(feature = "cuboid")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELgxDIIA2C4NAthIMA1C0NAuDANhhhCEAwCCIQwC0MoODcLYbDUTImDkNQgDiEwxh4IIgiKIopi4IINDcdg1C4OAyjSNojCCQA4jsLg2DYbIWi+GgwDeQ43kWJgxDeKhsDkLouhWDpCh+VI3lyGI8FaMYclORYkkcLYNksWgiD4PAvgOBZznaBoICITQxiEMgyHYLQ4iwLgyi+WwxDWcp0nmeIEnqCRtn6Ow0juPIohIOA3oydaQD6AQA",
        categories = "shapes,maths,buildings",
        tags = "brick,block,container,storage,geometry,rectangular,hexahedron",
        contributors = ""
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwOIQGMeIMg4Ig+DwL4DgWB4bGwaRuggeAxgwM4QHmJwiikIB5DKDA2hAeIxCKM4bC+IokiGI4IjCMoqiyLo1hiFAgiaDAyDGGocjsZY9iSL4skuKo2DGR5FjeNJDk2Oo+lGP5UkySJjlaRo0leFI5k8PoBA",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "cylinder")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch4D0Ig5CKCh5g0M4QGOEgiDWFYMCIMQyCIPg8C+A4FgcZYgHAYR0GgIBkg0TQzCANR2DENBhDkIIwDAII6joMQ4jsVoZiAL4oioPoB",
        categories = "shapes,design,maths",
        tags = "shape,elliptical,geometry,container,storage,tin,pot",
        contributors = ""
    ))]
    Cylinder,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx5D0Ig1CIIByHiDQ5hAY4TCIMQyhAcoMCIMwiD4PAvgOBYHGWIhwGEdBoCAZINE0MwghoYQ5CCMgwCCOY5DUIAyC4NoViIL4qiyKYri2LwiE0MgxCAOQuDMVoPkORRokeLIujCMg1HYMQ0jWN46mOOQ2C4NA3j4Lg4DiIYjlaWJJjCGozDIdg0GgNJukSSJxlqSwxjIMgwGGPY9juYw5C2Mg0C6PaNoeZAtpCk6OGMLQxlGOAtj8NaQDSi5nk6mQ0DETJ0DENp7nCAQA",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx4D0IgxDIIggHIeYNDOERjhQIg1hEcoMCIOQiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUVgxDkQQ5CCMgwCCPI8DENQgDIMQuDgNIhiOKosimK4ti8IhNkOMxWDiSAvkoaJMiyLowlKDxMDEOAgDENxIDIMpgjmZ5WliWpOjCMoPjiOo9nWPw0C4NY5DGeA4DebJND6AQ",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch5D0IgzCKCh4g0OYQGOEgiDEMoVgwIg1CIPg8C+A4FgcZYgHAYR0GgIBkg0TQzCANRWDEORBDkIIwDAII6joMgxCCNBWh6IAviiKonimK4tCKL5ADKNo4juUo9j+GYfiGRhoD6AQ",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA1EgORsC0NwghSFhoDEMRhDIIIcg2H4dC0MhWDeG4dCCIAwiKIhaCIPg8C+A4Fi8bBpG4ZQgHkMYJDkIggHgMoJDEMo+HiOwiDEOI+HmQZIDWLowjWN40jaOJGkKRI5k0MZPj+W5KjmR49i8L5SGUPoBA",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINAiCAcoMDKDxjHiDAxhMPg8C+A4FgeGhwGEdBoCAZIME0MQwC4MggDOKxDDULg1CANAgiwOAuDGNggDEMxhiyLAwCCQpCjUMB2C0MY/juRI8jSQx2DSS5BkOVZGkiUpAlWQo6leSpak2RZDGMMAtDQLg5C2Lg1C2aY4DKbQuDgIoaC+IYjiCIokiYIhNi6LAxmcOBhDkIKFmGPA3C4NpDnSG53GgPoBA",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = ""
    ))]
    Dessert,
    #[cfg(feature = "diameter")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ5CIIBygwMoQGMeIMg4Ig+DwL4DgWB4bh6BoIhIIoUgmFwiDWFYLiqGociKIA8HAYR0GgIBkgwTQ2C4NA4CAMwuDYNhhDEMAgkYIJHkcMZIkEOA2k4LpQi8L40jaG5XjeOQiG2PA0k2X5NDEMQuDGP5kmYOJVlqWY1luOpBkMIJfDiRZMkuSp6DGT5RnyUw2mybw+gEA",
        categories = "shapes,maths,design,tools",
        tags = "shape,circle,geometry,trigonometry,width,height,size,calculate,measure",
        contributors = ""
    ))]
    Diameter,
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
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQxDiDB5gmCwgHcaRkHQaIUhYIByhgIgygwcoIiYIg+DwL4DgWLRwGGHwgGSCRNDEMggjoaAuDAMYsi6M4fD6AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NHIeYKgwIIUhCDR3GkZB0GiCgxDiDRoGUaRnGgdIgiIPg8C+A4FiwcBhh4IBkgoTQxDUIA5GgLgwDEIosC+MoejGMxojWNw5CCOY9j+QYtkQaA+gEA",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEch5g0MoRGgZRpGcaB0g2D4RhgIoVCAcoTCKGg+DwL4DgWLRwGGCwgGSDRNDENggDgaAuDAMQii0L4zguMo0GiNo4DEMggkyPpAkKLpFGiR41jcIhNDiTg2lCQZDlQPoBA",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIIJCIM4NGgZRpGcaB0goMQ4g0dxpGQdBohmG4HHiCoMCCJYQCIPg8C+A4FiwcBhiAIBkgoTQxDYIA4GgLgwDGK4tjKIIxjMaI1jcOI7j2P5BC+QxokWNI2CITZKjmTJAiyT5GlKR5UjiOpYj6WpCl2AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch5g4MoSHiDgzhIcoZCKFwgHcaRkHQaIOhCEoVCKGw+DwL4DgWLRwGGJQgGSDhNDENggDgaAuDAMQii0L4ziWMo0GiNo4DiPI+kCQoukUaJHjWNwiE2TI6k6QZDlKVJJlaOY7lqP5clGSJfkqVwxDIIJsluUJEmiAQ",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAch5gkMoMg+CoMHKCAihEIB3GkZB0GiCQxDiDBoGUaRnGgdIgiIPg8C+A4FiwcBhh4IBkgkTQxDYIA4GgLgwDEIosC+MoejGMxojWN45CAMQyj2P5Bi2RBokaNI2CKOI6jmT5AkKU5VkiVxNDiO5clGQ5HmCSZYmSTZml6aQ8lOa5jkwNpvlKaYBA",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDIIggGgZRpGcaB0g2D4RHiDYQCAcocCKHhyHmHYRiWDgwCIPg8C+A4FiwcBhgsIBkg0bQxDcLg5DIIAxDQIAzC4NQtkINRhDILgykCSZLCAMJPlEMJFGwLZEDSOwykiSpMlyUpPkWTxMDGUA2iuLYyguMYzGiNYNE0No+DgaAuDAMZnC+aRomuNI2CITZkj4NJ0naeJ6nybZ+oANQgDahJ3iyeZsoibp/DEOAgDmj6GpOAQ",
        categories = "gaming",
        tags = "dice,random,tabletop,board,game",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dices,
    #[cfg(feature = "diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ0CIPg8C+A4FhSF4GgiCg1CAMQwGiEYThWGoZgSG4JE2HgyDGIoShSFooD6AQA",
        categories = "development,files",
        tags = "patch,difference,compare,plus,minus,plus-minus,maths",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Diff,
    #[cfg(feature = "disc_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bh6BoIheGYPgoIg0haEoMhqHIiiAPBwGEdBoCAZILE2FAghQaAuDAMQihsL4zjUPoB",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0Nggg4YwwC0MQuDeMAtDMLgyioLg4C0NI1hmG4hiOGodgaCIKhOD4RkaFYMg+GocgSQ4giKJImCITQxDiKgyiyNw3C2MAgjQMoujgII7kyPpSD6AQ",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGodgaCIWCKD4RhOJoKiiGYbiGH4B",
        categories = "devices,multimedia",
        tags = "album,music,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide_circle")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDIIoHDKCoMg4eIJCIOIThCCw2CIPg8C+A4Fh2IIGgiEYbCCFIRg2KIZhKD4mhyHojiKBIGimC4rHiLYriWFoOHmGYXh2H41h0YxpHIYxsgYcoRDCDhjHmKpQHiU5DkeSZLD6AQ",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[cfg(feature = "divide_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoShUIgzhIdxpGQdBog6EISHmDobhOJQihcPg8C+A4FiwbBpG4ZQgHgMoiDaGAxg6EQgHmPIPheP44kIIosC+Mo0jGM41jeIpDHiQQxkMeZFDGOo/lOOpIkoZZMjSNpXlGU5VkWPpAj2R4tl4PoBA",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgigkeIMDEMoPHKEgiD4PAvgOBYHhkbBpG6CB5DKEoUCAeAxgwNYPHmKgihODx4iWMA5hiGogiKGYcgaCIWjCDxjhGMIngqEg4jeG4Ej0PoBA",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyGMLQxC4M4NhINQtDILgwDkMggDOGAug0NIhE0OYOg+EoUhGIYOhqHAtDOLYXiMNQiD4PAvgOBY3jqBoIgqHYMGMM4TDOH4xDYLg2DYN4wCAMQwjAbZQjCEIWk+E4XjGGYbDILY0jKNo4j2PIEj6CZTDcIA2iCF4ZjWN45maZYFgeaAxDQIA4iuF4WmKco7gKZp2CIbZqDEOIymGcZkoKdY/G2RIVjSIYhn+jY9oQbQyDAIImpWcJjnOjpnoWSYVm+T5Ppeo6ZpAMank8MIsDGq6Mq2g6vp2sJYhWlq3oEbBpG4ZQgHkMYJDIIrGDKybKCAeLICKzx4s207KnGwrED6AQA",
        categories = "medical,food-beverage",
        tags = "gene,gmo free,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    DnaOff,
    #[cfg(feature = "dna")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ1GMNguDYNg3C0NoNDMLgzhsIAwCAMgwhYIg+DwL4DgWJIngaCIKDmHwyGMMQuDcOQ4C2Mg5jWHwuDUMY2hmOQ1jsOAwhUNQujkM4jiWKopgSK4JE2DofGON4zjqOI1C0Mo8j4IJADkNZbC6RA3CCR5JkuJpPk6BYHgkbQxmYNpjmKXA1mqTYCk+bwinENAgjYMY3nmbJ7m6LBtmaXp3jueIkmuKKHlCfoZkIMQ0jymqPkyhoqn0bYgCCLqboWkqfomEaXo0MYNqYaJtpScaqg0MI8g2rqQnqqJwDGHgxheMqXjyrw+gE",
        categories = "medical",
        tags = "gene,gmo,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere"
    ))]
    Dna,
    #[cfg(feature = "dog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA1C4MQ3DIQ4MCAMwuDcOAyCAOAuDQMgzCAMguDYNw5CANguDWFhjC2I4ahcNA3C0NIQDGIYpDAMA2jQIA3CALgwh0NwwiEMYYDKK5HhKG4XDYNQ2CAMZSC4Mg2DELYehKVA5jyRw0iuIwzDQNIuioIg+DwL4DgWaZsgaCIKDGNZWj+D4RDIYwwC2RwzieRw1DeM4jDQOYhhcNZmniIgujCWggjUMY3iiQY7pCPpZkKGJFnySJKkgMgtk6UJSp2VpYlqoZHDgNaJl+iYjiAOZmDWaJqm+boEnCCRNDiUg0HaZ5pmuuq5gWB68DGUZzsGtbDriAq6sicpHkmUopkkaKAEwMYbhEbJZDeiYYDUWq2sSbbRsecRNjWH5StWMhBjeEA1t6FwxvYIINvyl5ziqUBDDSUpDDKvg3iqGoilO3cLHMOJmDKW8QimZA0nqnY7qiyqhrGWaFDOoguDMMA5G0LQ5lUNMhjmGhBh0OAwlPMMyvvNsMhsNRjhivoNoCEJCoyyp9ju564gE",
        categories = "animals",
        tags = "animal,pet,puppy,hound,canine",
        contributors = "kemie,karsa-mistmere"
    ))]
    Dog,
    #[cfg(feature = "dollar_sign")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDIIggHkMYKg2D4JCIMoTHiEYLg0Pg8C+A4Fh0cBhHQaAgGSChNDENwgDUSA5C4NRhDOMQgjQNQgDCOY7joNxojKN42jWOo6DGOw3EgNgih0L4jiUPoBA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg1CAMQwGEMoOCCFIPDAIIZDELYUDQLQzEgMQ4hMLg5g+FInhqK4chQNgtDQLg0hCG4ZhuKw2C4MwgDcLgxGMLY6C4MpBDiOgtDGQ46CIPg8C+A4Fk0YxpHIYxsGUIBygkMwiCAYx5gkMQyl0Yx4mGY5NC+U5VlcPoBA",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = ""
    ))]
    Donut,
    #[cfg(feature = "door_closed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDAVg2GEMoOCAMIWhYLQyhoSA4hOFYXiGGoOHYMQ0CIPg8C+A4FimLIGgiCoUg8aIPiiKovi6BIwgmCw0CAMQyHYLgwDGN4rjsPoBA",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorClosed,
    #[cfg(feature = "door_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA0GgMxhDIIITDAIIWDGFIUHYMQ0CIPg8C+A4FiCI4GgiCoTDIMIQh+IYmiWBIngmC4NisaA5i6IoyjGBYHjQMYYDIdguDAMY6jCAoyj+CoMg4Lg1DaQwxDYLgxDUNxhhmGYWhgLQxC4Mg0DILg5DcTA1hQMBWDWUJShKGpdCCW5QleX5lDQbA0l8QYqhef5bg0NJuDEWpIjyAQA",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorOpen,
    #[cfg(feature = "dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyC4MQigkeYMg6EISHKFQiD4PAvgOBYHD6AQ",
        categories = "shapes,text",
        tags = "interpunct,interpoint,middot,step,punctuation,period,full stop,end,finish,final,characters,font,typography,type,center,.",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Dot,
    #[cfg(feature = "download_cloud")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCgxDKDQyHYOY3jmPo9gSP4JG0OINhWDJTC0NJIjqTA+gE",
        categories = "arrows,files",
        tags = "import",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DownloadCloud,
    #[cfg(feature = "download")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUdg0GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGx2C0NAiD4PAvgOBYrHAbxsHkbBpG4ZQgjGNh0HOCQ3g6GYUg+Do/DEMIqiyMYzjWN4rkyOB5DGCYPCIIB4DKUwylUeZYCIM5VHiUgiDGWorC+Tw+gEA",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[cfg(feature = "drafting_compass")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINQiCAcoMDKDxjHiDAxhMPg8C+A4FgeGhwGEdBoCAZIMG0MwgDIMQgDgLgwDILQxDQLgyDYIoaC+IYjiCIokiYIhthgLg5DkIA2C4Nw0CAMZEikMwuDQNI4huOxoj2I4lgwTQxkaGBjC2UA4DcIA0jIMAuDGKZmjMIAwlSOo+liP4niuKgxC0MppDaYQuDiU45lYPoB",
        categories = "shapes,maths,design,tools",
        tags = "geometry,trigonometry,radius,diameter,circumference,calculate,measure,arc,curve,draw,sketch",
        contributors = ""
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
    #[cfg(feature = "dribbble")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ5C4MQzCANQuDAORDDGLwyDIIIqDENAgDEMI8DALg0jsMguDINY/C4OQ0hmG4hiOIIiiSJgiE0MgxC4N5Hg4Lg4DQYwtDYLg2DILZXDQMZlkSOo8mWYQzDgIJug+Ggvk4aJQiOJYnDgLg1DYIJElkYw0C4Mw3nGiIqDSN5wDEN5YnOTZRD6AQA",
        categories = "brands,social,design",
        tags = "design,social",
        contributors = "ahtohbi4,karsa-mistmere"
    ))]
    Dribbble,
    #[cfg(feature = "droplet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYQ3CCEgwCCFYVDcLQ3GMMAtDILQxC0MwuDmIgtDULg1HOIopC0NIuC0NopGMLYpg6LYNDSJIuCCMg1EMNggDEMQuDEIA1kIM5HkINYRhOFpQhiEx6CIPg8C+A4FD6AQ",
        categories = "weather,gaming",
        tags = "water,weather",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Droplet,
    #[cfg(feature = "droplets")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQ2C4MxjDILgyCAMAgDQLQxC4OAzhgLQ0C4MA1haGgug4LQuDUNwthMMg2iYNwxC0M4nDkUw3hQOQgg8N4kgwNYQGOKQyjuGw0DWJgxDQIITDiGYujuNQ3DYU4eDGGwxCCV4uDUY4Xi6FYbDiGIiiSTIhiMegiD4PAvgOBZtnCBoIgoMYTDUNo8C4NhBDEMAuDmDJ/oGDIXoeDZMjUMAyGOKpNo+FYhjuTIPDUc4ejWJIekENRhg8OZkqCZKIjOWKBlqkw3mybpzD6AQ",
        categories = "weather",
        tags = "water,humidity,weather",
        contributors = "Andreto,ericfennis"
    ))]
    Droplets,
    #[cfg(feature = "drumstick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg0DUIIMg4YwtDILgxDMLg2DULQ0C6GQzDILYNDcLQxheFQuDIOYpDKJYnDcNgtDYLoQicMYlDmDgyCCFg5DOKY/CAN4XhyGQ0DYII6DSQI3DiEYODGUA0lKMA0CCGQ1DeUJSiQMYmg4LY1l+GQyjILgwiKHg2leJwzjKbpPnGYA4luZZNh+SZrDMegiD4PAvgOBZ/oKBoICIbQxicMo2g2MoWDGSaQDYYYWhClggDCEYRh2NZJjAM49g4OaiDSpKapqX4nlSnQylemKYqim4ZDENJiiEbKTimkZ+oChQ+gEA",
        categories = "food-beverage",
        tags = "food,chicken,meat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Drumstick,
    #[cfg(feature = "dumbbell")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2C4NQggyDgxDEIISCIPg8C+A4FheGoGgiCgyhOIQtDGJIWhiHYcgSHoJG0Mwgi+EwxieGYriqBYHi0MQ4CAMgyCANAtDSNIpgKK45iCD5AkKRI2kaOIfi6FAwCANwtDeTYbk+LIKDENI9hOVpYheNYbgE",
        categories = "maps,sports",
        tags = "barbell,weight,workout,gym",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dumbbell,
    #[cfg(feature = "ear_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ4C4NRhDOEAghMNQgDCDYYCAN4YGMMAtDGEA3C4OQyC0MoQDIIIpDANAthMNA2CIPg8C+A4FjWOIGgiCoMg8NYfC0Lg3DULgxDOIQuDSJAzDaKJHDSNI2juOoEjyCRNg8OIVkcNRBDaFJhheGYZDGDQ5CCQIfg0Lg2kmSw0iwLg4DGSgwmmEw3jONY3leVoFgeWQximFw2EGhZzmSG5nDGF5AE0MZmDMYYriuZaMnSJ4ig6U5+jkPBsGkbhlCAeQxgkMgiCAeAyqmqqmq4IgyrAeKorOnqiqQPoBA",
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
    #[cfg(feature = "egg_fried")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxC4NQigkeYMDEMoQhIcoMDOGA+DwL4DgWB4eHAYR0GgIBkgwTQzCAOBjDALYcDUIIXDULQ2CANoQjgII0DAIA0C4OIti0N4Qj4c40DKPo6jCQY8jaOo8DeU43lKMZSjaPY2HMLQ3C0MpfC0NYQjCMggg+N4tmqZBDjOaZAi0OQgi0OBaCKHgviWJw+gE",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4Mw5DkIIMg4ORDDWDQ2DIIA4C4MQ1DcIA0C4Ng1CAMQwhwOIQiGJAxDIYwtg2H4hDQM4lC4MofDkLg5iQNwuiwMAgDONw1DaMAwDINgghYMg1DkLQxj8NIfhINw1C2Q4YDUIg+DwL4DgWXZggaCIKDGOg1DOGQxkMOJWEEMYhDCH5xC6cwgkGeYlmiJYujAM5GiEMw0liO5XjoOZGj6V4mlCdpyi0LQynYOKTDWSKFDANJKlEMg5DeXJemOXRsGkbhlCAeAygkMgyCKqQxqyrx5rEIquCAearrarpdC+panD6AQ",
        categories = "food-beverage",
        tags = "egg free,vegan,hatched,bad egg",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EggOff,
    #[cfg(feature = "egg")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIYw2C4MgzC0LgwDUIA3C4OA3C0NQuDUN4aiELQxDCFgzDYLQ0C4Mw0C0MwuDkNQtDmM4rhuNYnjGIYgDCMIbDENAgiANo5iEII8i6I4tDQM5KhOI43jSJIZicegiD4PAvgOBQ+gEA",
        categories = "food-beverage,animals",
        tags = "bird,chicken,nest,hatch,shell,incubate,soft boiled,hard,breakfast,brunch,morning,easter",
        contributors = "mittalyashu,Andreto,ericfennis"
    ))]
    Egg,
    #[cfg(feature = "equal_not")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig5CIIB4gkIgxgyDgxgoNYNHmFYLCIPg8C+A4Fh2IIGHiGoXg6EIShiGgxieCIKi2HIeiOIoEgaL4RhOD4WiuPIUjCDIdh+Ng+gEA",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig5CIIB4gkIg1g0eAygoMYMgeFYLCIPg8C+A4Fh2IIGg+CoSg6GoXg0eYpieCIWhKHYfgQZQ+gE",
        categories = "maths,development",
        tags = "calculate,maths,operator,assignment,code,=",
        contributors = "ericfennis"
    ))]
    Equal,
    #[cfg(feature = "eraser")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMgxC0NAuDOEISGMLYPhgLQyC4NQgDALQzC4NBsDkLg2C2JQ2GOGINhyFwgiENIeGwNYmCCNYqDEII6jqG4dDCMIiEwMQzg0MQiD4PAvgOBZJkyBoICITQyDKRhIDeSJKk+ToElCCRth0MY6DkIA5lmS5dD6AQ",
        categories = "text",
        tags = "pencil,drawing,undo,delete,clear",
        contributors = "maxwellito,karsa-mistmere"
    ))]
    Eraser,
    #[cfg(feature = "euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGgMQyCIPg8C+A4FhSF4GgiCoMDENBoDmE4VhqGYEhuCRNDEOQgDYYQ3C4NwgjCMgwCCNgwC0NQuDILQyEGMIskGN5EjYNoNDIY42DQLoMDMLg1CAOIzC6U5TDKRJPDgLZVCCO49hKFIWicPoBA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Euro,
    #[cfg(feature = "expand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDEIIMC0NoQG0NggDYdgtDQLg4G0MAghkOBohiGgiD4PAvgOBYliiBoICITQzCAMQ2C4MhWgyHAgDAaIfi+DgxGyEQ2iSJoriqBIsgkTYMCAN4aFYM44jqIg4kqDQzGyEIVkOJ5HkaBYHkmMJNDiT5RjuGo9leFJCiWXIpgE",
        categories = "text",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Expand,
    #[cfg(feature = "external_link")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDMdg2GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWDiIIWiOFYbGgNgiD4PAvgOBYxHAbxsHkbBpG4ZQgjePB0HOCQxDUIAzhUMZHkkIA5jCMo3jmO49jGU4+HgMZEDAIggHmWQiDENJcHkMoJDOXB4mUIgyDGTwvlYPoBA",
        categories = "text,arrows",
        tags = "outbound",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4OA4CCDIOGEMwghQMAgDEIIXDQLgyDQIIch4Ig+DwL4DgWJIngaCIKDEMAuDeFA1C4MA4EGLguDSFI4jqGo+hkMQyCANRjDeP4XkaLggDcYQxDMLgxDaGJPlGPoXDELQxC4NpGDKWw4iOJYqimBIrgkTQ2luGZpDYMY3k8NQylKTgunGUoXngIJCkEc4UkmSBhgwN4foKH55heMwzDmWZqmGJpliQbBpG4ZQgHkMoJDIMgiCAeKYCKmqcHgMaZpweakqCjqSpQPoBA",
        categories = "accessibility,photography",
        tags = "view,watch,hide,hidden",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyHMMwtDeDQwhKFAghMMQwhgLQzhyGofhUNwthqEhaCIPg8C+A4FikYxpHIYxsGUIBjHmCYOCIIBygkM46GMeI4DKKIqi+MYzD6AQ",
        categories = "accessibility,photography",
        tags = "view,watch",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Eye,
    #[cfg(feature = "facebook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyGgLQzGENQghQMAghcMAthQNR2DMSA3HYNBoDMdg4GgNB2C2JwzGwMQtiOMBWDcYQxCCNoZjeNwtDGJB6CIPg8C+A4FD6AQ",
        categories = "account,social,brands",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Facebook,
    #[cfg(feature = "factory")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgwGGDIMDAIIThOERoDENoQg2FIdhYLQyFYOBsC0NwgDWIokiaKA0huEoeiCIBIi2EYehSIINFoIg+DwL4DgWPI/gaCIKDGJgxDiGI7j2QpBgSQ4JE0MYMkiSo8j6T5OgWB5RkeSQxkuWJAgE",
        categories = "buildings",
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
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDMIAxDkIAyDKFIZhIIA1hSE4VCIPg8C+A4FgcbojiaBoIgob4Mg6EIbhYMQxhqF4djiIYjiWBIsimAQA",
        categories = "multimedia,arrows",
        tags = "music",
        contributors = "colebemis"
    ))]
    FastForward,
    #[cfg(feature = "feather")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgyDQIAxDKDg0GENgghcMAghoMAtDgLg0DmHogDkTA1hGDQ1FYMQ5GiHw1HoIg+DwL4DgWMxsGkbhlCAeAxgkMQ2CIIB5DKCQyDKQx4kYIpJkSPwiDiMo0jmO44jqPJLgkOZKlAMQ3C4NZDkWQJik+ZZTC+VRlD6AQ",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[cfg(feature = "ferris_wheel")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKDIPD4PAvgOBYHhkcBhHQaAgGSDBNg4IAyHYNAihkL4giKH4hiOJQiG0NguDgIAxDULQzC4NYpi2GowGiMoiiSDBtDIMAuDcIA3j6QJCi6RZHjSJo4joOQgj8M5QkOL4zleSY2kyTo7lGP49hiRJjDyRZlG2XAyDKXQtjqXw4mGVpwjOZRNjqdRonuVZvnGNYnjoMQ4k4YZclwMI7CAMAtigMJ8mOAQ",
        categories = "maps",
        tags = "big wheel,daisy wheel,observation,attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    FerrisWheel,
    #[cfg(feature = "figma")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANQuDUQQzg8IISgwMAghcMQgDiEwyEgMQyHYNxIhyEIVhSE4XhmDYNg8egiD4PAvgOBYxjSBoIgqIAgDIaIVGGJ4niuGoXiOIBWDKL4xjOBBojaTYHgkTY7iCD5AhOQgghqGg3hiKIMlmWwtl0MJKjKN5PgWUYKgwMQ5g+EZYimXoaiWWg2h+IY/kGc5imSZpMjWApQjmC5aDKcJ8hadIbhMOZ5iKJKJnKi4rm2iA1oCaIB",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Figma,
    #[cfg(feature = "file_archive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyFYNBjDALQuDULgyC0MQgC4NoZC4NBDDWDYXCCFoiDIIA2g0aA4hUTAyDAIA3hUVowhKGw1hSKAxhSKgxh+FA0kALg5hyHpCDYaAtDIIg+DwL4DgWThwG8bB5GwaRuGUIJUlkdBzgkMYMjuDA4g2MQ4k2T5UlaWJak4YxpHIYxslsYx4mEMAiCAYx5gmMJ7HKf5qC+cZznWU4EgaCIKDGMQ3FYNqElEaKJgWB4JE2jggDEMh2hmk6Kpai6ZpsMQ4p+TJOlCooBA",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere"
    ))]
    FileArchive,
    #[cfg(feature = "file_audio_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DIIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgqFAxDcdgtDMYYMgyKQgDEIJHDAdgzkuMpSjAYxpHIYxsj0Yx4gkOQiCAcpDnUYx5kMN5nmubZvmqbJunCcgimadp4CCep8n6g6BgEA",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio2,
    #[cfg(feature = "file_audio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMgyGiDhjg4IAwCAMQtC4MoZC4NIbDaH4bDQLg2hsOYmC0MYfFaDQ1EwMYlg8MhIDYY4bg8MIrCCHIrh+JhDiWHQzCANAgDOFpHDQdgzCIPg8C+A4FlAcBvGweRsGkbhlCCVpbHQc4JjKEIZkcOIQhgOJPlGVpYlqXJVgSBoIgoMYYDIMB2isYYdh2GAxhmRoXHYMZ9mWgIZC2RwwFqbJSnOcoFgeCRNDaaZ7oafoXoKO6MoWh5/p2g6No+UxopKdKVh0MQ5nsMxhpel4YokMZ/k2pqRgE",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio,
    #[cfg(feature = "file_axis_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKk8MQwHYOBolKOI6lqWYFgeCRtl6Tw0C0NJTmaPIBA",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere"
    ))]
    FileAxis3D,
    #[cfg(feature = "file_badge_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjiO4GgiConCAMQzGEMwgkeGAxiILQ2kiT4risNhajeOY+j2BI/gkbYMkMMo0ksNYOC0MwtDGZJDkMLZiDWVY6lkPoBA",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge2,
    #[cfg(feature = "file_badge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANxWDQYQyCCEgwCCFQxhMLQyGgOAuDUTAyhUN4eFaIYRhOFopDGGoTGgLQ2CIPg8C+A4FjIcBvGweRsGkbhlCCOI9HQc4JDGDISkYIA4hOFQ4jGM44jqPI+jeBIGgiCg1CAMQ3GEMwgl+F4pDCL5gmaFZoikNhak+NJWlWBYHgkTQ3lsNoekqEwyGwLQzC2K5fhiKw1h6bY1GgPoBA",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge,
    #[cfg(feature = "file_bar_chart_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKicIAxDgdgtDaU46lqWYFgeCRNk+YJiDGZY7GiaJbmuH5fmELQznGZ4BA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart2,
    #[cfg(feature = "file_bar_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKicIAxDgdgtDSU46lqWYFgeCRNk+YJiDKZY7GiaJbmuH5fmELQ2nGZ4BA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart,
    #[cfg(feature = "file_box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMgyEgMQ4GEMoQCAMIZhsMgtDIVg3g4TIMg6EBIDaFoYhqK4ehAdg0CIPg8C+A4FjIcBvGweRsGkbhlCCOI9HQc4JgyGJGDiEIaDiMYzjiOo8j6N4EgaCIKDILg5DcIAxDMLgxDIYwtC4NguDMNpjlqXAuDCHZZlsMQuDcNB2l4Mg4GOGpylgM5wmYOJvmucxsDOaw4DMY5kl4Mw5msNKLo2WAwDaGaEC2caHomaJmmWWqXmyWKenGcx2C2dp4DCYw3m6fafDMOJpDen6Dqan6HGGcZqrmW4rhmHpspQMBsqahgzFqTY0lSU4FgeCRtnCsoNnOvw4DWyI1Giy5Vs60AgtINLUtaMrJjaApUs2CrQHa4pOsqAQ",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere"
    ))]
    FileBox,
    #[cfg(feature = "file_check_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoICIbQzCAMQ1hWFA0C2LowjKUg+gE",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileCheck2,
    #[cfg(feature = "file_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCAiG0OQgDGD4hDQLQ0lOOpaD6AQ",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis"
    ))]
    FileCheck,
    #[cfg(feature = "file_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGMLg1CAMAgDELQuDKGAuDSGg2h6Gg0C4NoaDmJQtDGHhWDeFBMDGJIVDISA2GOGoVDCKgghuKoeiUQ4khwMwgDQIAzhSRZFHYMwiD4PAvgOBZPHAbxsHkbBpG4ZQglWWh0HOCYxg6GJGDiDoXDiTpQlWV5ZluTxjGkchjGyXBjHmYg2CIIBjHiCZqCAcoJnuTwvnKdJ2lSBIGgiCoohUMYuhWZ4MhsNRWjGa5RowPoBA",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere"
    ))]
    FileClock,
    #[cfg(feature = "file_code_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoICIbQ5CAMZHDMLZbl2S4ylKUYFgeCRtDWWIaDMIJqmyX4zGgPoBA",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCode2,
    #[cfg(feature = "file_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCAiG0MYYDEM4bhaHJTjqWpZgWB4Jl4NAgDEN4QhucwymaOxoD6AQ",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[cfg(feature = "file_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQzCKCR4gwNoQHKDIPD4PAvgOBYHhkcBhHQaAgGSDBtDkLg3CAMQ0C4NAtC4OYwhiGogiKH4hiOJQiG0MwuDKKwxjGMIyC6NAvjYaI4iKJImi0NorDaKZGkQIoZkiOZLjqJg3C6UAxlIN4wi8MZWjWWQ8kmTY8i0OIrDCVIuC2ZZXkmWprG0MpGiuT4rmOZpYjeaY5ngOIpkELpuDGf51miao7G2XQ0CCKAzjOMaAnag5MjsTYsCAMh2DYaIUo2gqPgwTaTDULg1FYNBhkCQAwCCtAxqALQyGihg1EwMq0l2rq/rGoK1sai6yEgNrErOx65nOraZlmAQ",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FileCog,
    #[cfg(feature = "file_diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjiO4GgiConCAMQzi+N45j6PYEj+CRNDmQwwGgNpHjqS5KgWB5Nk8MQ3lKVJJgEA",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere"
    ))]
    FileDiff,
    #[cfg(feature = "file_digit")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig0CIIB5g0MQyhAch4g2FQghgIoaGgZRpGcaB0g0NgiD4PAvgOBYoHAYYLCAZINE0MQ0CAMh2DYaImigL4uguLYvGiMYzjYMgyGiNRhDKNwgDCTpQDILQyFYNwuDUTI1leNxIDaS5Nk+YZTjcdoPj2PxokGMIyCKNJPhQaI4jyKZomqQ5sm4IAxDgaJmnSQg+gEA",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileDigit,
    #[cfg(feature = "file_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKicIAxDgdgtDaU46lqWYFgeCRtDmX4PDMIJwDMLQzmWOxoD6AQ",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileDown,
    #[cfg(feature = "file_edit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQzC4NRWDQYQyCCFQwCCGAxhYLQyGgOIQEwMoYDeEBWiOFIWhmKwxh2FhoC0NYQCIPg8C+A4FjUcBvGweRsGkbhlCCO5AHQc4JDGDIVkkIA4haGA4jSNo7j2P5BjqBIGgiCgxDALg0ksMguDYMYUC6G5ihuGoNhYLg5DebZvEyJQ5DWFobkoMhsm4OQtg+dQgjINAzjGXw0FqUo3lkPoBA",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere"
    ))]
    FileEdit,
    #[cfg(feature = "file_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANhWDQYQyCCEgwCCFQxhMLQyGgOAuDUTAyhUN4eFaIYRhOFopDGGoTEgNAiD4PAvgOBYxHAbxsHkbBpG4ZQgjePB0HOCQxgyEpFCAOIThUOIwjKN45juPY2gSBoIgoMQwC4Mg5CCWQuDeEQuDQM4TmOZYVmmGguDYNgth4Mhjm+XAuDEMpvDUNguDObw3h0NQzGybwzDWew0oMNaDhAMpnmaZIpmqjA2omHgznKe51neHp6nyYA5pWbw5oUOaHhijKAlsIAzmANBMnoNZeDgbKrm6qw1DUYwxlsLa6DKGK6kWa4dDEOQtqsNw0FqTozlUPoBA",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FileHeart,
    #[cfg(feature = "file_image")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOBjGkchjGyRRjHmSwzCIIBjHiSwwmQcoJDKUwvlqXJej2BIGggIhtDKGAxDcLQxC4MA5n2fw5imFYrDCGwuDihoyiSbY4jqdA+gE",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere"
    ))]
    FileImage,
    #[cfg(feature = "file_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgqFAxDWEAwkuMpSlGBYHgkbQ5CAMZHDMLZnmmW4zGgPoBA",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[cfg(feature = "file_json_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgqDAxDIYQxCCWophYLZaDEdgxlmW4YhcMZfmWYJmmqW5imSXJsmCS4ylKUYFgeCRNkcMQ4nCbJnl8dpfn+XZgmma6Gl+gqEomZqLDGdIzGgPoBA",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson2,
    #[cfg(feature = "file_json")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKDGGInGEMQgmKKwwC2Ygxh6YZjiKY5nmyaJtnGaQxmuZJymOU46lqWYFgeCYLDSYw4nabZfmcdpnoWK5om+caMmeiKKo+IqRDGeo7GgPoB",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson,
    #[cfg(feature = "file_key_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwFYNBhDIIITDAIIWDGFAtDIaA4C4NRMDKFg3h8VoihKFIXioMYbhQSA0CIPg8C+A4FjIcBvGweRsGkbhlCCOI9HQc4JDGDITkYIA4hSFg4jGM44jqPI+jIYxpHIYxsj8Yx4gmMAgHKCQyCIIBjHmRQ2k8L5WliWo3gSBoICIbYOg0MAtDSHwgnkNZqjUaJvgWB4JG0OYNhmiJ+nAPoBA",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey2,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgYxpHIYxsGUIBjHmCYfCKRB4kcMJJHKCQyjeOY+kCQo4juBoICIbYfCAMQwC2DYPmKUo6gQaJXmeB4JlyDwxDGXpemWWA+gEA",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey,
    #[cfg(feature = "file_line_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCAiG2HwgDEMwtDONJkDWG4bEyTwxDeU46loPoBA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileLineChart,
    #[cfg(feature = "file_lock_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANRWDQYQyCCEgwCCFQxhMLQyGgOAuDUTAyhUN4eFaIYRhOFopDGGoTEgNAiD4PAvgOBYxHAbxsHkbBpG4ZQgjePB0HOCQxgyEpFCAOIThUOIwjKN45juPYxHIZRjHQIB5kQMwiCAch4kSXR3GkZIFgmTQgGgZRpGcaB0gkNZdmAIgyk4L5VleNoEgaCIKkoMQzHaGonhQIIYDALYMDAdp1jGM57D6AQ",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock2,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgchlGMdAgHIeIJDEIggHcaRkgWCQ4keRAik4IBoGUaRnGgdIJDaRx5kUMo3jmPpAjiO4GgiCgxg+Jx2hsYQzCCb4YDGGg2hcdpfjiOoEGgPoBA",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock,
    #[cfg(feature = "file_minus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgoMwgDENRoDaS4ylIPoB",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileMinus2,
    #[cfg(feature = "file_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOJXkUeAyksNQiCAeQxl6YB4mMIg5mAeZdCIMZfjgL5aD6AQ",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileMinus,
    #[cfg(feature = "file_output")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgqFAxDWEAwkuMpSlGBYHgkbQ1CAMYaDMIJnmmW4zGgPoBA",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileOutput,
    #[cfg(feature = "file_pie_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGEMoOCAMIWhgMgtDIVg3C4NRMDENIfg4SA2hOFYXiqG4OHYMwiD4PAvgOBYxHAbxsHkbBpG4ZQgjePB0HOCYihWRQ4g6Fw4jCMo3jmO49jaBIGgiCojDANAgDEMQuDcMRhDULg4lmYZjhgMYYDgLoUmoMg5kyM5TlKBYHgmCwzmIM5aDYQZlnqfoYioIJIDEMAuDENxWgwaJ+FqcI0GgPoBA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FilePieChart,
    #[cfg(feature = "file_plus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgoMwgDENRoDaS4ylKUYFgeCRNDaVwyHaW4wl2NIB",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FilePlus2,
    #[cfg(feature = "file_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOJXkUeQxkuUggHgMpLDIIggHmYgiieZR4l2aZkjgL5almQ5FmGSw1mubQ5mWXJ3nyaAxnicJygEA",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis"
    ))]
    FilePlus,
    #[cfg(feature = "file_question")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjiO4GgiCgxhiQguDMYwui0LoNDULQuDgLg5C0MYUC4MYQlWIgglYMguDaSpHDOSoOk6NAxkWGZmDOG4QmsMo3jmPo9gSP4JguFQxDcaAuDAMZvjqcw+gEA",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere"
    ))]
    FileQuestion,
    #[cfg(feature = "file_scan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAVg3C4NRMDENITCAMhIDYYQyhkIINiELYeDIdgxDYY4NDELorDmH4kGiFw1CIPg8C+A4FjUcBvGweRsGkbhlCCO5AHQc4JhaH5JDiGYNDiNI2juPY/kGOoEgaCIKieGQyh2H4hg6I4jlCN5XlaBYHgmC4NDKXYkiCcJsmONZljmApXmmCoMg4NJeh6YAxi+ZI4GiZ5YmqW4Wn6cYgiOGaDmaAQ",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileScan,
    #[cfg(feature = "file_search_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOBjGkchjGyRRygkMoOCIIBjHmS4NDWZBjHiSwxmOOAvlqXJej2BIGgiCgxDMLgyg+H59n+f5SnGOxoD6AQA",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "karsa-mistmere"
    ))]
    FileSearch2,
    #[cfg(feature = "file_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DMIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgoNQgDENxhDMIJbhcMYYDALQ2lyZIpmAIA2HqS4ylKUYFgeCRtDmVw4C0MYfnaH5rjMaA+gE",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileSearch,
    #[cfg(feature = "file_signature")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDkLg1HaERhDIIIWg2DQxC2FgyEgNoVhcIIZg6HIcFYNIhhiI4OheHBoDiERMDEOAgDWEQiD4PAvgOBY6j2BoIgqNY0GgMY5juQI/gSQYJE2NAuDSFoQDYMYVC4MYXliLJZlkMguDkN5amGMw2mANYOmIMQzg4OBsmAOQtDOZwgDSUQzC2dg0DQWpIjyTA+gEA",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FileSignature,
    #[cfg(feature = "file_spreadsheet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKk8MQzGgMpTjqWpZgWB4JE2Xg3mGY47GiZpbmmTJfm2OJkjyApamiCp0myYp3m8PoB",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere"
    ))]
    FileSpreadsheet,
    #[cfg(feature = "file_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyHYNRoDUIg+DwL4DgWFYYgaCIKDIMQgDYdg2C4NRjDAIAuDgLQuDcIAxiULYwDWL4lGgLQ3GOLA4CAMIyjGLY/DWQh2C0OYnjuQYzjWQ4zEgMQ3GwNAgDQeoUhaG4agSHIJE2Lg4HYOIqieKQzC4MguiQNIqmebQ1C6a5iDST4ThWF5blqBYHl0M4vg+Yg4mQLpmmiapsmiaJvnGcJPDGV53hmAQ",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = ""
    ))]
    FileStack,
    #[cfg(feature = "file_symlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DcIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoICIbQxhcMZHDMLZbl2S4ylKUYFgeCYLCCWR2C0MYnhSKZng2GxoDaX4zGgPoB",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere"
    ))]
    FileSymlink,
    #[cfg(feature = "file_terminal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCAiG2T4fhCG5iDKU46lqWYFgeCYLhUMQ4GgNJljsaA+gEA",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere"
    ))]
    FileTerminal,
    #[cfg(feature = "file_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOJXkUeQxksMwiCAeQyl6YB4mMIpSCAeJdCKH5TC+WpZkORZmgmaZrksNpgmKSw3nubAxn6OJwnOcpEmGbA5mWgAwnuZ6KmqZ5SoOcYBA",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileText,
    #[cfg(feature = "file_type_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoIgqFAxDMdgtDEaA2HYMZLjKUpRgWB4JgsIAxDgaAymCMxomOU5mDWaAyHYNptmKAQ",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType2,
    #[cfg(feature = "file_type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKDkIAxDMdgtDEaA2h6U46lqWYFgeCYLDGXw4GgMpojsaJrlubonl+HQ2nWaoBA",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType,
    #[cfg(feature = "file_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKicIInHYNpTjqWpZgWB4JG0MYPmsLQzm6bggDOY47GgPoB",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileUp,
    #[cfg(feature = "file_video_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAOBWDQYQyCCEgwCCFQxhMLQyGgOAuDUTAyhUN4eFaIYRhOFopDGGoTEgNAiD4PAvgOBYxHAbxsHkbBpG4ZQgjePB0HOCQxgyEpFg2E4VDiMIyjeOY7j2NoEgaCAiG0MYXDWHggkaHh2C0NhsC2Xg1k2M5UjEchlGMdAgHiCQyCIIBoGUaRnGgdIJDacx3GkZIFgmTAgHmRJyCAcpwCIMZnmubQ+gEA",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVideo2,
    #[cfg(feature = "file_video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCAiG0MYYDEMQgg8MwtmQdgtDYWpTjqWg+gE",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere"
    ))]
    FileVideo,
    #[cfg(feature = "file_volume_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSPYEgaCIKDEMY0DEM4OGMLgzDILoNDULg5mkIJfDUcwtC4MZPl8MZyg+b5TjqWpZgWB4JgueQymQNoNnWbguDgN5uCAM5xmUNoQC6LQxo6e47GifpboGdQ1GgLgwDGmJ9gEA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVolume2,
    #[cfg(feature = "file_volume")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DMIg+DwL4DgWMBwG8bB5GwaRuGUII2jsdBzgmEYVkQOINhcOIvjGNo4jqPI1gSBoICIbQ3CAMQwC0M4kDIdg0GgMhslyXgtDgWpLjKUpRgWB4JE0MQxlgMRjC4Ng0C6R5yDGeZXnIMxzC0LgzDaDQuhqf5pjMaA+gEA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere"
    ))]
    FileVolume,
    #[cfg(feature = "file_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjiO4GgiConCAOR2DSN45j6PYEj+CYLhUMQ3GgLgwDGR46ksPoBA",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere"
    ))]
    FileWarning,
    #[cfg(feature = "file_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3C4NRMhGH4NEgNoThWF4phuDR2DQIg+DwL4DgWMIzgaCIKhGLA2GgNovjGNo1gSN4JG0MwgDEMojDUIA1j+MpDkKBYHkUOJIkoNQtkyTowlCNIB",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX2,
    #[cfg(feature = "file_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkSOJXkUeAxgkOYOCIIB5l0IonmAIB4DKS4NDWYR5mqZYwm2OAvlqWZDkWY5LDKZ5cmufZwl+bZinAMZylOdZ4D6AQ",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX,
    #[cfg(feature = "file")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsVg3g4TIMg6EB6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JgyFpMDiEIYDiN45j6QJCkQPoBA",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis"
    ))]
    File,
    #[cfg(feature = "files")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg1CAMhIDgLg2GMLQuDQIAwheEwyC0MQuiANYXDMLgzheDQ3igIIgDEdgxDILg4GMMAghgLoxhOD4tiaPQ3g6LIhg4aA5jIY4YhqNg4hcMpBDGKImiSDoXj+I4tFYNoOEyDJADIegiD4PAvgOBZhmSBoIgoMwgj8NovjmNI2DSOIykCPIliWVZOkORQ4mCYpnmaBJogmC4PDIdg1GgNZ/mOgw+gEA",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis"
    ))]
    Files,
    #[cfg(feature = "film")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwaBlGkZxoHSCQxDiDB3GkZB0GiHYfD4PAvgOBYoHAYYkCAZIJE0NwgDMdoeCKKAvi6JIti8aIxjMMwgDcLg1GgNI6imPRoj+MIyCITZEDEMhojmO5Nk+QZRlMIAxDaR5JkuPJAluQpSDGNY3liTJmDyTZoE2apFmKSpZm+cZdnSYJ2mSWoBA",
        categories = "photography,multimedia",
        tags = "movie,video,reel,camera,cinema,entertainment",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Film,
    #[cfg(feature = "filter_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLgwgwIAzEgMhsDgIA5C4NA2FYMQ5GwNAgDIdgtDgLg1DQbAuDkLQxg4NQ1CIPg8C+A4FjKNYGggIhtDIMoRC0NQgjCMo0gQaI3kaB4JG0MQ3hGQpCjGM44D6AQ",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "gubser,ericfennis"
    ))]
    FilterX,
    #[cfg(feature = "filter")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgyDIIAzCCE4VDEMAgDEMguDQNobhoMQ5hsNIWDGJYbh2H4WhcIg+DwL4DgWBxuD6AQ",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "colebemis"
    ))]
    Filter,
    #[cfg(feature = "fingerprint")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyEODA2C4NQghKFIMg4IAyGEMQwg2Hogg0IA4CANAiD4PAvgOBYoiuBoIgqFAxDmExDDWE4NiQNoNhSO4OGMMAtC4Nwug4LQxC4M5EDMNAtDKJ4pi6LYEi+CRNDGRAyDmGpIDAMhjkUMpChINAzk6SYTC0Mwul6UIqlSU4FgeVoZh0YYMgyIZBniQINmyYpFhqEwxkIMo7iaKJviyApUnOCg4C4NoXl8LgyoSkZkDWR5JDKEw3k6bpSoycowlcNINmuP4ep0M4kh6EqtkeIwuDgOKhnCo5VgqGA2GibAxrei4uo6C5IiQMQ2mCYoMkUM6EjcMw1qeQQ2sEaJxroTZbhIOBhjuO4hDEIJbjeX4eC4NA3CCHpIliQpehq1g+gE",
        categories = "account,security,medical,devices",
        tags = "2fa,authentication,biometric,identity,security",
        contributors = "karsa-mistmere"
    ))]
    Fingerprint,
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
    #[cfg(feature = "flag_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhjDMIAwCANYNCCDAyHMNAtDEIIaDEdgxDEIg+DwL4DgWJIngaCIKDSDQyFYNIjiWKopgSK4JE2LgxDUcwxhuHZAhQMoWg2M4mjeJBsGkbhlCAeAygkMgyCIIB5DGUpVHiWAilSVpRl2VIkC+S5ND6AQ",
        categories = "account,social",
        tags = "unflag",
        contributors = "karsa-mistmere,cyberalien,ericfennis"
    ))]
    FlagOff,
    #[cfg(feature = "flag_triangle_left")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAyDIVgyEyDQ3GwMQwCANQiD4PAvgOBQ+gEA",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleLeft,
    #[cfg(feature = "flag_triangle_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgyFYMhsDEMAgDULYThUIg+DwL4DgUPoBA",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleRight,
    #[cfg(feature = "flag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ1HMMQtDEIA0hIIA1CAMggDiGYUhaFQxFYMxzhaEYMhENQtDILQ4ioLYni+DR6CIPg8C+A4FjUbBpG4ZQgHkMoJg4IggHiQQiDSRB5DGCQyDKRB4kySI0jaO49D6AQA",
        categories = "account,social",
        tags = "report",
        contributors = "colebemis,ericfennis"
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
        svg = "gAPBwGEdBoCAZA9CITQ4C4NQgDENINEEMoNCCFIODAIIZhkMQxg8MhjDALQxC4Mw4C2DQtDKIwtDOIwuDAN4rhSEIuC4MgyDQLYRDANQ0hYLQ2CCFYXhYIIRDmRwgDaRIPC6QgzCAM4VlENYNGENwglmHIPiOPwwiGLwxDWUw0DOLoUDIOY/DGLRhkWRYbhqFpEg0egiD4PAvgOBQ+gE",
        categories = "weather,social,gaming",
        tags = "fire,lit",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Flame,
    #[cfg(feature = "flashlight_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMHYNBhDIIITDAIIWDELYTDIaAthGG4XiGGQyhoVgxDAYwwhqK4kiQNAiD4PAvgOBYxjSBoIgoN4UGgMQxhCKYUhqQoTDQdgxjCMo3jEbBpG4ZQgHkMYJDYIpRDKVJWHiUwij6WpYl0OJJC+TZPkyTpQluCQylaUprm2YAymwIB4nGbIxmSaA+gE",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2GMMAgDILQyhKFAgDQdgxDAYYVhWEYRDGFwyGgLQ0h2EggiAIIihMMhWhuEIUjOLomFaJAxDIegiD4PAvgOBY9GwaRuGUIB4DGCQ2CIIB5DKSpMHmSQikuR5PCKDI8j6Q5FkKRJGlKCY5lGV5jkeU5mHiZQyloL5cGUPoBA",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMCANAuDcMggDIMAuDUNRhDGDggg2HwgC4OYOC4NA1GgMQyhINobh2IINiILQxiUNRsjILgyhMLYqDUNAyCIPg8C+A4FkGRIGgiCoPDIdoqDMNAzkCQpHkaBJIgmCw0hUdg2C6T5RkGQ5WlWBYHlgOIYhUaA3lKYpFgKVpmgoN4ODYaA5m2VA8GwaRuGUIB5DKCQyj8IB4oIIqECKhgxoOix5o2iZtnyfg+gE",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,non toxic,lab,chemistry,experiment,test",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FlaskConicalOff,
    #[cfg(feature = "flask_conical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYNwuDUMg3GEMoOCCDYNDELQuDIMQxC4OA5DYTA0C4N4XDIMISDUYQxCCMIahkIAuDmMQuDQNRoDEMooDaL4xjSM42C2IY6GwLQ1C4MA2DmRosj0NxBiqQ5CDENAgDmEoUFYMgiD4PAvgOBZhmSBoIgoOISg4aA3mCYpnmaBJogkTQ3jENo8DCcJjnQPoBA",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    FlaskConical,
    #[cfg(feature = "flask_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyHYNwuDMMQiD4PAvgOBYWhmBoIgoMQ0CAOYSFYMQuDkOYVheHIbgSHYJE0OAuDWDhoDeKoYi6LYFgeMIgiKEhhDaMwgkONINDEIAxC2IQwjiLICi6PYKDWMwykoNhoDEMonDaT46gEA",
        categories = "science,gaming",
        tags = "beaker,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,danielbayley"
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
    #[cfg(feature = "flip_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMxIDUYQyCCEgwCCFQwC2EgyHYMQ0GOFQxC6IQ5hOJQyGgMwiD4PAvgOBYri6BoIgoMQ2g2KIRiWFwgDGJocDSOYUhaPIZhMaAtimK4tgQaIwkyB4JE0MYaDAdgyiqLIxk6BZQjSEodlaWJLi+ApPjOUoSDiYZKlqZZcmeU4TmuWZMD6AQ",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal,
    #[cfg(feature = "flip_vertical_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAzC0NQgDWEIQGgMQwCIPg8C+A4FhqHYGgiCoMCAMgxhSE4RDWFoYhqHIEGiH4wgeCRNDQIAxDISAyhmG4gjKBY0CITYXjiOg4j2L4egKM4ikQNpGGgLY8i6P5MkGTgyDKUZTkmP4B",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    FlipVertical2,
    #[cfg(feature = "flip_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA4FYNRhDIIITDAIIWDALQyhoSIRhOFYXheGoUHYMwiD4PAvgOBYoiuBoIgqDAgDENolhKFIhhYMYjDKHY3iCOoahodgtiaKIqgQaItkmB4JE0NIzj0MonimLpLgWTYKDGOo9DiVJIiyApMjATY0lEaIal+VpiliZAyhMMQymiU5HlaAQ",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipVertical,
    #[cfg(feature = "flower_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GEMwghEMAgDGFYShIbQtDOG4QhiFIWDCG4ZhwMx2DETQ5CAOIehOF4UhEM4pisaAxG0NQgDCLY5hcMYjDMbYlGgLY2C2DYmkQIg+DwL4DgWSxjGkchjGwZQgGMeIJgwIggHKCQylwYx5gkOJKkyUZTlWS5OgaCIKgyFQwieYJLk2BBomud4HgmC4NDIMhjDQLoNhQN5EC4Ng2DcIKGDULaCoQLaLDGiKKpKDhamadpPgKepun0IJ/GOj6DjmkqHomhqNCCkI8pOlaLosNaZnWbA+gE",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3C4NRhDSEAghMNQgDAIAxhqFYUhaC4Ng8NRBhaHYXhmG4ZiKGgyiCDoQFYORtC2JQzhKHoUiiGImiYTYrgwSIyisMI3heJY6DGNI4DUbYWkoNRIDGTAtDOJhWlIIg+DwL4DgWWhjGkchjGwZQgGMeIJgwIpmHmaQymscoJDOWZbmCYpklqXYGggIhtDiGg2hqEAtDGEJ0lyBBonmiYHgmC4lDmFAxoEOKHnqi4Fo2fZ/n+hYXp6lqJpie6ODGJampKgaTqGXoBA",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwM4QGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTQzCANxWDUYQyCCMAwCCMwxjELQyGiD4bC+IokiGI4licIhNDENwgDOOovjGNJNjaMAyHaO4cj4aJAiSJooDKNpGlKS4yk6OIxGiOIalSQZXkKKJHlsSIulCTY1jiOB2mWPJVD6AQ",
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
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldHorizontal,
    #[cfg(feature = "fold_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDYIg+DwL4DgWFYYgaCIKgwIA4FYMoUhaG4agSHIJE0NAggwSIjhWF4oieBYHioMQwi0MhIDiJIyhmAoojaHg2joaAtjCJYzkGNYdE2D5GkiPomkyKQiG0MQ1i0OQtDOXZdCAM5TkuG5DliWg1mCXprmOGYBA",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldVertical,
    #[cfg(feature = "folder_archive")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDENYOGMeYSDkIg+DwL4DgWB4bHAYR0GgIBkgsTQyDALg5CAMQ5C4OBBDIII0DAII3jcMo0DEOBWDgYY0jaOI4C0MpGGgLQ3iyQY1kSNwxC0MQuDYOQtiwTIwDYIAziyM5OjmRJLDkM5cEgNJNkOOZGjUdgxDOaZPkSQhoDULgxhqHIiiSIYjiWJwiE2E4uDEdpSnkL57GifYkiaKKDDEN6Gg2G6Jn4PoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ2CKCR5gyDoQHKDIPD4PAvgOBYHhkcBhHQaAgGSDBNDcIAyDASA0GEMopCAMIxCAMQtDKNhWDWLowjKMgximNhoDMLg5juL49jSNAuDYOZEGwLg4j8MQuDKRozkiUw2DeRBIiqVpYjAMgihkL4giKH4hiOJQiE2Do0DQdgyGyUpjhqZhoD6AQ",
        categories = "files,time",
        tags = "history,directory,clock",
        contributors = "karsa-mistmere,jguddas"
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
    #[cfg(feature = "folder_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CKCR5gyDoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MQwC4MwgDIMBIDQYQyi0IAwjUIAxC0Mo6FYNYyjSNo2DGLY6GgMwuDmP4zkGOI4C4Ng5kgbAuDiQwxC4MpKjeTJXDYN5IEiLpalyNAyHaR4YhoL4hiOIIiiSJgiG0MpXDeOJRDQLZInqaYbmwaJuiOJYMG0MQ1liOA2nui59mub6BnChAxooNotnWK56DmGZ+o8PJ/oOcgxlGVqHDKmJIpujptp6b6goWUaVnQLg3nqeQxqmf6Qq6k5UjihwzrULa3mquasoKcaFDSK53k+OK1rinafsiLqzomvY5C4NLQm2AQA",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FolderCog,
    #[cfg(feature = "folder_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlkYxpHIYxsGUIBygkMQiCAYx4mcMppGMeZnDOWJal+YZjD6AQ",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
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
    #[cfg(feature = "folder_edit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NAgDEMAuDYYQyC4MQghWFwwg+HIVDkOYYC4OQ4EwNoPDkbAtg6FwxC0M4iFoIg+DwL4DgWM42gaCIKDKDwxC4NRWDWFIYCCG4bhcMgtDIaIvDmRI9keHI/DYOYiGwLg4iwLgylCRpfkiEg3iISAyDCXpSkmGB2hCaJfi2PZMC2Vg1jKNI5D6AQ",
        categories = "files",
        tags = "directory,edit,rename",
        contributors = "karsa-mistmere"
    ))]
    FolderEdit,
    #[cfg(feature = "folder_git_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgwEgNBhDKDQgDCFQgDELQyhoVg1hKFIWhYMYNhoaAzC4OYfhOIYYhgLg2DmKBsC4OIjDELgyiqF4sjcNg3igSIOjqPIUDIdg1CIPg8C+A4FkoYxpHIYxsGUIBjHiCQxDMIggHKCQylwYx5lmYJKC+UJSlSSpNgaCIKDEOIYDkY4ajSFQtDWdYbnkNR2DiSZLmyT5RlOVZeCKYJWliiAwmGYwiDEOaAmehJqgE",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeYLDEM4OGMeISg0Pg8C+A4FgeGhwGEdBoCAZILE0MgwCCKRhDKKwgiqMYrC0MhWDiLYvjIMI0jQaAtDcLg5jiLoyDELQxC4Ng5C2QRMDmSQgDOQRBi6RIwleQA5DOURIDSQ5XjGNIrHaE5fjqLwyFoIoaC+IYjiCIokiYIhNDENAghMaIUmybhonCI4licN54DOeprhufQ+gE",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit,
    #[cfg(feature = "folder_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAyDASA0GEMoOCAMIWCAMQtDKGxWDWE4VheF4NhwMhoDMLg5iCFIihmGQuDYOYpGwLg4g0MQuDKK4Yi2OA2DeKRIg+O49hUMh2jgNQiD4PAvgOBZMk+BoIgoMYoDmGZADQNRjC2OIcl+XguDENIbjULY5C2KA3DOEwuDQM4Om+cZFigNA0haNAzDYLgzDSfZloCbpwnKhJ1m8NZoDAMR2DAY4pDWkIuDILg1DOaIUmsNBMDGQA1g4MRakuTZSD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMgwCIIBygwMoQGMeIMDENgiD4PAvgOBYHhwcBhHQaAgGSDBNDEMAgg4SA0GEMotCCLIsDELQyjgVg1jGM41CAMYtjgaAzC4OY9jKP5BDELg2DmRhsC4OJLC4MpIjSWI2k0N5GEiDpXkqMwyHaFIcC+I4liKJImigIhtDKMgxDQLQ0C4NQgnUNYbh2aBomqJYngyb5LneS57meaw+gE",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMQ0CIIB5gkMQ3g0dxpGQdBogkOINHKCIKg0aBlGkZxoHSCQ1CIPg8C+A4FikcBhhcIBkgkTQxDAIAyDASA0GEMo4CCN43DELQykQVg1j2P5BCAMY4kQaAzC4OZJj6S5NDELg2DmUhsC4OJXC4MpUkCZJClkN5SEiOZjlaPwyHYMguieKQvi+F4ujAaIyjSOZMDcdpEmyTJAC0NJAnCKIqnYaA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ0g0Lg0DULQyC4ORBDIIIZDAIIcDEIA5C4MoPDEMBIDIMBhhmG4dg2EA5g+FQ1GwLQxC4NQ1CANoqhqLYejWFokjcSA0jyLI/DKFBWDUYwwkCNg5hSGoUGgM4WDORo+i6Ng2DaFhsC4OIZjYMpZhyZ4Ql2FhIDEOJmi2H4rHYMgiD4PAvgOBZ2GMaRyGMbBlCAcoJDEIggGMeKEDShhjHmhA1nWd58n6gA+gE",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
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
        svg = "gAPBwGEdBoCAZA9CITQyCANwuDUVg1GMMAtDELoWDkLYMDKGhoDMLg5DMYYbCAMIlCAMYoC4Ng2iAbAuDiDIWDKIwggyJo4iqLIgEgMgwjWN4nimGx2DGP4kjkMYajaPQiD4PAvgOBZPlKBoIgqMgzGiRpOlCVZUgSVoJG0NYohQMwgmiapdlGYQ+gE",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlkYxpHIYxsGUIBjHmCZOCIIBygkMpqGMeJom6WQvl+YZjlmXIGgiCgxhQMQ1HYNZYlqeg+gE",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = ""
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxC4NQigkeYMDEMoQhIcoMheEQ+DwL4DgWB4eHAYR0GgIBkgwTQyDAIItGEMovCCLo1i8LQyFYOIxjONgwjiOBoC0NwuDmPIyjYMQtg8Ng5C2RRMDkLg2CAM5FEGMpIjSW5EDkM5VEgNJHluNY4i8dgxDOY4+jMMhaCKHgviWJ4kiaKIqCITZpC6XwxDSfAgDENaBDacIfnMaA+gE",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ3CIIBygwM4QGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTQxDALg3CAMgwEgNBhDKLggDCNggDELQyjsVg1jONY3jcMYujsaAzC4OZAjSQo5jkLg2DmSRsC4OJEDELgykuOJNlgNg3kkSIvluXY1DIdg0C4MYahyIokiGI4licIhtDKRJ2C2WA1nkLg1mwL5uGgPoB",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch,
    #[cfg(feature = "folder_symlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORWDUYwwC0MQuhQOQtgwMoYGgMwuDkMxhhkIAwiMIAxiYLg2DaHhsC4OIMhQMohCCDIkjaKIqh4SAyDCM41iWJ4ZHYMY9iKNwxhiNI7CIPg8C+A4Fk2UIGggIhtDiJg2CAMwtlyXpMk6U5SgSVIJguWR2hOPoliSQYbDaYJPmQPoBA",
        categories = "files",
        tags = "directory,symlink,symbolic,link",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSymlink,
    #[cfg(feature = "folder_sync")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgwEgNBhDKDQgDCFQgDELQyhoVg1hKFIWhYMYNhoaAzC4OYfhOIYYhgLg2DmKBsC4OIjDELgyiqF4sjcNg3igSIOjqPIUDIdgxCIPg8C+A4FkqTYGgiCgxhMMQwHYNBoDSSZLlCT4ElGCRtlSGA0i4NQtjcNRjigLYog2OJpC4NQgieaJqHMMovi+dZzmebA0C6gQ4i2NpzlyTJgl+BYHgkTQyhOkB2C2WaUoiXoCmCjQiG2kIYDicp0moY5ujEOYaC6hponaZxzqgNpurCdqhqSgpuDSbqghmu6HkqiZOgEA",
        categories = "files,arrows",
        tags = "directory,synchronize,synchronise,refresh,reconnect,transfer,backup",
        contributors = ""
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
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDcYQyCCEoNhWEwtDIVg5hGEwghYMIYhgaAtDMLobhKFIeg4LQxC4Ng5C2JhsjEOAxiwLgyhyKYVjcNg3jEORIDiOoqjyKB2ieHYfh0MhaCIPg8C+A4FlCU4GgiCoSDgdgxDGRJLigaAxDST5RlYPoBA",
        categories = "files",
        tags = "multiple,copy,directories",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Folders,
    #[cfg(feature = "footprints")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ2HYLQyC4Mw4EODAxDELg1CCEg5DeDQwhoIAzCAOBjC4MAzhELg3DKDQuDQOQtDYIA0hqMxDDmE4fi4MQwiMLg4iAIA1hoY4/DMLoYhGRAuDYNpMDiTg4FaDhhi6Lo/DGDQtgwMBaCIPg8C+A4FmKZYGgiCgyj+bIQhKFJHisMY9iiKpJnSXJFDYMgtnaK4tC2GYxjOXY3DYQwxjYNokjSiggDeQYNgyOg1keQIYhyTZPpqUg2lSbJXpqWggj+XpgmKZIEGiZ6rgeCRNg6DQ3GgNJhmOaKtgWr4KhcM61reqpmgE",
        categories = "maps",
        tags = "steps,walking,foot,feet,trail,shoe",
        contributors = "karsa-mistmere"
    ))]
    Footprints,
    #[cfg(feature = "forklift")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMEgNRhg2DQwCCFQwC2Ex2DUIg+DwL4DgWHhjGkchjGwZQgGMeIJDEMwiioeYtDmMBygkModh+JImiiI4lieKY2CKOIqiwIocjGM45C+O5Ah6IYGgiCg4g4ORoDMbQ1C0MQ3HaXBoDYTQ2g4MhWDcY4YDELpqDmGQgDKGZXGwNQghyHoggQaA+gE",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[cfg(feature = "form_input")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoIgqDB5gkNoMHcaRkHQaIJDIMIMGgZRpGcaB0gkMYLD4PAvgOBYoHAYYZCAZIJE2JggiYaAuDAMQiigL4uhmLYvGiMYzDEN42DKOI6jyKY/GiQYwjIIhNkeN45juPZOD6AQ",
        categories = "text",
        tags = "2fa,authenticate,login,field,text",
        contributors = "mittalyashu,ericfennis"
    ))]
    FormInput,
    #[cfg(feature = "forward")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDUIAxDcIAyDCFAyhSEw3CIPg8C+A4FgeCYgHAYR0GgIBkhATQ0hQOB2C0MhhjCMIZhkMQgDQLQ0GgMQyh+IYoioPoB",
        categories = "mail",
        tags = "send,share,email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Forward,
    #[cfg(feature = "frame")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgyCIIB5DGCg2g0eYJCKEoHhCC4MD4PAvgOBYch+BoIgqDIOhkMQ4g0eIZDKJoUgqKQihyHoEGWIY2ieJYThWLorhWF4shGM4diKOIFhiMYqjqC4/kqPIlhuRY2D6AQ",
        categories = "design,photography",
        tags = "logo,design,tool",
        contributors = "Bowero,karsa-mistmere,ericfennis"
    ))]
    Frame,
    #[cfg(feature = "framer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ2FYORoDENBWDISA1GyE4NDQaAtDcbYeCAMAgDeJB2h6IIlDAaA3CIPg8C+A4FD6AQA",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Framer,
    #[cfg(feature = "frown")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQxDYIIqHMLQxC4NQtDILQ0jONQgjQNI5CKGwviKJIbGwaRuggeAxgwOYQHmSAikoIB5DKSYQHiUpOC4MAxj2HJDkWQpEkaTQxDWVJWmOWJalCVpPkyU4+l0ZQ+gE",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgzCIIB5DKCgyDKDR4hAIgxDWDR5gkIoSCIPg8C+A4FiCI4GhuCg5hqFoqgeHA0hSFgxjCIIigQZYgHAYR0GgIBkgoTYzCCEhWDQYQykMIAwkqSgtDKThIDaR5JkuVZOkMdgxDiH4hjqPI5juPY/CKQQ0CAMQzGgMpTkiVZnkkMh2muSJtkyS50GgMJsnaTJPDIVg5C4OAznubgwC0Lg1DkLQxC4NAyEyWgghmNZeGgPoBA",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[cfg(feature = "fullscreen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PByGUYx0CAeIJDcIggHmCQ4mgaBlGkZxoHSa5oHKZQiDGaB3GkZIFgkMQwksL5gmIPoBA",
        categories = "layout,multimedia,design,photography",
        tags = "expand,zoom,preview,focus,camera,lens,image",
        contributors = "danielbayley"
    ))]
    Fullscreen,
    #[cfg(feature = "function_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch5g4MoSHcaRkHQaIOhCEoVCIM4SHIeIWhKJoiCIPg8C+A4FiwcBhhwIBkg4TQ5CAMQ3GMMggDAIAyC4OAtDGQZDC2Qg4FYMQwGMMJJjoLQzC4MwglQMpTiuLYyhyMYzGiNY3jkMQxC4MhoDULg3lsL5dGgPoBA",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANx2DEMAiD4PAvgOBYThaBoIgoNggDWDw0hKFIZhMchlGMdAgHcaRkgWCQxDIIggHmCQzjIaBlGkZxoHSLw4jIeIvhEIBykEIoxhML4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMx2DEOAiD4PAvgOBYTHIZRjHQIB5gkMwiCAch4gkMohHcaRkgWCQxiYIBoGUaRnGgdIshEIIkCINoShSGYbhOFoGgiCgygyDoQjyFYEGgPoBA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIBoGUaRnGgdIJDENIMHcaRkHQaIYDiDByg4IgyCIPg8C+A4FikcBhh8IBkgkTQ0CAMgxGgMYoiqL4fi6MBojKNA5jeOY7ikL4+GiQIxjMIhNhmRo6jySpBk2QpPlGRY4lSSZLD6AQ",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMhoDEMAiD4PAvgOBYThaBoIgoNQgDaDw0hKFIZhMchlGMdAgHcaRkgWCQxDgIggHmL4RCAeIJDOMhoGUaRnGgdIvDKMhyjgIpDhML4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMhoDEOAiD4PAvgOBYTHIZRjHQIB5gkNgiCAch4gkMohHcaRkgWCYQiEaBlGkZxoHSLImCCJAiDOEoUhmG4ThaBoIgqDAyg6LYThWBBoD6AQA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDEIggHgMYKDaDh4gmCwwg4eYRguDQ+DwL4DgWHohgaCIKDEM4UhsOIUhaLIHhsOQih6IIEGWI42g+GwxDWLYnDULgwg2MInDKGYWDGRo0iSOIFjqJ4vhqJ4YgeSJUhWUJBh2H5MDwcBhHQaAgGSChNDENwuDMMggDUSA2C4Ng4GENAgnQMAgncMAtDMLg5DcOAgnwNQ5GMLZBDCbwwDUMqGkILgxkKjZno+ixDDKcAwnQOQuDQMQ2CCawxDSnA1p+oQ2GEM6BnirJ3qoMxjDGrAxC4NaGDWoAtDEbK0p0NK6pyohBmua55CCsqbDgMqAp4aKjDMNA0GGxKtsex7BqMNQ4DYTJnscOBjrWtbWrKoapquxqunsY56rQNQ0ramK/m+2q/nAOK2mgMrao0MA3o28ZCDGhsDlq/6UDEQZ0na1aTmqbB6jOH5fmEPoBA",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDIIggHiCQiDaDh5DKCoMg4eIWgsMAiD4PAvgOBYfiKBoVhcNIUhEMYdg+Gw4hmEYwh+IYEGWJI2i6Fw1C4MAxiqFwzjGO4UhsMZCjSJY4gWD4rjCOoLDiPY/geK5UieC4/kmNofHIZRjHQIBoGUaRnGgdIXg0IByHiCpqm0IpqHmCoTCAdxpGQdBom6HY0l6YA+gEA",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gantt_chart_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoSHmDgzhKFQihsIB3GkZB0GiDoQCIPg8C+A4FikcBhiQIBkg4TQ5CAOBoDeKIqi+JIujAaIyjQOAgDEMhoDaOwvj0aI/jGMwiE0MQxkUNhoDWSpMD6AQ",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    GanttChartSquare,
    #[cfg(feature = "gantt_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANhoDEMAiD4PAvgOBYThaBoIgoNggDEMhoDmEoUhmGIEhqCRNDEMYeDgaA3iOFYnD6AQA",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    GanttChart,
    #[cfg(feature = "gauge_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg2CAMguDcYQxDAIIVCCFgxhkIINDeHYSCIPg8C+A4FiMYxpHIYxsGUIBygkMgiCAYx4gkMYyjQeY3jKIwvimK4tiOJoGgiCgxDMLg0hcMIOhcOYdiKJJED6AQ",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = ""
    ))]
    GaugeCircle,
    #[cfg(feature = "gauge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDQIA0C0NAiD4PAvgOBYWhmBoICITQzC4M4QDEORhDEMIOimKQxg6Dg3iKDQwhWF4cD6AQ",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "ericfennis,Andreto,danielbayley,karsa-mistmere"
    ))]
    Gauge,
    #[cfg(feature = "gavel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIAxDMLQ3C4NQghINRjC0Lg4DOGoQDILgxhKGwtDMIAwiaKIniqJhhh8MQyCCLowisMYmiQTAxjUMQwCIPg8C+A4Fj6QYGgiCgxDaDpJDYLQ2j2P5EkOBJFgkbQ4CCV5Mk6PpAlOUoFgeVQ5hWWJYk+XZCgKU5hgoMo6DELQ4nGZ5RgEA",
        categories = "maps,tools",
        tags = "hammer,mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Gavel,
    #[cfg(feature = "gem")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMxoDEMhsDQIA2C0MQwCAMQzEwMggDkWgiD4PAvgOBYiiWBoIgoMQxg0IA4h6EoZDMIA0hYMwtjgNohiOKIngSKYJE2HQ5GgMgwjyJJAD6AQA",
        categories = "gaming,money,development",
        tags = "diamond,crystal,ruby,jewellery,price,special,present,gift,ring,wedding,proposal,marriage,rubygems",
        contributors = "connium,ericfennis"
    ))]
    Gem,
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
        svg = "gAPByGUYx0CAch4D0IgxCIIB5goOINgkIgzg0dxpGQdBogoMYRCAaBlGkZxoHSCg0CIPg8C+A4FikcBhhoIBkgoTQxDIIA4HYMYVikL4vhqLowGiMo0DEOQgjYdg3GGN43DAIJPDELZNEiS5NlCWJSDKUx2C0N4oiqPxokGMYzCITQ3C4NY4kyaggDKbpPlGUAtDUQQ0C4OI4lic42myeJ6nqcpICCeA1nWb5unCa6DDGWA1mCPpCD6AQA",
        categories = "gaming,account",
        tags = "present,box,birthday,party",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Gift,
    #[cfg(feature = "git_branch_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMx2DEMgiD4PAvgOBYThaBoIgoMQ4CAORhDODQgDAIAxiSJAtgyIoiiWLooDYeoShSGYYgSGoJgsIAyDGIYjiWJ4uiqI4tiiL4xjOFY3jaBYHjkMQ1CANhhDmH5GimVQ5kmNYCjeToch6UB2DaW5Ll2TYbE2PImDgaIqmWF4BA",
        categories = "development",
        tags = "add,create,+",
        contributors = "mittalyashu,ericfennis"
    ))]
    GitBranchPlus,
    #[cfg(feature = "git_branch")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDUIoHDGCgzg4eIJCINoThCFgiD4PAvgOBYcGMaRyGMbIGGMeYKhcIBjHiCgxDiDhyhGG4diKJImiGI4lieLoaiyKYLjEIIzCKEocC+N48hwcBhHQaAgGSChNjAIA5GEOZWCAMJbCAMQtlkOY1C+TZPD6AQ",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEMoOGMeIShQPg8C+A4FgeGhsGkboIHgMoLDmDh5DGGIOHiK4MimJgihMIoaC+IYjiCIoki8MQ1i2MgyDGKY9hQIB5jKNI2jgZQ+gE",
        categories = "development,maps,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommitHorizontal,
    #[cfg(feature = "git_commit_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYNgiD4PAvgOBYTGMaRyGMbBlCAYx4gmDAih8eYiDKJBygkM4ShSGYbh2E4WgaCIKgwIAxDWEIthWBBoD6AQA",
        categories = "development,maps,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "danielbayley"
    ))]
    GitCommitVertical,
    #[cfg(feature = "git_compare_arrows")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQigkeYMDaDxygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDEMggDYaA1GGKYpDAIIxDEIIvHYN4YhqIIih+IYjiUIhtDENQgDkLQzkcIJIheGQvjsaIZhyBoIGOEQiDEOIUhaD4KgwMQ5jmG4ElOPYiiSJooCCWBIDeLo1jKcAxC0MpzFaYJNk+ZY/gwbQ5mqRAzkqSZMjqPg+gE",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "danielbayley"
    ))]
    GitCompareArrows,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CKCR5gyDoQHKDAzCIPg8C+A4FgeGodgaCIWCKGIRgwNoQgqKIZhuIYfDwcBhHQaAgGSDBNDEMwgDYaAzGEMggkEMAgkQMZCkIdg3i0L4yjSGpOjWNwijmR4OEgOJAkiRJGC0MpeFYOZMlEPoBA",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ4hAcoMDMIg+DwL4DgWB4bh6BoIgqDA2hAY4SCKJwghcIoZhuHYEiOIYzgeCYLg2FYRiaFoYhqHIiiAPBwGEdBoCAZIME2FAgDkdgyGMMAgC4NgtC4NAgDELQxloSA3GOVw2CAMJclcNJmDEVg5kAL5FkeG5vkiSgikwMpaDIdowhycg+gE",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis"
    ))]
    GitFork,
    #[cfg(feature = "git_graph")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQiCAcoMDODxjHmDA2CIPg8C+A4FgeGhwGEdBoCAZIME0NQgDkdoYhoL4hiOGodgaCIKgyDoJhYIgxDiD4RCKE4ujOHw8jCJImCITQxDIIAzHaPIZhuRoygSNIJguOw5hSOoYhCEpRhyVZEkaJYnDENggDENQuDcQQ5ioIAwnGcwxm+WoulOAQ",
        categories = "development",
        tags = "code,version control,commit graph,commits,gitlens",
        contributors = "danielbayley"
    ))]
    GitGraph,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CKCR5gyDoQHKDAzCIPg8C+A4FgeGodgaCIKgwNoVheEBjhIIomhqHIEiKGhwGEdBoCAZIME0NggDIMRWDkYQ5CCQgwCCRZFkIOYZhuM41D6AQ",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQigkeYMDaDxygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDUIA5HYMQyhiGogiKGYcgaCIVCKF4JgsIgxDmDxjhGOw4i6G4EjSH4hiOJQiG0MYoDkLQzlAIJRheGQvjAaJHiKJImiwIA2GgNRhDIIJkDAIJnDGZZlHYN5DlgPoBA",
        categories = "development,arrows",
        tags = "code,version control,open",
        contributors = "danielbayley"
    ))]
    GitPullRequestArrow,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDYIA5HYMQyhiGogiKH4hiOJQiG0MgxCAMwtiiE4ZC+MBojKIokguN45DmPI8i6QIzkONImDEOAgDEMQuDUVgxDWTJBhmHIGgiEILlKFIWCKZIJguDY/l+HoBA",
        categories = "development",
        tags = "code,version control,rejected,closed,cancelled,x",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_create_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgiCAcoMDODxjHiDA1CIPg8C+A4FgeGhwGEdBoCAZIME0NQgDkdgxDKGYbiGI4giKJImCIbQxikOQtDOPAgj2E4aC+MRojOI4lieLQgDYaA1GEMgglAMAglMMZRlEdpBjCNJGjWSQ5CCOR2g6QpEl2SAiE0MpQDEOBoC2ZJbjKAQ",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreateArrow,
    #[cfg(feature = "git_pull_request_create")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINgigkeYMg4IBygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDYIA5HYMQyhiGogiKH4hiOJQiE0MQzCANhoDMYQyCCPwwCCQgxkCQB2heGQvjAaIyiKJImDEOAgDENR2g6SpMk6NImDKRZSGgLZYi+Mw+gE",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreate,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CKCR5gyDoQHKDAzCIPg8C+A4FgeGodgaCIWCKGIRgwNoQgqKIZhuIYfDwcBhHQaAgGSDBNg4IA2FYNYtC+Mo0hqQY1jcIo5DgIAxDEdgtDGP5EhobBpG6CB4DKLAgHkMYMDmEB4lwIoplqWAiDKT4aC+U5VD6AQ",
        categories = "development",
        tags = "code,version control,open,draft,dashed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEOIOGMeYShQPg8C+A4FgeGodgaCIQgsNoVhcIomgmC4NhqHIEiKGhwGEdBoCAZILE0MQzCANhoDMYQyCCQgwCCRQxkOQx2DcIoujONYaGwaRuggeQxgsOYOHkMoLDIMYOHiXIpmCV5ji6UpUD6AQ",
        categories = "development,arrows",
        tags = "code,version control,open",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitPullRequest,
    #[cfg(feature = "github")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyDIdgtDQYQ0C4OAghWFwwCCGwwC0MQtDMLg1GMM4cCANgtDKKAtDWIwuDAOIfC4Mg1C2NA3ioLg0jKIIii4Mo9C6DI0jIMguDODYej+J4eDGHIhCAMYjjoNg0jeNouDOKZUhoQw2g4IINiuYxjjeJpTgyZ4OkiSggj8QYuDQMImnKdJNicNAgDkY4bkydYjiiYpUkgOY7DmNw2heUwwjaFoNlMNqODENwulyD5SjQM43gymQ4DUdg0CIPg8C+A4FqSp4GgiCg5lIOJmhUNZPDKLYqC2OQyqOpaqD6AQ",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Github,
    #[cfg(feature = "gitlab")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIAxDMLgyDkLYQDMMwtDEMBhC4NAyhyDQwCCIQwC0LgxDSJQxDgLgziuLYijCJIRDKKQxiwOY3jGIolDKEAwDeH4fjqMoniYOBsC2HgyDYIA2C4Ng3EiLgyEyTgxCCEJLhuHZCiOO4mimLpikOPA2iUMIujgMw5mSEY+kCXIdm2RYqEyDYPhEOYbDcNAunyQwghGQA4DMTAxg0MgxGyOA2hOTg4Die42DeV5eiGLAxiWhBaCIPg8C+A4FD6AQ",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Gitlab,
    #[cfg(feature = "glass_water")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULgyCAMgyEgOAuDgYYPg8MAghoMQtDILQxC4Nw5EwNQgDMaAxDQbIghUMQgDEN4ODEQYYhuN4vgyDoQDIWgiD4PAvgOBZAkOBoIgoNowDIYYmiaGocCCSoak6N5QlKG4/kGRg+gEA",
        categories = "food-beverage",
        tags = "beverage,drink,glass,water",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GlassWater,
    #[cfg(feature = "glasses")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeILDaDhjHmCwxDUIg+DwL4DgWB4bh6BoIhCFw4g6CoMhSFgihiGociKIA8HAYR0GgIBkgsTQxDQIIYGEMggkEMAgkQMAtDKSJCkuRpFkoMovC+NI2huU43jkIhNDILg1j4Mwgl0NxjC4NwtDELpfmcNJKDOSJRlaVY1leOgymeXQxmkOQgmILZkmaaJ/DWSAtm2UIblKcg+gE",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg1DQIAxDUSAxDcYQyCCGAwCCGwwC2GAyHYNIODQIg+DwL4DgWJ4qgaCIKDcIAzC4Mw0FYNRhDOMocjyG46DMdgwheGY9hGRIhDAY4bg2DQ5keGZBkOGpFDKH5BkoLZMC4OYfhmHxojOFYmiiLYsgSLoJE0MQxhmTQ1FYMQ4lKRYelWSJzh2EYflaWZ4j2ewyEgMguDANZjimZ4nGMaRyGMbBlCAcoJDEMAiCAYx5pMMqWGMeKaoei6No8PoBA",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Globe2,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDIYQxDQLg1CCLYvCAMI0jaNQyjWMowjuN43C2OYZhuIYjiCIokiYIhNDKMQyGiQYaC+RBoD6AQ",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Globe,
    #[cfg(feature = "goal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVgyGwOAgDQLYUDQIg+DwL4DgWG4egaCIKDIMAuDUNYOiYMgzEEOQgi8MIOjOGAuDmGYbh2BBoiCO4HgkTYUDEMBhimKYyDEIIyDiNggDILgwDKGociEPoBA",
        categories = "gaming",
        tags = "flag,bullseye",
        contributors = ""
    ))]
    Goal,
    #[cfg(feature = "grab")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDELg1FYORhDIIIWDAIIZDALQyh0dgwhWF4aiSHIWDIdoQDQIg+DwL4DgWLYwgaCIKDENIODAVg4iKGIlh2H4hieJYah2Fx2DKLIujOMoEjSCYLhkOQuDmE49kSJpBleG5Ficdg1kqL5Ok2BYHlANoODSIJbj+HookKI5ciaRwwmGTICk6Zo2g2D5rkOGQxg4II4DAdgzGGDYNlwMQtokaAtDSiAgoqJKMDijYjj6gaBoSdpjgE",
        categories = "cursors,design,layout",
        tags = "hand",
        contributors = "ericfennis"
    ))]
    Grab,
    #[cfg(feature = "graduation_cap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDAdg2guDgwGyDwtDWFAgDULYPhsegiD4PAvgOBYiiWBoIgoNoODIdg1GMMwgjIOYzi0IIQhiIYjigPoBA",
        categories = "buildings,maps",
        tags = "school,university,learn,study,mortarboard,education,ceremony,academic,hat,diploma,bachlor's,master's,doctorate",
        contributors = "Tummerhore,ericfennis"
    ))]
    GraduationCap,
    #[cfg(feature = "grape")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA1FYMhsC0NQuDgOYOhUOQiD4PAvgOBYcGMaRyGMbBlCAYx5gkMYUhYIggHKCQzi8Yx4isNguDaG4diKJImiGI4lieNYJDgLgxDGL4xCKM4oioIg3C4NI7C+PZCkCPpDjYIgxDILgzDWNJPkiOZhjCMpUlaPw8mqWorDMLg5kmZ5MmKCYtmGHJVkGa5tk6KwwC4MIaiiWwxkaLJKmieptliQp0k2RAijgNY6n+XJwDKaZ8GWjompCNKGoEOJ2lyUQ0lOjKcp6J5LpGY6EpKeY8quAQ",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByggIgygwaBlGkZxoHSE4VD4PAvgOBYoHAYYRCAZIJE0MwgDEMhohQIooC+LoRi2LxojGM43CAMx2jqPI+GgPoBA",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOE0MwgDkaIaigL4uheLYvGiMYzjUMQ1jiEY6jwaI+jCMgiE0OQgDMdo5imSZLkCTRNkSUZTkeVY/D6AQ",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIOQiCAcoMDGDxjHiEgyCIPg8C+A4FgeGodgaCIRCKE4JguDYUhaJYOhqHIEiKIIwgeCYrDWFIog6EIShmG4hh+AozgiFYXjiEo3juJY9i+HhljKTYnkeKoSjqJITi6P5OkGUJECKSJWkaJY3liQg+gE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gwOYQHKDAxCIPg8C+A4FgeGodgaCIKgwNYQGOEgihQIIWg2GYbiGH4CgSIoRhOFYXieC4NhSGocjSMoxiOO4OjiLo2g2Jo+kKIJAgiLYYkgMYmgmO5KjCTpNh6Q4XiuUInimU4vj+Ww+gE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINQigkeIMDEMoPHKEgiD4PAvgOBYHhmHIGgiFgiDGDxjhGJA5iaC4NhiGogh6AoEiGEIMg4IIjiWCYsg6GYbjOMYwiKF41iSFI7hKFI+kKH5AkOJIrkmJooDGKpLk6TYdgiJ42lGRoVheV5almNI5l6VZTlKYohmSB5IimYJQkWaJrkGTpFjeZpvnSL5YgE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYwwC0MQuhQOQtDIIAyhgaAyCIPg8C+A4FiCI4GgiCgxDcIAzh0Y4UDEIAwhoIAuDmNAyHaHogiKBBoiWPoHgkTQyjGKo6hEIIwC2NoahiGhohiH4hiaQIFkKCorkUSIQhOFYyhiTIXhuOZSjyVQ8HIZRjHQIB3GkZIFgkNwiCAeZznUaBlGkZxoHSCQ1nUeJ4CAcqDCIMZTC+apsiCjJtm+cRooShwxDCeZ7n2fwioGdoJDGHqFpWiqPD6AQ",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[cfg(feature = "hammer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDILQ4C6DYSDUYwtC4OAzhkMwtDILgxDeHAtDMIAwiaKIniqJhhh+Dwgi4Mopg6JokEyLw5CIPg8C+A4FjuPoGggIhNiELg2DSDoNDKMgxDCRw0jqPJBkCBJCgkbQyk8OQxg4MQuDcLZfDINZiC6ZIXkeGA2hgOYdl8NJth2H5kHaGA4DaNw2C4MJdDSRxhDWEw2CCgg1oSKwwiQLg5nGX5IEgORsoyHw4DIQZ7DEOAgpmm4rl2L4SDQdpfocbIyDIaIfDQN6nmehJflyUo9lYPoBA",
        categories = "tools",
        tags = "mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Hammer,
    #[cfg(feature = "hand_metal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDILg1FYMQwGEMgghcMAghoMAtDKHh2hWF4ZhuG4ehgdgxC4NAiD4PAvgOBYujGBoIgoMQ0g4MRWDmFoYiUMYmjkMB2DKLYvjSM4EjWCYLhqFIRFYNY+iSQYdkMdg5keMJLkqBYHgkbQ3g4NQtioNw2mYLpolSJYch4Lg4DOGJxDIbAzC4NggngNhDDeEYYiqOAgDkLojheD4YDIaAyGGDYNhyJQ4C0OBWDebZPkKGx2DWW5JgE",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[cfg(feature = "hand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDEVg2GEMgghQMAghcMAtDKGx2DCE4VhiIoahQMoeCIPg8C+A4FimLIGgiCgxDSDgwFYNIghaI4bh2H4liOGIbhUdgyiiKovi6BIwgmC4XDEMAuDWEY5kCJI9lSGZBiUdg4kaK5KkmBYHkyDAgDiWIOmmNAwHaEoNg2WQxC2bxohsY4bC6cAtDSUQtnkNgtDULg5DmeAzDQbAtDMLqAouEo/nGFZ5DOeA4DITA3g4NZekiAQ",
        categories = "cursors,accessibility",
        tags = "wave,move,mouse,grab",
        contributors = "ericfennis"
    ))]
    Hand,
    #[cfg(feature = "hard_drive_download")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYOAiD4PAvgOBYThaBoICIbQxDYIA2C0NAgDSIoihKFIZhMchlGMdAgHcaRkgWCQyDAIggHiNY4GgZRpGcaB0gmEQgHKOgiDKOB5gkMQ0igL4si6GIEhqCRNh8MQ4GgLgwDGT4qgKVIHlYMQwCCWZbl2X5UD6AQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIA2C0NIQhAIA0CIPg8C+A4FheGoGggIhNDEMggDIdg4haGIdhcchlGMdAgHcaRkgWCQyDAIggHmCQxhUIB4jWOBoGUaRnGgdIJicIByj8IgyigL4si6HIEh6CRNg0MQ4GgLgwDGT4qgKVIHlYMQwCCWZbl2X5UD6AQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyDIIoHDKCoNCAeYQCIMYSHmCYWg0Pg8C+A4Fh0cBhHQaAgGSChNDULg0DUIIrDEMQgDIIIXHYNhhjSNAwCCPI8joaAxjiOo9kWPwtDIdgtDYbAtDOLA1ksLg4DkQZEj6RZCC4Nw2CANBIDcLgyDSOYzkaPQtDGWw5jULoxHoIodC+I4lh2IIGhSCpCg6CIKDaDoZnqf4Pn4LgwDGcYenedoEniGp7gejwwnyFQxDChqIhOlZ/nKi4BA",
        categories = "development,devices",
        tags = "computer,server,memory,data,ssd,disk,hard disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    HardDrive,
    #[cfg(feature = "hard_hat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ4GEMYNCAMIThWEQxGiDoQhKFIdg0LQxHYLQyhuEYeDCIIgEgM4lhWHYgg0dgyHoIg+DwL4DgWNo5gaCIKDGFJAFYNYth6F4gGiJIXi6EoXHYNY1jePI7gSPYJE0NINDWIosDYIJekaXwtDYaAwlGOJVlSBYHlcMZZmQMBhl6YIWl+Xx2DOZ5TgE",
        categories = "tools",
        tags = "helmet,construction,safety,savety",
        contributors = "Andreto,ericfennis"
    ))]
    HardHat,
    #[cfg(feature = "hash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig5CIIB4gkIgyDCDR5DGCoMg6FgiDQIg+DwL4DgWHohgYeIahyDoQhKFIaDENYUhCLodh+JIjgSBoIgoMgxg2JoKDGE4HhoM49hAOIziCN42gWGY/DaRY/iiFYKkSB4qjyHpJiKAQ",
        categories = "text,social",
        tags = "hashtag,number,pound",
        contributors = "colebemis,ericfennis"
    ))]
    Hash,
    #[cfg(feature = "haze")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1C4MggDaDQgDELg0hKFAiD4PAvgOBYZhyBoICITYODEMxoDKGIah+HoEiCCYjDCEomiiGYbi2LIFgeCRtDEN4UCCPg2hYNAthMNIpjaHYCi2OoiDKJA3EiNIqjeS45iGI4ODIMZSkiK5Wi6IgxkKJRhhWFYxmkLQ4CAMJelWH5NE0MYODUVgyC4NZvh2AQA",
        categories = "weather",
        tags = "mist,fog",
        contributors = "ericfennis"
    ))]
    Haze,
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
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading1,
    #[cfg(feature = "heading_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMgxisaAtDQYwwjkIA0C0M4+C0NggjwMQuDULQykqSI5C0MYii+F4B",
        categories = "text,development",
        tags = "h2,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading2,
    #[cfg(feature = "heading_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMQ3C4NYNDCNxjDELg3C0MQgDONwgDCQpEj0NRhDIIJMkaRgxC2TAyiKL4XgKJozgqNYNjYNRjkySQggwLgzmOQI3kuTZFmwMJSlKVYkgE",
        categories = "text,development",
        tags = "h3,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading3,
    #[cfg(feature = "heading_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMQ3g0MB2DQaA0iKL4XgKJozgoMgxjcdoRi6JIBA",
        categories = "text,development",
        tags = "h4,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading4,
    #[cfg(feature = "heading_5")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmiWBYHigMQ3g0Mx2C0MxoDSIovheAomjOCo1g0NwuDcYwuDQLgyC4OAuDODZRlGUw1CAMAgk4NwtDELgxlqSAtk4NRTDEOZPjeK5VDEMxIjWPokgE",
        categories = "text,development",
        tags = "h5,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading5,
    #[cfg(feature = "heading_6")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmhMYxpHIYxsGUIBjHmCQxiEIBygkMgijgeI7DmIgvjKNI2iWBYHigMgwg0MBjC0MggDILQzCAMwuDWWQgi2I4wgE",
        categories = "text,development",
        tags = "h6,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading6,
    #[cfg(feature = "heading")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQyGiDgiD4PAvgOBYThaBoIgqDAyDAVg0hKFIZhiBIagkTQxDgIIeiCIoViYPoBA",
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading,
    #[cfg(feature = "headphones")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQ0GgMxhDIIITDAIIWDGFIUHaEYThWF4NC2HhIDWEoahaGIiiIdgtDcYQ5CCMIog2DQ4hcdouh6IIph4aAtDGJofjyKwth2J4ghkMoihAIg+DwL4DgUPoB",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Headphones,
    #[cfg(feature = "heart_crack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDQYwxC4NA5C2Eg0DYIAzC0MwuDIMYaC0NQuDUQYjDUIInCAMIri0MQ2iSGhjhYLg3hkMIcCCJAtDSMQyjQNZAC0Mo1DSQ48jsMomjGKosk4IAyCAOIkGOLJEDODoxj0MIoliJxsDcIA3FoIg+DwL4DgWZppgaCAiG0MZRDGGwxhaUJHhuWI/DKZZnmwPoBA",
        categories = "emoji",
        tags = "heartbreak,sadness,emotion",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartCrack,
    #[cfg(feature = "heart_handshake")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDQYwxC4NA5C2Eg0DYIAzC0MwuDIMYaC0NQuDUQYjDUIInCAMIri0MQ2iSGhjhYLg3hkMIcCCJAtDSMQyjQNZAC0Mo1DSQ48jsMomjGKosk4IAyCAOIkGOLJEDODoxj0MIoliJxsDcIA3FoIg+DwL4DgWZppgaCIKDGUYoDkLgwDSYguDkNhhkQMZhnyYZPk+HQwDgdgwGMLg4kSipQC4MYdDiXY6DANxskSlI0Dme6JlGi5Rk+IIdDeDQwpaeIZkQNg2mWZ5smuBJtgkbQxDiDpBj8MqsmisKvgWB6yDGKK0keuZmruaoBA",
        categories = "emoji,account,security",
        tags = "agreement,charity,help,deal,terms,emotion,together,handshake",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartHandshake,
    #[cfg(feature = "heart_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyCKBwygoMoMCAeYJguDR5g+C4MD4PAvgOBYcHAYR0GgIBkgoTQxDYLg1CCKosi4MggDIMRsC0N43GMLQxiyOwuDQNQtDOQguDKQgtDWLBhkmLZMCAMJPi6MwuDENAtDQLgzDUIocC+IokiGI4licIhNDgLg3DYIAzlQY48DGSYRlMMZsDcOJrkUNIuC4OZ7kGPItDKaJWjKWJBDIQZOk6UJQDGM4ymcNRjlCggxkaPAznidgtoINg3CCSQxDeXIdl8aA+gEA",
        categories = "social,multimedia",
        tags = "unlike,dislike,hate,emotion",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    HeartOff,
    #[cfg(feature = "heart_pulse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDQYwxC4NA5C2Eg0DYIAzC0MwuDIMYaC0NQuDUQYjDUIInCAMIri0MQ2iSGhjhYLg3hkMIcCCJAtDSMQyjQNZAC0Mo1DSQ48jsMomjGKosk4IAyCAOIkGOLJEDODoxj0MIoliJxsDcIA3FoIg+DwL4DgWZppgaCIKh0MpRDEMhIDmJBsjuIJRj2KI/mGEpdiQaIjDIN5lmebA+gE",
        categories = "medical",
        tags = "heartbeat,pulse,health,medical,blood pressure,cardiac,systole,diastole",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartPulse,
    #[cfg(feature = "heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDQYwxC4NA5C2Eg0DYIAzC0MwuDIMYaC0NQuDUQYjDUIInCAMIri0MQ2iSGhjhYLg3hkMIcCCJAtDSMQyjQNZAC0Mo1DSQ48jsMomjGKosk4IAyCAOIkGOLJEDODoxj0MIoliJxsDcIA3FoIg+DwL4DgUPoB",
        categories = "medical,social,multimedia,emoji,gaming,shapes",
        tags = "like,love,emotion,suit,playing,cards",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heart,
    #[cfg(feature = "help_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQ5C4MA5CAORhDMIIxDAII0DEIA1C4OIxDEY40DILYxDOQYyCKGwviKJIhiOJYnCITYOCAMQ3GiKwxkaHJJGgPoBA",
        categories = "accessibility,text,shapes,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    HelpCircle,
    #[cfg(feature = "helping_hand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ1CANQuDEMgthGExBgyDAwCCGwxg0MAuDINAgDkSAxDMYQyCCKodg2HAgDQaAtDILg1G0NAtC4Ng4jCEg3C0NAuDgORhDGQo8kYOI8huLY0DmKo0DMNhskCIYQC4OQ0hgIIai+HgxkEOQ2g0NxIDmQoolCL5MC2Rg0jQNQ5EwN4NDkIg+DwL4DgWeJ7gaCIKiqYAgmMNp3nmfg+gE",
        categories = "emoji",
        tags = "agreement,help,proposal,charity,begging,terms",
        contributors = "karsa-mistmere,jguddas"
    ))]
    HelpingHand,
    #[cfg(feature = "hexagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FD6AQ",
        categories = "shapes,brands,development",
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
    #[cfg(feature = "home")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOYNC0N4NCANx2DEMRhDIIIZDAIIcDELYZDISA1hiGodieHwyiAegiD4PAvgOBYuHAbxsHkbBpG4ZQgjSOR0HOCYODKGYODGGQxDUIJGkqSZDi2L40jaOI6D6AQ",
        categories = "account",
        tags = "house,living",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Home,
    #[cfg(feature = "hop_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CANYOEMMQ5CANwgDIMIOCCFQyDEIAxDEYwtDELgzDIMwuDINg1C0MguDYNA2iYOQtDQLohDiKorCIPg8C+A4Fj6QYGgiCoRg+DISheFIchiGpJh+Hhjg6LoOlWLQxC2Og1j2P5EkOBJFgmC4NkmZoji+DwwjYIJsDaJJekCYphgWB5khmIIlDUY4fnuGAgimDwyCCN5dj6c5CgKYp3gqIYbhmfYbh+gqAoagJVn+bAzC2VaGnKYKLnaRhNDKhKmmmbgtoKnYtkiJIOqCdKimOCo3DcOAzoULq4DKEwuDAN66joNA5oSfqXkkOJ9iSGI2pOVYzkmk6FiCEJXDUTY6qaF5qDeE57riIImDOSYRDQNa6i8MA4hUMQ4hCIwuDiOr0quvLjDcNpahEN4ruO+w4rKihsGkbhlCAeQxgkMgiCAeAywzDcJxEIqmw4eMLxacsFwcPoB",
        categories = "food-beverage",
        tags = "beer,brewery,drink,hop free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    HopOff,
    #[cfg(feature = "hop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CANYOEMMQ5CANwgDIMIOCCFQyDEIAxDEYwtDKDoOC2D4nDiJwxCIPg8C+A4Fi+MoGgiCoRg+DISheFIchiGo6h+HhjieJY5igLQxC2Kw1i6MI1jSBI2gmCw2huIYOGOQ4gCAM5YCANokhsMAtDQIJlmKLYvjGU5SgWB5VhmIAxlqH51g8Mpehueg0iyJg1C0M5omaJ4Ok+bYzgKU5xgqWZ5DCW5gl+eQgn6lZ/g+ZQzC2J6XoiUaLnCNxNhmWJXDWW4YiCG6UqyD5Inqm6GoGSJ4qCbqilSCp+DcOKDr0OAyEOKw0Dmg51DAN56DGl5dDEOIQiOd6FnkLaoq+z5gmeG62oCZggieSI8iCrg2iCF46DiIw0oKJ5fDkMg3C0OYODAOKdh68wxl+Hg4FquKKjWjRNs6zqDDOIwutC7cLDigcOw0OAxueZZrlCboBA",
        categories = "food-beverage",
        tags = "beer,brewery,drink",
        contributors = "karsa-mistmere"
    ))]
    Hop,
    #[cfg(feature = "hotel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDSJ4TikMIZhkWgiD4PAvgOBY3jqBoICIbQ5CCHQgC4Mw0DgLQuDINBjDELg0DYNQtk8MAxDMIAzC4OA0lQLpWlgNZGDANIVEwMQ1kMNo2jiPY8gSPoJE2DQ3GiXwxmyOZwm+BYHnKRJ1neeZugKcJ+gqJQgoGVqDnuhZ9j+C4TDEMZ2oyN56juj5xogNpDpWgqYoSPaHnOn6WnioqOqSkQxiKGwtDYLg1G2ZQwFYMgyo2O4BA",
        categories = "buildings,maps,travel",
        tags = "building,hostel,motel,inn",
        contributors = "karsa-mistmere"
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
    #[cfg(feature = "ice_cream_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcYw1CAMAgDgLQyC4Ng5hULQ2EgNBjhQMwuDMMQgDMIA2hWKRaG0LQ0CANBoDiLg0C0Mx2DMTQ1C4MYwDEMRhiOEpDhODpHDYLg3iYMAiD4PAvgOBZPlKBoIgqDI9j+QZFkWFImiaSZLhOTpQlWVIElaCYLjyEpJDWQgukScpGkwLQ3mST5RmkPoBA",
        categories = "food-beverage",
        tags = "gelato,food,dessert,dish,restaurant,course,meal",
        contributors = "kemie,jguddas,karsa-mistmere,ericfennis"
    ))]
    IceCream2,
    #[cfg(feature = "ice_cream")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQxCANAuDAOINDALgzDUYYPg8MAgh2HQxC4OA0h4TAxgyDgiD4PAvgOBYri6BoICITYnCANxBDUII6h+Ho3jeKosjGMIEjKCY1gwNxhDIIJMj2HIQEiSpMk6PocC0NJBi2RQ+gEA",
        categories = "food-beverage",
        tags = "gelato,food",
        contributors = "karsa-mistmere"
    ))]
    IceCream,
    #[cfg(feature = "image_down")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIOQigkeYMg4IBygwMgiD4PAvgOBYHhkcBhHQaAgGSDBNDEMAuDMIAyDESA1GEMosCAMI0CAMQtDKORWjCMoyjWNQxiyORoDENIxjOQI3jMMh2igLg4hiGogiKH4hiOJQiG2LY3DULQzC6OJgDGSI/jYMI5lCRguDCRhMDaLAxlIL5UGiVoiiSDBtkaNw5C4NQgisMx2C0NpznWd5YnoMQ3iwMp/oGX6HlcPoBA",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImageDown,
    #[cfg(feature = "image_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5HYMQwGEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNwiD4PAvgOBYxGwaRuGUIB5DKCQ1CIIB4DGCQxDaQB5kMIo/kGPQiDIMowjKN45jEYxpHIYxsjoYx4gkOZAHKCZQCAYx5l6UQvlaWJajGNIGggIhtgwIAxDULQzC4MA4Dad55nuIIWiqGwuDgMg4hgTA2hUMZom4PoB",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2D4JguEx5hGCwiD4PAvgOBYcHAYR0GgIBkgoTQxDALg0DEIIqiwMRhDIII0DCL4vC0MguDgM46jwM4bh2IokhyH4GHiFQ2g6EIKDEMwuDWDoIhIMZThmT5RkKHoEGWRpdg+WA4kyFQylaB5YheFQxlKHJciAPJEiWJwiE2UA1DkIJ3DkQQxC4OZ5n6gAgjehZ6CANR2DENIzjWhKPjeNAyGiixjlENaPn4MA1DILQugyL4xp6eJbnKIYjnOKJmi8NRWDWjY2pCOo6EgOalqgPoBA",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIdg3GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNwiD4PAvgOBYxGwaRuGUIB5DKCQ1CIIB4j0IgyDKQB4DGCQxDaQB5kkIo/jEL43jmNo4jqToJkaQZPDEOZNkMOJHkOXowjKVBljEYxpHIYxsjocpakAYx4gmXwgGMeZ2mYL5rm2b4xjSBoICIbYMg4NQtDMLgwDgNqKoyjoghaKobC4OAyDiGBMDaFQxnygg+gE",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgGgZRpGcaB0goMQ4g0dxpGQdBohmGwgg8Igzg2CYmCIPg8C+A4FiwYxpHIYxsGWB4Qg0Y4lDmOopj2LAvjKNI2iwcBhiAIBkgobQyDEIAxDULQzC4MA4DaU5VlcYQyCCXQwCCYAwC0MguDgMg4mETA2l4MYri2R4gD6AQ",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Image,
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
    #[cfg(feature = "indent")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgzCAOAgDcIAxDIIISDENgiD4PAvgOBYHgmH4kgoeAyhAMQxCIIB5ioIoYi4eAxhAMoti+NoyDKHogieJoIiiMYsi4eY7h2L4xkmNY3i2HwvkAPInkqKw4jSRI5keVo0juOI+lGQg+gE",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Indent,
    #[cfg(feature = "indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMxoDEMgiD4PAvgOBYThaBoIgqDA4g+EYThWBBohiI4HgkbYMDEMwgDgLg1i2EoUhmJYFieHAgisaAzjKIoXgKJobE0OY5DMYw2C4Ng2DcIAwCCSJKDcLQxk4MJTDCPY0gE",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    IndianRupee,
    #[cfg(feature = "infinity")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMGMLQyhELg2DcLQ0hcLQ2hcYQ0CCHgwg4IIhiEOBjg2IYYDELgzDMIIbDQWhtiSJwgDKFA3h+Og2h+HY6iSI4jC2JoRkKHori2GofFoIg+DwL4DgUPoB",
        categories = "multimedia",
        tags = "unlimited,forever,loop,maths",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Infinity,
    #[cfg(feature = "info")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDENh2C0NIZhuIYjiCIokiYIooDIIA4GgLgwDGMAvjIaA+gEA",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[cfg(feature = "instagram")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggGgZRpGcaB0g2D4RHmGYRHIeINDWHocCKIggiCDgiD4PAvgOBYrHAYYLCAZINE0MQ2CAMQxC4Mw3EENAgkEMI6kUMQyC4NgzCAOJCk6RJEDGOo5juPQ3HqKosjGC4rGwaRuGWJwyg0MQ3C4NQxhEeAxmSZomHmbAiDaZ4bmOcp0isL5emAPoBA",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDAIggHkMYKDSDh4hGCw5g4eYJCKEw+DwL4DgWHohgaGoKDKDYHhsNYUhYMYTg+FooCKHoggQZYjjeKoKhiD4bjOB4uiyMYSjSH4kD6AQ",
        categories = "text",
        tags = "oblique,text,format",
        contributors = "colebemis,ericfennis"
    ))]
    Italic,
    #[cfg(feature = "iteration_ccw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAYwwC0NAuDQLQzC4NgtDiG4bHOGwghiGg4CCJIiiWKA4GgOAiD4PAvgOBYuHAbxsHkbBpG4ZQgjSOR0HOCQxDaDg0CCDIOiSQpGDKLYvjSNo4joPoB",
        categories = "arrows,design",
        tags = "arrow,right",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCcw,
    #[cfg(feature = "iteration_cw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGMMAtDQLoMDMLg2C0OAgDiGRzhqFg2huGwtiCI4aDgSA0CIPg8C+A4FiwcBvGweRsGkbhlCCMo3HQc4JhoMgyCCDAxieDYqiyLozjWNxlD6AQ",
        categories = "arrows,design",
        tags = "arrow,left",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCw,
    #[cfg(feature = "japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA5C4NRWDIMRtDALQxDGEBMDYIAzG2HA2hoMQ4h0Ig+DwL4DgWJ4qgaCIKhwMQ1GiDImiiLYsgSLoJE2MQxjQMo2imOg+gEA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis"
    ))]
    JapaneseYen,
    #[cfg(feature = "joystick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDcYQyCCEgwCCFQwC0MoZEgNYRhOFoghiEgyHYMoehSIYfDIaAxDSJ4phWGokhkWgiD4PAvgOBY3jqBoIgoNoODUdoZjaOI9jyBI+gkTQxhIMQ1FYOZGjmSo3GMaRyGMbBlCAYx4gmTgil4eYJDaYxygkM5UliWpcD6AQA",
        categories = "gaming,devices",
        tags = "game,console,control stick",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Joystick,
    #[cfg(feature = "kanban_square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANx2DcIg+DwL4DgWEoVgaCIKDEMoNHYNIRhOGIXgSGYJE0MQ2h4OYhhSJYkgWB4nDUIAzGGHYdDAII6DALY4i2I4CiWMoKDmNRoDGQIvkKMYaigNJHkmEouhaTImhuRo2jiO5cDEII/lOQYYkQTQyl4OR2lKIpLmOTpmCAMQ0mmSpVm2J5vDEOY3l+XI6DGPpfnQaIwleT5fDGSKCoSZJGmaiZhmyQ5OjSZp7jmXY+j6ipWmQM5wnKapUoOnJOp6aKhiOAQ",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,draft,template,boilerplate,code,coding",
        contributors = "danielbayley"
    ))]
    KanbanSquareDashed,
    #[cfg(feature = "kanban_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoSHcaRkHQaIOhCEh5g4M4ShUIojD4PAvgOBYoHAYYcCAZIOE0OAgDcdg3CKKAvi6HIti8aIxjMMQyjYdg0jqKY9GiP4wjIIhNDENpGDmSY8kAPoBA",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    KanbanSquare,
    #[cfg(feature = "kanban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CANR2DEMQiD4PAvgOBYThaBoIgoMQyg0dg2hKFIZhiBIagkTQxDiHwxDSIoViYPoBA",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    Kanban,
    #[cfg(feature = "key_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ4HYMxjDAIAuDYLg0g2GQxGgNB2C0MxoDOHogDIbAxhcLYnDQYYWDUIItCCEwxjELQ0jUWgiD4PAvgOBY6GMaRyGMbBlCAYx5gkNwuDUIggHKCZLk0Yx4gkMYtjmO5AkKRA+gEA",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = ""
    ))]
    KeyRound,
    #[cfg(feature = "key_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg0CCDQ3GMLg5C2FIQC4NYWDkIAzg4IAwGwNYZCCIw1hMOYUheDQ1iCHYOGwLYeDeL4ShuFAtiyOIeg8MBMDgLo0ikOBjhuRo4iyIIyg4WgiD4PAvgOBZPlKBoICIbQxg+NAzh2TpQlWVIElaCRNimDwxDALg2hAIAxDgdgzGMMAgmuHwxm6bhoDQdoyGgM59DMaAyGwMYOC2hg0l+UZjD6AQ",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = ""
    ))]
    KeySquare,
    #[cfg(feature = "key")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ1C4NQiCAcoMg+EYJHiDA3hAIg+DwL4DgWB4eHAYR0GgIBkgwbQyDEIAyC0OQuDYIIyDaHYfiWJ4kiaKIqCIbYOhAIIbDUIAzkcTAyDKRBsC0M5PjgL46GgPoBA",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHIeYNDKER4hWERoGUaRnGgdINDENoRHKGIOhGFAiDQIg+DwL4DgWLRwGGCwgGSDRNDYIA4GgLgwDAMYsi6M4LjKNBojaOAxDCO49j+QYtC+RBokaNY3CITQxDSTY+kCQpSkeVZIleWQ4lyT5flOYpJliZgxDKTpelGag8lObJZDIIJvnGUJDmGdZHneIp6nCXZ9mCRaAlaOA3noNhokuaZhgE",
        categories = "text,devices,development",
        tags = "layout,spell,settings,mouse",
        contributors = "it-is-not,ericfennis"
    ))]
    Keyboard,
    #[cfg(feature = "lamp_ceiling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYNQiD4PAvgOBYThaBoIgoNggDcaIMGwNAgDkSAyiILQ5FqEoUhmGIEhqCRNDkLgxDcIAxDYYQzCCPAwjgII/DULg2h0MIshWMA+gE",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampCeiling,
    #[cfg(feature = "lamp_desk")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQIA1C0MwghEMggDcIA4C2GA3C0MhaCIPg8C+A4FiCI4GgiCoMg6EIShCLAzi+EoeiCIoEGiJY2geCRNDkLg1CANo+CCDQxDIbIRDaH4hiaOIFjoIhNhMMh2hwYwwC0MQulkOYcCAMocGgNBhhSFAwCCZgxl6Xh2DISAzjOS42D6AQ",
        categories = "furniture",
        tags = "lighting,household,office,desk,home,furniture",
        contributors = "karsa-mistmere,jguddas"
    ))]
    LampDesk,
    #[cfg(feature = "lamp_floor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMhoDYbAzCANxIhAMwtDcWgiD4PAvgOBYch+BoIgoMQyCAOR2DEM4bh2IohgSI4JguDYODaLYejEPoB",
        categories = "furniture",
        tags = "lighting,household,floor,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampFloor,
    #[cfg(feature = "lamp_wall_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDMaA2GwMwgDcSA4hMLQ3FoIg+DwL4DgWHohgaCIKDENIODMVg4GEMggi8MAgjIMAtDKNoXh2H4kiOBIlgkTYpDkaAyi6MIzkiMo3DIVg1kaMZJjaOA0HYNoch6II+D6AQ",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallDown,
    #[cfg(feature = "lamp_wall_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0GgNhsDMIA3EgOISC0NxaCIPg8C+A4Fh2IIGgiCgxDQIIMHYNRhDIIIuDAIIxDELYuDKFoch6I4igSJIJE2KAxDUaAyi2L4ykiDY2HaRY2kiM41i8SA0HYLQ2huHYfj0PoBA",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallUp,
    #[cfg(feature = "lamp")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhoDgbA0CAMQwEgNBMgwMhaCIPg8C+A4Fh2IIGgiCgxDKEwyHYNoch6I4igSJIJguDYqC0MhjDALQxC6PA5jeDY3GgNBhiiKAwCCSAxg2DR2DISA4huHYfjEPoBA",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Lamp,
    #[cfg(feature = "land_plot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIA4CANgtDMLYRDMdgxDAIg+DwL4DgWG4egaCIKg8MQxC4OQ5C0NQuDUIAzC4MQ0GEMQgjUMAgjiOo2C4Nw0GwOItCANAuDgNhhg2DY7jiSpAi0LZEkaNI2jmVY6C2J4+EwMQ2jYMoahyIYggSIoJG0NguDQOZekWLomC4MINmgM5gh2ZJjgWB4JE0MQ3i2NYMm2EJCDEOYxDWdZigE",
        categories = "shapes,design,tools,maths,sports,gaming",
        tags = "area,surface,square metres,allotment,parcel,property,plane,acres,measure,distance,isometric,flag,golf course,hole",
        contributors = "danielbayley"
    ))]
    LandPlot,
    #[cfg(feature = "landmark")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgyDIIggHiCQiDODh5DKCoMg4eIWgsMQiD4PAvgOBYfiKBoQgoNoZhuKYHhEMQ4hSGwxh2H4hgQZYkjeD4uDCMYKjOFIujCD4yj2NYljmBY7j8NIqkyPgikCLY/jCR43kmJoykOCJUlCUonlGVYgkgPBwG8bB5GcbxuCCZoEHQc4/DIIJzDIMAgDcIA0niHogmaaJqG4PoB",
        categories = "money,maps,buildings",
        tags = "bank,building,capitol,finance,money",
        contributors = "connium,ericfennis"
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
    #[cfg(feature = "laptop_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIIJCININHcaRkHQaIKDEOINGgZRpGcaB0hiDAgHiCgzg0colgsIg+DwL4DgWLRsGkbhliQMoKDKIx5jiCwwg0eQxjmP4kkKK4tC+M41D6AQA",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis"
    ))]
    Laptop2,
    #[cfg(feature = "laptop")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDYVg3GEMgghSDYXC0MoZEgNoThUIIXiCGYVHYORtg+IBIDSJw2iCDguDIOIVC4NQ1GEMYOi6DQxC0Lg5i8NA1EgMwuDYMo3jmIY8j4LQxC6QRMDSDg2CIPg8C+A4FD6AQ",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis,csandman"
    ))]
    Laptop,
    #[cfg(feature = "lasso_select")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgyGENQghEMAghQMQtDILQ0CIPg8C+A4Fh2IIGgiCoMDENguDkMxjiqKQ0DMIAxi4Lg3DSDYqDmKgxhyHojiKBIkgkTQzC4M43DENBBikOAgkyFZQDGDYyDAYwwhoLg0DIIA0lkOAtk0MZXDgc5iCCRQ1mGFA4GENwuDEOQgm6cJQhYLZGjEMo9h+QZAgWB5DhEMZsluW4WnWGpToadZQDQep7j+ApBoCCpJniDQyGidwwDkYZGDWn6MhcLgyDSdwzDIdgtmKnpHkaN4UoeeKnDSLQwk2FJvqAMJFDKM63GybpHk6rZFDOMaxjKd6+C4NQ5pqXQ0DkbIYs2DJFDgNatqAM4TlGyw4m8NB2DCj4dnyIYBA",
        categories = "arrows,design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    LassoSelect,
    #[cfg(feature = "lasso")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgyGENQghEMAghQMQtDILQ0CIPg8C+A4Fh2IIGgiCgzC4MwgDENBBDYLg4CCLowhSFoNioMBjDCGguDQIA0C4NQtjAMY6Dgc5ECCJw2jcIA4juEZOkgOBhDEMoqlaNIqC2QQxhyHojiKBIkgkTYRDGU5WliKoVhWGo2mqWYUDQepeh+Yg+gEA",
        categories = "design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    Lasso,
    #[cfg(feature = "laugh")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ4CAMQzGENggi8MAgjIMQti8NYwjmMo0jYLQ1GiDhahmG4hiOGhsGkboIHgMYMDmEB5k0IpPCAeQyk6EB4leUwuDAMZDC+SJKkeSZLlIMQ1lCW5UlqEw1l2X5VlKT4amGZQ+gEA",
        categories = "emoji",
        tags = "emoji,face,happy,good,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Laugh,
    #[cfg(feature = "layers_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYLgwDIIAxhANQuDQOAgDMLgxDMYQxhEIAwiCH4hDELg3DQTIbCAMolicYYQhCIYkC2MRsC0OAuDULQ0C4OA3h2I4ih4MAti2KA3C4OYXhIIg+DwL4DgWTpRgaCAiE2KobiYNIviuIozjETAyjkII4j6QJDkKIJFluKYemMMg2l2MZqjWOA1CCPJnh6aYkkKWxak2T5UD6AQ",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy,duplicate,double,shortcuts",
        contributors = ""
    ))]
    Layers2,
    #[cfg(feature = "layers_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYdh+GIxjMOAtjcORhimK4aiyHIODMWgiD4PAvgOBZSlWBoIgqJYnCAOYzkWM4qh6SIqkuZ4vDGQogkaOoWhaPZjDYNZBjKNJhkeSZnkyLwzGyYQ1k0NQ5lGU5YleBJZgkbZcjUMQ0mCN50j+ZZKnGaZrjaOJujyaIenQMJCneRqVnuG59n+kqCoSUpUokPoBA",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy,duplicate,triple",
        contributors = ""
    ))]
    Layers3,
    #[cfg(feature = "layers")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYdh+GIxjMOAtjcORhimK4aiyHIODMWgiD4PAvgOBZSlWBoIgoMoWDEN4eDULQ5C6XggDSZA2juFotDGTZADCIoql8Ng1lGU5YleBJZgkbZciqI5hmOZZnDGaYWmuLpuiCcJdoCdpUnoPoBA",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy",
        contributors = "colebemis"
    ))]
    Layers,
    #[cfg(feature = "layout_dashboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIB3GkZB0GiCQ3gwaBlGkZxoHSCQ5gwcoOCIMQiD4PAvgOBYoiuBoahyHoJDWDIIgqIokiaEYThWF4MjkNInimLotgSBoShSFgihiDYJDGQYHk4MoZhuHYfCKIQgiOTpCiqRpFgWTY3lGJQ2gySI9kuVIxleNJajmXZEgE",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig3CIIByHiDQxhAaBlGkZxoHSDYPCCEwiDOEB5g2Ig+DwL4DgWJ4qgaIAxDSEIShSEIIgqDIOiOJYWhiGocjmJ4pgQdIskOH4UjGB4JguHY6CKMIyi+PIZhuTZBi2RYFhGUggheVI/h6JJPkmIIikqN5WiiWIB",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gweIJgsIByhcIgxgwdxpGQdBohQIg+DwL4DgWJ4qgaIIiiQIoVg2D4RhOMoWhiDIIh0NIMhuCYeieKYEHSJxwGGIwgGSCRNDENAgDQaIVkOSIjkeSRokuTZPCAOZTiaKJWGiWJKkwIpOlAMQ1mCVZZmWWpnmkIAyDCbZim+AQ",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig3CIIBoGUaRnGgdINDEOIQHmDQzhAch4hiEIhCKHg+DwL4DgWJ4qgaHIlh+JAxiOGA0hCCIKgyDoQhKFIWg2D4nimBB0iyRIHgmC5AjGIoRhOFYXjsIIyjYIIvDGNpCi0PoBA",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gweIJgsIB3GkZB0GiCQxDiDByhcIgxCIPg8C+A4FimLIGiSIIMhuHYfCKFYNg+EYTjeDIIiYNIWhiKIqi+LoEgaJQxkGB4gkyDoQhKFIjkqM4ch6U4piuSA+gEA",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIIJCIM4NHmCoRCAdxpGQdBogoMQ4g0aBlGkZxoHSCg3CIPg8C+A4FimLIGiCIokiaDYPhWE4LDSDYIhyDYXhmGwiDmKIqi+LoEjCIYjiUIong6HA2hKHI6geD4MhaGIagoNZEiuSA+gEA",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[cfg(feature = "layout")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDB4gmCwgHKCAiDKDB3GkZB0GiFIWhuGIeCIPg8C+A4FiwbBpG4ZYHDGCQ5hcMoJDIMYXjeCoMHmOwijmLAvjKNIxjONR4kCOY2jyPggHiRJQkOOIri2SRlD6AQ",
        categories = "design,layout",
        tags = "window,webpage",
        contributors = "colebemis,ericfennis"
    ))]
    Layout,
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
        categories = "food-beverage,emoji",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[cfg(feature = "library_big")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQxDiDB3GkZB0GiCYWCAeYJgsIByggIgxCIPg8C+A4FikcBhhsIBkgkTQ3CAMx2hWKIqi+G4ujAaIyjQMgwC4NAghULg5GMLgyC4NQtC4MZIlKUQ2lQMxsC0MZKC4NxjlENZNluVZVlwM5WEwMZclOYgxmCY5PmaUguDaZJZlwOZRl+T5RDKVJslidRajsL49GgPoBA",
        categories = "text,photography,multimedia,maps,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    LibraryBig,
    #[cfg(feature = "library_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh3GkZB0GiDoQhIeYVhIcoUCIMgiD4PAvgOBYoHAYYbCAZIOE0NwgDcdgxDCJ4pi6G4ti8aIxjMMQxjaOI6igL49GiP4wjIIhtDENY2CAMggjmO5KkAPoBA",
        categories = "text,photography,multimedia,maps,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    LibrarySquare,
    #[cfg(feature = "library")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIINDQIAxDQIg+DwL4DgWFYYgaCAiE0MQyg4doShSFobhqBIcgkTQ4CAOIjDKJYXimKIFgeK4QDSIw2jKJ4BA",
        categories = "text,photography,multimedia,maps,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "johnletey,csandman,ericfennis"
    ))]
    Library,
    #[cfg(feature = "life_buoy")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQ0C4OQzCCKosi4LgyDSMYzCKGwviKJIhiOJYnCIbQxioOItDkLgxDeNQ0C2Ko2jiOhojyJImiiQgukQIJWliTY0lyN4clCUo+iiRpIlmQwzkyMpdmuX45j2G4egaCISCINIWguDYPgmGJ6m6coggEA",
        categories = "accessibility,medical",
        tags = "preserver,life belt,lifesaver,help,rescue,ship,ring,raft,inflatable,wheel,donut",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    LifeBuoy,
    #[cfg(feature = "ligature")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgwFYOBjDALQyC4MggDELg4C0NAgDSHIYC4NQgDCDYahoIAziKDQiD4PAvgOBYujGBoIgoNoYDIaA0i2L40jOBI1gkTQxh0MY6DIdg4j2MJBkCBYHkOOIOjuTI/gKQZRgqRYNDCVYuk2MoBA",
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
    #[cfg(feature = "line_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFYYgaCAiG0MQ5CAOQtDUIA1C0NIoC2DAzhSFobD6AQ",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere"
    ))]
    LineChart,
    #[cfg(feature = "link_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoIg+DwL4DgWHohgaCIKDGEw3GgMhhhOFYXg0IA0CAOIdh+JIeGwaRuGUIB4DGCY1CAeQygkMQyCKQ5ACKR5JHiRZMkiHgvjqPI5juPR5kuSI+luSZEgkMpck+YZSh+VRlD6AQ",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoaAyCIPg8C+A4FiCI4GgiCgxhMN4dGGE4Vg2MYYDAaAth6IIigQaIgGwaRuGUIB5DKCQxh6QQxkSRh4kgIg4CIIB4kMIgxDaH4hj2Pw+gE",
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
    #[cfg(feature = "linkedin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA4GGDYNDAIITDEIIRHYNxoC0NB2C0NxhDIIIihOJQtDKJ4jiqJYUikMoZhuHYfhCF4UjaFg2C0Nh6CIPg8C+A4Fj4chlGMdAgHcaRkgWCQ0CIIBoGUaRnGgdIJDEMpPHiCZZCAeYJDmPY/kSRo+GMaRyGMbBlCAY5bCKTptl+cJPHKXJiC+Z5pmsPoBA",
        categories = "account,social,brands",
        tags = "logo,social media,social",
        contributors = "okcoker,csandman,ericfennis"
    ))]
    Linkedin,
    #[cfg(feature = "list_checks")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ3CAMoQCANAtDQIg+DwL4DgWGIbgaCIKgyD4RhGFIWhiGoEGiHYqgeCRNDGDA2GgOIXhmHosgWLgijCDAxDKNI2imHICi2II9g0OJBiiOIBA",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "ericfennis"
    ))]
    ListChecks,
    #[cfg(feature = "list_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDISAzCIPg8C+A4FhSF4GgiCoMCANoRhOFYahmBIbgmCwwg4OIhhSFomiWBYHigMgxh8dgxDAYQyCCPIqioMQtjwMhoC0NYii+GICiaMwiG2HoMkKPZTDKSIkgE",
        categories = "multimedia,text",
        tags = "queue,bottom,end,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListEnd,
    #[cfg(feature = "list_filter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDEOAiD4PAvgOBYThaBoIgoNwgDEMoPDCEoUhmGIEhqCRNDEMIeDgaA0iOFYnD6AQA",
        categories = "text",
        tags = "options",
        contributors = "danielbayley"
    ))]
    ListFilter,
    #[cfg(feature = "list_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDISAzCIPg8C+A4FhSF4GgiCgxDYIA2hGE4VhqGYEhuCYLh8MQ4iKFIWieJoFgeKQyg2DxoC0NojjCGIBA",
        categories = "multimedia,text",
        tags = "playlist,remove,song,subtract,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListMinus,
    #[cfg(feature = "list_music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUVg2CIPg8C+A4FhSF4GgiCgxDgLg1g4OBhDKIAgiWIQwg4IIqDALYhiiJ4mi2LI1DUWoThWGoZgSG4JE0MQyg4MhIDOOYWj2PIFgePwxDYIA2kWR47gKPZMh2QoelKFJIhiAQ",
        categories = "multimedia",
        tags = "playlist,queue,music,audio,playback",
        contributors = "karsa-mistmere"
    ))]
    ListMusic,
    #[cfg(feature = "list_ordered")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig2CIIB4gkIgyDGDR4DGCgxDCDR5haCwiD4PAvgOBYfiKBoVheGYHhwMQyhSEIShqEIsh6IIliSBImi+E4OiuKYbhcOIxkCNIhjiHxwGEdBoCAZIKE0NAgDYaAxHYNJEkiSpHkmS5NCKTwghgaIth8L5YGiWpKkyTg2mAOBIDQYwwC0MQgDILQynULQzHOc5zC4NZ3nOV5bD6AQ",
        categories = "text",
        tags = "number,order,queue",
        contributors = "ericfennis,csandman"
    ))]
    ListOrdered,
    #[cfg(feature = "list_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDISAzCIPg8C+A4FhSF4GgiCgxDYIA2hGE4VhqGYEhuCYLh8MQ4iKFIWieJoFgeKYsCAOR2DaI4whiAonjSCgyg2DxoC2OoviWAQ",
        categories = "multimedia,text",
        tags = "playlist,add,song,track,new",
        contributors = "ericfennis"
    ))]
    ListPlus,
    #[cfg(feature = "list_restart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA2EgMwiD4PAvgOBYThaBoIgoNwgDEMoQhKFIZhiBIagkTYdDEOIhhOFYmiWBYHiiH4eDgYQ1CCOQwCCPI8DkLQzCANAujmRI7j2PQtkeS5FGMLQxC4M5CDALQykWRw0kELg0g2UZdEwMZeDSIovheAomjOCpih4MB2DQaJki6JIBA",
        categories = "multimedia,text",
        tags = "reset,refresh,reload,playlist,replay",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListRestart,
    #[cfg(feature = "list_start")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDISAzCIPg8C+A4FhSF4GgiCoMg4OIRhOFYahmBIbgmCwwCANohhSFomiWBYHigMgxh8Vg4GEMggjuKo+C0MpAGgLQ1iKL4YgKJozCIbYeDiQJAjyQJGiSAQ",
        categories = "multimedia,text",
        tags = "queue,top,start,next,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListStart,
    #[cfg(feature = "list_todo")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAch4gkMYMg8IgzgwdxpGQdBogkNoMGgZRpGcaB0hwIg+DwL4DgWJxwGGGggGSCRtDMIAxDcIAyjgIA0C0NImiiLYaiyLhojCCRNDGNA2GgOI/C+QRokOL4xCKSI0DEMpMk6UJSkWVJWjUOJaieT5ED6AQ",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = ""
    ))]
    ListTodo,
    #[cfg(feature = "list_tree")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIaAtDgIg+DwL4DgWFYYgaCIKgwIA2EiE4VheBBohqJoHgmC4NDEOIRiOFobiiBYqgoM4gHYNBjDCDguDELg5CAMpDkMaAzhSMomjSHIrjgMQwHYNo8j6QJCkSWJHkmJYZgEA",
        categories = "files,text,layout",
        tags = "tree,browser",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListTree,
    #[cfg(feature = "list_video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMEgMwiD4PAvgOBYThaBoIgoMQ2CANoQhKFIZhiBIagmC4NDEOIhhOFYmiWBYHgkbYdg6DQ1CAMwtjkMx2C0NhaiKL4XgEA",
        categories = "multimedia",
        tags = "playlist,video,playback",
        contributors = "karsa-mistmere"
    ))]
    ListVideo,
    #[cfg(feature = "list_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDISAzCIPg8C+A4FhSF4GgiCgxDYIA2hGE4VhqGYEhuCYLh8MQ4iKFIWieJoFgeCRtDEOYODALQ0CANIjjCGICieNAijYNY5j2PY/iWAQA",
        categories = "multimedia,text",
        tags = "playlist,subtract,remove,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListX,
    #[cfg(feature = "list")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig4CIIB5DKCg2g0eIQCIMgxg0eYJCKEg+DwL4DgWHohgaCIKgyB4VheGYbDEMoZhWLgih6IIEGWI42g6LYog+CgxiiJoLhOKoYjSJI4gWDoVhKB4bDOQ4KDMLgwhiOoRjOH5HgKOZBk+KZRlOVYaj6L5KmSWI1iKW5Jl2MI+kCFZSlSLJvmiR4B",
        categories = "text",
        tags = "options",
        contributors = "colebemis,ericfennis"
    ))]
    List,
    #[cfg(feature = "loader_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIYQ5CCEgwg6DgtDYLoMDkLQ4C4NQ2CIPg8C+A4FD6AQA",
        categories = "multimedia,layout",
        tags = "load",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,ericfennis"
    ))]
    Loader2,
    #[cfg(feature = "loader")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDIIggHmCQiDaDh4DGCoMg4eYWCKDQ+DwL4DgWHohgaFYXg2D4RDKKIIieGYbDEOAih6IIEGWI42imCg3C4N4TgeGw0C4OQzhSEY8j6L4KkKRIzh+JI4gWQIXDYLgyDSGYRDEOQuDAN5KguVZXkaF5cl6To1iKAo5hqLpThyWZui2EpolCa5ShCcoqiibYLiyMIyjSdokg+MJml+b5MkWB5Hj2P55mGVpYoKNpRgakKKhSMJilijJll2iJ9kiE6UiKAQ",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDIIggHiCQig2BwygqDIOHiFQiDUIg+DwL4DgWHohgaCIWhOGYKDKKIRDEOYOHmGoXh6IIEGWI42g+MoTjGCocg+LY8hGDY0iSOIFkCJ4wi2L46kqFIqkSH5GDwYxpHIYxsgYYx4k8coKDeDhjHmSo0laWJah6Z5ZgaXwiDOYpdguE5jmWH5rmmAQ",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyCKBwygoNYNHmCQiDGDAgHmD4VgwPg8C+A4Fh2IIGgiCgxDmEoahaDR4hoMoXhOJoch6I4igSBoxguLIqheJYbimEAih2H43jaBYHhSK4OjKEpJiiGIujORIhDwcBhHQaAgGSChNDcLgxDEIJemAQw1C4OAzCAOAuDMOQgDUIAxDCX5vnEMhjDAIAzmcN56l+aZ9oGcQum6eZ7DYMQtmeaQ0mcOQtDKXwxkKHpWliHaWlmWwiE0MZrDeYQxnsOQ2GOX6PC4Np7DKjwxC6rKvq2hA2CAMAtnsOA3refwtrqvqKDafa2q4M57omrqkrGlAvpmRoklGQILjCFI9tSy41gEA",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0Ig1CKBwxgoMoNHmCQiDGEQgHmD4VhEPg8C+A4Fh2IIGhOCoWg2CIQheGYmhceIaDEOQih2H4EGWIo2hiGorhSDIOi2KIUieNIjjiBY/huEoUDKK4wjKB5ChyHpFDwYxpHIYxsgYcoKDeDRjHiQAgGMeZAjSVpYloPoB",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[cfg(feature = "lock_keyhole")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDENoOGMeISDIIg+DwL4DgWB4bHIZRjHQIBoGUaRnGgdIYg4coXCKGQgHcaRkHQaISDiDowDODoRgwMIahyIokhscBhjcIBkgsTQ3CAMQwFYNxhDUIJVDAIJYDGT5YDAdo9hsL5HjcPoBA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = ""
    ))]
    LockKeyhole,
    #[cfg(feature = "lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIBoGUaRnGgdIKDEMYNHcaRkHQaIVDiDYJCKFoNHIeIKgwIImCIMwiD4PAvgOBYuHAYYcCAZIKE0NwghYVg3GENQgkEMAgkQMY8kQMB2DSLYvjSHA+gEA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[cfg(feature = "log_in")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNBhDIIITDAIIWDGFIUHYMYRhOFYXCAMQth8aAtDQIg+DwL4DgWKhwG8bB5GwaRuGUIIwjUdBzgkMYYDeIoNDGE4+CAN4piuMIyjSNoqkyNx4DKCQzCIIB5lIIpDlUeQxj0MpVHiXZZDWSAvk8PoBA",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[cfg(feature = "log_out")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgxEgNRhDKDQgDCFQgDELQyhoVoRhOE4WhYMYNhoaA0CIPg8C+A4FikcBvGweRsGkbhlCCL40HQc4JDENoYDeDYjDGE49CAN4oiqL4xjONYpkyNh4DGCYOCIIB5lIIpDlUeAygkOZVHmXZZDKSAvk8PoBA",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[cfg(feature = "lollipop")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCKCR5gyDoQHKDA4CIPg8C+A4FgeGhwGEdBoCAZIMG0MgxCCKQtDQLgzi2L4ZhuIYjiCIokiYIhNg4IIOGEMorCAMJDkUNJGCCR5EksLQ4kUNgglCS5FDGQQwjML41GgPoBA",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = ""
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgwGgMBhDKDQgDCFQgDELQyhoVg4hKFIWhYMYNhoaAxDKH4TiGGIUDIdgxhGE4qheGYyhAIg+DwL4DgWOY8gaCIKDiGA4FYNIpheIokDIaJHjKSYsjKLw0jiOo/j6BJAgkTYwg2D5UjmO5ZjkYxpHIYxsGUIBjHmCYOCIIBym6cBjHiCQxDaVQvmWZ5pmSZpomqcgiDKdJ2CIOJ0m2hAwnqfKBD6AQ",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "m_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINgkIgzg0dxpGQdBohWFwgHmCobD4PAvgOBYoHAYYgCAZIKE0OAgDENhWDgbA0CCPA0C0NB2heKAvi6IA+gEA",
        categories = "transportation,maps,navigation",
        tags = "metro,subway,underground,track,line",
        contributors = "danielbayley"
    ))]
    MSquare,
    #[cfg(feature = "magnet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ1C0NIQCANguDeD4UDcNxhDeFQ5CCHA3h4MAgiMMYNiYMQxEwMQzCAMgyGyEIShQMw5C2NA2GEMguDENIujyPojkILQzkQTIMg4Ig+DwL4DgWS5OgaCIKDUIA4CCPg0kqTJRlCBJSgkbQxDKDZVlmW5Nl8PoB",
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
    #[cfg(feature = "mail_question")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDALg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHYMQyGOGAxC6LQ5iKMQyGiKYRCIPg8C+A4FjiO4GggIhtgwIA3C0OAuDkNwgDULg3GGLw0g6SJRhqDocC4MA2hkTINDeN45j6PYEj+CRNDEOIOkwMg4GMLodC4NIRC0LpHDkLQxhWLoinqVQxnsNpwm0M5whGdIRlIM4ki0M4mh2FpfjqY5igWB5lDKGIMHaWAxpCYYBA",
        categories = "mail",
        tags = "email,message,letter,delivery,undelivered",
        contributors = "karsa-mistmere"
    ))]
    MailQuestion,
    #[cfg(feature = "mail_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDILg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHaDxjhgMQuiwOYijAMhoDeEQiD4PAvgOBY3jqBoICIbYMCANwtDgLg5DcIA1C4Nxhi4NIOkeUIag6HAuDANoZEyDQ3jaOI9jyBI+gkTQxDiIgxGEMwgmuK4khuWZrm2b4kDYdgwFqXo5mKNxjGkchjGwZQgGMeIJmYIqEHmhw4okcoJDOep+oCgphgWB4JkGFodiwNQtp2epggE",
        categories = "mail",
        tags = "email,message,letter,search",
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
        svg = "gAPByGUYx0CAeQ9CINAiCAaBlGkZxoHSCQxDaDB4gkMoMHcaRkHQaIZDCDByhgIoaD4PAvgOBYoHAYYfCAZIJG0MgyCANwtDgLg5DcIA1C4NxhDGOw0CCQw5kUMAgkoMQtDILgwDaSxMjYNwiigL4uh8PoBA",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA0hEIIThWDoYhgVg5C4NRDg0NwgDQIA1CANoliwSAxDgYwyC6HorDELg4iqKh2DgWgiD4PAvgOBZDHAbxsHkbBpG4ZQgkmTh0HOCQxDULA5g4OJZlsLAxDGQpEkmS5Nk+SIEgaCIKi+LYmlqLZgCCKZyiQNRWg+HIehaGIdHYMJikWaZDmaUB5DKVqACAeQxokIggHiiAiDejx4o0Ig2oGhQ+gEA",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAeIJDaDBoGUaRnGgdIJDEM4MHcaRkHQaIYhAIByg4IgyCIPg8C+A4FikcBhh8IBkgkbQyDIIA3C0NwuDEIAzC4Nw4GMLQuDWOwzC0MQuDKP5IkoOAgDATA2jiKIqi+H4ujAaIygkTY3DgdgxDEYwwCCSpKDkII3mwaAxguKQvlgaA+gEA",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Mails,
    #[cfg(feature = "map_pin_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NAzCCDIOEEOAuDANgghSFggDCG4dDQIAxDAY4chcOIgDKGInGEMg5C4OYfiyLofhyNIQC0NQiD4PAvgOBY6j2BoIgoMYtDGJgxDMLg1DKEwuDaJZOheNYcDKHIhGGJomlMLQ4lwIA3C4OImmCYodjQLZJkuYQyjmO5Aj+BJBgkTZFg+dRBDILg3iaeZ7maHQ5iAMBhg+D5TCChQgn2fJ6lqf5hDcLQukibY8nGcIFgecwxDSLggi0Mg1oSiJ/DALZ5DENanpMNqVm8PBsGkbhlCAeAygkMpsrUMa4CIIB5rcIq5r4ea8sKlaxrMPoB",
        categories = "maps,navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[cfg(feature = "map_pin")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAY4NDYLQ4g4MoUhYc4UC2Ew4C0MQyGGFYVg2DQxg4NggDAWgiD4PAvgOBYuGMaRyGMbBlCAcoJDMIggGMeYJg+PhjHiQgyi2L40jaOA+gE",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[cfg(feature = "map_pinned")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4GMMAgDQLg1C0NggDmFoYHOFgthOFQ2C0ORhheF4RhEMQgDEMggDAIg+DwL4DgWMBjGkchjGwZQgGMeIJisIo8HmCQ4kEcoJDKL4xjaOI6jCM4GgiCg4C4OAzDWKg0EgNRhimKYni0LQuDkLg3GwLYsDYY5iDELgxmwLgynAM4tCALg2C4NIqnsMRogwY53nWb55ioLZfmyYZxmyYgzmecg2l2e5gDCYoZmUaAtDOVZXkqMoEGgPoBA",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "danielbayley"
    ))]
    MapPinned,
    #[cfg(feature = "map")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgzCANggDkIISDENYTCAMgxheHIeDEOAghmIIViSI4Sh0Ig+DwL4DgWBxui0bIMGUIB5DKEIiCKOAxhAM49HiPwiDmQo6kWLIujUbhljSNo+hANo9jmEIrCAeJIhmQpEluLQvkyToBA",
        categories = "text,maps",
        tags = "location,navigation,travel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Map,
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDUIAzCAMgxhSFoYDkIg+DwL4DgWB4Jh6IYGgiCoMG6DoQDmGoZheGYSh2H4miMZYejcIB5DKEAxDAIggHgMYQheQR5kQIgzkEeI9hENI0C+N45iiQpJkuO5JkaO5ODGUJCl2QIelKKA+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQ5C4Mg2CCDIOCAMwgDEMR2DMbAxDQLobhOHQ0CIPg8C+A4FiOJoGgiCgyDGFQ1C4Mw0FYNhsC0N4xi4MguDAM4iiSKYogSKoJE2Fgug8MQ2C4OBhhSFAwhWFQtjAOAtDGSI/iWQ4jGwaRuGUIB5DGCQyCIIB4mQIpmmgMplmweZumuZojC+XpgD6AQ",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification,disable,silent",
        contributors = "karsa-mistmere"
    ))]
    MegaphoneOff,
    #[cfg(feature = "megaphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQxg0OAtDUdgxDITIMDENB2C0Mx6CIPg8C+A4FiCI4GggIhNg4Lg2g0NguDgYYMgwMINg2EowC0MYsh+IYmD6AQA",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification",
        contributors = "mittalyashu,ericfennis"
    ))]
    Megaphone,
    #[cfg(feature = "meh")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bGwaRuggeAxgsOIPHgMoTDaDx5iyDA1i+J4yCKGwviKJIhiOJY1DmL4xkAIIrgsOQuDAMY0kaN4cjoZY8iQIIwkyRIxDENZIkqU4/iqNZYk2OY9D6AQA",
        categories = "emoji",
        tags = "emoji,face,neutral,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Meh,
    #[cfg(feature = "memory_stick")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ5HYLQzCIPg8C+A4FhSF4GgiCgxDCDYPhGE4VhqGYEhuCRNDENIghCEoUhaJ4mgWB4pDEOItiKMIlgKJ41gqOAxDEVg5iOMYYj2NIciqDJCkSRo8hqP4qDKDZDkWO4ykmKIKlUMQ1GgMgwlCWpSkuVQ3GGVZVh+HwxCAMgtDIaAxDaapwCCbYNngMh2DELgxnebJ5oSHwzC4OAzDcVgxmma6Fg2cpwEgNKCpAMZynKEA1oClp6m2EaIooWpkhiAQ",
        categories = "devices,development,gaming",
        tags = "ram,random access,technology,computer,chip,circuit,specs,capacity,gigabytes,gb",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MemoryStick,
    #[cfg(feature = "menu_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEcoTCIMoRHmFIRGgZRpGcaB0g2DwiD4PAvgOBYoHAYYLCAZINE0NwgDgaAxDCJ4pi6C4ti8aIxjONQxDKOI6igL49GiP4wjIIo0CAMQ2keO5KkAPoBA",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0Ig0CIIB5gkIgxDKDR4DKCgyDCDR5haEYTD4PAvgOBYfiKBoIgqDIOhwNoahCLIHhyGAih+IYEGWJI2geEIphWF4Zg6EAxDiGockKM4giUPoBA",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options",
        contributors = "colebemis,ericfennis"
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
    #[cfg(feature = "message_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMgxCAMQuDkLQ1C4NxhDgLg1CCGYbDCEIgDMLg4CCIg4HoIg+DwL4DgUPoB",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[cfg(feature = "message_square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhWDUYwwC0MQuhQOQtDIIAyhgaAyCIPg8C+A4FiCI4GgiCgxDEIAzGgM4fiGJolgSJ4JE0MQ4iwaAxGOFIrDCGggC4OZBh6IIijSM4FgeNgyisOR2kaMZJgKNJMgqTggDGEJAj4LZDhqGIaGiE4wkiJJVkuKI3DSWg3mSL5HjKaY1CIbQ3m4LZtDQdgtDWZpziaV4LloMp9lKZxoD6AQ",
        categories = "account",
        tags = "comment,chat,conversation,draft",
        contributors = "danielbayley"
    ))]
    MessageSquareDashed,
    #[cfg(feature = "message_square_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNhsGkbhlCAeQxgkMQwCIIB4kIIg5kYeQykORZHk4IoPjWN48j6O49j8eJSDEMpMkkN5Ml0M5GkiQ5fjYL5XGUPoBA",
        categories = "account",
        tags = "comment,chat,conversation,add,feedback",
        contributors = "danielbayley"
    ))]
    MessageSquarePlus,
    #[cfg(feature = "message_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgUPoB",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    MessageSquare,
    #[cfg(feature = "messages_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA5GEMgghEMAghQMQthEMhIDYbAtg0NBWDQYwwC0MQuiYOYYhKGBoDiEIShWMQxjAMh2DUWgiD4PAvgOBY6j2BoIgoMQ4g4aAyi+E4yjQdgxDGHQ0h4aAtDaSYxhaGIYHaJY5juQA+gEA",
        categories = "account",
        tags = "comment,chat,conversation,copy,multiple",
        contributors = "danielbayley"
    ))]
    MessagesSquare,
    #[cfg(feature = "mic_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIA4C0OQuDANAghEMA2GEMguDiDYahwIAwCAMYgCAMwuDkOIlicOBMDENoiDIIg+DwL4DgWMxjGkchjGwZQgHKCQ1CIIBjHmCQ3kMYx4gkMZIjML45juPQ+gEA",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Mic2,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2D4JguEx5hGCwiD4PAvgOBYcHAYR0GgIBkgoTQxDgLg4DkIAxDMLgyDMQQ3C4MQyCCNo4CAMI9j8MYujgdgtg2HAviKJIhiOJYnCITQ1i8MB2DIYQ3jqP4+j6PA1huHZJGiS4kiaKAxlEOQuDMNBWDUYQzCCb5aj0LQ1C4Ng4C0MZpDOXpIkyYpNiiLg5HYM5unCWY/nWPAyjeRpfn+AoEgaEIKjiDoYpYOaYhWDIOHiFaXkeHxlD6AQ",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMwghEMAghQMAthEMx2DeEIShWH4UDaFRWDWHYTiCGIYFoIg+DwL4DgWLYwgaCIKDEOQgDEMB2g8Nwgj6Fo5C0MQ0hUdgtDKLIujOLRsGkbhlCAeAygmDAilIMZVkkIB5lkIo3lceZUCIMpJi0L5OlAPoB",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,listen,radio,podcast,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Mic,
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
        svg = "gAPByGUYx0CAeQ9CINAiCAdxpGQdBogkMgwgwaBlGkZxoHSCQxDWDB4hODByiEIgyCIPg8C+A4FimLIGiSHYWhiGocCIN4Mg6EISCIOIMgiPYggkNooiqL4pHAYYRCAZIJE0MQ4CAOB2jiKQvkmEZIkoaJMk4NggDEOR2ieVpYGiWpLk0IpPlGYZjkWV5bD6AQ",
        categories = "food-beverage,home",
        tags = "oven,cooker,toaster oven,bake",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Microwave,
    #[cfg(feature = "milestone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2EgNRhDIIITDAIIWDALYTDIdgzhKFIXiGFobGgMQzGwNAtDMLg1EyDIOFoIg+DwL4DgWM42gaCIKDGE4mHYOIyjSOY4gSOoJguEwzh2Qo1kYPoBA",
        categories = "arrows,navigation,development,gaming",
        tags = "signpost,direction,right,east,forward,version control",
        contributors = "karsa-mistmere"
    ))]
    Milestone,
    #[cfg(feature = "milk_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhoDgIg+DwL4DgWEoVgaCIKDmDR2DELgzDQMxNDENYdDILg3DgORhDQIIuDAIIxjELg2DcMoNC4MgxDkbI1DUNguDkOA0i2L4ykgMQgjWN45DIMoeC4MQzDETQ3C6DJXDgbAtlIMg4lIOQyEGLowkiMQ3CAMQwjoMQyFYMgwGGOI4jOSJ0GgNpzg2Z53C2UAtDOEYThiF4EhmCZWmoNRhkENJpo6aZ2kqJoxpGOKXn2MQzC6Ig1p0Mw1oOFKHhIbBpG4ZQgHgMYJDIIqrDKrqvCAeatCKtB5rKuKvhIL6nqkPoBA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CAMYSDEMIThWFwyDAIg+DwL4DgWB4Jh6IYGgiCoMG6DoQhuE4XhSL4YCANIdh+JojGWHo5CAeAxhCFAij0MotDGQh5j8IoWkeRAiDONgvjmO4oj2SZPkOQIcCCSJFkyQI1h6UYoD6AQA",
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
    #[cfg(feature = "minus_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgg4aA4hmG4hiMPoBA",
        categories = "maths,shapes",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    MinusCircle,
    #[cfg(feature = "minus_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIBoGUaRnGgdIJDEOIMHKCAiDKDB3GkZB0GiGIaD4PAvgOBYoHAYYkCAZIJE0OAgDEMhoieKYuiQPoBA",
        categories = "maths,development,text,tools,devices,shapes",
        tags = "subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    MinusSquare,
    #[cfg(feature = "minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FD6AQ",
        categories = "maths,development,text,tools,shapes",
        tags = "subtract,remove,decrease,decrement,reduce,negative,calculate,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    Minus,
    #[cfg(feature = "monitor_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMoQCANAtDQIg+DwL4DgWGByGUYx0CAch4gkMgiCCJAiiYIB5gkM4nHcaRkgWJQwicaBlGkZxoHSCQxhaGAvh6IIYhuBoICITQxhEMQ3HaQIZkaRYEkeCRNDiEAxGgOIXlGVA+gEA",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ5CKCR5gwNoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MgyCAMQyHYMxhiqKgwCCMwxC2MRIDSMAgjKNIrjeNxWDWO49jWPI3GiD4aC+IYjiCIokiYIhNiyKw3HYNIZhuTRok+I4licOI8DEaA4lqTJQD6AQ",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GggIhtDENYODALQzCAM4jiOE4VhqFByGUYx0CAaBlGkZxoHSCQxDQIggHiCQyjsdxpGSBY+DCOx5gkM47HKPQij+FAvi2L4ZgSG4JguDQxDcdo6lCK4ClWB5XDgIAyDEaA4imFpVD6AQ",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,download",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorDown,
    #[cfg(feature = "monitor_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIIMEgNBhDIIITDAIIWDELQyhoVg1GMMAtDELg1g6GolDIIg+DwL4DgWKotgaCIKDKEwxDWHYShSF47iCGwyEgOYpiuMIvgSMYJE0OIUDEaA4kKLJGkWBYHkgMY1Dcdg0k+RICkaVAiG2E5ihYMgwluUYBA",
        categories = "connectivity,devices",
        tags = "share",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    MonitorOff,
    #[cfg(feature = "monitor_pause")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDMVg3CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIUHIZRjHQIB4gkMgiCAdxpGSBYuDCMB5gkM4wHKLQii8IBoGUaRnGgdIJh6E4ViiKoZiWB4JgsMoODcdg0kqJIYgKT4cE0OAgDIMRoDiWIaD6AQ",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIA3CANQgDMLYRDMWgiD4PAvgOBYZHIZRjHQIB4gkMgiCAdxpGSBYlDCJxoGUaRnGgdIJDENInHmCQziccokCKJoZC+H4hhmHIGggIhNDEMggDENx2jiQpHkaBJIgkTQ4CAMgxGgOIYhqVIBA",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdg3h+FYiiAMhoDgIg+DwL4DgWMIzgaCIKDGFgxDkdgtDMLg5DYIJADENYvjGNo1gSN4JE0NwgjwaJHjCMpMjAchlGMdAgHIeIJDIIggHcaRkgWCQ2mKXwiDGaQgHmCQxmEIBoGUaRnGgdJxDCSAvlmWw+gEA",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NQgDIMBIDgIg+DwL4DgWFYYgaCIKDENwgDkaAuDAMYUhaG4VHIZRjHQIBoGUaRnGgdIJDENgiCAeYJDSORyHiCQyjmQAiDGQggHcaRkgWNgwicL4ri2GoEhyCRNDgIA2EgNBhDKDwgDCYJgC2XgyHYNpdl+YZrl8Mhoj2FYXlSFRjGkchjGwZQgHKNo5GOO5FDWfpEh+T51neeQ+gE",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB5g0N4QGgZRpGcaB0g2DwgHiDQ5CIPg8C+A4FiOJoGhIIgzhWF4ZhsIgxDSEIfCIMoQgiCoMjcMIQHKNo4iOJYEHSIxwGGCwgGSDRNDEMggDENx2jSQ5IguR5JGiS5NDgIAyDEaA4iKJJXGgPoBA",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMwtDOEIQCIPg8C+A4FheGoGggIhNDEMoNDMVg3haGIdhcchlGMdAgHmCQzCIIB4gkMo0HKNgijgIBoGUaRnGgdIJDENI0HcaRkgWNwwigL4si6HIEh6CYhiMMQ3HaR4XhmVJTgWB5WDgIAyDEaA4k+KoB",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CAMQyg4LQ1hQIg+DwL4DgWGIbgaCIKDmDoQhKD4VDWF4Zh6GByGUYx0CAeIJDIIggGgZRpGcaB0gmDI1HcaRkgWMwwjUcoyCKNAgHmCQzikL4ti+HYEh+CRNhGEA3HYNJPiuApUgeVg4CAMgxGgOJdlQPoB",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDODB3GkZB0GiCQyDCDBoGUaRnGgdIJDENIMHKCIKCIPg8C+A4FikbBpG4ZYHDGCQ4gweQyhcMY3jSCo7geOQiDENooiqL4xi6MIyHiQQxguDY9DEN43kEMo/HiUYLikL5HGUPoB",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,virtual machine,vm",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    Monitor,
    #[cfg(feature = "moon_star")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzGENgghEMAghSFA5CCGIahUIAxh0LQ5iAWgiD4PAvgOBYliiBoIgoMYYDMdg0iSJoriqBIsgkTQyh4NRoC2M4lieOA+gEA",
        categories = "accessibility,weather",
        tags = "dark,night,star",
        contributors = "karsa-mistmere"
    ))]
    MoonStar,
    #[cfg(feature = "moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzGENgghEMAghSFA5CCGIahUIAxh0LQ5iAWgiD4PAvgOBQ+gE",
        categories = "accessibility",
        tags = "dark,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Moon,
    #[cfg(feature = "more_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMYQGMeITg8Pg8C+A4FgeGodgaCIKhiEISg2FYXg0OQihqHIEiKIIwgeCYqDWFYLg2D4RhOLYbiGH4BA",
        categories = "layout,development",
        tags = "ellipsis,menu,options,operator,code,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[cfg(feature = "more_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMYQGMeYTg8Pg8C+A4FgeGodgaCIKhiEISg2FYXCINQihqHIEiKIIwgeCYLg2D4JioMQ5iaE4thuIYfgE",
        categories = "layout",
        tags = "ellipsis,menu,options",
        contributors = "colebemis"
    ))]
    MoreVertical,
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
    #[cfg(feature = "mouse_pointer_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CCDA3C4MA3CAMYSDILg1DELYPDMORMDIMYTDGEA3HoIg+DwL4DgUPoBA",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ericfennis,csandman"
    ))]
    MousePointer2,
    #[cfg(feature = "mouse_pointer_click")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CCDA1CAMQyhALg4C0NQuDITAyDGEA0FoIg+DwL4DgWIYkgaCAiE0N4YCAMotDgIIXDGIIiieJoEiiCRtjMIIVi8OQthSNYjjmOIFgeCRNDENAgDQLochEIA2kSN4CjmSYKDaEAyC0MQugwMpVkaAQA",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    MousePointerClick,
    #[cfg(feature = "mouse_pointer_square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxhDIIIQDAIITDALYQDIIg+DwL4DgWG4egaCIKDEOYNg+EYUioMYphmG4dgQaIgjGB4JG0MYQjgIA0CAMYTDELg3C0NAuDMTAyjkNhahqHIhjOBY1gqDAyDGKISiuF4XkyMIfgKNIjE2JgzGgMZbk6XpQmCJpUGiLpNjGT4igkTQxjyY5li+Z4hlETQzCAOR2nib5dnuYJUn8dpulyMponKCp+nWgZmnCAQA",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = ""
    ))]
    MousePointerSquareDashed,
    #[cfg(feature = "mouse_pointer_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDEVg1GEMgghQMAghcMAtDKGxIhKFIWhiGIbhUdgxDSE4ViKGYqDIaA2CIPg8C+A4FjKNYGggIhtDGFI9CANIOhcMQuDcLQ0C4MxMDKPg2FqMYzjgPoB",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    MousePointerSquare,
    #[cfg(feature = "mouse_pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCCDA3C4MA3CAMQ2C4OYSDILg1DELYPDMOQgh4OQthmGxMgwMx6CIPg8C+A4FiyL4GgiCgxgyNggDaOYri2Mg+gE",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ashygee,ericfennis"
    ))]
    MousePointer,
    #[cfg(feature = "mouse")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAdxpGQdBogkMQ0gwaBlGkZxoHSCQyDCDB4gkNYMHKIQiDcIg+DwL4DgWKhwGGEQgGSCRNDEMggDYdoVioL4whEPoBA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig1CAMQxCCEoShSFgiD4PAvgOBYHgmHIfgaCIKgwboOhAMQ5hMM4Ti2LIujCG4diSIRlhyOAgHkMYQDUIggHiPoRkEeQyisOZBHiSAiiyNQvjgPoB",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDMIA1CAMQ5hSFoYDEMQiD4PAvgOBYHgmH4igaCIKgwboOhCHIahmF4xDOHogieJBlh+OAgHkMouDkIggHiPgiDWQR4DGP5BHmSZFjUL44D6AQ",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDgIITDIMggDGF4ShgNgiD4PAvgOBYHgmH4igaCIKgwboOhANoUCCGoXi8MYdh+IYEiiJQ8iSCh4DKEIWCIIB5DGEIZkMeZAhEMpDHiRgik2N49D6AQ",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig4CAMYSDEMggDKFwxDaEw4CIPg8C+A4FgeCYgiOBoIgqDBug6EIShyFoYhOHA2h+IYoiUZYgjoIB5DGEAyCKPgykGQggHiQAihaQx4kWS5CiAL46D6AQ",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig1CAOQgDIIAxhaEgxDUIg+DwL4DgWB4Jh6IYGgiCoMG6DoQhSGoWhaGwghyHoggSJ4kgKN4jikb4Ng8IoyDGFIYhWFpEDmHYfiaPIljuKILj6K5AkOE5GheMZEjSS5PjmPAgHgMoQDIMgiCAeQxhCGJmHiaQimWZ5ikGZY1k0PJfmGapwHmcpkmaaJjmybprnWKA+gE",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ4CIIBygwNIQGMeIMg8Pg8C+A4FgeGhwGEdBoCAZIME0MQyCCDhWDIbA3CCFIaC+IYjD6AQ",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNIQGMeYMDEOAiD4PAvgOBYHhwcBhHQaAgGSDBNDENgghkVoPhwL4jiUPoBA",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWG4egaCAiG2DIMhOFYahyIYbGMaRyGMbBlCAcoJhkIBjHiCQ2CKOB5gmDoqC+LowjKLYvjGM41CKN45kAOI9GOPwiDGPIbkOSJGgE",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWGxjGkchjGwZQgGMeIJDYIggHKCYZiceYJg6GociGI4liCIokiaKIzDiLBjjIIgxiuLYvjUL43jwPoBA",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4MwxCCDIOCANQgDIMRsDcLQ0CANwgDQLQxC4MYZDOIg3CIPg8C+A4FimLIGgiCgxDQLg1DMIA4C4OA4CAMQyhUbIgiYIIliOKIqi+KRsGkbhlCAeAxgkMgik8MpSlMIB5lEIpYHmVpclOKQvkuTQ+gE",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Navigation2Off,
    #[cfg(feature = "navigation_2")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDIIITDEOYUDEIIShoNwgDWGIahMMgiD4PAvgOBYHG4PoBA",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation2,
    #[cfg(feature = "navigation_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NAzCCDIOCCDwxDEbA4CAMoZhCGQuDUNwtDWDQzCIPg8C+A4FiaKYGgiCgxDcLgzDkIIVC4N4PDKGgyGwLQ5jeDw0C4NgxiWJ4siYbBpG4ZQgHkMoJjoIpODGUZTHiUAilIIB4lWWpGC+SpMD6AQA",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NavigationOff,
    #[cfg(feature = "navigation")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgzCAMQxCAMgyhaE4SDKFYUhoIIShQIg+DwL4DgWBxuD6AQ",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation,
    #[cfg(feature = "network")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ2CIIByHiCQxg0dxpGQdBogmDAghCCoaGgZRpGcaB0hkIg+DwL4DgWJ4qgaFIWhgIoeiCIokjKDYPhGDYcDKDYIh2Jooi2LIEi6FYXiWB4Jj0IIfiGI5JjmCo7gkOZBimRYnHAYYXCAZIJE0NQggsdgtDMYQxmMIAwmuagxC0MRoDEMpomqbJsmmeR2DOV5bheWpcGiXpgnOYwyFYOJ9oEPoBA",
        categories = "development",
        tags = "tree",
        contributors = "ericfennis,johnletey,csandman,karsa-mistmere"
    ))]
    Network,
    #[cfg(feature = "newspaper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ2GEMoNCAMIWhgMgtDIVg0hOFYXiGG4bEgOIfhSIYWhuDR2hGJ4YhcMYrDIWhtiGL4pjKGgyHYLQ5GMMAtDELpDDmM4bGgMgiD4PAvgOBZMk+BoIgoMQ4CAMQ0GgLQ4kuTZSlGBJTgkTQxDWWA4lsNZek6YphgWB5kDGFw2GgOB2lqXBWDYWpsmCAQ",
        categories = "multimedia,communication",
        tags = "news,feed,home,magazine,article,headline",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Newspaper,
    #[cfg(feature = "nfc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAOAuDMMhhDcLg0DMIIThUIAwhoIAxhyEwzDYIg+DwL4DgWJIngaCIKDmFIMDYLgyDEYQxDELg3gyNo4gyG4bh6P43DUOIjiWKopgSK4JE0MQyC4OYeDQLo0DENZPh6VZXhyPwgC4MJYlaRIkiaSZIgWB5LDGMQzDcIIRDIMJTgycJyluHYcnCRZkiiAQA",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Nfc,
    #[cfg(feature = "nut_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0FYMgiD4PAvgOBYThaBoIgoNQgDEMB2DQYQ3C4MAwDQIIkiaKAwCCLYtDULgyDcNwgDYLg3DgNxjC4NIMC6Hw0C4OAwDKMg5g0MZAkULg1kgTIMCAMgyGwLg2DIMQtlaWI8DOWgzlYOZaDQOA4h6SwylqTgyiOJQximbounKLZCgwNQtkYOQzDmEoUhmGIEhqCYLDmHogmAMw0DOfYVoGgIFgeg5RgwYwtkqiZik0N5ekoOQwneSomneRqjmoNI3p6aahmSZwzDWXqklKlpuDiWg2DQNKzqeswwrWaaqiUMg4jiWZKDYOQuDkMa1DOHpnr2xYlqySg3DUN54iWzZps2QA5rmOA3jcOK5koNQ1m+1JikaWBtDGxg1iiMQ5DmO5DjqY7WmewpaDiypnuYNqzjmNahDOeQxDaZw2DizZKDG+cNrMMr0rwM8BtQNa+tmeAtl6Tccrmua1t8N4sroNqhwu2MPt2EYTo2Fw8GwaRuGUIB4DKCZTCLNwxzrPB5zkIs7CAec+0OjMzzUPoBA",
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
    #[cfg(feature = "octagon")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig3C4OA2CAMggDENguDENIWhaF4ShSH4YhqHIkhuHQyiCE4ViqHoZiiHohhWM4WCIPg8C+A4Fgcbg+gE",
        categories = "shapes",
        tags = "stop,shape",
        contributors = "colebemis"
    ))]
    Octagon,
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwM4QGMeYMg4Ig+DwL4DgWB4bh6BoIgqGA5hCEgig+CYXCINYahyIoggKBIjiyJoVguLoogyD4bh2NYzHAYR0GgIBkgwTQxDALg0CAMgxC4ORhksIJVDAIJYlgOZSDQMQtDENZNDENowC+Q5FhuaJGkgIpKDMLg1k8LgxlSWJXlmWQtlwOJelaYpemWP5rD6AQ",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "outdent")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig3CAOAgDMIAxDIIISDENgiD4PAvgOBYHgmH4kgoeAyhAMQxCIIB5DGKwyi4eIxCIMoti+KgihiHogieJoIgqMIQh0II1hCOI0juLIuHmO4dh8L5ADyJ46isOJOjYMZZkeTI5kiN4tlKQIBA",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Outdent,
    #[cfg(feature = "package_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAORoDEOB2DEMBhDIIIWDAIIZDELYWDISA1hWF4aiSHAyh0Vg5FoIg+DwL4DgWLYwgaCAiG2DA5hcLg0DULQ0C4ORBh6JIbCANwuDINAgDMaA5C4NQyiKGIlCAMQuDiVQuDETAyDGDYsi6M4ygSNIJE0MYWDMdg2mCL5kD6AQA",
        categories = "files,development",
        tags = "box,container,storage,sealed,packed,unopened,undelivered,archive,zip",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Package2,
    #[cfg(feature = "package_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIIMCAMoQCANAtDQIg+DwL4DgWGIbgaCAiE0Mgxg4MBWDgYYRhEMAgiwMAtDGMAuDcMxsC0N4VimEoui0LYrjYN4TEGKotkWLAzCAOB2iiRI8iyJAxjONZBDSOorkaOxsDKMgxhaGIagQaIdmGB4JG0NwuDWEwuDKQQ5CANQuDENYXhmHodG8bB5GwaRuGUIBwG+fR0HOCQzmyb5BDGEaLhAMIziQN51hqeZ7n0ZYYnyfggHgMYJosIqcDKnwyqEeajCKoAgHmngiDKpZfpqmIB",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task,delivered",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[cfg(feature = "package_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMGgNgiD4PAvgOBYThaBoIgoMgxg4MBWDgYQyCCJAwCCJwwC0MYrC4NwzGwLQ3C0NIjiWKI4iqJoxDcIA0EGJImjkIAzCAOB2iKQZDieHgxi6MI9jWSopjiOwyi0MQ0hKFIZhiBIagkbQ3C4NY+C4Mo9DkIA1C4MQ1luFZfhgbxsHkbBpG4ZQgHAb54HQc4JDOZ5qj0MYkoaJQwi6Hg3nCfJ1neeYTpGeh5DGCQyDIIggHilwioamx4DKCagCAeajp+moTC+lA+gE",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[cfg(feature = "package_open")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg5DEIA4C4OA0hELg1DYIAyC4MgzGEMYODMIIgDmIgwCCJwwC2IA4hAMBMDMLoQDSMoehsMQyhqMo5imKAtC4MA1CCMQ2DkbI4hyOQ2iEYY5jyKJQiSFYvDKEJIDUMZNjuOo4lCKY/DAOQtkQNxaCIPg8C+A4Fmia4GggIhtjGYYWhSI4bDMNQtksNpZiSIp/l6I4jhOLhskSQoklqXY3k+J4QjKQwukUTISDegI0DcNZNhqggxiuDpTjCd4XlmjZco6g5AmKZJmmiaoEGiaBsGkbhlCAeAxgmOAirgMq7DKvR5roIgysEIB5r8IgxDOZ5prStptrGB4Jguj4xDUdoxDMN5aDCGYbt+nqgDGVoTDMbJ7pIMA4h+IaEiW44gDcOIoukNpjkAOBBuG4JAhmPYziOSw4DcVrMhezqwmyAQA",
        categories = "files,development",
        tags = "box,container,storage,unpack,unarchive,unzip,opened,delivered",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMGgNgiD4PAvgOBYThaBoIgoMQ5g4Mx2hGE4VgQaIYiWB4JE0Mgxg4MBWDgYQyCCMwwCCNgwC0MY6C4NwzGwLQ3C0NIyjSN5HjmNZADcIA0EGM41kgIAzCAOB2jGUJSjaLQxj2P5MkSWY4keSgyjwMQ0hKFIZieBYpCIbQ3C4NZNC4MpMh4NQuDENZqiSF4CG8bB5GwaRuGUIBwG+hh0HOCQznaHpMDGM6UjQMI9i0N5+oqg6FoeE6fogeQygmlAiCAeKlCKp6pDGpgyqgeavCIMqxiOog+gE",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDAVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoisModDENAiD4PAvgOBZMk+BoICIbQ3C4NYuC4MotDkIA1C4MQ1kuTZSlEbxsHkbBpG4ZQgHAb5sHQc4JDOW5ei0MYUnqFQwh6DQ3mSTpomqbBlkya5tCAeAxgmegiCAeaNCIMgypAeAyo6lqRpkIqPkwL6JocPBjGkchjGybhjHijg4likBygkMqvCAYx5o6YJjqCpanqmUYElOCYLn6XIOlexaVg4OaCmaAQ",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDAVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoisModDENAiD4PAvgOBZMk+BoICIbQ3C4NYuC4MotDkIA1C4MQ1kuTZSlEbxsHkbBpG4ZQgHAb5sHQc4JDOW5ei0MYUnqFQwh6DQ3mSTpomqbBlkya5tCAeQygmegiCAeAxo4MqQHmkwiDKlaRo0IqPkwL6JoeAoElOCRtDGeY1lkNRtC2WYXDWr6CmaAQ",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageX,
    #[cfg(feature = "package")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3C4NQgDQLgyDcIA5CANQuDENQiD4PAvgOBYch+BoICITQyDEIA4GEMggiwMAgi8MAtDGMwuDcMxsC0NwtDSK4tjCQIyi6OYTDQQYsi6QQgDOKR2iqSJKi+KAxjaOJFj6SYxj8MBsjuRpQlqL4nCAMQ2FqG4diKIYEiOCRtDMLpMhMOI2haKY2C2Goch6bJrgWB4JE0MZIDIVqDmifIggE",
        categories = "files,development",
        tags = "box,container,storage,sealed,delivery,undelivered,unopened,packed,archive,zip,module",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Package,
    #[cfg(feature = "paint_bucket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAxDELQ4hGEQuDYIA4hUYQyCCGwwCCHoghwLg4GwNQuhuJgyGOI4jhwIItDKLQwEyDIODEWgiD4PAvgOBY6j2BoIgoNYukQNY5juQI/gSQYJE2GwxDMaAxkeOo8kyS4FgeTgyhsMgwhqLoeDGDgtDSHxjDALQxhWDguDcLYxmcMpmi8M5uhaXgunMIA0jiVpKgEA",
        categories = "design",
        tags = "fill,paint,bucket,design",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[cfg(feature = "paintbrush_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDkLg5FYMQ2GgMxhDIIIZDAIIchwMgtDIdohEgNR2DIY4cDELorDmGovDKFh2DOEYYi+KodCCDQwFoIg+DwL4DgWP5CgaCIKDaDgyFaMQxiIMQwj6QJFkSBJGgmC4NiINJSkGVpVgWB5YlCGonl2VIBA",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Paintbrush2,
    #[cfg(feature = "paintbrush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLgzDcIAyC4NgzCAMQ0CANxsC0MQuDUOYch4ORhDKEQgDCJ4nC2Eg4iUMBMDiGRsDkII0h2H4hh+JImiiPYqiwMhMDGEAxDAbA0g4NwtkiD4kC4MYlhKUIpDGKgzC0MxaCIPg8C+A4FlyX4GgiCo0DgY4rCCV4YDOHgthANBsjGRRjDKHAgDYLQ1nib5bl2YphgSY4JguSJ7kOHggoaFg1n6XqCD6AQ",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[cfg(feature = "palette")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQzC4NQigkeYMDaEISHKDIXD4PAvgOBYHhyH4GgiCoMDEN4XhOJwwiqGQihuHYjiGAoEiSCYLCIOIqGOFAiimEQgi+MYejaNIziWOYWkGQ5Bj2JwykSSIcHAYR0GgIBkgwTQxDIIAyEOS5fmOYpel0cw0hAIAxDCa5tmwYwuDkMg2CCbwuDYNA4C0Lg3DSdQxniegtoENg4Didp8DQMw3nwMZ7C4OAzDWiqMoQLpdpQLgyDmfAyDinQuoukA2DUMqVnugaZGGhQ0mugp2rEMavDahqXrUOBooEOQ5DYY4PDANazm0NYQDWlJRDUMAzCCxbHpSzg1DQQwyrupQghYMJdmuKZ/rOZpeDIegihwL5VlcPoBA",
        categories = "text,design,photography",
        tags = "color,theme",
        contributors = "ericfennis,csandman"
    ))]
    Palette,
    #[cfg(feature = "palmtree")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA4GMMAtDILg3DaEguDSFg1C0NQuhsNRTDIIIdDINAgiIOBoDIbAxC0MQgi8MRoDQIg+DwL4DgWNo5gaCIKgwIA3C4MQ0EGHQ4iKR4iDAIJMjENoeCANhjDMLgwiaTIdDWJwuiWI5RDUaAtDMbIumaMZiDONY3jyO4Ej2CRNkcOQgDmFAxGOFwxluE57heDYdDQNwtC4M5bkINJkDSXQ0C2iwyh2g4UnehA3i+fQynoMp5DELg5hunQ5hqXaDp0OAtoiE6GHqa44m+boFgecQxjGWhjlGE4bkMNwgouoJSh6MxjpmWqEqCmYtkSrZtgEA",
        categories = "nature",
        tags = "vacation,leisure,island",
        contributors = "ericfennis"
    ))]
    Palmtree,
    #[cfg(feature = "panel_bottom_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHIeINDKEYdCIM4RHmDYjCAcolCKHw+DwL4DgWLRsGkbhlCAeAxiaIAyh4MYkjyDg1iSOZBCKLQvjONYtHAYYLCAZING0MQ1CAOAtDMIAzleV5Gi6TILD6AQ",
        categories = "layout,arrows",
        tags = "drawer,dock,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_inactive")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh4hWEhyhgIgyhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOE0MQ0CAMQ1GgMYnimLoji2LxojGMwxDmN45h6KAvj4aJAjCMgiE0M5HGiSY9kGTpClATZGjiOo8kuWIBA",
        categories = "layout",
        tags = "drawer,dock,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomInactive,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByg4IgygwdxpGQdBogkMQ4gwcoIhSDBoGUaRnGgdIch4Pg8C+A4FiwbBpG4ZYNDGHA1gweAygkMgxgweY8CIMY5geN4KCKLAvjKNIsHAYYaCAZIJG0OQgDEMAgDMLQzlqWpJi2T4aD6AQA",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINHmCgzg0coZguDYJCKGwgHcaRkHQaIVhcPg8C+A4FiwbBpG4ZQgHkMYVDWII4iKIAygoMgxhiPwiDGOosC+Mo0D6AQA",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh5hWEhyhQIgyhqGIdhIdxpGQdBog6EAiD4PAvgOBYrHAYYmCAZIOE0OQgDMdopisL4xiaMIyGiNIOG0MQ2CAMQ1C0M5MjmTIqiyPxoD6AQ",
        categories = "layout,arrows",
        tags = "primary,drawer,hide",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_inactive")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEeYdhEcocCIMgiD4PAvgOBYoHAYYLCAZINE0OQgDENB2DGJ4pi6C4ti8aIxjONQxDkdomigL49GiP4wjIIo0CAM5HjuSpAk2QZPlGRo6kmSw+gEA",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftInactive,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHKCAihEIBoGUaRnGgdIJDEOIMHcaRkHQaIeiAIIPgoIg+DwL4DgWLRwGGJQgGSCRNDkIAzHaH4si6M4ljKNBojaCRtDENAgjoM47C2TYLi0L5BGgPoBA",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMGgZRpGcaB0gkMQ4gwdxpGQdBohmG4NggIoRCCD4KCIPg8C+A4FiwbBpG4ZYoDGCQ5gweAyjiDB5jeKoHjyJgxiuLYyjQPoBA",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAaBlGkZxoHSCQxDiDB3GkZB0GiFIWCAeYJgsIByggIgygwcogiYIg+DwL4DgWLRsGkbhlh8MoJDIMYMHgMYUDWDB5j6Co8jgIgxkCLQvjONYtHAYYbCAZIJG0OAgDkIAzlkLZaguSpPhsPoBA",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_inactive")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoHAYYWCAZIJE0MQ1CAMQ0HYMYnimLoWi2LxojGM41jcOR2hGKAvj4aJAjCMgijSNgzkiPJLkGTpClCUggkeO5KkwPoBA",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightInactive,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEcoTCIMoWHiDYaCCHAihUIBoGUaRnGgdINg8Ig+DwL4DgWLRsGkbhlhIModDGER4DGKg1hEeY9iGO44g6P4tC+M41i0cBhgsIBkg0bQxDAIAxDULQzlkIJahWSJNgsPoBA",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByggIgyhceIJhsIIdgqDB3GkZB0GiFIWD4PAvgOBYrGwaRuGWIAyhQNYMHgMY3gweY7iKB42hoMQiisL4xjMPoBA",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHmDgzhIch4g4MofhwIojCCIQih4Pg8C+A4FiwbBpG4ZYoDGHYbDKDg5hIeI6iYMYbjcIo8iwL4yjSLBwGGFwgGSDhtDkIAxDYIAzC0M5WlYIpGkuFw+gEA",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_inactive")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NHmCoRCAdxpGQdBogoMQ4g0aBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIKE0MQ0CAORoDEIooC+Loai2LxojGMwxDmNxogyPI+GiQIwjIIhNDOR5JimS5NkKTxNkaOI6kqQQ+gEA",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopInactive,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAch5gkMoMHKCAihEIB3GkZB0GiCQxDiDBoGUaRnGgdIdh8IIPgoIg+DwL4DgWLRsGkbhlgcMYJguBwyhAMYMHmPAiDmP44kKLIujONYtHAYYbCAZIJG0MQ1CAMQ0C0MwgDOWJYkcL5MhsPoBA",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojgeJYLCIPg8C+A4FiwbBpG4ZQgHkMoKDmDR4jiCwxjsMYmiSQQijqLAvjKNA+gE",
        categories = "layout,design,development",
        tags = "drawer,menu bar,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[cfg(feature = "paperclip")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELg0DQIAxg0MA1C0OQuDEOQgheGRhDYIIfDAIIiDELQ4g4OYmigbInDUN4qi4QYQhCJIRhEOAgicOA0GyMIai0NxhDIIJDiKJAtDILg4DOSJKDOLIoioNA4CIPg8C+A4FD6AQ",
        categories = "text,design,files,mail",
        tags = "attachment,file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Paperclip,
    #[cfg(feature = "parentheses")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxHMLQ0C0M4RC0OQghKF4ZCIPg8C+A4Fh2IIGgiCgxDYIAzHMNIphgIA5hGL4xDmHIeiMPoBA",
        categories = "development,files,maths",
        tags = "code,token,parenthesis,parens,brackets,parameters,arguments,args,input,call,maths,formula,function,(,)",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Parentheses,
    #[cfg(feature = "parking_circle_off")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0NQgikMQ0CCLIZhuIYjiCIokiYIhNDEM4uDMYY7jsMIuCCQQwC0NhIDkdoPhoL4yGiNIjiWDBNDmLg3HYLQyC4Mw0jCTY1D6AQ",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    ParkingCircleOff,
    #[cfg(feature = "parking_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OQgDENxWDcaA0GEMwgjIMAgjUMY2CANhIDmGYbiGIw+gE",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ParkingCircle,
    #[cfg(feature = "parking_meter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAORhDMIIQDAIAxhQIA2CAMAiD4PAvgOBYch+BoIgoMQyhQMh2DOG4diKIYEiOCRNDGFQxDUaAyiyHowi+BYHjIMYMg4NwgkSE4VDALQxDMLoYDILgzEMNguDSFA0lQIA4hSDJakEaA4HMMZNC2V5OmMNwuDcY5QC0Lg4lSSguDWcJPiuHI7iCAowj+JYnkGKo6i6AQ",
        categories = "transportation,maps",
        tags = "driving,car park,pay,sidewalk,pavement",
        contributors = "danielbayley"
    ))]
    ParkingMeter,
    #[cfg(feature = "parking_square_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NgggwNhBDIIITDAIIWDEIA1g8aAxDQYYThWFwghmIR2h6IIUiOGAtC4NQ5iQLg0DEIg+DwL4DgWNo5gaCIKDMIA4C4NxWDEOYpiKFoWiGHQwC4M41jePI7gSPYJG2IYUksMJRjiVZUgWB4JE0MZAmUYZAkCGIrC0NhIDkdgyl2U4ClWYoKjAMQ3HYLQyk+c5fgEA",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquareOff,
    #[cfg(feature = "parking_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDB4gmCwgHcaRkHQaIUhYIByhgIgyCIPg8C+A4FikcBhh8IBkgkTQ5CAMQ3FYNxoDQYQzCCPwwCCQgxkMIA2EgOYoiqL4fD6AQ",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquare,
    #[cfg(feature = "party_popper")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4OAgDEMQuDMIAyhQMhsDEMAuDcLQzhsOQiD4PAvgOBYiiWBoIgoNAgDMaAuDAMYhiOKIngSKYJE0MoVDiL4xjOJI3jaBYHjkMQ1hSPoyiKQYmgKN5FgqO4UDCSpAjWT5EiobZTDILQyC4Mg0hsNRhmAOYUC6aAwCCbAwC2EQ5DaLQuDEMh2DAY51g0NgtC4NQ3g8Lg2DOcAuDSSIRoQaJ+DMOBjn4OJzm+iqDoYN5zhENA0EwMYshmV5ClmOAilyFQxoWDZehIM6QnyjQ0oYOJgl6cYOhGEJ4q6EIbn4N6nmGtaHhOEY7EgMQ3qGToolEbYQhQIKsqqegzmOkp+sCs6Gs+tp4EMOZ/hWY5omiDA1hWaA2mEMxWsmTJYsyKhNtyrZxsSaoTmAOITmOyLQDWkYevuHYwhzBQtwCwg5oW9pfg2hb9hyXpIw/AZ0DAN8FCCSAyFqyhoD6AQ",
        categories = "emoji",
        tags = "emoji,congratulations,celebration,party,tada,🎉,🎊,excitement,exciting,excites,confetti",
        contributors = "karsa-mistmere"
    ))]
    PartyPopper,
    #[cfg(feature = "pause_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bGwaRuggeAxhOFQgHkMoMDmEB4iyDYpHmJ4NDUIobC+IokiGI4ljUMQ0hCNITjeKoxi4IIwhOQo5jsZQ+gEA",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis"
    ))]
    PauseCircle,
    #[cfg(feature = "pause_octagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDUVg5CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIZiWB4JE0NwuDeHggDIaA4C4NQ3DITAyDIIIsi4NB2jONY3DENguDIOA2jAMhIjyHo4g6RJGDYVpMDQTJUjAeoThWGg+gEA",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "mittalyashu"
    ))]
    PauseOctagon,
    #[cfg(feature = "pause")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CINgiCAeYJDSDBoGUaRnGgdIJDGCwgHcaRkHQaIPCIPg8C+A4FiOJoGg4IoQhuHYfiGB4Yi2EoUhaGILiOJYEHQPoBA",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDEMYOGMeYLDQIg+DwL4DgWB4bh6BoIhCEg4hWFwiieCYLg2G4dgSI4hjGB4PikMQ2g6CoMhWEYMDCGociKIA8HAYR0GgIBkgsTQ5CAMQwGENQglMMAglYMZUlQdgzC4NRhl2U5hleZAxC0NguDgNJPC4MA0DUUZoDUMpPDcLg0DgIA0ncNpPmiahBmOY5WliVJek8MBakEL5GkgPoB",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggGgZRpGcaB0g0MgwhEeYZhEch4h4IIhCINQiD4PAvgOBYoHAYYLCAZINE0MQ1CCDxoC4MAxieKYuguLYvGiMYzDkIA2GgNo9C+PxokGMIyCITZGDEMJJkuTQ+gE",
        categories = "devices,gaming",
        tags = "computer,chassis",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PcCase,
    #[cfg(feature = "pen_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaA5CIPg8C+A4FhSF4GgiCgxDYLg1CAM4gGEMgugyDong0MAgiwMYiiITA3CAMQ5GwLQ0jSNI4FqE4VhoPoBA",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenLine,
    #[cfg(feature = "pen_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA0EgNBhDIIITDAIIWDALYTDIdgxhGG4XiGFobGiHoShSIohDKGh2C0NwiD4PAvgOBYxjSBoIgoMQ4C4NYUj2EguDGG5ChWIYNDMIAzEyQwgDENRsC0NJOk6UhajCMo3D6AQ",
        categories = "text",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenSquare,
    #[cfg(feature = "pen_tool")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDkIA3C0NwgDOFYThELQzhoegiD4PAvgOBYfiKBoIgoMQ4g6GwxC4NYTi4TINDIbAzi6Dg0jEMYWikbIvDWHYfiGBBoiSRIHgkbYzhGLg4DaTA1k6HogiWHxjGkchjGwZQgGMeIJDEMQil0eZgmIIBygkMpTC+V5ZlsPoBA",
        categories = "text,design,cursors",
        tags = "vector,drawing,path",
        contributors = "ashygee,mittalyashu,ericfennis"
    ))]
    PenTool,
    #[cfg(feature = "pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGEMguDgNQghEOAzCAMAgDGGwgDSHhMDcLoUDIMIjhWFQyGwMYjC0NYjFoIg+DwL4DgUPoBA",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Pen,
    #[cfg(feature = "pencil_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDAaA5CIPg8C+A4FhSF4GgiCgxDYLg1CAM4gGEMgugyDong0MAgiwMYiiITA3CAMQ5GwLQ0jSNI4FqE4VhqGYEhuCRtDGIYhDOIo+haQg+gE",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    PencilLine,
    #[cfg(feature = "pencil_ruler")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIINDQIA0CIPg8C+A4FhSF4GggIhNDEMwgDcIA4C6IgyiUYYnDQMQgiqLAwCCMAwC0MwuhAMBMieIg1C4M4pjaLIujGQ4yCCNQ0EyIofhOFYahmBIbgkbQ4CANotC0MpMhaUJPgWB5SDKLZhjwNQtDELg1jmZwxiINguDiPonDiYZymGRYzDQLZIjWDQxm4NRalqToClCX4KDGVJ+leWYUluGKEl6HILkqLA0j2EY9GMLg5panKbhCKpWjCRxslgLpWicNhjC2n6fqYNJupyNI2jETAxiybKClyAQ",
        categories = "tools,design,layout,text",
        tags = "edit,create,draw,sketch,draft,writer,writing,stationery,artist,measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent",
        contributors = "danielbayley"
    ))]
    PencilRuler,
    #[cfg(feature = "pencil")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAzGEMguDgNQghEOAzCAMAgDGGwgDSHhMDcLoUDIMIjhWFQyGwMYjC0NYjFoIg+DwL4DgWM42gaCAiG0MYUhSHw0jKNI5D6AQ",
        categories = "design,cursors,tools,text",
        tags = "rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley"
    ))]
    Pencil,
    #[cfg(feature = "pentagon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NQgDgLg3GMLYRg0LQxCAMQuDSFA3CAMguDIbIgDiD4RGMLoMiWGIaDSGQuDmLw0GgOQuDGKIxDCLw2hQNovDmF4biMLg4C2EISimFIlDCQQ3h0LYgiILQ3hsLQ1ikYYghiWggjqXpQhuXRaCIPg8C+A4FD6AQ",
        categories = "shapes",
        tags = "shape",
        contributors = ""
    ))]
    Pentagon,
    #[cfg(feature = "percent_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ1CAOQtDYIA2hmG4hiOIIiiSJgiE0OYrGgLgwDGMQvjMaI1iOJYME2KQgimPY/kGQw+gEA",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentCircle,
    #[cfg(feature = "percent_diamond")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NwgDEMAuDMYYMDQMQghWFwwCCG4dCAMwuhYbA3C4NQ5CCJImhSIYXhmHIvhuIIWhyI4lDkLYpDmK4zi6HodC2MgxEwMYgg6DA3DGO4tiyMIckCTAwFoIg+DwL4DgWVJXgaCIKDkLgyCCXgyGgLgwDGU5VlqWYEluCRtDENIlmGJQtDUIA1miVpsmuBYHgkTZwg2D5xDiZJmnmaoBA",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentDiamond,
    #[cfg(feature = "percent_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAeIJgsIByiAIgyCIPg8C+A4FikcBhhEIBkgkbQxDUIA5C0NggDaKIqi+EYujAaIygkTQ5jgaAuDAMY+C+QBokKMYzCITY2CCNpKkyTpQD6AQ",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDkIggHmCQiDWDh5DKCoMg4eIWhIIg+DwL4DgWHhjGkchjGyBhjHiCg2C6EwgHKCgyi6DhjHmLI0h4L4kiaKIjiWJ4GjEIozi+KoXDeNAgjaSI5h+PJBD6AQ",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINQiCAcoMDGDxjHiEgyCIPg8C+A4FgeGhwGEdBoCAZIMG0OQgDIMAgDMLQ2i0IA2hmG4hiOIIiiSJgiG2MA4jKKoyC2GIaC+NhojiI4lgwTQxDIIAxDAdg0jSRo5D6AQA",
        categories = "accessibility,people",
        tags = "people,human,accessibility,stick figure",
        contributors = "mittalyashu,ericfennis"
    ))]
    PersonStanding,
    #[cfg(feature = "phone_call")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDYLg5DIdgzGGDYNDAIIZDELQyC4MQ4CCDQxDkLg3DmDolieGoshwOAuDYMwtDMLgwDeKQuDWOI6hmGwtDaP44iuJImiiPYOjONQ3C2Lw2DcQYXiyGwgDSHwxiIaIVlGR5XiOJojh4OA0g6YZjkeGYmiILg4l2IpSkgLg0jqHgxDETIvDCKIlDkMRhg+Dg2m+GaBDYbAxC4MpLoeiYWm6XJqnULZxjoMZlmSa5mm+YaHk+W4tiKI4QhIegiD4PAvgOBamqmBoIgoMZVDCcxhnqb5XiEN4RDSpanqyq4Eq2CRNrCNY6DYQY6jyn4gg4MK8qiwA+gE",
        categories = "connectivity,devices,communication",
        tags = "ring",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere"
    ))]
    PhoneCall,
    #[cfg(feature = "phone_forwarded")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDgIAyhSFQ2CCEoZDAIg+DwL4DgWB4Jh6I4KHgMYQDENAiCAeYpCINotHgMoQDIMotHmNYxh2H4mh4cBhHQaAgGSEBNjeGQ2C4OQyHYMxhhWFQwCCVAxC0MguhqFQxDkLg3DmGZemCVZllcOAuDYMwtDMLgwDeYguDWcZzlSVgtDaeJxmSXZfmGdoZmybg3C2aA2DcQZSmWVggDSWgxhQaJQoqgKQlyX5clkOA0hmmqcoCVJfhQLg4paFKLoELg0nOWQxDETJoDCYZeDkMRhDGGK4qiVIYDYbAxC4MqEsCwpRqelajq4LaqnMMaep2pKfqimrAoilJmhaSpMDIeo9iCQhoD6AQ",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneForwarded,
    #[cfg(feature = "phone_incoming")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDYIAyCCEggDiFIVDgIg+DwL4DgWB4Jh6I4KHgMYQDIMgiCAeYpCKLIuDKEIcCAeI0hENodh+JoeHAYR0GgIBkhATYrhYNguDkMh2DMYYVhUMAglMMQtDILgxhmFQxDkLg3DmFpemCVJllYOAuDYMwtDMLgwDeYguDWcZzlOVQtDaeJxmSXZfmGdoWmybg3C2aA2DcQZRmWVQgDSWQxhQaJPoqgKQlyX5clgOA0hamqcoCU5fhQLg4paFKLoELg0nOWAxDETJoDCYZeDkMRhheF6gCCEw2GwMQuDKhK/sGUKnpWo6uC2qpzDGnqdqSn6opqv6IpSZoakmSwyHqPIgkEaA+gEA",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDYIoHDGCgyDKDh5hAIoTCAeYJCIOAiD4PAvgOBYfiKBoIhGGB4haDIUhaGIagqHYfiGBBlh8cBhHQaAgGSChNhIIIMC4OQyHYMxhDIIJJDAIJMDELQyC4MQ4kqQQ5C4Nw5laWJakyTgtDgLg2DMLQzC4MA3lsNZqk2bZPDYLQ2luWZzl2bplmcN5gmINxBkmS5uCANJSDGShokef5tk6VQxliSQxlEOA0kGkaTl6bZYkoLg4oWiaXk8Lg0muUQxDETJhDCWpXDkMRhgyQZypeTJyDYbKNDKeq3DeSJVp+mqlC2oZrpCm6TsSkqKm2kaNn2nqBkCQpEHqHogjiOg+gEA",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DgIAxDMLgzDEYQxDaEYZDAIIchyEw0DEIAyg4bAxC4Mg3C2J4pGEMojh2MYiiQMQxC0Lg0DWEYkDgNI7C6PYxh6I5AicN4wi+Q4ikaLwyHYM4ujCSgtjSEIvDEOQuDcOYRlmW5ChELQ4g4MwthMMJHliOJXlkNJJjKZoUmWJA2DcbZUg6KoTDMNIXl6XJql+U5niqYw2DMQZNmCIg0C6NYjGiUKKkqMJMj+QQxjyPpDhyWpEDiM5SnCOI6jQMRMmMMJclkOQxCIPg8C+A4FrAbBpG4ZQgHgMoJDIIggHkMa9r8eLCCIMq+sCvLHr6sAvrauA+gE",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgyDIIA4CCEoWCAMQ2hYIg+DwL4DgWB4Jh6I4KHgMYQhoIggHgMoQhKLB5ikIg4jKL4Rh2H4mh4cBhHQaAgGSEBNheGguDkMh2DMYYThMMAglAMQtDILgxhWEwxDkLg3DmGZbl2UZilMOAuDYMwtDMLgwDeXwuDWbpwlCUgtDadZumGWpcl6c4Zmmaw3C2ZQ2DcQZOmKUggDSVgxhYaJMoefaNlmXJZlUOA0hml6Zn2UJchYLg4pOGKSC0Lg0nCVQxDETJlDCXpbDkMRhhqGYbp0IIbDYbAxC4MqBr2v5NqSY6gqupqopqoaZDGm6ImKl69oWkbFkYNpIDIeo6iCPxoD6AQ",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOutgoing,
    #[cfg(feature = "phone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDYLg5DIdgzGGDYNDAIIZDELQyC4MQ4CCDQxDkLg3DmDolieGoshwOAuDYMwtDMLgwDeKQuDWOI6hmGwtDaP44iuJImiiPYOjONQ3C2Lw2DcQYXiyGwgDSHwxiIaIVlGR5XiOJojh4OA0g6YZjkeGYmiILg4l2IpSkgLg0jqHgxDETIvDCKIlDkMRhg+Dg2m+GaBDYbAxC4MpLoeiYWm6XJqnULZxjoMZlmSa5mm+YaHk+W4tiKI4QhIegiD4PAvgOBQ+gE",
        categories = "text,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Phone,
    #[cfg(feature = "pi_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDYJCIM4NHmCoaCAaBlGkZxoHSFIWD4PAvgOBYoHAYYSCAZIKE0NwgDcaAxDAIooC+LoSi2LxojGM45jYdo5juKY+GiQIwjIIhNDENggDENxhDIIJYDAIJbDELQyl8Vg3kmPZBD6AQ",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0Ig5CIIB5DGCg0g0eYJCIMgwg0eIQgsIg+DwL4DgWHhwGEdBoCAZIKE0NAgDcYwwC0MQuDcIIyDMLQzCCNwzGgMQzh2H4kiaI4lieKQiE0MQ4CCFxjjGMwgjCN42jiOBWhKHgvkIaA+gEA",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdgxDAY4WDELorDmIIkGgNAiD4PAvgOBY0HIZRjHQIByHiCQyCIIBoGUaRnGgdIJDeQ5ACIMZCCAeYJDEM5DHcaRkgWVAwjONY6jwPoBA",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANAuDUdg1EgMxtC0MQtDYIIZDYbQxDMIAwHYLQzGMMIWC4MQ2C0Lg4DQLQyi+LxoC0N4VDkIA5HYMolCAMQuDANQuDkNQgDKRZFGgMwiD4PAvgOBZMHIZRjHQIB5gmHoPCIIBylcIgylseJYmAIB3GkZIFlgMJbGgZRpGcaB0gkN5Lk2UpUD6AQ",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture,
    #[cfg(feature = "pie_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELoMCAMQ1C4OA5EEMQwhCGIYDGEAgDgIAyhMMwiD4PAvgOBYliiBoIgoMgyhAMoWhuGggjUMYwDIdoXHqJImisPoBA",
        categories = "charts,files",
        tags = "statistics,diagram,presentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    PieChart,
    #[cfg(feature = "piggy_bank")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIA1GMLQxC4NQgDALQyC4OAghINAtDMIAyh6E4RiMMQxC0LgzhEMYOhWG4Zi6HwyCANITFYMgwGgNB2hcaAzHYMo6jwNBjieE4vDeEYgj0MpDGiFxjhaRg1iSVInkEMBWDUegiD4PAvgOBZemGBoIgqMw5HYMZRi+EoNjOcBoDGXZfmSY4EmWCYLDaGwxGgMJ0mCeA+gEA",
        categories = "money",
        tags = "money,savings",
        contributors = "ericfennis"
    ))]
    PiggyBank,
    #[cfg(feature = "pilcrow_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoHAYYLCAZINE0MQyCCNRIDkLg1GEMo7CCPg1CAMJDjeQwtDUSAxDeJ4pi6C4ti8aIxjONQgDcdgxDCTQvk8aJRjCMgijQNpXlmW4ol2Ug+gEA",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "danielbayley"
    ))]
    PilcrowSquare,
    #[cfg(feature = "pilcrow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIA0HYMQ2CIPg8C+A4FhSF4GgiCgxDeDoQhKFIWgQaIZiWB4JgsOYOEgOQuDUYQ0jCDo0DAII3jkIA5EiDIThWGg+gE",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Pilcrow,
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
        svg = "gAPBsGkbhlCAeAyD0IgyDIIoHDGCoNCAeYQguDh5gmC4ND4PAvgOBYch+Bh4hUMYShSCgxDeF4ZgyDoIimG4diKHBwGEdBoCAZIKE0OQgDkdgxC4Nw2GEMggkcMAgkoMQtkIMQxCCQg3DkbJOkMOAuDkQZHkmS5fDWUg1C4Mg0FaKhoiYIocC+No4jWN45jsIhNDGYQ5C4M5mDaaZGkiX5KoELQ0EgNwuDgOZrh2bhoD6AQ",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIggHkMoKDKDYPgmCw3g4eIRguDQ+DwL4DgWHhwGEdBoCAZIKE0NQgDENxoDENB2C0MQuDcNhhDIII6DAII9DCNAuDEMZBDcORskUOAtC4ORBjqPI+i2LYsDEMI2DYVg2jCOY7lGP4+C0NBIDiXJQl+PQ0jAdg0leZZei2QZDi2NpHkmTJOl2ZwglQNQuDINBaCKHgviSJg+gE",
        categories = "maps,account",
        tags = "save,map,lock,fix",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pin,
    #[cfg(feature = "pipette")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMoMDELQxGgMxsDkLQ5CIPg8C+A4FhqHYGggIhNDODQxHYLYUhaGIahyBBoh+L4HgkbQxDUIA2CAMwuDSKY8GEMguDGDZCCAMAgkOQ4lDMTAxDgIA5GyPI/kGQ5VkaSJIimOhsj4OJekCRZXkeSY6imUg0jwWoZhuIA+gE",
        categories = "text,design,science",
        tags = "eye dropper,color picker,lab,chemistry",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Pipette,
    #[cfg(feature = "pizza")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAxDEaAuDAMQiD4PAvgOBYWhmBoIgqD4ODWEYThWF4chuBIdgmCw2g4NojhSFoYimKIFgeCRtDKLggDIMAgDYLZAj0QY9jyPpHCCPo6DENoljOGoCimN4KDULg3DGDg3C6DxhDGWgwDSWYSmGSJYg8Lg0C2Zw0k6J4BA",
        categories = "food-beverage",
        tags = "pie,quiche,food",
        contributors = "karsa-mistmere,ericfennis,jguddas"
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
        categories = "transportation,travel",
        tags = "plane,trip,airplane",
        contributors = "ahtohbi4,csandman,ericfennis"
    ))]
    Plane,
    #[cfg(feature = "play_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAbxsHkZxvG4IIiGkbh0HOEwwCAOAgDENozDKM4wjSOIxCKGwviKJImG4PoBA",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[cfg(feature = "play_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJG0OQgDgIA2CANAtjkNBaCKKAvi6Gg+gEA",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlaySquare,
    #[cfg(feature = "play")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig1CAMwgDEOYVDIIISDIMYahMIg+DwL4DgWBxuD6AQA",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "colebemis"
    ))]
    Play,
    #[cfg(feature = "plug_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMh2DYIg+DwL4DgWEoVgaCIKDENYNg+EYThiF4EhmCRNDEMggDENx2DWIIUiSI4FgeJodDgaAxDSL4igKJI0goNoqDEVo3igdgzGGQZBDCKoqC2KAgDAdgwFqO4xgE",
        categories = "devices,development",
        tags = "electricity,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[cfg(feature = "plug_zap_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMIAyC0MoOC4NRoDMTAxhENwiD4PAvgOBYch+BoICITQxDAIAxDQdgtDOG4diKIYEiOCYmDSKYri2L4ejOMoFgeNQxDGKQ5GMLQxC4NwgDCLZHC4M5NDOLAyGgOB2DIY4okgN5Og2UJeFqO4xgKM5AiWGIODKLIuhyPIggE",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlugZap2,
    #[cfg(feature = "plug_zap")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4MwgDIMINGEMguDSD4VCAMIZhsM4YDATAxDIIAxDgbAtDaJwthSDorhOGIUhaGoyhyFRaCIPg8C+A4FjiO4GggIhtiIMoiDMLQzjeOY+j2BI/gkTQ3C4NYjh2UwxhoMQxkmOpNkyBYHk+V5SiODJWg4MQ0luS4Ck2YJBiQIJGhYNBoDaJpzmqXYBA",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "mittalyashu,ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    PlugZap,
    #[cfg(feature = "plug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDUIg+DwL4DgWFYYgaCIKDkIA4FYMoUhaG4agSHIJgsNYgiKJIXiiJ4FgeKgxDiIB2DUYQ0CCPAwCCPwxC2PA0GiQ47j2QJKkINJDFYOBai+JoB",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Plug,
    #[cfg(feature = "plus_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgg4aA4hmG4hiOIIiiSJgiE2DggDgdothoL4wGgPoBA",
        categories = "maths,development,shapes,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusCircle,
    #[cfg(feature = "plus_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEaBlGkZxoHSDYPhEeYUhEcoTCIMgiD4PAvgOBYoHAYYLCAZINE0OAgDEMhohCKAvi6C4ti8aIxjONwgDgdo6imPRoD6AQA",
        categories = "maths,tools,development,text,shapes",
        tags = "add,new,increase,increment,positive,calculate,calculator,button,keyboard,toolbar,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusSquare,
    #[cfg(feature = "plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyGgMQ0CIPg8C+A4FhSF4GgiCoOCANR2hGE4VhoPoBA",
        categories = "maths,tools,development,text,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis"
    ))]
    Plus,
    #[cfg(feature = "pocket_knife")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DEYwwCAMYNhOFQyFODA2CCDA3HMMoWiAMgtiAMYkheIYNCIPg8C+A4FiyL4GgiCgxDgIA2GgLgwDGK4tjKMYEjOCRNhuNo6jyPoukKQYFgeRAyDALg4gwOJTDMYQ0CCWoSl0LQ1C4Ng2l+YQ2GwLQxiUMpZlsIIShSEpgmIIJyDYWpKkCApCk+NY3DEMZlFYMprlqXJuocNAtDQVg2niTIB",
        categories = "tools",
        tags = "swiss army knife,penknife,multi-tool,multitask,blade,cutter,gadget,corkscrew",
        contributors = ""
    ))]
    PocketKnife,
    #[cfg(feature = "pocket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMxoDENhhDIIITDAIIWDGFIUHaEQxhiFogCAMQth6IgwEGJYliGGYTDEMRWDWEoaiuFAtDIegiD4PAvgOBY6HAbxsHkbBpG4ZQgkCRR0HOCQ4iaIotgyEImjmO5AkKRJGD6AQ",
        categories = "brands,development",
        tags = "logo,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Pocket,
    #[cfg(feature = "podcast")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYSg0Pg8C+A4FgeGhwGEdBoCAZILE0MQxCAMQ3GGKoqDAIIxioMoyGOMQuDULQuDMNAgDOOw1CANI5GGOY5jKSQxC2MBjjsMQ2kyOZBC0NJUkQNRaCKGgviGI4giKJImCITQ4isNBhkKQozisIJmDCW4bl4aJgiOJYniyKw4kUOQgn2bAwkyMZwlycw+gE",
        categories = "multimedia,social",
        tags = "mic,music",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere"
    ))]
    Podcast,
    #[cfg(feature = "pointer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDQYQ4CCEgwCCFQxC2Eg4CIPg8C+A4Fh2IIGgiCgxhIMQxHYLQxGGDYNhWMQtDKMx2DCLggjCFoWjOOY2hyHojiKBIkgkTYPg4MBWDmOI6jKNAyjaTY7k+PgxkCH5EkOBYHkYMYVDkLg1FaEIvlSPJQlKZoxmiVgwliQoCkSXYmiiLZrg6eQ0hYdgzhGE5nhiGhojMY4zC6FAtDSYgtogNgtDULg5DmhwzDQbAtDMLqPpoNpTmwMY5ogM6HDgMhMDeDg1nCWoBA",
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
        contributors = ""
    ))]
    Popcorn,
    #[cfg(feature = "popsicle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg2CAMQ0C4NBjC4OAthaFgtDIIAwhuFhsC2DQxiILgxGGEg5DUIIpisMIQh0LQ3iYIIzDEbIjCCI4VDkLg3CAMomj6QAuDmGAxFoIg+DwL4DgWS5OgaCAiG0MoclYLQ1C4NZZluSpMlEPoBA",
        categories = "food-beverage",
        tags = "ice lolly,ice cream,sweet,food",
        contributors = ""
    ))]
    Popsicle,
    #[cfg(feature = "pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA3GMMAtDULgzhULQ4hKFIWg0MAiD4PAvgOBYfiKBoIgoMQwg4dgxDSHogiWJIEiaCRNDYIAyDEaAxDKL4hjOMoFgeNY3DEM47h2H4/iOAQA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[cfg(feature = "power_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2Dggg4Vg2hmG4hiOIIiiSJgiE0OAgDcLg1EENguDEII+kAMAgkSRIpDEOBhDaQZFk6RA0C0MQwjuLgvjAaA+gE",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley"
    ))]
    PowerCircle,
    #[cfg(feature = "power_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLgzDYIA2C4Ng0EEOQghcMAghoMQgDIMAuDcNwgDENQiD4PAvgOBYoiuBoIgqEgxhCMg2GGF4ZiSG4kDKEw4jyPonimLotgSL4JgsMoeHYNJCiqRpFgWB4JG2SpWhqH5OkSAQ",
        categories = "connectivity",
        tags = "on,off,device,switch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PowerOff,
    #[cfg(feature = "power_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOE0MQyCANx2DWJ4pi6F4ti8aIxjMOAgDkYQ1C4MQ0CCSJKCAMJPlGS5EDQLg5DUIJVleUZQlANAthGKAvj0aA+gE",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley"
    ))]
    PowerSquare,
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
        contributors = "danielbayley"
    ))]
    Presentation,
    #[cfg(feature = "printer")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig2CAOQghIMggDEOAgheGYTCIPg8C+A4FgeCYgHAYR0GgIBkhATYShkSA0GGF4XDAII3DELQyjsdgtDWNIbjiQwxhuOxoDENpBjaRJCDIdpAjWQ45juGxojuH4hiiKogHIZRjHQIB3GkZIqhAMQyCIIBoGUaRnGgdIQDiah5mcNJqHiEA2lkL5emAPoBA",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANwgDMIA1CIPg8C+A4FhSF4GgiCg5CANhWDOE4VhqGYEhuCRtDGEIODILQyiOFonhQYxpHIYxsGUIBjHiCQ5CKOx5gmK5AHKCYihQL41jeOYmgWB4JE0MQxC4OIQDEMhIDIMBhDIIJeDAIJhDGX5fHYNJdmWYZji+XxImiXpgmIIAxi+Lx2C2cJqnOZIuDIaAyC4MQ3jGJYCieUIKDENp0DagKFjOAQ",
        categories = "multimedia,photography,devices,communication",
        tags = "cinema,film,movie,home video,presentation,slideshow,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Projector,
    #[cfg(feature = "puzzle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg0DMOQgDcLg4DUYwtC4MA0g0MwyDKGQ1g0Ng0DgLgyDiDQ4DcOBsDELg1DYOAgi6MA4GODoTDSEw3DANozhmKgujyPoujwNBzhgMgzDWP5KDOGJDj+RhsC2Lg2DEMY/lcMRhC4OYll8IAwmKM4YDgM4TDINw2heOIYDAN5mDAMoYiSGA5jGdwyDUYYfDUMJZn6gJklkMAtDOJgxDQIKIDKio3DQNA2C4MQ2pOFQ1g4OYTDme5ejGXabg2opkmMMZJmuFJolSVpZq2fYOhoIIfDSspjqaVZCDCmY8Des6xDKv61sGt5lkWGpQj2rIvnmNKgi6c5EhkMo+sWhoUDecYmDmbYboicA0mYNIvsi0IfngOKwkyfqEmWjZooeJpopCI4YDGJYouGL5qp+cbmGG5rStGpZikmKJmtmy41rmNRBrSc7CxCxaul6YQxDIY7XleaQzDauZKuOQ65kYTLjkoIIltmNwyvrLIvDipwukuiA5DG2gzDCiA1DGmZwhOKoTDWJ4/nAM9Ely7LsqaZKNiG8Z7tydQ4nSlJ4ncM5Pi8NQ4rmgNdDfWa7hjWKTj3VA2qnONZzUN4tvsNcMnvDrAxGxLtxePw5l+N8boS8smx2UoaroNtujWP41yu79apPWL4tm26a2EN7jDi+p/ou5p3qDSYv3ejLyr67w3t2I6U1yFA5uPQranivrmFoIg+DwL4DgUPoBA",
        categories = "development,gaming",
        tags = "component,module,part,piece",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Puzzle,
    #[cfg(feature = "pyramid")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQgDENguDgOBhDGDwgDCGIPC0LgzDILQxC4NAzGwOYgDMLgwDKFYXhmGYWiENg0hiJYPigMAxiyFouhuHQyg+Ig0GwLQ4g2Fg0hINhhj+P48DGIAuDkOIYFoIg+DwL4DgWV5agaCIKDGTB2DIMJWliXQ+gEA",
        categories = "shapes,maths,travel",
        tags = "prism,triangle,triangular,hierarchy,structure,geometry,ancient,egyptian,landmark,tourism",
        contributors = ""
    ))]
    Pyramid,
    #[cfg(feature = "qr_code")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIB3GkZB0GiCQ1gwcoOCIMYMGgZRpGcaB0hcIg+DwL4DgWJ4qgaCIKh6IIiiQIoYCCGoJh2DY5DaDIShSFo1iaKItiyBIGhuEIvDGPQgh+IYjiWEYThWUo4hyQ4pkeJxwGGFQgGSCRNDIMQgkwaAtDMYQyCCbAwCCbwwC2bAyHaC4nC+XYVlyXhomCYpkm0MR2C4MIdniehonyX5hCITQxmwN52mubZwpYMZzm0SA3lmiaLn6jRNDOZgyGihaHiing8omf6OpAIAzqahqdn2n6to+bJMoSs6IrWq59reTKkGiqJ5r6rKhoGkK7sWqrImKr5kHYLbNrWAQ",
        categories = "development,social",
        tags = "barcode,scan,link,url,information,digital",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    QrCode,
    #[cfg(feature = "quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMgxGODAwCANwtDGEwtDgVg1GMMIVC4Mg1C0Lg3DUNgtDILgwDGFAyicSA0GOHogCCHQyCCI4hjYMQuDkNwyFYMYPhKO4gjiDZHjqNAghaQ5LHaQpLhWDYng0c5SikMA4lKOwwDMMRWDIMIckuSoWmYegiD4PAvgOBZqm2BoIgoMQ1g2D4RheFoUhmG4dkSIY4iyKYrieJxoC2MIynWNY3iSVI7j2P5BmOf5GjalxojiY4okWMw0ieRg0HYM6UmWZAxmiapsgQaA+gE",
        categories = "text",
        tags = "quotation",
        contributors = "Billiam"
    ))]
    Quote,
    #[cfg(feature = "rabbit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA4C4NQ0FYNBhDIIIWg0MQgDALQ0hsdgzCIPg8C+A4FiOJoGgiCgxDiFwxGgLQ4GGHoeg2GYdh0IA3juG4+hoNwtDcaAuDITA5C4NggDYLoUDELg5DMIJPlGP5ThcLg4C0MpZEwMQ1lmO5EDIYwzC6UoNkqXI8koNh2DGFYXj6OIWDKMQxGMLZPjyHJSk8MwtlKIYjiWBBoiihoHgkTQ3kmGgxlwNQzGGgpWhyT5KDSZ4iiSKaIgWiosn4NqUCCaJWlwMoeDWnKFieAqJisTYtlOdguDAMatp6AQ",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = ""
    ))]
    Rabbit,
    #[cfg(feature = "radar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLgwDcIA0C4OQzEEMQwCCFwghiHAgDaEw5CAMwuDMNAiD4PAvgOBYoiuBoIgoNIeGiDgxieKYui2BIvgkTQyC4Mohg0NgyhaGIakeGwgDIMYkDEIA4iQNY3iqO46gWB49DGHwyjINwuDcNhBDaHpKk+GJRDIM4Zh8Ng3lSOYCjuWYKDEMoZDiNAwjaKJViycpYjCC5fDmIQxk0NpimSZIdk8MQ1mCEJbC6bpwlYPBjGkchjGwZQgHKCQyCIIBjHiCZ2qMYx5qeop9pmm6dlePAiG0MYjDSjgwC4NYhpCiQtr4NqWiyAQA",
        categories = "navigation,maps,security,communication",
        tags = "scan,sonar,detect,find,locate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radar,
    #[cfg(feature = "radiation")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMGgMAuDAMQiD4PAvgOBYWhmBoIgoNwuDUIA0C4MhjC0LgzigNYoDmKA3C0MYpigNBDDMLg5CANYhCAMopCAOAuDGPYODGJ5CiELg0g6TAxGgNRjDCMZJDgLY+DiPQtjcNJTDGU4uDKWohlaY4kDQeoVheHIbgSHYJE0MpDgwYwuDYIAwg6NJ5l6M4+i6MpVjuLIkl6IJ8lyM4ynyYYyjeKp1jiU4ij6LI+naJAzg6JQuDeRINg2Nw1k+aIWhibZsgWB5viCIgxDkLg4keoYonKSJLo2PZ1pqDY7naMoNkGDZ4mOm4wrCSpTo+LKRjKXKUmSg6JsWZJBjCS5SoqmpVqCyKUiOKakmqp4B",
        categories = "science",
        tags = "radioactive,nuclear,fallout,waste,atomic,physics,particle,element,molecule",
        contributors = "danielbayley,ericfennis"
    ))]
    Radiation,
    #[cfg(feature = "radio_receiver")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ2HYMgiD4PAvgOBYThaBoIgoMQ5g2D4RhOFYEGiExyGUYx0CAaBlGkZxoHSCQ4CIIB3GkZIFgkMgwjQeYyjQeI6jQcpBCKIYUieKYYiSB4JE0MQ4g0MhojyIoZD6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0C4OQgDEOQuDEQwxg8NQuDKDwgDgLg4CCDIOiAIg+DwL4DgWJIngaCIKDeHYPDaGBjC0MguDONI2jiN4xhUMAthwNYjiWKokGMaRyGMbBlCAcoJDIIggGMeYJDGT5RHiVJPiQL5GkiSopgSK4JE0MYxhmLg4GONQzCCa5tjYII8CAMIbC6QZbkSAphgeY4QhKH4NEMMpshyHqDhaf5+hWEJCiaYQ+gEA",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Radio,
    #[cfg(feature = "radius")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgzDQIAxDcLg1DIYQxDCEYZhkMQgDALQyC4OAyCCIYjCIPg8C+A4FikYxpHIYxsGUIBjHmCQxDkIggHKCQyjsYx4jiOopC+L4xjOKYsgaCAiG0MQzC4NIdlCUodDQLgxDgIJYlqKIqkuLowjKNI9CKP41jcIgxmiQY4j+RZHmQPoBA",
        categories = "shapes,maths,design,tools",
        tags = "shape,circle,geometry,trigonometry,radii,calculate,measure,size",
        contributors = ""
    ))]
    Radius,
    #[cfg(feature = "rail_symbol")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ1GgMQ0CIPg8C+A4FhSF4GgiCoMDmEIShSFoEGiGYkgeCRthEIAyDALQ1i8IA2C2M4wDWE4VhoPoB",
        categories = "transportation,maps",
        tags = "railway,train,track,line",
        contributors = "danielbayley"
    ))]
    RailSymbol,
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
        svg = "gAPBwGEdBoCAZA9CITQxDcIA1GMMAtDELg3hILgzC2GIaHOGQghOGggDOEAgC4OIXh4Lg1iUIAyEgMQxGOGQuDkIIRg0MwuDELYNDcdgwiMMguDKKA4CANJGkYIg+DwL4DgWS5OgaCIKDENorjgORjhcLYXC4NpcDWHpcg2Ew1lwNohl0MZpjSE5WmiZZcmuOIYm6bBsnaQQ4lqQY4kEN5CmKQollybZChaZI5jWNYWl+W5/meLg3jGE4NhEM4oiAM5KkyUZQgSUoJgufoeDgYaYpgMKLhGQQyC0Nack2oKfgWB6iDGmAyi0NBhkOQ6qqqa4RDQaAxDKsaegKoK2lSaA5GgLgwDGyKzgEA",
        categories = "animals",
        tags = "mouse,mice,gerbil,rodent,pet,pest,plague,disease",
        contributors = "henri42,jguddas,karsa-mistmere,danielbayley"
    ))]
    Rat,
    #[cfg(feature = "ratio")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDIMINHcaRkHQaIKDGDAggkIg2g0eYVCIPg8C+A4FimLIGhmG4dguF4OhCEoUCKH4lgqJIHiKIJAiiKovD6AQ",
        categories = "connectivity,devices,design,photography",
        tags = "screens,rotate,rotation,aspect ratio,proportions,16:9,widescreen,4:3,responsive,mobile,desktop,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[cfg(feature = "receipt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tiuLoUFoIg+DwL4DgWNo5gaCIKDENggDgaAtDYYYhiEMIbkqSg0GgNJHhaSoVhWTRIDiNY3jyO4Ej2CRNDGJw3FYN5ZjiXQ+gEA",
        categories = "money,travel",
        tags = "bill,voucher,slip,check,counterfoil",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[cfg(feature = "rectangle_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINgiCAaBlGkZxoHSCQxDKDB3GkZB0GiCQyDCDB4h2DByiEIoWD4PAvgOBQ+gEA",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDIIggHmDYQCAch4hSEYYCINoRGgZRpGcaB0hQMAiD4PAvgOBQ+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDQIA1C2EoUCIPg8C+A4FheGoGggIhNDIMAgDkSA5C4NRBDWKIRiyI4vCCEIPigdgwiqLIrg2MIjieDYiEgMQzhaGIdD6AQ",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Redo2,
    #[cfg(feature = "redo_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYSDcIg+DwL4DgWB4bHAYR0GgIBkgsTQyDEIA3HYNhoC0NoahyIokiGI4licIhNDMIAxDcYQ5CCQgwCCRYrDkLZCkuRpNisNggDILgzGyPZShmGwvjUaA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DYaA2CIPg8C+A4FhSF4GgiCgyDEIAxDIQQ5CCJAwCCJ4nDYIA1C4MxMgwOIThWGoZgSG4JE2HoNg4LYQj6M4WjeNoFgeOYMiEYYkiaKJNDENQgDYLg3GwMwtDKU5BjUPBjGkchjGwZQgGMeYJiEIggHKZpoGMeJmDKQZdl+YQ+gEA",
        categories = "arrows,development,shapes",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = ""
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
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0GMMAtDELoTDkLQyCAMoXCIPg8C+A4Fh2IIGgiCgyDCGRjhMMQgiiGAuDmGYZhyHojiKBIkgkTQyhgOIQCCKwtjCGYXjOHYfjiN4FgeOgxDaQAwGOEoUi2F5ChaGgyjSSIhgKOJMCIbQzCANwgmOZwtDOW42l6S4lE2TwxDAVg1hCU5lhMM5pmaaRoDGa5JDwchlGMdAgHiCZaCAaBlGkZxoHSCQ4CIIB5gmDKUHcaRkgWkqUHKiAilqR6DoWSo5gqDJAg+K4tjKQ4YDIdoPiiQawkWo41oGI5gjutaslSLggrCGazj+toxlmRq6iGAQ",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0GMMAtDELoTDkLQyCAMoXCIPg8C+A4Fh2IIGgiCgyDCGRjhMMQgiiGAuDmGYZhyHojiKBIkgkTQyhgOIQCCKwtjCGYXjOHYfjiN4FgeOgxDaQAwGOEoUi2F5ChaGgyjSSIhgKOJMCIbQzCANwgmOZwtDOW42l6S4lE2TwxDAVg1hCU5lhMM5pmaaRoDGa5JDwchlGMdAgHcaRkgWCQ4CIIBoGUaRnGgdKMo4eIJloIB5gmDKOHKmAilqR6DoUPoB",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Replace,
    #[cfg(feature = "reply_all")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig3CAMYSDKE4WhINwiD4PAvgOBYHgmHIfgaCIKgwboOhAMYWhQIISiyF4vhuHYkiEZYjGEdBoCAZIQE0MotDgdgtDIYQ0CCSAwCCSwwC0NJPEiGoch6OhoD6AQA",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    ReplyAll,
    #[cfg(feature = "reply")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig5CAMQ3CANITDIIISDcIg+DwL4DgWB4Jh4cBhHQaAgGSEBNDIMITDgdgtDIYYXheL44C0NI6EgNIdh+JooD6AQ",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Reply,
    #[cfg(feature = "rewind")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIAxDkIAyhSGISCANYUhOFQiD4PAvgOBYHG6IolgaCIKG+DIOhAMoahYMQzhmF4Yh2MoUDmIYjiqJw+gE",
        categories = "arrows,multimedia",
        tags = "music",
        contributors = "colebemis"
    ))]
    Rewind,
    #[cfg(feature = "ribbon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg3DUIA5C4MAxGMLQuDUMggDKEw4C0MQuDgMwgDMLg2DQLYlDEOAgDULg0DkbAthwNokhiKIcDkNwgiiJQ1jON42DiPw0C6OoWiAOA2kANgxkCHolDcN4pC4MZLi4NA0CIPg8C+A4FlyX4GgiCoMg6EIShQQY7jsMAgm6bg2C6GoSDEQ5yDCNQ4hgII1g0OIan4YwwlSEIciILYQi4OYeDUQwxi4Mg0hsIIrjaEKWDegwgg4M4XDAOaVi+FwyoaEwxFqW5dmKYYEmOCRthIM6YkUNYjjSPQuDMMaql6rqtgWB6wDGII6pWewwpOOahpOPo4jeVA5p4Na9qyAqusKZaTDim5ODGG5AhSNgyDKj7fDGbrohGlZuDgYYahq6aVjyb7Vr+AQ",
        categories = "social,medical,emoji",
        tags = "awareness,strip,band,tape,strap,cordon",
        contributors = "karsa-mistmere"
    ))]
    Ribbon,
    #[cfg(feature = "rocket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4NQgDENoNGMLQxg2DwuDINgtDIIA1huHRzDMLg3DQLYWh4MhjiMMYmDiDA3hsLgxDOJgwDmMQ5DEYQyjIOAgjwMY+DAIJDDCOIsC6Nh6CIPg8C+A4Fk2UIGggIhtDGHAxh6NAzjuHAyhyRYPj8LYiDkNRBlgLg4j6apskScAxj+XxjkOPA3DKJg3j4N4NC0NoPjqYAuDODqDoWcJDiwNI/kuTZPgQaJSpGB4JE0OYPDISA0HODZbkkM5kDQY4VDaeYVDCPoOkOq5Mk6U6TgWlYKliDw1HYNYhqCJg1g6JYpqgOIUC6ppEC2q7Hq6kJRgEA",
        categories = "gaming,development",
        tags = "release,boost,launch,space,version",
        contributors = "ericfennis"
    ))]
    Rocket,
    #[cfg(feature = "rocking_chair")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgzC4NQgDIIA2hMIAxDKGQxDiGocDUIg+DwL4DgWB4JiSKYKHgMoQDWEwiCAeQxhCG4yCAeI2CIOY5HmLwiDIMIjiWLIrgiCpAhCQ4zjuN4ijqQYej+PI4iKJAvkeAhhHQaAgGSEBNhwN4Uh4YQxDOGpqDAIJtm2VIUkSWRwl0aA+gEA",
        categories = "furniture",
        tags = "chair,furniture,seat",
        contributors = "connium,ericfennis"
    ))]
    RockingChair,
    #[cfg(feature = "roller_coaster")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ5FYNQiD4PAvgOBYThaBoIgoMQwg2Dw2C4OIShSGYYgSGoJE0MQ0h8dgtDeIokhWKIngWB4qDEOAgDUdg0jOJoCiiOIcjuDovDaQI1kKN4bE0Mgyh8Vg5kqF5MimCpRg6Uxhi2LYeh4MQgDQLQ0GOUYei0MQuDMMwggwNBzl6b5jl2YwgmGeAtDMLYhDaEYTjSF4B",
        categories = "maps",
        tags = "attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    RollerCoaster,
    #[cfg(feature = "rotate_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYLg0DYNggDcLg1EMMQ1C4Ng0DMIA0C4MgzDcIAxDMLg5DUMggikMYpikOYfDMOYqhIIINDQN4iiKLBzDKMIyDEMAgDWIwwGMLgzDSKZBhmOAtC4MA2j+TgyCIPg8C+A4FlaWYGggIhthcLgxDkNIjiUNwwiKJQ4DGZQxC4OA2C2b5xCCa5tlWV5cluBJdgmC4/hgNQ3GOc5wDCHg4DgNQth4Mg3m6DgzC2OqSC2gogCAMJzpuPYgDmnKXHOHo3Del5EpcY6JDMOKaCAOJwDeHJvDcMati+rQ0nmWJ9D6AQ",
        categories = "design",
        tags = "gizmo,transform,orientation,orbit",
        contributors = "lscheibel"
    ))]
    Rotate3D,
    #[cfg(feature = "rotate_ccw")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQyGEOQghEMINCCFA5C2EQ5C4Nw1hKHIehSIgtDaHA0CAMomEyDA4CIPg8C+A4Fi+MoGgiCoMDMdg1GgNYujCNQ+gE",
        categories = "arrows,design,photography",
        tags = "arrow,left,counter-clockwise,restart,reload,rerun,refresh,backup,undo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCcw,
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
        svg = "gAPBjGkchjGwZQgGMeA9CINgigkeYMDEOYPHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0OQghMaA4C4NRjC4NAgDAIAuDkLQuDGKguDOOAyhmG4hiOIIiiSJgiE0NQuDIIJKDIQQzi4IJRDUM4zleNA2lIMQyEiXJAC+QhokSI4lgwbZMmmNAyDCYJimSRonDKOgxkoMxhlSU5SjSfAtlGPZ/m6RZwmaSJ1k0aAtDSPKCkOAoEgaCIKhIOIPGOEQiDWFYXmCHaRD6AQA",
        categories = "maps,navigation",
        tags = "path,journey,planner,points,stops,stations,reset,clear,cancelled,closed,blocked",
        contributors = "danielbayley"
    ))]
    RouteOff,
    #[cfg(feature = "route")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmCwxDkIg+DwL4DgWB4bHAYR0GgIBkgsTQ5CCGBoDgLg1GEM4vCCMg1CAMI3jkMAtDcaAtDEMYxjONY6iuN48EgMQ1hqHIiiSG4egaCIQhcOIUhYIpLgmC4NhuHYElIPoBA",
        categories = "maps,navigation",
        tags = "path,journey,planner,points,stops,stations",
        contributors = "danielbayley"
    ))]
    Route,
    #[cfg(feature = "router")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDgIggHiDgyhEcoTCKFQgHcaRkHQaIUDCER5g4MQ0CIPg8C+A4FikcBhh8IBkg4TQ2C4MAxCAMQ4EgNooiqL4fi6MBojKNAxDCN45jsSJIj8L5BGiQ4xjMIhNDENY6DAdonimUJElORZVlcNwuDgNAgmUMQ3GGaJoDAIJwDALQ1C4Ng2nGT5RmGRpWDKSZ3CANAuDMNBhDgIKInKcQtDEMaEjmIpenuAQA",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIByg4IgygwaBlGkZxoHSCQxDiDB3GkZB0GiHYfhGCIUCIPg8C+A4FiwbBpG4ZYNDGD4MHmNwiDGFYNDKCQyDGOZAjyFYsC+Mo0D6AQA",
        categories = "layout,design,text",
        tags = "split,lines,queue,series,list,vertical,horizontal",
        contributors = "danielbayley"
    ))]
    Rows,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxGEOQghEMAghQMYShIIg+DwL4DgWG4egaCIKgwNBhDENoNimFIWiqKoahyIYbGMaRyGMbBlCAYx4gkNQijoeYJDEOY/HKQowC+NI2jgPoBA",
        categories = "development,social",
        tags = "feed,subscribe",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Rss,
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
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    RussianRuble,
    #[cfg(feature = "sailboat")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgSAyGENAghQMAgheF4UDQaAxhKG4YiGGgtDQWgiD4PAvgOBYoiuBoIgoMgxg6FAxheDQzjSHQ4iaKIqgQaItkCB4JE2NggDIdgxDaJ4pi4PoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCAMQxHYMxhDGDQgDCFYXhMMRoDENoShSFogg0LYPC0MwiD4PAvgOBYoiuBoIgoMQyg0ORIDSHoTiEMYjiMdgtDKOIXhaGYjhuHYZkKFIZHaQJIjqI4NGiJQuDiJooiqBBoi2WYHgkbYMg4IA3C4Nw3C0NguDCN4zjOOggDILg0DaFRMDKGQxEgMxaieKYuluBZdjGcA5DeNJkoWhQxDWG5wDUbJTDcNQgDQLg1GGbJJjucA3DgLgyDee5Xn6AQ",
        categories = "food-beverage",
        tags = "food,snack,dish,restaurant,lunch,meal",
        contributors = "kemie"
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
        svg = "gAPBwGEdBoCAZA9CITQxDMIA3CAOQgg0NYOGwNAgDQIg+DwL4DgWG4egaCAiG0MYPDEMYYhgLYXDSLIshqHIhiCBIigkbQ4CAMQyiqFw2C2P4uDQWoxh2NY0gWB43DENggjkMwtDORYzgKNZKgqEQyDEYZNk0MAgl8MJAkCU5HgE",
        categories = "connectivity,science",
        tags = "space station,orbit,transmitter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Satellite,
    #[cfg(feature = "save_all")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CANBhDIIIQDAIITDGEQtDIaAxDAbA0g0dobC4MoPhGFImDGGAgDELg4EgOIkhKJ4YhgWgiD4PAvgOBY3jqBoIgqG4RHYNBoDaNo4j2PIEj6CRNDEOIqDgdgtDcaAtlIN5HjmS5KgWB5Nk+EQyEiDoQjGFYzDIVpGjeW47gE",
        categories = "text,files",
        tags = "floppy disks,copy",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    SaveAll,
    #[cfg(feature = "save")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAyDESA1GEMoOCAMIWCAMQtDKGxWhKFIUheFwxg6GxoDEMRsDUIA1HaKIThWIoZhuDh6CIPg8C+A4FjgcBvGweRsGkbhlCCPpDHQc4JDEN4OiSTIZDMIJNDGUpNg+N45j6QJCkSPY/kGQ5FkcbpJgmTZWCAOIZisOJZjqYJdGUPoBA",
        categories = "text,files",
        tags = "floppy disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Save,
    #[cfg(feature = "scale_3_d")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDEOYOGMeYShQPg8C+A4FgeGodgaCIKgyFYRCINYVheKAihqHIEiKGhwGEdBoCAZILE0NQgDcdgxDIaI/i2G4zjWMo0jaOAiG2O4TCANgtDaQwvkUaA+gE",
        categories = "design",
        tags = "gizmo,transform,size",
        contributors = "lscheibel,ericfennis,jguddas"
    ))]
    Scale3D,
    #[cfg(feature = "scale")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIIMCAMwtDiEAgDgYwtC4OA3C4Ng1C0MQuDkMoOC0M4OHMLQyC4MYRC4M4ehEMRaCIPg8C+A4FjWOIGgiCojg+EYTiaFoYhqHIeiCIokiYMYoiqLIYi+JYfjONY3gQaI6liB4JE0NwgDIMRoDEMI0jaO5agWXAiE0MYjDMdgxDiZpXjmApbj0TYmDcaAyGOIwwCCSAgDeKZgg6gqHl8Mp9nSaIB",
        categories = "maps",
        tags = "balance,legal,license,right,rule,law",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Scale,
    #[cfg(feature = "scaling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAzCAOQgDENQiD4PAvgOBYWhmBoIgoMQyg4SAzHYMQ4GiJh2C0OYVheHIbgSHYJE0MQ2g4aA1HaFIWhiMYwgWB4zDENISDUSA5iqO4uj6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgGMeIJDGHQgHKaQimYeZpjaIJgmKZJOiaPg1CCahzDILg1C2ew3oKDQgoOh4aoCiYqoGhKOoKS44iOAQA",
        categories = "photography,multimedia,layout,accessibility,security,devices,account",
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
    #[cfg(feature = "scan_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k2Ao8icTYqDGGwxDCS44iOAQ",
        categories = "devices,shopping",
        tags = "checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer,qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ScanLine,
    #[cfg(feature = "scan_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgGMeIJDGHZmHmaZrHKCQzksL5gmKZJOiaCRtDENggnwLQxC4OaAoKc5NgE",
        categories = "photography,multimedia,layout,accessibility",
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
    #[cfg(feature = "scatter_chart")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINwuDUIoJHiDIOhAIBygyDwiD4PAvgOBYHhyH4GggY4TCIMQ4hqCYLCINYrhgIoahyHoEiSIo2geEoMDEMYwhmFoKjyPoQjSI4hgKOYIjGK4mhSTYtDENozh2RxljiIIllENJNicMYVhGTJFlWSocHAYR0GgIBkgwTQzCAMx2ikaIphuHZnmkPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[cfg(feature = "school_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQwCKCR4gwMQyhAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MgyCAMgwFYOBoC0NBsC0NoxjQIA0EgMh2hQYYqioMAgkGQY/GgMQ2j6K5CkuRAtDIWoZhuIYjiCIokiYIhNDYIAxDcdguDAMZRC+UxolWI4lieWwxDOX5hmOZZnleJwxDiXJemCYoamSVpymmWZ1lybZ5nCfQ8mWfxNDENIrjsLQ1kmQJMk6Th2DCkZMkKTorHYNaFlSAQA",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CANggDgLYMDgIA0CIPg8C+A4FheGoGgiCgxhMMQwhQIAyHYOBhDKJggiSJAxC2KwyEgNIqiyLggjAMoxHYLQ4GwNIxhaGIdhyBIegkTQxgwMonhGNorjgMIxjwMJQi2WJTjIdoVheGZHkaBYHkmIQgDUdgxDeQ5fhuApHmMIhNg6Z5pmuRQ8GMaRyGMbBlCAYx4gkMQyCKfx5gkOaFHKCaEl6eZ7n0PoBA",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NAyCAOYNg8OAgDEMgiD4PAvgOBYZGMaRyGMbBlCAYx5gkOAiCAcoJheJR4gkNIYhqH4hiOGYcgaCAiG0MQ0CANgtDgLg1hSQ5FjOG4EGiHogiKJImgkMQ2iqLAii4Y4wCKMoZC+NZPjiS4HgkTQxDALoUj6aIVj8MYpl2OZhgWY4KlOFQyGgLYXnCS5yjqZAyg+Fp5nuGpxgE",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors_square_dashed_bottom")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDENodhGH4WhIMh2iWJ4ViCGINCIPg8C+A4FjWOIGgiCgxhSDhIDiNI2juOoEjyCRNiWDQyGiGJEjeSI1GMaRyGMbBlCAYx5gmQ5bHiXgiCAcoJDKUZVleWZHgWB5KDkLg0DGDJwnKDAxhGeJRkaApIm6Pg0C4OIWoGgwxoaQ41lKOQ8mmWJaGOYQil+XIJiWY5lCKZ6Ko6a59m2PRtocIA2C0OAuDUOA2CCp6pDae5TgEA",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[cfg(feature = "scissors_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAch4g4MoSHmFoShWD4SHcaRkHQaIWhEPg8C+A4FiYYxpHIYxsGWE4aCAY4cDiEhjhkIo3iYL4si6MImHAYYiCAZIOE0OQuDQMQ0CCSpMk4MQyCCUwij2Q4ikKRBokaSJNC4OJVDSYZVmIMY8ieWRoiuLYvjGOYODENoSHKM41g6aY+m6QQ8muXgiG2aAgDYLQ4C4NQ4DYIKHomdJYlwPoBA",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDgLgxDIIIoioIIuiqGIaiCIofiGI4lCITQyDAIA0iyKYrDENQuDgOIyC+NBohmHIGgiCoMhSFgiDGR4PhEIoThmG4Ek6NoiiSJgxDSRYvmQOAgjyaQwkiSg+gE",
        categories = "text,design,tools",
        tags = "cut,snip,chop,stationery,crafts",
        contributors = "colebemis,ericfennis"
    ))]
    Scissors,
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxGgMQyGEMoNCAMIWhgMgtDIdobEgMQwHaEoUhSFwxCAMQtDSFhWDWE4ViaFoqhYdgzGgNAiD4PAvgOBY6j2BoIgoMQ5igN4ti+JYYDCG4ejiOo8gQaI/lKB4JE0MQ1CAOBoC0NY5juQJUgWVpDlqEZdl+UJigEA",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "danielbayley"
    ))]
    ScrollText,
    #[cfg(feature = "scroll")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxGgMQyGEMoNCAMIWhgMgtDIdobEgMQwHaEoUhSFwxCAMQtDSFhWDWE4ViaFoqhYdgzGgNAiD4PAvgOBY6j2BoIgoMQ5igN4ti+JYYDCG4ejiOo8gQaA+gEA",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Scroll,
    #[cfg(feature = "search_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMQxCAMoQCANAtDQIg+DwL4DgWGBjGkchjGwZQgGMeIJg4IokHmJwxikcoJDiF4Zh6IIihiG4GgiCgyg+PIVC4M4/DOMoagQaA+gEA",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick",
        contributors = ""
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAOQtDIIIQhIIg+DwL4DgWFYYgaCIKDEMwgh+EYPiQMoUhaG4VGMaRyGMbBlCAcoJDgIggGMeIJDEMY1GMeY5juFQviuLYvhqBIcgkbQyDGEQxC0NAuDOT5RieF5HD6AQA",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = ""
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FhcYxpHIYxsGUIBygkOAiCAYx4gkMQxicYx5iuLYXC+HYfiGF4agaCIKDIMQgj0LQ0C4M5BkOFoYjkPoBA",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/",
        contributors = ""
    ))]
    SearchSlash,
    #[cfg(feature = "search_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FheGoGgiCoRg+IQghSFoYh2FxjGkchjGwZQgGMeIJDEMQiCAcoJDiNhjHmM41hcL4qiyLocgSHoJG0MgxCCSgtDQLgzk6UImhmRg+gE",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort",
        contributors = ""
    ))]
    SearchX,
    #[cfg(feature = "search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCIIBygwOIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwbQyDEIIqC0NAuDOLowhqHIiiQPoBA",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass",
        contributors = "colebemis,ericfennis"
    ))]
    Search,
    #[cfg(feature = "send_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCCDIMDkLYPCAMYQDkWgiD4PAvgOBYZhyBoICITQ2hMMhoDENoYhqHw+gEA",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "danielbayley"
    ))]
    SendHorizontal,
    #[cfg(feature = "send_to_back")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIB5g0MQ0hAaBlGkZxoHSDYPCAch4g0MoQiEIoUCIPg8C+A4FimLIGiWI4RiKFoYhqHIOhCII0geCYLh2KIqi+KRwGGCwgGSDRNDcIIUHYMRhDIIJSDAIJVlWUgyGgMZBC+RYLkSRhokiSoUCAN5blGU5WmwMZrDKT5dl8aA+gEA",
        categories = "design,layout",
        tags = "bring,send,move,under,back,backwards,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    SendToBack,
    #[cfg(feature = "send")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIAyC0N4ODALQ0C0OYWhQWgiD4PAvgOBYch+BoICITYMg4IAxDGKQzhuHYiD6AQ",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "colebemis,ericfennis"
    ))]
    Send,
    #[cfg(feature = "separator_horizontal")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgzCIIB5DKCgxDKDR5gkIoSg0eIQCIMgxCIPg8C+A4FiAcBvGweYjgaJoEHQc4KDgIIxhIIA0CAMQ2jKH4hiaKIqiWJ4pgSKxvi2L4XjmOI3DIIAyDCMo3DaOwvj2QokgE",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDEIggHgMYKDEMoOHiCQihODh5hEIgzCIPg8C+A4FiAcBvGweYjgaJoEHQc4KDgIIxDQIITjKNQ2h+IYmiiKolieKYEisb4ti+GA2jgIAyDCNQykkOI6C+PJBiSAQ",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[cfg(feature = "server_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0NAuDUIAxDASA0GEMggjIMAgjUMQtDKORWjCMo0jaLIzjkaAxDaMYzkCN5IDIdo9kiNY3jmMxoC2KoZhuIYjiCIokiYIooiqLA0i+R4/lCUpMk6ZpAj6RJGj6SZsjkdgtmqcQwjmQ5VDWVwvlkaJbiOJYnDYIA2GgLgwDGfZ/oGXaEiwOKIoqjJco6gwiG0MQ1C4N4sDMLg0lUOZVhiGp+pYPJ/pgbQ5C6MotC6pKyqWlZaqqXKsrGhabp0LgzqOtqArigpepqoK8pwN5VqKi6no2xKPpmsQ4CCrrAqELbOliqarsYOK/p8Lq8sywqXsYMYpp61IsuWz7drm6KgDQILgtioA5uaAQ",
        categories = "development,devices",
        tags = "cloud,storage,computing,cog,gear",
        contributors = "karsa-mistmere"
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
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQ4gweYJgsIByheCoMHKCIdCAdxpGQdBohgMAiD4PAvgOBYri6BocDENIeiCGYjiWJ4KimGochmN4Mg6EIShSKosjGKxsGkbhlCAeQxgkNoMHgMpSC4MAxhaVgilOB5Rl2RwvkuTZKkyTpQgkMYVgeXA2liWpPlya5UmCU4rmOZw+gE",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA3GgLQ5CIPg8C+A4FhSF4GgiCgxDQIAxDcSA1hOFYahQYxpHIYxsGUIBygkMwiCAYx5gmIYzGMeI3DeJQvimK4tiiKosi6MAijKNI7CKPY0jaTI+kCRQ+gEA",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgyDIIAyGgLQuDQNBhhCEAwCCGwwC2GR2C4MQ4hiEYcicMQtDEIAxC4NwzGyFA0DODg1iWGooh+HIxiINYUDCJIZieHYfi6NIvjyDwuDOQYmh2J5GhGRhsj2Io3kOLJZi0NwyiENQxleT4piuWw0jwMQ1C4MA5mGQ4Ui+UpIg6DZMm2T4Ni+U49j+TY4huK4alSM41naWoskYVgyDChYbhkaIVheQpPo2Hx2hSI6FmSKp6jOFAyjakook6VJomqfZYo2RpvjCc4UDObKhkSqp4jCl4+qamYqpsNw0pYLqgk6opji6Zp7mqsLBk+s5Tp4Mquqek5Fi+q5nmmQK5oGMgzp6wJ+iyupbDMVqRsmHIfh8egiD4PAvgOBbrGMaRyGMbBlCAcoJDMIggGMeIJgy+xjHm/wyuq7LxvO9Q+gE",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4MwgDEMBhC4N4TCAMIWg8LQuDYMg2C0MQuDANw5EwMYgDQIAzhKFA3hiFwxg8LgxDkOIaDANAziUNoNCCDA5iuFYXi+Gg1DcMoxDEWgiD4PAvgOBZMHIZRjHQIB4gkMwiCAaBlGkZxoHSCQ3locpXCIMZaHcaRkgWYpaHmCQxDSS5NlKVJMGMaRyGMbBlCAY5mDGFA1mSWAuoSf5wmeg50C+eZ7n0PoBA",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = ""
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEOIOGMeYLDUIg+DwL4DgWB4bh6BoIgqDIVhEIg2hWFwiDEMoahyIoggKBIjg+LAxDmJ4ShSCYLg2G4djWMxsGkboIHkMYSDMLg1DGDh5DKEg3C4NI6CAeJKCIOJNlceJSi0NZVi+QZFkeG5mkiYAxDCVZemCXA1leSYLDaTZPliWgxmINJPmWRhlD6AQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyHYOBhDIIITDAIIWhaEwyGiDoShSF4ghkLYPC0OAiD4PAvgOBYoHAbxsHkbBpG4ZQgi6Mx0HOCQxDYII9g6Hw4j6J4pi6MIyjSKJIjUeQxgkMgiCAeJOCKDpRHgMo7lAIB5lmVQ1kQL5LD6AQA",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByhcIgygwaBlGkZxoHSE4VhqCIdCIPg8C+A4FiwbBpG4ZYHDGGIWjcIg5gweAygkMgxhaP47iuLYyjSMYzjUeY6DENZDhOUI2jiB5EkGRgvkgZZKjSVoJjwIB5leQpijqYR4meWZbl2NY+lKUYdmWTZgj2TpQiyWpLD6AQA",
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
    #[cfg(feature = "shield_alert")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgqDIVHYNIti+NIzgSNYJguDQxDYaAuDAMY8jCQA+gE",
        categories = "account,security,development,notifications,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,warning,emergency,attention,urgent,alarm,crest,bravery,strength,tough,attacked,damaged,injured,hit,expired,disabled,inactive,error,exclamation mark,!",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldAlert,
    #[cfg(feature = "shield_ban")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoICIbYUDWKYUgyLYvjQPoBA",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,cancel,error,crest,bravery,attacked,damaged,injured,hit,expired,eliminated,disabled,inactive,/",
        contributors = "danielbayley"
    ))]
    ShieldBan,
    #[cfg(feature = "shield_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoICIbQ5imDY7CANITi2L40D6AQ",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secured,safety,protection,protected,guardian,guarded,armored,armoured,defense,defence,defended,blocked,threat,prevention,prevented,antivirus,vigilance,vigilant,active,activated,enabled,detection,scanned,found,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audited,admin,verification,verified,certification,certified,tested,passed,qualified,cleared,cleaned,disinfected,uninfected,task,completed,todo,done,ticked,checked,crest,bravery",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldCheck,
    #[cfg(feature = "shield_ellipsis")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgoOIpDEaAuDAMYti+NIzgSNYJguDQxjqPI+i6MJDkKBYHkUMYnkiO49j+TYygE",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,securing,protecting,guarding,armoring,armouring,defending,blocking,preventing,antivirus,detecting,scanning,finding,auditing,admin,verifying,crest,upgrading,loader,loading,throbber,progress,dots,more,etc,...,…",
        contributors = "danielbayley"
    ))]
    ShieldEllipsis,
    #[cfg(feature = "shield_half")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgqDIODIVgyi2L40D6AQA",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,logo,sigil,flag,team,faction,fraternity,university,college,academy,school,education,uniform,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,ranking,army,cadet,scout",
        contributors = "danielbayley"
    ))]
    ShieldHalf,
    #[cfg(feature = "shield_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgoOIpDEaA4i2L40D6AQA",
        categories = "account,security,development,gaming",
        tags = "unshield,cybersecurity,unsecure,unguard,unblock,antivirus,clean,clear,disinfect,patch,fix,stop,cancel,remove,relax,admin,crest,bravery,weakened,damaged,hit,unarm,disable,deactivate,decommission,downgraded,minimum,-",
        contributors = "danielbayley"
    ))]
    ShieldMinus,
    #[cfg(feature = "shield_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLg3CAMQ0GENguDkIIUhYMAghqGguDMLQyFYNRsC0OAth8MwuDKEIqCIPg8C+A4Fi+MoGggIhtiuOoaDIMIujCNY0gSNoJE0NIOCCR4PDQIA1HYNxjhoNggDiEIalUMQwGGPYeCCXAzhuYYaDULg2DILZHDMOI/jGQw+gE",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,interception,threat,prevention,unprevented,antivirus,detection,undetected,exploit,vulnerability,vulnerable,weakness,infected,infection,comprimised,data leak,unaudited,admin,verification,unverified,inactive,cancelled,error,crest,bravery,damaged,injured,hit,expired,eliminated",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShieldOff,
    #[cfg(feature = "shield_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgoOIpDEaA4i2L40jOBI1gmC4NDENRWDePYwkEPoBA",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,extra,added,professional,enterprise,full,maximum,upgraded,ultra,activate,enable,audit,admin,verification,crest,medic,+",
        contributors = "danielbayley"
    ))]
    ShieldPlus,
    #[cfg(feature = "shield_question")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoIgoOQuDEIA5GEM4gCCJYljoNQuDiDQxiSDgtj6HpMi2L40jOBI1gmC5HDcaAuDAMZPjCUw+gEA",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,undetected,scan,find,exploit,vulnerability,vulnerable,weakness,infection,comprimised,data leak,audit,admin,verification,unverified,uncertified,uncertain,unknown,inactive,crest,question mark,?",
        contributors = "danielbayley,jguddas"
    ))]
    ShieldQuestion,
    #[cfg(feature = "shield_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBYujGBoICIbQxDQLg1CAOQtjwNYti+NIzgSNYJG0OY7j0IJAkKMJGD6AQ",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,prevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,inactive,cancel,error,wrong,false,crest,bravery,attacked,damaged,injured,hit,dead,deceased,expired,eliminated,exterminated",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldX,
    #[cfg(feature = "shield")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIcw4C0NAghIMQwFYNRsC2EgzhwIAzHYNxjDAIA2hUIIXiiFwiD4PAvgOBQ+gE",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis"
    ))]
    Shield,
    #[cfg(feature = "ship_wheel")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeYLDEMoOGMeIShQPg8C+A4FgeGhwGEdBoCAZILE2EwgDIdg3C4NQihoL4hiOIIiiSJgiG0MQ5CANQtDULgyDOPZBDOMIbjMaI1iOJYnDIMgghMaAti2L4xkmS43guOo8juP5Fl+QpHjKNpZk0IoolAMQ0i4VpPmOWA8kmZ4oDCRZRDMLg3DePZRDmcJlnKNp0DmLpRDISIZkigZzjidZ3DGdpCn2VqLjSAoEgaCIKCIMouhWFwihOFYRqKiocpmH4BA",
        categories = "transportation,travel,maps",
        tags = "steering,rudder,boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "danielbayley"
    ))]
    ShipWheel,
    #[cfg(feature = "ship")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGMLg2C4NQgDELoMDGDYThWGoUDCHQtgwNYhhULgzCCH4WDmGwyhuGYRi6F4ci2FIZjSKIgiKJIWieKQuiuFI3DEIg+DwL4DgWRZIgaCIKDGKwzDiDQwEEMYWDaFZXjiH4fg6FQ0GwLQ5C0NJiCANBjl2P4/DQIA1iabYtDiGQ3C4Nw2kSRpLkqBJMgkTZPhUMxWDcYYMgyXIoiGIRIoWh5boqhx2niRZHn2fIFgefwxhgMB2DSeaWkmAp9pqTqSDOoZ7gE",
        categories = "transportation,travel,maps",
        tags = "boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "karsa-mistmere"
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
        svg = "gAPBwGEdBoCAZA9CITQ2CAMggDMIA2HYMQ0GGDoODAIIZhmFxohSFoNhqIocC0MhWDYbAtDMLQ0FoIg+DwL4DgWMIzgaCIKhANoeDiL4xjaNYEjeCRNDGDAxDAYQ0CCS4bCAMQtDiGo+jKQg+gE",
        categories = "shopping",
        tags = "ecommerce,cart,purchase,store",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ShoppingBag,
    #[cfg(feature = "shopping_basket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAMQxCANAtDcIg+DwL4DgWFYYgaCIKDEOYNDELYRhOFYXgQaIaiiB4JE0MohGgMgwhSFobiqBYsgoMwugyDoNC4NggDcLg0GGL4vDAIJJkmLwxkAaA5C4OBjC6IJJk4OAtC4NwgDILZODYbJODeEpEjSJ4ZgKK4dG2II+g8OZnjaao4h0TQ0jyDQ1jwaAxDWcoojeHIJG2fohl8IJxiaNoBA",
        categories = "shopping",
        tags = "cart,e-commerce,store,purchase,products,items,ingredients",
        contributors = "danielbayley"
    ))]
    ShoppingBasket,
    #[cfg(feature = "shopping_cart")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDiDhjHmCwyg0Pg8C+A4FgeGodgaCIVheDYPhGDA5g6CoMCKGocgSIoaHAYR0GgIBkgsTQyC4MA1CCPI+GgMhsjwNg2CAMY8DQMhhDKQAgDCUZTk8MQuDUOBoDkLg3DiTpQlKYZJC4OQ1C2Vg1DcbJWDaZg3C4NAzEgNQukqLobjSNg+gE",
        categories = "shopping",
        tags = "trolley,cart,basket,e-commerce,store,purchase,products,items,ingredients",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShoppingCart,
    #[cfg(feature = "shovel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyHYLQ1GwNYRCANYWhUNR6CIPg8C+A4Fh2IIGgiCg5C6FwxDSKAgDENggDiHIeiOIoEiSCRtDEN4NhaGIoigYQzigMwgkINZEDAIJJDGFQwHOSZQkqQZDkWVJRDGSoREyOoNjKH42D6AQ",
        categories = "nature,tools,gaming",
        tags = "dig,spade,treasure",
        contributors = "Andreto,ericfennis"
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDcbAtDKFQiD4PAvgOBYZhyBoIgoMQ3CAOB2C4OBBDYIIrDAIIuDEIAxDOKIODAdgwEgMY3DCKguDWLI/i+Q4xiQOBoDAYZAkCLowjKTRahiGofh6BIggkbQxDSMg0hWDpShuVg+gE",
        categories = "nature",
        tags = "forest,undergrowth,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Shrub,
    #[cfg(feature = "shuffle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ4GgMQuDQY4RDMIAwCAMguDULQuDYIAzC4MwthENxsDYLgxC0OIeGMLg3iSKYZjEN4giKNBIDIMgiD4PAvgOBY9kCBoICIbYOhkIA0koLZLDSPI+kOQoEkSCYLCANoQC4OYUhuF4ZluW42h+Go7j2P5UlOBYHlaOoNg8LQ1lsY4xhYMAthoNodjCIYjhEOBsh2HAuDiUJokGApUmyRpIDGTpMk6hpSgE",
        categories = "multimedia,arrows",
        tags = "music,random,reorder",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Shuffle,
    #[cfg(feature = "sigma_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoShUIgzhIdxpGQdBog6EISHmDobD4PAvgOBYoHAYYgCAZIOE0MQ2CAOAuDkVg3EgOBsDQIA1C2QA1GgOB2C0MY5CKKAvi6IA+gEA",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
        contributors = "danielbayley"
    ))]
    SigmaSquare,
    #[cfg(feature = "sigma")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA3FYNBIDYbA2CAOAthUOBoDEMh2C0MwiD4PAvgOBQ+gE",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
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
        contributors = "ericfennis,danielbayley,karsa-mistmere"
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
        contributors = "ericfennis,danielbayley,karsa-mistmere,azdle"
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMwiD4PAvgOBYThaBoIgoMQ4C4NQgDEMxoC2IhMg0OYfCANYqDYaImDKKIfFqEoUhmGIEhqCYLg2Ih2DiNYVjkPoBA",
        categories = "arrows,navigation,development,gaming",
        tags = "bidirectional,left,right,east,west",
        contributors = "danielbayley"
    ))]
    Signpost,
    #[cfg(feature = "siren")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQyGENQghEMAghQMYSC0NR2DCEIShWH4XhGGg2EgNx2C0NhaCIPg8C+A4FiyL4GgiCoRDKHAyCCOYUhaOgtDIaAxjiOofj2OQyHYMhIhqP4qiyLoEGiMZRgeCRNDKF4OkGK4tjKU4FlWCgxDgLoRDSZYNDiEpclCMIClSNJXg2QAxmyXpvmCcYOjodp1k+d4ymEbZnDkMg5CChKGC4NwwDei6NnaUZfjOVp7g4dg2pGMIB",
        categories = "medical",
        tags = "police,ambulance,emergency,security,alert,alarm,light",
        contributors = "karsa-mistmere"
    ))]
    Siren,
    #[cfg(feature = "skip_back")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDkIAyDAIITDEMgghIIA0huE4VCIPg8C+A4FgcbojGyDBlCAeAxhANQiCAeYwhEOYzHgMoxjMeY7CKMojC+KxuGUPoB",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig1CANAgDGEgxDIIISDIMIahMIg+DwL4DgWBxuiEbIMGUIB5DKEAxDkIggHgMYvjGLI1hGMh4i4IowiCIopG4ZQ+gEA",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMYQGMeIMDkIg+DwL4DgWB4bh6BoIhKDYVguDYPgmF4NDWGociKIA8HAYR0GgIBkgwTQ4CAMgwHYMhoDgdgtg+GwvjSNobkmN45CIbYOC4NQgDENwtlILQxleUwxGgMR6i+SI1GiS5jjiOgxDaPQwGEMo9CAMJwnIMZSDYLQzC4MpTjyPJxDGcJZnSDpwEGbpunGiAgjyPphkwPoB",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slack")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgzCIIB4g0MYPCAeYNDKEByhIIgxC4NYQGgZRpGcaB0g0OAiD4PAvgOBYrHAYYLCAZINE0MQ5CAOIfFYMQwGiHg1EGQQgkQMJFCCR44jqH4qiyMYLiuLoGhyKQgiKJImiiEIXh0NIahyQYQgiCoMg6TotgQdIwjIaI0jYNZFDWPAxDQSAzh+Q4fkWe5KkkIJxDGc4gisL5QGiUpqleI4lieZ4HgmC5bhGE5fhaE4VhuE5NoWU5sjONQijeg5FDkSJ1HaQRhkSRpIkqHwtmKhaHomBQgpqHZNpCZaTlijYNhWHIZpcIopp2aqfm6oRNjucQ1qcMBWniQqsn2rpMs6aK0gE",
        categories = "account,social,communication,brands,development",
        tags = "logo",
        contributors = "colebemis,ashygee,wojtekmaj,mittalyashu,ericfennis"
    ))]
    Slack,
    #[cfg(feature = "slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIINg8MgiD4PAvgOBQ+gEA",
        categories = "development,maths",
        tags = "divide,division,or,/",
        contributors = ""
    ))]
    Slash,
    #[cfg(feature = "slice")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4CAMQ0C0NggDYaA5HYLQzCIPg8C+A4FhqHYGggIhNDEOAuDMNwgDMLg2DMIIMg4bIuDMTAyDGJ4pDaLAzGEMguDEMggj6QAgDCDZFheFxahmG4gD6AQ",
        categories = "design",
        tags = "cutter,scalpel,knife",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Slice,
    #[cfg(feature = "sliders_horizontal")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDQIoHDGCgyDGDh5hAIoNCAeYJhcIg+DwL4DgWHohgaCIKDODh4hYMQwhSFoYhqCoNh6IIEGWI42hmKwyimGwxjyGY+kCKoRhONIkjiBYPgoOI9ieFJCi6Co/h2H5IgKOYVhGLYHj4NophaEpQluVY1iKWJKjEIgylyJgiiiS4LkCWpri2R42kmJYrhiboMlKa5jCKX53meJJdkyYKIjqU5cmqfqEjeaIGnQMZNnEMZfoeC6ZmoMo8pAPoBA",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDQIggHgMYKg2B4RCIMgxg4eIJCKDQ+DwL4DgWHohgaGoShmFYTHmFQxDCDoIgoMwih6IIEGWI42g+GwxDKL4VheL47j2D4sj2NIkjiBYHhuMoUgoOIogqPIZkKM4fkiAo5jCCw2lGFouk6FoYjqCgyi6R42kmJYbmaQYxl6bZhlOaIilmSorlKKo7hOJgil2RJllaNZ1iSgILmCfYMj6T5uCKUJ0jedoGniXJeDGUJkhaQ5bDGXaQD6AQA",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB3GkZB0GiCgxDSDRoGUaRnGgdIKDIMINgmC4NHiCg1g0coliMPg8C+A4FiwcBhhIIBkgoTQxDILg2DYNwgDgIAxDCQQyGgNBsC2Oo8j6FosC+MoSD6AQ",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoICIMYMHmCQ2gwdxpGQdBogkN4MGgZRpGcaB0gkMYLD4PAvgOBYoHAYYaCAZIJE0MQzCAOAuDMMhhDcLg0jaPY/CAMJDCAMZFj0M4VigL4uhqLYvGiMYzDENo+DYIJWDIMRhDEMQuDeWJemCWJEkSR5nl8NQ4CKTJOGiUIwjIIo0DkLg5kcNAulwMQ1neR59n+RZnCALgwoCfpsm6UQ+gEA",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAdxpGQdBog4MQ0hIch4g4MoSh4Ig1hIeYfhyJoPCIPg8C+A4FiwcBhhcIBkg4TQxDIIAxDgaAuDAMYri2MoXD6AQ",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDEdgxGEMQwg6FYVDGDgtDkLYUCIPg8C+A4FiCI4GgiCg4g4NBzDELg1CCDQ0jEIA0C2Mo3h+IYmiAbBpG4ZQgHkMYJDkIggHgMpFC4MAxkceZKCKRpIkSUo6C+PpAj2P5BkmCQxDWTJOlSXw1k+VZTlCRZXlkZYlgSJ4JE0MQ2CANRoDaV48gKcIHnIMQ5jEdp5iCIpwD6AQA",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQ4CAMQ0HMMQuDUIAyCANIyjQLYzDSOAihsL4iiSGxsGkboIHgMYMDmEB4DKSAuDAMYQHmRwikkIB5kyVI8hyQpEkGQ4IlKSJKlgMQ1k6UJWliVZGhMNZaC+XBlD6AQ",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[cfg(feature = "snail")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQzGENgghEMINCCFAxgyFA0CCG4XhYLQ4hYIIMhmIoahYIg+DwL4DgWKhjGkchjGwZQgGMeYJg4Io2HiOQwjscoJDiKYrjCMo0iqLYGgiCokDEaIYGMNAuh0IA4C0MwuhGVw4FYNxhiSIgxh+HR2DaRIsgQaJJmqB4JE0MYhDODQ5C6Yw1C4MpokqbIFm6TYMnMMgwC4OQgnieoqmmLoBA",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley"
    ))]
    Snail,
    #[cfg(feature = "snowflake")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyCKBwygoMoMCAeYJCIMYSHmD4WgwPg8C+A4Fh2IIGHiGoXg2GYQhiFYSgiConh2H4EGWHRwGEdBoCAZIKG0MgwCAMQ2C0NJDCCRA0CKMY2jiNY3jmOwiG0NAgDiRpGkWSJKk6TY4jqPJBleU5HkOSYeksaJck+PJVj6YZWlmZpbgE",
        categories = "weather,seasons",
        tags = "cold,weather,freeze,snow,winter",
        contributors = "lscheibel,ericfennis"
    ))]
    Snowflake,
    #[cfg(feature = "sofa")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA5FYNhhDIIITg2FgtDKGBIhGE4VCCF4dHYMwiD4PAvgOBYliiBoIgqEwxDEdg1hKFIfjaDYdGgMYcjWFo2hkMh2C2M4djeHwtDSHx2DKG5CDKNIeheSQwFqJImiuKoEiyCRNkkMQ4kuVonlqWYFgeXIMCCX5hiWY4pgKWpngoMYTDQdg5mKWIBA",
        categories = "furniture",
        tags = "armchair,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere"
    ))]
    Sofa,
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
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Space,
    #[cfg(feature = "spade")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAORjC0MQugyEQ1C0MwgDMLgyhYIA1hIQYegyIQgDCJImDeEggDIMBjhEOImDMLYpDSEgtDIIIUjiKQyC4N4qCCNIMDIYYjiOJZHh2NYhGMMI2C6MYUC0NIWC2IRsC0N5YlgIA3FoIg+DwL4DgWYJjgaCIKDGNwxDgdg0l+YZmD6AQ",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Spade,
    #[cfg(feature = "sparkle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAzC0MQuDkIA1C4OBhg2DQwCCGwxhALgyDgNwghGIQ4EwM4kDIbIVDiJIShgIIahyJIviaNoiEyDIyDEbIRDkLYtjGM4djgN4fiYTAyDGKhskGFofDmQ40h2SA4DiVg3FoIg+DwL4DgUPoBA",
        categories = "shapes",
        tags = "star,effect,filter,night,magic,shiny,glitter,twinkle,celebration",
        contributors = "Shiva953,karsa-mistmere"
    ))]
    Sparkle,
    #[cfg(feature = "sparkles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAzC0MQuDmDAgDULg4DEMxhg2DQwCCHgxhALgyDcNQghGJA1EwM4nDIbIWhiLIRhMMobCCHYfieJ4jiWO4pEyFAyDEbIzgwLYwhmNo4iCPoliKP5Ci0bJHheGYijSSo5iCT5OiiJZAg0MxaCIPg8C+A4FmWaIGggIhNiYMx2DSZJmmuaoEmyCRNDEOYnDecp0meeJ3gWB56iwNRonOZaCmmAp4oabgxDeJw5oqgZ2gEA",
        categories = "shapes,cursors,multimedia,gaming,weather",
        tags = "stars,effect,filter,night,magic",
        contributors = "karsa-mistmere"
    ))]
    Sparkles,
    #[cfg(feature = "speaker")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAdxpGQdBogkMQ2gwch4gmCwghgIg0haCIKgwaBlGkZxoHSGQwCIPg8C+A4FiwYxpHIYxsGUIBygmHggGOHAxhoY4gDGHosC+Mo0jaLBsGkbo3HgMYThoeQygmFYblQIo/C4MAxgweZQCKFZFkuTQ+gE",
        categories = "multimedia,devices",
        tags = "audio,music",
        contributors = "colebemis,ericfennis"
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
    #[cfg(feature = "spline")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ5CKCR5gwNYQHKDAyCIPg8C+A4FgeGodgaCIWCKGIJgsIoUhGDIOhmG4hh8PBwGEdBoCAZIME0NQgDENxBDEMo8kEMAgkQMY8DcIIUhoL4zjUPoBA",
        categories = "design",
        tags = "path,pen,tool,shape,curve,draw",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split_square_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ5EgNRjC0MQgDALQyhOF4XFYNxjhaFAxhcIIYDIaAzCIPg8C+A4FimLIGgiCgxDYIA1iYY4UDCI4NjsMh2DEMIeg2E4jiKJQtieKYrgQaIpGwaRuGUIB4DGCQxDIIpTDKVpYCAeZVCINJZHmWwiDIMIoiqT5RD6AQ",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[cfg(feature = "split_square_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAOBWDUYwwC0MQgDELQyCAMoXGgMQwGOFAwhmFYiDIdgzCIPg8C+A4FimLIGgiCgxDmFQ2iaEYVhOGYXhkSA3GOOoShqFoaiULYnimK4EGiKRsGkbhlCAeAygkMgwCIIB5lQIgxDKWB5DGCZdlgeJhCINIoiqTpQD6AQ",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareVertical,
    #[cfg(feature = "split")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzGgNR2DUIg+DwL4DgWFYYgaCIKDiDhIDOEoUhaG4agSHIJgsMggDIMh2C0OAuDMYQ0CCNgwCCOQwC0MQuDENwyC0MguDiQRMDODokheKIngWB4JG0MQ1CAOQgDYLQ2kuJoBA",
        categories = "development,arrows",
        tags = "break,disband,divide,separate,branch,disunite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Split,
    #[cfg(feature = "spray_can")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoC4MAxCIPg8C+A4FhSF4GgiCg3CANYPhGE4VhqGYEhuCRNDEMQgDeIYShSFoniaBYHimDIuhCMIkjOAonjaHQgDmL4jjKGI+jWHILCCK5EjGJQ8HIZRjHQIB5gkNQiCAeIJDGWQgHcaRkgWCQ0loaBlGkZxoHSZZFlKVI0igIhtDEOZCCAMp5HYMQwGMMAgC4NgtC4NJMC2LAxGgLQ2GOhA2CAMKIoQNKTDEVorGwMgtDKRZQhqQJ1gwMaGDinKej2oIcqKTJ3qanZPjOAQ",
        categories = "design,tools",
        tags = "paint,color,graffiti,decoration,aerosol,deodorant,shaving foam,air freshener",
        contributors = "danielbayley"
    ))]
    SprayCan,
    #[cfg(feature = "sprout")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgwGgMQwCIPg8C+A4FhSF4GgiCoRg0MBjDULg1C0MojC4OAtDYLg0CAMwthGE4VhqGYEhuCRNDmIwgjoNBjDELpADgIJCg0LgykYM4uC4N4lCCLAtDOJw0C0NIoC0LovkAMpYDaJZZjALg5lGVZHkaKY7laLQwCCIoiDgeoyhaNo1gWB44DGVgxCANhhgyDJsoGYZ7j6QJjkGS4vC4NgglaWoskSMJEouX5KkCTZWl6Jg3kGVaRlYOYNnGFJzhiAQ",
        categories = "nature,gaming",
        tags = "leaf,nature,plant",
        contributors = "ericfennis,mittalyashu"
    ))]
    Sprout,
    #[cfg(feature = "square_asterisk")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NGgZRpGcaB0goMQ4g2CQihEIB3GkZB0GiGYbD4PAvgOBYoHAYYkCAZIKE0MQyCAOB2ieKYuiSLYvGiMYKG0OAuDUIAxDQIA3C0NAiigL48GiPowjIIpDkWRwwkoIJNk+UQ+gEA",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[cfg(feature = "square_code")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEch4g0MoRiEIoeD4PAvgOBYoHAYYLCAZING0MQwCCNQtDIII6jwIooC+LoLi2LxojGMwxDSN5JDKOZNiOP5BGgPoB",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = ""
    ))]
    SquareCode,
    #[cfg(feature = "square_dashed_bottom_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIIMC0MgghGEwiD4PAvgOBYWhmBoIgoMQ0g6IQyhCJQyhWF4chuBIdgkTQ1hIMRhhMIINg0MYmFYNYzhKNY+DGEoQGiII8hGNoOj0Mh2kSNJHjiFIWhiLIrgWB4uDmMZDiiUoagKLJWCITYglkMZbiqAQ",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottomCode,
    #[cfg(feature = "square_dashed_bottom")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxGEMoNCAMITCAMQtDKGBWDWEIShSFAxg2GBoDENIdhGH4WhIMh2iWJ4ViCGINCIPg8C+A4FjWOIGgiCg5g0MYkjSNo7jqBI8gkTYlkCQo1jeRw+gEA",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottom,
    #[cfg(feature = "square_dot")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDB4gmCwgHKGAiDKDB3GkZB0GiFIWD4PAvgOBYoGMaRyGMbBlCAY4ICIMYfjSHY4gwcoUCKKAvi6MIyD6AQ",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareDot,
    #[cfg(feature = "square_equal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoHAYYWCAZIJE0NwgDEMBojeJ4pi6Foti8aIxjONQxDSOQwjsL49GgPoBA",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareEqual,
    #[cfg(feature = "square_slash")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByHiCQygyGYKgwdxpGQdBohSFg+DwL4DgWJxsGkbhlgcMoJDmDB5DGFA1hyNwijQIB4jIIgxjmJwvi2Lw+gEA",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareSlash,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGMLQxC4MQgDALQyC0Lg5heFxWDQY4WhKEobDIIIYDIaIfiKFYmCCGotDIIg+DwL4DgWM42gaCIKg6DQ2hCK4WhiGocDIdgth+IYTkSJYnikY5Bi2L5NjKNI5jMchlGMdAgHIeIJjEIJfCIMQ0CIIB5gmZZnHcaRkgWCQ4mcaBlGkZxoHScZVC+WZbD6AQ",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = ""
    ))]
    SquareStack,
    #[cfg(feature = "square_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYQ2CCEgwCCFQwC0MQyhYIg+DwL4DgWHhjGkchjGwZQgHKCQ0CIIBjHmCQxDGLhjHiMgyh2H4kiaKIeHIZRjHSKo3CKOQgjEIgzi4dxpGSBYyDiLpFksIBoGUaRnGgdJRjoL5AkIPoBA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUserRound,
    #[cfg(feature = "square_user")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIByg4IgygwaBlGkZxoHSCQxDiDB3GkZB0GiHYfD4PAvgOBYoGMaRyGMbBlCAY4ICIMQwgwY4TDGFYRg8IooC+LowjKKBwGGJAgGSCRNDcIAyDEdgtDIYQylAIAwlkIAxlCVBoDaVpYlqWpdlcMh2hWQpIiQPoBA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUser,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKEYUCIM4RGgZRpGcaB0g2D4RHmDYaD4PAvgOBQ+gEA",
        categories = "shapes",
        tags = "rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
    #[cfg(feature = "squirrel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2GENAghEMAghQMAthGEQ3CCG4WhULYbDcY4XDWEgtiUNAtDEMAuDWEIthKMIUDGHw5hUIAyjCOYlh6FI8EOG4rCAMwgDGNJEDGIoUjmDY5DIIIllEaIrCIPg8C+A4FlaWYGgiCgxDaOAwiOKgukELgzC2RJpDOUxhk+T49jgLQyHadIQhKN4WhiGBWDSVZXlyW4El2CYLDULpwDIYZrnqH5OiegJYoSg4FgehoMkWbQuDAMaSoKAQA",
        categories = "animals",
        tags = "animal,rodent,pet,pest,nuts,retrieve,updates,storage,stash",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Squirrel,
    #[cfg(feature = "stamp")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgyGgMQ0CIPg8C+A4FhSF4GgiCgxDkLgyDcIAxDMLg3DMQQyC6DIqgwMAgi+LwxDeK4jDMaAtDEMYpjWLYwj+Lw0iMNYrFaMxhDGI5Aj+SQxhANJIkqMZMjkdo5isYwwC0Lg2DaWwyl4MQuDOW4nlcNw3FqE4VhqGYEhuCRNhGNhWDiKxDnOIgxgyeoMDUYQzCCgZTlqZAzGOV5djALaBDGjKCHOSQykqJQ1kYM5rhabw+gEA",
        categories = "design,cursors,tools,maps",
        tags = "mark,print,clone,loyalty,library",
        contributors = "karsa-mistmere"
    ))]
    Stamp,
    #[cfg(feature = "star_half")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDcLg4CANYRCAMgxCAN4ODQLoYg0OQuDMbA3C0MRMgyFgiD4PAvgOBQ+gEA",
        categories = "social,multimedia",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "mittalyashu,ericfennis,danielbayley,karsa-mistmere"
    ))]
    StarHalf,
    #[cfg(feature = "star_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4Mw0CCDIOCAMggDkLgyDcbA1CANAuDgNxMDWHoUDIMQgDGFAxDcLg3DeJ4MDEOITDEbAtC4NQ5C0MwuDQMwiD4PAvgOBZAkOBoIgqMY8ikMosDaE4UhaGI1DYLg5DELQxEyKITjUMY8g+TZXj+QZGkAbBpG4ZQgHgMoJDIMgiCAeQxm+ch5m4IpwnIeJ1nqZAvmiag+gEA",
        categories = "multimedia,social",
        tags = "dislike,unlike,remove,unrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StarOff,
    #[cfg(feature = "star")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDIIITDENQuDAOQgDgLgyDaFITDmHQ3CAMYkDENAuiiJYcDEOIUDGGIVhUNwuDeJIXDiEwyjEMITieKYriGI4bC4OQxkWHoljsIg+DwL4DgWBxuD6AQ",
        categories = "account,social,shapes,multimedia,weather,emoji,gaming",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "colebemis"
    ))]
    Star,
    #[cfg(feature = "step_back")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDgIggHmCQiDIMIOHkMoKDSDh4heC4ND4PAvgOBYfHAbxsHkZxvG4IIlgQdBzgoMQ0CyEwgjMMQyCCMgshmHwviWJ4pG4PoBA",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDAIggHgMYKDaDh4gkIoTgeEQiDQIg+DwL4DgWHhwG8bB5GcbxuCCJIEHQc4KDEMAsDQIIMCwMQyCCMQsgyHYfiSJooG4PoBA",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4OAgDILgzEGEYRCAMAgDGFggDWDxIDQYQyg+GoXDALYhDIdg1GENggiyJIaiwNh2DCK4tiOMAtDYVofieN4lDKJhoC0MRhC6EIhheGYXhQMwiD4PAvgOBZPlKBoIgqDgxDUdpEjGPo2jKNJei+Fw2jkdgtDSTpQlWTxjGkchjGwZQgGMeYJDEMAiCAcoJDKexjHifp6k8L5vnGcw+gE",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[cfg(feature = "sticker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg1CAMxIDUYQyCCFQwCCGAwC2FQyHYMQ0GOGAxC6JA5haKAyGiIIUiiGoZhaHBWDiDhMgyDoQFoIg+DwL4DgWPI/gaCIKgyEB2DYaA2juPZCkGBJDgmC4jDYcwuDgIAxigMRjiQM4wDILZamEMZMj6UJPgWB5SlgMQzGgMJmk6ApQmuRQ2lmb5xjyZ5AgE",
        categories = "communication,social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Sticker,
    #[cfg(feature = "sticky_note")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg1CAMxIDUYQyCCFQwCCGAwC2FQyHYMQ0GOGAxC6JA5haKAyGiIIUiiGoZhaHBWDiDhMgyDoQFoIg+DwL4DgWPI/gaCIKgyEB2DYaA2juPZCD6AQ",
        categories = "communication,social",
        tags = "note,comment,reaction,memo",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StickyNote,
    #[cfg(feature = "stop_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhyGUYx0CCCwiDmEB3GkZB0GiDA2hAaBlGkZxoHSL4QhKJ4ZhuIYjD6AQA",
        categories = "multimedia,shapes",
        tags = "media,music",
        contributors = "colebemis,ericfennis"
    ))]
    StopCircle,
    #[cfg(feature = "store")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCANwgDQLg0DELYQhIQYMgwMAghoMYNC4OAzCAMhoDgLgzDQYYYhuK4dDGEQyC4NQ5EwMoMDcIg+DwL4DgWOY8gaCAiE0NAgDEMh2DiKYiiuGoahgaJGkqGZMiILZHC0OI4jqP4+gSQIJE0MQ1iKV4oiqTYblaVholaUpUDCVoiHYNJajuXpdgWB5gjYaAyDCdZcgKXp6kKNYNHYM5umiE4YHYMIpC6Dowg6iwti6MgtC4NgzpGkZvpkOIZhenqTlSLQ2kUMqQpKnqVpcOaZpunaUkyoKiqWpaLqmqariKrYspaMawpqnA3r+Ta2huo6srSHAgDivK4seRbBpixKzp+H63qS04dkSRqOoqwAylYVo3jmdo9gE",
        categories = "buildings,maps",
        tags = "shop,supermarket,stand,boutique,building",
        contributors = "karsa-mistmere"
    ))]
    Store,
    #[cfg(feature = "stretch_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goNINHcaRkHQaIKDIMINGgZRpGcaB0goNoNgmCwiD4PAvgOBYoiuBoThWF4LhoIIch6IIiiSGINg8IgxhGB4lgyKIqgQdA+gEA",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDIMINHcaRkHQaIKDaDR5hWDYJCINAiD4PAvgOBYoiuBoZhuHQih8IIhguDYIiIIIkDGJoOhCEoUguF4oiqBB0D6AQ",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA0EgORhDMIITDAIIWDALQyC4OITDQIg+DwL4DgWIYkgaCIKDENAgDEMhhiyLIYi2FwgDgSA2iCIoniEbBpG4ZQgHgMYJh+QgygkMgwCIIB5kQIoukweZIlAMo6C+PpAD6AQ",
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
    #[cfg(feature = "subtitles")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQzGgNAiD4PAvgOBYThaBoIgoMQ1g2DwyhKFIZhiBIagmCwgDkaIhhOFYmiWBYHiiDoqhCIovheAomjOCgyDGDQ1GEMggkQMAgkcMQtkQMhIDcbAtDQIA0FaQpMkiWJADKSxoDENJDkWWJJmEMhajiJIBA",
        categories = "",
        tags = "captions,closed captions,accessibility",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Subtitles,
    #[cfg(feature = "sun_dim")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME2DggDQaAuDAMYZhuIYjiCIokiYIhNDIMAgg6LIujAL4yGiNIjiWJ4pjqPovhqQY1kSNonDSPAykqQJCk+Ro4DENwuDYNQ3CANguDMNAzlWTJXDyQpZiiXJemCW5dl+Z4xk6ao1myYpkDOPJunOLZLnWM53kWNxNnqZZhmOZZ0k2M4B",
        categories = "accessibility,weather",
        tags = "brightness,dim,low,brightness low",
        contributors = "mittalyashu,bduffany,karsa-mistmere"
    ))]
    SunDim,
    #[cfg(feature = "sun_medium")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCAMx2DGGociKJIhiOJYnCKKQyCAMgwi2LwvjEaIziSJooDMIIOGiLobj+NJCjWKI7kcMpJj6QJOkQIhtDEOAuDMNg0CANQuDaXgtC4NwwDeZ5plWTQ8kCWBtDaXQ0kYMZqDYNQ3maaJqn2bYym+NJxmKZA2mGY5emufpskuVqCkONpaniepHpSjKYoCQYB",
        categories = "accessibility,weather",
        tags = "brightness,medium",
        contributors = "mittalyashu,karsa-mistmere"
    ))]
    SunMedium,
    #[cfg(feature = "sun_moon")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA4GEMguDgMwghGEwgDCGIaDQIIch6GgxCAMQtDSJAiD4PAvgOBYoiuBoIgqDIVHYMonimLotgSL4JguDQyDCNI2iqOo5gWB4JG0NAuDmHZLiILocDGUJCjiAo6kcIhtDENwuDeIpcl6UpRlOKJDiyVpGjATYNgwaI1mWVYulia4Zm2b43kSaI7lkNguhSW5dC2YpPDSVJ5nKMJaDkLohkoOaClChKGiyAQA",
        categories = "accessibility",
        tags = "night,dark,light,moon,sun,brightness,theme,auto theme,system theme,appearance",
        contributors = "itsjavi,mittalyashu,karsa-mistmere"
    ))]
    SunMoon,
    #[cfg(feature = "sun_snow")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIA5GEMwghGDQxCCDYNDYIg+DwL4DgWG4egaCIKDIIAxDIaAxhqHIhiCBIigmCw0CAMgxFYM4rh2L4ugWB4xgwIA0jeOYtgKL4+gqQI1HYLYqhuOofkaPYjG0MwuDaMwxDgLgzDYLg3C2X5EjuUowCIbQ0lyM5eDMNJhmCYpPkWIZIjKJooDiY5RnSVAxDeQQthGOJymSfIJG0MZZn+gp6GiPJmG2NYmDWgaBhKgaND6AQ",
        categories = "weather",
        tags = "weather,air conditioning,temperature,hot,cold,seasons",
        contributors = "karsa-mistmere"
    ))]
    SunSnow,
    #[cfg(feature = "sun")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME2DggDIdoPhoL4hiOIIiiSJgiigMoqDCLIZhuMBojKI4lgwbQ0C4OQzCCRZHCAMQuDQMZMk4MY8i+M5AjSQwxDcLg2DaTJbl2UZPmKU4uj6V5CjaOIOGiLY9lYPI+mkTQyDCTAym2VJnnGM5pG0NguDMNJflwNgtk2Y6ImWb4xnyQY1G0MQ5C4MA3kmRgzoeUpknqVoBA",
        categories = "accessibility,weather,seasons",
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
    #[cfg(feature = "swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyDEVgzGgOAiD4PAvgOBYWhmBoIgoNggDENhoDmFYXhyG4Eh2CYLg0OQuDUaA3iaGIqD6AQ",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    SwissFranc,
    #[cfg(feature = "switch_camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDkSA0GEMgghQMAghcMQtDKGxWDeE4VhiIoNhwMhoDUIg+DwL4DgWKotgaCIKDEMwgDUaIfhSFojiEMh2DEMIgjuGYbhUaAtiiKosgQaIqGMaRyGMbBlCAYx4gkMQyCIIBygkM5bGMeZYlqSpPlGU4vkyB4JG0MQ4hWHAzC2NZyl+SowmmBZrCIbQ2iGdJzCCdorniAQ",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[cfg(feature = "sword")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQLg1CAMQ3hQIAzCANoah6HYbheGYShQIg+DwL4DgWB4JieLIKHkMoQDEMwiCAeAxjONQgHmOYRDmNh4jKP4miiL4ugiMJDDIMJBj4MQ2kGS5NjyT5RicL5HDyL43k+QJVhAMgxlKYZjjyQwxkCWJHgEA",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQLg1CAMQ3hQIAzCANoah6HYbheGYShQIg+DwL4DgWB4JieLIKHgMoQDEOQiCAeYyhEM42HgMYzjuN4+hGNYnC+L4ugiMI5DIMI2jiEJMk6QgxDaPJTlWRZHDyL5BlAMZWjONQgjGXpOjmNImiiWoqgaSYLG+DYPhGE4VDaIw4h4Mgxnme4diKFQ5iWRZslqXJPhEOJgCINY8jmYh5lMNJpkaSZIgmXYRDeiqamOOaSjeS5NlmlZbm6kJho2EKMqCZZjkKO6ji2AQ",
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
    #[cfg(feature = "table_properties")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzHYMQ4CIPg8C+A4FhQchlGMdAgHcaRkgWCYRCIIB4gkM4lHmKIlGgZRpGcaB0iOEggHKJwiDKE4VhqHIUheBoIgoMgxCAORIimFIWgQaI/kyB4JE2RAggySI7kuGIBA",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhQchlGMdAgHiCQzCIIBoGUaRnGgdIJhGIh5iCIhyh8IgyiIdxpGSBYqhKFAvhqHIUheBoIgoMwgDkaIrjuQI/gSQYJE2RAxDWR46hWSoBA",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQwgweYJDiDByggIgygwaBlGkZxoHSEw0CIPg8C+A4FikcBhhEIBkgkTQ1CANBhDIII6DAII9DGOwtDIaAxDKOY7j6SZAjoMh2DENpHjySpCjsaJCC6J4pC+L4Ri6MBojKNA4CAMQ4GgLgwDGKIqlwaA+gE",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = ""
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAch4g4MoSHIeYWhKGoPhKFQiDSEh3GkZB0GiDgxDYIg+DwL4DgWLRsGkbhlCAeQxikOIfDKKQyC4MAxh+OQiDGF43j2RY7i0L4zjUPoBA",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINwiCAcoMDWDxjHmDIOD4PAvgOBYHhmHIGgiFYMDGDoJgsIolg+EQihOGYbgSIYZHAYR0GgIBkgwTQxDIIIlGgMQwCKL40jaM41jeOQiG0MwuDQNo+DALg1DQIA3C4MA4C2V5ZkOGpFGgPoBA",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,pills,pharmacy",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Tablets,
    #[cfg(feature = "tag")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyEgMh2DEMBsDkLgyDkIIWhgYwuDkNIeDSDguDQOIhCAM4kg2FA2C4NQ4C2LYvh2HwtiGIQtDKJA4CAMAtikNAyEyDIOFoIg+DwL4DgWSJLgaCIKDcIA3GgLgwDGR5Jk4PoBA",
        categories = "account",
        tags = "label,badge,ticket,mark",
        contributors = "colebemis,csandman,aaofyi,ericfennis,karsa-mistmere"
    ))]
    Tag,
    #[cfg(feature = "tags")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CANRIDIdg3GwNguDKDIUhYYwuDkNIbDQIAyC4NA4h4IAziIMggDAbInDUOAti0OIahwLYeh4LYhiOKowigTIMDUWgiD4PAvgOBZDkaBoIgoNggDkLgwDEVg5kKRJJkiBJKgkbQxDWDQghQM5gC4MxhjmIIiiqagxmqJw0EwMQ3CAMZUkORZZD6AQ",
        categories = "account",
        tags = "labels,badges,tickets,marks,copy,multiple",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Tags,
    #[cfg(feature = "tally_1")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBQ+gEA",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,one,1,first,bar,prison,cell,sentence",
        contributors = ""
    ))]
    Tally1,
    #[cfg(feature = "tally_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBoD6AQ",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally2,
    #[cfg(feature = "tally_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZD6AQ",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally3,
    #[cfg(feature = "tally_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviMPoB",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally4,
    #[cfg(feature = "tally_5")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviOMYaigMgyCANggj8MQ4i6IoXgEA",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = ""
    ))]
    Tally5,
    #[cfg(feature = "tangent")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINAigkeIMDEN4PHKDAyCIPg8C+A4FgeGhwGEdBoCAZIME0MQ1C4NQ5CCKg0DGLgujAIIpisOYZhuIYjhqHYGgiCoShQIIWCKGIQgyDoahyBI/iCIokiYIooDIIAyDIcwtDQLQ5C0MYrl6Xw1FOV41lWZQxhiS47GgPoBA",
        categories = "shapes,maths,design,tools",
        tags = "tangential,shape,circle,geometry,trigonometry,bezier curve",
        contributors = ""
    ))]
    Tangent,
    #[cfg(feature = "target")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bh6BoIgqGYQhIIg2haGINhqHIiiCAoEiOCYLi2J4Mg+CYsg4Iobh2M4ggE",
        categories = "brands,gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[cfg(feature = "tent_tree")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINAiCAcoMDKDxjHmDIOD4PAvgOBYHhkcBhHQaAgGSDBtDENAgDUIAzC0M4siwIoZC+IIih+IYjiUIonikMQwiyLowDOMoajUaI3iKJIME0MQ3CCKBWhOM5GkiOZLk2Tw0EgNxsC2Kw4GgMgwFqRI0jiVZKCITQ4lkdg4mWVA8kaaRtDmWYqCCb5TmeAQA",
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
        contributors = "MoltenCoffee,ericfennis"
    ))]
    Tent,
    #[cfg(feature = "terminal_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQxCAMgthGEwiD4PAvgOBYWhmBoICITYOg0MxoDSFYXhyFhyGUYx0CAdxpGSBYJDEOAiCAch4gkMo2HmCQzjYaBlGkZxoHSM41jePQijsII5CKP4WC+KosD6AQ",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CAMQ3hMMITDEIISDUIg+DwL4DgWB4Jh6I4KHgMYQDEMgiCAeAyhAMgwi0eYpCIMQ5jSMI3jmHgviYPoBA",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "colebemis,ericfennis"
    ))]
    Terminal,
    #[cfg(feature = "test_tube_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3CANguDgMgggwLgxDgYQyhIM4VhwIAwiAIAxC0MwuDkOQtC4MAxHYMIah+Gw4h2IYhg0MAtDQTAxg8MwiD4PAvgOBZAkOBoICIbQxDaFYQhCP5BkaRYEkeCRNDGFJLEgNJQkKVA+gEA",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube2,
    #[cfg(feature = "test_tube")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMh2DEN4OGMMAgDELg0C2GQxhCDgtDKDofDUaAwGOHIaCAMIhiCHYtDWMBWDIIg+DwL4DgWNo5gaCIKDiIwyGgN41jePI7gSPYJguDYPDENhoC0NZFjiSQ+gE",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube,
    #[cfg(feature = "test_tubes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMh2DENwuDUQQyhKDYWDAIIZDEIA2hYMoODCFIfhiGgghwNInDmEhWDIIg+DwL4DgWMIzgaCIKDKGYOhCEhhhUNYXkGGYbC2QJCHYMI/iSQ4mDGRoSlANYti+MY2jWBI3gkTQzg0aA3lWMpZliBYHlsMYpDKX5hleApZmaCoMDENhIDSbJjm6ZY4E2OonDYaAtDWd40gEA",
        categories = "science",
        tags = "tubes,vials,phials,flasks,ampoules,ampules,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTubes,
    #[cfg(feature = "text_cursor_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANBoDEYQzCCEgwCCFQxhOGYShSFgghgMwtDODwiD4PAvgOBYliiBoIgoMYSDIMBoC2EIbh2F4hiGGo3h6OgzEgNYkiaK4qgSLIJguHg2EgNBhDIIJPhWOAyC0Mh2C2TZPlGHYYlQMojiWJ5GkWBYHkiLwgDgaA3k6UI8l2UB2lmbpSj2WozDeQpiimApGmaCg5CANx2DEMJ6kSAQA",
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
    #[cfg(feature = "text_quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIA2EgMwiD4PAvgOBYThaBoIgoMgxCAMQyEgOIShSGYYgSGoJE2HYfDiIokhWKIngWB4qDOHwyHYNowiaAQ",
        categories = "text",
        tags = "blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextQuote,
    #[cfg(feature = "text_select")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxhDIIIQDAIITDALYQDIIg+DwL4DgWG4egaCIKDEOYNg+EYUioMYphmG4dgQaIgjGB4JE0MosiWKISiuF4RhqHIhjOBY1gqDI4juKoTDGF4XkCMIfgKNIjE2JgzGgMZPkKUpElSJo4liWoxkOIo2DENINmGL5biGRRNmeEQxmqQZjlyZYKDMIA5HaWZrnWbZUjiep8mKUaAjaeZnoSfqGlONqCoqfZ0lEbBpG4ZQgHgMYJDcIggHkMoJDinh4qEIgxDWnh5psIqji+laXhusKYpqnKqqwMYZp+pq5qSvKdq+lhlrKwqZrwM6kqyna7gkMQ2rezbPsGsYBA",
        categories = "text,cursors",
        tags = "find,search,selection,dashed",
        contributors = "danielbayley"
    ))]
    TextSelect,
    #[cfg(feature = "text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIA2C4MRIDMIg+DwL4DgWFYYgaCIKDIMQgDEMoQhKFIWhuGoEhyCYLDWEIhDiJYVheKg+gE",
        categories = "text,files,cursors",
        tags = "find,search,data,txt,pdf,document",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Text,
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
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGgMQwCIPg8C+A4FhSF4GgiCg5CANB2DENoThWGoZgSG4JG0Mwgh6LAzC2L4khaKIngWB4JE2DggDaLY+j2I4UjSGICiiOAiG2PQxDgIIwiwMQuDWDZRjOJpFjeHILDCH4hDCUQ0GENIfCCWwxg0LZiDAVpggyDJbmWYwwFqVY1gEA",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,cold,freeze,freezing",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ThermometerSnowflake,
    #[cfg(feature = "thermometer_sun")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA5GENAghEMAghQMAtg0NwuDUIg+DwL4DgWHohgaCIKgwIAzHYModh+JIjgSJYJG0NguDYIAxDgLg0C0MY7jiO4tiCMYwgWB4JE0MoUDQdgxDCGw0hCEoVjiOAthMVpRg2DYWlWExakKL4CjGR4KhEMQzEiLIekOIpjkaJhNjUM4RhqdISC4OQzCANZ5DOYZEgEA",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,warm,hot",
        contributors = "ericfennis,karsa-mistmere"
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
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDQVgyCIPg8C+A4FhSF4GgiCg5g4OAuDEMoODCDg0EgNIhDcYYjiOJYlDELQxC4OQyC0MguDUNhsjgMwzC0OBBi0IIvg4IA2jkIAyEgMgwiySpElEMZQDIdg4k+LpSjeShojcLg3DaWJRi+Mpfh6MwxDETIikoMhok4M4hDMIJxDGc5FjGP5xDgOBahOFYaD6AQ",
        categories = "account,social",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[cfg(feature = "thumbs_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwHYMQyCIPg8C+A4FhSF4GgiCgxDUIA1C4OA4g0NINDAaIhDgMxhDIIIuDAIIxDGDQuDmLgyC4NQ2GwLY5DMMwgDgQY4jKRo0DEN46i8MhIDSLYvkaM4+j4dgtDiUIwkeL4+GiOQ3DaWZSkcLg3DkLQxC4MQxEyEYvGgMBhDOapBnMMZBjGMwgnWIg4FqE4VhoPoB",
        categories = "account,social",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[cfg(feature = "ticket")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoMYQDWGpFkeTJLgSTYJE2UIyDeU5GkiWJXgWB5alwMQxl+VZigEA",
        categories = "account,transportation",
        tags = "entry,pass,voucher,perforated,dashed",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Ticket,
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
        svg = "gAPBsGkbhlCAeQyD0IgyCKBwxgqDAgHiDwiDEMINHiCYVDQIg+DwL4DgWHohgaE4KDGEYZicNYNgiJwxi2FAxhyHoggQZYeGMaRyGMbIGGMeInhEcoKDiDRjHmJ40h+Oo8j4PoBA",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAdxpGQdBogkMgwgwaBlGkZxoHSCQxgsIB5gkNoMHKCAiiMIByiGJwiD4PAvgOBYuGMaRyGMbBlCAY4mDiJITgwY4rh6LYvjSNo4D6AQ",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0Ig2CIIB4goMoNgmC4NHKD4UCAdxpGQdBohAMINGgZRpGcaB0goMYRD4PAvgOBYrGMaRyGMbBlCAY4XDGDI3hOKYVhAIorC+MYzjUPoB",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleRight,
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
        svg = "gAPBlGwbBpHAcxlCAch4D0IgzCKCh5g0MoQGOEgiDEMYVgyGIUD4PAvgOBYHGWH4igaCAghaDQxDILg1hCC4sDCMYXDiL4biyHogieJA+gE",
        categories = "shapes,design,tools,food-beverage",
        tags = "donut,doughnut,ring,hollow,3d,fast food,junk food,snack,treat,sweet,sugar,dessert",
        contributors = ""
    ))]
    Torus,
    #[cfg(feature = "touchpad_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANBhDIIIQDAIITDALYQDIdgxDKD4RhSH4ThgaAxDYIg+DwL4DgWJ4qgaCIKhAMQ0iMMomiiLYsgSLoJE0MoxjOF42imOo5gWB48huEQwHYLYlieQ4rgKOpHCIbYYkqSpCjiUpGi+PYxDYVg2h2EogheFxIDEMJakSAQ",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TouchpadOff,
    #[cfg(feature = "touchpad")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAdxpGQdBogkMgwgwaBlGkZxoHSCQxDaDBygiCoMHmCQ0CIPg8C+A4FikcBhhEIBkgkTQyCAMQ0GiFIoiqL4Ri6MBojKNAxjaFB2C2H4pC+PhoD6AQ",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Touchpad,
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
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJhUIBoGUaRnGgdITDKDByggIgxCIPg8C+A4FikcBhhEIBkgkTQxDAIA4FYNRjDALQuDaPg0C0MZDkMSA2GEMQgkqN5NkOSx2guKQvi+EYujAaIyjQMQ5jiOo8j6QAukKRJlGgLQzkmSwgk2bJPDGUYoiqVRoD6AQ",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANBoDkbAxCANwiD4PAvgOBYWhmBoIgoNAgDEMRWDSFYXhyG4Eh2CRNDiIQwiSJoYiqKYFgeLAxi4NRjC0Lg2CAMAthILg0kKIR2DWPoyiiAoqjcIhtDEMIhhKIggC4ORjj6QJXDkLpJi4MQuDEbI9joaJCkuNA8GMaRyGMbBlCAYx4gmFJzHmCQxDUIggHKCZfjKbZvnGFqDnCchjnkIp7n2dJ2n2fwinyFgvoehZNjaHhNDGP45milInmul5ypIMqOnWjA4o6i45oKbqID6AQ",
        categories = "transportation,sustainability,food-beverage",
        tags = "farming,farmer,ranch,harvest,equipment,vehicle",
        contributors = "danielbayley"
    ))]
    Tractor,
    #[cfg(feature = "traffic_cone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4MwgDYLgyGENAuDUNQghSFggDCG4dDULg0hsIg+DwL4DgWJIngaCIKDcLg5CAMQwC4Nxji8Lg4CAMogjGDYYC4MY9DMcwzhELYVj8MQtDGDYjiWKopgSK4JE0MZFjCRQ1GGTA5g6XIOhyYQtkUOJHDEbJjjEMBjmaRpAiCbQ2h2TA3jqEQgg6F5DDWS4+nwMxjDCRwyhuRw0maRw1FqTomlKUYFgeCRti6FwxjsMgthSdY7jWiYNkcOI0qCPQxHODY4jipBsi6coZjaDIXjuTJ6huqwuDamYVjYN5Hl6R51kqTKlr2oLFsKaKaC2Ow4oyUIBA",
        categories = "transportation",
        tags = "roadworks,tarmac,safety,block",
        contributors = "danielbayley"
    ))]
    TrafficCone,
    #[cfg(feature = "train_front_tunnel")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgyFYMQyGEMQwCCFAghUMYWg2FQwHaFAiD4PAvgOBYiiWBoIgoMQ1CANguDiHwuDQYQzg2MIYhsMQtDaGBWi8OIhiOKIngSKYJE2F4sGgLgwDGQokkaRYFgeSAxDSFg1kyTpQkSApGlWK4ZDkYZYliHYWC0NJqHYLQzGGPY9hmOoMh6b5mjmGZqCANBal2UpflSKhtDmFg5C2DAzn+JqBkcIhtiyhoNCCioilGJoBA",
        categories = "transportation,maps",
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
        categories = "transportation,maps",
        tags = "railway,line",
        contributors = "danielbayley"
    ))]
    TrainTrack,
    #[cfg(feature = "tram_front")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ2gwch4gkMoMGgZRpGcaB0hOFQghgIg0CIPg8C+A4FikcBhhEIBkgkTQ0CAMQxGiFIoiqL4Ri6MBojKNAxDIIAzHYOI8C+PhokCMYzCIbQ4jcOQtkaC4pkyQZPkKURtDGVAyDKVwtlmPZcDyTZDCITZUDENRoDCS5Nl2bBNhSN5xnOWp1gE",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDEOAiD4PAvgOBYThaBoIgoMQ5g0dgxDQYwwCAMQtDEIAyC0MopEgNxjicIAwiuJ40DIVg2hKFIZhiBIagkTQ4g0VoijOKImiyKgyGiIooiSLIokkdgyjqFY+hMbBpG4ZQgHkMYJDEMQiCAeAymAMJjHiXwiDGaJdmabA3lWWZbliWpcHmcAxnKZJriGaZ6DSY5emCYoTC+dBlD6AQ",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash2,
    #[cfg(feature = "trash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDEOAiD4PAvgOBYThaBoIgoMQ5g0dgxDQYwwCAMQtDEIAyC0MopEgNxjicIAwiuJ40DIVg2hKFIZhiBIagkTQ4g0VoijOKImiyKgyGiIooiSLIokkdgyjqFY+D6AQA",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash,
    #[cfg(feature = "tree_deciduous")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ5GENAghEMAghQMQtDILgyDQLQ3C4MwyEEMwuDUIIjiWFIWCAOQgDYLgwDMVg2GEM4mhWDY4DaFR2i8NI0iSJpAimOIjhsIA1C4Ng1EGEYTjcMYNjqDhaCIPg8C+A4FlaWYGgiCgxDKDQ5HYM5VleXA+gEA",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TreeDeciduous,
    #[cfg(feature = "tree_pine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDcIAxDQIAzhELgzGEMYOCAMIZg4LQug0MYeEgNIehaGIahoMYdDcLYgDcTIfDQaIdhWF4XieHIeiyHhMDkIA5jILgyEGNYbigIA4CAN4UEwMQyhEbIQiONImhuKQukiLRIDENRshIM4UiWNpViqDohgwWgiD4PAvgOBZqm2BoICITZNCAMgyHYLQzmma5wD6AQ",
        categories = "nature",
        tags = "tree,pine,forest,park,nature",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    TreePine,
    #[cfg(feature = "trees")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMHYLgyEEMwghSDYNDEIA4C4OYODYdgwEgNYgGgMBhhSFgghgLQxC0NQuDgVoMieFYqjaGQ2ioWgiD4PAvgOBY9kCBoIgoN4eHYNo8j6Q5CgSRIJguFAxDkdgzkuP5Pk6BYHlEMQyg4ORohsMxhhmGYXjYLg3iyaxMDEOIODQaAumWZ42mma5tDeb45mKEZmg6eJqDie5vigbJtDSDguDWWJNgEA",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere"
    ))]
    Trees,
    #[cfg(feature = "trello")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwcoOhSDBoGUaRnGgdIJDEOIMHcaRkHQaIhiMPg8C+A4FiyL4GhMN4kiaKIPgyGI1CCG4dh+CQ5CKLIugQdIxkYIIlieKYKgyEwxDSGoch6IAiDWOoJjWRIyD6AQ",
        categories = "account,brands,development",
        tags = "logo,brand",
        contributors = "bdbch,csandman,mittalyashu,ericfennis"
    ))]
    Trello,
    #[cfg(feature = "trending_down")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgyDIIAxDeFAzC4NQgDiGYbh0MYYhqEw3CIPg8C+A4FgeCYmimBoIgqDBug6EAxDaFIWhKOAgjoMQxiWJ4uisZQ+gEA",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingDown,
    #[cfg(feature = "trending_up")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgyDIIA3CAMQzC4NYWDWGQgDiHQxDCHYTDENwiD4PAvgOBYHgmKIrgaCIKgwboOhAMQ2hQIISjqPIXieKYwi0ZQ+gEA",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingUp,
    #[cfg(feature = "triangle_right")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgYYNg0MAghQMQthISAzGMLQxC4MYVh0LgzC0Lg2iUNIiDMTAyDALg0CANIjGMLg5iUN4OiaKI5DYLg3FoIg+DwL4DgUPoB",
        categories = "shapes,maths",
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
        svg = "gAPBwGEdBoCAZA9CITQ2CAORIDQLg1GEMoRCCFA1CAMIZCAMYZC0NRIDYIg+DwL4DgWJIngaCIKDEOINGgMYRhOFYXhuGo4h8SIuiOJYqimBIrgkTQ0hYMoxiKJImkGQIFgeQwxhoMYQDYNhWDENxjhqEQ1C0Lg0DcLg5DiXg5DeHAuDIMRDmEOIYi4Lg3hiZwyDCaZFnQMo9kuKICkGT4tkWUwulWV5ZlsNQ1l+YZjmKZ4ymoQwxDYLgxm8OJxm+dJ2DKgp5nuP5+k6LBNi6FohHYNxhgyDI4hsMQyhkVgyFqoJMgE",
        categories = "sports,gaming",
        tags = "prize,sports,winner,achievement,award",
        contributors = "karsa-mistmere"
    ))]
    Trophy,
    #[cfg(feature = "truck")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ4EgMxjC0Lg2CAMAtDGEw0hiGBWDcY4XhQLobDGDYYGgMQwGOFIWg0IIji4MR2DEMQiD4PAvgOBY3jqBoIgoMQ0CAORoDQbJCDQdg0iCLw2hqJolDEaAtDKNo4j2NxjGkchjGwZQgHKCZVCAYx4gkNwimQeYJg6VgvlqXJejyBI+gkTQxgyDhIDmbpYDycJdl+ZZsmiapsDiaZhCKVY3m+W6BD6AQ",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley"
    ))]
    Truck,
    #[cfg(feature = "turtle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDAIINDQdgzGEMYOCCEIahgMRoDKFoYhuEAxC0MR2C2FQ4CCKojhmJQ2hmFIgheIoch6M4ZjmI4licMxsDILQ0GgNBaCIPg8C+A4FkeSoGggIhNDQLg4g0NwuDmK4ODCRpIk2TIEk6CRNDENQuDGKpWliDJalySZgl+BYHmIMQ2lcM5aEgMgwGGDYNhuNAgDSeZtl6AQ",
        categories = "animals",
        tags = "animal,pet,tortoise,slow,speed",
        contributors = ""
    ))]
    Turtle,
    #[cfg(feature = "tv_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgxGgMQwCIPg8C+A4FhQchlGMdAgHmCQzCIIB3GkZIFgkMoSCAaBlGkZxoHSCQxDSIh4iiIhyjYIgyhOFYahwPoBA",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAcoICIMoMHiCYRCAdxpGQdBohQMIMHKE4QgwaBlGkZxoHSCQxDUIg+DwL4DgWLRwG8bB5GwaRuGUIIzjgdBzikNwgDIIAxkOQZBhGLQvjONY3jkPoBA",
        categories = "devices,multimedia,communication",
        tags = "television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,entertainment,showtime,channels,terrestrial,satellite,cable,broadcast,live,frequency,tune,scan,aerial,receiver,transmission,signal,connection,connectivity",
        contributors = "colebemis,ericfennis"
    ))]
    Tv,
    #[cfg(feature = "twitch")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAyEgMx2DENhoDUdg0GwNAtDSFYZhsVgyHobQtDEMAgDkVg3G0NQgDSKQiD4PAvgOBQ+gE",
        categories = "brands,social,account,gaming",
        tags = "logo,social",
        contributors = "ahtohbi4,johnletey"
    ))]
    Twitch,
    #[cfg(feature = "twitter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA0HMLQuDcIAyC4MQtg0MwuDQYwxC4NggDEMAtDmG4hDcLgzC0MQ4iGHoghWFQxg6G4RiANoYEMM4hDULo9DUIIliCOw1GOMYUh8II9iANIWkGDoRDkLZNg0NAtDaSQ3C2Goth6MwwCCKoehmKwuDIegiD4PAvgOBQ+gE",
        categories = "brands,social,account",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Twitter,
    #[cfg(feature = "type")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CANwghKEgyDCFQghiEwiD4PAvgOBYHgmH4kgoeQxhCGAiCAeQyisMItHiMAiDENYzioIg5h6IIniaCIKHiOgxDKLYphANJHjWLAgjSEJFj0L4/gEA",
        categories = "text",
        tags = "text,font,typography",
        contributors = "colebemis,ericfennis"
    ))]
    Type,
    #[cfg(feature = "umbrella_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQiD4PAvgOBYThaBoIgoMQ1C4NYODEYQxC4OIgiSJggDCKggDELQzh8LQxHYLQ4EgMojisMYrjyLQgjANAyDgLQ2h8Nw1hKFIZhiBIagmCw3h+LQyjcMhBjuLY9j0OQuDAMA0j8Lg0DWSIThWTZMgWB4JG2DZuisMgwkmZ4XgEA",
        categories = "weather",
        tags = "rain,weather,uncovered,uninsured,antivirus,unprotected,risky",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    UmbrellaOff,
    #[cfg(feature = "umbrella")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDIYQxDALgwDaDoThWDggDCGwtDKHAwFoIg+DwL4DgWJIngaCIKg+DgyHYOBhg2DYghsIA0huI4liqKYEiuCRNi6MAxjuJo/D6AQ",
        categories = "weather",
        tags = "rain,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Umbrella,
    #[cfg(feature = "underline")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CANB2DYYYMgwMAghSFAxDKFRWDQIg+DwL4DgWHhsGkbhlCAeAxgmHIoDKCQyDAIggHmKgijCMh5i6Noxh4L4kiYPoBA",
        categories = "text",
        tags = "text,format",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Underline,
    #[cfg(feature = "undo_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ0CCDw5GwNQtDUIg+DwL4DgWGIbgaCIKhEaAxDALg1GENYmCCKQ1CAMIug2K4qiwdgwiiM4qi+LwxhWOA1EgMQxheGYeD6AQ",
        categories = "text,arrows",
        tags = "redo,rerun,history,back,return,reverse,revert,direction,u-turn",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Undo2,
    #[cfg(feature = "undo_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDEN4OGMeISDIIg+DwL4DgWB4bHAYR0GgIBkgsTQzCANx2DYaA2hqHIiiSIYjiWJwiE0MgxCCExhDkIJADAIJDDALQ5keQZKkWRAtDYIAyC4MxMioMQzjEL4zGgPoBA",
        categories = "text,arrows",
        tags = "redo,history,step,back",
        contributors = "danielbayley,karsa-mistmere"
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
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldHorizontal,
    #[cfg(feature = "unfold_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDYIg+DwL4DgWFYYgaCIKgwIA4FYMoUhaG4agSHIJE0NAggwSIjhWF4oieBYHioMQwi0MhIDiJIyhmAoojaHg2joaAtjCJYzkGNYdE2D5GkiPomkyKQiG0MQ1i0OQtDMIAzl2XZTkuG5DliWg1mGYJeDOY4ZgE",
        categories = "arrows,layout",
        tags = "arrow,expand,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldVertical,
    #[cfg(feature = "ungroup")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDaEB4g0NYQHmDQ0hAcobCIMQiD4PAvgOBYoiuBokDGJggh+JYhgeCYLg2D4RhOFYXCKGQgiODYmiiKoEHQPoBA",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,separate",
        contributors = "danielbayley"
    ))]
    Ungroup,
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
        svg = "gAPBwGEdBoCAZA9CIbQxDgLg4DQIAxDILgyDWEQuDcMgtDGGAxGgLQuDAMhhDWIQwhCJQwicIAwiyLIghILQ3iENwgikMA2jaJo5i2PQtDYLg5hYMBshuGAyhcNwxCIPg8C+A4Fk2UIGgiColDGNQxhwNw1kaSpJDGJImiiY4uj0IAuhIIIzDCNY3jmb5mi6QJCiwbJbDGXpLk2T4EGiTRsGkbhlCAeQygkNQioUMYJDKih4owIg4o+h6SkyTqBoOgKCoQeaRpMIB4pWiahpGjqFpWk58pkZaboOoaVDENqKp6CQxDmj6RrKtKVDKjqrpyrqEpCtq4rCjanoatqzouy6XC+rA+gE",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unlock_keyhole")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ2CKCR4gwMQyhAcoTCIPg8C+A4FgeGhyGUYx0CAdxpGQdBohMOIQhIIgzhAaBlGkZxoHSE4VCCC4NDCFouhWGgviGI4aHAYYpCAZIME0NwgDEMBWDcYQ1CCVAwCCVwxCAOQuDMMwtDILg1hmG5GikPoBA",
        categories = "security",
        tags = "security",
        contributors = ""
    ))]
    UnlockKeyhole,
    #[cfg(feature = "unlock")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIBoGUaRnGgdIKDEMYNHiCgzg2CQihaDRyhmC4NHcaRkHQaIVDgIg+DwL4DgWLRwGGKAgGSChNDcIIWFYNxhDUIJADAIJDDEIA5C4OQtheLQvjOKA+gEA",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Unlock,
    #[cfg(feature = "unplug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIA1CAMwtDMIg+DwL4DgWFYYgaCIKDIIAyh+EYThWF4EGiGongeCRNDYLgziAMIvGEMguDSII2CAMI6jwM45DATAxh8MQ4GwLQ2kcLY1jCS40jmNY3juUo9jYWoUhaG4pgWKwiE0Nwug8MY+mGOwxDGV4mhmAoqh0TQxjKYYumGMAxDSaJZmuW4dguHw2CCfp+kuSozlCOJRjyUoSjaRo1kijZOjehZTDCipRlaJZZgEA",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet,disconnect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Unplug,
    #[cfg(feature = "upload_cloud")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ0C4OA5DkQQ3CCFQwg2GQxDULg3DEIA4GgMYdDkYYPDUIInCCGIYh8MguigOAuDINAyCIPg8C+A4FjiO4GgiCgxDKDQyHYOY3jmPo9gSP4JG0MQ2g0NgtDSVJUimSI6kwPoBA",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UploadCloud,
    #[cfg(feature = "upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUdg0GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGx2C0NAiD4PAvgOBYrHAbxsHkbBpG4ZQgjGNh0HOCQxDcIA4g6FAzCCQA4iqLIxjONY3iuTY4HkMo+DUIggHgMY+DKVh4lMIgxlsIB5lkIgzkkL5QD6AQ",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[cfg(feature = "usb")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDeDhjHiCwxDAIg+DwL4DgWB4bh6BoIhCCwyhmD4WCINIOgqDIahyIogDwcBhHQaAgGSCxNDQLg3CAMQ5C4M5ADkIA1jAL41jeG5LjiOgiG0MgxCAMwtkSVAyCAMhakmTpNjaT47kIMg2CANwuDYOJHkAMhslqSIbkqYZgjeOYLG2GJADSbJaDMLg1legJenSNJhneUQxmsMZaDELZUpCj5Al2cpfgE",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeIJDmOxjHmCQ3iqLIxjONYrHAbxsHkbBpG6NpMlEdBzgmDAgDEMZaDiWgzg4MggkCK4tk2T5RGUPoB",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ1CKCR4gwMQ4hAcoMDMIg+DwL4DgWB4bh6BoIgqDA3hAY4SCIOYWgwNIahyIogDwcBhHQaAgGSDBNDEMAgg4SA2GENAgkSPpHC2RA0HYMowC+NY3huUI4joIhtDIMQuDePw2C4NAtC4OZghmG5PjYaJSmeOYMG2DguDKPwzmGYJiC6ZIclOaY3muVgxl0No/DiWp2nSTp5jSapVm0OQuDGPwym+hJhoaZ56lSbAxoygIUlqYJfDGlJRoie6Kn4Lg4o+kqfqGaKjpefQ0naXAupunqspafJXDCWpxqeP62mWeYBA",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwiCAcoJDSPBjHiCQ5iqLIxjONYrGwaRujYeAygmDI8HkMZSDGVJRCIMZYCAeJWCIMopisL5Mk4PoBA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx4gkOQijceYJDePBygkNIqiyMYzjWKxsGkbo2HgMoJDGOwgHkMYJDiPB4lYIpSjweZQlyRIrC+S5NkqTI2lWUQxl6YAxmwIJPlENpZlsMopmOZRlD6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMAuDQMw0C0NwuDYMgiD4PAvgOBYlGMaRyGMbBlCAcoJDUIggGMeIJhmNRjHmCQ4iSJori2L4qiyLowjyOY/jaOAiDGS4yCIM5AC+QpHiWKIGggIhtDEOQuDWGA0C4MwthsLg5lSWZYgSWoJl0Npog2Gg4maY5piWJ5tmyBYHm+DguDeGJfDWZg5naap7gKbZ+lwMQ1C6DAxnGh5ooieZroufZbG2gKCpOYKGhuiYppqbqOpCkpfDGog0qQaJ8qeXaEg0MaBnahqvrGjZwnKj6RriaK6gE",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserRoundCog,
    #[cfg(feature = "user_round_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCKLh5gkOI0HKCQ1iGI4oiqLIiiWBoIgoMoMDEORoh+PYkgQaA+gEA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundMinus,
    #[cfg(feature = "user_round_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCKLh5gkOI0HKCQ1iGI4oiqLIiiWBoIgoMQ5hgNh2iCIokgQaJCk+B4JguDJHGiH49k6JoBA",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundPlus,
    #[cfg(feature = "user_round_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMQuDgNwzC0NwiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCKLh5gkOI0HKCQ1iGI4oiqLIiiWBoICIbQxDeGJJDUII8iKJIEGiQpRgeCRtDKDJIC2TJOiOQw+gEA",
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
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeIJDmOxjHmCQ3iqLIxjONYrGwaRujYeQxgkOI7HgMoJDKKQgHmVgiDEM5UlGXZFisL5Mk6S5NjaVYJDGRZalyXo7lCUpgleKZkmYZQ+gE",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgOYUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeIJDGKY8HmCQ3iqLIxjONQ+gE",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYYNg0MAghQMAtDENoVCIPg8C+A4Fh0YxpHIYxsGUIBjHmCQ4CKKR4gkMQwi4coJDWHIeiOJYnh2IIGgiCgyDKDgwGOFwzC4Mw3C0MgtDYLg1C0NAtDgYQ1CCV4WhULQuDSUQ4kmOIfgQaA+gE",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    UsersRound,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx4gkOQijceYJDePBygkNIqiyMYzjWK4ugaCIKDIMoOhCEohheJAtDOVwuDiQIri2BBokqX4HgmC4NDMLgxDOVIkCAMYjDcLg3DWRZei+AQ",
        categories = "account",
        tags = "group,people",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Users,
    #[cfg(feature = "utensils_crossed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIAyC0MguDODoSGGE4TDAIIZhsIA0C4MhsDELg4CCIg4hYIIYhqK4eDKGhMDKLg4CIPg8C+A4FjWOIGggIhNDENYlkEM4SimFYth2H4rhyGQ2GwN5FlAMxjC6UA3g4IJVhSJAwEyQJCFobYclcN40jaO46gSPIJG2EQxg6JggDYLg0C2cwzmaN5qmmBYHmwMQ5CANQtmSeZogE",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere"
    ))]
    UtensilsCrossed,
    #[cfg(feature = "utensils")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DcYwwCAMQuhQOYNhgMhoDQYQyhiEogg0LQyFYMgiD4PAvgOBYoiuBoIgoN4NHYMgwieKYui2BIvgkTQyDGEw1iUdgwGENQgkeIQwC2Rw1HYNoRhOFQuheHpWGgMxaG2IIPjeKo7D6AQ",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Utensils,
    #[cfg(feature = "utility_pole")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMgwCIPg8C+A4FhSF4GgiCoNDUaIRhOFYahmBIbgkTQzCAM4QiKFomiWBYHigN4ri2FIvhiAomjOCgxjWLAyi6JI7jKHBNDIMY2kKOJEhqPRtDEOQgDULY1DeVpWkOMIBA",
        categories = "buildings,home,sustainability",
        tags = "electricity,energy,transmission line,telegraph pole,power lines",
        contributors = ""
    ))]
    UtilityPole,
    #[cfg(feature = "variable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxHMLQ0C0M4RC0OQghKF4ZCIPg8C+A4Fh2IIGgiCgxDYIAzHMNIphgIA5hGL4xDmHIeiOHRsGkbhlCAeAygmNAgHmPwiDENQikIMZAkgeJKkWR4dC+OY7jiOo8k2S49kSRpIkOCZckmS5RlMZQ+gE",
        categories = "development,maths",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[cfg(feature = "vegan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMhhDINguDaDYRhMMAghcMQgDGFwyDAYwuDkLYRDiDAxC4NQtDmKAgDQLQxDQIg+DwL4DgWM42gaCIKDGEw4GMNIYCANgtgyRJEkEMIjg2Sw2jKNI5jiBI6gkTQxDcLg0hoM4SGGHIbheGZCDMIAzk+NZTD6AQ",
        categories = "food-beverage,sustainability",
        tags = "vegetarian,fruitarian,herbivorous,animal rights,diet",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Vegan,
    #[cfg(feature = "venetian_mask")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyGENQghEMAghSFIRhEOAghqFoNhIIIMhqHIVh4NQtgyGIkh2Jg1FYNxoC0NRhiKKoVjGIIbjmHQwjGJxIDIWgiD4PAvgOBZDkaBoIgoNoNDEYwxC6EwgDMIJSlSIInjaVY8DOJ5BkORYEGiSJjgeCRNDGGgxk8LZRhMLZVlKcY4gyFJclSX5CkSSQ+gE",
        categories = "account,gaming",
        tags = "mask,masquerade,impersonate,secret,incognito",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    VenetianMask,
    #[cfg(feature = "vibrate_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiHotgeCRNg4OB2DEMBjDAIAuDUNQuDQNQgDGQpCGgNhjjyQY6DELY/kGTAxHaEoYhqH4vgWMQiE0MQ2kIMAuDMNBWkcMJNj2TZAC2UJqGgLQ0mANJUiyHQ8GwaRuGUIB5DKCYjCIIB4DGfZ/HifAin6eqCoecp2ngPoB",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiGRyGUYx0CAaBlGkZxoHSCQxDQIggHiCQ4j4eYJDWPh3GkZIFkGPhykAIgxhiGowjIPoBA",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DYIA2EgMQ0GEMgghcMAghoMYYhgdgyC4Mw0GyHQxEwMoXDgdg4CIPg8C+A4Fi+MoGgiCgxhCOYWh6GocC2FwyEiFZBhuRgxkCQBWDiPIZkeGJAGgMolhwMBai6MI1i8bBpG4ZQgHkMoJikIggHgMZjmUeZoCIMplHiYptm6LwvlyXg+gEA",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIA4C0NggDQIIRDQVg4FoIg+DwL4DgWGxyGUYx0CAeIJDIIggHcaRkgWCQxDSKR5gkNopHKJgiigIBoGUaRnGgdIvjocozjmGociGIw+gE",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAdxpGQdBog4MgwhIeIZhIeYODSEhyh0IgyCIPg8C+A4FikcBhhcIBkg4TQyCAOBohqKIqi+F4pGMaRyGMbBlCAcoeCAY4lDiEhjiCD4iikL5AkKRIujAaIyjQOAgDEMhokyUo9GiP5BkORZKg6EJNk8MYikaHpSlSZw+gEA",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = ""
    ))]
    Videotape,
    #[cfg(feature = "view")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQyHMMguDUNA1C2DA3hYYw0C6FA0CAMAgDcIIXiMcwthGEw2iMLYiDUYwthuFIMDCLIWjUNR6CIPg8C+A4FjuPoGgiCoOg0MxhDGDYfkqII0DKSpJk2S4gDKOY7j2BBokCWYHgkTQykkMQ3HYMhhk+T5SDGJwgDISA1mabJTg2J4nHaJ46jyQZbgWXYKmCIRWm+Z5yk6J5unCaJTmsMpkniWI/gEA",
        categories = "design,photography",
        tags = "eye,look",
        contributors = "zenoamaro,ericfennis,csandman,karsa-mistmere"
    ))]
    View,
    #[cfg(feature = "voicemail")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINgiCAcoMDSDxjHmDAxDIIg+DwL4DgWB4bh6BoIhEIoTgmCwiDEOIUhaKoZhuHYEiOGxsGkboIHgMYMg4IB5juKo9HkMoXj0eJEiqLIxjaOA+gE",
        categories = "connectivity,devices,social",
        tags = "phone,cassette,tape,reel,recording,audio",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Voicemail,
    #[cfg(feature = "volume_1")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIA1CANggDkIAyhiGggDGFYXh+HoTDGGYShQIg+DwL4DgWBxuiocBhHQaAgGSEBNh8Lg1DQIA4C4NA2GGFYVDAIJGhORg3C4MA3imK4yjQPoB",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Volume1,
    #[cfg(feature = "volume_2")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIA1CANggDkIAyhiGggDGFYXh+HoTDGGYShQIg+DwL4DgWBxuiocBhHQaAgGSEBNh8Lg1DQIA4C4NA2GGFYVDAIJGhORg3C4MA3imK4yjSMYzjWNwijkOZMDcIA0C4OQzGEMZIkaZIekeHpdDENJPiyVA+gE",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Volume2,
    #[cfg(feature = "volume_x")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIA1CANggDkIAyhiGggDGFYXh+HoTDGGYShQIg+DwL4DgWBxuiobIMGUIB5DKEIfCKNQxhAOY6HiN4RDaP48CIMgyimK4yG4ZYxjONZBjmO49kSOJDCCQIQkeSQvkuTYB",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    VolumeX,
    #[cfg(feature = "volume")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIA1CANggDkIAyhiGggDGFYXh+HoTDGGYShQIg+DwL4DgWBxuD6AQ",
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
    #[cfg(feature = "wallet_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDQaAuDAMQiD4PAvgOBYWhmBoIgqDQ3GgMQyGEMggiYMAgikMYnicdgxDCJYtimKwtiYMhIDWMooiqDo2jYVo6jePYrieNoiDSFYXhwPoBA",
        categories = "account,money",
        tags = "finance,pocket",
        contributors = "danielbayley"
    ))]
    Wallet2,
    #[cfg(feature = "wallet_cards")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDB3GkZB0GiFIWCAeIJgsIByiAIgyCIPg8C+A4FikcBhhsIBkgkTQzCAORhDIII6DAII9DGOwtDIaAxDSOY7j6SZAjqJ4pC+L4bi6MBojKNI2DEMRoDMYwuDiSguDYLo2DILgxC4ORsmaZg5GOZg2CCbggDSZZwmAIA1C4N4+mmZQtmeXA1n4NZ1DOfg5jufZnEgMgxiiKpQGgPoBA",
        categories = "account,money",
        tags = "money,finance,pocket,credit,purchase,payment,shopping,retail,consumer,cc",
        contributors = "danielbayley"
    ))]
    WalletCards,
    #[cfg(feature = "wallet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIVg3EgNRhDIIIWDAIIZg0MAtDQaAxDQdg0CIPg8C+A4FiaKYGgiCgzCANR2iGFYXhqN4ZhYMogDYdgtDWJYniyK4Ei2CRNDEOIODKNYYjiN4fiKHhakGKJFD6AQA",
        categories = "account,money",
        tags = "money,finance,pocket",
        contributors = "mittalyashu,ahtohbi4,ericfennis"
    ))]
    Wallet,
    #[cfg(feature = "wallpaper")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIOAiCAcoMDKDxjHmDA5CIPg8C+A4FgeGhwGEdBoCAZIMG0OQgDENwgDYLgxC2LgxGEMggjUMAgjgMY2C4OAxC4MAxEwMo1DENRWDWNI2jmTAwC0MpPEgNJKjeTZPjYdgxDCVJNkyNQyGgMQ2lyOJljaT4ZhuIYjiCIokiYIhNDiNgxGiDoaC+axom2I4lgwTQxkUNx2DSaZ5m4PoBA",
        categories = "account,devices",
        tags = "cover,lock screen",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wallpaper,
    #[cfg(feature = "wand_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDELg2DQIAzg4NAtg0Mg4hULoXGGFgxCCHQgDCIYhhkNwyiETAyC4Mw2h8OIThyGoeiCIo1iODYmGyFg4h+Gg4jGJ4WiONo4icMIpg2DwgDWKw2kCPZGkOJJFFoIg+DwL4DgWV5agaCIKDGEA3hGEZWliXZcgSXoJE0NQgDYdg0maWZqmmBYHmwMQ5h8NJxnOaICmqeAiE0MYiDIdgyn+daBneXxNmMOBIDOi5bo2a6EgyHw2GgLZyledKWl2g6Fh4MxIDmlRoD6AQA",
        categories = "design,gaming,cursors,photography",
        tags = "magic,wizard",
        contributors = "karsa-mistmere"
    ))]
    Wand2,
    #[cfg(feature = "wand")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA0FYMgiD4PAvgOBYThaBoIgqDAgDENh2C2EYThWBBohiJoHgkTQ4CAORoiOFIZiiBYqgoMgwi6MISjKJo0hqKwxDcLotDEMZEh4OYeDOPIlheAophuC4Ni8MJNjOUI1lKQpIDYLgykkIA1lePpZkAIhtDMIAyDGLgtDmZJPhmNoLDKXwgl6YJGmKcYngE",
        categories = "design,gaming,cursors,photography",
        tags = "magic,selection",
        contributors = "mittalyashu,ericfennis"
    ))]
    Wand,
    #[cfg(feature = "warehouse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIA4C4Mw1FYMgwGGDYNDAIIZDELYXEgNIWCCGIaCCHAyh0VoPhEQYXiSGwgDMLgyDYIA2C4NRsDgLYxDKIYji8MQuDQOIajmMIyiyIouiWIoNioNRaCIPg8C+A4FlOVoGgiCo0DEOBoDEMpSlSWZYgSWoJE2XQ0mCYpTlWZ5THIZRjHQIBoGUaRnGgdIJmEIggHcaRkgWfpiCAeIJDagB5n4MJjC+c51D6AQA",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "watch")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKDA2CIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIAxDCL4ug6LwzjWGYbiGI4lieIBhHQaAgGSDBtDENguDGNg3C4Ng2C0Lg4DELQ0C4MA1GGLoujGWwtDILQxkwMRol2TA4lgIJaCCXIzmEbJPDcOAglSVo4C8cI/GiPpAkKRJLDicZGC4M5HnENJnmmW5oi+YRoDILg3DKh5qpOMZemANgxGyUJSnMNZ1neQA+gE",
        categories = "time",
        tags = "clock,time",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Watch,
    #[cfg(feature = "waves")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhjC4NguDUIAxC6DAxCAMoShQQw3CCHoehMOYbDUY4aDYIAwhkLg0hkIITgyGoTiqMgtgwNY2hQLgzimOojjCGwxCIPg8C+A4FkSR4GgiCoXDKD4RhOFYXiuUpVj2NY3jmJ5YiyLowleNISjmOIXjuPYVj+V5CkSRoEGiSZvgeCYLhQOJQkGFoUmuYZVmSW4Ql2LY3i6MpdmWL45hWPIqmmG6GmyRZKD6AQA",
        categories = "weather,maps,multimedia",
        tags = "water,sea,sound,hertz,wavelength,vibrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Waves,
    #[cfg(feature = "waypoints")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINAuDUIggHKDAyg+ERjHiDAxDIIg+DwL4DgWB4eHAYR0GgIBkgwbQxDALgyCANguDMLQzC4OQgjYOYdh+JYnh6IYGgiGIMg6EIShSFoJgsIobjyIIEkKJImiiKgiE0NwghsaItk+PhokCUYHkuGockgIoVkeRJNDmFoelCIhllOJ4pisMY2DiWg3C6WY6jWN5elSYZxmeaYXhmTZmgqGpthCb5BiOAQ",
        categories = "security,account,maps,navigation,development,social",
        tags = "indirection,vpn,virtual private network,proxy,connections,bounce,reroute,path,journey,planner,stops,stations,shared,spread,viral",
        contributors = "danielbayley"
    ))]
    Waypoints,
    #[cfg(feature = "webcam")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQwCIIBygwOIQGMeIMDEMgiD4PAvgOBYHhyH4GgiCoYg+EYMDOFYXg2Goch6BIkhwcBhHQaAgGSDBNDcIAyDIaIOhuHY1jeNI2jiOgiE2GY+DIdgtDSQwvkUaA+gE",
        categories = "connectivity,devices,communication",
        tags = "camera,security",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Webcam,
    #[cfg(feature = "webhook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDYLg5DgaAtDWEQ5GMLQxC4MQgDCGoRhYOQ0C0MguDSDYbDkQQ0CCLQwh6DggDKDg3GMLgwDELQuDcLgyiANAuDUN4lCIPg8C+A4FkeSoGggIhtDaNQgDOHAzhWPA4jcNZXhGPY6iaDI7DULZVDEYYti+ModhAOA5C2QQwDaRpIk2TIEk6CRtDGNJSmYMwghYNwzEMMYWDaUp8jyDoQDmDqAgyj5oi6MYwh2MA4nSSZ4D6AQ",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere"
    ))]
    Webhook,
    #[cfg(feature = "weight")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDWDhjHiCwxDIIg+DwL4DgWB4bHAYR0GgIBkgsTQ2C4NQgDgYQyCCMAwCCMwwC0MQuDkMIsjgNA2EwMguDEIAxDiKxBjCMo0ksNIxDEaAxDaL4xkuNZEjkMg1C2QQ1DQTAxDkLpNmENZIlSVozDEN5iDiLRahqHIiiQPoBA",
        categories = "maths",
        tags = "mass,heavy,lead,metal,measure,geometry,scales,balance",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Weight,
    #[cfg(feature = "wheat_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMoMDEMAthAIg+DwL4DgWFYYgaCIKDENggDiEguDENwgDGJA3hSFobhqBIcgkTQzC4NImDEMguDUMwgDWJwxGyKI6ieOQzGGM49kcIAwkqJ5MDQLg5DQTI9DEORsiOOpYkWSZJkuSwxkoLZPlEWorheL4ugWB4JG0OIhC2RJEkaOQgl2TJeCCY5SDmJw1kCRJaGOOQ1nANQ1C4OIiigMqHDmipQiqFZnhmAovmsIhNhCUJgocMg2oIMaEC6nguDMNKFkENpwDcMxMDGOwzn+QpBludJ2l8IKjm6Tw2DKZotpWaodE0MpLDIaAyHYMhhDSeZ3ieYp5GgLQyFYNrMs6eJgqcNJlpKwIbpemYojWJw3jSNo7lWV60kOOpzkidLamKULNDCU4nlatJavCdbyky271kq3osmiwYwh6IIfoWR61vGPbznqShMnyoayDO/IzDSfMany2pDDkN5wo6v8GuGwwxDgLg3s2rwuDAOaCp6cKhjkMZwqbK8YjkNhMDKYAxj+WsZra/5evSvZwDIOMlpQbBpG4ZQgHkMYJr4IB4DLVtXHnWgig4ItY1XX5m0/UQ+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CINwigkeYMDEMoPHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCAOR2DaGYbiGI4ah2BoIGOEQihOFYXg+CoSg6GocgSNYgiKJImCKKA0CANx2DiLwvjEaJFiOJYnDIMggDGTQxGOKguDULZgloLQxloSAzGOYg1CAMJlmubwxHaZZQlIPoB",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIoHDGCoNCAeIQguDh4gmC4ND4PAvgOBYcHAYR0GgIBkgoTQ4C4NQgDENorGGLIsDAII0DEIA3jUIocC+IokiGI4licIhNDIIIqDgMhhDGLJMjWT43DQLgxDcLQyC4Ng1juHY+GiQIkiaKAxDCWA2CANRjlIMAxC0LgzmaKgxlIOYtDGbg0CAMwuDcNpbj2QZfkKYovDiTZ2DINZLjaNKMi0IJXgyLZYDifpdoGYZEk0M6Ki2jY2mcLgyDSVp7n2PKWgKBIGHmFQyDCDoIhGr4ThUMYShiCq2C6a5+h8ZQ+gEA",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[cfg(feature = "wifi")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQzGEMQwg2EoUg2DQ0CAMAiD4PAvgOBYch+BoIgoOAugwMQ2icYYMgyFQxCAN4ZhuHYiiGBIjgkTQyCCJg4DKEIoi6GYWDKFI0h6OIcGwaRuGUIB5DKCZGCIIB4DGCQxDKVR4lIIpaC4MAxlUeZYCKVIcC+TJOD6AQ",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis"
    ))]
    Wifi,
    #[cfg(feature = "wind")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg3CCDQ3GEMguDUIIUhYMAgDGG4bC4OAgDQLgzEgMgiD4PAvgOBYoiuBoIgoOQuDaIYzEEMoXCCGocjyHA4iWJ4pi6LYEi+CYLhSNAxjINI3jmO46hsNIbDaQIoiqRQ+gE",
        categories = "weather",
        tags = "weather,air,blow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Wind,
    #[cfg(feature = "wine_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgyGgOAiD4PAvgOBYThaBoIgoNwgDEMBoDMbYdiALQxC4Mw0DOEoUhmGIEhqCRNDEMoeDUdg3iyFYwi+BYHjIN4oDCHZBDOQxBjSKAzh4MpKCAMJPlGHYfGENQglaUJZCCRQ5DEIA0iiXYLC4NgzgyTQ5DgMRDDgLg3laTQ2DaDJtDgN41k2KQgDmDRoDYY4nlYNINCANqEgyUAuDSQwtC4MA1C4OAwDmjQxDSVonDGaY6i4PBsGkbhlCAeAygmDgiCAeQxqaqB4qsIgyqgealrCsYTC+n6hD6AQ",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMg+CoMGgZRpGcaB0gkOIMHcaRkHQaIcCIPg8C+A4FiYcBhiIIBkgkTQ3CAMQxHYNBhDIII6DAII9j2OgyGgNIlieLIiiaKYGggIgxguDYThEIIWhiGokCCE5Oh6IIiiSJoogQdA+gE",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "wrap_text")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig2CIIB4DKCgyDGDR4gkIgzg0eYQgsIg+DwL4DgWHhwGEdBoCAZIKE0MwgDEMhoDENRhiyLAwi2N42DYaAtDSHYfiSJojG8bB5iGBhwG+BB0HOCgxDaLZPDENItDiUAgDIMI+C+SJEkaHpGgeGwxDiGYWmOFIWhiDpilmHoggQZQ+gE",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis"
    ))]
    WrapText,
    #[cfg(feature = "wrench")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg3CANguDMYQxCCFQwCCGIahYLg0GwMQuDaHA2hSFoZieGIgDSGRsDODg3C2Lg3DcYYiiKGwxC0NwuDmK47j0bAthEOYVkMMRhDILgxDIIJJkuKIWjGMRskaQo8keNpQhWPw0jqPIejGDoijINh6CIPg8C+A4FD6AQ",
        categories = "account,development,tools",
        tags = "account,tool,settings,spanner",
        contributors = "Andreto,ericfennis,csandman"
    ))]
    Wrench,
    #[cfg(feature = "x_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDUIA5C0NggDYIobC+IokiGI4licIhtDmLIwjCMocjUaA+gEA",
        categories = "maths,shapes,development",
        tags = "cancel,close,delete,remove,times,clear,error,incorrect,wrong,mistake,failure,linter,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XCircle,
    #[cfg(feature = "x_octagon")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig3C4OA2CAMggDENguDENIWhaF4ShSH4YhqHIkhuHQyiCE4ViqHoZiiHohhWM4WCIPg8C+A4Fgcbo4HAYR0GgIBkhAbQxDUIA5C2FQ2jeOZAkKP5BkORQiG0OZKCCTZPjqVA+gEA",
        categories = "maths,shapes,notifications",
        tags = "delete,stop,alert,warning,times,clear,maths",
        contributors = "colebemis,ericfennis"
    ))]
    XOctagon,
    #[cfg(feature = "x_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgHcaRkHQaIKDEOINg8Igzg0aBlGkZxoHSGIaCCCYdCIPg8C+A4FiwcBhhYIBkgobQxDUIA5C0NggDaK4tjKFoxjMaI1jcOY7j+P5BC+QxoD6AQ",
        categories = "maths,shapes,notifications",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XSquare,
    #[cfg(feature = "x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2g4IIMCIPg8C+A4FhSF4GggIhtg+DwxDKEQyhOFYaD6AQA",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    X,
    #[cfg(feature = "youtube")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyC4NQgDENxhDINAuDEMgghOFYXDAIIcDGHQtDGHIXhuHYPg8Lg0iGKQgDQOYNDaLYvDWMYch6Dw2C6GxBiSJo3DIMYNCCEYZhaGIUkaNonh6I4Yj6D4rDSKJSi6DYOlUNYOkoMYhjmO49luGJChAIg+DwL4DgWZppgaCAiG2IoPg4NQtDMLZ0DMeplmebA+gE",
        categories = "account,multimedia,social,text,brands",
        tags = "logo,social,video,play",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Youtube,
    #[cfg(feature = "zap_off")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDILg0DEIA2C4Nw1CAMQzCAMocDALg1DcIA0C4OQyCIPg8C+A4FgeCYsi+BoIgqDBug6EAxDiI4lhKKIWDKFgxDCHA1C4Ng2iGK4tjSMRljOBI1gmCxvg2DwiDgIJbh4MQ0hyIJfhyQpiksMQ2k2LpTlCLJQCAeAxhCKggHmcginQeQynOdB4nueIqiwL5tgEA",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[cfg(feature = "zap")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDMIAyCCEwxDQIAxhWGIaDGFIVDKHwxDCGociWEoUCIPg8C+A4Fgcbg+gE",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis"
    ))]
    Zap,
    #[cfg(feature = "zoom_in")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQxCKCR4gyDoQHKDA4CIPg8C+A4FgeGhsGkboIHgMoTDYLg2DWEB5iaDYoiqEB4DGDAyg8IB5jQIo2hmG4hiOIIiiSOoUjiOoYjiLgxDSMpKg+Ggvj8ZZBiMIIlhOTJJhON45luMpHj2UZCD6AQA",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQxCIIBygwOIQGMeIMg4Ig+DwL4DgWB4bGwaRuggeAxgwMoPCAeAyhgNguDYNYQHmLYNi+MYzicIophqHIiiSIYjggeY6hkII0hiKoshgNIQiaE49C+PxlD6AQ",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
