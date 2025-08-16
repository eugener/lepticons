
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideGlyph {
    #[cfg(feature = "a_arrow_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NQgDEMxoDYIg+DwL4DgWFYYgaCAiG0MoPDYIA0g0LQ5iODQgDmFIWhuGoEhyCRNDEOAgDcdorhWF4wi+BYHgkbQxDSD4gkORgtDSLI7hmAQA",
        categories = "text,design",
        tags = "letter,font size,text,formatting,smaller",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowDown,
    #[cfg(feature = "a_arrow_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzC4NQgDEMxoDYIg+DwL4DgWFYYgaCAiG0MoPDYIA0g0LQ5iODQgDmFIWhuGoEhyCRNDEOIhFYN4sheMIvgWB4JG0MQ0g8MYjC2QpHjmLoBA",
        categories = "text,design",
        tags = "letter,font size,text,formatting,larger,bigger",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowUp,
    #[cfg(feature = "a_large_small")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDQaAtDUIg+DwL4DgWFYYgaCIKDENoODYdgtDMLg1GEMomCCKQ1CAMIug4IItDAVofhSFobhqBIcgkTQ0ioMQzGgNo3heO46gWB4JG0M4hCCPw1C0OZPioOZFjmAQA",
        categories = "text,design",
        tags = "letter,font size,text,formatting",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    ALargeSmall,
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
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAcoICIMoMHmCYLD4PAvgOBYoHAYYRCAZIJE0MQ3CAMQyGgLQyGyOwgDWOwtDEMI+DUSA3CKKAvi6EQ+gE",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHjkOZNiyLouDQLQ0maNIBA",
        categories = "devices,notifications,time",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockCheck,
    #[cfg(feature = "alarm_clock_minus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQzCIIBygwOIQGMeIMDEMgiD4PAvgOBYHhwcBhHQaAgGSDBNDUIAzCAMggDaG4diOJYiiSJooCIbQyi8NgtDOP4yC+NBojaJYnikNguDMOAgDEOAuDcIA0i4MZCkSRo4ikMQ3C4NpUk+XpSDIMJVleN5ZkgIhNDmTgzGiMYckOaIBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQzhAcoMDgIg+DwL4DgWB4bHAYR0GgIBkgwTQ1CAMwgDIIA2hqHIiiSIYjiWJwiG0MouDYLQzj6MQvjMaI1iSJooDYLgzDgIAxDgLg3CANItDGQZDkWN4oDENwuDaU5Ol2UQyDCVJWjaWJHCITYOk0MB2jCG5CmcPJDmkTQ5k0MxonCMpngE",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockPlus,
    #[cfg(feature = "alarm_clock")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeILDEMoOGMeYSDMIg+DwL4DgWB4bHAYR0GgIBkgsTYTCAOR2DQbAyCCFIbC+IokiGI4licIhNDUIAzjEIA2hqHI1GiN4kiaCxtDKMA2C0M5PkONI4keOYoDYLgzDgIAxDgLg3CANIxDGUpFlWSY7DENwuDaYpdmyYAyDCY5llSAQ",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_smoke")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAOBhDIIIQDAIITDELQyhcVgzGgMgwHYM4PhGFIjhaEAyFoIg+DwL4DgWKotgaCAiG0MQ5g0LQuDgIAzGOOAxC4No4DYIIWj+EAxEgN49kCFAtj+FguDSTguhgMRMDWDYpiuMIvgSMYJE0MZDDIMRjDCFwuliGAymmEQtDWWosl6XYFgeYAxDGEZlmebJqmifpwiqcougKXp2gqY57n+bp9m6gZbnOAQ",
        categories = "home,devices,travel",
        tags = "fire,alert,warning,detector,carbon monoxide,safety,equipment,amenities",
        contributors = "danielbayley"
    ))]
    AlarmSmoke,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByggIgygwcoXh2DBoGUaRnGgdIThUPg8C+A4FiwcBvGweRsGkbhlCCMo3HQc4TDEIAzCAMZAkSQw0CAOJDDeQ5FkyC4sC+Mo0jaOA+gE",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "alert_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bGwaRuggeAxhmDx4DKKAgHmJwiDiDx5iuDIahyIokiGI4Ii6Ew2imNIUC4MAximL4UjKQY/hsL44GUPoBA",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[cfg(feature = "alert_octagon")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig3C4OA2CAMggDENguDENIWhaF4ShSH4YhqHIkhuHQyiCE4ViqHoZiiHohhWM4WCIPg8C+A4Fgcbo4GyDBlCAeQyhAMQyCKQwxhAOJJHiSwikeTpFlGSI4C+QBuGWP5BCAeJUkcLgwDGSR5lCGZOmeSJDmANo3jmWZbgE",
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
        svg = "gAPBsGkbhlCAeQyD0Ig2CIIB4gkIgzg0eQxgqDIOhUIgyDEIg+DwL4DgWHohgaCIKDEMoNg+Cg3hOGYoiqL4th6IIEGWI42geLw4hOEAxjyGInDmKoQDWHYfiQPoBA",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIByHiDQyhCEwiDSEBoGUaRnGgdINDGDwgHmFAiD4PAvgOBYoiuBobh2H4NDmEIlCKNAghcMYZhGF4VgeCYLg2D4oiqBB0igcBhgsIBkg0TQyDIIJREiFZFkqCw+gE",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAaBlGkZxoHSCQ2gwdxpGQdBogkMYVCAch4gkMoMiEIojD4PAvgOBYoiuBolDmDIOhCEoUgyCAiDGC4fiWIwgheGYbCKMYoiqBB0igcBhhoIBkgkTQyDIIJRFaJ4pkqGg+gE",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDENINgkIoXCAdxpGQdBogoNoNHmCg1CIPg8C+A4FimLIGg+EYThUMIYhWG4lCIN4Nh2H4hCKI4HhmDIpiuBB0ikcBhiAIBkgoTQxDcIAyDIdgtieRZKiCSZLGiTZPlEIA3FaRIqloaJckyTgiE2UpUlYM4omaXZpl6a5tCANZknIL5nD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB5g0NYQHIeINDKEBoGUaRnGgdINDENIQhcIojD4PAvgOBYoiuBoWhiEIIgqDIOhqHIeiAIgxDCEISCIN4kiGJ4pi6KBwGGCwgGSDRNjwIAyHYMo9igL5IguR5JGiS5NlOUJSlSKZXGgPoBA",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINQiCAdxpGQdBogkNoMGgZRpGcaB0gkMQ0gwch4gkMoMiEIoeD4PAvgOBYoiuBolh2DIOhCEgihQIIWhiGocDCH4liOB4JDcIooiqBB0igcBhhEIBkgkTQ0CAMh2DKPZFkqEZJksaJNk+HZSlSVoplgaA+gE",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDWDBoGUaRnGgdIJDENIMHKCIKgwdxpGQdBogkNgiD4PAvgOBYoiuBocgmCwgiCIokCKJgghKFIWhgMIMh0MY4g4Ig3ieKYuigcBhiMIBkgkTQxDIIAyHYMo+igL5KiMPoBA",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB4g0MoQHKEgihQIB5g0NYQGgZRpGcaB0g0MQ0CIPg8C+A4FimLIGhsIg3hCCIKgyDoeiCIokCIMQwhCFwxhmFoTiiKovikcBhgsIBkg0TQyDIIAyHYMo/ikL5KgsPoBA",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB4g2DwgGgZRpGcaB0g0MQ0hAeYNDWEByhIIgyCIPg8C+A4FimLIGgiCoMg6I4licIIgCIN4QhaGIahwMIQiUMYPimK4EHSKRwGGCwgGSDRNDIIAyHYMpBkaS4LD6AQ",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOQiCAeYJDeDB3GkZB0GiCQ2gwaBlGkZxoHSCQxDCDByggIgyCIPg8C+A4FikcBhhUIBkgkTQ0CAMgyFaJ4pC+L4Vi6MBojKNAyDCN45juKo+GgPoBA",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NGgZRpGcaB0goMQ0g0eYKDWDR3GkZB0GiCg2CIPg8C+A4FimLIGh0Ig3g2CIKgwIIThWF4ZDCDYPDGHwgiGI4lCKJ4piuBB0ikcBhiQIBkgoTQzCAMh2DKPZIk2JJMk4aJQlIMgxlWV5ZiqWxoD6AQ",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_justify")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgzCIIB5gkIg2g0eQygqEoHhUIgyDEIg+DwL4DgWHohgaD4KDEMoNHiGYbiqEIMg6GYoh2H4kiOBIGiuCotjGJw4i6CowiYIgxj+HogjgPoBA",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[cfg(feature = "align_left")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgyDEIoHDKCgzg4eYQCINoTgmFgiD4PAvgOBYch+BoIgoMQ1hiJQyhOFQxiqD4RhuHYiiGBIjhkMQ3g4eIVhIIIUiUOIoCIMZBhyHo1D6AQ",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[cfg(feature = "align_right")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig2CIIB4gkIgzg0eQxgqDIOhUIgyDEIg+DwL4DgWHohgaFIKDEMoThCKINHiGYbi2EA5h2H4kiOBIlhkMQ4i2L4cg6EA3iqJ48h6II4D6AQ",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINgiCAch4gkMoMHcaRkHQaIJgsIBoGUaRnGgdIJDGGYPCINAiD4PAvgOBYoiuBoOhCDIbh2H4JDmEoUhaGIMiQMYmgeO4oiqBB0igcBhhYIBkgkTQyDIIAyEiEZCkeFg+gE",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDR3GkZB0GiCg5g2CQihYIB5goMQ0CIPg8C+A4FimLIGg+EYThWDYIgqDAghiGocCIMYiiCIokCKJ4piuBB0ikcBhhsIBkgoTQyCAMh2DIMIoiqSobD6AQ",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIB3GkZB0GiCYLg0eIJDWDRyhcIgyg0aBlGkZxoHSCQ2CIPg8C+A4FimLIGhwN4fiGI4lCKJ4HgmDIOhCEoUDCGoch6KYrgQdIpHAYYSCAZIJE0MgyCANxoC2GZEkmEpIkoaJMk4N5SEgMYoiqWBolqS5NCKT5RDGUwtDOYwvmWZ5cmkTQ1CCbRIkOZJbD6AQ",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIByHiCQyg0dxpGQdBogmC4NGgZRpGcaB0gkNoNhAIg1CIPg8C+A4FimLIGhyHogiKJIJDeDYPhGE4VheGQwg2CAigyKYrgQdIpHAYYXCAZIJE0MggDIMBolKKIqkmF5IkoaJMk6UAxlOVZElgaA+gE",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIByHiCQyg0dxpGQdBogmC4NhAIg1g0aBlGkZxoHSCQ2CIPg8C+A4FimLIGg+EYThWF4ZDCH4hiOJQiicIIcDeDYICKDIpiuBB0ikcBhhcIBkgkTQyCCCxoDKN5FkqF5JksaJNk+UQ0lSVoqlgaA+gE",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHIeIODKEYUCINYRHcaRkHQaIODENIRHmIYQD4PAvgOBYoiuBoch6IAiDEMIXg4N4khWEYTjqB4JguDYPCKKIqgQdIoHAYYfCAZIOE0MggDEMhoDKNZEkmHw+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDR5goMYMCAdxpGQdBohkNINgkIg1CIPg8C+A4FimLIGhiC4Ng+EYThWDYIgqG4dh+IQiDEMIkgoN4oiqL4pHAYYgCAZIKE0MggDIMhoDKQYpC+SYgD6AQ",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHiDQ1hEaBlGkZxoHSDQ2hEcoTCIMoRHmDQxh4Pg8C+A4FimLIGiUIoeCCIINiOEoNDeEYIgqDIODCFoYhqHIyCKKYrgQdIpHAYYLCAZINE0MggDIaAykCR5MgsPoBA",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHmDg5hEdxpGQdBog4MQwhEeIODeERyiAIgyCIPg8C+A4FikcBhhoIBkg4TQyDIIAyDASInikL4vhqLowGiMo0jYIA0juKIqj8aA+gE",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHmDQxDWERyHiDQyhEaBlGkZxoHSDQ2hGGQihYPg8C+A4FimLIGgiCoMg4MIlg0N4ch6IIiCKJISg0M4XiaG4piuBB0ikcBhgsIBkg0TQyCAMgxGgMo1kWSoLkmSxok2T5RDOVZXiqWRoD6AQ",
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
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEMoOGMeYLDUIg+DwL4DgWB4bGwaRuggeAxhKFAgHkMoLDiDh4iwIoTg4eYnCIMoUhsL4iiSGxwGEdBoCAZILE0NQghMSAyGEMQwkiTpQCCTgylAaAtg2Oo/kEPoBA",
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
    #[cfg(feature = "anvil")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwGMLQyC4OAgDALQ1hELgyheFxoDUIg+DwL4DgWIYkgaCIKgwNB2DgaA3GGFIUDCFY1DgLQ4FqIIiieJoEiiCRNDmDQyHaH4hiOP4+gWB5BDENZEkaO5JiWAo/k2CpQDIMBhDMIJejSNAxl8LQzGgOJdl+NZimoMx2DESA1jqSI9gEA",
        categories = "buildings,tools,gaming",
        tags = "metal,iron,alloy,materials,heavy,weight,blacksmith,forge,acme",
        contributors = "danielbayley"
    ))]
    Anvil,
    #[cfg(feature = "aperture")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ0C4MwxCAOAgDULg3DQIA5C4OQ0hmG4hiOIIiiSJgiE2Ng2DmLhoDEMQuDQOI6C+PBoj6I4licN4ri+DowjINAtjaOJOlCUpAgyQwukUIAxDYIAzjcNQgDYLgwDaYI/mKVJCimK4tmkSAyC4OAznSPQ8lCd4onANgymgMgtjGM41jeOYak+dYBA",
        categories = "photography",
        tags = "camera,photo,pictures,shutter,exposure",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAdxpGQdBog4MgwhIch4hmEoeCIMoSHmDg0CIPg8C+A4FikcBhhcIBkg4TQxDAIA0HaJ4pC+L4Xi6MBojKNAyCAOBohqKIqj4aJAjGMwiE0No4jqSo9kEPoBA",
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
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDODBoGUaRnGgdIJDWDByggIgxgwdxpGQdBogkMgwCIPg8C+A4FikcBhiMIBkgkTQ0CAOB2DEMRhDIII9DAIJAkCPQyGiC4pC+L4ji6MBojKNImjeOY7kSQZWDELZEGiWYoiqShokyMYzCIbQ5CAMQ1CAMwtDOapql2SZNmGTpjE0MY9ncdg5nCXw+gE",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHiDQyhEeYNDOERyhMIgxhEaBlGkZxoHSDQ1CIPg8C+A4FikcBhgsIBkg0TQ0CAOB2DEMRhDIII9DAIJAkCPQyGgMQyjyPpBkuQwtDIVg4iiKovguLowGiMoNG0OQuDUIAxDcIA1C2J4pC+VBolaMYzCKW5dl+PZemWU5XD6AQ",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQ1gweYJDODB3GkZB0GiCQyDCDByggIgxCIPg8C+A4FikcBhh4IBkgkTQ0CAOB2DEMRhDIII9DAIJAkCPQyGgMQyjyPpBkuQwtDIVg4iiKovh6LowGiMo0DGQJHGgNJSC+VBoD6AQA",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere,danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiFxyGUYx0CAch5gkMgiCAeIJDENY2HcaRkgWCYVCCMwikIaBlGkZxoHSCQ2haGIti+HIqgeCRNDGIokHYLQ2GgLY1iiHZTgWVYgjqIwwGiJ4YmKAQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ2CANIQhALQ0CIPg8C+A4FheGoGggIhNDcIAyDAVoVheGYEGiHIqgeCRNDGIgxiUNBoC0MoWhiHYsgWLogDENYNDAaInjqKoXHIZRjHQIByHmCY4CAeIJkAIggHcaRkgWCYVCAaBlGkZxoHSCQ2laTwiDGRQvkmSw+gE",
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
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAYR0GgIBkgsTYUCAOB2DgIobC+IokiGI4licIhtDgIIqDQII9DQLQ0i+HIyGgPoB",
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
    #[cfg(feature = "arrow_down_left_from_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAyDEaA2GEMoOCAMIWhgMgtDIVg1hOFYXiGG4bEiHoUhSIYWhuDh2DYIg+DwL4DgWMIzgaCAiG2DYPCAOQtDmL4xjaNYEjeCRNDmDgxEgMx2C2LowjKRQ+gEA",
        categories = "arrows,navigation",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftFromSquare,
    #[cfg(feature = "arrow_down_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINHmCgzg0dxpGQdBohWFwggkIobD4PAvgOBYoHAYYgCAZIKG0MQ2CAOAtDiNwiigL4uiCLYvGiMYKE2NQgjUSA4FaF49j8aA+gEA",
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
    #[cfg(feature = "arrow_down_right_from_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDEVg1GEMgghQMAghcMAtDKGxIhKFIWhiGIbhUdgxDSE4ViKGYqDIaA2CIPg8C+A4FjKNYGggIhtgyFQxC0OZAjGM44jeBI5gmC4NDENR2DYaAtjCMo0kcPoBA",
        categories = "arrows,navigation",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownRightFromSquare,
    #[cfg(feature = "arrow_down_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEeYdhEcocCIMgiD4PAvgOBYoHAYYLCAZING0OAgjWN4nimLoLi2LxojGDRNDENo2HYOBIhCKAvjsaA+gE",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEhyHiDgyhKGQihYIB3GkZB0GiDoQCIPg8C+A4FikcBhiQIBkg4TQxDIIA4HaEYpC+L4ki6MBojKDhtDgII2CANJJkkLQ0iiKo+GgPoB",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYMQ0CIPg8C+A4FhSF4GggIhtDEOQgDkLQ3CAN4jiOE4VhqFBjGkchjGwZQgGMeIJgwIggHKNo4GMeYJDIMYpC+LYvjEPoB",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTQxDYIIOEgOAihsL4iiSIYjiWJwiG2DggDgLQ0CCP5BjCHIzGgPoB",
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
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIByggIgygwdxpGQdBogkMQ4gwaBlGkZxoHSHIeD4PAvgOBYoHAYYaCAZIJG0MQyCAOAtDQII6jwIooC+Loai2LxojGCRNDENggjUSInimQRoD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ4CCDhoDgIobC+IokiGI4licIhtg6Kw2CANAtj6QIvhyMhoD6AQ",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEeYdhEcocCIMgiD4PAvgOBYoHAYYLCAZINE0OAgDEMhohCKAvi6C4ti8aIxg0bY3jYNggDQLZJkuJ4pj0aA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyFhyGUYx0CAch5gkMgiCAdxpGSBYJDSMx4gkMQ1jOMAijoIBoGUaRnGgdIJiWFgviqLIbgSHYJE0MYhDIMB2C0NhoC2MpNiiApSgeVI+CCWBojqYJSD6AQ",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp01,
    #[cfg(feature = "arrow_up_10")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAOAgDQLQ0g+DwiD4PAvgOBYWhmBoICITQ3g8dgxDaFYXhyG4Eh2CRNDGIQxDAVg0GgLQyiaGIqimBYHiwMQ1CCMBoDSN4oDwchlGMdAgHIeYJjYIB4gmPgiCAaBlGkZxoHSCYlCAdxpGSBYJkMIJNCIMZDhYL5HkkPoBA",
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
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsbQxDYIIUC0NIui4IA0CKGwviKJIhiOJYnCITYUiwNhWDiNIcjcaA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQ1CAOQgDcLQ3g+DwiD4PAvgOBYWhmBoICITQxDIIAxDYVgyhWF4chYYxpHIYxsGUIBjHiCYhCKMh5gkMgxjcco1igL4si6MA+gEA",
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
    #[cfg(feature = "arrow_up_left_from_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMIAzGgNhhDIIITDAIIWDGFIUHYMQ0hKGoWhgLYTDISA1h+FYXCAMYjiMdgtDYIg+DwL4DgWM42gaCAiG2DYNDkIA5jKNI5jiBI6gkTY/FaD4xjONZHD6AQ",
        categories = "arrows,navigation",
        tags = "outwards,direction,north-west,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftFromSquare,
    #[cfg(feature = "arrow_up_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJCIM4NHmCoRCAaBlGkZxoHSCgxDiDR3GkZB0GiHYfD4PAvgOBYoHAYYkCAZIKE0OAgDENhWDgaInimLoki2LxojGM43jYNggjWPAvj4aA+gE",
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
    #[cfg(feature = "arrow_up_right_from_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDMdg2GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNgiD4PAvgOBYxjSBoICIbYMCAMwtDkIA5jCMo3jaBI4gkTQxDWPYuhGQ4zkcPoB",
        categories = "arrows,navigation,social",
        tags = "outwards,direction,north-east,diagonal,share,open,external",
        contributors = "danielbayley"
    ))]
    ArrowUpRightFromSquare,
    #[cfg(feature = "arrow_up_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEeYUhEcoTCIMoRGgZRpGcaB0g2DwiD4PAvgOBYoHAYYLCAZINE0OAgDgaA4HaEIoC+LoLi2LxojGDRtjUMQ2jYLY7imPhoD6AQ",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINHcaRkHQaIVhcIIJCIM4NHmCojD4PAvgOBYoHAYYcCAZIKG0MQ2CAMQyC0NI6joIA0CKKAvi6HIti8aIxgoTY4jcNhWheQZDGgPoB",
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
    #[cfg(feature = "asterisk_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOE0MQyCAOB2hGKAvi6F4ti8aIxg4bQ4C4NQgDENAgDcLQ0ieKY9GiP4wjIIpEkaSAwksIJOjuUQ+gEA",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    AsteriskSquare,
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwNIQGMeIMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTQxDYIA4HYNRhDMIIxDAII0jSKwwHYLQxGEMY0j6NQgDGNQtDSLIahyIokD6AQA",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYShQPg8C+A4FgeGhwGEdBoCAZILE0MgwC4MggimKxjDILgwDQLYxDAM4yDILQ3C4Mw2C0NAuDULQxDELg5kCQo0kENY6DkLg4DcLQ2kqRJGkiTI1jINItjIMwtjkII8j4IJMCCRZHmWSpqk0IJPlEIJUDWXJoDmbBaCKGgviGI4giKJImCITQxDULg3mehQ3GOTI6kyXJyjSb6HlmdZajelo0mOP45laaZZoybI0pCbpQlKZp1l2N6plymphpWTJ4nqfBoD6AQ",
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
        tags = "hatchet,weapon,chop,sharp,equipment,fireman,firefighter,brigade,lumberjack,woodcutter,logger,forestry",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHkMYJDgIggHiZQiDEMpoHgMoJm2aB5nGbJulkL5fmGXpgmKZJyDadJ2DGgppoQMguDAMZvmuc55nsZQ+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlmXIGgiCg5C4MA5CAORhDMIJqkuHA1g2agxGOGQyC2agznaa5YlqX5ZGwaRuGUIB4DKCQxDKZAxCKgwxoYMqLHmjQiDEN6QoWk6VlkL5/oEPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMYJDEMgimMMpmmgIB5mUIgxDaaR5mucZolkL5fmGXpgmKZJsmkeJ2mcLgwDGdJwDidJ2oueZ7GUPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDENgimMMYJDiaR5msIgxDKbplnGc5ZC+X5hD6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDEMgiCAeQxgkOJpHibAimeaR5mWcg2liWpfmGXpgmKZJmniY5xm6apxnOap2nOWQvnsZQ+gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQzC4OA1CAOAuDYMhhDQIIWDAIIZDGFwuDcOAtDSHg3heJYZhsIA2h6GIliyKIih+HYxhaLgghyGYqDeNImhqNohiOMg4i2PYbC2OYPhmO4nj6MIgjCJJKj2N5Gh4NhaCIPg8C+A4FlkbBpG4ZQgHgMoJDkIggHmZQiDENZoHkMZmmgeJxmybpZC+X5hl6YJimqCZtm+dZnmOa6BmOg5YlqehlD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDgSA2GGDYNDAIIWDELQyhoVg3hMIIVheF4ahoIg+DwL4DgWJ4qgaCIKDEN4ODQVg0h+IYWDCJAyGgLQxjeIo5hqIB2DEMImiiLYnHIZRjHQIB3GkZIFgkMQzCIIBoGUaRnGgdIJDiWByHiVZYHmCQ2liZAimGJwvkyTonGMaRyGMbBlCAY5nCIMpHCAcoJDKWBjmuD5IC+c51necp0naeKAnyg5rDmg57n2h6Jo4PoB",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0NAuDkIIpisMYpDIIIvC6D4aC+IYjD6AQ",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHiDQyhEaBlGkZxoHSDQxhUIB5g0NoRHKE4OCIPg8C+A4FikYxpHIYxsGUIBjiaHoRGOIQijgIByhSKIqi+MYzikcBhgsIBkg0TQ2CCHhoC4MAxE0MQ4k8MpRlOQQvkeCw+gEA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "bar_chart_2")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDgIggHmCQiDIMIOHkMoKDGFIHheC4ND4PAvgOBYfiKBoQgqE4VhwNIOgiGAyi2HAxjCH4hgQZYkjeG4KDaLYRj2D4RimD4yiyNYlD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFRyGUYx0CAch4gkMQiCAaBlGkZxoHSCQ3iQdxpGSBYJDSJIhCKLQgHmIgwhSFobh2Goch6NgxDWJImiiKoiDKJI6CKRofkSLowjIIo0hUL4/HQPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley"
    ))]
    BarChartBig,
    #[cfg(feature = "bar_chart_horizontal_big")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMx2DEOBohAIg+DwL4DgWFRyGUYx0CAaBlGkZxoHSCQ0CIIB3GkZIFgkMQyigeIJDeKB5gkNYoHKMgiDGFIWhuHYahyHo2jwM4oiCIokiaOY7j0II7jSKYri0Io0hUL5AHQPoBA",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley"
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
        svg = "gAPBsGkbhlCAeQyD0IgxDAIggHgMYKDEMoOHiCYLhSB4RCIMoND4PAvgOBYfiKBoQhIOIVhcMYpgeFw0g4eYbh0IofiGBBliSOIPhsNoqgqPouhKQYygqNI2iUPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQ5CANoNC4NQgDOEBhDGEAghaEQwCCGwwC0MQthAQw0C4Ng4DOEggDSEolieKoqhAVgxDcYQyCCNodhyN43GgMQyjWO45hsMgtDIdgtDUIg+DwL4DgWSxsGkbhlCAeAygkOAiCAeQxgmSZVl0IgxDCWh5lcIg3kqTJRlOUJSlSZoJj6Wh4mEMplmGc5VmcMp3ksL5sGWbpTmCCZpnuhp4nIOZlnwMZqoCb6DlSdZyoeXKLo2CQyo+iJimmf6BD6AQ",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bath,
    #[cfg(feature = "battery_charging")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA3GgMRhDIIITDAIIWDGFIUHYNoShqFoYC2EwyGiIgiD4PAvgOBYoiuBoIgoNoOEgNIehWF4XiKG4diOOIghqJAxieKYui2BIvgkbQxhkNwtDMIA1GgNBsk6UJDiqR4oGwaRuGUIB5DKCQxDMIpfDGYpCCAeJnCIMgymUeJhm2b4oC+W5dD6AQ",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    BatteryCharging,
    #[cfg(feature = "battery_full")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMAiCAdxpGQdBog4MQ2hIch5g4MocHiH4SiIIogCCHgiDcIg+DwL4DgWLRsGkbhligMYZDGJAyh+Jx4jiJonHmPIPDOLIujONYyjSNh4kSG43jmEpDhmRggj+Dobi0L5JGWS41leQIQlOYo6leRJjiiaJGluXZfjaVJFiSYg0juGZ1lGD46m2TA+gE",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAaBlGkZxoHSCQxDCDByggIgygweIJhsIByh2GoMHcaRkHQaIUDYIg+DwL4DgWLRsGkbhlCAeAyh6Hx5DGFAxhyPYajuOQiDEM4si6M41jKNI2jiCYrgeQQxj+N5BlEeZEkaSAvkoZQ+gEA",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goN4NHcaRkHQaIKDENoNGgZRpGcaB0hgMINgmC4NHKD4mD4PAvgOBYrGwaRuGUIB4DKCgygyDo3CIMQzg0eQxhgMYkkKC4MisL4xjOMIyjSNoKhqNZGlIeY8j6QJGDGRJJksZZNjOU4ikCV4/jWV4jg6WpciyXg+gE",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA3GgMhhDIIITDAIIWDGFIUHYNhjhgLYZDILYTDIaIjCIPg8C+A4FimLIGgiCg2g4SA0hKGoWjmI4bh2GAgiGGolDKKIqi+KRsGkbhlCAeAxgkMpDkwMpPlEeZOCIMQxCIIB5lOWAzkQL5IkqR5JkuVoJDeW5dgkMZglKbQwluTZxmGYxlmWSpclcMZqnCWJylyXp9C4MJakyfJyimYpmD6AQ",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIIJCIN4NGgZRpGcaB0goMQwg0eIKgyB4dguDR3GkZB0GiGQ2CIPg8C+A4FiwbBpG4ZYODGGQxhwMoeh8eI3guHx5jsIgxDOK4tjKNA+gE",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
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
        svg = "gAPBwGEdBoCAZA9CITQ5CAORjC0Lg2DSEQ0C0MQuDUMoXDkNYVDILg0DAMgghcMQ2DUQQ2CCKgwCCLYtDgIAyDIYQxDMLg5iqNo4iyLo+DmOAthMMQiD4PAvgOBZGkmBoIgoMQwC4Nw1CANQuDAOQzimK4+i0MYyiMOBji2Hw0DEMYQDaX4TDYOIWhEOAziuEQ2kWR5MkuBJNgkTZWDOZoklENo0DQIKFi+Pg2iAOA3iQLgyDAOBNlCEYjn6ZhhhMMAxlSmqcl2JIylcM4Mh8MA0naSJ6kYbBpG4ZQgHgMYJDIIggHkMq0rWt6zCKux4rmvq1kYL6tq8PoBA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIOAuDUIggHKDAyg+ERjHiDAxhWEA+DwL4DgWB4eHAYR0GgIBkgwTYbg8IAyGENoujINQgDCNo2C2MgyDIIA0C4NhjC0MQuDEIAzkUMwtC4Nw4kcLg5C2SAxk6MgwDgQQzkeOI3jeNZUGMNI4g4NJDC6TgxkSZY/DMQY0CCb5djiLY1DIWgih4L4lieJImiiKgiG2VIzi+RQ5j6D4xC4NJVouTpykYLgzkaPYyDSh6WoekAthUNggDULgyEMMagmIMgwqEIJpi+PQ3qwYZalqm6dDiZg2DYTIVmIMY0niH57GgPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDkLg0CAMQ0C4ORDDIMAuDKEA2g4IAyDGEA3h2HwxDcSAzHMMwthmKQ5GMMAtDMLgzh0Lg3C0NggDaNwgjUIAwhCMguiOE4yCIPg8C+A4FkeSoGgiCgxheM4eGEMYTg+Vg5g+P5cCCMZbkaSJNkcYxpHIYxsGUIBjHmCQ4CKax4gkMZvCAcoJDOYQvmWZ5pD6AQ",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley"
    ))]
    BellDot,
    #[cfg(feature = "bell_electric")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg4CANBBDYLgzCCDQ3CAMIZCAMQgDKGg5CIPg8C+A4FiOJoGgiCg5CAORoC4MAxiKJIpiMYxpHIYxsGUIBjHiCYhj4eZBCIIBygkN40C+OI6jyIxyGUYx0CAdxpGSBYJDEMJGHKQAiDKRpfDSRpECIMQ2kYaBlGkZxoHSCZpiML5RlOKIEiqCYLDSHA5GOFYaDQLg2C0MaDhCg6FoOS42DyTY7j0Y5mmiXYJmGPpfh+S6Pk+AQ",
        categories = "devices,notifications,home",
        tags = "fire alarm,flames,smoke,firefighter,fireman,department,brigade,station,emergency,alert,safety,school bell,period break,recess,doorbell,entrance,entry,ring,reception",
        contributors = "danielbayley"
    ))]
    BellElectric,
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
    #[cfg(feature = "between_horizontal_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDMIggGgZRpGcaB0g0N4RHiDYQCAeYdhEcocg4Ig+DwL4DgWJxwGGCwgGSDRtDIMggDENQtDOOQgjqEInC+LYLieKoGhOFYXhmIokDGEYgg4NIRgiCoMg6Hokj6KJED6AQ",
        categories = "layout,design,tools,account",
        tags = "insert,add,left,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalEnd,
    #[cfg(feature = "between_horizontal_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIOAiCAdxpGQdBogkMQzgweYJhUIByggIgxgwaBlGkZxoHSCQ3CIPg8C+A4FikcBhhEIBkgkbQyCAOQgDOOQtjqFYpC+L4RimLIGhyCwgiCIokiaFoTDSDIbhODIOhCEodj6KpED6AQ",
        categories = "layout,design,tools,account",
        tags = "insert,add,right,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalStart,
    #[cfg(feature = "between_vertical_end")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMwiCAch4g4MYSHmDoRCCFQihsdxpGQdBog4NwiD4PAvgOBYoHAYYjCAZIOG0OQgDIMggDMLQzjmOYnimLojiiK4GhmHoSgiCoMhaG4UhaEogiKJAiiaHIWDSP4qgQdA+gEA",
        categories = "layout,design,tools",
        tags = "insert,add,top,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalEnd,
    #[cfg(feature = "between_vertical_start")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMwiCAeIOhEIB3GkZB0GiDg3hIcoUg+Eh5g4OAiD4PAvgOBYoHAYYbCAZIOG0MQ1CAMgtDMIAzjmOYnimLobiiK4GiEMQ0iOJYfkaEoIgqDIOhCEoYhqHAih6KIqgQdA+gEA",
        categories = "layout,design,tools",
        tags = "insert,add,bottom,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalStart,
    #[cfg(feature = "bike")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ3C4NQiCAcoMDOEISGMeIMDEOIXD4PAvgOBYHh+IoGgiFAihaEYJguDYPiyGYMDWHogiaJICgSJ4JhqDYsikMYYi6EYfiGOo4HAYR0GgIBkgwTQxDIIIOhAVgxDQbAtDOWggDSXJSDMaAyCKRZJksPoBA",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIBoGUaRnGgdIJDaDRyHiCQyg2GYKgwIB3GkZB0GiCYMD4PAvgOBYoiuBoPhGE4Vg2CAih+IYjiWNoXh2Gwgh2FooiqBB0igcBhiQIBkgkTQ2CAMgwGiJ4pkeJJGkgaJKkyCwgDGUZTC+VRoleSZLCKTZdDQaAyHaQZUliZJZmYTZcmqbJumGcIBA",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeYLDEMQuDmDhjHiEoND4PAvgOBYHhwcBhHQaAgGSCxNDYLg3CAMwuDQYwthUIAyC4NQgDAIA1C4Mo1j0IIrDcQ4rjgOYuiwIA5C4No1CCE5MCKHAviOJYiiSJooCIbQ4jQMQwC4MZPjALg4lKHZVGiV4lieKQxDcLgzkiMY0jaOI6jwMgtjaPpCj8NAthSPp5jOTYrkeFA1meVJYmuWYLG0MY8mKX5hoGZJmlOaaOm0IhNDGfgymAOIyjYNozoANJMnuq6qDcLY8DOM4+qarJiqoOAtq6O49oumw8mmnafj4MYvDkdoUDavqNsCWLCsWN68jGgaznu1ZyDCy5Ws2bJap+LagjAY4sjO4I9peTaJqyiqasywbejyOLQDm47GmWYw4mOOJ2tqaoBA",
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
    #[cfg(feature = "blend")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIOQigkeIMg4IBygwNwiD4PAvgOBYHhmHIGggY4RCIMQ1g+CoMiaD4VCKF4ZhuBIhD6AQA",
        categories = "design,photography,tools,development",
        tags = "mode,overlay,multiply,screen,opacity,transparency,alpha,filters,lenses,mixed,shades,tints,hues,saturation,brightness,overlap,colors,colours",
        contributors = "danielbayley"
    ))]
    Blend,
    #[cfg(feature = "blinds")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoDEOAiD4PAvgOBYThaBoIgoMgwCANxIhGE4VgQaIYiWB4JE2HQgDEMYhhKFIZieBYpgoMYeDEOYPDCMYkheAoohsTQ4i0NYPDKPozkGNZDDSDR2DENJKiWExjGkchjGwZQgHKCZJCAYx5gmOgimEeIJlOI5XlmWw+gE",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[cfg(feature = "blocks")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMQ0CIIBygiCoNHcaRkHQaIJDeDRoGUaRnGgdIYg0eYJDMIg+DwL4DgWJxwGGFggGSCRNDEMAgDIMRWDgYQxCCPI1j8LQxkESA0juPQgj+SJBj0dgxDKRo+kiUo8DEaJOlCUpJkIMR2C0NZYkkMJBkOJYnC+LYWD6AQ",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CCDAxDAIIPC0NQgDUVgyGyFA1EyDg3CIPg8C+A4FiAbBpG4ZQgHgMoJDIMQiCAeQxgkMQyjAeIzCIMQ4jAeYsjqNogC+JooiWJ4pjiCQzj2P41jePw2j2OZOkKRBlD6AQA",
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
    #[cfg(feature = "bolt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FlAYxpHIYxsGUIBjHiCQxDIIggHKCQ0mIYx5l+YZQC+VpYloPoBA",
        categories = "account,tools,home",
        tags = "nut,screw,settings,preferences,configuration,controls,edit,diy,fixed,build,contruction,parts",
        contributors = ""
    ))]
    Bolt,
    #[cfg(feature = "bomb")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDkIggGMeILDEMYOGMeYSDMIg+DwL4DgWB4bHAYR0GgIBkgsTQxDQLgzDUIIrDaLgxDaLAgDILg3GGNw0DGNgujwIAwkEII9DOP5BGwMQuDaRJLjqR47kOQo9kKRg0GwLZKDmMguluGociKJIhiOJYnCIbQyDKNpZC6XA1l8L5hGgPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORjGkchjGwZQgGMeYJDEMgikUeIJDmSxykiOI6j+QZDjmPIGgiCg4g0MoSDIYYMgyMIfl2JpKjmO4EGiPpAkKRJRCIMZLkaSJKkySA1lML5VnAPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDENAiD4PAvgOBYoiuBoIgoModDgdolEiIQ1GGGoZhiHYdDGHAtDUSIyieKYuigYxpHIYxsGUIBjHmCQ4CIIBygkMpVGMeIJiWRgvkqTJOi2BIvgkbYyhkLQ0hibA1l+SICmWB5ng4IAzh8MZwmUPoBA",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,public,secret,unlocked,hidden,revealed,knowledge,learning",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDEMAiD4PAvgOBYoiuBoIgoModhQdg3EiIQ1GGGoZhiHYzhwLQ1EiMonimLooHIZRjHQIByHiCQxCIIB3GkZIFgkOJSGgZRpGcaB0gkNZSk8IgxDKUh5gkNpFC+SZLi2BIvgkTQxDiIBWDSOoZh4MZAgwMB2meKIqnEPoBA",
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
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ5C4NR2C0MQ1EEMoQCCFw1CAMIcg0IA2hgMhIDIMB2iUSIhDUYYahmGIdh0MYcC0NYkDAIg+DwL4DgWORjGkchjGwZQgGMeYJDgIpFHiCQxDKShygmT45C+P5BkOOY8gaCIKhSDQzGEMwgmKMYzDaHI4jqWg+gEA",
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
        contributors = "danielbayley"
    ))]
    BookmarkCheck,
    #[cfg(feature = "bookmark_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIdgxDYegiD4PAvgOBY0GwaRuGUIB4DGCQxDUIggHmQAiiaRB5DKQQwkQeJMCIOYzjWOo8D6AQA",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkMinus,
    #[cfg(feature = "bookmark_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIAyDELQ3C0NIRCANBWDUYQyg4IAwh0IAxg4LQyGgMQwhqHIeh6IYbDIdgxDYegiD4PAvgOBY0GwaRuGUIB5DGCQ3CIIB4DKCQxDKQx5kYIgxDOQx4kCTZJjQL46jyOY7j2RYJDmUJSDENZKkyJpKmAMIzjWVxlD6AQ",
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
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAORWDUYQyCCEgwCCFQxhMLQyGgMQyhGE4WiGGISDIdg0CIPg8C+A4FimLIGgiCg4CAOB2DGKIqi+LoEjCCRNh2NI2jiK48juBYHj4MQ2kGN4pkSLQ8HIZRjHQIB3GkZIFgkMgwCIIB4luXh5gkOZeHKYAiDKXhoGUaRnGgdIJh2Q5SlSKRjGkchjGwZQgGOaA4maYZ+mMIgxDWQ54nqfJ3nme59GOhaHoKaZen+cg2omjqMgEA",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = "danielbayley"
    ))]
    BoomBox,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA4FYNBIDgIg+DwL4DgWFRyGUYx0CAch4gkMgiCCIQiDSJB3GkZIFgkMQ2iQeYJhMIBoGUaRnGgdIuiOFQvhuHYVhiBoIgqDQxDQaI9haQ5CgSRIJE0MgwCCSJKhSTJPk6BYHlEMQ1lUMx2kuF5agKT5dgoOZhmOWJlhmAQA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME2DggDQLg1GEMosCCLw1CAMI0jQLYrDkNgtC4NA2jGMIyjaNQwC0MQuDkOAgDOQIzkKRI3kcMwyioLgyDSS5ZlCNQuDOWA1iySpCk+Q5AjqSwuDAOBBmOMJbCCKQxDmLIukGbpljkNo9libYzm+L46C0M5pkqTJMm+XQ0C2YA1mKdp+kORpdDKOJWnyj5lkWR5JoKbKYm+KYrDWGYbiGI4giKJImCIbQxmAN5wDCPY8nMNKkC+phoqiI4lgwbZzlQMaDpSSI9reua7qqvrCC6P6uC4N48oqSLHqmya9qwMaykqwLSrS1anDyubYq2r5woMNa0tK4K6uKqbksCsZIuqxoari1ruryq6trKM7PDes7UvayL5sq2aDDEILAwEObsD6AQ",
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
    #[cfg(feature = "brick_wall")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoHAYYWCAZIJE0MQyCAOR2DaJ4pi6Foti8aIxjMMQ2CAMQ1jmOwvj0aI/jCMgijSRQzkmKJLkCTpBlATQzkYNRohmSpMlmQpRl0OZghqVpjDyTJlE0OJelWPJYm2QJvnGVI6muWIBA",
        categories = "buildings,home",
        tags = "bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone",
        contributors = "danielbayley"
    ))]
    BrickWall,
    #[cfg(feature = "briefcase")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggGgZRpGcaB0g0MQ0hEch5g0MoRHiH4ciKDoRh4Ig3CIPg8C+A4FiwcBhgsIBkg0TQxDYIAyDEVg1GEMo7CAMJDkMLQykcaAtDSQJCkST5Hjsdo5iuLYygsPoBA",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIOAiCAdxpGQdBogmCwgHiE4MGgZRpGcaB0hcIByhYIgyCIPg8C+A4FiYcBhhEIBkgkTQ0CAMQwGEMggjgMAgjsMQtDKPxWDSN45jyRgxjmPxokOOI6keRYkiYL4shGK4tGiL4xDGMwyjaTZGjuO5NkuRJOmGSQyHYLZMkWZwwj+P4lieVBoD6AQ",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[cfg(feature = "brush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5C4MA2CAMQxC4OQgDiDQ3C2FoOGEMguDgNQgh2HwgDCEImDSDQzCCKAwDMbIZg2D4aDgIg+DwL4DgWNo5gaCAiE0N4XhCKA5DQYwthINoPDALYqhIMw1k0IAzg0MokhALgzDMLYdiCEg1DKXIhlWWAwDiWAxmMNIUh0MJWDSY5umOVolDSSIeisLYsDQYZUDCaZ+mmJaDk2TZVHqNY3jwPoBA",
        categories = "text,design,tools",
        tags = "draw,paint,color,artist",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDYIggHmDQyhEeINDSERyhcIoVCAcoTh2ERoGUaRnGgdIUDAIg+DwL4DgWLRwGGCwgGSDRNDkIAyDIdgtDQaA2HaGYtC+M4LjKNBojaOA4CANhoC4MAxiyLpHGiSY1jcIhNg+T5RlOVZGkqWZLluXQyl+UpUkWV5lkyXAxmkMQwmCbJWmQPJXnCaAgDGQJrmKbp6kqfJenSdqCnme5noegJhm2i6FmeTqIoGkZIoSWpNn6j53mOSIBA",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANggDIIA3CIPg8C+A4FhSF4GgiCgxDCDRoDSE4VhqGYEhuCRtDKEA3C0MgtDGI4WieFByGUYx0CAdxpGSBYJDENgiCAaBlGkZxoHSP5BCAeIJiIIB5gkM5CHKTQiDKMo2jiJoFgeCYLCAMQxGiQIyiWAonl6Cg4mENRoC4MIxhSM4YmiXYcE2QJtm+cZmjSdoogqDgxDkdpYnOZ4amqeZsDIMR2i6foYgE",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANh2DYIg+DwL4DgWEoVgaCIKDENYNg+EYThiF4EhmCRNDIIAxDIaAxDkLoQhKFIkiOBYHiYMYMjgaAzHMLg1C0MQuDcLg4C0MpEGMLgxC0Lg0C4MpMDiT5Ak8IAwkwNJMkuRJMlCQQyGyVJZDUQwyDCSoNkSKYuDGDYpgwNhIDQYYoigMJWlaRggDIdgxDCO4gjKFg8GMaRyGMbBlCAcoJDIIggGMeIJDejxjHmCY4oGhaHomNIlgoOZvGgNaBiKhKGoiiqRpiEKLo2laXCKmYxpuqQ+gE",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAzGgLgwDEIg+DwL4DgWFYYgaCIKDENAgDKEIShSFobhqBIcgkbQyCAOYhDALQ1iWF4piiBYHgmC4tDEMhWDYLozhWNYZDwchlGMdAgHIeIJDMIggHcaRkgWCQxDaUBoGUaRnGgdJWDCUJNCINJQHmVgyjSR5JjeKoKi+PR2kKJo2gKKY5h4NQgnGc5EGibZ4E2IAxDcaJXjSJ4BA",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[cfg(feature = "cable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAORhDIIIQDAIITDELQyhcVg1GgNh2DKD4RhSIoWhAMhaCIPg8C+A4FimLIGgiCgzCANRWDOKIqi+LoEjCCRNDeNI2jiK48juBYHj4MQ5CAMY1DYLg1GEM5QCCUw1iKEwwC2QAwHYMQxlKVJWliTJbhQVg5kOOoCjySIKDGQAyDEdoXmqRZskeMRNnKEZznWKZEi2eI9goMoQkoaAth2F4ghKI4RhcaIfiWZAxiGJp2i2AQ",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,energy,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley"
    ))]
    Cable,
    #[cfg(feature = "cake_slice")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINwiCAcoMDKDxjHiDA5CIPg8C+A4FgeGhwGEdBoCAZIME0NwuDIIIpDkIAzCAMQxHYORjDAIAuDYLg0jGPQxGgMQ2GOOQgjcMQtjuMQtjMLY1DALQyC0MwtDYLQ3C0OBslKRAyjmGYbiGI4giKJImCITZBjEMxIDOXwvmEaJjiOJYnmkMQ3mybpwD6AQA",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = "danielbayley"
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDYIggGgZRpGcaB0g0MgwhEch4hmER5h8IIeCINAiD4PAvgOBYoGwaRuGWIwyg2D4RHgMYNDiII4CKEAgHmM49ieKYujCLYvjGN40j6QI0jqMpLjuNImigL5FGWKBwGGCwgGSDRNg8IAxDAaAuDAMZDC+WoLlmWxol2XwxDKYpkmaaJVmsaJtlyXgiE0OJ0mWZ5pnme5vn2YJzDENKCneKaFDyeZwn6gKLo2hJuoak6JmIOKXnimaRm6m6Vp6dqYmyAQ",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAdxpGSBYJDEOAiCAeYJhAIByHiCQyjAaBlGkZxoHSLYvCCNwiDOIAviSJoXgSGYJE0MwgDEMBoi6R4igKTIHgkbQ5lGHQyg0IA0C2EIShSTA+gEA",
        categories = "time",
        tags = "date,day,month,year,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[cfg(feature = "calendar_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3C4NRWDYYQyCCFQwCCGAwC0MocEgNYUhaGYjhuFQyHYMQ0iGF4kiIMhoDOEAiD4PAvgOBY0jeBoIgoMQ2hYdg0jONY6jmBI7gkTQ4kCQo0jaR5GgWB5JDMIAxDAaA1kOT44gKR5Tj2Dw1laYpWj+PguDMVopluRQ8GMaRyGMbBlCAYx5gmPgiCAcoJDaexjHieZ/k6cJynQPoBA",
        categories = "time",
        tags = "date,day,month,year,event,clock,hour",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarClock,
    #[cfg(feature = "calendar_days")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAeYJhAIBoGUaRnGgdIJDEOAiCAeIJDONh3GkZIFjONQgHKOAiDKIAviSJoXgSGYJE0MwgDEMBojSR4igKTIHk6DAxDQaAuDAMZWkyS4FlqGwylGXpgmKEoUmSWJmhoTYcmqX5hmOFpxk2CpcDid5tiGcIYmedJpjSgJ5GiZZ8nSHaImyig+gE",
        categories = "time",
        tags = "date,month,year,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[cfg(feature = "calendar_fold")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiF4EhmCRNDIMQgDENxWDYYQyg0IAwjKMgtDKNhIDWL4xjOPY2h4MQ0juMI9jSMAyGgMQxFqIIUiSI4FgeJgzioMJJDiTYigKJJShsNYNg4LZCkeNIzimN5IhCEpOhaAQ",
        categories = "time,files",
        tags = "date,month,year,event,birthday,birthdate,ics",
        contributors = "danielbayley"
    ))]
    CalendarFold,
    #[cfg(feature = "calendar_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMQwGgMQ4FYNhhDIIIWDAIIZDALQyh0SA1hWF4aiSHIWDIdgxDSIoYiWIwyGgNwiD4PAvgOBY0jeBoIgoOIXHYNIzjWOo5gSO4JE0MQ2j+QY0jaRpFgWB5IDIMQuDIOYNDQLg3hULg0gwMpfgyG4ah0Lg2DULQuDUMhjmsMwuDGHpsDcLgzmsOJsDMbJwlsMw0nCap3iuYpgheY4uiaaKDDUM5vnecp0DUNp3msNw5nuaw5DULg5oEMaIo6VwgnENw0EwMZ2DWFwyGycQ2C2cQ1DUY5Wh6t6hlaKpnnoMQ5rKXA0FqQpPjiAQ",
        categories = "time",
        tags = "date,month,year,event,heart,favourite,subscribe,valentines day",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[cfg(feature = "calendar_minus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAeIJDMIggHKKQiDKLB5gmEAgGgZRpGcaB0gkMQ4iwdxpGSBY9j+EgviSJoXgSGYJE0MwgDEMBoj6IIUkyS4FgeTpSlENhohCR4igE",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "danielbayley"
    ))]
    CalendarMinus2,
    #[cfg(feature = "calendar_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiF4EhmCRNDIMQgDEMxWDYYQyg0IAwjKMgtDKNhIDWL4xjOPY2h4MQ0juMI9jSMAyGgOIghSJIjgWB4mDOKgwGgMZKhKTIWgKJJQhuHQxDkaA2kuIoBA",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAaBlGkZxoHSCQxDgIggHcaRkgWL4xCAeYJhAIB4gkM4yHKPwiDKIAviSJoXgSGYJE0MwgDEMBojCR4igKTIHk6UpRDYaIQhKFJMkuBZahsMpRDSH5hiKAQ",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "danielbayley"
    ))]
    CalendarPlus2,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiF4EhmCRNDIMQgDEMxWDYYQyg0IAwjKMgtDKNhIDWL4xjOPY2h4MQ0juMI9jSMAyGgOIghSJIjgWB4mDOKgwGgMZKhKTIWgKJJQhuHQxDkaA2kuIpbk+GhNmCKg2HaY5YiKAQA",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmDQ0CIPg8C+A4FikcBhgsIBkg0TQxDYIAyHaJ4pC+L4Li6MBojKNAzCAMQwGiGo8j4aJAjGMwiE0OI4jqKIqkyTpClCNQ3kYNBoC0NpWj2QZZkOUQxkWDxIDeY5YDyTJnE2XQxl8LgwDGbplnCQZyDGdA4Gid55kuZYBA",
        categories = "time",
        tags = "date,day,month,year,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[cfg(feature = "calendar_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIVg2GEMgghQMAghcMAtDKGxIDWE4VhiIoahQMh2DENIghaI4hDIaA3C4NQiD4PAvgOBY0jeBoIgoMQ2hUdg0jONY6jmBI7gkTQ4kCQo0jaR5GgWB5JDODgwGgMQ4kOT44DwYxpHIYxsGUIBjHiCZZCKZR5miWggHKCQzluX5hmOUZICIbQyiWHAxjELZ+jKTpFgEA",
        categories = "time",
        tags = "date,day,month,year,events,search",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarSearch,
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAch4gkMgiCAeYJhAIB3GkZIFgkMQ4i0aBlGkZxoHSN45CCKgiDOIAviSJoXgSGYJE0MwgDEMBojiR4igKTIHgkbQxDSUQ0C2XoQhKFJMkuBZaCKXAwl8IJilaZoBA",
        categories = "time",
        tags = "date,day,month,year,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDENoNg+EYThiEhyGUYx0CAeIJDMIggHcaRkgWCQxDiLB5gmEAgHKKQiDKLBoGUaRnGgdIyjSEgviSJoXgSGYJE0MwgDEMBojOIIUkwPoBA",
        categories = "time",
        tags = "date,month,year,event,birthday,birthdate",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgyCKBwygoMoMCAeIPguEh4gmCwiD4PAvgOBYcHAYR0GgIBkgoTQ3CANxIDQYQyCCMAwCCMwwC2MAyHYOYvjGNI+jOOBoDENobh2IokiGI4licIhNDkLg1CANBoDUTAxioNxoDOPIyj4MY9jkN5QkUL5HGiSYkiaKAxDQLgxDKXwxDWbpwEEMwgneM5xCCTw4DgIAxDALp+mSZg+gE",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CANBoC0NRMDcIA3EgNBhDIIIcDAIIfDALYcDIdg5huHYgiqH4kGgMQ2iiHorh2IxWieJIziIMojhIMxsiODgtDMegiD4PAvgOBZGGMaRyGMbBlCAYx4gkMQyCKUh5lUM5YHKCZckYL5Mk6UA+gE",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[cfg(feature = "candlestick_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CANR2DQIg+DwL4DgWEhyGUYx0CAeYJDkIggHiCQ3iAcoiCIMYgGgZRpGcaB0gkNogHcaRkgWCYQhIL4YhqEoVgaCIKgwMYODKEYTj+PoEkCCRNDENwgDMdpGjqSQ8jyG4ri2L4JDiM41jcIoQiGCZEiCHQiDWJYnimOpYkqBYHk2TwgDGUgzkeFJLnCTIKDOUR2DEOBooKeZJgE",
        categories = "charts,money",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,exchange,hedge fund,statistics,diagram,graph",
        contributors = "danielbayley"
    ))]
    CandlestickChart,
    #[cfg(feature = "candy_cane")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NwgDIMRhDKDwgDCFQgDELQzC4NQtDIbA4C4NgtDENBhDYIIohaFgxhgMAuDSKYUhOLIYhqMA2DSHozheLQwjcNI5h4WgiD4PAvgOBZGkmBoIgoMQ3g0NQgg4MZTDILgxkWR5MkuBJNgkTQxi8OQgDQLg4hgMwgDmW5Il+XoFgeCRtlGZQ5g2FJnDSbpdgKX5zgqZ5liWeZViGbZGm+SoBA",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = "danielbayley"
    ))]
    CandyCane,
    #[cfg(feature = "candy_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ4C4NQggwNQtDEIAxGENAuDmDoXhkIAwh2Hw3CANxsDGEgiD4PAvgOBYoiuBoICITQxDELg4DQMwgDYLgxDgNxBhsNIhkCIYeh6EwxjqDg3g2FoYkEIJDh+RoUC4MwxjiFw2DUN4nimLotgSL4JjINIUkkVgxDSXYqmGYIFgeY5pjmDR2jSNgzmuX4CmGcIxDGRgwHaSw1nmbZ7m+MBtkiIoUC2Do0DMNpLDYOAzEGOI4kWHwxDkLg3DAOAgDMSAyDGdQuDIOQyGGmJSh+VQxiGkA0DETAyDKIhso6FKFiyh5igqoaxhIIIRpCkgtC6lKWq2moeheqa4qUSAzHaEqoqqrKiq4MLJlYN7XDOtK2hSI7Gr0aIoGwaRuGUIB5DGCQyCK7wyvK8wgHi9gire9B4vG/Jruu7Q+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQyDEIA4C0MggDILQxC4NQtDMLg3EGEYRDAIIfh8MQ1C4Ng0DYIA1EgOAuDQYYdiCMQwhQLg5DAMwghUMg1DcTA1jmH44DgIg+DwL4DgWRZIgaCAiE0N45DQaAuDAMZEkaS5KgSTIJE0MZQDGUpUlaRZHluRRyGUYx0CAdxpGSBYJDGQwgGgZRpGcaB0gmdB5nIMAiCAch4gkMqBoQIgzlcL5pmuWoFgeXY/nMdqGmWWYClukZODEOY5DilaLlmAQ",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIAyGgNAiD4PAvgOBYThaBoICIbQyDEIA4C0MoOC0MQuDULQzC4NxBiOI4NjAIAxDULg2DQNggDUSA4C4NBhi4IIxDCJQuDkMAzjILgyDUNxMDWMoNkgOIShSGYYgSGoJE0N4yDQaAuDAMZUhWWJXgWB5aDGXAxl6YJihOZIXDwchlGMdAgHcaRkgWCQxlMIB4gkMwiCAeZ9DChByoEIgyoQaBlGkZxoHSCZTnCdJ2maWYKk+fh2o2cJWgKWJogoMQ5jIOKfmOVoBA",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAxDcaAyGMLg2CAMIOC0Lg0hgMR2C0MxjDCGQ5hkNwtDELomigNYnC4ORDDEOIpg4MIUg6FQxhcMY4DAc4tDOLQ0C0MguDKQwuiCGYshqLQxiWLQ4iUSA1GOGYViKKIokKWouGyQQgkQORBDOM5kDeFpohcMoODIdg0iEIIUhqDp0DGEQiD4PAvgOBZ5GMaRyGMbBlCAYx5gmDwioUeIJDeihygkMp4nqf6BoOeZ8gaCIKg2DxoDak57gQaJ+oCgqEGOjAiokIKQCKkqFoeq6OnkL6VqcPoBA",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[cfg(feature = "caravan")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDmDBoGUaRnGgdIJDSDB3GkZB0GiGAiD4PAvgOBYiiWBoShSFoJDEMIMggIougyDgihAIIbh2HwihmIokgQdIiHAYYeCAZIJE0MQ4CAMQ5FYORhDQIJSDAIJVDALQ0lkSA2lGU5WmCWJSDQdg4GEMggmiV5gmgMhoguPpDh6IhjGkchjGwZQgHKCYLCAY4xDiDBjjWTYhiOdZ3nmQpEGiRpIi6TA5GgMQyHYLZwiOchoD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMDcIg+DwL4DgWB4bHAYR0GgIBkgwTQxDAIA5HYNoahyIokhuHoGgiFoMDGGYJguDYPhGE4wh2BI2iGI4licIopDQIA3HYOJCjIaA+gEA",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQ1CANAtDiEAgDgIg+DwL4DgWGIbgaCAiE0NINDMaA2heGYehgYxpHIYxsGUIBjHmCQxDIIoyHiNYWCAcoJDOKAviyLowh2BIfgkTQyDEIA5HaJ4YhqRw+gEA",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDENoNHmCg0g0dxpGQdBogoMgwg2CYLCIPg8C+A4FikYxpHIYxsGWB4ig0Y4ZCIMYkCAY4mDiKIqi+MYzikcBhiAIBkgoTQ4CAMQyGiQIpC+R4gi6MIyjQco2j2OY7jeJoWkEL5DlqRpIGiSoKG0NggiMIAuDcLQyC4ORBDELg0k+eggDCfpPCAOAuDGTw2GgNwuDgYZ5nujaAn+haPDEbJyCAM5klYaA+gEA",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = "danielbayley"
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAOBWDYYYMgwMAghQMQgDILQyGgMYQhKFYgheEh2DEMoRhiIIWhqGBoC0NgiD4PAvgOBYxjSBoIgqDIlGEOQgj6FIWg2DYwjKN42gSOIJgsIIdGENQglCQZNCANJVkWM5JjEbBpG4ZQgHkMYJDIMAiCAeJiCIMpmHkMpjmWZ5umoLgwDGWJcl4PoBA",
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
    #[cfg(feature = "cctv")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAORoC4MAxCIPg8C+A4FhSF4GgiCgxDYLg3DUIAxDISAyDIbAtDMLoiDeKoQDkLQ0C4MwyhOFYahmBIbgkTQxDiDYsikNAgDgLQxDCNIxDULgyGEMguDkMgglCUggDCV4jkeNIvDkMRMisNg5CCTA2k+UZTlWU5YlgMQgiuUpbDMWo3haO46gWB49lMMYOisN5mmmWZsiMLpGDELpfmMMQ1nWOYCjueoKmkMR2jKjp3gEA",
        categories = "security,devices,communication,connectivity,photography",
        tags = "camera,surveillance,recording,film,videotape,crime,watching",
        contributors = "danielbayley"
    ))]
    Cctv,
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0OQgg4IAyiwIA0C0NIZhuIYjD6AQ",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh3GkZB0GiDoQhIeIVhIcogCIMgiD4PAvgOBYoHAYYbCAZIOG0OQgDEMggjiOA0C0NInimLobD6AQ",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQzC4OA3EENAghQMAghcMQgDcLg0hoNhhDULgxhqIokhiKIaDELgwDULYrDWFA1CCM4XhmG4sDiGBBiaJYjhqNoNg0NguDUOQggyFIWimDY6g6EA3FYMgxEgNhaCIPg8C+A4FlkbBpG4ZQgHgMoJDEOAiCAeQxmYN5pHibAiDaaR5mUIgxm6WQvl+YQ+gE",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHIeINDKER5g0M4Rh0IojD4PAvgOBYoHAYYLCAZING0MQ2CAMQwC0NAgDSOo6CKKAvi6Cw+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDQIAxDYLQ0i8IIwDQIobC+IokD6AQ",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[cfg(feature = "chevron_left_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEaBlGkZxoHSDYPhEeYUhEcoTCIMgiD4PAvgOBYoHAYYLCAZING0MQ0CAMQ2C0NI6CCOw0ieKYugsPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQwCAOAgDSLQti4NIZhuIYjD6AQ",
        categories = "arrows,navigation,shapes",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    ChevronRightCircle,
    #[cfg(feature = "chevron_right_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOG0MQwCAOAgDSOQtjoNInimLoXD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQ4CAMQ0CANAti2MQihsL4iiQPoBA",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[cfg(feature = "chevron_up_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIB3GkZB0GiCQxDiDByggIgygwaBlGkZxoHSF4ZD4PAvgOBYoHAYYVCAZIJG0OAgDENAgDQLY4jwIooC+LoVD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bh6BoIhIIg0haC4Ng+CYYioIobh2BIjhsbBpG6CB4DGDAyDELgxDeEB5DKDA4hAeJDi4IB5joIpFjCNY3jSNo4kgOAuDWJ5KkwNguDANpGkwMwuDkNZBkgMYnk+U5SjeSpnlmR4TDULg0l8II5hMMAuDiRZajuPQ5mmHJQGUPoBA",
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
        svg = "gAPBsGkbhlCAeAxD0IgyCKBwygoMoMCAeYJguDR5g+C4MD4PAvgOBYcHAYR0GgIBkgoTQxDIIIqEgMh2DQaAxDQIocC+IokiGI4licIhNhGLIvjSNo4GiOokiaKAxDiQRoC0Lg1jWHZFkePIoDeQYwlKN47lWSY+ksIA4GMMAtDKUJmmaaAyC2UZEl0PJFl+P4rmOZZnDWaZ4mmbpTl2AQ",
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
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CITQxDALgxCAMoODgYQ5C4OQzCCFYXCAMIcCCDwzC4OIcCIPg8C+A4FiaKYGgiCgxDcLg2CCIQ3DGFIWDWGY5h6HYPhENg5hALg3iWJ4siuBItgkTQyDGIgyh+DY3hqGJVj2H4eiEOJGiiSpJgWB5MDKDQykKMIyjgOY6hqOodj4LYRDeQ5Bl2SICkqYoviGQpOlCag0jsOaBm+HwtluJIml6Kp4mGLhNDYLqBmQLpmmqbI8oUMZxjIOackWip3iyepNhKH58mqVoWhimocoeIp2l+jZLgqNYPpENKXoKboej+RKcnWoayGMaRyGMbBlCAYx5gkMQyCKyh4s2zwgHKzZdsSxrID6AQA",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = "danielbayley"
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCIWg2EIKhOD4RiiGYbiGH4B",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQxDcIIOGgLgwDEIobC+IokiGI4licIopDKLAyi6MIyhyNRojeJImiiK4ti+MYzkQPoBA",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwGiDgiD4PAvgOBYThaBoIgqDAxDSEAwhKFIZhMYxpHIYxsGUIBjHiCQxDIIosHmL4xCAcoviGEwviaKIqD6AQA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MgyCCKosg+GgviGIw+gE",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDUIoHDKCg5g4eIJCKEggHiEILg0Pg8C+A4Fh0YxpHIYxsgYYx4goMQyg4Yx5iuLQgHKKwwCKHQviOJYnD6AQ",
        categories = "shapes,development,maths",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[cfg(feature = "circle_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDAYQ2CCEgwCCFQwC0MQyhYIg+DwL4DgWHhjGkchjGwZQgGMeIJhoIoqHmLQwi8coJDSHYfiSJooiOJYnimNQiDGM4qiyQgyi8Y4xkeOAvjqPw+gEA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUserRound,
    #[cfg(feature = "circle_user")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGodgaCIKheFYMDOEBjhKDYPhqHIEiKGhwGEdBoCAZIME0NwgDIMAuDYNgyFYMQ5GEMo8CAMJJCAMY8C0MhoDaRpIkqSpNkcMh2DGP5BhmG4zjUPoBA",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUser,
    #[cfg(feature = "circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWBw+gEA",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[cfg(feature = "circuit_board")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKE0MQxCAORoDQYQyCCPAwCCP4/DILQyFaJ4pi6EooGMaRyGMbBlCAY4lCIOYNGOIZWgeCoMigL5Mk6UIti8aIxjMN49DEdgtjqPI+kAII2kMMo5CKXpJGiS5Nk+UZThQNYNHKXJXiEMaAl6YJ8D6AQ",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDQIggHiDg4hEeYODKER3GkZB0GiFIRHKEwiDGIYXiQIg+DwL4DgWKhwGGHggGSDhNDENggDQaAyGEMggj4MAgkEMY/j8dgxDSPZFkGQwtj4MhIDaSpAkIIAxk6ThWlKT5VkOP5OjuKYrjCHovjEaIzg4bQ5lYNJFj4NAthCKgvmQaA+gEA",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgxCIIB3GkZB0GiCg4g0eIUg0aBlGkZxoHSCg0g0coXguDYJCIMgiD4PAvgOBYrHAYYSCAZIKE0OAgDQSA2GEMggj4MAgkEMAtj4Mh2DENI9j+QpNkGRhoDEMpLkCTo/kUdpFiqLIxhKMIyGiNI2DENo5GiU5GlYMZMkeIYrC+XRol+M41CITQymuSRIDGDJvnGc5hnUbQxDUIAxkQNI5oqbpcmAPoB",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIB5g0MoQHKEgiDGFR4g2GQgGgZRpGcaB0g0NIQhuDgiD4PAvgOBYrHAYYLCAZINE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PIqiyMYLjCMhojSNgxj8MQxGiJorC+ZRomeM41CKN5tDacJjnOaJ2mmeBNDiV5vC4MIZnKdKBmqeaFjgaKIoqZKAgE",
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
    #[cfg(feature = "clipboard_pen_line")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIB4g2DwgHKEgiDGEBoGUaRnGgdINDSEB5g0MgiD4PAvgOBYoHAYYLCAZINE0OAgDQSA2GEMggjsMAgj4MAtjsMh2DENI6jyP5Kj6QxoDEMpIj2S48kIdgtC4NYnimLoLi2LxojGMwxDaNholCQ5TDEIAxC4NwzmuWgvlwaJejCMgijSaw4k6cZznWYJ3E0MQ4C4NAgDkLo5miQJrCCbwzEwMQ3msNxsC2hpqDGlxan2Xw+gE",
        categories = "text,account",
        tags = "paste",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardPenLine,
    #[cfg(feature = "clipboard_pen")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDSEB4g2DwgHmDQyhAcobCIMQiD4PAvgOBYoHAYYLCAZINE0MQwC4NAgDEMguDYYQyCCPwwCCQgxCAM5GEwOJADEbAtjiRQxk4WonimLoLi2LxojGMwxDYIA0GgMo+kCQ5lkWPwyHYMQ0mOQZmC2aBoC0NQuDWVAvlYaJYjCMgiE2TwznUVo9miZZEkCcJhneeQ+gEA",
        categories = "text",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardPen,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgxCIIB4goOINGgZRpGcaB0goNINgkIgyg0coPguDR3GkZB0GiEAiD4PAvgOBYrHAYYnCAZIKE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PIqiyMYnjCMhojSNg5lcMh2C0MRoDaSJjC+ZRomeM41CKN5FDEN5iiudponmaZ7jePwxDEdg2nWdw+gEA",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCKBx5gqDAgGgZRpGcaB0goNINg8Igyg0dxpGQdBogoOINgkIomD4PAvgOBYrHAYYjCAZIKE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PAiisL4xiOMIyGiNIKG0MQ1leWI5DaY4smYaJojONQiG0OZwCCcp0mWaQ+gEA",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDQIggHIeYODGER3GkZB0GiDg4hEch4hWEYUCIMoRiEIoeD4PAvgOBYrHAYYbCAZIOE0MQ2CANBoDIYQyCCPwwCCQgxkCQB2DENI+kaQpEC2PwyEgNpLkGQwgDGT5PFaU5QlaRJAk+PAiisL4xhsPoBA",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDKMQ0C4NQgDgIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4OIxhiGgviGI4licPoBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDIIA5C4NQgDgIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxg+GgviGI4licPoB",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDKMYwhQIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GwaRugiIYmHQc4TDIIA2CCDoxi4MQ2C4NYzhmG4hiOJYnD6AQA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDKMQ0C4NYxDYIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDKM4xDYLg1CKGwviKJImigPoBA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCFIxDIIA5C4NYxDYIobC+IokiaKA+gE",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDIIA4jENAihsL4iiSJooD6AQA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAbxsHkbBpG6CIiicdBzhkIA2CCDoxDKMYwDENAihsL4iiSJooD6AQA",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEN4OGMeISDIIg+DwL4DgWB4bHAYR0GgIBkgsTQ0C4MggDENQuDEQQ3CCMwwi2N4uC4NwxCAOBoDGOg5GGKg1CCRAgjaNo8DILpFDiK4ahyIokiGI4licIhtjmMwxk8NAtC4OZgg2GwvlMaJViSJoLG0OYri2L5imGY5RmaVpplebAxDALg2CAMp8DcLgzmAOZ1meeJrlkMQzjCLYqkyhJhoedw8meipao2fqAjqYJfDGlJUpaVqYnsLg4o+g6eC2oJloio5qlgbZPDOLZPn4MaeqGaKwnmi4qlyL6orkLg0rsPoBA",
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
    #[cfg(feature = "code_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAch4gkMoMiGCgiD4PAvgOBYoHAYYRCAZIJG0MQwCCNQtDIII6jyJ4pi6EYti8aIxjMMQ0jeSAyjmTIjigL5AGgPoB",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = "danielbayley"
    ))]
    CodeSquare,
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
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDIIITDKEw4C4NYUhMMQ1hkIIShuFIgh6GoXh+IQyCIPg8C+A4FgcbosGyDBlCAeAyhCEgiCAeQxhCFo8HmOYRiWPB4j+EYqiwL40G4ZYsi8eZOjaC4Ng8IoWCCGIaiGHYficNYri2UpUlGBJTjWChvgyDpAiSKJhiKX5ikyZY1jOapIjqKo9kmfZDhCXJHkSO5MmaAQA",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[cfg(feature = "codesandbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDYVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoiuI4vkOM4Mg4Nh6CIPg8C+A4FlAcBvGweRsGkbhlCCVpbHQc4JDcLg1i4LpLDGFA2C4OI9myZg0mgMZPlGVpYlqXJVleWZbl2XxumGY5lg4OYeDkIJkmYMZyDaNIODKdZSnyeRlnueJ+l4b5gmIIpphSD6EowLqOqGi6GDcOaSnefZ6gKlKZoCggiDOaItmwOalqAMguDCDQyDCPwgrgNqrrCrqVCAeQxgkMq8DAOAiCAeLMCKarSHgMoJteyratakZQC+lQ+gEA",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIA4GgMRhDQIITDAIAxhcIIWg8LQxCIPg8C+A4FiCI4GgiCgzg6EA0HYOYShSGoyDELYTDQSA3jCFYzjWNRah+IYmiAbBpG4ZQgHkMoJDQIggHiSgiDaTR5DGCQyk0eJVlGQAvkSRpDkWR5JkuWJQDEMJYlqZ5TlqV4gl2YZgkaTpmkySJQnaWYJDGdpUlaXJeGUPoB",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIOAigkeIMg4IBygwNgiD4PAvgOBYHhkcBhHQaAgGSDBNDEOAuDAOQgDEMAuDMNxBDYII0DCLY4i6MA0i2DoZC+IIih+IYjiUIhNDeNRoDEdg0hiGpBGiQ4iiSDBtDENguDcMYtDMLg4ikN5aDELQyl8MggmYOAyk+QJED6AQA",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[cfg(feature = "columns_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByggIgygwaBlGkZxoHSE4VD4PAvgOBYoHAYYRCAZIJE0MQyCAMx2hQIooC+LoRD6AQA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,parallel,series,split,vertical,horizontal,half,center,middle,even,sidebar,drawer,gutter,fold,reflow,typography,pagination,pages",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns2,
    #[cfg(feature = "columns_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEch4g0MoRhgIoVCAaBlGkZxoHSDYPCIPg8C+A4FikcBhgsIBkg0TQ5CAMx2ieKQvi+C4ujAaIyjQMQ1jeOYQjuPRoD6AQA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages",
        contributors = "danielbayley"
    ))]
    Columns3,
    #[cfg(feature = "columns_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHmDgzhIeIdhIcogCIMgiD4PAvgOBYoHAYYXCAZIOE0NwuDUIAzHaGooC+LoXi2LxojGMwxDKOI6hGPI+GiQIwjIIhNDENo2keO4pksPoBA",
        categories = "layout,design,text,security",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages,prison,jail,bars,sentence,police,cops,cell,crime,criminal,justice,law,enforcement,grill",
        contributors = "danielbayley"
    ))]
    Columns4,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAdxpGQdBogkOIMGgZRpGcaB0hODBygiCoMHmCYLD4PAvgOBYlHAYYRCAZIJE0MQ0CAMhjDELgxCAMI0CALg5jwMh2DQY47jcMQtj6NAtDKNAiiUL4rhGKosGiLowDKO41kaOo8kmTJBkORY4kiPwykuTZPlEaJTi2LwijGRQ4EgNRjC2Nw3joLQznYLp7n4dp2k6JpqiobxsHkbBpG4ZQgHAb6KHQc4JngMo5DGcQgngMQ1oKUKGoiihliWKIGh+MohgmpwghaGIahwIIeiODIOhCEgihST6kD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GcbxuCCIRpG4dBzhMNguDINAgDcLg3DYIAxDQLoOjiOo8jSNo4i+MQgDkLg4DiRZHkkMZDjKQA2hmG4hiOJRuD6AQA",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggHmDQyhEeINDWERoGUaRnGgdINDiERyhcIoVD4PAvgOBYoiuBobh2H4hhaFIjiWFYSg2D4RgiCoMiYMAiiiKoEHSKBwGGCwgGSDRNDYIAxDgaInimSILkeSRokuTQxDKUJSDaQpVlkPoBA",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CIbQyDALg5CAMQ4C4NQ1C0OAtDENYODgYQxhAIAwiCIIYC4OIgGyFoQhoOQ4CIPg8C+A4Fi8ZRsGwaRwHMZQgGMeYJDEOQijweI/DKQhyj4IgzkeRAikGLwvjWN45GUPoBA",
        categories = "shapes,maths",
        tags = "conical,triangle,triangular,geometry,filter,funnel,hopper,spotlight,searchlight",
        contributors = "danielbayley"
    ))]
    Cone,
    #[cfg(feature = "construction")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDgIggHiDgyhEdxpGQdBohQMIRHKEwiDGER5g4NgiD4PAvgOBYoHAYYaCAZIOE0MQ3CAMQ0HYN4nimLoai2LxojGM42jiOo8C+PhokCMIyCKNI2DMdgzkiSpMkKThNlGU5VkGV5Dk8MQwjcNAgDILgzCANpol2Pw8kqYBtjiaggDcLo2naO4okmXpvkGcQ4nSgYQnuVoBA",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAxDgYQ0CCEgwCCFQwC0OIWCIPg8C+A4Fh0YxpHIYxsGUIBjHmCQxDEIggHKCQzi8Yx4iwMoch6I4lieHRyGUYx0CCNgijMIIrCINIvHKRI4CAdxpGSBYsDiLxoGUaRnGgdJUjkL4/kGHRsGkbooHgMYJlWR5oCKTh4DKaYvHmcJJl6Y5lmKZIoHmbJunSDJynSSpDmygIdC+dxlD6AQ",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcIAxDgYQyCCEgwCCFQwC0MoZEgOYRhOFoghiEgyCIPg8C+A4FiYchlGMdAgHiCQzCIIB5gkNI0HKMQiiQIBoGUaRnGgdIJg+NB3GkZIFkUOIlieLIuiYYxpHIYxsGUIBygmPRjjsMZcjYIgxDCTgvlOVZXiYbBpG6WB4DGCZNjAMpxjQeZ0CKOI1nCPJlmubZqmyWB5nyPR4ngMQ2naeJ6m+RaKiYL5/GUPoBA",
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
        contributors = "danielbayley"
    ))]
    Container,
    #[cfg(feature = "contrast")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME2DggDEOBhDYIIuDAIIxjMLYOHaDh6hmG4hiMPoBA",
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
        contributors = ""
    ))]
    CookingPot,
    #[cfg(feature = "copy_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDUIINg0NAtDQIg+DwL4DgWGByGUYx0CAeIJDgIggHmJImHcaRkgWCQxhYIByigIgyiYaBlGkZxoHSL4xHKI41heGYeiCGIbgaCAiE0NIODYYwtDELgxCAMAtDILQuDmV5XFYNBjlaUpSluEpXGgMQwGOYpVhEIJam2NoYhqBBoD6AQ",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDIIggHkMoKDENYOHmCYLhSB4RgsOAiD4PAvgOBYfHIZRjHQIB3GkZB0GiEg0g4aBlGkZxoHSL4OHIeIKg2D4Kh0IByHmPIOjsIodh8L4lieHxwGGLQgGSChNDQIAxDYYwtDELgxCAMAtDILQuDmYJgFYNBjl+W5bmQMggmEMhoDEMBjmuXpvCCY54g2SZOi0PoBA",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAxD0IgxDUIggHmCYLDKDh4DKCoMg4eYWgsOAiD4PAvgOBYfiKBoQheDYHhEMYTg+G4YgeL4dh+IYEGWHxyGUYx0CAaBlGkZxoHSFw0g4dxpGQdBokSDhyHiCotHIeZQhmCodgeVoeiCOY7h8cBhkoIBkgoTQ0CAMQ2GMLQxC4MQgDALQyC0Lg5nGcRWDQY5wmybJ1DIIJyDIaAxDAY58m+gAgnSiYTjSX5KD6AQ",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDgIoHDGCgxDKDh5hCC4NCAeYJguEw+DwL4DgWHhyGUYx0hmCoYHIeIKhMIByHmLYOGgZRpGcaB0hENIOHcaRkHQaI6g6LAig2HgviSJoeHAYZACAZIKE0NAgDENhjC0MQuDEIAwC0MgtC4OZel4Vg0GOXZZlmYgyCCXwyGgMQwGOaZcm0IJhnaHYfkyQA+gE",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDgIoHDGCgxDKDh5gmC4NCAeYQguEw+DwL4DgWHohgaGoRhiFYRhOD4qg6CInCKHoggQZYeHIZRjHQIByHmCorj0IoYGgZRpGcaB0hENIugqGByHiPoOHcaRkHQaJJjGH43jmHhwGGVQgGSChNDQIAxDYYwtDELgxCAMAtDILQuDmb5vFYNBjm6apqnMMggnAMhoDEMBjnqbZ+CCcqHh2H5dlUPoB",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENAiCAeIODiEhyhQIgyhceYOhsIB3GkZB0GiDoQhKHQihYPg8C+A4FiwcBhiQIBkg4TQ0CAMQ2GMLQxC4MQgDALQyC0Lg5kSRBWDQY5Dj+P5IDIIJFDIaAxDAY5PkKUwgkeXIbiwL4yiQPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ0C4OAzCCKYrDMYQ0CCMwwi6NwwC0NQuDYNoZhuIYjD6AQ",
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
        svg = "gAPByGUYx0CAeA9CINAiCAdxpGQdBogkMQ2gweYJgsIByggIgygwaBlGkZxoHSE4VD4PAvgOBYoiuBoOhCEgihUIIXCIOYfiGI4ljODIcjiKIqgQdIoHAYYRCAZIJE0MQ1CAMh2h6QZGhGRZHGiSZLk2TwwlEIpTleVpIkoIhNDIIJNGiUoplQaJiliZJmCAOZql+bJhDybZZmUMgwmgNZ1mCVZ5lee5mn6dJrC+bZvoYOZPl6gpuoSY5Lo+faRneVYB",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCAOQuDMYQyC4OAgjCMoqjYLQzC4NQgDEII5j2PwgjaQpEjkNIzC4N5IkqQ4qjmOwxhmG4hiOIIiiSJgiiiSosi6NJIjWRAwjiOo8j4LpAmiYprkaS5uk2Z5QlIL5UGgPoB",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAdxpGQdBogkMgwgweYJDWDBygiCoMGgZRpGcaB0gkMQ0CIPg8C+A4FikbBpG4ZQgHkMolhWB41gqC4HDGE4Wj0IgxhWKQvi+MQ+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhsGkboIHgMYMDKDwgHgMoTDiEB5iaDYpHmLIyhmG4hiOIIiiSNYzjWFAgjCE4piWDA2jcL45GWO4jkKMZIkKPoQiuRJUjGFIakqPJNj2VpSi2V5fkMIookmSw+gEA",
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
        contributors = "danielbayley"
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwOIQGMeYMg4Ig+DwL4DgWB4bGwaRuggeQygwNoQHkMYMDOEB4iwIouCAeInCKKYbC+IokiGI4ljGM4miiL4xDIMYvjYMYUjmOxlj2JI0kCKpFkeNI2ikIJCg2S4ck2T4IjCDJGiqSYUlaGJmiuYpHkyPg+gE",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "cylinder")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch4D0Ig5CIIBjgwIgxDKEByHmDQzhAY4ZCINQiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUdgxDQYQ5CCMgwCCPI8DEOI9FaIIiC+KosD6AQ",
        categories = "shapes,design,maths",
        tags = "shape,elliptical,geometry,container,storage,tin,pot",
        contributors = "danielbayley"
    ))]
    Cylinder,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx4D0IgxDIIoKHmDQ1hEcoMCIOYWhMIgzCIPg8C+A4FgcZYgHAYR0GgIBkg0TQzCCDxhDkIIwDAII3jcNQgDILg2hqIAviiKonimK4tCITQyDEIA5C4MxWhWQZDGiRYqiyLowDUdgxDSM41jiYY3DYLg0DePAuDgOIfiGVJWkeLoPjEMh2DQaA0myQpGm+WJJDGMAyDAYY7juOZhDkLYwDQLo7ouhZiC2jqRowYwtDGT42C2PQ1o4NKJmWTKXDQMRMnIMQ2nmboBA",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAch5D0IgzCIIBjHiDQxDKEByhMIg5hAY4MCINQiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUVgxDkQQ5CCMgwCCPI8DENQgDIMQuDgNIhiOKosimK4ti8IhNkOMxWDiSAvkoaJMiyLowlKFRMDEOAgDENxIDIMpgjmZ5WliWpOjCMoVjiOo9nWPw0C4NY5DGeA4DebJND6AQ",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "gAPBlGwbBpHAcxlCAYx5D0Ig1CIIByHiDQ5hAcoMCIM4QGOEwiDEMgiD4PAvgOBYHGWIhwGEdBoCAZINE0MwgDUVgxDkQQ5CCMgwCCPI8DIMQgjYVoPiIL4qiyKYri2LwijGQgyjiOo9lSP5Bh+IYjkgaA+gEA",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA1EgORsC0NwghSFhoDEMRhDIIIcg2H4dC0MhWDeG4dCCIAwiKIhaCIPg8C+A4Fi8bBpG4ZQgHgMYJDEOAiCAeY7CIOY/HkMo8DWPx4kcIgxDKLowjWN40jaOJBgmRJAkwMZJjmQpOkqW4+i8L5SGUPoBA",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDGDYPHmCw0CIPg8C+A4FgeGhwGEdBoCAZILE0MQwC4MggDOKxDDULg1CANAgiwOAuDGNggDEMxhiyLAwCCQpCjUMB2C0MY/juRI8jSQx2DSS5BkOVZGkiUpAlWQo6leSpak2RZDGMMAtDQLg5C2Lg1C2aY4DKbQuDiGYbiGI4giKJImCITYuiwMZnDgYQ5CChJhjwNwuDaQ50C+dhoD6AQ",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = "danielbayley"
    ))]
    Dessert,
    #[cfg(feature = "diameter")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDEOYOGMeYShQPg8C+A4FgeGodgaCIKgyFYRCINYVheKAihqHIEiKGhwGEdBoCAZILE0NguDQOAgDMLg2DYYQxDAIJFCCRpGDGR5ADgNpNC6T4thuM41jKNI2jgIhtjsNJMl6TAxDELgxj6Y5lDiVAvlYaJYjWN45kCQggl4OJEkuSpJnsMZOlCfZSDaa5tD6AQ",
        categories = "shapes,maths,design,tools",
        tags = "shape,circle,geometry,trigonometry,width,height,size,calculate,measure",
        contributors = "danielbayley"
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEhyhQIgyhIeYVheHYahIdxpGQdBog6EAiD4PAvgOBYrHAYYmCAZIOE0MQyCCOBoC4MAxiqLIxiYPoBA",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHKCAihEIB3GkZB0GiCQxDiDIPgqDBoGUaRnGgdIdh8Pg8C+A4FiwcBhhsIBkgkTQxDUIA5GgLgwDEIosC+MobjGMxojWNw5CCOY9j+QYtkQaA+gEA",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIBoGUaRnGgdIKDEOINgkIgzg0eIKhsIB3GkZB0GiFYXgeHYLCIPg8C+A4FiwcBhiQIBkgoTQxDYIA4GgLgwDGK4tjKJIxjMaI1jcMQyCCSo9j+QQvkMaJFjSNgiE0OJMDaTpAiyUZGD6AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCKBx4gqDAggkIgzg2D4Ug0dxpGQdBogoMQ4g0aBlGkZxoHSH4hD4PAvgOBYrHAYYdCAZIKE0MQ2CAOBoC4MAxCKKwvjGHYwjIaI0jYOI6jyPpAiyQxokWM41CITZKjiTI/kGUJSkeVI3jmWI9lqT5GD6AQ",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMg+CoMHKCAihEIBoGUaRnGgdIJDEOIMHcaRkHQaIhiMPg8C+A4FiwcBhigIBkgkTQxDYIA4GgLgwDEIosC+MoojGMxojWNw4juPY/kGLZEGiRo0jYIhNkuOZNkCQpRlOSJVjiOpZj6W5QkeXpJlYMQyCCa5ak+Q5ngE",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEhyhQIgyhIdxpGQdBog6EIXHiDobCCJQihYPg8C+A4FiwcBhiAIBkg4TQxDYIA4GgLgwDEIosC+MogjGMxojWN45CAMQyj2P5Bi2RBokaNI2CKOI6jmT5AkKU5VkiVxNDiO5clGQ5HmCSZYmSTZml6aQ8lOa5jkwNpvlKaYBA",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEMgiCAeYODEMISHKFAihEIB4g6HB3GkZB0GiFYcHKHobCIPg8C+A4FiwcBhiQIBkg4bQxDcLg5DIIAxDQIAzC4NQtkINRhDILgykCSZLCAMJPlEMJFGwLZEDSOwykiSpMlyUpPkWTxMhYIA2iuLYyiSMYzGiNYOE0No+DgaAuDAMZnC+aRomuNI2CITZkj+dJ2niep8m2fqADWZaDneLJ5myh5un8MQ4CAOaNoWkYB",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bh6BoIgqGYQhIIg0haGINhqHIiiAPBwGEdBoCAZIME2Dggg4aAuDAMQihsL4zjUPoB",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQ2CCFBjDALQxC4N4xC0MwuDKKwuDgLQ0jYIobC+IokhuHoGgiF4Zg+CgihWEISgyGockSIA8kGJYnCITQxDiKwyi2OA3C2MQgjUMovjkII8lCQIjGgPoBA",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc_album")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gweIJgsIBoGUaRnGgdIThUIByhcIgyCIPg8C+A4FikYxpHIYxsGUIBjiUMYnjWCAijiDBygkNYoiqL4xjOKRwGGEQgGSCRNjgII4GgLgwDGQgvkiEQ+gE",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "danielbayley"
    ))]
    DiscAlbum,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx4gsMQyg8Yx5hOFQ+DwL4DgWB4bh6BoIheGYWhKDIVgmC4ahyIoggEA",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide_circle")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDYIggHmCYLDKDh5DGCgxhOB4WCIOAiD4PAvgOBYfiKBoVheDYaheGYIiuFIRgyHogiWJIEgYeIbhiDothKFIbh2D4Rh2H4hjaHxjGkchjGyBhjHmLggGMeJQHKFwwjIL5IkqTA+gEA",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[cfg(feature = "divide_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgHcaRkHQaIKDEOINg8Igzg2CYdg0aBlGkZxoHSGIaD4PAvgOBYrGwaRuGUIB5DKGIRHkMY4h+NwiDENofjsIoqiyMYzjCMo0jqGJBjWPpAkKPAgHiUIMisL5HGWSYzlSVobj6Go1kOYh4kMMZXkaSg+gEA",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeILDEMoOGMeYLDYIg+DwL4DgWB4bGwaRuggeAxgsNYOHkMoShQIB4iyDA5iqJ4MhSGwviKJIbh6BoIhaEg4hWEY2g6CoMhqHI9iCAQA",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyGMLQxC4M4NhINQtDILgwDkMggDOGAug0NIhE0OYOg+EoUhGIYOhqHAtDOLYXiMNQiD4PAvgOBY3jqBoIgqHYMGMM4TDOH4xDYLg2DYN4wCAMQwjAbZQjCEIWk+E4XjGGYbDILY0jKNo4j2PIEj6CZTDcIA2iCF4ZjWN45maZYFgeaAxDQIA4iuF4WmKco7gKZp2CIbZqDEOIymGcZkoKdY/G2RIVjSIYhn+jY9oQbQyDAIImpWcJjnOjpnoWSYVm+T5Ppeo6ZpAMank8MIsDGq6Mq2g6vp2sJYhWlq3oEbBpG4ZQgHgMYJDIIggHmyAisqxgysm0B5tKz7KnGwrED6AQA",
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
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHiCQiDGDYHDGCoUHiFoSg0Pg8C+A4Fh0cBhHQaAgGSChNDENwgDUSA5C4NRhDOMQgjQNQgDCOY7joNxojKN42jWOo6DGOw3EgNgih0L4jiUPoBA",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg1CAMQwGEMoOCCFIPDAIIZDELYUDQLQzEgMQ4hMLg5g+FInhqK4chQNgtDQLg0hCG4ZhuKw2C4MwgDcLgxGMLY6C4MpBDiOgtDGQ46CIPg8C+A4Fk0YxpHIYxsGUIBjHiCQxDIIggHKCQzl8Yx5lyXpNC+U5VlcPoBA",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = "danielbayley"
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
    #[cfg(feature = "dot_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAcoICIMoMHmCYLD4PAvgOBYoGMaRyGMbBlCAY4hDGI40iUIo3gwcoTCKKAvi6MIyD6AQ",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    DotSquare,
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
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUdg0GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGx2C0NAiD4PAvgOBYrHAbxsHkbBpG4ZQgjGNh0HOCQ3g6GYUg+Do/DEMIqiyMYzjWN4rkyOB4DKCQxDIIggHmUgiDOVh4DGU5VleXgig+SAvk8PoBA",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[cfg(feature = "drafting_compass")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDGDYPHmCw1CIPg8C+A4FgeGhwGEdBoCAZILG0MwgDIMQgDgLgwDILQxDQLgyDaGYbiGI4giKJImCIbYTC4OQ5CANguDcNAgDGQ4pDMLg0DSOAvjoaI8iOJYLE0MZFhMYwtk8OA3CANIyDALgximZYzCAMJTlWV4+ieK4qDELQymgNpgC4OJShqVI9D6AQ",
        categories = "shapes,maths,design,tools",
        tags = "geometry,trigonometry,radius,diameter,circumference,calculate,measure,arc,curve,draw,sketch",
        contributors = "danielbayley"
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
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDkLgxDMIA1C4MA5EMMYwDIMggisMQ0CCDY9DALg0jwMguDINY/C4OQ0CKGwviKJIhiOJYnCITQyDELg3keFAuDgNBjC0NguDYMgtlgNAxmaRI7j2ZpiDMOAgm+Gock8aJRiSJooDgLg1DYIJEloYw0C4Mw3nKiIrDSOJxDEN5ZnSTpSD6AQA",
        categories = "brands,social,design",
        tags = "design,social",
        contributors = "ahtohbi4,karsa-mistmere"
    ))]
    Dribbble,
    #[cfg(feature = "drill")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA5GMMAgC4NgtC6DQxC0MQgDESA0GEMggiCEYRhgMgtDIVoeiCIggiSIYnGiD4Ti2G4SheGxaCIPg8C+A4FjuPoGgiCgxDgIA2GgNI6jyQZAgSQoJguDQ0GgMxhhqGojjWWB2DKV41lqGJYGgLQzkuPZPk6BYHgkbQ1hsMInCAOJnk2ApPmyRIgDEMB2DOEIShSFobhmGxInSO5oj+d5rkMbQ3huRomoiTJpoyUIKm8MgyGOGQupCcQzp6oqki2FQ2hahZiGgNxjjOLqDlynIRhOFYXqqOaJk2AQ",
        categories = "tools,home,devices",
        tags = "power,bit,head,hole,diy,toolbox,build,construction",
        contributors = "danielbayley"
    ))]
    Drill,
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
    #[cfg(feature = "drum")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCCDA4CAOAiD4PAvgOBYThaBoIgoMoMDILYPhGE4VgQaITGUbBsGkcBzGUIByHmCQ1CKLx4gkMQwjQY42CIMQyjqMQiDmEoUiiKosGWGIlgeCRNDcIAxDMLg0HYNwukOI4ZkqBZMCITY+lCVIihSWoCkuG5fk+UZTlWV5EiSF5mlyaIMDkdg4GGOAgDUIAwn2fwyn4MBWliZIlD6AQA",
        categories = "multimedia,devices",
        tags = "drummer,kit,sticks,instrument,beat,bang,bass,backing track,band,play,performance,concert,march,music,audio,sound,noise,loud",
        contributors = "danielbayley"
    ))]
    Drum,
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
        svg = "gAPBwGEdBoCAZA9CITQ2CAMQ4C4NRhDOEAghMNQgDCDYYCAN4YGMMAtDGEA3C4OQyC0MoQDIIIpDANAthMNA2CIPg8C+A4FjWOIGgiCoMg8NYfC0Lg3DULgxDOIQuDSJAzDaKJHDSNI2juOoEjyCRNg8OIVkcNRBDaFJhheGYZDGDQ5CCQIfg0Lg2kmSw0iwLg4DGSgwmmEw3jONY3leVoFgeWQximFw2EGhZzmSG5nDGF5AE0MZmDMYYriuZaMnSJ4ig6U5+jkPBsGkbhlCAeQygkMgyCKpgxqmrB4q4IqrCAeKorOq59qKpA+gEA",
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
    #[cfg(feature = "eclipse")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCAMhhDcIIuDAIAxCCMYUjKFYbC+IokD6AQ",
        categories = "science,design,development,accessibility,photography",
        tags = "lunar,solar,crescent moon,sun,earth,day,night,planet,space,mode,dark,light,toggle,switch,color,css,styles,display,accessibility,contrast,brightness,blend,shade",
        contributors = "danielbayley"
    ))]
    Eclipse,
    #[cfg(feature = "egg_fried")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMLg1CIIBjHiCwxDGDoQGMeYUDKFw+DwL4DgWB4eHAYR0GgIBkgsTQzCAOBjDALYNDUIIcDULQ2CANoOjgII0DAIA0C4OIti0N4Oj4c40DKPo6jCQY8jaOo8DeU43lKMZSjaPY2HMLQ3C0MpfC0NYOjCMgghaN4tmqZBDjOaZAi0OQgi0OBaCKHgviWJw+gE",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2C4Mw5DkIIMg4ORDDWDQ2DIIA4C4MQ1DcIA0C4Ng1CAMQwhwOIQiGJAxDIYwtg2H4hDQM4lC4MofDkLg5iQNwuiwMAgDONw1DaMAwDINgghYMg1DkLQxj8NIfhINw1C2Q4YDUIg+DwL4DgWXZggaCIKDGOg1DOGQxkMOJWEEMYhDCH5xC6cwgkGeYlmiJYujAM5GiEMw0liO5XjoOZGj6V4mlCdpyi0LQynYOKTDWSKFDANJKlEMg5DeXJemOXRsGkbhlCAeQxgkMgiCAeKrCKrapDKrKzHitayq2XQvqWpw+gEA",
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
        svg = "gAPBsGkbhlCAeQxD0Ig5CIIB4DKCgxgyDoJCINYNHmEILCIPg8C+A4Fh2IIGhmEYXgeFQxiceIViuGoShyHojiKBIGg+CongiOINiyEYTiUIowh2H41D6AQ",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEcocCIMoRHmHQiD4PAvgOBYoHAYYLCAZINE0NwgDEMBojeJ4pi6C4ti8aIxjONQxDSOQwjsL49GgPoBA",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    EqualSquare,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig5CIIB4gkIgxgyBwxgqEx4hUIg1CIPg8C+A4Fh2IIGhiCobhSCgxieCIpieD4pgyHYfgQZQ+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNh2DYIg+DwL4DgWFYYgaCIKDEMAgDENAgDIMYOhSFobhqBIcgmCw4iEM4SGEMokCCIIgDELY1DISA1jSNo4iGO47FYOJAjWQomDKO4QiiF4sD6AQ",
        categories = "arrows,text,social",
        tags = "outbound,open,share",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4OA4CCDIOGEMwghQMAgDEIIXDQLgyDQIIch4Ig+DwL4DgWJIngaCIKDEMAuDeFA1C4MA4EGLguDSFI4jqGo+hkMQyCANRjDeP4XkaLggDcYQxDMLgxDaGJPlGPoXDELQxC4NpGDKWw4iOJYqimBIrgkTQ2luGZpDYMY3k8NQylKTgunGUoXngIJCkEc4UkmSBhgwN4foKH55heMwzDmWZqmGJpliQbBpG4ZQgHkMYJDIIqWDKmaaCAeKdCIMqfHimKjo6kqUD6AQA",
        categories = "accessibility,photography",
        tags = "view,watch,hide,hidden",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMQyHMMwtDeDQwhKFAghMMQwhgLQzhyGofhUNwthqEhaCIPg8C+A4FikYxpHIYxsGUIBjHiCYOCIIBygkM46GMeY4DKKIqi+MYzD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQyDALgyDQIAxDKDg0GENgghcMAghoMAtDgLg0DmHogDkTA1hGDQ1FYMQ5GiHw1HoIg+DwL4DgWMxsGkbhlCAeQygkMgyCKPQxgkOJDHiPwikIIB4kUIgxDaMo0jmO44jqPB5k8MQ1kOPoJlySJKDmSJbDcLpdjML5VGUPoBA",
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
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDGDYPHmEoND4PAvgOBYHhkcBhHQaAgGSCxNhMIAyHYNAihkL4giKH4hiOJQiG0NguDgIAxDULQzC4NYpi2GowGiMoiiSCxtDIMAuDcIA3j6QJCi6RZHjSJo4joOQgj8M5QkOL4zleSY2kyTo7lGP49hiRJjDyRZlG2XAyDKXQtjqXw4mGVpwjOZRNjqdRonuVZvnGNYnjoMQ4k4YZclwMI7CAMAtigMJ8mOAQ",
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
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGEMoOCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4OHYMQ4CIPg8C+A4Fi+MoGgiCgxDSKg0iWFInhmDhoDSLowjWLxjGkchjGwZQgGMeIJDEMAik4eYJDKUwgHKV5EC+SJKkyNIEjaCYLhcNxWDaXZGgKY4HmWUggDEMh2C0MZrmOYoFm+OIXiydQyniM4B",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileArchive,
    #[cfg(feature = "file_audio_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMgiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIuGMaRyGMbBlCAYx5gkMQ3CKTB4gkM5SHKT5CC+RpIkqM4EjWCRNhSUB2C0MxhgyDI9DEIA4hYdpWi6MJgkWR5JksY5UCIOZSk2T5RCCWAiDGWpcngPoBA",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileAudio2,
    #[cfg(feature = "file_audio")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CAMgyGiDhhDKEAgDCGIaDILQyFYNxsC0NYiEgNoVheGYph2EB2DMIg+DwL4DgWMIzgaCIKDENIsDSJ4WimG4QGgNIvjGNo1gSN4JE2FgxDmPoaDEIJSjsMB2DGUIZlIMQtlUdpdGENggmKQJbj8do9haP5TlOXYYl+WJqlGbJVkWMpJD6AQ",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileAudio,
    #[cfg(feature = "file_axis_3_d")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtDgIAxk8NAtkaMYzkuSoFgeCRNk8MQwHYOBoDiR5XjWAQ",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileAxis3D,
    #[cfg(feature = "file_badge_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxGMaRyGMbBlCAYx4gmJQij0eZADCQhygkM4wjKN45juMY0gaCIKDENIOHYNInhOKYig4aA0kuM4EGiUJjgeCRtlUIIlC6DQxCANZtC0MwtDGc5rmsLZxDWYZRD6AQ",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBadge2,
    #[cfg(feature = "file_badge")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIaA2GGDYNDAIIWhYMgtDIVg3GwLQ1iASIShSF4mDCG4OHYMwiD4PAvgOBYujGBoIgoMQ0ioNITg6J4mhQaA0i2L40jOBI1gkTQ1CAMQ3GEMwglCFgxicLQ2lGWIYj4NhakOMJHkaBYHkkN5MDYLpLDiDgyh8MwtDELZQlScA1miXpFgEA",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBadge,
    #[cfg(feature = "file_bar_chart_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDgIAxDgdgtDGR4zkuSoFgeTYllCUgtDaVpJgKS5bjoNpelMM5iliAQ",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBarChart2,
    #[cfg(feature = "file_bar_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDgIAxDgdoZkeM5LkqBYHk2JZQlILZGjGVo1gKS5ajoNpdlMNpVkmAQ",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBarChart,
    #[cfg(feature = "file_box")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg1CAMgyEgMQ4GEMoQCAMIZhsMgtDIVg3GwLQ1iMSA2haGIaiqHoQHYNAiD4PAvgOBYxjSBoIgqDItDSKIXiqHIQGiL4xjOBBojaR4HgkTQzCAMQzC4MY+huKwxk8Lg3DYdpRDKPYXj+VQgC4OQ3lgNw4EwNoQDGWZUkCGgyC4MJRDAMRMDGVwxDmZJvmIMQtm2WhWgyfZgmIMAtmQN6BlmaQ4k+bQzn6K5ynSip2FqMIyjeSYFkuCpmDENx2DWm5GjWApKjmC6Ck+DYXqKIQtg2jJyDip6dgE",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBox,
    #[cfg(feature = "file_check_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkbQzCAMQ1hWFA0C2QYvjQPoBA",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCheck2,
    #[cfg(feature = "file_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtDkIIMhSEw0C2RoxjOSw+gE",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,danielbayley"
    ))]
    FileCheck,
    #[cfg(feature = "file_clock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGEMoOCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4OHYMwiD4PAvgOBYujGBoIgoMQ0ioNIlhSJ4Zg4aA0i2L40i4YxpHIYxsGUIBjHmCYMCKTR4gkOJSHKCQ2kML5HkmS4zgSNYJE0OQuDUIAxDeZggDiaA2C4Mg1FaOJbkWAQ",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileClock,
    #[cfg(feature = "file_code_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkbQ1CAMYaDMIJPlGLYvjSRoFkgIhtDmTA4lALQzl+X5TkOMoBA",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCode2,
    #[cfg(feature = "file_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtDGFgxDOGYUhqR4zkuSoFgeTY7CAMQ3g6GZiDKVpJgE",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[cfg(feature = "file_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMgiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIuGMaRyGMbBlCAcoJDMIggGMeYJhGUBjHiCQ2kIL5GkiSozgSNYJE0NggDEMB2DGW5EgKYYHmOZQxDeaZrmGYIFm+N4XhESA5nWMptniNhNDOZg0EiLIujCdqBmIIhtDmZgxC0Lg4DilQ4n8aJ3o6hKYmYNguDGFKFnKmqcnmkJmDelKWq2maKmyNJ5p6lqSp+pZqrGdoBA",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileCog,
    #[cfg(feature = "file_diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoOQgDEMBoDaMIyjeNoEjiCYLhMMQzi2QYzkWRIFgeR47DEN4/k2Q4BA",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDiff,
    #[cfg(feature = "file_digit")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBoi4chlGMdAgHIeIJDIIggHcaRkgWCYsCAaBlGkZxoHSCQ2lCTQik8IB5gkMZPkKR5JjORIHgkTQxheaBoDIdpgkKNJtgWb43nMOJAi2L56gEA",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileDigit,
    #[cfg(feature = "file_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCYLhMMQ4HYLQ2keM5LkqBYHgkbQ5CCDAgDOYJgC0M5VkmAQ",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDown,
    #[cfg(feature = "file_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMgiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIzgSNYJE0MQwC4Mg5CCSguDeEwuDQM4NlSVo9DCGwuDYNgtC4NQyGOYJNC4MYamENguDOYA3DiYQzh+bA1mwNJgDOIZ2lOVZXn2WpcDaeg1DOZJsmeaQ1mubZRDmcZgDmdQ5ncMZXoSTAgDOUQ0Eyaw1k8OBspqX6aDUNRjDGTAtqkMqVqmEZcnAMQ5C2mg3DQWpCjCRg+gEA",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileHeart,
    #[cfg(feature = "file_image")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeMRjGkchjGwZQgGMeYJDEMwilEeJUDCVxygkMpHC+S5Nk+NoEjiCRtDKFgxDcLQxC4MZunCQIghgMguDibwwDETAxiKX4xjOZg+gEA",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileImage,
    #[cfg(feature = "file_input")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkTYUDENYQDCLYvjSRoFkgIhtDkIAxDgIAzC2XZflCQ4ygE",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[cfg(feature = "file_json_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgmCwgDEMhhDGS4YiiTwxHYMZOlCPQxC2U5Qk+WZdlWV5elGXYti+NJGgWSIKDiSw4mKZIXlqVJbnCX5zl2ZJzlsdp1lyPQwluW5mkOMoBA",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileJson2,
    #[cfg(feature = "file_json")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCYLhaJRhDEIJSkILZSDGHJRlOIJTlaW5XlyYJYDGWpUmGU5HjOS5KgWB5NjuUw4mWXJPlYdpWnOKZXl6YJ6ladp4n2IJ/DGaZJgE",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileJson,
    #[cfg(feature = "file_key_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNgiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIuGMaRyGMbBlCAYx4gmQZMHmCQxiwIBygkMpCC+RpIkqM4EjWCRtDGF5kC0NAuDUIJoDWWpEgKYIHmIOQgDEMZ1nWbpgD6AQA",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileKey2,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxGMaRyGMbBlCAYx5gmHQij0eJADCQhygkMowjKN45juMY0gaCAiG2HQgDEMAtDQLoNlsNZLjOBBolCYoHgmVYNDEMZXleYJRD6AQA",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileKey,
    #[cfg(feature = "file_line_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRth0IAxDMLQzC6DZUDWGYZEwOJQDeR4zksPoBA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileLineChart,
    #[cfg(feature = "file_lock_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMQiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIuHIZRjHQIB5gkMQzCIIBoGUaRnGgdIJDWTx3GkZIFgkOJPHiCQyk8cphCKLIuC+RpIjOBI1gkTQ4CCTR2huJY8nOFgtgwMB2mOaZEgEA",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileLock2,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxHIZRjHQIB5gmJQiCAaBlGkZxoHSCQ2j8ch4j2Px3GkZIFgkOI/ksIpTjEL43jmMY0gaCIKDGFolHaGYnhOYggDEIA0hUdgyjCMpdD6AQ",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileLock,
    #[cfg(feature = "file_minus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkTQzCAMQ1GgNoti+NA+gEA",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileMinus2,
    #[cfg(feature = "file_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDkIIMGgNpHjOSw+gE",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileMinus,
    #[cfg(feature = "file_music")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ0CKCR5gwMQ2hAcoMDIIg+DwL4DgWB4bh6BoIGOEoNDiFoYhCCoMhWG4dgSI4bHAYR0GgIBkgwTQ0CAMQyC4NBWDQYQyCCRQwCCSAxkYLQyGgOAuDUTAykgN5RFaVJEkaSZcDGTZGGgLZWDWGocjSNozjWN45CITQ4j0OB2mILg3EyFAgDkdg3mUL5nGgPoBA",
        categories = "files,multimedia",
        tags = "audio,sound,noise,track,digital,recording,playback,piano,keyboard,keys,notes,chord,midi,octave",
        contributors = "danielbayley"
    ))]
    FileMusic,
    #[cfg(feature = "file_output")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkTYUDENYQDCLYvjSRoFkgIhtDUIAxhoMwglyXpQkOMoBA",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileOutput,
    #[cfg(feature = "file_pen_line")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDgIA1C0M4QEgNhhDIIIWDAIIZDALYWDIdgxhSHoaiSGYeGgMQyhWF4liQModCIPg8C+A4FjKNYGggIhNg2DIojGM44jeBI5gkTYMC4NAgDkLoiiyGQxCCUQzCAMxMDEN5SDcbAtkqUQxl0WpAjSRA+gE",
        categories = "files",
        tags = "edit",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    FilePenLine,
    #[cfg(feature = "file_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIaA2GGDYNDAIIWhYMgtDIVg3GwLQ1iASIShSF4mDCG4OHYMQwCIPg8C+A4Fi+MoGgiCgxDSKg0hODoniaFBoDSLowjWNIEjaCYLDALo6gwLokj6FgxCCVAzCAMxMDiDgxh+TpVC0NBakSMZID6AQ",
        categories = "files",
        tags = "signature",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePen,
    #[cfg(feature = "file_pie_chart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDIaAyGEMoOCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4OHYMwiD4PAvgOBYujGBoIgoMQ0ioNIlhSJ4Zg4aA0i2L40jOBI1gkTY5DEMQuDUYQ2C4MIUlGU4YDGGA4k4IJaDWQ4wkeRoFgeSY4CCDBjDALQzC4M4bC4NwtDacpyHYNhal+RYBA",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePieChart,
    #[cfg(feature = "file_plus_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkTQzCAMQ1GgNoti+NJGgWSIKDaTAyHaUJClOAQ",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FilePlus2,
    #[cfg(feature = "file_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDkIIMGgNpHjOS5KgWB5NiWUA4HYLZTjGVY1gE",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FilePlus,
    #[cfg(feature = "file_question")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMYWjsLgzGMLorC4NAuDULQuDgLg5C0MYSC4MYOk+IAglAMguDaQ5ADOQ5FkiRZUj6Fwxj6GYOmUMowjKN42gSOIJguEwxDcaAuDAMZpjObQ+gE",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileQuestion,
    #[cfg(feature = "file_scan")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIAxDAVg3GwLQ1hQSA2GEMgghqDYdC2GgyHYMYYiAIIdiaG4bGgNAiD4PAvgOBYujGBoIgoMQ0hsdg0hmKYng2IIri2L40jOBI1gkTYjg6PIlj+H4bkOMJHkaBYHkmDJMj2HIoDGKQylKRYCkeV4KlkMgyluKInDKH5hlSY5WjaSg2huaZOl2H5ui6U4ygE",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileScan,
    #[cfg(feature = "file_search_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeMRjGkchjGwZQgHKCQyC4NQiCAYx5gmO5VlcYx4lsMZdjEL5Lk2T42gSOIJgsMwuDMIIdm+cYNDEOJHjOag+gE",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSearch2,
    #[cfg(feature = "file_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMwiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIuGMaRyGMbBlCAYx5gmEQiCAcoJiyTB4gkNZCC+RpIkqM4EjWCRtDkIAxDgLQxC6IZplmLowmAPoBA",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileSearch,
    #[cfg(feature = "file_sliders")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDgIIlGgOJHjOS5KgWB5NDGFgxDEdgylSSYCkuWYKk8MQ3lKYZWmOWI5guPIdl+a41gEA",
        categories = "files,development",
        tags = "cogged,gear,mechanical,machinery,configuration,controls,preferences,settings,system,admin,edit,executable",
        contributors = "danielbayley"
    ))]
    FileSliders,
    #[cfg(feature = "file_spreadsheet")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDgIAxDMaAykeM5LkqBYHk2O5QlKVIxlaNYCkuWoKk8MQ3lOVZJmOWY5guPJomqYJJgEA",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSpreadsheet,
    #[cfg(feature = "file_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA3GgLQzGEMgghQMAghcMQtDKGxWDIIg+DwL4DgWIYkgaCIKgwIA2HYNguDUY4XC4OAtC4NwgDGMAtjoNY5jCEA3GONg4hiPI7jeRw1kodgtDmMpEkmPY/kuPRIDENxaiCIoniaBIogkTY4Dgdg4jSMggC4MwuDILovDSNJsnINQunCZg0lcNZbiOX5egWB5hDOOQymWZ4zmubZvnGbZtnSdp1lcMZ7l2AQA",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = "danielbayley"
    ))]
    FileStack,
    #[cfg(feature = "file_symlink")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNwiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIzgSNYJG0MYXDEOAgDMLZPlGQowkaRYFgeCYLCCTB2C0MYljyGAxg2GxoDaU5EgEA",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSymlink,
    #[cfg(feature = "file_terminal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtDgIIdg6GZTDKR4zkuSoFgeCYLhMMQ4kWVpJgE",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileTerminal,
    #[cfg(feature = "file_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCYLhYORIDiR4zkuSoFgeTYdCAMQzlGU5JgKS5YjoNpbDeXoxlSNYB",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileText,
    #[cfg(feature = "file_type_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkTYUDEMx2C0MRoDYdgxi2L40kaBZIgoNQgDEMh2DaVZDjKApHjaC5dDgaAymKV4BA",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileType2,
    #[cfg(feature = "file_type")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDkIAxDMdgtDEaA2hyR4zkuSoFgeTYllCGw2lmSYCkuXo6DGUA4GgMpkluAQA",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileType,
    #[cfg(feature = "file_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCYLhOJR2DaR4zkuSoFgeCRtgwIIMC0M5el4IAzlOSYBA",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileUp,
    #[cfg(feature = "file_video_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBoi4chlGMdAgHiCQyCIIB3GkZIFgkOJPHKTAiDGTxoGUaRnGgdIJDaTx5gkMZOkKR5JjORIHgkbQxhcMQ1C4NQggwMp2HYLQ2h+eZ2i2L40D6AQ",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileVideo2,
    #[cfg(feature = "file_video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtDGFgxDEIINDMLZUHYLQ2i+MYzksPoB",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileVideo,
    #[cfg(feature = "file_volume_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRNDgIIMGgLgwDGR4zkuSoFgeTQxDELoNDEM5fhKX4OmWKQxhcM5WkmApLluOpgiaDYNmiFw2myWIBA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileVolume2,
    #[cfg(feature = "file_volume")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYMwiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGgNIti+NIzgSNYJG0NwgDEMAtDODRIDIdg0GgMhsk8MhakKMJGkWBYHgkTQxDGSwxGENQgmiPZkhcNpbkSAQA",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileVolume,
    #[cfg(feature = "file_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgqJQgDkdg0jCMo3jaBI4gmC4TDENxoC4MAxkCM5ED6AQ",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileWarning,
    #[cfg(feature = "file_x_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGgMQ0GEMoNCAMIWhgMgtDIVg3GwLQ1iASA2hOFYXieG4NHYNAiD4PAvgOBYujGBoIgqEYqhKFIUieGYNGiLIujCBBojORIHgkbQ4CAMQyC6IQ1CANYti+NJGgWSAiG0M5Mk6UZflSQ4ygE",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileX2,
    #[cfg(feature = "file_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjeNoEjiCRtjsLoNiWUAtg0NZHjOS5KgWB5NDmUAglKVQglaMZYjWAQ",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileX,
    #[cfg(feature = "file")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAyEgNhhDKDggDCFYVC2EwyHYMYRhqF4WhaGhoDEMoShSIYXDKGRWDcWgiD4PAvgOBYxjSBoIgoMQ0g4dg0ieE4piKDhoDSMIyjcPoBA",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    File,
    #[cfg(feature = "files")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA3GgLQzGEMgghSDYNDELQyhoVgyCIPg8C+A4FiCI4GgiCg5CAMQ4hOFQgheK4ahwNIuhaMIrhWGhoDcbA0CANB2DEMI2jiGIahUWofiGJolgSJ4JE0M4OC4NpCDILg4EEMZViuXYxg0NJdDIMhoDmWZLiKTw+gEA",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis,danielbayley"
    ))]
    Files,
    #[cfg(feature = "film")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEaBlGkZxoHSDYPhEcoTCIMoRHmFAiD4PAvgOBYoHAYYLCAZINE0NwgDMdoeigL4uguLYvGiMYzDMIA3C4NRoDSJ4pjwaI+jCMgiE2QwxDIaI5kuP5OkCUJSCAMQ2kaSJKjuWQ8kyQZRDGNY3leZI9maP5oE2apEmGSY6kyWpynSX52mOeYBA",
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
    #[cfg(feature = "fire_extinguisher")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIA2C4NRWDMYQxCCFQwCCGAwC0MYcGgLQyhSFoZiSG4VDEdgzhAIg+DwL4DgWLYwgaCIKDmFg4GgOIsi6M4ygSNIJgsOAgDOHwzjyL5Aj+BYHkIMYVhMNoOiWGQtlMNh2lCSY+gKQJOgqDQxkYNJckuXpNjWCw3hYMBhDQIJwhqVpEDCWpuDIIJ5nOGJ5DIaA0GGfpVn2IBamaMYBA",
        categories = "home,tools,travel",
        tags = "flames,smoke,foam,water,spray,hose,firefighter,fireman,department,brigade,station,emergency,suppress,compressed,tank,cylinder,safety,equipment,amenities",
        contributors = "danielbayley"
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
    #[cfg(feature = "flag_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhjDMIAwCANYNCCDAyHMNAtDEIIaDEdgxDEIg+DwL4DgWJIngaCIKDSDQyFYNIjiWKopgSK4JE2LgxDUcwxhuHZAhQMoWg2M4mjeJBsGkbhlCAeQygkMgyCKTwxlKVR4lEIpTlmV5ckeS5ND6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQ1HMMQtDEIA0hIIA1CAMggDiGYUhaFQxFYMxzhaEYMhENQtDILQ4ioLYni+DR6CIPg8C+A4FjUbBpG4ZQgHgMYJDQIggHkMoJg6RB5kEIgyDKRB4kcIpDjUL47j0PoBA",
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
        tags = "fire,lit,burning,spark,embers,smoke,firefighter,fireman,department,brigade,station,emergency",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Flame,
    #[cfg(feature = "flashlight_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMHYNBhDIIITDAIIWDELYTDIaAthGG4XiGGQyhoVgxDAYwwhqK4kiQNAiD4PAvgOBYxjSBoIgoN4UGgMQxhCKYUhqQoTDQdgxjCMo3jEbBpG4ZQgHgMYJj4IggHmUwiDaVh5DKCZblGXgiDEOJJC+TZPkyTpQlKCQylyWZvleYgynIeJ0m+MZnmsPoBA",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA2GMMAgDILQyhKFAgDQdgxDAYYVhWEYRDGFwyGgLQ0h2EggiAIIihMMhWhuEIUjOLomFaJAxDIegiD4PAvgOBY9GwaRuGUIB4DKCYMCIIB5kkIg2kweQxgmUZHlSUI8j6Q5FkKRJGk6SgylKWI5kweJlmOR5PmaPQvlwZQ+gE",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDAIIMCANAuDcMggDIMAuDUNRhDGDggg2HwgC4OYOC4NA1GgMQyhINobh2IINiILQxiUNRsjILgyhMLYqDUNAyCIPg8C+A4FkGRIGgiCoPDIdoqDMNAzkCQpHkaBJIgmCw0hUdg2C6T5RkGQ5WlWBYHlgOIYhUaA3lKYpFgKVpmgoN4ODYaA5m2VA8GwaRuGUIB4DKCQyj8IB5DGgwioaggioSih4oijZtnyfg+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIA1GEMwghEMAgDGFYShIbQtDOG4QhiFIWDCG4ZhwMx2DETQ5CAOIehOF4UhEM4pisaAxG0NQgDCLY5hcMYjDMbYlGgLY2C2DYmkQIg+DwL4DgWSxjGkchjGwZQgHKCQyCIIBjHmCQ4lsYx4gmDJKkyUZTlWS5OgaCIKgyFQwieWpLk2BBomud4HgmC4NDIMhjDQLoNhQN5EC4Ng2DcIKGDULaCoQLaLDGiKKpKDhamadpPgKepun0IJ/GOj6DjmkqHomhqNCCkI8pOlaLosNaZnWbA+gE",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA3C4NRhDSEAghMNQgDAIAxhqFYUhaC4Ng8NRBhaHYXhmG4ZiKGgyiCDoQFYORtC2JQzhKHoUiiGImiYTYrgwSIyisMI3heJY6DGNI4DUbYWkoNRIDGTAtDOJhWlIIg+DwL4DgWWhjGkchjGwZQgGMeYJgwIggHKCQzmsYx4mkMpZluYJimSWpdgaCAiG0OIaDaGoQC0MYQnWXIEGieqKgeCYLiUOYUDGgg4oie6MgWjp+oCgKGhen6XoqmZ8o8MYlqek6CpSopegE",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTQzCANxWDUYQyCCMAwCCMwxjELQyGiD4bC+IokiGI4licIhNDENwgDOOovjGNJNjaMAyHaO4cj4aJAiSJooDKNpGlKS4yk6OIxGiOIalSQZXkKKJHlsSIulCTY1jiOB2mWPJVD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ5CKCR4gwMQ1hAcoMDIIg+DwL4DgWB4bHAYR0GgIBkgwTQyDALg5CCDguDgQQyCCMwwCCNo2DKMwxDgVg4GGM41jeNwtDKRRoC0N4skCNJDjYMQtDELg2DkLYsEwOZTCAM4sjKTY4kOSg5DOWxIDSTJCjiRY0HYMQzmiTpDkEaA1C4MYahyIokiGI4licIhNhSLgxHaUZ4C+ehonyJImiiggxDehYZhuiJ9D6AQ",
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
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDEOIOGMeYShQPg8C+A4FgeGhwGEdBoCAZILE0MQwC4MwgDIMBIDQYQyi0IAwjUIAxC0Mo6FYNYyjSNo2DGLY6GgMwuDmP4zkGOI4C4Ng5kgbAuDiQwxC4MpKjeTJXDYN5IEiLpalyNAyHaR4NhoL4hiOIIiiSJgiG0MpXDeOJRDQLZInqaYbmwaJuiOJYLG0MQ1liOA2nui59mub6BnChAxooNotnWK56DkIpqn+kKDnIMZRlahwypiSKbn6jw8n+n6FlGlZ0C4N56nkMaoo6barm+raTlSOKHDOtAtranKqqycaFDSK53k+OK0rena6oKyIurKia+jkLg0tCj4BA",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere"
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
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgwEgNBhDKDQgDCFQgDELQyhoVg1hKFIWhYMYNhoaAzC4OYfhOIYYhgLg2DmKBsC4OIjDELgyiqF4sjcNg3igSIOjqPIUDIdg1CIPg8C+A4FkoYxpHIYxsGUIBygkMgiCAYx5gkMZZlseJeDOSZLlCUpUkqTYGgiCgxDiGA5GOGo0hULQ1nSG54DUdg4mWTIEGiT5RlOVRjmIIoOlqV6JlqXJeDmf5noUPoBA",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygyD4JHmDAxDMIg+DwL4DgWB4bHAYR0GgIBkgwTQyDAIIqGEMosCCK4yiwLQyFYOIujCMwwjWNRoC0NwuDmOYvjMMQtDELg2DkLZCEwOZKCAM5CEGL5FjGWJBDkM5SEgNJEliMo1iwdoYmCO4wDIWoahyIokiGI4licIhNDENAghgaIZhsL5uGicIkiaKA3ngM56myfZxD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ2CKCR5gwMgwhAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MQwCCFBIDQYQyisIIqioMQtDKNhWDWL4xjMIAxiuNhoDMLg5juMI9j8MQuDYOZEGwLg4kkLgykaMpWjSSw3kQSIUlWSIxDIdgyhmG4hiOIIiiSJgiG0MowDENAtDQLg1CCcw1mQL5mGiaIjiWDJtkmdZJnmew+gE",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDUIggHiDgxDSER5hQN4RHcaRkHQaIODiERyhMIgxCIPg8C+A4FikcBhh8IBkg4TQxDAIAyDASA0GEMo4CCN43DELQykQVg1j2P5BCAMY4kQaAzC4OZJj6S5NDELg2DmUhsC4OJXC4MpUkCZJClkN5SEiOZjlaPwyHYMguhCKQvi+H4ujAaIyjSOZMDcdpEmyTJAC0NJAnCKIqnYaA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CIbQ2CAMQ0g0Lg0DULQyC4ORBDIIIZDAIIcDEIA5C4MoPDEMBIDIMBhhmG4dg2EA5g+FQ1GwLQxC4NQ1CANoqhqLYejWFokjcSA0jyLI/DKFBWDUYwwkCNg5hSGoUGgM4WDORo+i6Ng2DaFhsC4OIZjYMpZhyZ4Ql2FhIDEOJmi2H4rHYMgiD4PAvgOBZ2GMaRyGMbBlCAYx5gkMQ1CIIByoWiBjHihQ0nWd58n6gA+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQyCANwuDUVg1GMMAtDELoWDkLYMDKGhoDMLg5DMYYbCAMIlCAMYoC4Ng2iAbAuDiDIWDKIwggyJo4iqLIgEgMgwjWN4nimGx2DGP4kjkMYajaPQiD4PAvgOBZPlKBoIgqMgzGiRpOlCVZUgSVoJG0NYohQMwgmiapdlGYQ+gE",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
    #[cfg(feature = "folder_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4NAgDEMAuDYYQyCCFQwCCGAxCAM4cEwNoPDkbAtg6GwxiQWgiD4PAvgOBYri6BoIgqFQxDELg1FYNYUhaGY+hsMgtDIaAzC4ORjC4N4/C6RYdjcN5GGwLg4g8LgykgNISg8IJGlWUA5EgMgwjyF4/j0Mh2hCZI+hqQoWGgLQ5jiKosjEPoB",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgwGgMQ2GEMoNCAMIWhgMgtDIVg4hOFYXiGG4bGgLQ3C4OQzh+FIhCAMQtDELg2DYLYoGyNQ4hqMQyEGFIshiF4nikIAzEgNIrkCFobg0dgxDMY4XjGMQ5hWPhaCIPg8C+A4FlkYxpHIYxsGUIBjHmCZOCIIBygkMpqGMeJom6WQvl+YZjlmXIGgiCgxhQMQ1HYNZYlqeg+gE",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley"
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyC4NQigkeIMDEMYQhIcoMg+EQ+DwL4DgWB4eHAYR0GgIBkgwTQyDAIItGEMovCCLo1i8LQyFYOIxjONgwjiOBoC0NwuDmPIyjYMQthcNg5C2RRMDkLg2CAM5FEGMpIjSW5EDkM5VEgNJHluNY4i8dgxDOY4+jMMhaCKHgviWJ4kiaKIqCITZpC6XwxDSfAgDENaBDacIfnMaA+gE",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ3CKCR5gyDoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0MQwC4NwgDIMBIDQYQyi0IAwjUIAxC0Mo6FYNYyjSNo2DGLY6GgMwuDmP4zkGOI4C4Ng5kgbAuDiQwxC4MpKjeTJXDYN5IEiLpalyNAyHYNAuDGGYbiGI4giKJImCIbQykOdQtlcNZ4C4NZrC+bRoD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIIMEgNRhg2DQwCCFQwC2Ex2DUIg+DwL4DgWHhjGkchjGwZQgGMeIJDEMwiioeYtDmMBygkModh+JImiiI4lieKY2CKOIxjOMIrgmHIeC+O5Ah6IYGgiCg4g4ORoDMbQ1C0MQ3HaXBoDYTQ2g4MhWDcY4YDELpqDmGQgDKGZXGwNQgkqH5QD6AQ",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[cfg(feature = "form_input")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHmDQ2hEch4g0MoRhiDoRGgZRpGcaB0g0MYaD4PAvgOBYoHAYYLCAZINE2JggiYaAuDAMQiigL4uguLYvGiMYzDEN42DKOI6jyKY/GiQYwjIIhNkeN45juPZOD6AQ",
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
        svg = "gAPBsGkbhlCAeQyD0Ig2CIIB4gkIgyg0eAxgoMoSgeFYLCIPg8C+A4Fh2IIGHmGgxDiE4QhiFIWhiCIKieHIeiOIoEiSKorhqDIOhCO4lhaMofjaNYFhmQI8jCKIHjiE4miiHZCiGAQ",
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
        svg = "gAPBjGkchjGwZQgHIPQiDEMAiCAYx5gsMQyg8Yx4hOFQ+DwL4DgWB4bHAYR0GgIBkgsTQxDYIIqHMLQxC4NQtDILQ0jONQgjQNI5CKGwviKJIbGwaRuggeAygsOQuDAMYPHkMZJk6SAiDmDx4lCVI9hyQ5FkKRJGlgMQ1laU5ikuTQgk+UZplOVY+lwZQ+gE",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKDODh4gkIgxDWDh5hGC4ND4PAvgOBYfiKBobgoOYUhwNIahaKYPhYMYsh+IYEGWHxwGEdBoCAZIKE2MgggwVg0GEMpCCAMJJkkLQyk0SA2kaSJKlSTZCHYMQ4CKNI5juOI6jyPgikANAgDEMxoDKUpHlSZpIDIdpqkebJLkqcxoDCa51kuTgyFYOQuDgM56m0MAtC4NQ5C0MQuDQMhMlkIIZlyYA+gE",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[cfg(feature = "fullscreen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PByGUYx0CAdxpGSBYJDEMAiCAaBlGkZxoHSCQ4mweIJDebB5nSbByncIgxksL5gmIPoBA",
        categories = "layout,multimedia,design,photography",
        tags = "expand,zoom,preview,focus,camera,lens,image",
        contributors = "danielbayley"
    ))]
    Fullscreen,
    #[cfg(feature = "function_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIB4goM4NgkIojgeIYLCIPg8C+A4FiwcBhhIIBkgoTQ5CAMQ3GMMggDAIAyC4OAtDGQZDC2Qg4FYMQwGMMJJjoLQzC4MwglQMpTiuLYyhKMYzGiNY3jkMQxC4MhoDULg3lsL5dGgPoBA",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANx2DEMAiD4PAvgOBYThaBoIgoNggDWDw0hKFIZhMchlGMdAgHcaRkgWCQxDIIggHmCQzjIeIvhEIBoGUaRnGgdIvDiMhyjgIoxhML4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMx2DEOAiD4PAvgOBYTHIZRjHQIByHiCQyCIIIfCINoiHcaRkgWCQxiEIB5gkM4iGgZRpGcaB0iyEYTC+GYbhOFoGgiCgygyDoQhKFJBD6AQA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMGgZRpGcaB0gkMQ0gweYJgsIB3GkZB0GiGQ4CIPg8C+A4FikcBhiQIBkgkTQ0CAMgxGgMYoiqL4ki6MBojKNA5jeOY7ikL4+GiQIxjMIhNhqRo6jySpBk2QpPlGRY4lSSZLD6AQ",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMhoDEMAiD4PAvgOBYThaBoIgoNQgDaDw0hKFIZhMchlGMdAgGgZRpGcaB0gkMQyCIIB4gkM40HKNgijMIB5jGEQgHcaRkgWMQ4iIL4migPoBA",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCAMhoDEOAiD4PAvgOBYTHIZRjHQIByHiCQyCIIIfCIM4iHmCQ2iIdxpGSBYJhCIhoGUaRnGgdIwiGEwvhmG4ThaBoIgqDAyg6MY7kAPoBA",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDEIggHiCYLDCDh4DGCg2g4eYWguDQ+DwL4DgWHohgaGoKDmFIbDiGYRDEM4UhGK4eiCBBliONYHi0MopgoMQ1hmGwxjuD4tDULgwh2H4kjeBZEj0OJHg2OY9hOB5BlWFZPCKM5LDwcBhHQaAgGSChNDENwuDMMggDUSA2C4Ng4GENAgnQMAgncMAtDMLg5DcOAgnwNQ5GMLZHDCbwwDUMqGkgLgxkijZno+ixDDKcAwnQOQuDQMQ2CCawxDSnA1p+oQ2GEM6BnirJ3qoMxjDGrAxC4NaGDWoAtDEbK0p0NK6pyohBmua55CCsqbDgMqAp4aKjDMNA0GGxKtsex7BqMNQ4DYTJnscOBjrWtbWrKoapquxqunsY56rQNQ0ramK/m+2q/nAOK2mgMrao0MA3o28ZIDGhsDlG/6UDEQZ0na1aTmqbB6luH5fmEPoBA",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDAIoHDGCg2g4eYJgsMoThCFgiD4PAvgOBYch+Bh4hkOIOgiComCCFIKDENIYi2DYch6BBliGNYPi0NYwgsM4nhUMQ1C4MAxhOQI+jOIo3gWOYLiqKJOkORYrhkMZTiyC5FkmNYcHIZRjHQIB3GkZB0GiCgyg0IBoGUaRnGgdItheK4Rg4ch4miJ55jOXpgD6AQ",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gantt_chart_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEhyhQIgyhIeYVhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOE0OQgDgaA3ieKYuiOLYvGiMYzDgIAxDIaA2joL48GiPowjIIhNDEMZEDYaA1kmSw+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQxDULg2CAMguDcYQxDAIIVCCFgxhkIINDeHYSCIPg8C+A4FiMYxpHIYxsGUIBjHmCQxDIIggHKCY0i8eIyjSIwvimK4tiOJoGgiCgxDMLg0hcMIOhcOYdiKJJED6AQ",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CAMQyg4LQ4CAOBhhIMQxDkIIZhsIAwhCEAtDOJBsDiFAiD4PAvgOBYri6BoIgoMQ2hCNg2C0NoqiyMYwgSMoJG2FYVjmO4ri2QI/gWB5ChwN4WhaPJJi+ApAk2CgyDGEAxhSKZIj6AQ",
        categories = "maps,tools",
        tags = "hammer,mallet",
        contributors = "Andreto,ericfennis,jguddas"
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDGEYUCIM4RGgZRpGcaB0g0NIRHmDYQD4PAvgOBYoHAYYLCAZINE0MQyCAOB2DGGooC+LoLi2LxojGMwxDkII1HYNxhjaNgwCCTgxC2TBIkqTJPleUQylIdgtDcIo8j4aJAjCMgiE0NwuDWN5LmkIAym2TpQk8LQ1EENAuDiN5XnKNZrneeZ5nGRwgncNZ0m6bZvmqggxlcNZfimYQ+gE",
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
        svg = "gAPBsGkbhlCAeAxD0Ig2CIIB5gkIgzg0eQygoMQ1g0eIVgsIg+DwL4DgWHhjGkchjGyBhjHiFg4g0Yx5gqDAgHKCoSh4L4kiaKIjiWJ4piuHIzjWLowCIMYtjeOY+h4cBhHQaAgGSChNkcIA5GEOZWCAMJbCAMQtlkOYdh+TZPD6AQ",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwM4QGMeIMg4Ig+DwL4DgWB4bGwaRuggeAxhOEB5ieDYPCAeQyhiLR4jAIg5hqHIiiSIYjiWKwxDWKY+i2L4xhCM4MDIMY3C+ORlD6AQ",
        categories = "development,maps,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommitHorizontal,
    #[cfg(feature = "git_commit_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYNgiD4PAvgOBYTGMaRyGMbBlCAcoJDMIggGMeIJgyIxjHmJwyhKFIZhuHYThaBoIgqDAgDENYQi6FYEGgPoBA",
        categories = "development,maps,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "danielbayley"
    ))]
    GitCommitVertical,
    #[cfg(feature = "git_compare_arrows")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDWDhjHmCw2CIPg8C+A4FgeGhwGEdBoCAZILE0MQyCANhoDUYYqioMAgjIMQgjAdg3hmG4hiOIIiiSJgiG0MQ1CAOQtDOSAgkmDYaC+PBohqHYGgiEILDEOYUhYIgxDiDoKgyOocgSVI+iOJYnikIJdEgN4vjaM5xDELQynQVpZk6UJmkCCxtDma5FDOS5Kk2O4/D6AQA",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "danielbayley"
    ))]
    GitCompareArrows,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEOIOGMeIShQPg8C+A4FgeGodgaCIWgsNoVhEIomgmC4NhqHIEiKGhwGEdBoCAZILE0MQzCANhoDMYQyCCQgwCCRQxkOQx2DcIoujONYyjSNo4CKOpIhMSA4kGSZFkcLQyl8Vg5k2G5PGgPoBA",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ4CKCR4gwMQyhAcoMDMIg+DwL4DgWB4bh6BoIGOEgiDaEIKgyKAghcIoZhuHYEiOIYzgeEYTg+CYLieFoYhqHIiiAPBwGEdBoCAZIME2DggDkdgyGMMAgC4NgtC4NAgDELQxloSA3GOVw2CAMJclcNJmDEVg5kAL5FkeG5vkiSgikwMpaDIdowhycg+gE",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis,danielbayley"
    ))]
    GitFork,
    #[cfg(feature = "git_graph")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDaDhjHiCw1CIPg8C+A4FgeGhwGEdBoCAZILE0NQgDkdoThoL4hiOGodgaCIKgyFIRCIMQ4hSFgihiLozh8PIwiSJgiE0MQyCAMx2juGYbkWMoEjSD45hOD4+DEOYOjaDZBlSQ5FiWJwxDYIAxDULg3EEOYqCAMJwnKW4qlCL4iGgPoBA",
        categories = "development",
        tags = "code,version control,commit graph,commits,gitlens",
        contributors = "danielbayley"
    ))]
    GitGraph,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDEOIOGMeIShQPg8C+A4FgeGodgaCIWgsNoOgqDIVhEIomhqHIEiKGhwGEdBoCAZILE0NggDIMRWDkYQ5CCQgwCCRZFkIOQii6M41D6AQ",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINQigkeYMDaDxygwMwiD4PAvgOBYHhkcBhHQaAgGSDBNDUIA5HYMQyhiGogiKGYcgaCBjhEIgxDiD4KgwMQ5hSFouhuBI0h+IYjiUIhtDGKA5C0M5PCCUIXhkL4wGiRoiiSJosCANhoDUYQyCCYwwCCZgxmSZB2DeQpXD6AQA",
        categories = "development,arrows",
        tags = "code,version control,open",
        contributors = "danielbayley"
    ))]
    GitPullRequestArrow,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDYIA5HYMQyhiGogiKH4hiOJQiG0MgxCAMwtiiE4ZC+MBojKIokguN45DmPI8i6QIzkONImDEOAgDEMQuDUVgxDWTJBhmHIGgiCoMhSFgilKFIRmYOJMl+HoBA",
        categories = "development",
        tags = "code,version control,rejected,closed,cancelled,x",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_create_arrow")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDWDhjHmCw2CIPg8C+A4FgeGhwGEdBoCAZILE0NQgDkdgxDKGYbiGI4giKJImCIbQxikOQtDOPAgj2DYaC+MRojOI4lieLQgDYaA1GEMgglAMAglMMZRlEdpBjCNJGjWSQ5CCOR2hiQpEl2SAiE0MpQDEOBoC2ZJbjKAQ",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreateArrow,
    #[cfg(feature = "git_pull_request_create")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDaDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxNDYIA5HYMQyhiGogiKH4hiOJQiE0MQzCANhoDMYQyCCPwwCCQgxkCQB2g2GQvjAaIyiKJImDEOAgDENR2hOSpMk6NImDKRZSGgLZYi+Mw+gE",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreate,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ4CKCR4gyDoQHKDAzCIPg8C+A4FgeGodgaCBjhIIg2hWF4QgqDInhqHIEiKGhwGEdBoCAZIME2DggDYVg1hmG4zjWMo0jaOAijoOAgDEMR2C0MZAC+QhohobBpG6CB4DGLIQHgMpcCAeZbCIOYQHmXwiDKUIulaWA+gE",
        categories = "development",
        tags = "code,version control,open,draft,dashed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ4CIIBygwM4QGMeYMg4Ig+DwL4DgWB4bh6BoIhaDA2hCEgihSCYLCKJ4bh2BIjhscBhHQaAgGSDBNDEMwgDYaAzGEMggkQMAgkcMZFkUdg3hqHI1jeGxsGkboIHkMYMDmEB5DKDAyDGEB4l6LpilmZYwlSVg+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CINgiCAcoMDSDxjHmDAxDUIg+DwL4DgWB4bh6BoIgqFw4hSFgihiD4RCKE4bh2BIjhscBhHQaAgGSDBNDENAghgYQyCCQgwCCRQwC0MpJkOTJHkaSwyhqHI1jeNI2jiOgiE0MguDWPwzCCXg3GMLg3C0MQumCaA0ksM5JlIL5UGiVo3jmOwymiXgxmoOQgmMLZlmeaaCDWSQtm6UYwnIPoBA",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDELg1DQIAxDUSAxDcYQyCCGAwCCGwwC2GAyHYNIODQIg+DwL4DgWJ4qgaCIKDcIAzC4Mw0FYNRhDOMocjyG46DMdgwheGY9hGRIhDAY4bg2DQ5keGZBkOGpFDKH5BkoLZMC4OYfhmHxojOFYmiiLYsgSLoJE0MQxhmTQ1FYMQ4lKRYelWSJzh2EYflaWZ4j2ewyEgMguDANZjimZ4nGMaRyGMbBlCAYx5gkMQyCIIBypQMKXGMeKUpaJwvoujaPD6AQ",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Globe2,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCAMhhDENAuDUIIujAIAwjWN42DKNozjGPI4jgLY6CKGwviKJIhiOJYnCITQyjIMhokKRJGGgPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQyDIIA1FYMhsC0NQuDgOYOhUOQiD4PAvgOBYcGMaRyGMbBlCAcoJDMIggGMeIJDENguDaLBjHmMIUhaG4diKJImiGI4lieLoJDgLgxDGLIpCKK4tjcIg3C4NI7C+PZCkCPpDi8IgxDILgzDWSoqjWT5IjOYYclWQY/DyVomi2WwxDMLg5kmToJjmYYomOaZuGWWJCneXAwC4MIanCMJGDGepLiufZrn+baQogIoyDWNKCnILgymKTJUn6gJvjaMJRDSU57p6lAxoQOKfpCoZaninZNqOXIao+WQ+gE",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEeIdhEcogCIMgiD4PAvgOBYoHAYYLCAZINE0MwgDEMhohqKAvi6C4ti8aIxjONwgDMdo6imPRoD6AQA",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHiDQzhEeYdhEcocCIMgiD4PAvgOBYoHAYYLCAZINE0MwgDkaIaigL4uguLYvGiMYzjUMQ1jiEI6jwaI+jCMgiE0OQgDMdo5imSZLkCTRNkSUZTkeVY/D6AQ",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIOQigkeIMDEMoPHKEgiD4PAvgOBYHhmHIGgiCoMg4IIWCIMYPGOEYog6GYbgSIYfjGB4QgwNYVheCYLg2GIaiCHoCjSIoshOKo8DGOImheL5AGWM4diKSJKieKY2i2PowlGUIhjuEpUjqK43lmTg+gE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDEMoOGMeILDkIg+DwL4DgWB4bh6BoIgqDIVhcIoZg+EQiDWGociKIICgSI4rhKKoWhiDolg2G4djSMoxiSEonhKLo2gyFI+kKIZAkOJoPigMZHhCC4ukuTpNh+CJVgyOJSkePIvj+Ww+gE",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDWDhjHiCwxDIIg+DwL4DgWB4bh6BoIgqDIUhaDA5hSEQihOG4dgSI4hjGB4JheJ4SiuOYviKIICjSJI3g+KIYjqDIZjyQIzh+QYmg+LJFkOF4qkmTJLiOUotg6JYNk+F5IhyPRlleNYQlOW5ChWX4amGSo/kyWQxiqXopmiJpVjKb5YmadZZhONp3m2VoB",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYwwC0MQuhQOQtDIIAyhgaAyCIPg8C+A4FiCI4GgiCgxDcIAzh0Y4UDEIAwhoIAuDmNAyHaHogiKBBoiWPoHgkTQyjGKo6hEIIwC2NoahiGhohiH4hiaQIFkKCorkUSIQhOFYyhiTIXhuOZSjyVQ8HIZRjHQIB4gkNwiCAcpvCIMZyHmcJyHcaRkgWeggGgZRpGcaB0gkNZTC+apsiCjJtnSCZ3oGg6FocIqJCCeZ2h6bqSDCe59n8Ipxjyjw+gEA",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[cfg(feature = "guitar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDAIA3CAMQuDcLYRDcYQxhAIINhuGoUC4NBsh4NoiheGYcDCHg0hoTAxg8NB2DMWgiD4PAvgOBY0jeBoIgqLYOC0NQuhiQQxjONY6jQYxpHIYxsGUIBjHmCQxDILg1CIIBygmVpYGYaRsGyCRjHUchyGUbh0EMbxsG8cpYGMeJThGV40C+SpMk6OYEjuCRNDaEAyGEMggoOJ4QC4OIeDIbIfC0Lg5EMOISCCkg4CAOaIpSEIYDgY5VpaDQ1oQLqDqKooNhEMqOpaVQzh4OajDiIaPh8QaDoWGq5lSEA4GGKoqhwMQtr8YwtDMLgzh2I5VhOIw2r4ILArq0bDkaNp7nqBYHgkbZ/DGf63taSIB",
        categories = "multimedia",
        tags = "acoustic,instrument,strings,riff,rock,band,country,concert,performance,play,lead,loud,music,audio,sound,noise",
        contributors = "danielbayley"
    ))]
    Guitar,
    #[cfg(feature = "hammer")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDUIAxDILQ4C6DYSDUYwtC4OAzhkMwtDILgxDeHAtDMIAwiaKIniqJhhh+Dwgi4Mopg6JokEyLw5CIPg8C+A4FjuPoGggIhNiELg2DSDoNDKMgxDCRw0jqPJBkCBJCgkbQyk8OQxg4MQuDcLZfDINZiC6ZIXkeGA2hgOYdl8NJth2H5kHaGA4DaNw2C4MJdDSRxhDWEw2CCgg1oSKwwiQLg5nGX5IEgORsoyHw4DIQZ7DEOAgpmm4rl2L4SDQdpfocbIyDIaIfDQN6nmehJflyUo9lYPoBA",
        categories = "tools,home",
        tags = "mallet,nails,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis"
    ))]
    Hammer,
    #[cfg(feature = "hand_coins")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDUaAyGEMgghQMIOCCFwwC0NBoC0MxjC0Lg2hkLQxC6JwyiYLg0iMTAzg4NwiD4PAvgOBY0jeBoICIbQ3hWDYnDaKw0GMLgziKLQ4iKJInDSTBokWJ4NhcMookmFQukuKRsi2Q4tDSE4VhmZIblYNw1C2Vg5DEbIcC6FAzC4OYzjWOo5gSO4JG2FAxiSf51jaeY0GMaRyGMbBlCAcoJmsIggGMeYJnSkB4gmfqBoWh6JoShqIoqkYJDWjxjpYIg2o+jAiDOmaepyAQ",
        categories = "money,account",
        tags = "savings,banking,money,finance,offers,mortgage,payment,received,wage,payroll,allowance,pocket money,handout,pennies",
        contributors = "danielbayley,kayleyhill"
    ))]
    HandCoins,
    #[cfg(feature = "hand_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDQaAyGEMgghQMIOCCFwwC2EAtDMYwtC4NoZC0MQuiYMolC4NIiEwM4ODYIg+DwL4DgWM42gaCAiG0N4VheJg2ioNBjC4M4hiwOIhiOJg0ksaJEiaDYXDKJ5IhULpKigbIskKLA0hOFYZmOG5VDcNQtlUOQxGyHAuhQMwuDmMo0jmOIEjqCRthQMQ1CCI4xjONZ4neBYHgmCw5C6fg4ouRQ3iGPommiQYOouaQuDcQZmi+nJkmMMYjmCZg4lgN6lhqJJ+iYOBjkCb5ZmKk5YDgTKhg4MhanSg43gEA",
        categories = "social",
        tags = "love,like,emotion",
        contributors = "danielbayley,kayleyhill"
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
        svg = "gAPBwGEdBoCAZA9CITQxDgIAxDILg1FYMQwGEMgghcMAghoMAtDKHh2hWF4ZhuG4ehgdgxC4NAiD4PAvgOBYujGBoIgoMQ0g4MRWDmFoYiUMYmjkMB2DKLYvjSM4EjWCYLhqFIRFYNY+iSQYdkMdg5keMJLkqBYHgkbQ3g4NQtioNw2mYLpolSJYch4Lg4DOGJxDIbAzC4NggngNhDDeEYYiqOAgDkLojheD4YDIaAyGGDYNhyJQ4C0OBWDebZPkKGx2DWW5JgE",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[cfg(feature = "hand_platter")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzFYMgiD4PAvgOBYThaBoIgoNQgDEMBhDcLgxCCIokDAIIoiQMQ0imEoUhmGIEhqCRNi2HxoDENovhWM4ygWB41g2LI5DIYYNg2KoeikIA0GgLYRhOPYXgKM5BCIbQxDULo3iKLQzC4MpQC4OJHCCSZMiQMpkmcLg5GwLZgDaDguDMYwtC6Ig4C0MZ5h6YZjDifwyk8NJ3n2J5jDGeA0oGfJhEyHQxDiPIxlWQIbE2kg0HYNxIlGMI+gE",
        categories = "food-beverage,people",
        tags = "waiter,waitress,restaurant,table service,served,dinner,dining,meal,course,luxury",
        contributors = "danielbayley"
    ))]
    HandPlatter,
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyHYOAiD4PAvgOBYThaBoICIbQxDYIA2C0NAgDSIoihKFIZhMchlGMdAgHiCQyCIIByjEIozCAeYJDENI0HcaRkgWMgwjQaBlGkZxoHSCYRhML4si6GIEhqCRNh8MQ4GgLgwDGKIVlSU4FgeVgxDAIJZluXZfiqAQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDYIA2C0NIQhAIA0CIPg8C+A4FheGoGggIhNDEMggDIdg4haGIdhcchlGMdAgHcaRkgWCQyDAIggGgZRpGcaB0gmJwgHmCQxhUIByHiNY4kkIgyigL4si6HIEh6CRNg0MQ4GgLgwDGT4qgKVIHlYMQwCCWZbl2X5UD6AQ",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgxDIIggHgMYKDKDYHhGC4UHiCQig0Pg8C+A4Fh0cBhHQaAgGSChNDULg0DUIIrDEMQgDIIIMHYNhhjSNAwCCPI8joaAxjiOo9kWPwtDIdgtDYbAtDOLA1ksLg4DkQZEj6RZCC4Nw2CANBIDcLgyDSOYzkaPQtDGWw5jULoxHoIodC+I4lh2IIGhCCg2g6CIKkKfIWn+D4aDYLgwDGcYenedoEnigQwoCfp7geGqChmfgwoaiJyouAQ",
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
        svg = "gAPBsGkbhlCAeAxD0Ig0CKBwygoMgwg0eYJCIOYTg+FgiD4PAvgOBYch+Bh4hmEYThUMQ1g2CIKgwIB5hmKYbh2IohgSI4ZDiJ4KDOGIQDGK4ohKHIejeNoFg6Cgxi6FI8j4IgykCB4oDaM5FiCAQA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyGgOAiD4PAvgOBYThaBoIgqDAxDgVg2hKFIZhiBIagkTYOg2H4hhOFYmhMYxpHIYxsGUIBjHmCQxiGOB4jsOQiCAcoJDKIgvjKNI2iWBYHigMgwg0MBjC0MggDILQzCAMwuDWWQgi2I4wgE",
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
        svg = "gAPBsGkbhlCAeAxD0IgyCIIB5gmC4NHgMoKDKDIOhSC4MD4PAvgOBYcHAYR0GgIBkgoTQxDYLg1CCKosi4MggDIMRsC0N43GMLQxiyOwuDQNQtDOQguDKQgtDWLBhkmLZMCAMJPi6MwuDENAtDQLgzDUIocC+IokiGI4licIhNDgLg3DYIAzlQY48DGSYWlMMZsDcOJrkUNIuC4OZ7kGPItDKaJWjKWJBDIQZOk6UJQDGM4ymcNRjlCggxkaPAznidgtoINg3CCSQxDeXIdl8aA+gEA",
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
    #[cfg(feature = "heater")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIA4GMMgtDMLYRDMIAwC0NgiD4PAvgOBYch+BoIgoMQ1C4NYOhCEoUhKF4ZhuHYiiGBIjgkTQ2CAMQwGgLgwDGMYejWNIFgeN45DENI9j+QYzgKNZGiUMI6DYdgtDSTZDk+RYkgsNJUlaWIckKIJbjaJQ4mCV5ZmWIpRE0MpTDYYQyCCdZTlODZ1DIdo7nSdoXoEMYUnYSA0n+d6Ci0MhWDiiKBnmdoUGgM5sGiRJnE2KZxHYMqWpibwxDmdgwp2n4B",
        categories = "home,devices,travel",
        tags = "heating,warmth,comfort,fire,stove,electric,electronics,amenities",
        contributors = "danielbayley"
    ))]
    Heater,
    #[cfg(feature = "help_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ5C4MA5CAORhDMIIxDAII0DEIA1C4OIxDEY40DILYxDOQYyCKGwviKJIhiOJYnCITYOCAMQ3GiKwxkaHJJGgPoBA",
        categories = "accessibility,text,shapes,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    HelpCircle,
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
        svg = "gAPBwGEdBoCAZA9CITQxDcLg1CANYOEMMQ5CANwgDIMIOCCFQyDEIAxDEYwtDELgzDIMwuDINg1C0MguDYNA2iYOQtDQLohDiKorCIPg8C+A4Fj6QYGgiCoRg+DISheFIchiGpJh+Hhjg6LoOlWLQxC2Og1j2P5EkOBJFgmC4NkmZoji+DwwjYIJsDaJJekCYphgWB5khmIIlDUY4fnuGAgimDwyCCN5dj6c5CgKYp3gqIYbhmfYbh+gqAoagJVn+bAzC2VaGnKYKLnaRhNDKhKmmmbgtoKnYtkiJIOqCdKimOCo3DcOAzoULq4DKEwuDAN66joNA5oSfqXkkOJ9iSGI2pOVYzkmk6FiCEJXDUTY6qaF5qDeE57riIImDOSYRDQNa6i8MA4hUMQ4hCIwuDiOr0quvLjDcNpahEN4ruO+w4rKihsGkbhlCAeAygmpgiCAecLCLDcPDHDMOHjFcSnLBcHD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5HYMQwGEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNwiD4PAvgOBYxGwaRuGUIB4DKCQyDIIo7DGCQxDaQR5kMIg1kePZKjCMo3jmMRjGkchjGyOhjHmCQ5kEco+kEYx4lyTwvlSVpYjGNIGggIhtgwIAxDULQzC4MA4DadJ2niIIWiqGwuDgMg4hgTA2hUMZlmsPoBA",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2D4JguEx5hGCwiD4PAvgOBYcHAYR0GgIBkgoTQxDALg0DEIIqiwMRhDIII0DCL4vC0MguDgM46jwM4bh2IokhyH4GHiFQ2g6GIKDEMwuDWTIVDIMYOhCTpQlKHIegQZZGl6FISlaD4ZDEOJTk6UoHmaDZckeIYjiWJwiE2Wg5CCdxBDELg5nifJ+CCN6DnkIA1HYMQ0jONaCo2N40DIaKJGOUQ1o2fAwDUMgtC6DIvjGnA1DmQgvkQaJxiSJoolWLw1FYNaLjajo6joSKjlypg+gE",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIdg3GEMgghQMAghcMQthQMhIDWE4VhiIoaDKGxWh+HIihmFYbGgNwiD4PAvgOBYxGwaRuGUIB4DKCQyDIIo7DGCQxDaQR5kMIg1kePZKjCMo3jmNo4joeJJDEOZHkmQAgHmTQ4kGPJElmMQvlEZYxGMaRyGMbI6GMeIJlkIBjHmcpBHKPpPC+apsm6MY0gaCAiG2DIODULQzC4MA4DaiaLo2IIWiqGwuDgMg4hgTA2hUMZ7oEPoBA",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoSHIeYWhKFQiDOEoah6Eh3GkZB0GiDoQCIPg8C+A4FiwYxpHIYxsGUIBjiEOYYhuOIdjuLAvjKNI2iwcBhicIBkg4bQyDEIAxDULQzC4MA4DaU5VlcYQyCCXQwCCYAwC0MguDgMg4mETA2l4MYri2R4nD6AQ",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgzCAOAgDcIAxDIIISDENgiD4PAvgOBYHgmH4kgoeAxhAMgxCIIB4DKEAxi0IB5ioIoYi4eYxjgMoeiCJ4mgiCo7hCHYvjyM4uimK40jaRo/C+QQ8ieSIyk6SQ4jqNwxlqL43iyUZBgE",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCAMQ2HYLQ0CKGwviKJIhiOJYnCKKQyCAOBoC4MAxjGHI0GgPoB",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[cfg(feature = "inspection_panel")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIB3GkZB0GiCQxDiDBoGUaRnGgdIXhkIByg4IgyCIPg8C+A4FikcBhhUIBkgkTQ3CANxoC4MAxiiKovhWLowGiMo0DGNo4jqPIpC+PxokGMYzCKNQgkaOY7j2TJCk+Q5RE2RpUkiV5Lk0PoBA",
        categories = "tools",
        tags = "access,cover,tile,metal,materials,screws",
        contributors = "danielbayley"
    ))]
    InspectionPanel,
    #[cfg(feature = "instagram")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0Ig1CIIB3GkZB0GiCgyDCDRyHmCoMCAaBlGkZxoHSFIWCCCQiDKDYZicIg+DwL4DgWLRwGGEggGSChNDENggDEMQuDMNxBDQIJDDCPJHDEMguDYMwgDiRJQkaRgxjyO49j8Nx6iyLozhKLRsGkbhlCAeQxgoNguhseJmCIMQ3mmDR4DKCpumkMYpnMIpogyLQvmCYg+gEA",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig0CIIB4DKCgxDCDR5hCC4NHiCQiDEOQiD4PAvgOBYfiKBoZhGDIOhYNYUhYMoTgeGovh6IIliSBIGgiCophWCoziqCodg6GgxiyH4hjgPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDcYQyCCEgwCCFQwC0MoZEgNYRhOFoghiEgyHYMoehSIYfDIaAxDSJ4phWGokhkWgiD4PAvgOBY3jqBoIgoNoODUdoZjaOI9jyBI+gkTQxhIMQ1FYOZGjmSo3GMaRyGMbBlCAYx5gkNgiCAcoJDOYxjHiCZOlSWJalwPoBA",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHmDQzhEch4g0MoRiEIoeD4PAvgOBYoHAYYLCAZINE0OAgDcdg3CKKAvi6C4ti8aIxjMMQyjYdg0jqKY9GiP4wjIIhNDENpGDmSY8kAPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQyCAMQ4HYMxjDAIAuDYLg0g2GQxGgNB2C0MxoDOHogDIbAxhcLYnDQYYWDUIItCCEwxjELQ0jUWgiD4PAvgOBY6GMaRyGMbBlCAYx4gkMYtCIIBmGkbBsgkYx1HIchlG4dBDG8bBvHKTBjHmCQ3C4NZMHKCZkjmO5AkKRA+gE",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = "danielbayley"
    ))]
    KeyRound,
    #[cfg(feature = "key_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILg0CCDQ3GMLg5C2FIQC4NYWDkIAzg4IAwGwNYZCCIw1hMOYUheDQ1iCHYOGwLYeDeL4ShuFAtiyOIeg8MBMDgLo0ikOBjhuRo4iyIIyg4WgiD4PAvgOBZPlKBoICIbQxg+NAzh2TpQlWVIElaCRNimDwxDALg2hAIAxDgdgzGMMAgmuHwxm6bhoDQdoyGgM59DMaAyGwMYOC2hg0l+UZjD6AQ",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = "danielbayley"
    ))]
    KeySquare,
    #[cfg(feature = "key")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDULg1CIIBjHiCw3g6EBjHmCwxg2Dw+DwL4DgWB4eHAYR0GgIBkgsbQyDEIAyC0OQuDYIIyDYIoeC+JYniSJooioIhthuDgghUNQgDOSBMDIMpFGwLQzlCOIfjsaA+gE",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[cfg(feature = "keyboard_music")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAcoIgqDBoGUaRnGgdIJDENoMHcaRkHQaIJDIMIMHmCQ0CIPg8C+A4FikcBhh8IBkgkTQ2CAOBoieKQvi+H4ujAaIyjQMQ0jcaAuDAMYoiqPRoj+MYzCITQxDiRpIkqO5Nk+QZRE0MggDEMhoiKS48kCW5ClKNphHaOpMmcPJNmmUwwmAMptmWWpxkCc5EnaeJZnCcpdlSf5umaPoBA",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,keys,chord,octave,midi,controller,instrument,electric,signal,digital,studio,production,producer,pianist,piano,play,performance,concert",
        contributors = "danielbayley"
    ))]
    KeyboardMusic,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAch5gmC4NgiCoMGgZRpGcaB0gkMQ2gyDwiDSDB3GkZB0GiEAwCIPg8C+A4FiwcBhicIBkgkTQ2CAOBoC4MAwDGK4tjKJ4xjMaI1jcMQwjqPI+kCLAvkMaJFjSNgiE0MQ0kyPY/kGUZGlSR5WlgOJbk6XpSmGSJXmUMQyk2XZQmkPJSmuWAyCCbpwk+QpgnSRp2h2eZvlyfJfkSf5VjcN55DYaJKmiYIBA",
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
        svg = "gAPBsGkbhlCAeQxD0IgyDIIoHDKCoMg4eIQgsMYTgkIgzCIPg8C+A4Fh2IIGHiGQ2g6CIKDEOIohUMYXCCFIKieHYfgQZYijeB4ZiuLYqjCJYqDCE4ukONYjjmBY7iqLIxjwNI+CKL5EiqUJHjeSYkjyTYylKTYpl6UZTleIQ8HAbxsHkZxvG4IJngQdBzioMggnQMgwCANwgDSeYch6Z5pmsbg+gEA",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQ0hEcoTCIMoRHiDQzhaHIZhEaBlGkZxoHSDQxhoPg8C+A4FiwbBpG4ZYSDKDQyDCGwxjiER5jyGY6CAeI3hmK4tjKNA+gE",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0MQ4CAMQzGENggi8MAgjIMQti8NYwjmMo0jYLQ1GiDhahmG4hiOGhsGkboIHkMYMDmEB4k0IpPCAeAyk4LgwDGEB5leU5DC+SJKkeSZLl6VJMk6UJeDENZZluVZSm2YJiGUPoB",
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
        contributors = "danielbayley"
    ))]
    Layers2,
    #[cfg(feature = "layers_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYdh+GIxjMOAtjcORhimK4aiyHIODMWgiD4PAvgOBZSlWBoIgqJYnCAOYzkWM4qh6SIqkuZ4vDGQogkaOoWhaPZjDYNZBjKNJhkeSZnkyLwzGyYQ1k0NQ5lGU5YleBJZgkbZcjUMQ0mCN50j+ZZKnGaZrjaOJujyaIenQMJCneRqVnuG59n+kqCoSUpUokPoBA",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy,duplicate,triple",
        contributors = "danielbayley"
    ))]
    Layers3,
    #[cfg(feature = "layers")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDILg4DMIINDEOBhDKEQgDCGIYC0MQuDYNoYEyDYgDYLgwhQMQgimGYshqHYPGwOAuDUOAgDMLg5DGFYXi2GYdh+GIxjMOAtjcORhimK4aiyHIODMWgiD4PAvgOBZSlWBoIgoMoWDEN4eDULQ5C6XggDSZA2juFotDGTZADCIoql8Ng1lGU5YleBJZgkbZciqI5hmOZZnDGaYWmuLpuiCcJdoCdpUnoPoBA",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy",
        contributors = "colebemis,danielbayley"
    ))]
    Layers,
    #[cfg(feature = "layout_dashboard")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIB3GkZB0GiCg3g0aBlGkZxoHSCg5g2CQiDODR5gqIg+DwL4DgWJ4qgaD4RhMIoVgeIIMCCNQ0haGIahwIg1iOJQiieKYEHSLJFjeCgxjmNJKkCCwyjqGYbh2DYvhKFJCiiLZHgWDoQliMpSjyCo/CCJILDaH5Bk2C5akSK4BA",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gwdxpGQdBohSDB4gmCwgHKHgiDEIg+DwL4DgWJ4qgaF4ZhsIoVgeH4dgkMQ0gyDoQhKHIhiOJYnimBB0iyRAgkCOY/jeDIIiSSovhqPo7hGE4yiaKItkaBY0k+DIikySI1g2D5Vj6UYxhWQpagEA",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgxCIIIJCIM4NHcaRkHQaIKDeDRoGUaRnGgdIYg0eYKhEPg8C+A4FiaKYGgiCoMg6JIahyHogCKGQghOFYXjeIovDQIomiiBB0iYcBhhYIBkgoTQxDQIA0GiGZCkeFpGkgaJKkyTggDmUpBieVRoleSZLCKTZPDENZflSWJklmZpoCAMgwmyYZugEA",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMYMHcaRkHQaIJDeDBoGUaRnGgdIJDEOIMHmCYLD4PAvgOBYoiuBoPDENIkiaDIOiGEoUhaGIahyHogCKGYoiqBB0i2RIHiGMgghOFYXkCM4QkqNoQjyHYfjuQouD6AQ",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIBoGUaRnGgdIJDeDByg4IgxgwdxpGQdBogkMQ4CIPg8C+A4FimLIGhyEIShSFoYgyCIdDSGoch4IIgiKJAihmKYrgQdIukaDYljqB5LgyM4VheQofiGI42CCG4liiKovD6AQ",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQ3gwdxpGQdBogkMQ4gwch4hyDIhgoIg+DwL4DgWJ4qgaDoQhKFIWhiGoJDmDIICIMQ0iOCYLCCIIiieKYEHSLJFCCF4ZhsIg1gyL4RhMIoVgeHI8CCJAxDaH5ZiaKItD6AQ",
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
        categories = "food-beverage,emoji",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[cfg(feature = "library_big")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh4hWEhyhiD4SHcaRkHQaIOhEPg8C+A4FiYcBhiIIBkg4TQ3CAMx2hAIomC+LIiiuLRoi+MQyDALg0CCEAuDkYwuDILg1C0LgxkaUJPDaUgzGwLQxkgLg3GOTw1kuWZTlOWgzlQTAxlqUZgDGXphk2ZJQC4NpileWg5k+XZNk8MpSmqVpzFqOInjsaA+gE",
        categories = "text,photography,multimedia,maps,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    LibraryBig,
    #[cfg(feature = "library_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAch4g4MoShUIgzhIdxpGQdBog6EISHmDobD4PAvgOBYoHAYYgCAZIOE0NwgDcdgxDAIooC+Logi2LxojGMwxDGNo4jqPI+GiQIwjIIhtDENY2CAMggjmO4pksPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0NAuDkMwgimK4tC4Mg0jCMoZhuIYjiCIokiYIhtDGKQ4iwOQuDEN40DQLYpjWGgvjgaI6iOJYnkALpCCCVZXkuM5bjaTo7lGPInkSRpYkEM5KjGXJql6T4ah2BoIGOEoNg+CYLnWFYMDSXpwh+AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoIg+DwL4DgWHohgaCIKDGEw3GgMhhhOFYXg0IA0CAOIdh+JIeGwaRuGUIB5DKCQxDIIo+DGQZDCAeJACKQpEHiRgijWHgvjqPI5juPR5lCSI/gkMpIk+XpOkuX42lSWA+gEA",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ3EgNxBDUIITDAIIWDEIA3hoaAyCIPg8C+A4FiCI4GgiCgxhMN4dGGE4Vg2MYYDAaAth6IIigQaIgGwaRuGUIB5DKCQxh4IB4DGCQ4CKR5DCIMQ2kweZJk+N4hj2Pw+gE",
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
        svg = "gAPBwGEdBoCAZA9CITQxDYIA4GGDYNDAIITDEIIRHYNxoC0NB2C0NxhDIIIihOJQtDKJ4jiqJYUikMoZhuHYfhCF4UjaFg2C0Nh6CIPg8C+A4Fj4chlGMdAgHmCQ5CIIB4gkMpMHcaRkgWCQ0kwaBlGkZxoHSCQxlCPgvkSRo+GMaRyGMbBlCAY5OCKV5tkmcJMHKT49j+Z5pmsPoBA",
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
    #[cfg(feature = "list_collapse")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQzCAMQwCAMguDULYRDUTIMDUIg+DwL4DgWG4egaCIKgwMQ5hCEoUhKF4NDSGociGIIEiKCRNg4IA2GgMQxi+HYzjKBYHjWNwxDKOo8huPofgKM5CCKNoPDEOJHj2MYBA",
        categories = "text",
        tags = "items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold",
        contributors = "ocavue"
    ))]
    ListCollapse,
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
        svg = "gAPBsGkbhlCAeAyD0IgyDEIggHmCQiDaDh4DGCgxDCDh5haEgiD4PAvgOBYfiKBobhcMoUhyGIahEMYpgeEYMh6IIliSBIGgiCozg+Kw4i2F4/geK4Zh+IY4h8cBhHQaAgGSChNDQIA2GgMR2DSNAvkqTJJkuTZPCKUQghgaIpkaWxol2TJOlANpjDgSA0GMMAtDEIAyC0Mp3C0MxznWdQuDWeZ1lmaA+gE",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDYIggHmDg1hEeIODOERyhcIgxhEdxpGQdBog6EA+DwL4DgWJxwGGIwgGSDhtDMIAxDcIAyjgIA0C0NAiicL4tiOLIuGiMIOE0MY0DYaA4j+KJCGiRIvjEIpJjQMQyk2T5BkWU5GlWV41DiW5AlEPoBA",
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
        svg = "gAPBsGkbhlCAeAxD0Ig4CKBwygoMgxg0eYPCINoTgmFgiD4PAvgOBYch+Bh5hkMQyg0eIVhGKIZgwIIUgqJobh2IohgSI4li6MAiDGLoIgqPoqhKHIejeNoFi+FYXgeGQziiFQzC4MISi+GYXkSNYCjeSYxieVZdiyCpOg6YpSkONJGlqSI/CKY4kjGQZllOE4Vj2M5FiCAQ",
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
        svg = "gAPBsGkbhlCAeQyD0Ig2CIIB4gkIgxDKDR4DGCoSg0eYWCKEw+DwL4DgWHohgaD4XhOB4QDKKIahcOIUhuGIeiCBBliONYOhANwuDeDIOhsNAuDkM4ZkCQpEimCo7j0IoziSN4FjmFw5C4MA3jCFw2C4Mg0kWWZbl2SYRlSVpNh+T4CjiJoLl6EYohWCoshCMpnjWUIliqLIxnKJ5YhGL5OnaaZRi2Y5VleUgikuPpwCKQZDhmc5alyZo0iKg4GgiCqPkijQxpOYZrDGZKIoWi6Vk+AQ",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0Ig1CIIB5gkIgxDKDR5DGCoSg0eIWCKEw+DwL4DgWHohgaCIKDKE4HhsMQ5hSK4pg+F4dh+JIjgSJYvhmEIYg6G4whCDIeiCN42gWKoyjqSI9heLYOhCKAikKNQ8GMaRyGMbIGHKCg3g0Yx5koYx4kiQpVleWYemaWIGl+YZjhGKZbCIM5Rh+apogEA",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDIIggHiCQig2DwygoNYOHmFYLg0Pg8C+A4Fh2IIGgiCoMg4eIaDKE4QiYOYYhqJ4dh+BBliKNYHhqF4PhGJ4UiaE4lhIIoziON4FjmCoriiPZBj2L4/huRIekYPBwGEdBoCAZIKE0NwuDEMQgl+YRDDULg4DMIA4C4Mw5CANQgDEMJgnCcgyGMMAgDOaA3nuYJqn6gpyC6b56nwNgxC2aJqDSaA5C0MpgDGUwvleWYdpeWpcCITQxmwN5iDGfA5DYY5gpALg2nwMqQDELqtrCrqFDYIAwC2fA4DeuKAC2u6/osNp+rerwznyiqvqWsqVpqR4khGLLQjCSosiqHJUjUPoBA",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgxDIIggHgMoKDWDh4gkIoNgeEYLg0Pg8C+A4Fh2IIGhWCgxDmFIaDKGIIiaLIagwIodh+BBliKNYPhaMYHhaGIQi6Dh5hqE4ziON4FjyJoohmCorimQI5kCRY1h0YxpHIYxsgYYx4lAcoKDeDhjHmUoelaWJaD6AQ",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[cfg(feature = "lock_keyhole")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ2CIIBygwMYQGMeITDIIg+DwL4DgWB4bHIZRjHQIILg0MIQGgZRpGcaB0hiEIXCIM4QHKM4ZCAdxpGQdBohMOIahyIokhscBhj4IBkgwTQ3CAMQwFYNxhDUIJVDAIJYDGT5YDAdo1hsL5Hj4PoBA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = ""
    ))]
    LockKeyhole,
    #[cfg(feature = "lock")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIB4goM4NHcaRkHQaIKDEOINHKD4Lg2CQiDEMYNGgZRpGcaB0hiIw+DwL4DgWLRwGGFggGSChNDcIIiFYNxhDUIJADAIJDDGO5DDAdg0CKLQvjOFg+gEA",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[cfg(feature = "log_in")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzGgNBhDIIITDAIIWDGFIUHYMYRhOFYXCAMQth8aAtDQIg+DwL4DgWKhwG8bB5GwaRuGUIIwjUdBzgkMYYDeIoNDGE4+CAN4piuMIyjSNoqkyNx4DKCQzCIIB5lIIpDlUeAxj0NZVHmXZZDKSAvk8PoBA",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[cfg(feature = "log_out")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMgxEgNRhDKDQgDCFQgDELQyhoVoRhOE4WhYMYNhoaA0CIPg8C+A4FikcBvGweRsGkbhlCCL40HQc4JDENoYDeDYjDGE49CAN4oiqL4xjONYpkyNh5DKPAyCIIB5DGU5VHiWAig6WpSCIOZIC+Tw+gE",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[cfg(feature = "lollipop")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeILDEMYOGMeYShQPg8C+A4FgeGhwGEdBoCAZILG0MgxCCKQtDQLgzi2LwihoL4hiOIIiiSJgiE2EwghMYQyisIAwkSRg0kcIJIkWTAtDiRg2CCUZMkYMZCDCM4bjYaA+gEA",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = "danielbayley"
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ2CAMgwGgMBhDKDQgDCFQgDELQyhoVg4hKFIWhYMYNhoaAxDKH4TiGGIUDIdgxhGE4qheGYyhAIg+DwL4DgWOY8gaCIKDiGA4FYNIpheIokDIaJHjKSYsjKLw0jiOo/j6BJAgkTYwg2D5UjmO5ZjkYxpHIYxsGUIBjHmCYOCKax4gkMQ2nAcpulUL5lmeaZkmaaJqGOcgiDicJsm4MJ2niYZ7oAPoB",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "m_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKE0OAgDENhWDgbA0CCPA0C0NB2haKAvi6Eg+gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDILg1FYNhhg2DQwCCGAwC0MocEgNIVCCF4ZhmHIiHaDxjhgMQuiwOYijAMhoDeEQiD4PAvgOBY3jqBoICIbYMCANwtDgLg5DcIA1C4Nxhi4NIOkeUIag6HAuDANoZEyDQ3jaOI9jyBI+gkTQxDiIgxGEMwgmuK4khuWZrm2b4kDYdgwFqXo5mKNxjGkchjGwZQgGMeYJmYIqEHihw4okcoJDOep+oCgphgWB4JkGFodiwNQtp2epggE",
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
        svg = "gAPByGUYx0CAeA9CIMgiCAaBlGkZxoHSCQxDaDBygiCoMHmCQ0gwdxpGQdBogkMgwCIPg8C+A4FikcBhiMIBkgkbQyDIIA3C0OAuDkNwgDULg3GEMY8DQIJEDmRgwCCSwxC0MguDANpMEyNw3iiKoviMPoBA",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDcYYNg0MAghQMQthISA0hEIIThWDoYhgVg5C4NRDg0NwgDQIA1CANoliwSAxDgYwyC6HorDELg4iqKh2DgWgiD4PAvgOBZDHAbxsHkbBpG4ZQgkmTh0HOCQxDULA5g4OJZlsLAxDGQpEkmS5Nk+SIEgaCIKi+LYmlqLZgCCKZyiQNRWg+HIehaGIdHYMJikWaZDmaUB5DGVqACAeKICINgiCAeQyomkB4pMIg3oGhQ+gEA",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINAiCAch4gkMoMHcaRkHQaIJDENoMg8IoaCAaBlGkZxoHSGAzCIPg8C+A4FikcBhhYIBkgkbQyDIIA3C0NwuDEIAzC4Nw4GMLQuDWOwzC0MQuDKP5IkoOAgDATA2jiKIqi+FoujAaIygkTY3DgdgxDEYwwCCSpKDkII3mwaAxguKQvlgaA+gEA",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Mails,
    #[cfg(feature = "map_pin_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NAzCCDIOEEOAuDANgghSFggDCG4dDQIAxDAY4chcOIgDKGInGEMg5C4OYfiyLofhyNIQC0NQiD4PAvgOBY6j2BoIgoMYtDGJgxDMLg1DKEwuDaJZOheNYcDKHIhGGJomlMLQ4lwIA3C4OImmCYodjQLZJkuYQyjmO5Aj+BJBgkTZFg+dRBDILg3iaeZ7maHQ5iAMBhg+D5TCChQgn2fJ6lqf5hDcLQukibY8nGcIFgecwxDSLggi0Mg1oSiJ/DALZ5DENanpMNqVm8PBsGkbhlCAeAygkMpsCAeQxrgIq1r0Iq6Hmt7CmyOgvrGsw+gEA",
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
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgzCANggDkIISDENYTCAMgxheHIeDEOAghmIIViSI4Sh0Ig+DwL4DgWBxui0bIMGUIB4DKEA5CIIB5joIoij0eQxhAM49HiRQijyLQvjUbhljSNo+koNpIkqGZDkCK44kCWZNk+UYBA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDUIAzCAMgxhSFoYDkIg+DwL4DgWB4Jh6IYGgiCoMG6DoQDmGoZheGYSh2H4miMZYejcIB5DGEAzCKOwyhAMQwkAeJChENJGj0IoXjQL43jmKAgHiTI/juTJOlSSJEkAeZckqHpQigPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQ3C4MgxCAMQ1CAMguDYNgggwMQ0GEMoTCAMIfhAIAuDEMwthQMhMDQLg0hMLg4EGHYdiCIIPhcMhoDEMoch6NIiDGFYvGyQI3iSG4yiGNYjhqLopDENguDcOYQDUIg+DwL4DgWV5agaCIKDGD46CANYkjKDZWliXZcgSXoJG2JYQh2ZQ4DgLQ5i+aZZm2bIFgeCRNDiGBoDieprDwYxpHIYxsGUIBjHiCY6CKjx5pIN6UHKCZVlcL6JoujZ9m6YIdDEOB2icaAtC6nJqnyAQ",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[cfg(feature = "megaphone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4Mg2CCDIOCAMwgDEMR2DMbAxDQLobhOHQ0CIPg8C+A4FiOJoGgiCgyDGFQ1C4Mw0FYNhsC0N4xi4MguDAM4iiSKYogSKoJE2Fgug8MQ2C4OBhhSFAwhWFQtjAOAtDGSI/iWQ4jGwaRuGUIB4DKCQyDIIggHkMZlmgeZkCKZpoHia5wlqXpgD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bGwaRuggeAxgwOIQHmJ4NDWKgyhOLggHiMINDYIobC+IokiGI4ljUOQuDAMYQiaDA5i+R4qiySI5jsZY9iSM4sDGMo0jGQpECCK5KluQI4hyTw+gE",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMGgZRpGcaB0gkMQ4gwdxpGQdBohmGwgg+CgiD4PAvgOBYoHAYYgCAZIJE0NwgDgaAxDCJ4pi6IIti8aIxjONQxDKOI6igL49GiP4wjIIo0CAMQ2keO5KkAPoBA",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgyDAIggHmCQiDEMoOHgMYKDSDh5heEoUD4PAvgOBYfiKBobgoNoVhyGYHhGDIahGKYfiGBBliSNYPhEMQ4hWLoNg+HI7iqGAijOJQ+gEA",
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
    #[cfg(feature = "message_circle_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoICIbQxhWMAtiCNIPiSJoriqBIsgmL4ajCD4PjONolieOw+gEA",
        categories = "development,account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley"
    ))]
    MessageCircleCode,
    #[cfg(feature = "message_circle_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg1CCDQxGMLYOCAMAtDGFIZDGDoaHOGAgC6G4OiIIg+DwL4DgWJ4qgaCIKDEOQuDMIA2C4OBhDEMAuDSD46jyDwwhaFgtDKIpFiWJ4pgQaIskyB4JE0Mo7DkIIMg4Y5HiSG4hiMNYfkeGZhg6Jooi2ToFlCMA3C4MpWjIM45juPZWnOQZDkKRoZnqZZLiuApPi+C47g+UwuDmWQ1iKVpdlaWxzmIMaODWGp9megJpoKDY+myhQgDIMhsDSHYcDWlpMmiLpRhGdZYkKHaLpGkxzrGjalqef4tmoTY2DgIKjDecpAq2dJCsaSKSnySpngEA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley"
    ))]
    MessageCircleDashed,
    #[cfg(feature = "message_circle_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoIgoMQ1C4OAgDkLgyGEMguDWD45hiPQwC0M48DAbAtC6QQ0kUMw1kmNwuDSII4k+PYXj+QYgkENhslgIJBjqWJAjkYwxjULZjiCYwxC2OIMDKYA3iSJorD6AQ",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley"
    ))]
    MessageCircleHeart,
    #[cfg(feature = "message_circle_more")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoIgoOIWDIaAuDAMYkiaK4qgSLIJE0MYgj6Mo0jaJ46jmBYHjwMQ2jCQY1iWRIpgEA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley"
    ))]
    MessageCircleMore,
    #[cfg(feature = "message_circle_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALg1CAMQ0C4ORBDkIIWDAIIZhkOQuDEIAzg4Ig+DwL4DgWJIngaCAiG0Mggi+DIwDCI4liqKYEiuCRNDULg2CCPQ2EMMwgDgLpEDILovDGSYPDSEA2GwLYvj8NpTGOIZPDELg4CAN4+hCHoQg0MwtlsN41iaOQ+gEA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley"
    ))]
    MessageCircleOff,
    #[cfg(feature = "message_circle_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoIgoOIWDIaA4iSJoriqBIsgkTQxiAOB2jOJYnjgPoBA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley"
    ))]
    MessageCirclePlus,
    #[cfg(feature = "message_circle_question")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoIgoOQuDCExhDMII0hWFYXDULg4jQMRjhUMgtjQM5CjWJImiuKoEiyCRNDGIAxDcaIwDGR4nksPoBA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,help",
        contributors = "danielbayley"
    ))]
    MessageCircleQuestion,
    #[cfg(feature = "message_circle_reply")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoICIbQxhUMQ1C0M40CCNQziSJoriqBIsgmC4WDIaA3GGIIghWMYPg8dgxjqJ4+D6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley"
    ))]
    MessageCircleReply,
    #[cfg(feature = "message_circle_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoIgoMYgDgdg0iSJoriqBIsgkTYvhsaAuDAMY0ieOA+gEA",
        categories = "account,social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley"
    ))]
    MessageCircleWarning,
    #[cfg(feature = "message_circle_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBYliiBoICIbQxDUIA5C0NggDaJImiuKoEiyCRthONo2jiJ48D6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley"
    ))]
    MessageCircleX,
    #[cfg(feature = "message_circle")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3C4OQgDIMBBg6DgwCAMQghUNIWDYLgxEwMoPDIWgiD4PAvgOBQ+gE",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[cfg(feature = "message_square_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCAiG0MYVDiGYyhONY3jyO4Ej2CZAiAOIykQMpHjiSw+gE",
        categories = "development,account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley"
    ))]
    MessageSquareCode,
    #[cfg(feature = "message_square_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhWDUYwwC0MQuhQOQtDIIAyhgaAyCIPg8C+A4FiCI4GgiCgxDEIAzGgM4fiGJolgSJ4JE0MQ4iwaAxGOFIrDCGggC4OZBh6IIijSM4FgeNgyisOR2kaMZJgKNJMgqTggDGEJAj4LZDhqGIaGiE4wkiJJVkuKI3DSWg3mSL5HjKaY1CIbQ3m4LZtDQdgtDWZpziaV4LloMp9lKZxoD6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley"
    ))]
    MessageSquareDashed,
    #[cfg(feature = "message_square_diff")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ1CAMQ5C0MggDIVg1GGEYRDAIIZDGEoQGgMQ0haEoaiSHIXHYMQwiKGIlhCEgiD4PAvgOBYxjSBoICITQ5g0MBoDaMIyjeNoEjiCRNDGEQ3HaQIxjORZEgWB5HjwMQ3j+QZPjWAQ",
        categories = "development,files,account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add,patch,difference,plus,minus,plus-minus,maths,code review,coding,version control,git",
        contributors = "danielbayley"
    ))]
    MessageSquareDiff,
    #[cfg(feature = "message_square_dot")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDELg3CAMxIDUYQyCCFQwCCGAwC2FQyHYMQ2GwNAtDQaAxDKFIWhmK4YDKHB2hyDgiD4PAvgOBY0GMaRyGMbBlCAYx4gkMQ4CIIBygkM5GGMeYJDaM41jqPI+D6AQ",
        categories = "account,social,notifications",
        tags = "unread,unresolved,comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley"
    ))]
    MessageSquareDot,
    #[cfg(feature = "message_square_heart")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKiwLg4CANwuhAMZBiCRw4iCFZNhkLg2haHguDILgzC2VpYDMYZKkmSInhaT4glUOBMDGEgxDMbJVDeTw3GMLg5lgOZBk+R4YmSNY3jwPoBA",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley"
    ))]
    MessageSquareHeart,
    #[cfg(feature = "message_square_more")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKDiDgwGgLgwDGNY3jyO4Ej2CRNDGEgxkKRJGjaOJLkqBYHk0MQ2kGQ5FkeVo6gE",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley"
    ))]
    MessageSquareMore,
    #[cfg(feature = "message_square_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUVg1GEMgghQMAghcMAtDKGxIDkIg+DwL4DgWIYkgaCAiG2FIshcMgwiCIoniaBIogkTQzC4NggjkNhjC0Lg0C4M5ADYLg4kWDpBHYMQ2GwNAtDQaAxjCIYjjUPoBA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley"
    ))]
    MessageSquareOff,
    #[cfg(feature = "message_square_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKDGEg3HYNo1jePI7gSPYJE0OYODAaJEjaOJJD6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley"
    ))]
    MessageSquarePlus,
    #[cfg(feature = "message_square_quote")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKDiDgyi6FIwhOGRWDgSA4jWN48juBI9gkTYskGQ5FhWKQykgaIZkyOJQD6AQ",
        categories = "account,social,text",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,blockquote,quotation,indent,reply,response",
        contributors = "danielbayley"
    ))]
    MessageSquareQuote,
    #[cfg(feature = "message_square_reply")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCAiG0MYVDcLQzCCRZHjWN48juBI9gkTQxDeDgzHYLQxi6FIwhmGYckmOJND6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley"
    ))]
    MessageSquareReply,
    #[cfg(feature = "message_square_share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDIdgzGEMgghQMAghcMQthQMhIDcbAtDQIA0FYNYThWGIpg0MobGgNwiD4PAvgOBYxjSBoIgoMQ2CAMxoDUdg1jCMo3jaBI4gkbY7CAOAgDULZCjGM5HD6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,network,forward",
        contributors = "danielbayley"
    ))]
    MessageSquareShare,
    #[cfg(feature = "message_square_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKDEMwgDiHI1jePI7gSPYJE0MQ3g6Gw3kWOJJD6AQ",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley"
    ))]
    MessageSquareText,
    #[cfg(feature = "message_square_warning")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCIKDGEg3HYMo1jePI7gSPYJE2QIODMaAuDAMZFjiSQ+gE",
        categories = "account,social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley"
    ))]
    MessageSquareWarning,
    #[cfg(feature = "message_square_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgWNo5gaCAiG2LAuDUIA3kILZDDWNY3jyO4Ej2CRtDmQpElOSJKjiTg+gE",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley"
    ))]
    MessageSquareX,
    #[cfg(feature = "message_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDUYQyCCEgwCCFQxC2EgyEgNxsC0NAgDQVoQhqFomg0MoZGgMQ0hGE4mheLwyHoIg+DwL4DgUPoB",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    MessageSquare,
    #[cfg(feature = "messages_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA5GEMgghEMAghQMQthEMhIDYbAtg0NBWDQYwwC0MQuiYOYYhKGBoDiEIShWMQxjAMh2DUWgiD4PAvgOBY6j2BoIgoMQ4g4aAyi+E4yjQdgxDGHQ0h4aAtDaSYxhaGIYHaJY5juQA+gEA",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubbles,copy,multiple",
        contributors = "danielbayley"
    ))]
    MessagesSquare,
    #[cfg(feature = "mic_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIA4C0OQuDANAghEMA2GEMguDiDYahwIAwCAMYgCAMwuDkOIlicOBMDENoiDIIg+DwL4DgWMxjGkchjGwZQgGMeIJDENwij8eYJkQIBygkNYyjSOY7j0PoB",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Mic2,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDIIggHgMYKg2D4JguEx5hGCwiD4PAvgOBYcHAYR0GgIBkgoTQxDgLg4DkIAxDMLgyDMQQ3C4MQyCCNo4CAMI9j8MYujgdgtg2HAviKJIhiOJYnCITQ1i8MB2DIYQ3jqP4+j6PA1huHZJGiS4kiaKAxlEOQuDMNBWDUYQzCCb5aj0LQ1C4Ng4C0MZpDOXpIkyYpNiiLg5HYM5unCWY/nWPAyjeRpfn+AoEgaGIKkGDoQpaEx4hWOIOgiEqPh6kw+gEA",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyGEMwghEMAghQMAthEMx2DeEIShWH4UDaFRWDWHYTiCGIYFoIg+DwL4DgWLYwgaCIKDEOQgDEMB2g8Nwgj6Fo5C0MQ0hUdgtDKLIujOLRsGkbhlCAeQygkMpJCAeJUCKDAilIMYJjeXR4l+W5Ji0L5OlAPoB",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goNINGgZRpGcaB0goMQ1g0dxpGQdBogoMgwg2CYLCIPg8C+A4FimLIGh2H4hCIOINgiGYNg+NIlgoNoShSFoYCIN4oiqL4pHAYYgCAZIKE0MQ4CAOB2kSKQvkmIJIkoaJMk4NggDEOR2gyVpYGiWpLk0IpPlGYZjkWV5bD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMhoDgIg+DwL4DgWEoVgaCIKDmDR2DELgzDQMxNDENYdDILg3DgORhDQIIuDAIIxjELg2DcMoNC4MgxDkbI1DUNguDkOA0i2L4ykgMQgjWN45DIMoeC4MQzDETQ3C6DJXDgbAtlIMg4lIOQyEGLowkiMQ3CAMQwjoMQyFYMgwGGOI4jOSJ0GgNpzg2Z53C2UAtDOEYThiF4EhmCZWmoNRhkENJpo6aZ2kqJoxpGOKXn2MQzC6Ig1p0Mw1oOFKHhIbBpG4ZQgHkMYJDIIggHgMquq+satCKtR5rOuKvhIL6nqkPoBA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CAMYSDEMIThWFwyDAIg+DwL4DgWB4Jh6IYGgiCoMG6DoQhuE4XhSL4YCANIdh+JojGWHo5CAeAxhCFAiCAeY/CKFpCHkMoQDOQh4koIgyDGNgvjmO4oj2T5HkORZRk2RZMkOWY1h6VIoD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgg4aA4hmG4hiMPoBA",
        categories = "maths,shapes",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    MinusCircle,
    #[cfg(feature = "minus_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAch4gkMoMiGCgiD4PAvgOBYoHAYYRCAZIJE0OAgDEMhohWKAvi6EQ+gE",
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
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMoQCANAtDQIg+DwL4DgWGByGUYx0CAeIJDIIggHKIwiiUIBoGUaRnGgdIJDGFggHcaRkgWJAwiYeYJDOF4Zh6IIYhuBoICITQxhEMQ3HaFoYhqBBokWU4HgkTQ4hAMRoDiQZShyAQ",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgiCAcoMDODxjHiDAxDkIg+DwL4DgWB4bHAYR0GgIBkgwTQyDIIAxDIdgzGGK4rDAII0DELYyEgNIxCCM41iyOI4FYNY8j6No9jgaIZhsL4iiSIYjiWJwiE2LYsDcdg0hqHJOGiUIkiaKA4j0MRoDiW5NlEPoB",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAxDMVg3CIPg8C+A4FhSF4GggIhtDENYODALQzCAM4jiOE4VhqFByGUYx0CAaBlGkZxoHSCQxDQIggHIeIJDKO4+CKQAgHcaRkgWPwwjseYJDOKQvi2L4ZgSG4JguDQxDcdo6hSFpVlSBYHlcOAgDIMRoDiUIrgEA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDAIAxDMVg3CIPg8C+A4FhSF4GgiCgxDSDoQhKFIWgQaIUHIZRjHQIB3GkZIFgkMgwCIIBoGUaRnGgdIJh6NByHiMY0kAIgyjQeYJDOE4ViiKoZiWB4JgsMoODcdg0kqJIYgKT4cE0OAgDIMRoDiWIaD6AQ",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIA3CANQgDMLYRDMWgiD4PAvgOBYZHIZRjHQIBoGUaRnGgdIJDENAiCAdxpGSBYJDIMItHIeIzi0eYJDOLY4CIMoYhqH4hhmHIGggIhNDEMggDENx2iyGYbgQaJGlWB4JE0OAgDIMRoDiQpUh2AQA",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIA4FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdg3h+FYiiAMhoDgIg+DwL4DgWMIzgaCIKDGFgxDkdgtDMLg5DYIJADENYvjGNo1gSN4JE0NwgjwaJHjCMpMjAchlGMdAgGgZRpGcaB0gmOgiCAdxpGSBYJDaZh5mQMpmHiZJtCAcpzCKcZVlmWw+gE",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NQgDIMBIDgIg+DwL4DgWFYYgaCIKDENwgDkaAuDAMYUhaG4VHIZRjHQIB4gkMQyCIIB3GkZIFjEMI0GgZRpGcaB0jENo0HmCQ0jQcowCKM4VC+K4thqBIcgkTQ4CANhIDQYQyg8IAwl+Xwtl0Mh2DaXJemCapeDIaJIk6KQ8GMaRyGMbBlCAcoxjQY5GCIMQ1nyS4ficL5znWdw+gE",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAeIJDmDB3GkZB0GiCQ2gwaBlGkZxoHSFwiD4PAvgOBYiiWBoahyHoJDENIMHKDgiDKDIICIM4RhOFYJDIMIMjKNIiiSBB0iIcBhhUIBkgkTQxDIIAxDcdovkKR4VkaSBokqTA4CAMgxGgOIhiOVhoD6AQ",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAMQwCAMwtDOEIQCIPg8C+A4FheGoGggIhNDEMoNDMVg3haGIdhcchlGMdAgGgZRpGcaB0gkMQ0CIIB4gkMo6HmCQzjodxpGSBY9DCOhyjwIo+hcL4si6HIEh6CYhiMMQ3HaOZPiqApUgeVg4CAMgxGgOIohmVA+gEA",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDQLg1CAMQyg4LQ1hQIg+DwL4DgWGIbgaCIKDmDoQhKD4VDWF4Zh6GByGUYx0CAch4gkMgiCAaBlGkZxoHSCYMjYdxpGSBY0DCNozCKNQgHmCQzikL4ti+HYEh+CRNhGEA3HYNJPiuApUgeVg4CAMgxGgOJdlQPoB",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgyDAIggHIeINDKER5g0M4RGgZRpGcaB0g0MQ0hGFIOCIPg8C+A4FikbBpG4ZQgHkMYVDGFwyjaJY5CIMQ2iWNQiDiKIqi+MYujCMh4kEMYWCAeI8k2OI6jOTA3kQL5GGUPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMYQGMeITg8Pg8C+A4FgeGodgaCIWhMOYVguDYPhGEwihqHIEiKIIwgeCYog6FYXCINYQhKDYthuIYfgE",
        categories = "layout,development",
        tags = "ellipsis,menu,options,loading,loader,throber,etc,et cetera,operator,code,coding,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[cfg(feature = "more_vertical")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDEIggGMeYLDEMoOGMeIShQPg8C+A4FgeGodgaCIWhiDoKgyFYRCINQihqHIEiKIIwgeCYSimEg5hWF4MhmG4hh+AQA",
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
        contributors = "danielbayley"
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAeIODWEh3GkZB0GiDgxDSEhyhQIg3hIeYODIIg+DwL4DgWKhwGGGwgGSDhNDEMggDYdofioL4whsPoBA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig1CAMQxCCEoShSFgiD4PAvgOBYHgmHIfgaCIKgwboOhAMQ5hMM4Ti2LIujCG4diSIRlhyOAgHgMorDkIggHkMYQDWQR4kSEZBHmPgiiyNQvjgPoB",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDMIA1CAMQ5hSFoYDEMQiD4PAvgOBYHgmH4igaCIKgwboOhCHIahmF4xDOHogieJBlh+OAgHkMouDkIggHgMY/kEeI+CINZBHmRJJjUL44D6AQ",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDgIITDIMggDGF4ShgNgiD4PAvgOBYHgmH4igaCIKgwboOhANoUCCGoXi8MYdh+IYEiiJQ8iSCh5DGEIZCIIB5DKQQykMeJGCKFpJkCTIeiCPQ+gEA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig4CAMYSDEMggDKFwxDaEw4CIPg8C+A4FgeCYgiOBoIgqDBug6EIShyFoYhOHA2h+IYoiUZYgjoIB4DKEIWCKPgxkEMpDHmRQikcIB5kCS5HiAL46D6AQ",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig1CAOQgDIIAxhaEgxDUIg+DwL4DgWB4Jh6IYGgiCoMG6DoQhSGoWhaGwghyHoggSJ4kgKN4jikb4Ng8IoyDGFIYhWFpEDmHYfiaPIljuKILj6K5AkOE5GheMZEjSS5PjmPAgHkMYQhgIggHgMoQDIMplHiYgimuYJokGa41k0PJfmeY5wm2eplHmcpqn6bp0h+TYBA",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeILDiDhjHmCwxhMPg8C+A4FgeGhwGEdBoCAZILE0MQyCCGBWDIbA3CCDYaC+IYjD6AQ",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDQIggGMeILDEMoOGMeYSDgIg+DwL4DgWB4bHAYR0GgIBkgsTQxDYIAxDgVoUhsL4iiQPoBA",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWG4egaCAiG2DIMhOFYahyIYbGMaRyGMbBlCAYx4gkNgijQeYJg6ORygmGYbC+LowjKLYvjGM41jwOI5GOOwiDGOAgj8IpBhyRJJD6AQ",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5CAMQ4FYNRsDEMgtDIdgxDMIg+DwL4DgWGxjGkchjGwZQgGMeIJDYIonHmCYOiwcoJhmGwviGI4liCIokiaMgihmLYviuJ4pCKMI1jePA+gE",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ5C4MwxCCDIOCANQgDIMRsDcLQ0CANwgDQLQxC4MYZDOIg3CIPg8C+A4FimLIGgiCgxDQLg1DMIA4C4OA4CAMQyhUbIgiYIIliOKIqi+KRsGkbhlCAeAygkMgyCIIB5lEIpTlUeAxlKVR5l2WZHC+S5ND6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ4C4NAzCCDIOCCDwxDEbA4CAMoZhCGQuDUNwtDWDQzCIPg8C+A4FiaKYGgiCgxDcLgzDkIIVC4N4PDKGgyGwLQ5jeDw0C4NgxiWJ4siYbBpG4ZQgHgMYJDIIggHmUAilKTgylGWB5lqV5SiYL5KkwPoBA",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIBoGUaRnGgdINg8IB5g0MYZHIeIchCIAih0Ig+DwL4DgWJ4qgaIwyhCCIKgyDoQhKFIWhiEIbiSHojDGJooi2LIEgaN4VheNYHgmC46CCIw5juDYwCCH4hieKZFiccBhgsIBkg0TQ1CCHR2C0MxhDGZAgDCbJrDELQxGgMQymma5tm2ap6HYM5BC+XILluXRol+YZ0mQMhWDifqAGgPoB",
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
    #[cfg(feature = "notebook_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDMLg0CAMhIDYYQyhAIAwheFwthUMh2DGE4chmGIYhwaAxDKFIWiOGQyhsdgtDeDgiD4PAvgOBY0jeBoIgqFQ2GgNIzjWOo5gSO4JE2FQxDCQJCjaRpFgWB5IkoNJNjST44gKRpTj0IAxDiV5DlCW5SjyCw4g6EAuiALgxDea5viKXwgDOdRMh+XwxGwLYPDGX59FqTpEgEA",
        categories = "text,social",
        tags = "pencil,notepad,notes,noted,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,eraser,rubber",
        contributors = "danielbayley"
    ))]
    NotebookPen,
    #[cfg(feature = "notebook_tabs")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAch4gkMgiCAeZAkKPwihAIBoGUaRnGgdJADCQh3GkZIFgkMQ2iAL45juL4lgoMQ1CAMh2DKUosiKAokjGYZjDcaA1luaoYm0TZiigMpxnON51hqd5jDGcJymmLoBA",
        categories = "account,connectivity,communication,social",
        tags = "notepad,notes,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking,alphabetical,a-z,organizer,organiser,planner,diary,stationery",
        contributors = "danielbayley"
    ))]
    NotebookTabs,
    #[cfg(feature = "notebook_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAeYJDIIggHIeI/kGRAihAIBoGUaRnGgdI/DCQR3GkZIFgkMQ2iAL45juL4lgoOQuDUII0DWW4igKJIxmGY4oDISJZmiN4YmwTZimSWZxhCLIigE",
        categories = "text,social",
        tags = "notepad,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,lines,opened",
        contributors = "danielbayley"
    ))]
    NotebookText,
    #[cfg(feature = "notebook")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCANhoDQIg+DwL4DgWEoVgaCIKgwMQwg+EYThiF4EhmCYLCAMQ0h+EoUiSI4FgeJocDiK4hi4PByGUYx0CAaBlGkZxoHSCQyDAIggHcaRkgWCQxDaRxyHiRJHHmUwglIIoQiyOY7i+JYKk4IAyHaRYgi2FoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMh2DQIg+DwL4DgWEoVgaCIKDEMoNg+EYThiF4EhmCRNDENoehCEoUiSEhyGUYx0CAaBlGkZxoHSCQxDgIggHiCYQCAdxpGSBY7DaPh5kGPhykAIgyiAL4wjKI4FgeJoMDEMBokmLIigKJJYgqWg0GiPZfi6YZXhqCwgjwaA1lKIoBA",
        categories = "text,social",
        tags = "notebook,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,homework,research,lines,opened",
        contributors = "danielbayley"
    ))]
    NotepadText,
    #[cfg(feature = "nut_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIA0FYMgiD4PAvgOBYThaBoIgoNQgDEMB2DQYQ3C4MAwDQIIkiaKAwCCLYtDULgyDcNwgDYLg3DgNxjC4NIMC6Hw0C4OAwDKMg5g0MZAkULg1kgTIMCAMgyGwLg2DIMQtlaWI8DOWgzlYOZaDQOA4h6SwylqTgyiOJQximbounKLZCgwNQtkYOQzDmEoUhmGIEhqCYLDmHogmAMw0DOfYVoGgIFgeg5RgwYwtkqiZik0N5ekoOQwneSomneRqjmoNI3p6aahmSZwzDWXqklKlpuDiWg2DQNKzqeswwrWaaqiUMg4jiWZKDYOQuDkMa1DOHpnr2xYlqySg3DUN54iWzZps2QA5rmOA3jcOK5koNQ1m+1JikaWBtDGxg1iiMQ5DmO5DjqY7WmewpaDiypnuYNqzjmNahDOeQxDaZw2DizZKDG+cNrMMr0rwM8BtQNa+tmeAtl6Tccrmua1t8N4sroNqhwu2MPt2EYTo2Fw8GwaRuGUIB4DGCYRCAec5CLOx4DLOs7HnQs/y6FMzzUPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwM4QGMeYMg4Ig+DwL4DgWB4bh6BoIhaDA1hCEgig+CYLg0OYahyIoggKBIjgmF4uiiDIrgqJowh2NYzHAYR0GgIBkgwTQxDALg0CAMgxC4ORhksIJVDAIJYlgOZSDQMQtDENZNDENo/kORYbmeRpICKSgzC4NZPC4MZUliV5ZlkLZcDiXpWmKXplhsL5qD6AQ",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "outdent")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig3CAOAgDMIAxDIIISDENgiD4PAvgOBYHgmH4kgoeQyhCGAiCAeQxisMotHiKgiDEMYzjAIgyjiHwvieJoIgoeI6jyLYphCHYujqSo0iuPYgkAPInCCRIQkaVY1jeR5aDiR46DGXo+kCAQA",
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
        svg = "gAPBwGEdBoCAZA9CIbQxDYIIMCAMoQCANAtDQIg+DwL4DgWGIbgaCAiE0Mgxg4MBWDgYYRhEMAgiwMAtDGMAuDcMxsC0N4VimEoui0LYrjYN4TEGKotkWLAzCAOB2iiRI8iyJAxjONZBDSOorkaOxsDKMgxhaGIagQaIdmGB4JG0NwuDWEwuDKQQ5CANQuDENYXhmHodG8bB5GwaRuGUIBwG+fR0HOCQzmyb5BDGEaLhAMIziQN51hqeZ7n0ZYYnyfggHgMoJosIggHkMYJDIMqhHmngiqCnKkqup5fpqmIB",
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
        svg = "gAPBwGEdBoCAZA9CITQyDALg5DEIA4C4OA0hELg1DYIAyC4MgzGEMYODMIIgDmIgwCCJwwC2IA4hAMBMDMLoQDSMoehsMQyhqMo5imKAtC4MA1CCMQ2DkbI4hyOQ2iEYY5jyKJQiSFYvDKEJIDUMZNjuOo4lCKY/DAOQtkQNxaCIPg8C+A4Fmia4GggIhtjGYYWhSI4bDMNQtksNpZiSIp/l6I4jhOLhskSQoklqXY3k+J4QjKQwukUTISDegI0DcNZNhqggxiuDpTjCd4XlmjZco6g5AmKZJmmiaoEGiaBsGkbhlCAeAxgmOAirgMq7DKvR5r8IgxDOwq6CIMrBq+tK2m2sYHgmC6PjENR2jEMw3loMIZhu3aeqAMZWhMMxsnukgwDiH4hoSJbhiANw4ii5w2mOQA4EG37ekCGY9jOI5LDgNxWsaF5nmmbg+gE",
        categories = "files,development",
        tags = "box,container,storage,unpack,unarchive,unzip,opened,delivered",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIIMGgNgiD4PAvgOBYThaBoIgoMQ5g4Mx2hGE4VgQaIYiWB4JE0Mgxg4MBWDgYQyCCMwwCCNgwC0MY6C4NwzGwLQ3C0NIyjSN5HjmNZADcIA0EGM41kgIAzCAOB2jGUJSjaLQxj2P5MkSWY4keSgyjwMQ0hKFIZieBYpCIbQ3C4NZNC4MpMh4NQuDENZqiSF4CG8bB5GwaRuGUIBwG+hh0HOCQznaHpMDGM6UjQMI9i0N5+oqg6FoeE6fogeQygmlAiCAeAxqYMqoHipQiqcIB5qsIgyq2I6iD6AQ",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDAVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoisModDENAiD4PAvgOBZMk+BoICIbQ3C4NYuC4MotDkIA1C4MQ1kuTZSlEbxsHkbBpG4ZQgHAb5sHQc4JDOW5ei0MYUnqFQwh6DQ3mSTpomqbBlkya5tCAeQygmegiosMYJDIMqQHikgio8IB4o2maVkwL6JocPBjGkchjGybhjHijg4likBjHmjpgmMIBypOr6gqWp6plGBJTgmC5+lyDpXsSlIODmgpmgEA",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIAxDAVg4GEMgghQMAghcMAtDGGwuDcMxsC0NwtDSE4VhiKIahaIQ3CANBBhSFopCAMwgDgdoSjGM4Xg0MYeiCLYljqGYoisModDENAiD4PAvgOBZMk+BoICIbQ3C4NYuC4MotDkIA1C4MQ1kuTZSlEbxsHkbBpG4ZQgHAb5sHQc4JDOW5ei0MYUnqFQwh6DQ3mSTpomqbBlkya5tCAeAxgmegiosMqODKkB5pIIqPCAeaNCIMqUkwL6JoeAoElOCRtDGeY1lkNRtC2WYXDWr6CmaAQ",
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
        categories = "design,tools",
        tags = "fill,paint,bucket,color,colour",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[cfg(feature = "paint_roller")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDYIggGgZRpGcaB0g2EAgHiDQyhEcocCKHggHmHQiD4PAvgOBYoHAYYLCAZINE0MQwCCDx2C0MhhDIII9jaNgxj6OhoDiPI+CCQJJkMMhWDeR4/ksMI6kSOonimLoLiiK4GgiCoMCINIRiEOIRhOFYXhmEYlg6Gogg0MZXiqBB0D6AQ",
        categories = "text,design,home,tools",
        tags = "brush,color,colour,decoration,diy",
        contributors = "danielbayley"
    ))]
    PaintRoller,
    #[cfg(feature = "paintbrush_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIAxDkLg5FYMQ2GgMxhDIIIZDAIIchwMgtDIdohEgNR2DIY4cDELorDmGovDKFh2DOEYYi+KodCCDQwFoIg+DwL4DgWP5CgaCIKDaDgyFaMQxiIMQwj6QJFkSBJGgmC4NiINJSkGVpVgWB5YlCGonl2VIBA",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Paintbrush2,
    #[cfg(feature = "paintbrush")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLgzDcIAyC4NgzCAMQ0CANxsC0MQuDUOYch4ORhDKEQgDCJ4nC2Eg4iUMBMDiGRsDkII0h2H4hh+JImiiPYqiwMhMDGEAxDAbA0g4NwtkiD4kC4MYlhKUIpDGKgzC0MxaCIPg8C+A4FlyX4GgiCo0DgY4rCCV4YDOHgthANBsjGRRjDKHAgDYLQ1nib5bl2YphgSY4JguSJ7kOHggoaFg1n6XqCD6AQ",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[cfg(feature = "palette")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINguDUIggHKDIPhEZhpGwbIMGMdRyHIZRuHQQxvGwbxyhEYx4gwMQzhUPg8C+A4FgeL4ygaCIpisN4VgmCwiDEMI8hMIo8heGYbh2H4hiOJYni+MYEjeNZRgeCYqCIOJFhiGgihyHogiKJImhGQ48gqDI7hCT42jSApUjiPgxDKQoUhCVoMg6dpGlyXpKmGTQimub4vHAYR0GgIBkgwTZyCAMhDnmjqSpEMggnIcw0g+lgwpumxjC4OQyDYIKcDELg2DQOAtC4Nw0qOpqoqqsA4DipKrDQMw3qsMaqC4OAzDWt65C2ppysELgyDmqwyDiygurivQ2DUMrCrILrGGGsA0pap7bpypbcDYNrWuIOBoqYOQ5DYY4tDANQxqQIA1g8NbBnMNQwDO8r0sG871DQQwyui0ggg4MKNDGO6uvClaNDIeqBjChaHD6AQ",
        categories = "text,design,photography",
        tags = "colors,colours,theme,scheme,paint,watercolor,watercolour,artist",
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
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwaBlGkZxoHSE4VCAeYJgsIByggIgyCIPg8C+A4FikcBhhEIBkgkTQzCAMQ1GiFIoiqL4Ri6MBojKCRtjgIA4C2NgzkmSY8C+PhoD6AQ",
        categories = "layout,arrows",
        tags = "drawer,dock,hide,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NHcaRkHQaIKDEOINgkIoRCAaBlGkZxoHSGIaD4PAvgOBYoHAYYWCAZIKE0MQ0CAMQ1GgMQiigL4uhaLYvGiMYzDEOY3jmDI9j8aJBjCMgiE0M5IGiSopkyTpDlATZHjiOo8leQg+gE",
        categories = "layout",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomDashed,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHmDQzhEch4g0MoRGgZRpGcaB0g2D4RhgIoVD4PAvgOBYoHAYYLCAZINE0MwgDENRoiOKAvi6C4ti8aIxg0bQ5jYMAgDMLY1ksIo7j0aA+gEA",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goM4NGgZRpGcaB0goMQ4g2CQihEIB3GkZB0GiGYbD4PAvgOBYoHAYYkCAZIKE0MwgDENRohoIooC+LokD6AQA",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAeYJgsIBoGUaRnGgdIJDEOIMHcaRkHQaIYhoIByggIgyCIPg8C+A4FikcBhh8IBkgkTQ5CAMx2hmKIqi+H4ujAaIygkbQxDYIAxDULQzkqN5KjsL49GgPoB",
        categories = "layout,arrows",
        tags = "primary,drawer,hide,chevron,<",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHiDQzhEeYUhEcoTCIMoRGgZRpGcaB0g2DwiD4PAvgOBYoHAYYLCAZINE0OQgDENB2DGJ4pi6C4ti8aIxjONQxDkdodigL49GiP4wjIIo0CAM5HjuSpAk2QZPlGRo6kmSw+gEA",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftDashed,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKEYUCIM4RHmDYaCAaBlGkZxoHSDYPCIPg8C+A4FikcBhgsIBkg0TQ5CAMx2ieKQvi+C4ujAaIyg0bQxDQII2DONwtkmGo7j0aA+gEA",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal,chevron,right,>",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEhyhQIgyhIeYVhIdxpGQdBog6EAiD4PAvgOBYoHAYYjCAZIOE0OQgDMdomigL4uiMPoBA",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggGgZRpGcaB0g2D4RHIeINDKER5g0M4Rh0IojD4PAvgOBYoHAYYLCAZINE0MQ1CAMx2hqKAvi6C4ti8aIxg0bQ4CAOY3jcLQzjcIo7j0aA+gEA",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide,chevron,>",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeIODOEh3GkZB0GiDoQhIeYVhIcoUCIMgiD4PAvgOBYoHAYYbCAZIOE0MQ1CAMQ0HYMYnimLobi2LxojGM41jcOR2iaKAvj4aJAjCMgijSNgzkiPJLkGTpClCUggkeO5KkwPoBA",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightDashed,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggHmCYLD4PAvgOBYoHAYYWCAZIJE0MQ1CAMx2hkIooC+LoWi2LxojGCRtDEMAgjULQzkqN5KjuKY+GgPoB",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal,chevron,left,<",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gweYJgsIByggIgygwaBlGkZxoHSE4VD4PAvgOBYoHAYYRCAZIJE0MQ1CAMx2hQIooC+LoRD6AQA",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoHAYYWCAZIJE0MwgDkaIZieKYuhaLYvGiMYJG0OQgDENggDMLY1kuOgvjwaA+gEA",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_dashed")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINHcaRkHQaIVhcIIJCIM4NHmCojD4PAvgOBYoHAYYcCAZIKE0MQ0CAORoDEIooC+Loci2LxojGMwxDmNxogyPI+GiQIwjIIhNDOR5JimS5NkKTxNkaOI6kqQQ+gEA",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopDashed,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEhyHiDgyhKGQihYIB3GkZB0GiDoQCIPg8C+A4FikcBhiQIBkg4TQzCAORoieKQvi+JIujAaIyg4bQxDUIAxDQLY2DOSpKiiKo9GgPoB",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINgkIgzg0dxpGQdBohWFwgHmCobD4PAvgOBYoHAYYgCAZIKE0MwgDkaIWCKKAvi6IA+gE",
        categories = "layout,design,development",
        tags = "drawer,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[cfg(feature = "panels_left_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEhyHiDgyhKGQihYIB3GkZB0GiDoQCIPg8C+A4FikcBhiQIBkg4TQ5CAMx2ieKQvi+JIujAaIyjSNgxDUaAxhuO49GgPoBA",
        categories = "layout",
        tags = "drawers,sidebar,primary",
        contributors = "danielbayley"
    ))]
    PanelsLeftBottom,
    #[cfg(feature = "panels_right_bottom")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMg+CoMHcaRkHQaIJDEOIMGgZRpGcaB0huHQ+DwL4DgWJxwGGGQgGSCRNDMIAxDUaAxhGJwvi2GYsi4aIwjKNggDMdocCKO49GgPoBA",
        categories = "layout",
        tags = "drawers,sidebar,secondary",
        contributors = "danielbayley"
    ))]
    PanelsRightBottom,
    #[cfg(feature = "panels_top_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh3GkZB0GiDoQhIch4g4MoSiEIoWD4PAvgOBYoHAYYbCAZIOE0MwgDkaIeigL4uhuLYvGiMYzDkIAyDEVg5CKOo8GgPoBA",
        categories = "layout,design,development,charts",
        tags = "menu bar,sidebar,primary,drawers,window,webpage,projects,overview",
        contributors = "colebemis,ericfennis"
    ))]
    PanelsTopLeft,
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMQwhAYx5hODw+DwL4DgWB4bHAYR0GgIBkgwTQ5CAMQ3FYNxoDQYQzCCMwwCCNgxjcIA2EgOQihsL4iiQPoBA",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh3GkZB0GiDoQhIeIVhIcogCIMgiD4PAvgOBYoHAYYbCAZIOE0OQgDENxWDcaA0GEMwgj4MAgkEMZCCANhIDmJ4pi6Gw+gE",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bGwaRuggeQygwOYQHiJ4NhUIB5DGEw1iqMYtCKGwviKJIhiOJY1DGMwgiuEw0hCJoojSRI3hyOhlD6AQ",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig0CIIB4g0NoQGgZRpGcaB0g0MYUCAeYNg8Pg8C+A4FiOJoGhaGIahyHogg6EIIgqDIxhGHIiiSKQ+gEA",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINAigkeIMDEMYPHKDAyCIPg8C+A4FgeGodgaCIWCKGIQhIOIPgqDIphqHIEiKIIwgeCYLCIMQ2hWF4qhGJQwhmG4hh8PBwGEdBoCAZIME0OQgDEMBhDUIJSDAIJVDGU5THYMwuDUYZclKYJWmMMQtDYLg4DSTguDANA1FGZw1DKTg3C4NA4CANJ2DaTpnmkQZimKVZXlOXZODAWpAC+RZHD6AQ",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDQIggGgZRpGcaB0g0MgwhEeINDWERyh0IgyhEeYZCIPg8C+A4FikcBhgsIBkg0TQxDUIIPGgLgwDGKIqi+C4ujAaIyjQOQgDYaA2j4L5AGiQoxjMIhNkcMQwkqTJOD6AQ",
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
    #[cfg(feature = "pen_tool")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDIIAxDkIA3C0NwgDOFYThELQzhoegiD4PAvgOBYfiKBoIgoMQ4g6GwxC4NYTi4TINDIbAzi6Dg0jEMYWikbIvDWHYfiGBBoiSRIHgkbYzhGLg4DaTA1k6HogiWHxjGkchjGwZQgGMeIJDEMQiCAcoJDKYxjHmYJikKV5ZlsPoBA",
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
        contributors = "danielbayley"
    ))]
    Pentagon,
    #[cfg(feature = "percent_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIMG0MQ1CAOQtDYIA2hmG4hiOIIiiSJgiE0OYrGgLgwDGMQvjMaI1iOJYME2KQgimPY/kGQw+gEA",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKG0MQ1CAOQtDYIA2CKKAvi6Eoti8aIxgoTQ5jcaAuDAMY9imQBokKMIyCITY1CCNZKkyTo/kMPoB",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig1CIIB4gkIgxDmDR4DKCoMgeFoRhMPg8C+A4Fh0YxpHIYxsgYcoKDILoYGMeIKDaLINGMeYwjKHQviOJYniKJImgaNIKDEN4yCCKQiiuLYvhGRIMjiOo/D6AQ",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwMYQGMeYMDUIg+DwL4DgWB4bHAYR0GgIBkgwbQ5CAMgwCAMwtDaLggDaGociKJIhiOJYnCIbYxDiM4rjMLYPhsL43GiOYkiaDBNg4IAxDAdg0jWR46D6AQA",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDYIAyCCEggDiFIVDgIg+DwL4DgWB4Jh6I4KHkMoQhwIB4DGEAyDIIosimEQ2jIeYuCKMYeC+JoeHAYR0GgIBkhATYwhYNguDkMh2DMYYVhUMAglMMQtDILgxhmFQxDkLg3DmFpemCVJllYOAuDYMwtDMLgwDeYguDWcZzlOVQtDaeJxmSXZfmGdoWmybg3C2aA2DcQZRmWVQgDSWQxhQaJPoqgKQlyX5clgOA0hamqcoCU5fhQLg4paFKLoELg0nOWAxDETJoDCYZeDkMRhheF6gCCEw2GwMQuDKhK/sGUKnpWo6uC2qpzDGnqdqSn6opqv6IpSZoakmSwyHqHYfkCQg+gEA",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0Ig4CIIB4gkIgxDaDR4DGCgyDKDR5hYIoZD4PAvgOBYfiKBoVgqEoahCDIHhyGYOhCGAih+IYEGWHxwGEdBoCAZIKE2GAghILg5DIdgzGEMggkoMAgk0MQtDILgxDiS5CDkLg3DmV5ZluTZPC0OAuDYMwtDMLgwDeXA1muTpulANgtDaXJanSXpvmaaA3mGYw3EGSpMm8IA0lMMZLGiSKAm6T5WDGWZKDGUg4DSQqSpSX5ulmSwuDihqKpiUAuDSbJSDEMRMmIMJblgOQxGGEpCnOmJNnMNhso4Mp7rgN5JlaoKbqYLaimykacpSxaToubqSo6fqfoKQZDkUeoziCOY7D6AQ",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DgIAxDMLgzDEYQxDaEYZDAIIchyEw0DEIAyg4bAxC4Mg3C2J4pGEMojh2MYiiQMQxC0Lg0DWEYkDgNI7C6PYxh6I5AicN4wi+Q4ikaLwyHYM4ujCSgtjSEIvDEOQuDcOYRlmW5ChELQ4g4MwthMMJHliOJXlkNJJjKZoUmWJA2DcbZUg6KoTDMNIXl6XJql+U5niqYw2DMQZNmCIg0C6NYjGiUKKkqMJMj+QQxjyPpDhyWpEDiM5SnCOI6jQMRMmMMJclkOQxCIPg8C+A4FrAbBpG4ZQgHgMoJDIIq6DGva+CAebBCKwx5ryx6+rAL62rgPoBA",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgyDIIA4CCEoWCAMQ2hYIg+DwL4DgWB4Jh6I4KHkMoQDIIggHgMYQhqLB4imEYrCAeYvCIOIdh+JoeHAYR0GgIBkhATYXhoLg5DIdgzGGE4TDAIJSDELQyC4MYVhMMQ5C4Nw5hmXZflOZJVDgLg2DMLQzC4MA3mELg1nCcpSlQLQ2necJjlyXpgnWGZrm0NwtmcNg3EGUJklQIA0lgMYWGiTqJn+j5bl6W5XDgNIZpmm5/lKXoWC4OKVhilAtC4NJylcMQxEyZwwmCXQ5DEYYahmG6fCCGw2GwMQuDKg6/sGT6mmWoqtqiqqcqOmwxp2ipkpmv6HpOx5IDaSgyHqPIgkEaA+gEA",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEhyHiDgyhKGQihYIB3GkZB0GiDoQCIPg8C+A4FikcBhiQIBkg4TQ3CANxoDEMIoiqL4ki6MBojKNI6jcdo6jwL4+GiQIxjMIhNDENggDENxhDIIJYDAIJbDELQyl8Vg3kmSw+gE",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDAIoHDGCg0g4eIQCIOYTgmFgiD4PAvgOBYcHAYR0GgIBkgoTQ0CANxjDALQxC4NwgjAMwtDMII1DMaAxDOG4diKJIhiOJYnCITQxDgIIMGOL4xCCLo1jSNo2FaEocC+QBoD6AQ",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "piano")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgLg1CAOBjC0MQuDQIAwC0MguDYLQuDgLQzC4MoZEENoeDcIImDiKAwheLgyCAOR2DEMRhjCMItjkII3GgMQ2jaO4ujqIwyHYLYNDUQwyjAOYbjsMIVhAIIMg6EAiD4PAvgOBZYluBoIgqMAxDQaAyDCV5Zl6XYEl+CRNDaUw0HYNJolqbJrgWB5uDGLZjnOdZqgKbJ6gqY5xn+WJ2lygp5mCCw4oedKJmqAQA",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,chord,keys,octave,acoustic,instrument,play,pianist,performance,concert",
        contributors = "danielbayley"
    ))]
    Piano,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA5FYNhhDIIITDAIIWDALQyhoSA0hKFIXiGGYTDIdgxDAY4WDELorDmIIkGgNAiD4PAvgOBY0HIZRjHQIBoGUaRnGgdIJDcIggHiCQxDKRx3GkZIFkoMJHHmSgzkccpJCKTI0C+Oo8D6AQ",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CANAuDUdg1EgMxtC0MQtDYIIZDYbQxDMIAwHYLQzGMMIWC4MQ2C0Lg4DQLQyi+LxoC0N4VDkIA5HYMolCAMQuDANQuDkNQgDKRZFGgMwiD4PAvgOBZMHIZRjHQIB3GkZIFgkMQwCIIBoGUaRnGgdIJDeXh5lsM4Pl4cppCIMpeHiW5xkwL5SlQPoBA",
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIeYdhIcocCIMgiD4PAvgOBYoHAYYXCAZIOE0MQyCCNRIDkLg1GEMo7CCPg1CAMJDjeQwtDUSAxDeJ4pi6F4ti8aIxjONQgDcdgxDCTQvk8aJRjCMgijQNpXlmW4ol2Ug+gEA",
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
        svg = "gAPBsGkbhlCAeAyD0IgyDIIggHmCYLg2BwxgqEx5hWCwiD4PAvgOBYch+Bh4hkMYTgiComg6GIpDeK4RgyG4diKHBwGEdBoCAZIKE0OQgDkdgxC4Nw2GEMggkcMAgkoMQtkIMQxCCQg3DkbJOkMOAuDkQZHkmS5fDWUg1C4Mg0FYMQ3GiKocC+No4jWN45jsIhNDGYQ5C4M5mDaapGkiX5KoELQ0EgNwuDgOYym2cQ+gEA",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgxDIIoHDGCoMg4eYQgsN4TgkIgyg0Pg8C+A4Fh0cBhHQaAgGSChNDUIAxDcaAxDQdgtDELg3DYYQyCCOQwCCPAwjMLgxDGQA3DkbJEDgLQuDkQY5juPYsiyKwxDCNQ2FYNovjiOpQj6PQtDQSA4luT5ejwNIvHYNJWmSXYskCQosjWRpIkuTZcmYIJTDULgyDQWgih0L4jiUPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwG8bB5GcbxuCCIRpG4dBzhcIA4CAMQ2jEMoxDCMYzDGNw4hmG4hiOJRuD6AQ",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[cfg(feature = "play_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwch4gkMoMhiCoMGgZRpGcaB0hOFQ+DwL4DgWJxwGGEQgGSCRtDkIA4CANggDQLY4DQWgiicL4thEPoBA",
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
        tags = "electricity,energy,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[cfg(feature = "plug_zap_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMIAyC0MoOC4NRoDMTAxhENwiD4PAvgOBYch+BoICITQxDAIAxDQdgtDOG4diKIYEiOCYmDSKYri2L4ejOMoFgeNQxDGKQ5GMLQxC4NwgDCLZHC4M5NDOLAyGgOB2DIY4okgN5Og2UJeFqO4xgKM5AiWGIODKLIuhyPIggE",
        categories = "devices",
        tags = "electricity,energy,electronics,charge,charging,battery,connect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlugZap2,
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
        svg = "gAPBwGEdBoCAZA9CITQxDIIAyDIdgtDUIg+DwL4DgWFYYgaCIKDkIA4FYMoUhaG4agSHIJgsNYgiKJIXiiJ4FgeKgxDiIB2DUYQ0CCPAwCCPwxC2PA0GiQ47j2QJKkINJDFYOBai+JoB",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet",
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
        svg = "gAPByGUYx0CAeA9CIMwiCAdxpGQdBogkMQ4gwcoICIMoMGgZRpGcaB0hOFQgHmCYLD4PAvgOBYoHAYYRCAZIJE0OAgDEMhohWKAvi6EYti8aIxjONwgDgdo6imPRoD6AQA",
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
        contributors = "danielbayley"
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQxhAcoTCIPg8C+A4FgeGhwGEdBoCAZIME2FAgDENxhDGKggDCMIvDKMBjjELg1C0LgzDQIAzjoNQgDSOBhjiOIyjEMQti4MBjjoMQ2kuOJAC0NJUkMNRahmG4hiOIIiiSJgiE0OIqDQYZBkGSYvmUMJbC+XRol+I4lieK4qDiRA5CCe5rDCS4xm6GpwmAPoBA",
        categories = "multimedia,social",
        tags = "mic,music",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere"
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
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[cfg(feature = "power_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwTYOCCDhWDYIobC+IokiGI4licIhNDgIA3C4NRBDYLgxCCQJCDAIJGkaKgxDgYQ2kOR5QkYNAthSPYvhyMhoD6AQ",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDiDRoGUaRnGgdIUhYIIJCIM4NHmCojD4PAvgOBYoHAYYSCAZIKE0MQyCANx2DUIooC+LoSi2LxojGMw4CAORhDULgxDQIJJksIAwlCUpMkUNAuDkNQglaWJSlGUQ0C2Fo8j4aA+gE",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig2CAOQghIMggDEOAgheGYTCIPg8C+A4FgeCYgHAYR0GgIBkhATYShkSA0GGF4XDAII3DELQyjsdgtDWNIbjiQwxhuOxoDENpBjaRJCDIdpAjWQ45juGxojuH4hiiKogHIZRjHQIB5hAMQ0CIIB3GkZIqmQMpnGgZRpGcaB0hAOJnHiEA2lkL5emAPoBA",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CANwgDMIA1CIPg8C+A4FhSF4GgiCg5CANhWDOE4VhqGYEhuCRtDGEIODILQyiOFonhQYxpHIYxsGUIBygmIggGMeIJDkIo/HmCYrjGNY3jmJoFgeCRNDEMQuDiEAxDISAyDAYQyCCXQwCCYAxl6Xh2DSXJkmCYovl4SJnl2X5hCAMYvi8dgtm+aZymOLgyGgMguDEN4xiWAonk+CgxDacw2n+hIzgEA",
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
        contributors = "danielbayley"
    ))]
    Pyramid,
    #[cfg(feature = "qr_code")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig1CIIB4g0M4QGgZRpGcaB0g2DwgHmE4QHKEgiDEIg+DwL4DgWJ4qgaFoYhqHIhiOJYeiCEYNDENoQgiCoMg6Jooi2LIEi6F4ZhuQI4CKFI2iSO4HgmC4yCCIo5kGKZFiccBhgsIBkg0TQyDEII6GgLQzGEMggmsMAgm4MAtmsMh2hSJwvlyC5bl0aJfmGY5sDEdguDCJZ3nkaJ7l6YAiE0MZrDedZqmyb6VDGcpsEgN5Yoiip9owTQzmUMhooShoop0PKIn6jaPCAM6loWnJ8p6rKOmuOqDrKh60qqfK2jqoxoqeeK9quoKAo+urEqmx5hq6Yx2C2zK0gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQxDMIAxDYYYNg0MAghQMQgDILgyDQIA1CIPg8C+A4FiCI4GgiCgxDiDgyGgLgwDGH4hiaJYEieCYLisMgxGgLQ4GGHIchSFgtDSRQgDeSIVkuFw3C0N4uDITA5C4NggDYLg0GGF4XhaDoYC4OAthkOBMDENZhkiURjDMLoTleYJJlYNh2DEYQyhiS5EniLQtnaEp6hULYSjKIo2jWBYHjgMoUDgLg1DQVpanyTKCkIdgzoWNICjaioKDeVQxnioqPhuEQgm+XZ+lUIA0m6mqHgEA",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = "danielbayley"
    ))]
    Rabbit,
    #[cfg(feature = "radar")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkLgwDcIA0C4OQzEEMQwCCFwghiHAgDaEw5CAMwuDMNAiD4PAvgOBYoiuBoIgoNIeGiDgxieKYui2BIvgkTQyC4Mohg0NgyhaGIakeGwgDIMYkDEIA4iQNY3iqO46gWB49DGHwyjINwuDcNhBDaHpKk+GJRDIM4Zh8Ng3lSOYCjuWYKDEMoZDiNAwjaKJViycpYjCC5fDmIQxk0NpimSZIdk8MQ1mCEJbC6bpwlYPBjGkchjGwZQgGMeYJnYIggHKCQyqQYx4qKqJ9pmm6dlePAiG0MYjDSjgwC4NYhpCiQtr4NqWiyAQA",
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
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ2HYMgiD4PAvgOBYThaBoIgoMQ5g2D4RhOFYEGiExyGUYx0CAaBlGkZxoHSCQ4CIIB5jKNB3GkZIFgkMgwjQch4j2NJCCKIYUieKYYiSB4JE0MQ4g0Mhoj+IoZD6AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQ0C4OQgDEOQuDEQwxg8NQuDKDwgDgLg4CCDIOiAIg+DwL4DgWJIngaCIKDeHYPDaGBjC0MguDONI2jiN4xhUMAthwNYjiWKokGMaRyGMbBlCAcoJDIIggGMeIJDGT5RHmVJPiQL5GkiSopgSK4JE0MYxhmLg4GONQzCCa5tjYII8CAMIbC6QZbkSAphgeY4QhKH4NEMMpshyHqDhaf5+hWEJCiaYQ+gEA",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Radio,
    #[cfg(feature = "radius")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDALgzDQIAxDcLg1DIYQxDCEYZhkMQgDALQyC4OAyCCIYjCIPg8C+A4FikYxpHIYxsGUIBjHmCQxDkIo1HiOI6CAcoJDKKIqi+MYzimLIGggIhtDEMwuDSHZPlGHQ0C4MQ4CCV5ZkSK4EGiLowjKNI2jiQ48meO5BCKQ4pC+RpkD6AQ",
        categories = "shapes,maths,design,tools",
        tags = "shape,circle,geometry,trigonometry,radii,calculate,measure,size",
        contributors = "danielbayley"
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDIIggHiDQ2hEeYNhAIByhMIoZGgZRpGcaB0hgMAiD4PAvgOBYoiuBochmCIKgyHYmCCFwihWGowhGH4hiODYPieKYuD6AQ",
        categories = "connectivity,devices,design,photography",
        tags = "screens,rotate,rotation,aspect ratio,proportions,16:9,widescreen,4:3,responsive,mobile,desktop,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[cfg(feature = "receipt_cent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCgxiENguDUdgxDGNI2juOoEjyCRNDENQgDkLg0GGDIMDCG5YlgNQuDKR43ksPoBA",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    ReceiptCent,
    #[cfg(feature = "receipt_euro")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCg4hcMhoDWNI2juOoEjyCRNDENggDkLg1GGDIMDCG5VlUNQuDKRI3kgPoB",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,€",
        contributors = "danielbayley"
    ))]
    ReceiptEuro,
    #[cfg(feature = "receipt_indian_rupee")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCg4CANxoDiNI2juOoEjyCRNDGJw3C4NQgj8MQ1GgMRhgyDAwCCWpcC2Q41jeSJHgWB5KlIMZCkSYY5gEA",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,inr,₹",
        contributors = "danielbayley"
    ))]
    ReceiptIndianRupee,
    #[cfg(feature = "receipt_japanese_yen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGggIhtDGJwwCAMwtDONI2juOoEjyCRtDkIA3kOQx2DcLg1keN5LkqBYHgkTZPDEMRoDaWJJgKS5dgqYA1mOZZagE",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,jpy,¥",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    ReceiptJapaneseYen,
    #[cfg(feature = "receipt_pound_sterling")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCg4hcMxoDWNI2juOoEjyCRNDEMIXDcVg5C4NRhDKUoNlaTZNhUNQgDCRI3kiR4FgeSo/DENxoDeX5GgEA",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,british,gbp,£",
        contributors = "danielbayley"
    ))]
    ReceiptPoundSterling,
    #[cfg(feature = "receipt_russian_ruble")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCg4hcNRoDWNI2juOoEjyCRNj8MQxkIYYhiEMIblMMAtDQaAtDMdgxDCRI3kgPoBA",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,rub,₽",
        contributors = "danielbayley"
    ))]
    ReceiptRussianRuble,
    #[cfg(feature = "receipt_swiss_franc")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCgxDCFw3FYNxoDWNI2juOoEjyCRNj+FwxGgNJHjeS5KgWB5NDiFw1kWVJJgEA",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,chf,₣",
        contributors = "danielbayley"
    ))]
    ReceiptSwissFranc,
    #[cfg(feature = "receipt_text")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCgxgwOBIDiNI2juOoEjyCRNDENoXDKQpEjeSJHgWB5KDEM4XDaUI1lKOYB",
        categories = "money,travel",
        tags = "bill,voucher,slip,check,counterfoil,details,small print,terms,conditions,contract",
        contributors = "danielbayley"
    ))]
    ReceiptText,
    #[cfg(feature = "receipt")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMh2DIMBsDILQxg0IIVhOGIXg2FIWhiHQyhcVgyGwLYhDGJoUiaF4piiJ4tisMRaCIPg8C+A4FjWOIGgiCgxDYIA4GgLQ2GGIYhDCG5JkkNBoDSRoWkmFYVkwSA4jSNo7jqBI8gkTQxicNwuDUdoUDGWI3lwPoB",
        categories = "money,currency,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,dollar,usd,$",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[cfg(feature = "rectangle_horizontal")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIB5goNoNGgZRpGcaB0goMYMCAdxpGQdBogoMgwg2CYLCIPg8C+A4FD6AQ",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDIMINgkIg2g0dxpGQdBogoMYMCAeYVCIPg8C+A4FD6AQ",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ3CKCR4gwMQyhAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MgxCANx2DYaAtDaGYbiGI4giKJImCITQzCCDhhDkIJADAIJDioOQtkCSZEkuKg2CAMguDMbI8lCD4aC+NBoD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQzCAMh2DYaA2CIPg8C+A4FhSF4GgiCgyDEIAxDIQQ5CCJAwCCJ4nDYIA1C4MxMgwOIThWGoZgSG4JE2HoNg4LYQj6M4WjeNoFgeOYMiEYYkiaKJNDENQgDYLg3GwMwtDKU5BjUPBjGkchjGwZQgGMeIJiEIggHKZpoGMeZmDKQZdl+YQ+gEA",
        categories = "arrows,development,shapes",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = "danielbayley"
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
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0GMMAtDELoTDkLQyCAMoXCIPg8C+A4Fh2IIGgiCgyDCGRjhMMQgiiGAuDmGYZhyHojiKBIkgkTQyhgOIQCCKwtjCGYXjOHYfjiN4FgeOgxDaQAwGOEoUi2F5ChaGgyjSSIhgKOJMCIbQzCANwgmOZwtDOW42l6S4lE2TwxDAVg1hCU5lhMM5pmaaRoDGa5JDwchlGMdAgHIeIJloIKJCKixoGUaRnGgdIJDgIggHmCYMpgdxpGSBaWlug6FkqOYKgyQIPiuLYykOGAyHaD4okGr5FlqR5siOYI7rSq5Ui4IKvhmso/rWMZZkaNZJgEA",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQIA0GMMAtDELoTDkLQyCAMoXCIPg8C+A4Fh2IIGgiCgyDCGRjhMMQgiiGAuDmGYZhyHojiKBIkgkTQyhgOIQCCKwtjCGYXjOHYfjiN4FgeOgxDaQAwGOEoUi2F5ChaGgyjSSIhgKOJMCIbQzCANwgmOZwtDOW42l6S4lE2TwxDAVg1hCU5lhMM5pmaaRoDGa5JDwchlGMdAgHmCYMCIIBoGUaRnGgdIJDiixyHiCZaCAdxpGSBaToulwilqR6DoUPoB",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgzC4NQgDIIA2hMIAxDKGQxDiGocDUIg+DwL4DgWB4JiSKYKHkMYQhuEwiCAeQyhAMgwjMeIvCIOYyCAeI2CINYyiQL4siuCIKjuMIikCQoej+NY3jmNI8jGIpGkiAhhHQaAgGSEBNhwN4Uh4YQxDOGpqDAIJtm2UYUjmRhwl0aA+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ5CKCR4gwNoQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0OQgg4aA4C4NRjC4NAgDAIAuDkLQuDGKguDOOAyhmG4hiOIIiiSJgiE0NQuDIIJKDIQQzi4IJRDUM4zleNA2lIMQyEiXJAC+QhokSI4lgwbZMmmNAyDCYJimSRonDKOgxkoMxhlSU5SjSfAtlGPZ/m6RZwmaSJ1k0aAtDSPKCkOAoEgaCBjhKDQ4hCCoMDWFYXmCHaRD6AQA",
        categories = "maps,navigation",
        tags = "path,journey,planner,points,stops,stations,reset,clear,cancelled,closed,blocked",
        contributors = "danielbayley"
    ))]
    RouteOff,
    #[cfg(feature = "route")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CINgigkeYMDEOYPHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0OQghMaA4C4NRhDOLggjENQgDCNo4DALQ3GgLQxDGMIyjSOYqjaOxIDENYZhuIYjhqHYGgiFgihiEIMkqCYLCIMQ4kuHIElEPoBA",
        categories = "maps,navigation",
        tags = "path,journey,planner,points,stops,stations",
        contributors = "danielbayley"
    ))]
    Route,
    #[cfg(feature = "router")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMQ0CIIB4gkMoNHcaRkHQaIQDCDRyg8IoRCAaBlGkZxoHSCQ4CIPg8C+A4FikcBhhYIBkgkTQ2C4MAxCAMQ4EgNooiqL4Wi6MBojKNAxDCN45jsSJIj8L5BGiQ4xjMIhNDENY6DAdoMimUJElORZVlcNwuDgNAgmUMQ3GGaJoDAIJwDALQ1C4Ng2nGT5RmGRpWDKSZ3CANAuDMNBhDgIKInKcQtDEMaEjmGZenuAQA",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows_2")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh4hWEh3GkZB0GiDoQhIcoYCIMgiD4PAvgOBYoHAYYeCAZIOE0MwgDEMhoiGKAvi6Hg+gE",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawer",
        contributors = "danielbayley"
    ))]
    Rows2,
    #[cfg(feature = "rows_3")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHmDgzhIch4g4MoSiEIoeD4PAvgOBYoHAYYXCAZIOE0MgxCAORIieKYuheLYvGiMYzjUIAxDWOQiigL48GgPoBA",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers",
        contributors = "danielbayley"
    ))]
    Rows3,
    #[cfg(feature = "rows_4")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAdxpGQdBog6EISHiDgzhIcocCIMoSHmHQiD4PAvgOBYoHAYYXCAZIOE0MgxCANwuDUSIeigL4uheLYvGiMYzjUIAxDKO4nimPxokGMIyCKNI2DENo5kqPZND6AQ",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers,grill",
        contributors = "danielbayley"
    ))]
    Rows4,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQxGEOQghEMAghQMYShIIg+DwL4DgWG4egaCIKgwNBhDENoNimFIWiqKoahyIYbGMaRyGMbBlCAYx5gkMQ5CIIByj2QBjHiCQ1jAL40jaOA+gE",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ5CKCR5gyDoQHKDAyCIPg8C+A4FgeGodgaCIKgwNYQGOEgiiYIIWCKGIahyBIihocBhHQaAgGSDBNDUIA3HYMQyGiQYZhuNY3jSNo4joIhtj2DggDYLQ2kUL5HGgPoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgHKCQxCIIBjHiaIdmseZuksL5gmKZJOiaPg1CAMQyHMMguDULZ7Deg4NCChKIhqgaKiqgqFo+g5zk2AQA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCANxWDUYQyCCEgwCCFQxhMLQyGgMgiD4PAvgOBYfiKBoIgoMQ3CAM4chGE4WjCGISDIdodh+IYEGiJI5geCRNDKGIpjWLoUjGGoTGiGoeiCJY7gWPYKiqQBIhCM4wheGoaHaSo3k0PBjGkchjGwZQgGMeYJDGHQgHKCQzCKZh4mmNogmCYpkk6JoJG0MQ2CCfQtDELg5oGg5LjiI4BA",
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
        svg = "gAPBjGkchjGwZQgHIPQiC4NQiCAYx5gsN4Ng8Yx4hOFQgGYaRsGyCxjHUchyGUbh0EMbxsG8cgiD4PAvgOBYHi6MYGgiCoMg6EISCINYahyHogiKJImiiKoshCGAiDEOIVi6MIEjaNJRgeG4dh8IohiOJYnimK4WjwMQxhqF4LmKGo4k6L41jOApUgiQJYlqRJdkeFpKhSOppjqEZmDaapQjIZZToKSZmnmYJmDSP5XkKW5Fl6SJ7i2a5vi4cBhHQaAgGSCxNDMIAzHaTBokylAvpimg+gE",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[cfg(feature = "school_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQwhAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MgyCAMgwFYOBoC0NBsC0NoxjQIA0EgMh2g4YYqioMAgkGQY/GgMQ2j6K5CkuRAtDIWoZhuIYjiCIokiYIhNDYIAxDcdguDAMZRC+UxolWI4lieWwxDOX5hmOZZnleJwxDiXJemCYoamSVpymmWZ1lybZ5nCfQ8mWfxNDENIrjsLQ1kmQJMk6Th2DCkZMkKTorHYNaFlSAQA",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ0CANggDgLYMDgIA0CIPg8C+A4FheGoGgiCgxhMMQwhQIAyHYOBhDKJggiSJAxC2KwyEgNIqiyLggjAMoxHYLQ4GwNIxhaGIdhyBIegkTQxgwMonhGNorjgMIxjwMJQi2WJTjIdoVheGZHkaBYHkmIQgDUdgxDeQ5fhuApHmMIhNg6Z5pmuRQ8GMaRyGMbBlCAYx4gkMQyCIIBygmhJ/HmCQ5mueZ7n0PoBA",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1C4NAyCAOYNg8OAgDEMgiD4PAvgOBYZGMaRyGMbBlCAYx4gkNAiiUeYJDiKhygmF4ZC+H4hiOGYcgaCAiG0MQ0CANgtDgLg1hSQ5FhiGo5h6IIiiSMAiheK4JDENoqiaKJJjSTY3gKBI6gkTQxDALoUj6ZYVj8MYujOS5egWB5hlWFQyGgLYykqX44l+cYKDKD4WnaeIbnqAQ",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors_square_dashed_bottom")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMgyGEMoNCAMITCAMQtDKGBWDSEIShSFAxg2GBoDENodhGH4WhIMh2iWJ4ViCGINCIPg8C+A4FjWOIGgiCgxhSDhIDiNI2juOoEjyCRNiWDQyGiGJEjeSI1GMaRyGMbBlCAYx5gmQwgHKCQyCKWx4l6UZVleWZHgWB5KDkLg0DGDJwnKDAxhGeJRkaApIm6Pg0C4OIWoGgwxoaQ41lKOQ8mmWJalyCYlmQY5mCKX5hCKY6Ko6a59m2PRtocIA2C0OAuDUOA2CCp6pDae5TgEA",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[cfg(feature = "scissors_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDIMAiCAeYODKEh4hWEhyhiD4SHcaRkHQaIVhEPg8C+A4FiYYxpHIYxsGUIByhkIBjhwOISGOFAijiJgviyLowiYcBhiIIBkg4TQ5C4NAxDQIJLk2TwxDIIJUCKPpEiKQ5FGiR5Jk4Lg4lYNJilaYwxj2J5aGiK4ti+MY2g6OIyjSOoODENpYieQJwlyRpICIbZpCANgtDgLg1DgNggoiip6lmXQ+gE",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINgiCAcoMDODxjHiDIOD4PAvgOBYHhkcBhHQaAgGSDBNDgLgxDIIIoioIIuioIoZC+IIih+IYjiUIhNDIMAgDSLIpisMQ1C4OA4jKGo1GiGYcgaCIRCKE4JhaDYUgsIgxkiM5Oh4PJLiSJgxDSRovmQOAgj2aQwkmNI4D6AQA",
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
        contributors = "danielbayley"
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ5CAOQtDIIIQhIIg+DwL4DgWFYYgaCIKDEMwgh+EYPiQMoUhaG4VGMaRyGMbBlCAYx4gkMQxCKMR5jSNggHKCQ4icL4ri2L4agSHIJG0MgxhEMQtDQLgzk6UJAimAQ",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = "danielbayley"
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FhcYxpHIYxsGUIBjHmCQxDEIojHiJooCAcoJDiFoYh2H4hheGoGgiCgyDEII8C0NAuDOQJCjKGYEGgPoBA",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/",
        contributors = "danielbayley"
    ))]
    SearchSlash,
    #[cfg(feature = "search_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDMLg1CAOIOC2Dw1CIPg8C+A4FheGoGgiCoRg+IQghSFoYh2FxjGkchjGwZQgHKCQ4CIIBjHiCQxDGNBjHmOI6hcL4qiyLocgSHoJG0MgxCCSgtDQLgzk6UImhmRg+gE",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort",
        contributors = "danielbayley"
    ))]
    SearchX,
    #[cfg(feature = "search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCKCR5gyDoQHKDA4CIPg8C+A4FgeGhwGEdBoCAZIMG0MgxCCKQtDQLgzi2L4ZhuIYjD6AQ",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINg8IByHiDQyhCHAiDENIQHmDYiCIPg8C+A4FimLIGiCHggiUIoygiCoMg6EIbh2EIShSFoYiiKovikcBhgsIBkg0TQ3CCIh2DEYQyCCVAwCCV5XlQMhoDGQwvkeC5GkgaJKkyIggDeXZTlWWJuDGbQylGX5hGgPoB",
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
        svg = "gAPBsGkbhlCAeQxD0IgxDIIggHiCQiDODh4DKCgyDGDh5haC4ND4PAvgOBYfHAbxsHmIoGiWBB0HOCg4CCMIMCANAgDENoxCKHwviWJ4piSJoogSKhviyLoLjiN42DIIAyDCMY2DaOogj2QojgE",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDEIggHiCQiDEMoOHgMYKhODh5hcIgzCIPg8C+A4FiAcBvGweYjgaJoEHQc4KDgIIxDQIITjKNQ2h+IYmiiKolieKYEisb4ti+Eg2jgIAyDCNQykkOI6C+PJBiSAQ",
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
        svg = "gAPByGUYx0CAch5D0IgyCIIB4gqDAgGgZRpGcaB0goOINHcaRkHQaIQDCDRyg+C4NgmJg+DwL4DgWKotgaJIQiOKIRigMQ0huHYfiGDYThWF4Zg2JYMiqLIEHSKhsGkbhlg4MYKDaJwylGJ5QCKUoOlSWAuDAMQikaS5NkqTJOHiV5ZHiWw2l2XwgHmVwxhqb5bnKYIrmIZQ+gE",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDAIA3GgLQ5CIPg8C+A4FhSF4GgiCgxDQIAxDcSA1hOFYahQYxpHIYxsGUIBygkMwiCAYx4gmIYzGMeY3DeJQvimK4tiiKosi6MAijKNI7CKPY0jaTI+kCRQ+gEA",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDILgyDIIAyGgLQuDQNBhhCEAwCCGwwC2GR2C4MQ4hiEYcicMQtDEIAxC4NwzGyFA0DODg1iWGooh+HIxiINYUDCJIZieHYfi6NIvjyDwuDOQYmh2J5GhGRhsj2Io3kOLJZi0NwyiENQxleT4piuWw0jwMQ1C4MA5mGQ4Ui+UpIg6DZMm2T4Ni+U49j+TY4huK4alSM41naWoskYVgyDChYbhkaIVheQpPo2Hx2hSI6FmSKp6jOFAyjakook6VJomqfZYo2RpvjCc4UDObKhkSqp4jCl4+qamYqpsNw0pYLqgk6opji6Zp7mqsLBk+s5Tp4Mquqek5Fi+q5nmmQK5oGMgzp6wJ+iyupbDMVqRsmHIfh8egiD4PAvgOBbrGMaRyGMbBlCAYx4gmDAivceb6DK/BygkM7quy8bzvUPoBA",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4C4MwgDEMBhC4N4TCAMIWg8LQuDYMg2C0MQuDANw5EwMYgDQIAzhKFA3hiFwxg8LgxDkOIaDANAziUNoNCCDA5iuFYXi+Gg1DcMoxDEWgiD4PAvgOBZMHIZRjHQIB5gkMQ0CIIBoGUaRnGgdIJDeWxyHiWJbHcaRkgWY5bmcIgzkuTZSlSTBjGkchjGwZQgGOcAxhQNZbGOVwioELqDCAcoJDOiZzC+eJ6nwPoBA",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = "danielbayley"
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeYLDWDhjHiCwxDgIg+DwL4DgWB4bh6BoIhCFwyg6CoMhSFgiDaGociKIICgSI4PhEIgxDmKILg2D4shiL4djSMhsGkboIHkMYXDMLg1DGDh4koIg4k2OggHgMoXDULg0icIB5lmOA3lyOobC+RZHhuaIIlGWpck+V5hlQNZWmCFwwmSDpJgsNpNk+ZprD6AQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQyHYOBhDIIITDAIIWhaEwyGiDoShSF4ghkLYPC0OAiD4PAvgOBYoHAbxsHkbBpG4ZQgi6Mx0HOCQxDYII9g6Hw4j6J4pi6MIyjSKJIjUeQxgkMgiCAeAyjuUJSk4IoOlEeZUlkNZEC+Sw+gE",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgg8Igzg0aBlGkZxoHSCgxDiDYJhSDR3GkZB0GiHYfD4PAvgOBYrGwaRuGUIB4DKEAxiAMYKhWEo3CIOYNHmO5ACKKwvjGM4wjKNB4kSPY2jiQo/DENZCkSVZGiySRlkuM4SkSQY+lKNZhiCP5BkeXJejSQ4KmKUQilmZYdlaY4LjmapMD6AQA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKDA4CIPg8C+A4FgeGhwGEdBoCAZIME2DggDIdg3C4NYZhuIYjiCIokiYIhtDEOQgDULQ1C4MgzjyQAzjAL4yGiNIjiWJwyDIIIOGgLYti+GpHjWSo2gyOY7jqPpEl+QZGkiWZMCKKJPDENIuFaTpjlgPJImaKAwkSUAzC4Nw3jyUA5m+M5xjWcw5i6UAyEiD5WmSgZLjedJ2DGdZBnyVYxnCHYGgiFgiDKLoQGOEoNg+CYLqKRqYh+AQ",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIOAigkeYMDIMYPHKDIUD4PAvgOBYHhmHIGgiFgihSCYLiQOYPGOEQihMIoZhuBIhhkcBhHQaAgGSDBNDILgwDUII9j8aAyGyPQ2DYIAxj0NAyGEMpBCAMJSlSUAxC4NQ4GgOQuDcOJPlGU5ikoLg5DULZXDUNxslcNpnDcLg0DMSA1C6S4vhqNY3D6AQA",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKEYUCIM4RGgZRpGcaB0g2D4RHmDYaD4PAvgOBYoHAYYLCAZINE0MQ2CAOAuDkVg3EgOBsDQIA1C2QA1GgOB2C0MY5CKKAvi6Cw+gEA",
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
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDkIAyDAIITDEMgghIIA0huE4VCIPg8C+A4FgcbojGyDBlCAeAyhANQiCAeYwCKMouDGMYzHmOoRDmIokisbhlD6AQ",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0Ig1CANAgDGEgxDIIISDIMIahMIg+DwL4DgWBxuiEbIMGUIB4DKEAxDkIggHkMYQDWMh4jUIowjIeYujuMYhC+KRuGUPoB",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIOQiCAcoMDGDxjHmEgyCIPg8C+A4FgeGodgaCIRCKE4JhaJYYgmC4lDWGYbiGHw8HAYR0GgIBkgwTQ4CAMgwHYMhoDgdgthiGgvjSNoakmN45CIbQxDILg1CAMQ3C2UwtDGWJUDEaAxHqL5IjUaJLmSOI6DENo9DAYQyj0IAwnGcwxlMNgtDMLgylSPI8nIMZxlqdZRnEQZvm+cqJCCPI+mKTA+gEA",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slack")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDgIggHmDgyhEeIODEM4RHcaRkHQaIOhoIByhcIgxC4NQiD4PAvgOBYrHAYYfCAZIOE0MQ5CAOIoFYMQwGiJw1EGQQgkQMJFCCR44jqKIqiyMYfiuLoGgiCoMg6EAgiWWYch6IAiiKE4mDSEYkhiTYri2BB0jCMhojSNg1kUNY8DENBIDOKJDiiRZ8kqSQgnIMZ0imaZQGiUprgeCYLg2YIWhiZAgl2H5YhGYoZmWJZBk6aovDyh5wCKN6EkUORInYdpBGGRJGkiSooC2nKGm6iYFouVqOiKlJflmYpZiWFYjpuaIslObYzjWo47nINaoDAVp5kKrZ+q+TLNp2hw+gE",
        categories = "account,social,communication,brands,development",
        tags = "logo",
        contributors = "colebemis,ashygee,wojtekmaj,mittalyashu,ericfennis"
    ))]
    Slack,
    #[cfg(feature = "slash_square")]
    #[strum(props(
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDEOAiCAeYODOEh4hWEhyhgIgyhIdxpGQdBog6EAiD4PAvgOBYoGwaRuGWEwxiUNYXjMIg5hcMo0hIeY7jiJ4pi6MA+gEA",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SlashSquare,
    #[cfg(feature = "slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIINg8MgiD4PAvgOBQ+gEA",
        categories = "development,maths",
        tags = "divide,division,or,/",
        contributors = "danielbayley"
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
        svg = "gAPBsGkbhlCAeQxD0Ig0CIIB4DKCgxgyB4QguDR4gkIgyDEIg+DwL4DgWHohgaD4KDODR5hWE4IgqE4YhEMIdh+JIjgSJYZhuF4VDEMopjyPoHhmPYziCN42gWQoRkGKpLheGQ4juJ5FjWAo3g6PA2k+Co6kqGoyhSXIyh6RoilaSYtl+UgiiiDpDkyFQymONJHmeOIRi+PIsjmP4KlqZJViSXgxmCMAilGYQihKa5RoCdaCmkMaIoYMZaliEaWk2Go+o6IoBA",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0IgyDEIoHDKCgxDSDh4hAIoTCAeIJhcIg+DwL4DgWHohgaFYKhgeYWDOFIbiiGwxDCHYfiSI4EiWLwyhSFgxjmB4bgyDophGOYeiCNo1gWGY7j2QgiiuGY4kGGw4jKRoigKNpQgoMoxg+EQ2jqW5dgiW4NkWNJYkmTZPhqYphguY5RmeR5pgaTYSlKEYYiYIpglqC5VmiJJKnqLIRnGCpUl4IpUnOV6DnwMo9m0IgxoqZKVn6d5go4ZQ+gE",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0IgyCIIIJguDRyHiCoMCAdxpGQdBogoMQ0g0aBlGkZxoHSFAwg2EwiDUIg+DwL4DgWLRwGGGggGSChNDEMguDYNg3CAOAgDEMJCDIaA0GwLY7j2P4ei0L4zhoPoB",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINgiCAaBlGkZxoHSCQxDKDB4gmFggHcaRkHQaIJDeDByhgIgxCIPg8C+A4FikcBhh8IBkgkTQxDMIA4C4MwyGENwuDSN4+kAIAwkQIAxkaPgzguKQvi+H4ujAaIyjQMQ2j8NgglcMgxGEMQxC4N5Zl+YZZkWRZImiYA1DiKIqk8aJRjGMwijUOQuDmSA0C6XQxDWeJIn6gJGmgIAuDCgZ/m2TZwD6AQ",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCKBx5gqDAgHcaRkHQaIKDENINGgZRpGcaB0hAMINgkIg1g2D4LCIPg8C+A4FiwcBhhYIBkgoTQxDIIAxDgaAuDAMYri2MoWD6AQ",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDIIAxDEdgxGEMQwg6FYVDGDgtDkLYUCIPg8C+A4FiCI4GgiCg4g4NBzDELg1CCDQ0jEIA0C2Mo3h+IYmiAbBpG4ZQgHkMoJDkIggHiRAiDkLgwDGRx4DGRZHHmUpLjoL4+kCPY/kGUYJDENZQkqYZNk+QpWkaQpKkaIJZl2JYEieCRNDENggDUaA2liPICnKB50DEOYxHae5ujyAQA",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhwGEdBoCAZIME0OAgDENBzDELg1CAMggDSMYzC2Mg0jeGYbiGI4aGwaRuggeQxgwOYQHkMpGhAeJKCIOQuDAMZMkWT47C+QJCj+QYIk2Ew1lGUwgkmSwgHiVQxDWSJVkeGpYlwPoBA",
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
        svg = "gAPBsGkbhlCAeAxD0IgyCKBwygoMoMCAeYJCIMYSHmD4WgwPg8C+A4Fh2IIGhmEISHiGoXg2CIKiqE4VhyHojh0cBhHQaAgGSChtDIMAgDENgtDSQggkMNAih0L41jeNI2jiOgiG0NAgDiRZFkSR5JksaJNjeOY7kCV5TkaQpIh6W5dk+O5Vj2YpWlmZ5OD6AQ",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIB3GkZB0GiCgxDaDR5gqDAgGgZRpGcaB0hkMINgkIg0CIPg8C+A4FikcBhhIIBkgoTQxDIIA2GgLgwDGKIqi+EopGMaRyGMbBlCAY4ljaDRjhgIgxieB4KieKQvkORZHi6MBojKNI2CCUY6jyPgvkAaA+gEA",
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
    #[cfg(feature = "spline")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQ5CIIBygwMoQGMeYMDUIg+DwL4DgWB4bh6BoIgqGIQhIIoUgmF4Ng+G4dgSI4bHAYR0GgIBkgwTQ1CAMQ3EEMQyj2QgwCCRQxj0NwghmL40jYPoBA",
        categories = "design",
        tags = "path,pen,tool,shape,curve,draw",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split_square_horizontal")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMQ5EgNRjC0MQgDALQyhOF4XFYNxjhaFAxhcIIYDIaAzCIPg8C+A4FimLIGgiCgxDYIA1iYY4UDCI4NjsMh2DEMIeg2E4jiKJQtieKYrgQaIpGwaRuGUIB4DKCQxDIIggHkMYJDSWR4lwIpXlkeZVCIMgwiiKpPlEPoBA",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[cfg(feature = "split_square_vertical")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAOBWDUYwwC0MQgDELQyCAMoXGgMQwGOFAwhmFYiDIdgzCIPg8C+A4FimLIGgiCgxDmFQ2iaEYVhOGYXhkSA3GOOoShqFoaiULYnimK4EGiKRsGkbhlCAeAygkMgwCIIB5lQIgxDKWB4DGCQ0lgeZhlyXpJk6UA+gEA",
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
        svg = "gAPBwGEdBoCAZA9CITQzCAMxoC4MAxCIPg8C+A4FhSF4GgiCg3CANYPhGE4VhqGYEhuCRNDEMQgDeIYShSFoniaBYHimDIuhCMIkjOAonjaHQgDmL4jjKGI+jWHILCCK5EjGJQ8HIZRjHQIB4gkMQ1CIIB3GkZIFgkNJbHmCZaCAaBlGkZxoHSYZFlKVI0igIhtDEOZCCAMp5HYMQwGMMAgC4NgtC4NJMC2LAxGgLQ2GOhA2CAMKIoQNKTDEVorGwMgtDKRZQhqQJ1gwMaGDinKej2oIcqKTJ3qanZPjOAQ",
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
    #[cfg(feature = "square_dashed_bottom_code")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDAIIMC0MgghGEwiD4PAvgOBYWhmBoIgoMQ0g6IQyhCJQyhWF4chuBIdgkTQ1hIMRhhMIINg0MYmFYNYzhKNY+DGEoQGiII8hGNoOj0Mh2kSNJHjiFIWhiLIrgWB4uDmMZDiiUoagKLJWCITYglkMZbiqAQ",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottomCode,
    #[cfg(feature = "square_dashed_bottom")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMgxGEMoNCAMITCAMQtDKGBWDWEIShSFAxg2GBoDENIdhGH4WhIMh2iWJ4ViCGINCIPg8C+A4FjWOIGgiCg5g0MYkjSNo7jqBI8gkTYlkCQo1jeRw+gEA",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottom,
    #[cfg(feature = "square_pen")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzEgNRhg2DQwCCFQwC2Ex2DENISCCFIWiGExohyHogheH4ZHYLQ3CIPg8C+A4Fi+MoGgiCgxDgLgzDcNYfC4NgyhEMgugwMY/kaIZHkcM4OEyDAgDENRsC0NJRlGVRai6MI1D6AQ",
        categories = "text",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    SquarePen,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CAMQwGMLQxC4MQgDALQyC0Lg5heFxWDQY4WhKEobDIIIYDIaIfiKFYmCCGotDIIg+DwL4DgWM42gaCIKg6DQ2hCK4WhiGocDIdgth+IYTkSJYnikY5Bi2L5NjKNI5jMchlGMdAgHcaRkgWCQ4CIIBoGUaRnGgdJimQeIJDENJkHKbgijEIB5m+cYzC+WZbD6AQ",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = "danielbayley"
    ))]
    SquareStack,
    #[cfg(feature = "square_user_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYQ2CCEgwCCFQwC0MQyhYIg+DwL4DgWHhjGkchjGwZQgHKCQ0CIIBjHiCYai4Yx5jIMYdh+JImiiHhyGUYx0CCMQiDOLo2kWLh3GkZIFjIOIuHKRAyi4aBlGkZxoHST45C+P5BD6AQ",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUserRound,
    #[cfg(feature = "square_user")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAaBlGkZxoHSCQxDiDByHiCQygwdxpGQdBohSFgghmCgiD4PAvgOBYoGMaRyGMbBlCAY4ICIMQwgwY4lDGGwgHKCYLigL4ujCMooHAYYgCAZIJE0NwgDIMR2C0MhhDKUQgDCWggDGUZVGgNpXlmW5bl6WAyHaG5DkmIA+gE",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUser,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDEOINgkIgzg0dxpGQdBohWFwgHmCobD4PAvgOBQ+gEA",
        categories = "shapes",
        tags = "rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
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
        svg = "gAPBwGEdBoCAZA9CITQ4C4Mw0CCDIOCAMggDkLgyDcbA1CANAuDgNxMDWHoUDIMQgDGFAxDcLg3DeJ4MDEOITDEbAtC4NQ5C0MwuDQMwiD4PAvgOBZAkOBoIgqMY8ikMosDaE4UhaGI1DYLg5DELQxEyKITjUMY8g+TZXj+QZGkAbBpG4ZQgHgMoJDIMgiCAeZuCKcJyHgMZvnIeZ6naZAvmiag+gEA",
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
        svg = "gAPBsGkbhlCAeQxD0IgyDAIggHgMoKDEOIOHmEQiDSDh4gkIoTCIPg8C+A4FiAcBvGweRnG8bggiaBB0HOEg0CyDAgjMMQyCAMYzhmIAviaKIqG4PoBA",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQyD0IgyDAIggHiCQiDaDh5DGCg0g4eIWhIIg+DwL4DgWHhwG8bB5GcbxuCCJIEHQc4KDEMAsDQIIMCwMQyCCMQsgyHYfiSJooG4PoBA",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0C4OAgDILgzEGEYRCAMAgDGFggDWDxIDQYQyg+GoXDALYhDIdg1GENggiyJIaiwNh2DCK4tiOMAtDYVofieN4lDKJhoC0MRhC6EIhheGYXhQMwiD4PAvgOBZPlKBoIgqDgxDUdpEjGPo2jKNJei+Fw2jkdgtDSTpQlWTxjGkchjGwZQgHKCQyCIIBjHmCQxDCeRjHid5/k8L5vnGcw+gE",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[cfg(feature = "sticker")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDULg1CAMxIDUYQyCCFQwCCGAwC2FQyHYMQ0GOGAxC6JA5haKAyGiIIUiiGoZhaHBWDiDhMgyDoQFoIg+DwL4DgWPI/gaCIKiCEB2DSLYXjCGIdGgNI7j2QpBgSQ4JE0OAgDEMxoDCUY+lWVIFgeVwxDaWpcl6PJgkCApVmSRYjDYcwulkMYoDEY4kDOMAyC2d5+DGX5TgEA",
        categories = "communication,social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Sticker,
    #[cfg(feature = "sticky_note")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAzEgNRhDIIITDAIIWDALYTDIdgxDSEoUheIoWhsaIeiCFYjhSGhWDgWgiD4PAvgOBYxjSBoIgoMQ1g4dofhuKokhQaA0jCMo3D6AQ",
        categories = "communication,text,social",
        tags = "post-it,comment,annotation,reaction,memo,reminder,todo,task,idea,brainstorm,document,page,paper,sheet,stationary,office",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    StickyNote,
    #[cfg(feature = "stop_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCKCR4gyDoQHKEwwCIPg8C+A4FgeGhyGUYx0CAdxpGQdBogwNoQgsIg5hCEovhAaBlGkZxoHSK4ZhuIYjD6AQA",
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
        svg = "gAPByGUYx0CAch4D0IgyCIIBoGUaRnGgdIKDaDYJguDR5goNINHcaRkHQaIKDIMAiD4PAvgOBYoiuBoPhGE4Vh6IIiiSJgghiDAghsIgxh2B46ieKYuD6AQ",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig2CIIB4g0NIQHmDQyhAaBlGkZxoHSFwwhAcoSCKGA+DwL4DgWJ4qgaJAxhQIIjheFY0geCYLg2DwghqHIeiAIonimBB0D6AQ",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIA0EgORhDMIITDAIIWDALQyC4OITDQIg+DwL4DgWIYkgaCIKDENAgDEMhhiyLIYi2FwgDgSA2iCIoniEbBpG4ZQgHkMoJi4IggHiRAiDIMJHHkMZFDKRx4lAIofiEL4+kAPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCIIBygwNIQGMeYMg4Ig+DwL4DgWB4bHAYR0GgIBkgwTYOCAMh2g+GwviKJIhiOJYnCKKQyisMIthqHIxGiM4kiaDBtDQLg5DMIJGkgIAxC4NAxk2Twxj2MI0kGNZEDENwuDYNpNlyXpSlCY5Ui+P5YkON45g4aIuj6Vw8j+ahNDIMJNDKbpVmico0mobQ2C4Mw0mCXQ2C2TpkomZpwjKfZCjYbQxDkLgwDeSpHDOiJTmWe5XgE",
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
    #[cfg(feature = "swatch_book")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDcYQ0CCEgwCCFQxC0OIWFYNRhDIIIfhWF4gC0MhoDSHoghaK4Nh8MhaCIPg8C+A4FjKNYGgiCgxDYLg3g4MxIDEOYpiGLIqDIdooi6K4XiWIBIDeMYzjiN4EjmCRNCCP4PGgMAuDAMZTjSV5WgWB4JG2DAghoMguDOJZvh4LoSm6FJHDOdAwDSYJ7EwMQ4C4NpboKc51nSTYOCCYAyoOeQ0DMNBMDkLg5g6lA4mOVYB",
        categories = "design,home,furniture,photography",
        tags = "colors,colours,swatches,pantone,shades,tint,hue,saturation,brightness,theme,scheme,palette,samples,textile,carpet",
        contributors = "danielbayley"
    ))]
    SwatchBook,
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
        svg = "gAPBwGEdBoCAZA9CITQxDEIAxDkSA0GEMgghQMAghcMQtDKGxWDeE4VhiIoNhwMhoDUIg+DwL4DgWKotgaCIKDEMwgDUaIfhSFojiEMh2DEMIgjuGYbhUaAtiiKosgQaIqGMaRyGMbBlCAYx4gkMQyCKVR5liWggHKCQzimK5PlGU4vkyB4JG0MQ4hWHAzC2NZymOSowmmBZrCIbQ2iGdJzCCdorniAQ",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[cfg(feature = "sword")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQLg1CAMQ3hQIAzCANoah6HYbheGYShQIg+DwL4DgWB4JieLIKHkMYQDEOQiCAeQyjMM42HiMoRjsIB4jmEY1icL4vi6CIwj4MQ2jyQwyDCPJMk6N5QlKRpIDyL5BkyNZWjOX4xhAMgxk+ZJmlmSg+gEA",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDQLg1CAMQ3hQIAzCANoah6HYbheGYShQIg+DwL4DgWB4JieLIKHgMYQDEMwiCAeAyjMOY2HmMoRjsIB5jmEY1icL4vi6CIKj2Mw2jyQwyDCNoxk2U5QlKRpIDyL43j4MZAjiEAyDGPI+mOT46iaKJaiqBpKgsb4Ng+EYThUNojDiHpjnoMYchaGIVDmJZGm2WpckydJTj4NZWhCQJCjMOJqkeSpJgmXYQDeaAilGjQiDSZYzpqWaVlub6Il+m5npgIo1jeQ6MqSLYB",
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
        svg = "gAPBwGEdBoCAZA9CITQxDUIAzHYMQ4CIPg8C+A4FhQchlGMdAgHiCQzCIIB3GkZIFgmEYiHmIIiGgZRpGcaB0iiEggHKHwiDKE4VhqHIUheBoIgoMgxCAORIiGFIWgQaI/kyB4JE2RAggySI7kuGIBA",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDIIAzHYMQ4CIPg8C+A4FhQchlGMdAgHiCQzCIIByh8IgyiIeYgiIdxpGSBYJhGIhoGUaRnGgdIwhKFAvhqHIUheBoIgoMwgDkaIxjuQI/gSQYJE2RAxDWR46hWSoBA",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDAIggHmDQ4hEeINDOERyhcIgyhEaBlGkZxoHSDQxDQIg+DwL4DgWKhwGGCwgGSDRNDUIA0GEMggjsMAgj4MY8C0MhoDEMo6jyP5KkGOwyHYMQ2kiPZLkOPBokMLooioL4wguL4xGiM41DgIAxDgaAuDAMYpiuXRoD6AQA",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = "danielbayley"
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMgiCAch4gmC4NgiCoMHcaRkHQaIJDENoMGgZRpGcaB0hAMIMg8Ig0CIPg8C+A4FiwbBpG4ZYHDGGw4ieNwiDGER4DKGwyC4MAxgweZAjyOYsC+Mo0D6AQ",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDUIggGMeILDeDhjHmEgiD4PAvgOBYHhmHIGgiFYLDGE4JguDYPhEIolhiGogh4PBwGEdBoCAZILE0MQyCCJRoDEMIuC+M41hmRI2jgIhtDMLg0DaPQwC4NQ0CANwuDAOAtlaWJCkcPoB",
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
        contributors = "danielbayley"
    ))]
    Tally1,
    #[cfg(feature = "tally_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBoD6AQ",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally2,
    #[cfg(feature = "tally_3")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZD6AQ",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally3,
    #[cfg(feature = "tally_4")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviMPoB",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally4,
    #[cfg(feature = "tally_5")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ0CANB2DENgiD4PAvgOBYThaBoIgoOYNg+EYThWBBohiI4HgkTQxgyDoQhKFIZiWBYngoMYdiyIIviOMYaigMgyCANggj8MQ4i6IoXgEA",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = "danielbayley"
    ))]
    Tally5,
    #[cfg(feature = "tangent")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CINAigkeIMDEN4PHKDAyCIPg8C+A4FgeGhwGEdBoCAZIME0MQ1C4NQ5CCKg0DGLgujAIIpisOYZhuIYjhqHYGgiCoShQIIWCKGIQgyDoahyBI/iCIokiYIooDIIAyDIcwtDQLQ5C0MYrl6Xw1FOV41lWZQxhiS47GgPoBA",
        categories = "shapes,maths,design,tools",
        tags = "tangential,shape,circle,geometry,trigonometry,bezier curve",
        contributors = "danielbayley"
    ))]
    Tangent,
    #[cfg(feature = "target")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gyDoQHKEwwCIPg8C+A4FgeGodgaCIKhOD4RiWFYMDaGYbiGH4CgSIong2JokjSKQig+GocjGH4BA",
        categories = "brands,gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[cfg(feature = "tent_tree")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDIIggGMeILDSDhjHmEgiD4PAvgOBYHhkcBhHQaAgGSCxtDENAgDUIAzC0M4siyGIaiCIofiGI4lCKJ4pDEMIsi6MAzjIL40GiNoiiSCxNDENwgigVoNhmRI3keOJKkyTg0EgNxsC2Kw4GgMgwFqQ5FlWSQiE0OJZHYOJllQPJFmgbQ5lmKggm6UpmgE",
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
    #[cfg(feature = "terminal_square")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQ3CAMQxCAMgthGEwiD4PAvgOBYWhmBoICITYOg0MxoDSFYXhyFhyGUYx0CAeIJDMIggGgZRpGcaB0gkMQ4jIcovCIMoyHmMI9kOQIyHcaRkgWOo8hYL4qiwPoB",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CAMQ3hMMITDEIISDUIg+DwL4DgWB4Jh6I4KHkMYQDEOQiCAeAyhAMgwi0eIpCIMQyi0eYwjeLIeC+Jg+gE",
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
    #[cfg(feature = "text_search")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyDEIA2EgMwiD4PAvgOBYThaBoIgoMQwCAMQyhCEoUhmGIEhqCRNh2Hw4iKE4VieExjGkchjGwZQgGMeYJDENQiCAcoJhGOR4jwN4jC+M41jeJoFgeCRtgyHw5C0MQulSVg5kiJYBA",
        categories = "text",
        tags = "find,data,copy,txt,pdf,document,scan,magnifier,magnifying glass",
        contributors = "danielbayley"
    ))]
    TextSearch,
    #[cfg(feature = "text_select")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMxhDIIIQDAIITDALYQDIIg+DwL4DgWG4egaCIKDEOYNg+EYUioMYphmG4dgQaIgjGB4JE0MosiWKISiuF4RhqHIhjOBY1gqDI4juKoTDGF4XkCMIfgKNIjE2JgzGgMZPkKUpElSJo4liWoxkOIo2DENINmGL5biGRRNmeEQxmqQZjlyZYKDMIA5HaWZrnWbZUjiep8mKUaAjaeZnoSfqGlONqCoqfZ0lEbBpG4ZQgHgMYJDcIggHkMoJDinh5psIqjpmoQiDENZPpWl4bq+mKapypKmDGGafqquKeHiu6di+sqxpas6mp2uoJDENq2smy6pskM6usQPoBA",
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
        categories = "account,social,emoji",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[cfg(feature = "thumbs_up")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMQwHYMQyCIPg8C+A4FhSF4GgiCgxDUIA1C4OA4g0NINDAaIhDgMxhDIIIuDAIIxDGDQuDmLgyC4NQ2GwLY5DMMwgDgQY4jKRo0DEN46i8MhIDSLYvkaM4+j4dgtDiUIwkeL4+GiOQ3DaWZSkcLg3DkLQxC4MQxEyEYvGgMBhDOapBnMMZBjGMwgnWIg4FqE4VhoPoB",
        categories = "account,social,emoji",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[cfg(feature = "ticket_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5jKHYnDQLQ0kWR5MD6AQ",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,booked,purchased,receipt,redeemed,validated,verified,certified,checked,used",
        contributors = "danielbayley"
    ))]
    TicketCheck,
    #[cfg(feature = "ticket_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoOYyDIaA2kWR5MD6AQA",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,remove,cancel,unbook,subtract,decrease,-",
        contributors = "danielbayley"
    ))]
    TicketMinus,
    #[cfg(feature = "ticket_percent")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIAxhQIITDYdgyGGDIMhOHwgh0aAxDaHIhheKITDILQyHaLIPhGKIVhUMAtDYVg3iaHopiyLBIDSOopheLIhFoIg+DwL4DgWSJLgaCIKDmDRoC4MAxkeSZOk2BJPgkbQxDWDY2CANpYkqXJbgWB4JE2YIUDWVJWmaWoBA",
        categories = "account,transportation,shopping",
        tags = "discount,reduced,offer,voucher,entry,pass,event,concert,show,book,purchase,%",
        contributors = "danielbayley"
    ))]
    TicketPercent,
    #[cfg(feature = "ticket_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoIgoOYyDIaA2kWR5MkuBJNgkTQxgwOR2lSRpIlkPoBA",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,book,purchase,add,+",
        contributors = "danielbayley"
    ))]
    TicketPlus,
    #[cfg(feature = "ticket_slash")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5C4NYyDSUggDULQ1kWR5MD6AQ",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,redeemed,used,marked,checked,verified,spoiled,invalidated,void,denied,refused,banned,barred,forbidden,prohibited,cancelled,cancellation,refunded,delete,remove,clear,error",
        contributors = "danielbayley"
    ))]
    TicketSlash,
    #[cfg(feature = "ticket_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAORhDMIIQDAIITDGFAgDYdgyGGDIMhOHwgh0aAxDaHIhheIAyC0Mh2iuD4RigIIWDALQ2FYN4mh6KIrisSA0jmMY0h0WgiD4PAvgOBZGkmBoICIbQ5C4NYyDSUggDULQ1kWR5MkuBJNgmUJWlGU5lluSJfD6AQ",
        categories = "account,transportation",
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
        svg = "gAPBsGkbhlCAeAxD0IgxDAIoHDKCgxDSDh5hAIgyhSCYXCIPg8C+A4Fh2IIGhWEQxg6CIRhiD4RDWGYRhOHYfgQZYdGMaRyGMbIGGMeIqg4Yx5jCDhygoOIch6N45jsPoBA",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "gAPByGUYx0CAch5D0Ig2CIIB4goMoNGgZRpGcaB0goMYRCCCYLg0dxpGQdBohAMINHKD4eD4PAvgOBYrGMaRyGMbBlgeEINGOHYajmKQ4CKKwvjGM41D6AQ",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "gAPByGUYx0CAeA9CIMgiCAeYJDaDByggIoQCAcoOhSDB3GkZB0GiCQyDCDBoGUaRnGgdIJDGCw+DwL4DgWLRjGkchjGwZQgGOGIrhGIIMGOEwxhCLQvjONY3D6AQ",
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
        svg = "gAPBlGwbBpHAcxlCAYx5D0IgxDEIggHKDAiDKERyHiDQzhEY4Zg6Fg+DwL4DgWBxliGJIGgiEoUDgLg1hyHgxhaCoUjOL4XjIMAiiGI4EiqJ4BA",
        categories = "shapes,design,tools,food-beverage",
        tags = "donut,doughnut,ring,hollow,3d,fast food,junk food,snack,treat,sweet,sugar,dessert",
        contributors = "danielbayley,jguddas"
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
        svg = "gAPByGUYx0CAaBlGkZxoHQPQiDENgiCAeIODKEhyhQIoWCAdxpGQdBohUMISHmDg0CIPg8C+A4FikcBhiAIBkg4TQyCAMQ0GgMojikL4viCLowGiMo0DGNo7HYLYRj2PxoD6AQ",
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
        svg = "gAPByGUYx0CAeQ9CIOAiCAaBlGkZxoHSCQxDKDByHiFIMHcaRkHQaIUgsIIZCIMwiD4PAvgOBYoHAYYfCAZIJE0MQwCAOBWDUYwwC0Lg2j0NAtDGQpCEgNhhDEIJJjaTJCkodomigL4uh+LYvGiMYzDEOY3jmO49j8LpBkOZBoC0M5IkoIJMmuTgxlCJ4plQaA+gEA",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANBoDkbAxCANwiD4PAvgOBYWhmBoIgoNAgDEMRWDSFYXhyG4Eh2CRNDiIQwiSJoYiqKYFgeLAxi4NRjC0Lg2CAMAthILg0kKIR2DWPoyiiAoqjcIhtDEMIhhKIggC4ORjj6QJXDkLpJi4MQuDEbI9joaJCkuNA8GMaRyGMbBlCAcoJl8IggGMeIJhSeB5gkMQ1jKbZvnGFqDnCchjn4IqAnedAioGeJ6CKFIWC+h6Fk2NoeE0MY/jmaKBpaTKYomk45neip/DijoJDKgpuogPoB",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDENoMg8Ig0gwaBlGkZxoHSGIaD4PAvgOBYoHAYYWCAZIJE0NAgDEMRohkIooC+LoWi2LxojGMwxDIIAzHYOI7imPhokCMIyCIbQ4jYOQtkaC48k2T5ClEbQxlQMgylcLZZkyQZckMIhNlQMQ1GgMJLj2aA8k2ahNhmNpvnGWpogE",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDEOAiD4PAvgOBYThaBoIgoMQ5g0dgxDQYwwCAMQtDEIAyC0MopEgNxjicIAwiuJ40DIVg2hKFIZhiBIagkTQ4g0VoijOKImiyKgyGiIooiSLIokkdgyjqFY+hMbBpG4ZQgHkMoJDENwil0MZgDGYx4l8IgxDCaJlmubYTC+WZbliWpcHmbwxmcIB4noNJomqIZjl6YJinKdBlD6AQ",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHKCAihEIIPgqDB3GkZB0GiCQxDiDBoGUaRnGgdIgiIPg8C+A4FiyL4GhQN4MhiNQghuHYfhkIIkiaKIJDkIosi6BB0jGR45hyHoJguB4JjiP4nikIg1jaIA0kSLYyD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQ4EgMxjC0Lg2CAMAtDGEw0hiGBWDcY4XhQLobDGDYYGgMQwGOFIWg0IIji4MR2DEMQiD4PAvgOBY3jqBoIgoMQ0CAORoDQbJCDQdg0iCLw2hqJolDEaAtDKNo4j2NxjGkchjGwZQgGMeYJg4IggHKCZVmAeIJDeVgvlqXJejyBI+gkTQxgyDhIDmbpYDycJdl8Y5rCIMZtmaaJlmGYw4m6gJygEA",
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
        contributors = "danielbayley"
    ))]
    Turtle,
    #[cfg(feature = "tv_2")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ3CAMgxGgMQwCIPg8C+A4FhQchlGMdAgHmCQzCIIByHiCQyiIdxpGSBYmhIIBoGUaRnGgdIJDENIiiUIonhQL4ahwPoBA",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CINwiCAaBlGkZxoHSCQxDWDB4gkMoMHKGAihoIB3GkZB0GiGQwhuCIeCIPg8C+A4FiwcBvGweRsGkbhlCCMo3HQc4UDcIAyCAMZCkCQIaiwL4yjSNo4D6AQ",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0Ig0CANwghKEgyDCFQghiEwiD4PAvgOBYHgmH4kgoeQxhCGAiCAeQyisMItHiKgiDmM4wCIMQ1h6IIniaCIKjSEAxDKOJEkaLo1DSLYvjGPQvj+AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ2CANB2DYYYMgwMAghSFAxDKFRWDQIg+DwL4DgWHhsGkbhlCAeQxgkMgwCIIB4DKK4tiiMQiiyLh4ioIoch4L4kiYPoBA",
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
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ3hAcoTCIPg8C+A4FgeGhwGEdBoCAZIME0MwgDcdg2GgNoZhuIYjiCIokiYIhNDIMQghQYQ5CCPwwCCQgwC0OZGkCSZEkMLQ2CAMguDMTIpDEM4wC+MhoD6AQ",
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
        svg = "gAPByGUYx0CAdxpGQdBoD0Ig4CIIBoGUaRnGgdINDaEB4g0NYQHmDQ0hAcobCIMQiD4PAvgOBYoiuBoIgqDIOh6DQxiEIIjjWEIShSFoYhqNYmiiKoEHQPoB",
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
        svg = "gAPBwGEdBoCAZA9CIbQxDgLg4DQIAxDILgyDWEQuDcMgtDGGAxGgLQuDAMhhDWIQwhCJQwicIAwiyLIghILQ3iENwgikMA2jaJo5i2PQtDYLg5hYMBshuGAyhcNwxCIPg8C+A4Fk2UIGgiColDGNQxhwNw1kaSpJDGJImiiY4uj0IAuhIIIzDCNY3jmb5mi6QJCiwbJbDGXpLk2T4EGiTRsGkbhlCAeAxgkOAiCAeaHCIMqKHgMqIooeaSCINZMk6gaDoCgqEoyk6LpaiaFo2j6FpamJ8psZadoOi6NDEOaUpYMqnpGCQxDakKxruq6eq6hK4o6t6xrOsK5ruorJpkL6sD6AQA",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unlock_keyhole")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwMQ2hAcoTCIPg8C+A4FgeGhyGUYx0CCCwiDOFomg8IISg0MIQHcaRkHQaITDiEBoGUaRnGgdITg+GgviGI4aHAYY0CAZIME0NwgDEMBWDcYQ1CCVAwCCVwxCAOQuDMMwtDILg1hmG5GjQPoBA",
        categories = "security",
        tags = "security",
        contributors = ""
    ))]
    UnlockKeyhole,
    #[cfg(feature = "unlock")]
    #[strum(props(
        svg = "gAPByGUYx0CAdxpGQdBoD0IgxDgIggHIeINDKERyHmFYRhmDgxhEaBlGkZxoHSDQxh4IIUCIMwiD4PAvgOBYuHAYYLCAZINE0NwgicVg3GENQgkEMAgkQMQgDkLg5C2HouC+NILD6AQ",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Unlock,
    #[cfg(feature = "unplug")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQxDkIA1CAMwtDMIg+DwL4DgWFYYgaCIKDIIAyh+EYThWF4EGiGongeCRNDYLgziAMIvGEMguDSII2CAMI6jwM45DATAxh8MQ4GwLQ2kcLY1jCS40jmNY3juUo9jYWoUhaG4pgWKwiE0Nwug8MY+mGOwxDGV4mhmAoqh0TQxjKYYumGMAxDSaJZmuW4dguHw2CCfp+kuSozlCOJRjyUoSjaRo1kijZOjehZTDCipRlaJZZgEA",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet,disconnect",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CINwiCAcoMDGDxjHiEgwCIPg8C+A4FgeGodgaCIVgwNIUgsIgyhiEIShmG4hh8PBwGEdBoCAZIME0NAuDcIAxDkLgzj4OQgDWLgvjONYakmNo4CIbQyDEIAzC2QpSDIIAyFqR5MkuNJNjmQAyDYIA3C4Ng4kWPgyGyWJGhqSJfl6NY3gwbQxDCPg0mqWAzC4NZVn+XJyjKX51k8MZpDGWAxC2UqPo6PpbnCXYBA",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwiCAcoJDSPBjHiCQ5iqLIxjONYrHAbxsHkbBpG6NpMlEdBzgmDAgDEMZaDiWgzg4MggkWK4tk2T5RGUPoB",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQ1CKCR4gwMQ4hAcoMDMIg+DwL4DgWB4bh6BoIGOEgiDmFoMDSEIKgwN4ahyIogDwcBhHQaAgGSDBNDEMAgg4SA2GENAgkSPpHC2RA0HYMowC+NY3huUI4joIhtDIMQuDePw2C4NAtC4OZghmG5PjYaJSmeOYMG2DguDKPwzmGYJiC6ZIclOaY3muVgxl0No/DiWp2nSTp5jSapVm0OQuDGPwym+hJhoaZ56lSbAxoygIUlqYJfDGlJRoie6Kn4Lg4o+kqfqGaKjpefQ0naXAupunqspafJXDCWpxqeP62mWeYBA",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwijceIJDmPBygkNIqiyMYzjWKxsGkbo2HgMYJDKKQgHgMoJgyPB5lAIgxDGWZWlyXorC+S5ND6AQ",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeYJDeOxjHiCQ5iqLIxjONYrGwaRujYeAxgkMZFCAeQylKOpVlEIg4jseJXCKU5GC+TJOkuTZPlsMopCCX5SDaOx5lsMQxnGYJ0mOZRlD6AQA",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[cfg(feature = "user_round_check")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCIIBygkNY0GMeYJDiIYjiiKosiKJYGggIhtDENoYDmDZNDQLQ0j6JIEGgPoBA",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserRoundCheck,
    #[cfg(feature = "user_round_cog")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMAuDQMw0C0NwuDYMgiD4PAvgOBYlGMaRyGMbBlCAYx5gkOAiCAcoJDWNhjHiCYZiSJori2L4qiyLowjyPo1jeCQzjuMwiDGNYlC+QpHiWKIGggIhtDEOQuDWGA0C4MwthsLg5kCJ4EGiWJsgeCZdDaaINhoOJmmOaZUlmboFnCXIOC4N4Yl8NZmDmeJqnyApvluXQ1C6DAxnOiJoome5sn2WpxoGg6TmCh4boqmaMn6jgxpCkpfDGoQ0qOKalpuXJemCDQxoKeKHq+baxn+cp0qika5miuw+gE",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserRoundCog,
    #[cfg(feature = "user_round_minus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx5gkOAii4eIJDEMIzHKCQ1iGI4oiqLIiiWBoIgoMoMDEORoh+PYkgQaA+gEA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundMinus,
    #[cfg(feature = "user_round_plus")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMwuDIOQyC0NgiD4PAvgOBYiGMaRyGMbBlCAYx4gkMQwCIIBygkNY0GMeYJDiIYjiiKosiKJYGgiCgxDmGA2HaIIiiSBBokOUIHgmC4MkgaIfj6T4mgEA",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundPlus,
    #[cfg(feature = "user_round_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQwCKCR5gwOIQHKDA1CIPg8C+A4FgeGhwGEdBoCAZIME0MggDIMRhDgIIuDAIIxDEIIOC4NAzDQLQ3C4NgyhmG4hiOGodgaCIWCIM4QGOEoNhSCYLk6QIcgSRogiKJImCIbQyimXQtDELg5mCYpTkIaA+gEA",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass",
        contributors = "jmsv,karsa-mistmere"
    ))]
    UserRoundSearch,
    #[cfg(feature = "user_round_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQyCAMgxGEOAghEMAghQMQgDEMQuDgNwzC0NwiD4PAvgOBYiGMaRyGMbBlCAYx5gkOAii4eIJDEMIzHKCQ1iGI4oiqLIiiWBoICIbQxDeGJJDUII8iKJIEGiQpRgeCRtDKDJIC2TJOiOQw+gEA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserRoundX,
    #[cfg(feature = "user_round")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDUIggGMeILDEMoOGMeYLDgIg+DwL4DgWB4bHAYR0GgIBkgsTQyDAIAyDEYQ4CCMIrjMLQxDYIAwhqHIiiQPoBA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserRound,
    #[cfg(feature = "user_search")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQwCKCR5gwN4QHKDA0CIPg8C+A4FgeGhwGEdBoCAZIME2DguDMIAxDUSA3GENAgjIMAgjUMAtjINB2DKGYbiGI4ah2BoIGOEoNhQIIWCIM4QgqDAxhSGocgSRIgiKJImCIbQyDEIJdC0MQuDmYZjj4L5AGgPoB",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass",
        contributors = "jmsv,jguddas,colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserSearch,
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx4gkOQijceYJDePBygkNIqiyMYzjWKxsGkbo2HkMoJDEM48HiUAiDKKQgHkMYJDiVJcCIMZAisL5Lk2SpMjaVZRkCWpWlKPJbl2X4JliRZlmkPoBA",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDkIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgOYUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAYx5gkNwiCAcoJDSPBjHiCQximKwvjGM41D6AQA",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_round")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDgIAyDEYYNg0MAghQMAtDENoVCIPg8C+A4Fh0YxpHIYxsGUIBygkNQiCAYx5gkOItGMeIJDEMIch6I4lieHYggaCIKDIMoODAY4XDMLgzDcLQyC0NguDULQ0C0OBhDUIJYhaFQtC4NJSDiSo5h+BBoD6AQ",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    UsersRound,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDYIAyDEdgtDIYQ0CCFQwCCGAwC0NIcEgNoUhaGYjhuFQ0HYMgiD4PAvgOBYrGMaRyGMbBlCAcoJDQIggGMeIJDmOxjHmCQ3iqLIxjONYri6BoIgoMgyg6EISiGF4kC0M5YC4OJFiuLYEGiS5ggeCYLg0MwuDEM5ViQIAxiMNwuDcNZGl+L4B",
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
        tags = "electricity,energy,transmission line,telegraph pole,power lines,phone",
        contributors = ""
    ))]
    UtilityPole,
    #[cfg(feature = "variable")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgxHMLQ0C0M4RC0OQghKF4ZCIPg8C+A4Fh2IIGgiCgxDYIAzHMNIphgIA5hGL4xDmHIeiOHRsGkbhlCAeAygmNAgHkMZACKQo/CIMQ1kYeJEkmS4dC+OY7jiOo8k2RZCk6QY+gmSpGHmSJflGUxlD6AQ",
        categories = "development,maths",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[cfg(feature = "vault")]
    #[strum(props(
        svg = "gAPByGUYx0CAeQ9CIMwiCAch4gkMoMHcaRkHQaIJDEOIMGgZRpGcaB0hiGggg+CgiD4PAvgOBYoGMaRyGMbBlCAZhpGwbIJGMdRygMbh0EMbxsG8coMGOJQ3C4NYMHKCZJkWCAikiSooC+LowjKKBwGGFggGSCRtkgOQgmEIAyC4N5lmeJ4plqFoti+MYzkwIpOCAY5QlKRYlDENp1jWN45juPY/kGQ5rlWcJYDybRol2XwxDMLg0CAMQwC4NppDcLZmDeh6Mm+V4zn+OAijqPBlj6QJCkSdpQnydZznWRoJnmVJWnGWZbo2XgimALpiq8MaZpuapUp8PK3jKdp7n2So0japKmoOqqGg2TbOneGLNoeyRlrmXK8G2kKSpSkaTpymaerqoJxq2GIRtYIrwrMIgxhGtqJt6AQ",
        categories = "security,travel,home",
        tags = "safe,lockbox,deposit,locker,coffer,strongbox,safety,secure,storage,valuables,bank",
        contributors = "danielbayley"
    ))]
    Vault,
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
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiHotgeCRNg4OB2DEMBjDAIAuDUNQuDQNQgDGQpCGgNhjjyQY6DELY/kGTAxHaEoYhqH4vgWMQiE0MQ2kIMAuDMNBWkcMJNj2TZAC2UJqGgLQ0mANJUiyHQ8GwaRuGUIB4DKCYjCIIB5nwIp+noMZ9n8eaGoOcp2ngPoB",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAOAggwMgthCD4PhKDwiD4PAvgOBYZhyBoIgoMoMDiFoThGJ4XhmG4EGiGRyGUYx0CAch4gkMQiCAeYJDWOR3GkZIFgkOI5GgZRpGcaB0jcNI5jYIpEiuMIyD6AQ",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDALg2DYIA2EgMQ0GEMgghcMAghoMYYhgdgyC4Mw0GyHQxEwMoXDgdg4CIPg8C+A4Fi+MoGgiCgxhCOYWh6GocC2FwyEiFZBhuRgxkCQBWDiPIZkeGJAGgMolhwMBai6MI1i8bBpG4ZQgHgMYJDIIpgDKY5kCAeZiCKaR5mebZki8L5cl4PoB",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyDIIA4C0NggDQIIRDQVg4FoIg+DwL4DgWGxyGUYx0CAeIJDIIggHmCQ2ikcomCKKAgHKK4xikdxpGSBYJDENIpGgZRpGcaB0jyKIbC+IYjD6AQA",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "gAPByGUYx0CAch4D0IgyCIIIJguDR5goNINGgZRpGcaB0goMQ2g0dxpGQdBogoMgwCIPg8C+A4FikcBhiMIBkgoTQyCAOBoiaKIqi+I4pGMaRyGMbBlgeJYNGOEgiDGFAgGODw4jsL5AkKRIujAaIyjQOAgDEMholGKQvj0aI/kGQ5Fk+HIekaEJOkqTJSlSaA+gEA",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = "danielbayley"
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
        svg = "gAPBjGkchjGwZQgGMeA9CINgigkeYMDEMoPHKDA0CIPg8C+A4FgeGodgaCIKhIOIVheDxjhEIoThmG4hh8PBsGkboIHkMYSg4IB4jiDYPHgMolg8eZBiyDoaC+M41D6AQA",
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
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDEIA1CANggDkIAyhiGggDGFYXh+HoTDGGYShQIg+DwL4DgWBxuiobIMGUIB4DGEAyDIIggHkMoQh+Ox5jcIg5jseI+hENopiuMhuGWMYzjySJAjWQwxkqPJDkWNZIjmSwvk2T4BA",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAdxpGQdBogkMQ4gwch4gkMoMhiCoMGgZRpGcaB0hOFQ+DwL4DgWJxwGGEQgGSCRNDMIA5GEMggjgMAgjsMY5C0MhoDENI3jmPJHj6OIaicL4thGLIuGiMIyjQMQxGgMxjC4OJIC4NgujQMguDELg5GyZJkDkY5kDYIJsCANJjm6XggDULg3jyZ5jC2ZZaDWfA1nMM58DmOZ7mUSAyDEIpMk4aA+gEA",
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
        svg = "gAPBjGkchjGwZQgGMeQ9CIOQiCAcoMDKDxjHiDA4CIPg8C+A4FgeGhwGEdBoCAZIMG0OQgDENwgDYLgxC2LgxGEMggjUMAgjgMY2C4OAxC4MAxEwMo1DENRWDWNI2jmTAwC0MpPEgNJKjeTZPjYdgxDCVJNkyNQyGgMQ2lyOJljaT4ZhuIYjiCIokiYIhNDiNgxGiGIaC+axom2I4lgwTQxkUNx2DSaZ5m4PoBA",
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
        svg = "gAPBwGEdBoCAZA9CITQyDIIA4C4Mw1FYMgwGGDYNDAIIZDELYXEgNIWCCGIaCCHAyh0VoPhEQYXiSGwgDMLgyDYIA2C4NRsDgLYxDKIYji8MQuDQOIajmMIyiyIouiWIoNioNRaCIPg8C+A4FlOVoGgiCo0DEOBoDEMpSlSWZYgSWoJE2XQ0mCYpTlWZ5THIZRjHQIB3GkZIFgmYQiCAeIJDafhoGUaRnGgdJ8mIIB5nwMJjC+c51D6AQA",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "washing_machine")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQzCANhoDMIg+DwL4DgWEoVgaCIKDEN4NGgLgwDGEYThiEhyGUYx0CAeIJhAIB5gkMgiCAdxpGSBYJDEOIzHKLAijIIBoGUaRnGgdIxDCIwvieKYSGMaRyGMbBlCAY4wCIMYuGOPgxkAcoJDWSpPlGU4XgSGYJE2XQgjoYQyC4NQgm+cQwCCdZ3C2cZznKcJ2n4MZ2nmSolgEA",
        categories = "home,devices,travel",
        tags = "tumble dryer,amenities,electronics,cycle,clothes,rinse,spin,drum",
        contributors = "danielbayley"
    ))]
    WashingMachine,
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
        svg = "gAPBjGkchjGwZQgGMeQ9CINAuDUIoJHiDAxDKERygwMoPCIPg8C+A4FgeHRwGEdBoCAZIMG0MQwC4MggDYLgzC0MwuDkII1DmHIeiSJodiCBoIhgIoahCCYLCKFYRGOE4NhuHYfgSQYjiWJ4pCITQ3CCFRoiyOwvj0aI/lKB4ShQOYbkeFIWCCQ5Fl+QIiDyYYoioMY1DiWw3C6Wo5jSNpfmGY4hgiTJrkuSAxmiRpuk+HpxGUPoBA",
        categories = "security,account,maps,navigation,development,social",
        tags = "indirection,vpn,virtual private network,proxy,connections,bounce,reroute,path,journey,planner,stops,stations,shared,spread,viral",
        contributors = "danielbayley"
    ))]
    Waypoints,
    #[cfg(feature = "webcam")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgHIPQiDgIggGMeILDEMoOGMeYSDAIg+DwL4DgWB4bh6BoIhCEoUgmCwzhWFwiDGGYbh2BIjhscBhHQaAgGSCxNDcIAyDIaIuhqHI1jeNI2jiOgiE2E4+DIdgtDSQwvkUaA+gE",
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
    #[cfg(feature = "weight")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQyCKCR5gwNYQHKDAzCIPg8C+A4FgeGhwGEdBoCAZIME0NguDUIA4GEMggi8MAgjIMAtDELg5DCK43DQNhMDILgxCAMQ4ioQYvjGM5KDSMAxGgMQ2i6MJKjSQ44DINQtkANQ0EwMQ5C6TJgDWR5TlWMgxDeYQ4iwWoZhuIYjD6AQ",
        categories = "maths",
        tags = "mass,heavy,lead,metal,measure,geometry,scales,balance",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Weight,
    #[cfg(feature = "wheat_off")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CIbQyCAMoMDEMAthAIg+DwL4DgWFYYgaCIKDENggDiEguDENwgDGJA3hSFobhqBIcgkTQzC4NImDEMguDUMwgDWJwxGyKI6ieOQzGGM49kcIAwkqJ5MDQLg5DQTI9DEORsiOOpYkWSZJkuSwxkoLZPlEWorheL4ugWB4JG0OIhC2RJEkaOQgl2TJeCCY5SDmJw1kCRJaGOOQ1nANQ1C4OIiigMqHDmipQiqFZnhmAovmsIhNhCUJgocMg2oIMaEC6nguDMNKFkENpwDcMxMDGOwzn+QpBludJ2l8IKjm6Tw2DKZotpWaodE0MpLDIaAyHYMhhDSeZ3ieYp5GgLQyFYNrMs6eJgqcNJlpKwIbpemYojWJw3jSNo7lWV60kOOpzkidLamKULNDCU4nlatJavCdbyky271kq3osmiwYwh6IIfoWR61vGPbznqShMnyoayDO/IzDSfMany2pDDkN5wo6v8GuGwwxDgLg3s2rwuDAOaCp6cKhjkMZwqbK8YjkNhMDKYAxj+WsZra/5evSvZwDIOMlpQbBpG4ZQgHgMYJr7Uwy1bVx51UItb1nXq+pLT9RD6AQ",
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
        svg = "gAPBjGkchjGwZQgHIPQiDMIggGMeILDeDhjHmCwxDIIg+DwL4DgWB4bHAYR0GgIBkgsTQxDAIA5HYNoahyIokhuHoGgiEIXhOD4WCKGIOgqDIwh2BI2iGI4licIopDQIA3HYOJCjIaJGiSJooDIMggDGTgxGOKwuDULZgloLQxloSAzGOYg1CAMJlmubwxHaZZRkcPoB",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeAyD0IgyDIIggHkMYKg2D4JguEx4hGCwiD4PAvgOBYcHAYR0GgIBkgoTQ4C4NQgDENorGGLIsDAII0DEIA3jWG4diKJIhiOJYnCITQyCCKg4DIYQxiyS41k6Nw0C4MQ3C0MguDYNY7C+PRoj+JImigMQwlcNggDUY5RDAMQtC4M5lioMZRDmLQxm0NAgDMLg3DaWpcl6QZhi8OJMnUMg1kqNo0oqLQglaDItlcOJ9kCf5gkOTAzoiLaLjaZguDINJVnqfIclulICgSBh5hUMgwg6CIKDGVpqq+Gayg6EISq6pYfGUPoB",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[cfg(feature = "wifi")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQ1CAMQzGEMQwg2EoUg2DQ0CAMAiD4PAvgOBYch+BoIgoOAugwMQ2icYYMgyFQxCAN4ZhuHYiiGBIjgkTQyCCJg4DKEIoi6GYWDKFI0h6OIcGwaRuGUIB5DGCZGCIIB4lIIgxDKVR4DKCZaC4MAxlUeZeCKVIcC+TJOD6AQ",
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
        svg = "gAPBwGEdBoCAZA9CITQ4CAMgyGgOAiD4PAvgOBYThaBoIgoNwgDEMBoDMbYdiALQxC4Mw0DOEoUhmGIEhqCRNDEMoeDUdg3iyFYwi+BYHjIN4oDCHZBDOQxBjSKAzh4MpKCAMJPlGHYfGENQglaUJZCCRQ5DEIA0iiXYLC4NgzgyTQ5DgMRDDgLg3laTQ2DaDJtDgN41k2KQgDmDRoDYY4nlYNINCANqEgyUAuDSQwtC4MA1C4OAwDmjQxDSVonDGaY6i4PBsGkbhlCAeAxgkMgiCAealCKp6jDKpqtHmr6sqeEwvp+oQ+gE",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAeIJgsIByg4IgygwdxpGQdBogkOIMGgZRpGcaB0hwIg+DwL4DgWJxwGGGggGSCRNDcIAxDEdg0GEMggjsMAgj6Po7DIaA0iaKIthqJ4qgaH4hiOJYNgkMYQhKCYVCCF4ZhsIodgeUoLieKYEHQPoBA",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "wrap_text")]
    #[strum(props(
        svg = "gAPBsGkbhlCAeQxD0Ig2CKBwygqDAgHiCQiDODR4g8IgyDEIg+DwL4DgWHhwGEdBoCAZIKE0MwgDEMhoDENRhiyLAwi2N42DYaAtDSHYfiSJojG8bB5iGBhwG+BB0HOCgxDaLZPDENItDiUAgDIMI+C+SJEkaHpGg6TQ4heGQxlmEoUhaB4UDGY4eiCBBlD6AQ",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis"
    ))]
    WrapText,
    #[cfg(feature = "wrench")]
    #[strum(props(
        svg = "gAPBwGEdBoCAZA9CITQxDQLg3CANguDMYQxCCFQwCCGIahYLg0GwMQuDaHA2hSFoZieGIgDSGRsDODg3C2Lg3DcYYiiKGwxC0NwuDmK47j0bAthEOYVkMMRhDILgxDIIJJkuKIWjGMRskaQo8keNpQhWPw0jqPIejGDoijINh6CIPg8C+A4FD6AQ",
        categories = "account,development,tools,home",
        tags = "account,settings,spanner,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis,csandman"
    ))]
    Wrench,
    #[cfg(feature = "x_circle")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQyCIIBygwMQwhAYx4hODw+DwL4DgWB4bHAYR0GgIBkgwbQxDUIA5C0NggDYIobC+IokiGI4licIhtDmLIwjCMocjUaA+gEA",
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
        svg = "gAPByGUYx0CAeQ9CIMwiCAcoICIMoMHcaRkHQaIJDEOIMHIeIJhEIBoGUaRnGgdIYhoIIdgoIg+DwL4DgWLRwGGFggGSCRtDENQgDkLQ2CANosi6M4WjKNBojaOA5jyQJAkIL5EGgPoB",
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
        svg = "gAPBwG8bB5GwaRuGUIIDggdBzD0IgxDILg0DEIA2C4Nw1CAMQzCAMocDALg1DcIA0C4OQyCIPg8C+A4FgeCYsi+BoIgqDBug6EAxDiI4lhKKIWDKFgxDCHA1C4Ng2iGK4tjSMRljOBI1gmCxvg2DwiDgIJbh4MQ0hyIJfhyQpiksMQ2k2LpTlCLJQCAeAxhCKpwDKc50Hmcginidp7iqLAvm2AQ",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[cfg(feature = "zap")]
    #[strum(props(
        svg = "gAPBwG8bB5GcbxuCCAxpG4dBzD0IgxDMIAyCCEwxDQIAxhWGIaDGFIVDKHwxDCGociWEoUCIPg8C+A4Fgcbg+gE",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis"
    ))]
    Zap,
    #[cfg(feature = "zoom_in")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeA9CIMQxCKCR5gyDoQHKDA4CIPg8C+A4FgeGhsGkboIHgMoTDYLg2DWEB5iaDYoiqLAxgwMoPCAeIzCKNYZhuIYjiCIokjmFAgHmOYYjeLpEi2Ew0jwL4+GWQIjjeR4yhONpMg2Nolk2T5RD6AQ",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "gAPBjGkchjGwZQgGMeQ9CIMQxCIIBygwOIQGMeIMg4Ig+DwL4DgWB4bGwaRuggeQyhgNguDYNYQHkMYMDKDwgHiJ4NimK4QHiLwijGGociKJIhiOCI6hOOY1DENItjuGQgiaGIPhsL5AGUPoBA",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
